use crate::resource_manager::resource_manager::*;
use crate::utils::*;
use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct ID {
    pub component_id: u64,
    pub member_id: u64,
}
#[derive(ScryptoSbor)]
pub struct State {
    pub total_token: KeyValueStore<Epoch, Decimal>,
    pub package: KeyValueStore<PackageAddress, bool>,
    pub vote: KeyValueStore<ComponentAddress, bool>,
    pub member: KeyValueStore<ComponentAddress, bool>,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneCreateVoteEvent {
    pub id: u64,
    pub voter: NonFungibleLocalId,
    pub url: String,
    pub keys: Vec<String>,
    pub start_epoch: u64,
    pub end_epoch: u64,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneWithdrawEvent {
    pub component_id: u64,
    pub address_id: NonFungibleLocalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneSetProposalStatusEvent {
    pub component_id: u64,
    pub status: bool,
}
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneRegisterEvent {
    pub id: u64,
    pub address: ComponentAddress,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneUnregisterEvent {
    pub id: u64,
    pub address: ComponentAddress,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneChangeRoleEvent {
    pub address_id: NonFungibleLocalId,
    pub role: String,
}
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneVoteEvent {
    pub component_id: u64,
    pub address_id: NonFungibleLocalId,
    pub key: String,
    pub amount: Decimal,
}

#[blueprint]
#[events(
    ArcaneSetProposalStatusEvent,
    ArcaneChangeRoleEvent,
    ArcaneWithdrawEvent,
    ArcaneVoteEvent,
    ArcaneRegisterEvent,
    ArcaneCreateVoteEvent
)]
#[types(Epoch, Decimal, ComponentAddress, PackageAddress, bool)]
mod arcane_main {

    // const ARC: ResourceManager =
    //     resource_manager!("resource_sim1t4czst3wl4maw93g3cnqz2tujsnf7rr7egjuzwv0a4njmumxtll7zw");
    // const CORE_BADGE: ResourceManager =
    //     resource_manager!("resource_tdx_2_1nt8n54ttjtdantt8eulk4f0v4s7jjgfzazrm7vkhrye8k895e43hsd");

    const ARC: ResourceManager =
        resource_manager!("resource_tdx_2_1tk08g7ulr5lp0lmm66me4gvzqp5dpq0dg84t8k78wlmacz7agwl8rs");
    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_tdx_2_1nt8n54ttjtdantt8eulk4f0v4s7jjgfzazrm7vkhrye8k895e43hsd");
    enable_method_auth! {
        roles {
            core => updatable_by: [];
        },
        methods {
            create_vote => PUBLIC;
            vote => PUBLIC;
            withdraw => PUBLIC;
            sign_up => PUBLIC;
            change_role => PUBLIC;
            set_status => PUBLIC;
            unregister => PUBLIC;
            package => restrict_to: [core];
            set_reward_address => restrict_to: [core];
            set_distance => restrict_to: [core];
        }
    }

    struct ArcaneMain {
        ids: ID,
        genesis_epoch: u64,
        arcane_vault: Vault,
        member_resource_manager: ResourceManager,
        state: State,
        distance: u64,
        reward_address: Option<ComponentAddress>,
        effect: bool,
    }

    impl ArcaneMain {
        pub fn instantiate() -> Global<ArcaneMain> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(ArcaneMain::blueprint_id());
            let member_resource_manager =
                ArcaneResourceManager::instantiate(component_address, CORE_BADGE.address());
            Self {
                ids: ID {
                    component_id: u64::zero(),
                    member_id: u64::zero(),
                },
                effect: true,
                distance: 92u64,
                genesis_epoch: Runtime::current_epoch().number(),
                arcane_vault: Vault::new(ARC.address()),
                state: State {
                    total_token: KeyValueStore::<Epoch, Decimal>::new_with_registered_type(),
                    package: KeyValueStore::<PackageAddress, bool>::new_with_registered_type(),
                    vote: KeyValueStore::<ComponentAddress, bool>::new_with_registered_type(),
                    member: KeyValueStore::<ComponentAddress, bool>::new_with_registered_type(),
                },
                reward_address: None,
                member_resource_manager,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE.address()))))
            .with_address(address_reservation)
            .globalize()
        }

        pub fn create_vote(
            &mut self,
            member_badge: Proof,
            arcane_package_address: PackageAddress,
            metadata: String,
            quarter: u8,
            keys_vec: Vec<String>,
        ) -> Global<AnyComponent> {
            assert!(
                self.state.package.get(&arcane_package_address).is_some(),
                "address not valid"
            );
            assert!(
                quarter > 0 && quarter < 4,
                "quarter must be between 1 and 3"
            );
            let id = member_badge
                .check_with_message(
                    self.member_resource_manager.address(),
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();

            let epoch = Epoch::of(self.get_epoch_of_quarter(quarter));

            self.ids.component_id += 1;

            Runtime::emit_event(ArcaneCreateVoteEvent {
                id: self.ids.component_id,
                voter: id,
                url: metadata.to_owned(),
                keys: keys_vec.clone(),
                start_epoch: Runtime::current_epoch().number(),
                end_epoch: epoch.number(),
            });

            let result: Global<AnyComponent> = self.call_blueprint(
                arcane_package_address,
                "ArcaneVoteFactory".to_string(),
                "instantiate".to_string(),
                scrypto_args!(self.ids.component_id, metadata, keys_vec, epoch),
            );

            self.state.vote.insert(result.address(), true);
            self.state.total_token.insert(epoch, dec!(0));

            result
        }

        pub fn vote(
            &mut self,
            member_badge: Proof,
            component_address: ComponentAddress,
            key: String,
            token: Bucket,
        ) {
            self.effect();
            assert!(
                self.state.vote.get(&component_address).is_some(),
                "address not valid"
            );

            let checked_nft_id = member_badge
                .check_with_message(
                    self.member_resource_manager.address(),
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let amount = token.amount();

            let (epoch, component_id): (Epoch, u64) = self.call_method(
                component_address,
                "add_voter".to_string(),
                scrypto_args!(checked_nft_id.clone(), key.clone(), amount),
            );

            let mut data = self.state.total_token.get_mut(&epoch).unwrap();
            *data += token.amount();

            Runtime::emit_event(ArcaneVoteEvent {
                address_id: checked_nft_id,
                amount,
                component_id,
                key,
            });

            self.arcane_vault.put(token);
        }

        pub fn withdraw(
            &mut self,
            member_badge: Proof,
            component_address: ComponentAddress,
        ) -> (Bucket, Option<Bucket>) {
            self.effect();
            assert!(
                self.state.vote.get(&component_address).is_some(),
                "address not valid"
            );

            let checked_nft_id = member_badge
                .check_with_message(
                    self.member_resource_manager.address(),
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();

            let (vote_epoch_at, amount_user_voted, component_id): (Epoch, Decimal, u64) = self
                .call_method(
                    component_address,
                    "get_amount_of".to_string(),
                    scrypto_args!(checked_nft_id.clone()),
                );

            let reward: Option<Bucket> = self.call_method(
                self.reward_address.expect("reward address not set"),
                "calculate_reward".to_string(),
                scrypto_args!(amount_user_voted, vote_epoch_at),
            );

            Runtime::emit_event(ArcaneWithdrawEvent {
                address_id: checked_nft_id,
                component_id,
            });
            (self.arcane_vault.take(amount_user_voted), reward)
        }

        pub fn sign_up(&mut self, mut address: Global<Account>) {
            assert!(
                self.state.member.get(&address.address()).is_none(),
                "address already registered"
            );
            self.ids.member_id += 1;
            let badge = self.member_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.ids.member_id),
                ArcaneNFT {
                    id: self.ids.member_id,
                    owner: address.address(),
                    role: Role::Member,
                },
            );
            self.state.member.insert(address.address(), true);
            address.try_deposit_or_abort(badge, None);
            Runtime::emit_event(ArcaneRegisterEvent {
                id: self.ids.member_id,
                address: address.address(),
            });
        }

        pub fn change_role(&mut self, nft: Bucket, role: String) -> Bucket {
            self.effect();
            let member_rs = self.member_resource_manager.address();
            assert!(
                nft.resource_address() == member_rs,
                "please provided arcaneNFT"
            );
            let nft_id = nft.as_non_fungible().non_fungible_local_id();
            match role.as_str() {
                "a" => {
                    self.member_resource_manager.update_non_fungible_data(
                        &nft_id,
                        "role",
                        Role::Admin,
                    );
                }
                "m" => {
                    self.member_resource_manager.update_non_fungible_data(
                        &nft_id,
                        "role",
                        Role::Member,
                    );
                }
                _ => panic!("Invalid role"),
            }
            Runtime::emit_event(ArcaneChangeRoleEvent {
                address_id: nft_id,
                role,
            });
            nft
        }

        pub fn set_reward_address(&mut self, address: ComponentAddress) {
            self.reward_address = Some(address);
        }

        pub fn package(&mut self, method: String, address: PackageAddress) {
            match method.as_str() {
                "a" => {
                    self.state.package.insert(address, true);
                }
                "r" => {
                    self.state.package.remove(&address);
                }
                _ => panic!("Invalid method"),
            }
        }

        pub fn set_status(
            &mut self,
            nft_proof: Proof,
            component_address: ComponentAddress,
            status: bool,
        ) {
            self.effect();
            assert!(
                self.state.vote.get(&component_address).is_some(),
                "address not valid"
            );
            let checked_nft_id = nft_proof
                .check_with_message(self.member_resource_manager.address(), "Proof Not Valid")
                .as_non_fungible()
                .non_fungible_local_id();
            let data: ArcaneNFT = self
                .member_resource_manager
                .get_non_fungible_data(&checked_nft_id);
            assert!(data.role == Role::Admin, "Not an admin");

            let component_id: u64 = self.call_method(
                component_address,
                "status".to_string(),
                scrypto_args!(status),
            );

            Runtime::emit_event(ArcaneSetProposalStatusEvent {
                component_id,
                status,
            });
        }

        pub fn set_distance(&mut self, distance: u64) {
            self.distance = distance;
        }

        pub fn unregister(&mut self, nft_badge: NonFungibleBucket) {
            let data: ArcaneNFT = nft_badge.non_fungible().data();
            match self.state.member.remove(&data.owner) {
                None => panic!("address not found"),
                Some(_) => nft_badge.burn(),
            }
            Runtime::emit_event(ArcaneUnregisterEvent {
                id: data.id,
                address: data.owner,
            });
        }

        fn get_epoch_of_quarter(&self, quarter: u8) -> u64 {
            let last_quarter =
                (Runtime::current_epoch().number() - self.genesis_epoch) / self.distance;
            (self.distance * (last_quarter + (quarter as u64))) + self.genesis_epoch
        }

        fn effect(&mut self) {
            self.effect = !self.effect;
        }

        fn call_blueprint<T: ScryptoDecode>(
            &mut self,
            package_address: PackageAddress,
            package_name: String,
            func: String,
            args: Vec<u8>,
        ) -> T {
            scrypto_decode(&ScryptoVmV1Api::blueprint_call(
                package_address,
                package_name.as_str(),
                func.as_str(),
                args,
            ))
            .unwrap()
        }

        fn call_method<T: ScryptoDecode>(
            &mut self,
            component_address: ComponentAddress,
            method: String,
            args: Vec<u8>,
        ) -> T {
            scrypto_decode(&ScryptoVmV1Api::object_call(
                component_address.as_node_id(),
                method.as_str(),
                args,
            ))
            .unwrap()
        }
    }
}

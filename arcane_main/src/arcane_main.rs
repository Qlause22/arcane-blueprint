use crate::resource_manager::resource_manager::ArcaneResourceManager;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
#[events(ArcaneRegisterEvent, ArcaneCreateVoteEvent)]
#[types(Epoch, Decimal, ComponentAddress, PackageAddress, bool)]
mod arcane_main {

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
            set_status => restrict_to: [core];
            package => restrict_to: [core];
            set_reward_address => restrict_to: [core];
        }
    }

    struct ArcaneMain {
        ids: ID,
        genesis_epoch: u64,
        arcane_vault: Vault,
        member_resource_manager: ResourceManager,
        state: State,
        reward_address: Option<ComponentAddress>,
    }

    impl ArcaneMain {
        pub fn instantiate(
            ARC: ResourceAddress,
            CORE_BADGE: ResourceAddress,
        ) -> Global<ArcaneMain> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(ArcaneMain::blueprint_id());
            let member_resource_manager =
                ArcaneResourceManager::instantiate(component_address, CORE_BADGE);
            Self {
                ids: ID {
                    component_id: u64::zero(),
                    member_id: u64::zero(),
                },
                genesis_epoch: Runtime::current_epoch().number(),
                arcane_vault: Vault::new(ARC),
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
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                ResourceAddress::try_from(CORE_BADGE).unwrap()
            ))))
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
                url: String::from(metadata.to_owned()),
                keys: keys_vec.clone(),
            });

            let result: Global<AnyComponent> = scrypto_decode(&ScryptoVmV1Api::blueprint_call(
                arcane_package_address,
                "ArcaneVoteFactory",
                "instantiate",
                scrypto_args!(self.ids.component_id, metadata, keys_vec, epoch),
            ))
            .unwrap();

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

            let result = ScryptoVmV1Api::object_call(
                component_address.as_node_id(),
                "add_voter",
                scrypto_args!(checked_nft_id, key, token.amount()),
            );
            let epoch: Epoch = scrypto_decode(&result).unwrap();

            let mut data = self.state.total_token.get_mut(&epoch).unwrap();
            *data += token.amount();

            self.arcane_vault.put(token)
        }

        pub fn withdraw(
            &mut self,
            member_badge: Proof,
            component_address: ComponentAddress,
        ) -> (Bucket, Option<Bucket>) {
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

            let (vote_epoch_at, amount_user_voted): (Epoch, Decimal) =
                scrypto_decode(&ScryptoVmV1Api::object_call(
                    component_address.as_node_id(),
                    "get_amount_of",
                    scrypto_args!(checked_nft_id),
                ))
                .unwrap();

            let reward_result = ScryptoVmV1Api::object_call(
                self.reward_address
                    .expect("Reward address not set")
                    .as_node_id(),
                "calculate_reward",
                scrypto_args!(amount_user_voted, vote_epoch_at),
            );

            (
                self.arcane_vault.take(amount_user_voted),
                scrypto_decode(&reward_result).unwrap(),
            )
        }

        pub fn sign_up(&mut self, mut address: Global<Account>) {
            assert!(
                self.state.member.get(&address.address()).is_none(),
                "address already registered"
            );
            self.ids.member_id += 1;
            Runtime::emit_event(ArcaneRegisterEvent {
                id: self.ids.member_id,
                address: address.address(),
            });
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
        }

        pub fn change_role(&self, nft: Bucket, role: String) -> Bucket {
            let member_rs = self.member_resource_manager.address();
            assert!(
                nft.resource_address() == member_rs,
                "please provided arcaneNFT"
            );
            let mut nft_id: ArcaneNFT = nft.as_non_fungible().non_fungible().data();

            match role.as_str() {
                "a" => nft_id.role = Role::Admin,
                "m" => nft_id.role = Role::Member,
                _ => panic!("Invalid role"),
            }
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

        pub fn set_status(&mut self, component_address: ComponentAddress, status: bool) {
            assert!(
                self.state.vote.get(&component_address).is_some(),
                "address not valid"
            );

            ScryptoVmV1Api::object_call(
                component_address.as_node_id(),
                "status",
                scrypto_args!(status),
            );
        }

        fn get_epoch_of_quarter(&self, quarter: u8) -> u64 {
            let last_quarter = (Runtime::current_epoch().number() - self.genesis_epoch) / 1u64;
            (1u64 * (last_quarter + (quarter as u64))) + self.genesis_epoch
        }
    }
}

use crate::resource_manager::resource_manager::ArcaneResourceManager;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
#[events(ArcaneRegisterEvent, ArcaneCreateVoteEvent)]
mod arcane_main {
    struct ArcaneMain {
        ids: ID,
        member_resource_manager: ResourceManager,
        genesis_epoch: u64,
        vault_address: Option<ComponentAddress>,
        reward_address: Option<ComponentAddress>,
    }

    impl ArcaneMain {
        pub fn instantiate() -> Global<ArcaneMain> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(ArcaneMain::blueprint_id());
            let member_resource_manager = ArcaneResourceManager::instantiate(component_address);
            Self {
                ids: ID {
                    component_id: u64::zero(),
                    member_id: u64::zero(),
                },
                genesis_epoch: Runtime::current_epoch().number(),
                vault_address: None,
                reward_address: None,
                member_resource_manager,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE))))
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
                quarter > 0 && quarter < 4,
                "quarter must be between 1 and 3"
            );
            member_badge.check_with_message(
                self.member_resource_manager.address(),
                "please provide a valid proof for Arcane",
            );

            let epoch = Epoch::of(self.get_epoch_of_quarter(quarter));

            self.ids.component_id += 1;

            Runtime::emit_event(ArcaneCreateVoteEvent {
                id: self.ids.component_id,
                url: String::from(metadata.to_owned()),
                keys: keys_vec.clone(),
            });

            let result = ScryptoVmV1Api::blueprint_call(
                arcane_package_address,
                "ArcaneVoteFactory",
                "instantiate",
                scrypto_args!(self.ids.component_id, metadata, keys_vec, epoch),
            );
            ScryptoVmV1Api::object_call(
                self.vault_address
                    .expect("Oracle address not set")
                    .as_node_id(),
                "add",
                scrypto_args!(None::<Bucket>, epoch),
            );
            scrypto_decode(&result).unwrap()
        }

        pub fn vote(
            &mut self,
            member_badge: Proof,
            component_address: ComponentAddress,
            key: String,
            token: Bucket,
        ) {
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

            ScryptoVmV1Api::object_call(
                self.vault_address
                    .expect("Oracle address not set")
                    .as_node_id(),
                "add",
                scrypto_args!(Some(token), epoch),
            );
        }

        pub fn withdraw(
            &mut self,
            member_badge: Proof,
            component_address: ComponentAddress,
        ) -> (Bucket, Bucket) {
            let checked_nft_id = member_badge
                .check_with_message(
                    self.member_resource_manager.address(),
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();

            let withdraw_result = ScryptoVmV1Api::object_call(
                component_address.as_node_id(),
                "get_amount_of",
                scrypto_args!(checked_nft_id),
            );

            let (vote_epoch_at, amount_user_voted): (Epoch, Decimal) =
                scrypto_decode(&withdraw_result).unwrap();

            let vault_result = ScryptoVmV1Api::object_call(
                self.vault_address
                    .expect("Oracle address not set")
                    .as_node_id(),
                "take",
                scrypto_args!(vote_epoch_at, amount_user_voted),
            );

            let (bucket, amount_voted_token_at_epoch): (Bucket, Decimal) =
                scrypto_decode(&vault_result).unwrap();

            let reward_result = ScryptoVmV1Api::object_call(
                self.reward_address
                    .expect("Oracle address not set")
                    .as_node_id(),
                "calculate_reward",
                scrypto_args!(bucket.amount(), amount_voted_token_at_epoch),
            );

            (bucket, scrypto_decode(&reward_result).unwrap())
        }

        pub fn sign_up(&mut self) -> Bucket {
            self.ids.member_id += 1;
            Runtime::emit_event(ArcaneRegisterEvent(self.ids.member_id));
            self.member_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(self.ids.member_id),
                ArcaneNFT {
                    id: self.ids.member_id,
                    role: Role::Member,
                },
            )
        }

        pub fn set_vault_address(&mut self, address: ComponentAddress) {
            self.vault_address = Some(address);
        }

        pub fn set_reward_address(&mut self, address: ComponentAddress) {
            self.reward_address = Some(address);
        }

        fn get_epoch_of_quarter(&self, quarter: u8) -> u64 {
            let last_quarter = (Runtime::current_epoch().number() - self.genesis_epoch) / 92u64;
            (1u64 * (last_quarter + (quarter as u64))) + self.genesis_epoch
        }
    }
}

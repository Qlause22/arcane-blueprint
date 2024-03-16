use crate::arcane_badge_manager::arcane_badge_manager::*;
use crate::arcane_core_data::arcane_core_data::*;
use crate::arcane_reward_vault::arcane_reward_vault::*;
use crate::arcane_vote::arcane_vote_factory::*;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_main {

    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");

    enable_method_auth! {
        methods {
            create_vote => PUBLIC;
            sign_up => PUBLIC;
        }
    }
    struct ArcaneMain {
        core_badge: NonFungibleVault,
        arcane_badge_member_rs: ResourceAddress,
        arcane_badge_component_rs: ResourceAddress,
        main_component_address: ComponentAddress,
        arcane_data_component_address: Global<ArcaneCoreData>,
        genesis_epoch: u64,
        arcane_vault: Global<ArcaneVault>,
    }

    impl ArcaneMain {
        pub fn instantiate(core_badge: Bucket) -> Global<ArcaneMain> {
            let (address_reservation, main_component_address) =
                Runtime::allocate_component_address(ArcaneMain::blueprint_id());
            let (arcane_badge_component_rs, arcane_badge_member_rs) =
                ArcaneBadgeManager::instantiate(main_component_address);
            let arcane_data_component_address =
                ArcaneCoreData::instantiate(arcane_badge_component_rs);

            let arcane_vault =
                ArcaneVault::instantiate(CORE_BADGE.address(), arcane_badge_component_rs);
            Self {
                core_badge: NonFungibleVault::with_bucket(core_badge.as_non_fungible()),
                arcane_badge_component_rs,
                arcane_badge_member_rs,
                main_component_address,
                arcane_data_component_address: arcane_data_component_address.into(),
                genesis_epoch: Runtime::current_epoch().number(),
                arcane_vault,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE.address()))))
            .with_address(address_reservation)
            .globalize()
        }

        pub fn create_vote(
            &mut self,
            member_badge: Proof,
            pict: Option<String>,
            description: String,
            quarter: u64,
            keys_vec: Vec<String>,
        ) -> Global<ArcaneVoteFactory> {
            assert!(
                quarter > 0 && quarter < 4,
                "quarter must be between 1 and 3"
            );
            let checked_nft_id = member_badge
                .check_with_message(
                    self.arcane_badge_member_rs,
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let epoch = self.get_epoch_of_quarter(quarter);

            let component_badge =
                ResourceManager::from_address(self.arcane_badge_component_rs).mint(1);
            let vote = ArcaneVoteFactory::instantiate(
                pict,
                description,
                checked_nft_id.clone(),
                keys_vec,
                component_badge,
                epoch.clone(),
                self.arcane_badge_member_rs,
                self.arcane_data_component_address.address(),
                self.arcane_vault.address(),
            );
            self.core_badge.authorize_with_non_fungibles(
                &self.core_badge.non_fungible_local_ids(1),
                || {
                    self.arcane_data_component_address.add_vote(
                        epoch,
                        checked_nft_id,
                        vote.address(),
                    )
                },
            );
            vote
        }

        pub fn sign_up(&self) -> Bucket {
            let badge = ResourceManager::from_address(self.arcane_badge_member_rs)
                .mint_ruid_non_fungible(ArcaneNFT {
                    role: Role::Member,
                    reward: dec!(0),
                });
            self.core_badge.authorize_with_non_fungibles(
                &self.core_badge.non_fungible_local_ids(1),
                || {
                    self.arcane_data_component_address
                        .add_user(badge.as_non_fungible().non_fungible_local_id());
                },
            );
            badge
        }

        fn get_epoch_of_quarter(&self, quarter: u64) -> u64 {
            let last_quarter = (Runtime::current_epoch().number() - self.genesis_epoch) / 92u64;
            (92u64 * (last_quarter + quarter)) + self.genesis_epoch
        }
    }
}

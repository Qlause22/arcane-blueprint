use crate::arcane_badge_manager::arcane_badge_manager::*;
use crate::arcane_member_component::arcane_member_component::*;
use crate::arcane_vote::arcane_vote_factory::*;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_main {

    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");
    const ARC: ResourceManager =
        resource_manager!("resource_sim1t4czst3wl4maw93g3cnqz2tujsnf7rr7egjuzwv0a4njmumxtll7zw");

    enable_method_auth! {
        methods {
            create_vote => PUBLIC;
            withdraw => PUBLIC;
            vote => PUBLIC;
            redem_reward => PUBLIC;
            deposit_reward => PUBLIC;
            sign_up => PUBLIC;
            reset_quarter => restrict_to: [OWNER];
            change_reward_per_quarter => restrict_to: [OWNER];
        }
    }
    struct ArcaneMain {
        core_badge: NonFungibleVault,
        commited_tokens: Vault,
        reward_vault: Vault,
        arcane_badge_manager: Owned<ArcaneBadgeManager>,
        arcane_badge_member_rs: ResourceAddress,
        arcane_badge_admin_rs: ResourceAddress,
        component_address: ComponentAddress,
        reward_rates_perquarter: Decimal,
        number_of_tokens_commited_at_epoch: KeyValueStore<u64, Decimal>,
        genesis_epoch: u64,
    }

    impl ArcaneMain {
        pub fn instantiate(core_badge: Bucket) -> Global<ArcaneMain> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(ArcaneMain::blueprint_id());
            let (arcane_badge_manager, arcane_badge_admin_rs, arcane_badge_member_rs) =
                ArcaneBadgeManager::instantiate(component_address);
            Self {
                core_badge: NonFungibleVault::with_bucket(core_badge.as_non_fungible()),
                commited_tokens: Vault::new(ARC.address()),
                reward_vault: Vault::new(ARC.address()),
                arcane_badge_manager,
                arcane_badge_admin_rs,
                arcane_badge_member_rs,
                component_address,
                reward_rates_perquarter: dec!(1000),
                number_of_tokens_commited_at_epoch: KeyValueStore::new(),
                genesis_epoch: Runtime::current_epoch().number(),
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
                quarter < 4 && quarter > 0,
                "quarter must be between 1 and 3"
            );
            let checked_nft_id = member_badge
                .check_with_message(
                    self.arcane_badge_member_rs,
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let nft_data: ArcaneMemberData =
                ResourceManager::from_address(self.arcane_badge_member_rs)
                    .get_non_fungible_data(&checked_nft_id);
            let arcane_vote = self.core_badge.authorize_with_non_fungibles(
                &self.core_badge.non_fungible_local_ids(1),
                || {
                    ArcaneVoteFactory::instantiate(
                        pict,
                        description,
                        checked_nft_id,
                        self.get_epoch_of_quarter(quarter),
                        keys_vec,
                        self.arcane_badge_member_rs,
                        self.component_address,
                    )
                },
            );
            let user_component: Global<ArcaneMemberComponent> = nft_data.owned_component.into();
            user_component.set_vote_owner(arcane_vote.address());
            arcane_vote
        }

        pub fn vote(
            &mut self,
            member_badge: Proof,
            arcane_vote_instance: Global<ArcaneVoteFactory>,
            commit_coin: Bucket,
            key: String,
        ) {
            assert!(
                commit_coin.amount() > dec!(0),
                "commit_coin must be greater than 0"
            );
            let checked_nft_id = member_badge
                .check_with_message(
                    self.arcane_badge_member_rs,
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let nft_data: ArcaneMemberData =
                ResourceManager::from_address(self.arcane_badge_member_rs)
                    .get_non_fungible_data(&checked_nft_id);
            let end_at_epoch = arcane_vote_instance.add_voter(
                key.to_string(),
                nft_data.owner,
                commit_coin.amount(),
            );
            let user_component: Global<ArcaneMemberComponent> = nft_data.owned_component.into();
            user_component.set_unwithdrawed_vote(arcane_vote_instance.address(), key);
            self.number_of_tokens_commited_at_epoch
                .get_mut(&end_at_epoch)
                .unwrap()
                .0 += commit_coin.amount().0;
            self.commited_tokens.put(commit_coin)
        }

        pub fn sign_up(&self, address: String) -> Bucket {
            self.arcane_badge_manager.mint_member(
                address,
                ArcaneMemberComponent::instantiate(
                    self.arcane_badge_member_rs,
                    self.component_address,
                )
                .address(),
            )
        }

        pub fn withdraw(
            &mut self,
            member_badge: Proof,
            arcane_vote_instance: Global<ArcaneVoteFactory>,
        ) -> Bucket {
            let checked_nft_id = member_badge
                .check_with_message(
                    self.arcane_badge_member_rs,
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let resource_manager = ResourceManager::from_address(self.arcane_badge_member_rs);
            let nft_data: ArcaneMemberData =
                resource_manager.get_non_fungible_data(&checked_nft_id);
            let (amount, end_at_epoch) =
                arcane_vote_instance.get_amount_tokens_commited(nft_data.owner);
            assert!(
                self.number_of_tokens_commited_at_epoch
                    .get(&end_at_epoch)
                    .is_some(),
                "no vote exist at {}",
                end_at_epoch
            );
            assert!(
                !self
                    .number_of_tokens_commited_at_epoch
                    .get(&end_at_epoch)
                    .unwrap()
                    .is_zero(),
                "no one votes at {}",
                end_at_epoch
            );

            resource_manager.update_non_fungible_data(
                &checked_nft_id,
                "reward",
                nft_data.reward
                    + self.reward_rates_perquarter
                        * (amount.0
                            / self
                                .number_of_tokens_commited_at_epoch
                                .get(&end_at_epoch)
                                .unwrap()
                                .0),
            );
            let user_component: Global<ArcaneMemberComponent> = nft_data.owned_component.into();
            user_component.set_withdrawed_vote(arcane_vote_instance.address());
            self.commited_tokens.take(amount)
        }

        pub fn redem_reward(&mut self, member_badge: Proof) -> Bucket {
            let checked_nft_id = member_badge
                .check_with_message(
                    self.arcane_badge_member_rs,
                    "please provide a valid proof for Arcane",
                )
                .as_non_fungible()
                .non_fungible_local_id();
            let resource_manager = ResourceManager::from_address(self.arcane_badge_member_rs);
            let nft_data: ArcaneMemberData =
                resource_manager.get_non_fungible_data(&checked_nft_id);
            let reward_bucket = self.reward_vault.take(nft_data.reward);
            resource_manager.update_non_fungible_data(&checked_nft_id, "reward", dec!(0));
            reward_bucket
        }

        pub fn reset_quarter(&mut self) {
            self.number_of_tokens_commited_at_epoch = KeyValueStore::new();
        }

        pub fn change_reward_per_quarter(&mut self, reward: Decimal) {
            self.reward_rates_perquarter = reward;
        }

        pub fn deposit_reward(&mut self, reward_bucket: Bucket) {
            self.reward_vault.put(reward_bucket);
        }

        fn get_epoch_of_quarter(&self, quarter: u64) -> u64 {
            let last_quarter = (Runtime::current_epoch().number() - self.genesis_epoch) / 92u64;
            let epoch = (92u64 * (last_quarter + quarter)) + self.genesis_epoch;
            if self
                .number_of_tokens_commited_at_epoch
                .get(&epoch)
                .is_none()
            {
                self.number_of_tokens_commited_at_epoch
                    .insert(epoch, dec!(0))
            }
            epoch
        }
    }
}

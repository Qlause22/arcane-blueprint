use arcane_main::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_reward {
    struct ArcaneReward {
        reward_vault: Vault,
        reward_rate: Decimal,
    }

    impl ArcaneReward {
        pub fn instantiate(reward_bucket: Bucket, reward_rate: Decimal) -> Global<ArcaneReward> {
            Self {
                reward_vault: Vault::with_bucket(reward_bucket),
                reward_rate,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE))))
            .globalize()
        }

        pub fn calculate_reward(
            &mut self,
            amount: Decimal,
            amount_voted_token_at_epoch: Decimal,
        ) -> Bucket {
            self.reward_vault
                .take(self.reward_rate * (amount / amount_voted_token_at_epoch))
        }
    }
}

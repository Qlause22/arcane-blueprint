use scrypto::prelude::*;

#[blueprint]
mod arcane_reward_vault {

    const ARC: ResourceManager =
        resource_manager!("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");
    enable_method_auth! {
        roles {
            vote => updatable_by: [];
        },
        methods {
            take_reward => restrict_to: [vote];
            deposit_reward => PUBLIC;
        }
    }
    struct ArcaneVault {
        reward_vault: Vault,
    }

    impl ArcaneVault {
        pub fn instantiate(
            core_badge: ResourceAddress,
            vote_badge_rs: ResourceAddress,
        ) -> Global<ArcaneVault> {
            Self {
                reward_vault: Vault::new(ARC.address()),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(core_badge))))
            .roles(roles!( vote => rule!(require(vote_badge_rs));))
            .globalize()
        }

        pub fn deposit_reward(&mut self, amount: Bucket) {
            self.reward_vault.put(amount)
        }

        pub fn take_reward(&mut self, amount: Decimal) -> Bucket {
            self.reward_vault.take(amount)
        }
    }
}

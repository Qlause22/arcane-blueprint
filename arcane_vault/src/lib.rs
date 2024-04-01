use arcane_main::utils::*;
use scrypto::prelude::*;

#[blueprint]
#[types(Epoch, Decimal)]
mod arcane_vault {

    struct ArcaneVault {
        arcane_vault: Vault,
        total_commited_token_at: KeyValueStore<Epoch, Decimal>,
    }

    impl ArcaneVault {
        pub fn instantiate() -> Global<ArcaneVault> {
            Self {
                total_commited_token_at: KeyValueStore::<Epoch, Decimal>::new_with_registered_type(
                ),
                arcane_vault: Vault::new(XRD),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE))))
            .globalize()
        }

        pub fn add(&mut self, token: Option<Bucket>, epoch: Epoch) {
            if let Some(bucket) = token {
                let mut data = self.total_commited_token_at.get_mut(&epoch).unwrap();
                *data += bucket.amount();
                self.arcane_vault.put(bucket);
            } else {
                self.total_commited_token_at.insert(epoch, dec!(0));
            }
        }

        pub fn take(&mut self, epoch: Epoch, amount: Decimal) -> (Bucket, Decimal) {
            (
                self.arcane_vault.take(amount),
                Decimal(self.total_commited_token_at.get(&epoch).unwrap().0),
            )
        }
    }
}

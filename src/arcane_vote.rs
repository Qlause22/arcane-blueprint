use scrypto::prelude::*;

#[blueprint]
mod arcane_vote_factory {
    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");
    enable_function_auth! {
        instantiate => rule!(require(CORE_BADGE.address()));
    }
    enable_method_auth! {
        roles {
            main => updatable_by: [];
        },
        methods {
            add_voter => restrict_to: [main];
            get_amount_tokens_commited => restrict_to: [main];
        }
    }
    struct ArcaneVoteFactory {
        end_at_epoch: u64,
        pict: Option<String>,
        description: String,
        keys: KeyValueStore<String, (u128, Decimal)>,
        voter: KeyValueStore<String, (Decimal, bool)>,
    }

    impl ArcaneVoteFactory {
        pub fn instantiate(
            pict: Option<String>,
            description: String,
            nft_id: NonFungibleLocalId,
            quarter: u64,
            keys: Vec<String>,
            member_badge: ResourceAddress,
            main_component_address: ComponentAddress,
        ) -> Global<ArcaneVoteFactory> {
            let voter: KeyValueStore<String, (Decimal, bool)> = KeyValueStore::new();
            let keys_new: KeyValueStore<String, (u128, Decimal)> = KeyValueStore::new();
            for key in keys.iter() {
                keys_new.insert(key.to_string(), (0u128, dec!(0)));
            }
            Self {
                pict,
                description,
                end_at_epoch: quarter,
                keys: keys_new,
                voter,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(NonFungibleGlobalId::new(
                member_badge,
                nft_id,
            )))))
            .roles(roles!( main => rule!(require(global_caller(main_component_address)));))
            .globalize()
        }

        pub fn add_voter(&mut self, key: String, address: String, coins_amount: Decimal) -> u64 {
            assert!(
                Runtime::current_epoch().number() <= self.end_at_epoch,
                "vote closed"
            );
            assert!(self.voter.get(&address).is_none(), "address already voted");

            if let Some(mut key) = self.keys.get_mut(&key) {
                key.0 += 1;
                key.1 += coins_amount;
                self.voter.insert(address, (coins_amount, false));
            } else {
                panic!("Key does not exist");
            }
            self.end_at_epoch
        }

        pub fn get_amount_tokens_commited(&mut self, address: String) -> (Decimal, u64) {
            assert!(
                Runtime::current_epoch().number() >= self.end_at_epoch,
                "withdraw not allowed at current epoch"
            );
            if let Some(mut data) = self.voter.get_mut(&address) {
                if data.1 {
                    panic!("address already withdrawed its tokens");
                }
                data.1 = true;
                (data.0, self.end_at_epoch)
            } else {
                panic!("address not voted yet")
            }
        }
    }
}

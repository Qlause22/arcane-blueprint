use scrypto::prelude::*;

#[blueprint]
mod arcane_member_component {
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
            set_unwithdrawed_vote => restrict_to: [main];
            set_withdrawed_vote => restrict_to: [main];
            set_vote_owner => restrict_to: [main];
        }
    }
    struct ArcaneMemberComponent {
        unwithdrawed_vote: KeyValueStore<ComponentAddress, String>,
        withdrawed_vote: KeyValueStore<ComponentAddress, String>,
        vote_owner: KeyValueStore<ComponentAddress, ()>,
    }
    impl ArcaneMemberComponent {
        pub fn instantiate(
            member_address: ResourceAddress,
            main_component_address: ComponentAddress,
        ) -> Global<ArcaneMemberComponent> {
            Self {
                unwithdrawed_vote: KeyValueStore::new(),
                withdrawed_vote: KeyValueStore::new(),
                vote_owner: KeyValueStore::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(member_address))))
            .roles(roles!( main => rule!(require(global_caller(main_component_address)));))
            .globalize()
        }

        pub fn set_unwithdrawed_vote(&mut self, address: ComponentAddress, key: String) {
            self.unwithdrawed_vote.insert(address, key);
        }

        pub fn set_withdrawed_vote(&mut self, address: ComponentAddress) {
            assert!(
                self.unwithdrawed_vote.get(&address).is_some(),
                "this address has already been withdrawed the tokens"
            );
            self.withdrawed_vote
                .insert(address, self.unwithdrawed_vote.remove(&address).unwrap())
        }

        pub fn set_vote_owner(&mut self, address: ComponentAddress) {
            self.vote_owner.insert(address, ());
        }
    }
}

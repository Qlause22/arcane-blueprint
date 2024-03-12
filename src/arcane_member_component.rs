use scrypto::prelude::*;

#[blueprint]
mod arcane_member_component {
    struct ArcaneMemberComponent {
        unwithdrawed_vote: KeyValueStore<ComponentAddress, String>,
        withdrawed_vote: KeyValueStore<ComponentAddress, String>,
        vote_owner: KeyValueStore<ComponentAddress, ()>,
        reward: Decimal,
        owner: String,
    }
    impl ArcaneMemberComponent {
        pub fn instantiate(
            member_address: ResourceAddress,
            address: String,
        ) -> Global<ArcaneMemberComponent> {
            Self {
                unwithdrawed_vote: KeyValueStore::new(),
                withdrawed_vote: KeyValueStore::new(),
                vote_owner: KeyValueStore::new(),
                reward: dec!(0),
                owner: address,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(member_address))))
            .globalize()
        }

        pub fn add_unwithdrawed_vote(&mut self, address: ComponentAddress, key: String) {
            self.unwithdrawed_vote.insert(address, key);
        }

        pub fn add_withdrawed_vote(&mut self, address: ComponentAddress) {
            assert!(
                self.unwithdrawed_vote.get(&address).is_some(),
                "this address has already been withdrawed the tokens"
            );
            self.withdrawed_vote
                .insert(address, self.unwithdrawed_vote.remove(&address).unwrap())
        }

        pub fn add_vote_owner(&mut self, address: ComponentAddress) {
            self.vote_owner.insert(address, ());
        }

        pub fn add_reward(&mut self, reward: Decimal) {
            self.reward += reward;
        }

        pub fn owner(&self) -> String {
            self.owner.to_string()
        }

        pub fn get_rewards_and_reset(&mut self) -> Decimal {
            let to_return = self.reward.clone();
            self.reward = dec!(0);
            to_return
        }
    }
}

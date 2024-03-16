use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_core_data {
    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");

    enable_method_auth! {
        roles {
            vote => updatable_by: [];
        },
        methods {
            withdraw_and_update_data => restrict_to: [vote];
            vote_and_update_data => restrict_to: [vote];
            set_reward_rate => restrict_to: [OWNER];
            add_user => restrict_to: [OWNER];
            add_vote => restrict_to: [OWNER];
        }
    }
    struct ArcaneCoreData {
        number_of_tokens_commited_at_epoch: KeyValueStore<u64, Decimal>,
        reward_rates_perquarter: Decimal,
        member_data: KeyValueStore<
            NonFungibleLocalId,
            KeyValueStore<VoteKind, KeyValueStore<ComponentAddress, ()>>,
        >,
    }
    impl ArcaneCoreData {
        pub fn instantiate(vote_badge_rs: ResourceAddress) -> ComponentAddress {
            Self {
                number_of_tokens_commited_at_epoch: KeyValueStore::new(),
                reward_rates_perquarter: dec!(1000),
                member_data: KeyValueStore::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(CORE_BADGE.address()))))
            .roles(roles!( vote => rule!(require(vote_badge_rs));))
            .globalize()
            .address()
        }

        pub fn vote_and_update_data(
            &mut self,
            epoch: u64,
            amount: Decimal,
            component_vote_address: ComponentAddress,
            id: NonFungibleLocalId,
        ) {
            let mut data = self
                .number_of_tokens_commited_at_epoch
                .get_mut(&epoch)
                .unwrap();
            data.0 += amount.0;

            match self.member_data.get_mut(&id) {
                Some(member) => {
                    if member.get(&VoteKind::Voted).is_none() {
                        member.insert(VoteKind::Voted, KeyValueStore::new());
                    }
                    member
                        .get(&VoteKind::Voted)
                        .unwrap()
                        .insert(component_vote_address, ());
                }
                _ => panic!("unregistred user"),
            }
        }

        pub fn add_user(&mut self, id: NonFungibleLocalId) {
            self.member_data.insert(id, KeyValueStore::new());
        }

        pub fn add_vote(
            &mut self,
            epoch: u64,
            owner: NonFungibleLocalId,
            component_vote_address: ComponentAddress,
        ) {
            self.number_of_tokens_commited_at_epoch
                .insert(epoch, dec!(0));

            match self.member_data.get_mut(&owner) {
                Some(member) => {
                    if member.get(&VoteKind::Owned).is_none() {
                        member.insert(VoteKind::Owned, KeyValueStore::new());
                    }
                    member
                        .get(&VoteKind::Owned)
                        .unwrap()
                        .insert(component_vote_address, ());
                }
                _ => panic!("unregistred user"),
            }
        }

        pub fn set_reward_rate(&mut self, new_reward_rate: Decimal) {
            self.reward_rates_perquarter = new_reward_rate;
        }

        pub fn withdraw_and_update_data(
            &mut self,
            at_epoch: u64,
            id: NonFungibleLocalId,
            component_vote_address: ComponentAddress,
        ) -> (Decimal, Decimal) {
            assert!(
                self.number_of_tokens_commited_at_epoch
                    .get(&at_epoch)
                    .is_some(),
                "no vote creates at epoch {}",
                at_epoch,
            );
            assert!(
                self.number_of_tokens_commited_at_epoch
                    .get(&at_epoch)
                    .is_some(),
                "no vote creates at epoch {}",
                at_epoch,
            );
            match self.member_data.get_mut(&id) {
                Some(member) => {
                    if member.get(&VoteKind::Withdrawed).is_none() {
                        member.insert(VoteKind::Withdrawed, KeyValueStore::new());
                    }
                    member
                        .get(&VoteKind::Voted)
                        .unwrap()
                        .remove(&component_vote_address);
                    member
                        .get(&VoteKind::Withdrawed)
                        .unwrap()
                        .insert(component_vote_address, ());
                }
                _ => panic!("unregistred user"),
            }
            (
                *self
                    .number_of_tokens_commited_at_epoch
                    .get(&at_epoch)
                    .unwrap(),
                self.reward_rates_perquarter,
            )
        }
    }
}

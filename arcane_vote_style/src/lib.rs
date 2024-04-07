use arcane_main::utils::*;
use scrypto::prelude::*;

#[blueprint]
#[events(ArcaneVoteEvent, ArcaneWithdrawEvent)]
#[types(VoterData, KeyData, NonFungibleLocalId)]
mod arcane_vote_factory {

    enable_function_auth! {
        instantiate  => rule!(require(global_caller(MAIN)));
    }

    enable_method_auth! {
        roles {
            main => updatable_by: [];
        },
        methods {
            add_voter => restrict_to: [main];
            get_amount_of => restrict_to: [main];
            status => restrict_to: [main];
        }
    }
    struct ArcaneVoteFactory {
        id: u64,
        keys: HashMap<String, KeyData>,
        voter: KeyValueStore<NonFungibleLocalId, VoterData>,
        metadata: String,
        status: bool,
        end: Epoch,
    }

    impl ArcaneVoteFactory {
        pub fn instantiate(
            id: u64,
            metadata: String,
            keys: Vec<String>,
            end: Epoch,
        ) -> Global<ArcaneVoteFactory> {
            let mut new_keys: HashMap<String, KeyData> = HashMap::new();
            for key in keys.iter() {
                new_keys.insert(
                    key.to_owned(),
                    KeyData {
                        voters: 016,
                        amounts: dec!(0),
                    },
                );
            }
            Self {
                id,
                keys: new_keys,
                status: true,
                voter: KeyValueStore::<NonFungibleLocalId, VoterData>::new_with_registered_type(),
                metadata,
                end,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .roles(roles! {
                main => rule!(require(global_caller(MAIN)));
            })
            .globalize()
        }

        pub fn add_voter(
            &mut self,
            voter: NonFungibleLocalId,
            key: String,
            amount: Decimal,
        ) -> Epoch {
            assert!(self.status, "vote is not activated yet");
            assert!(Runtime::current_epoch() <= self.end, "vote closed");

            match self.keys.get_mut(&key) {
                Some(value) => {
                    value.voters += 1;
                    value.amounts += amount;
                }
                None => panic!("invalid key: {}", key),
            }
            match self.voter.get(&voter) {
                Some(_) => panic!("{} has already voted", voter),

                None => self.voter.insert(
                    voter.clone(),
                    VoterData {
                        key: key.to_string(),
                        is_not_withdrawed: true,
                        amount,
                    },
                ),
            }
            Runtime::emit_event(ArcaneVoteEvent {
                component_id: self.id,
                address_id: voter,
                key,
                amount,
            });
            self.end
        }

        pub fn get_amount_of(&mut self, voter: NonFungibleLocalId) -> (Epoch, Decimal) {
            assert!(
                Runtime::current_epoch() >= self.end,
                "vote is not yet ended"
            );
            match self.voter.get_mut(&voter) {
                Some(mut value) => {
                    assert!(value.is_not_withdrawed, "{} already withdrawed", voter);
                    value.is_not_withdrawed = false;
                    Runtime::emit_event(ArcaneWithdrawEvent {
                        component_id: self.id,
                        address_id: voter,
                    });
                    (self.end, value.amount)
                }
                None => panic!("{} is not voted yet", voter),
            }
        }

        pub fn status(&mut self, status: bool) {
            self.status = status;
        }
    }
}

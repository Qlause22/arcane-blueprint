use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct VoterData {
    pub key: String,
    pub amount: Decimal,
    pub is_not_withdrawed: bool,
}

#[derive(ScryptoSbor)]
pub struct KeyData {
    pub voters: u16,
    pub amounts: Decimal,
}

#[blueprint]
#[types(VoterData, KeyData, NonFungibleLocalId)]
mod arcane_vote_factory {

    enable_function_auth! {
        instantiate  => rule!(
            require(
                global_caller(
                    ComponentAddress::new_or_panic([
                        192, 48, 187, 194, 245, 43, 245, 53, 142, 133, 187, 178, 148, 238, 130, 8, 246, 157, 21, 84, 82, 155, 114, 91, 72, 236, 23, 248, 158, 205
                    ])
                )
            )
        );
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
                        voters: 0u16,
                        amounts: dec!(0),
                    },
                );
            }
            Self {
                id,
                keys: new_keys,
                status: false,
                voter: KeyValueStore::<NonFungibleLocalId, VoterData>::new_with_registered_type(),
                metadata,
                end,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .roles(roles! {
                main => rule!(
                    require(
                        global_caller(
                            ComponentAddress::new_or_panic([
                                192, 48, 187, 194, 245, 43, 245, 53, 142, 133, 187, 178, 148, 238, 130, 8, 246, 157, 21, 84, 82, 155, 114, 91, 72, 236, 23, 248, 158, 205
                            ])
                        )
                    )
                );
            })
            .globalize()
        }

        pub fn add_voter(
            &mut self,
            voter: NonFungibleLocalId,
            key: String,
            amount: Decimal,
        ) -> (Epoch, u64) {
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
            (self.end, self.id)
        }

        pub fn get_amount_of(&mut self, voter: NonFungibleLocalId) -> (Epoch, Decimal, u64) {
            assert!(
                Runtime::current_epoch() >= self.end,
                "vote is not yet ended"
            );
            match self.voter.get_mut(&voter) {
                Some(mut value) => {
                    assert!(value.is_not_withdrawed, "{} already withdrawed", voter);
                    value.is_not_withdrawed = false;
                    (self.end, value.amount, self.id)
                }
                None => panic!("{} is not voted yet", voter),
            }
        }

        pub fn status(&mut self, status: bool) -> u64 {
            self.status = status;
            self.id
        }
    }
}

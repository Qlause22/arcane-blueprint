use scrypto::prelude::*;

#[blueprint]
mod arcane_vote_factory {
    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");
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
        keys_total_voter: HashMap<String, u128>,
        keys_total_commited_tokens: HashMap<String, u128>,
        keys: KeyValueStore<String, KeyValueStore<String, (Decimal, bool)>>,
    }

    impl ArcaneVoteFactory {
        pub fn instantiate(
            pict: Option<String>,
            description: String,
            nft_id: NonFungibleLocalId,
            quarter: u64,
            keys_vec: Vec<String>,
            member_badge: ResourceAddress,
        ) -> Global<ArcaneVoteFactory> {
            let keys: KeyValueStore<String, KeyValueStore<String, (Decimal, bool)>> =
                KeyValueStore::new();
            let mut key_value: HashMap<String, u128> = HashMap::new();
            for key in keys_vec.iter() {
                keys.insert(key.to_string(), KeyValueStore::new());
                key_value.insert(key.to_string(), 0u128);
            }
            Self {
                pict,
                description,
                keys_total_voter: key_value.to_owned(),
                keys_total_commited_tokens: key_value,
                end_at_epoch: quarter,
                keys,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(NonFungibleGlobalId::new(
                member_badge,
                nft_id,
            )))))
            .roles(roles!( main => rule!(require(CORE_BADGE.address()));))
            .globalize()
        }

        pub fn add_voter(&mut self, key: String, address: String, coins_amount: Decimal) -> u64 {
            assert!(
                Runtime::current_epoch().number() <= self.end_at_epoch,
                "vote closed"
            );
            assert!(self.keys.get(&key).is_some(), "Key does not exist");
            if let Some(val) = self.keys.get_mut(&key) {
                val.insert(address, (coins_amount, false));
            }
            self.end_at_epoch
        }

        pub fn get_amount_tokens_commited(
            &mut self,
            key: String,
            address: String,
        ) -> (Decimal, u64) {
            assert!(self.keys.get(&key).is_some(), "Key does not exist");
            assert!(
                Runtime::current_epoch().number() >= self.end_at_epoch,
                "withdraw not allowed at current epoch"
            );
            if let Some(mut data) = self.keys.get_mut(&key).unwrap().get_mut(&address) {
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

    // enable_function_auth! {
    //     new_vote => rule!(require(CORE_BADGE_ADDRESS));
    // }

    // struct ArcaneVoteFactory {
    //     member_resource_address: ResourceAddress,
    //     owner_id: NonFungibleLocalId,
    //     end_vote: Epoch,
    //     commited_token: Vault,
    //     votes: KeyValueStore<String, KeyValueStore<NonFungibleLocalId, (Decimal, bool)>>,
    // }

    // impl ArcaneVoteFactory {
    //     pub fn new_vote(
    //         member_resource_address: ResourceAddress,
    //         owner_id: NonFungibleLocalId,
    //         arcane_resource_address: ResourceAddress,
    //         end_vote: Epoch,
    //         vote_list: Vec<String>,
    //     ) -> Global<ArcaneVoteFactory> {
    //         let owner = rule!(require(NonFungibleGlobalId::new(
    //             member_resource_address,
    //             owner_id.clone(),
    //         )));
    //         let votes: KeyValueStore<String, KeyValueStore<NonFungibleLocalId, (Decimal, bool)>> =
    //             KeyValueStore::new();
    //         for vote in vote_list.iter() {
    //             votes.insert(vote.to_owned(), KeyValueStore::new());
    //         }
    //         let commited_token_vault = Vault::new(arcane_resource_address);
    //         Self {
    //             member_resource_address,
    //             owner_id,
    //             commited_token: commited_token_vault,
    //             end_vote,
    //             votes,
    //         }
    //         .instantiate()
    //         .prepare_to_globalize(OwnerRole::Fixed(owner))
    //         .globalize()
    //     }

    //     pub fn vote(&mut self, nft: Proof, vote_key: String, commit_coin: Bucket) {
    //         let nft_ticket = nft
    //             .check_with_message(self.member_resource_address, "invalid resource address")
    //             .as_non_fungible();

    //         match self.votes.get_mut(&vote_key) {
    //             Some(vote) => {
    //                 if vote.get(&nft_ticket.non_fungible_local_id()).is_some() {
    //                     panic!("address already votes");
    //                 };
    //                 vote.insert(
    //                     nft_ticket.non_fungible_local_id(),
    //                     (commit_coin.amount(), false),
    //                 );
    //                 self.commited_token.put(commit_coin)
    //             }
    //             None => panic!("vote key not found"),
    //         }
    //     }

    //     pub fn withdraw(&mut self, nft: Proof, vote_key: String) -> Bucket {
    //         let nft_ticket = nft
    //             .check_with_message(self.member_resource_address, "invalid resource address")
    //             .as_non_fungible();

    //         match self.votes.get_mut(&vote_key) {
    //             Some(mut vote) => match vote.get_mut(&nft_ticket.non_fungible_local_id()) {
    //                 Some(mut amount_of_commited_token) => {
    //                     let (amount, is_withdrawed) = amount_of_commited_token.clone();
    //                     assert!(
    //                         !is_withdrawed,
    //                         "id {} has withdrawed the token",
    //                         nft_ticket.non_fungible_local_id()
    //                     );
    //                     amount_of_commited_token.1 = true;
    //                     self.commited_token.take(amount)
    //                 }
    //                 None => panic!("Address not found"),
    //             },
    //             None => panic!("vote key not found"),
    //         }
    //     }
    // }
}

use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_vote_factory {
    const ARC: ResourceManager =
        resource_manager!("resource_tdx_2_1tk2zhlv50l4nl5flx2qc2y0zavp65xwt8khufun3kmq7xh90896gvc");
    enable_function_auth! {
        new_vote => rule!(require(get_core_address()));
    }

    struct ArcaneVoteFactory {
        member_resource_address: ResourceAddress,
        owner_id: NonFungibleLocalId,
        end_vote: Epoch,
        votes: KeyValueStore<String, KeyValueStore<NonFungibleLocalId, (Decimal, Vault)>>,
    }

    impl ArcaneVoteFactory {
        pub fn new_vote(
            member_resource_address: ResourceAddress,
            owner_id: NonFungibleLocalId,
            end_vote: Epoch,
            vote_list: Vec<String>,
        ) -> Global<ArcaneVoteFactory> {
            let owner = rule!(require(NonFungibleGlobalId::new(
                member_resource_address,
                owner_id.clone(),
            )));
            let votes: KeyValueStore<String, KeyValueStore<NonFungibleLocalId, (Decimal, Vault)>> =
                KeyValueStore::new();
            for vote in vote_list.iter() {
                votes.insert(vote.to_owned(), KeyValueStore::new());
            }
            Self {
                member_resource_address,
                owner_id,
                end_vote,
                votes,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(owner))
            .globalize()
        }

        pub fn vote(&mut self, nft: Proof, vote_key: String, commit_coin: Bucket) {
            let nft_ticket = nft
                .check_with_message(self.member_resource_address, "invalid resource address")
                .as_non_fungible();
            if let Some(voter_list) = self.votes.get_mut(&vote_key).as_deref_mut() {
                if voter_list
                    .get(&nft_ticket.non_fungible_local_id())
                    .is_some()
                {
                    let mut voter = voter_list
                        .get_mut(&nft_ticket.non_fungible_local_id())
                        .unwrap();
                    voter.0 += commit_coin.amount();
                    voter.1.put(commit_coin);
                } else {
                    voter_list.insert(
                        nft_ticket.non_fungible_local_id().clone(),
                        (commit_coin.amount(), Vault::with_bucket(commit_coin)),
                    );
                };
            } else {
                panic!("No Kind of Vote");
            };
        }

        pub fn withdraw(&mut self, nft: Proof, vote_key: String) -> (Bucket, Bucket) {
            let nft_ticket = nft
                .check_with_message(self.member_resource_address, "invalid resource address")
                .as_non_fungible();
            if let Some(voter_list) = self.votes.get_mut(&vote_key).as_deref_mut() {
                match voter_list
                    .get_mut(&nft_ticket.non_fungible_local_id())
                    .as_mut()
                {
                    Some(voter) => {
                        assert!(!voter.1.is_empty(), "user already withdraw");
                        (voter.1.take_all(), ARC.mint(dec!(3.829021412)))
                    }
                    None => panic!("NFT Not Registered"),
                }
            } else {
                panic!("No Kind of Vote");
            }
        }
    }
}

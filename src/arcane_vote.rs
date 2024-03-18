use crate::arcane_core_data::arcane_core_data::*;
use crate::arcane_reward_vault::arcane_reward_vault::*;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_vote_factory {
    const ARC: ResourceManager =
        resource_manager!("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");

    enable_function_auth! {
        instantiate => rule!(require(ResourceAddress::new_or_panic([
            154, 72, 172, 44, 200, 219, 182, 209, 221, 12, 183, 175, 159, 37, 117, 130, 93, 109, 124, 88, 13, 155, 199, 43, 109, 49, 205, 204, 60, 58
        ])));
    }

    struct ArcaneVoteFactory {
        self_address: ComponentAddress,
        vote_badge: Vault,
        commited_token: Vault,
        end_at_epoch: u64,
        member_rs: ResourceManager,
        description: String,
        pict: Option<String>,
        arcane_data: Global<ArcaneCoreData>,
        arcane_vault: Global<ArcaneVault>,
        keys: KeyValueStore<String, (u128, Decimal)>,
        voter: KeyValueStore<NonFungibleLocalId, (String, Decimal, bool)>,
        status: bool,
    }

    impl ArcaneVoteFactory {
        pub fn instantiate(
            pict: Option<String>,
            description: String,
            nft_id: NonFungibleLocalId,
            keys: Vec<String>,
            component_badge: Bucket,
            quarter: u64,
            member_rs: ResourceAddress,
            arcane_data_component_address: ComponentAddress,
            arcane_vault: ComponentAddress,
        ) -> Global<ArcaneVoteFactory> {
            let (address_reservation, self_address) =
                Runtime::allocate_component_address(ArcaneVoteFactory::blueprint_id());
            let voter: KeyValueStore<NonFungibleLocalId, (String, Decimal, bool)> =
                KeyValueStore::new();
            let keys_new: KeyValueStore<String, (u128, Decimal)> = KeyValueStore::new();
            let commited_token: Vault = Vault::new(ARC.address());
            for key in keys.iter() {
                keys_new.insert(key.to_string(), (0u128, dec!(0)));
            }
            Self {
                self_address,
                pict,
                voter,
                description,
                member_rs: ResourceManager::from_address(member_rs),
                commited_token,
                arcane_data: arcane_data_component_address.into(),
                vote_badge: Vault::with_bucket(component_badge),
                arcane_vault: arcane_vault.into(),
                keys: keys_new,
                end_at_epoch: quarter,
                status: false,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(NonFungibleGlobalId::new(
                member_rs, nft_id,
            )))))
            .with_address(address_reservation)
            .globalize()
        }

        pub fn vote(&mut self, nft_proof: Proof, key: String, commited_coin: Bucket) {
            assert!(self.status, "vote is not activated yet");
            let checked_nft_id = nft_proof
                .check_with_message(self.member_rs.address(), "please provide valid proof")
                .as_non_fungible()
                .non_fungible_local_id();
            assert!(
                Runtime::current_epoch().number() <= self.end_at_epoch,
                "vote closed"
            );

            assert!(
                self.voter.get(&checked_nft_id).is_none(),
                "address already voted"
            );

            if let Some(mut selected_key) = self.keys.get_mut(&key) {
                selected_key.0 += 1;
                selected_key.1 += commited_coin.amount();
                self.voter
                    .insert(checked_nft_id.clone(), (key, commited_coin.amount(), true));
                self.vote_badge.as_fungible().authorize_with_amount(1, || {
                    self.arcane_data.vote_and_update_data(
                        self.end_at_epoch,
                        commited_coin.amount(),
                        self.self_address,
                        checked_nft_id,
                    );
                })
            } else {
                panic!("Key does not exist");
            }
            self.commited_token.put(commited_coin)
        }

        pub fn withdraw(&mut self, nft_proof: Proof) -> (Bucket, Bucket) {
            let checked_nft_id = nft_proof
                .check_with_message(self.member_rs.address(), "please provide valid proof")
                .as_non_fungible()
                .non_fungible_local_id();
            assert!(
                Runtime::current_epoch().number() >= self.end_at_epoch,
                "withdraw not allowed at current epoch"
            );
            assert!(
                self.voter.get(&checked_nft_id).is_some(),
                "this id not voted yet"
            );
            if let Some(mut data) = self.voter.get_mut(&checked_nft_id) {
                assert!(data.2, "address already withdrawed its tokens");
                data.2 = false;

                let (total_token_commited_at_quater, reward_rate) =
                    self.vote_badge.as_fungible().authorize_with_amount(1, || {
                        self.arcane_data.withdraw_and_update_data(
                            self.end_at_epoch,
                            checked_nft_id.clone(),
                            self.self_address,
                        )
                    });
                let new_reward: Decimal = reward_rate * (data.1 / total_token_commited_at_quater);
                let reward_bucket = self
                    .vote_badge
                    .as_fungible()
                    .authorize_with_amount(1, || self.arcane_vault.take_reward(new_reward));
                (self.commited_token.take(data.1), reward_bucket)
            } else {
                panic!("address not voted yet")
            }
        }

        pub fn activate(&mut self, nft_proof: Proof) {
            let checked_nft_id = nft_proof
                .check_with_message(self.member_rs.address(), "please provide valid proof")
                .as_non_fungible()
                .non_fungible_local_id();
            let nft_data: ArcaneNFT = self.member_rs.get_non_fungible_data(&checked_nft_id);

            match nft_data.role {
                Role::Admin => {
                    self.status = true;
                }
                _ => panic!("role must be admin"),
            }
        }
    }
}

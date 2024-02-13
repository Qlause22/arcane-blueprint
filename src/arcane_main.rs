use crate::arcane_resource::arcane_resources::*;
use crate::arcane_vote::arcane_vote_factory::*;
use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_main {

    struct ArcaneMain {
        owner: NonFungibleVault,
        arcane_resources: Owned<ArcaneResources>,
        genesis_epoch: Epoch,
        const_per_quarter: u64,
        total_stake_at_quarter: HashMap<Epoch, Decimal>,
    }

    impl ArcaneMain {
        pub fn get_core_badge() -> Bucket {
            ResourceBuilder::new_ruid_non_fungible::<ArcaneCoreData>(OwnerRole::None).metadata(metadata! {
                init {
                    "name" => "Arcane Core Badge", locked;
                    "description" => "this NFT grant access to All of Arcane Labyrinth's features", locked;
                }
            }).mint_initial_supply([ArcaneCoreData { reward  : dec!(0) }, ArcaneCoreData { reward  : dec!(0) }]).into()
        }

        pub fn instantiate(core_nft: Bucket) -> Global<ArcaneMain> {
            let arcane_resources = core_nft.as_non_fungible().authorize_with_non_fungibles(
                &core_nft.as_non_fungible().non_fungible_local_ids(),
                || ArcaneResources::create(),
            );
            let owner = NonFungibleVault::with_bucket(core_nft.as_non_fungible());
            let const_per_quarter = 32 as u64;
            let genesis_epoch = Runtime::current_epoch();
            let current_quarter = Epoch::of(
                (genesis_epoch.number() - Runtime::current_epoch().number())
                    .div_ceil(const_per_quarter),
            );
            let mut total_stake_at_quarter: HashMap<Epoch, Decimal> = HashMap::new();
            total_stake_at_quarter.insert(current_quarter, Decimal::from(0));
            total_stake_at_quarter.insert(
                Epoch::of(current_quarter.number().wrapping_add(1)),
                Decimal::from(0),
            );
            total_stake_at_quarter.insert(
                Epoch::of(current_quarter.number().wrapping_add(2)),
                Decimal::from(0),
            );
            Self {
                owner,
                arcane_resources,
                genesis_epoch,
                const_per_quarter,
                total_stake_at_quarter,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn sign_up(&self) -> Bucket {
            self.arcane_resources.mint_member()
        }

        pub fn make_admin(&self) -> Bucket {
            self.arcane_resources.mint_admin()
        }

        pub fn create_vote(
            &self,
            nft: Proof,
            quarter: u64,
            vote_list: Vec<String>,
        ) -> Global<ArcaneVoteFactory> {
            let nft_ticket = nft
                .check_with_message(
                    self.arcane_resources.get_member_resource_address(),
                    "invalid resource address",
                )
                .as_non_fungible();

            let end_vote = Epoch::of(
                quarter
                    .wrapping_add(self.get_current_quarter())
                    .wrapping_mul(self.const_per_quarter),
            );
            ArcaneVoteFactory::new_vote(
                nft_ticket.resource_address(),
                nft_ticket.non_fungible_local_id(),
                end_vote,
                vote_list,
            )
        }

        fn get_current_quarter(&self) -> u64 {
            Runtime::current_epoch()
                .number()
                .div_ceil(self.const_per_quarter)
        }
    }
}

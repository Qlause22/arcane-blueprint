use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_badge_manager {

    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_tdx_2_1nfy2ctxgmwmdrhgvk7he7ft4sfwk6lzcpkduw2mdx8xuc0p6uny7rn");

    struct ArcaneBadgeManager {}

    impl ArcaneBadgeManager {
        pub fn instantiate(
            main_component_address: ComponentAddress,
        ) -> (ResourceAddress, ResourceAddress) {
            let vote_component_badge = ResourceBuilder::new_fungible(OwnerRole::Fixed(rule!(
                require(CORE_BADGE.address())
            )))
            .metadata(metadata! {
                init {
                    "name" => "Vote Component Badge", locked;
                    "description" => "This badge grant access to other component's methods", locked;
                }
            })
            .divisibility(0)
            .mint_roles(mint_roles! {
                minter => rule!(require(require(global_caller(main_component_address))));
                minter_updater => rule!(require(CORE_BADGE.address()));
            })
            .deposit_roles(deposit_roles! {
                depositor => rule!(allow_all);
                depositor_updater => rule!(require(CORE_BADGE.address()));
            })
            .recall_roles(recall_roles! {
                recaller => rule!(deny_all);
                recaller_updater => rule!(require(CORE_BADGE.address()));
            })
            .withdraw_roles(withdraw_roles! {
                withdrawer => rule!(deny_all);
                withdrawer_updater => rule!(require(CORE_BADGE.address()));
            })
            .burn_roles(burn_roles! {
                burner => rule!(deny_all);
                burner_updater => rule!(require(CORE_BADGE.address()));
            })
            .create_with_no_initial_supply();

            let arcane_nft_badge = ResourceBuilder::new_ruid_non_fungible::<ArcaneNFT>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Badge", locked;
                        "description" => "this NFT grant access to Arcane Labyrinth's member or admin pages", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(require(CORE_BADGE.address()));
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(require(CORE_BADGE.address()));
                })            
                .recall_roles(recall_roles! {
                    recaller => rule!(require(CORE_BADGE.address()));
                    recaller_updater => rule!(require(CORE_BADGE.address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(CORE_BADGE.address()));
                })
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(CORE_BADGE.address()));
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(global_caller(main_component_address)) || require((vote_component_badge.address())));
                    non_fungible_data_updater_updater => rule!(require(CORE_BADGE.address()));
                })
                .create_with_no_initial_supply();

            (vote_component_badge.address(), arcane_nft_badge.address())
        }
    }
}

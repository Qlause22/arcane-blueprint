use scrypto::prelude::*;
use crate::utils::*;

#[blueprint]
#[types(ArcaneNFT)]
mod resource_manager {
    
    struct ArcaneResourceManager {}
    
    impl ArcaneResourceManager {
        pub fn instantiate(
            main_component_address: ComponentAddress,
            core: ResourceAddress) -> ResourceManager {

            ResourceBuilder::new_integer_non_fungible_with_registered_type::<ArcaneNFT>(OwnerRole::Fixed(rule!(require(core))))
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Badge", locked;
                        "description" => "this NFT grant access to Arcane Labyrinth's member or admin pages", locked;
                        "icon_url" => Url::of("https://i.ibb.co/2vtP4Kr/arcane.jpg"), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(main_component_address)));
                    minter_updater => rule!(require(core));
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(require(core));
                })            
                .recall_roles(recall_roles! {
                    recaller => rule!(require(core));
                    recaller_updater => rule!(require(core));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(core));
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(main_component_address)));
                    burner_updater => rule!(require(core));
                })
                .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                    non_fungible_data_updater => rule!(require(global_caller(main_component_address)));
                    non_fungible_data_updater_updater => rule!(require(core));
                })
                .create_with_no_initial_supply()
        }
    }
}

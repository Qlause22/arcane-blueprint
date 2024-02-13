use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_resources {
    enable_function_auth! {
        create => rule!(require(get_core_address()));
    }

    struct ArcaneResources {
        admin_badges_resource_manager: ResourceManager,
        member_badges_resource_manager: ResourceManager,
    }

    impl ArcaneResources {
        pub fn create() -> Owned<ArcaneResources> {
            let admin_badges_resource_manager = ResourceBuilder::new_ruid_non_fungible::<ArcaneAdminData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Admin Badge", locked;
                        "description" => "this NFT grant access to admin page of Arcane Labyrinth", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(get_core_address()));
                    minter_updater => rule!(require(get_core_address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(get_core_address()));
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(require(get_core_address()));
                    recaller_updater => rule!(require(get_core_address()));
                })
                .withdraw_roles(
                    withdraw_roles! {
                        withdrawer => rule!(deny_all);
                        withdrawer_updater => rule!(require(get_core_address()));
                    }
                )
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(get_core_address()));
                }).create_with_no_initial_supply();

            let member_badges_resource_manager = ResourceBuilder::new_ruid_non_fungible::<ArcaneMemberData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Member Badge", locked;
                        "description" => "this NFT grant access to member page of Arcane Labyrinth", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(require(get_core_address()));
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(require(get_core_address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(get_core_address()));
                })
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(get_core_address()));
                })
                .create_with_no_initial_supply();

            Self {
                admin_badges_resource_manager,
                member_badges_resource_manager,
            }
            .instantiate()
        }

        pub fn mint_member(&mut self) -> Bucket {
            self.member_badges_resource_manager
                .mint_ruid_non_fungible(ArcaneMemberData { reward: dec!(0) })
        }
        pub fn get_member_resource_address(&mut self) -> ResourceAddress {
            self.member_badges_resource_manager.address()
        }
        pub fn mint_admin(&mut self) -> Bucket {
            self.admin_badges_resource_manager
                .mint_ruid_non_fungible(ArcaneAdminData { reward: dec!(0) })
        }
    }
}

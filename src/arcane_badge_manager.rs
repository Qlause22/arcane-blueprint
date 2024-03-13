use crate::utils::*;
use scrypto::prelude::*;

#[blueprint]
mod arcane_badge_manager {

    const CORE_BADGE: ResourceManager =
        resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");

    struct ArcaneBadgeManager {
        admin_badges_resource_manager: ResourceManager,
        member_badges_resource_manager: ResourceManager,
    }

    impl ArcaneBadgeManager {
        pub fn instantiate(
            main_component_address: ComponentAddress,
        ) -> (Owned<ArcaneBadgeManager>, ResourceAddress, ResourceAddress) {
            let admin_badges_resource_manager = ResourceBuilder::new_ruid_non_fungible::<ArcaneAdminData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Admin Badge", locked;
                        "description" => "this NFT grant access to admin page of Arcane Labyrinth", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(CORE_BADGE.address()));
                    minter_updater => rule!(require(CORE_BADGE.address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(CORE_BADGE.address()));
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(require(CORE_BADGE.address()));
                    recaller_updater => rule!(require(CORE_BADGE.address()));
                })
                .withdraw_roles(
                    withdraw_roles! {
                        withdrawer => rule!(deny_all);
                        withdrawer_updater => rule!(require(CORE_BADGE.address()));
                    }
                )
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(CORE_BADGE.address()));
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
                    minter_updater => rule!(require(CORE_BADGE.address()));
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(require(CORE_BADGE.address()));
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
                    non_fungible_data_updater => rule!(require(global_caller(main_component_address)));
                    non_fungible_data_updater_updater => rule!(require(CORE_BADGE.address()));
                })
                .create_with_no_initial_supply();

            (
                Self {
                    admin_badges_resource_manager,
                    member_badges_resource_manager,
                }
                .instantiate(),
                admin_badges_resource_manager.address(),
                member_badges_resource_manager.address(),
            )
        }

        pub fn mint_member(
            &mut self,
            address: String,
            component_store: ComponentAddress,
        ) -> Bucket {
            self.member_badges_resource_manager
                .mint_ruid_non_fungible(ArcaneMemberData {
                    owned_component: component_store,
                    owner: address,
                    reward: dec!(0),
                })
        }
        //     pub fn get_member_resource_address(&mut self) -> ResourceAddress {
        //         self.member_badges_resource_manager.address()
        //     }
        //     pub fn mint_admin(&mut self) -> Bucket {
        //         self.admin_badges_resource_manager
        //             .mint_ruid_non_fungible(ArcaneAdminData { reward: dec!(0) })
        //     }
    }
}

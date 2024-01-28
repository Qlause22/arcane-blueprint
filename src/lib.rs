use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct ArcaneAdminData {
    #[mutable]
    data: Option<KeyValueStore<String, String>>,
}

#[derive(ScryptoSbor, NonFungibleData)]
struct ArcaneMemberData {
    #[mutable]
    data: Option<KeyValueStore<String, String>>,
}

#[derive(ScryptoSbor, NonFungibleData)]
struct ArcaneCoreData {
    #[mutable]
    data: Option<KeyValueStore<String, String>>,
}

#[blueprint]
mod arcane_resources {
    enable_method_auth! {
        roles {
            core => updatable_by: [];
        },
        methods {
            mint_member => PUBLIC;
            mint_admin => restrict_to : [core] ;
        }
    }

    struct ArcaneResources {
        admin_badges_resource_manager: ResourceManager,
        member_badges_resource_manager: ResourceManager,
    }

    impl ArcaneResources {
        pub fn instantiate() -> (Global<ArcaneResources>, NonFungibleBucket) {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(ArcaneResources::blueprint_id());
            let core_badge = ResourceBuilder::new_ruid_non_fungible::<ArcaneCoreData>(OwnerRole::None).metadata(metadata! {
                init {
                    "name" => "Arcane Core Badge", locked;
                    "description" => "this NFT grant access to All of Arcane Labyrinth's features", locked;
                }
            }).mint_initial_supply([ArcaneCoreData { data: None }]);

            let admin_badges_resource_manager = ResourceBuilder::new_ruid_non_fungible::<ArcaneAdminData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Arcane Admin Badge", locked;
                        "description" => "this NFT grant access to admin page of Arcane Labyrinth", locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(require(core_badge.resource_address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(core_badge.resource_address()));
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(require(core_badge.resource_address()));
                    recaller_updater => rule!(require(core_badge.resource_address()));
                })
                .withdraw_roles(
                    withdraw_roles! {
                        withdrawer => rule!(deny_all);
                        withdrawer_updater => rule!(require(core_badge.resource_address()));
                    }
                )
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(core_badge.resource_address()));
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
                    minter_updater => rule!(require(core_badge.resource_address()));
                })
                .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(require(core_badge.resource_address()));
                })
                .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(require(core_badge.resource_address()));
                })
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(require(core_badge.resource_address()));
                })
                .create_with_no_initial_supply();

            (
                Self {
                    admin_badges_resource_manager,
                    member_badges_resource_manager,
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
                .roles(roles!(
                    core => rule!(require(core_badge.resource_address()));
                ))
                .globalize(),
                core_badge,
            )
        }

        pub fn mint_member(&mut self) -> Bucket {
            self.member_badges_resource_manager
                .mint_ruid_non_fungible(ArcaneMemberData { data: None })
        }
        pub fn mint_admin(&mut self) -> Bucket {
            self.admin_badges_resource_manager
                .mint_ruid_non_fungible(ArcaneAdminData { data: None })
        }
    }
}

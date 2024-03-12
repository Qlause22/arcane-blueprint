use scrypto::prelude::*;
#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneAdminData {
    #[mutable]
    pub reward: Decimal,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneMemberData {
    pub owned_component: ComponentAddress,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneCoreData {
    #[mutable]
    pub reward: Decimal,
}

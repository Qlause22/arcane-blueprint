use scrypto::prelude::*;
#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneAdminData {
    pub owner: String,
    #[mutable]
    pub reward: Decimal,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneMemberData {
    pub owner: String,
    pub owned_component: ComponentAddress,
    #[mutable]
    pub reward: Decimal,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneCoreData {
    pub owner: String,
    #[mutable]
    pub reward: Decimal,
}

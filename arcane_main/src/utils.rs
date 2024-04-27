use scrypto::prelude::*;

#[derive(ScryptoSbor, PartialEq)]
pub enum Role {
    Member,
    Admin,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneNFT {
    pub id: u64,
    pub owner: ComponentAddress,
    #[mutable]
    pub role: Role,
}


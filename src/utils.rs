use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum Role {
    Member,
    Admin,
}

#[derive(ScryptoSbor)]
pub enum VoteKind {
    Voted,
    Withdrawed,
    Owned,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneNFT {
    #[mutable]
    pub role: Role,
    #[mutable]
    pub reward: Decimal,
}

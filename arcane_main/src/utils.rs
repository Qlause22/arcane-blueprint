use scrypto::prelude::*;

pub const CORE_BADGE: ResourceAddress = XRD;

// pub const XRD_STOKENET: ResourceAddress = ResourceAddress::new_or_panic([
//     93, 166, 99, 24, 198, 49, 140, 97, 245, 166, 27, 76, 99, 24, 198, 49, 140, 247, 148, 170, 141,
//     41, 95, 20, 230, 49, 140, 99, 24, 198,
// ]);

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneRegisterEvent(pub u64);

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneWithdrawEvent {
    pub component_id: u64,
    pub address_id: NonFungibleLocalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneCreateVoteEvent {
    pub id: u64,
    pub url: String,
    pub keys: Vec<String>,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneVoteEvent {
    pub component_id: u64,
    pub address_id: NonFungibleLocalId,
    pub key: String,
    pub amount: Decimal,
}

#[derive(ScryptoSbor)]
pub enum Role {
    Member,
    Admin,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneNFT {
    pub id: u64,
    #[mutable]
    pub role: Role,
}

#[derive(ScryptoSbor)]
pub struct ID {
    pub component_id: u64,
    pub member_id: u64,
}

#[derive(ScryptoSbor)]
pub struct VoterData {
    pub key: String,
    pub amount: Decimal,
    pub is_not_withdrawed: bool,
}

#[derive(ScryptoSbor)]
pub struct KeyData {
    pub voters: u16,
    pub amounts: Decimal,
}

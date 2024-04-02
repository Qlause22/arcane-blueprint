use scrypto::prelude::*;

pub const CORE_BADGE: ResourceAddress = XRD;

pub const MAIN: ComponentAddress = ComponentAddress::new_or_panic([
    192, 93, 202, 187, 26, 26, 31, 221, 159, 229, 8, 231, 117, 124, 64, 181, 181, 126, 103, 80, 88,
    129, 118, 154, 167, 24, 153, 155, 43, 123,
]);

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

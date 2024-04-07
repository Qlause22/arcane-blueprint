use scrypto::prelude::*;

pub const MAIN: ComponentAddress = ComponentAddress::new_or_panic([
    192, 206, 133, 30, 225, 53, 204, 227, 80, 10, 175, 65, 238, 33, 97, 107, 198, 180, 82, 103,
    190, 67, 226, 200, 165, 117, 53, 93, 20, 35,
]); // component_tdx_2_1cr8g28hpxhxwx5q24aq7ugtpd0rtg5n8hep79j99w56469prsgdqh3
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneRegisterEvent {
    pub id: u64,
    pub address: ComponentAddress,
}

#[derive(ScryptoSbor)]
pub struct State {
    pub total_token: KeyValueStore<Epoch, Decimal>,
    pub package: KeyValueStore<PackageAddress, bool>,
    pub vote: KeyValueStore<ComponentAddress, bool>,
    pub member: KeyValueStore<ComponentAddress, bool>,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneWithdrawEvent {
    pub component_id: u64,
    pub address_id: NonFungibleLocalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct ArcaneCreateVoteEvent {
    pub id: u64,
    pub voter: NonFungibleLocalId,
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
    pub owner: ComponentAddress,
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

use scrypto::prelude::*;

pub fn get_core_address() -> ResourceAddress {
    ResourceAddress::try_from_bech32(
        &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
        &String::from("resource_tdx_2_1ntv3qkq2vrmjmga076y6jt0nedruhm5pn94nej08mf4cd2gf8gffra"),
    )
    .unwrap()
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneAdminData {
    #[mutable]
    pub reward: Decimal,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneMemberData {
    #[mutable]
    pub reward: Decimal,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ArcaneCoreData {
    #[mutable]
    pub reward: Decimal,
}

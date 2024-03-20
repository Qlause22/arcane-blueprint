use scrypto::prelude::*;

pub fn get_core_address() -> ResourceAddress {
    ResourceAddress::try_from_bech32(
        &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
        &String::from("resource_tdx_2_1n20fntprpwav2hssr4xtuyljdcrnl7pkhl4l4f8l09shjm9gs5ywr8"),
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

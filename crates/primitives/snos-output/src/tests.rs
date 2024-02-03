use mp_messages::conversions::eth_address_to_felt;
use starknet_api::api_core::EthAddress;
use starknet_api::hash::StarkFelt;

use crate::codec::SnosCodec;
use crate::felt_reader::FeltReader;
use crate::StarknetOsOutput;

// Starknet::update_state sample invocation from mainnet
// https://etherscan.io/tx/0x9a6f9ee53f0b558f466d4340613740b9483e10c230313aa9c31fd0ba80f1a40f
//
// Calldata:
// "0x0000000000000000000000000000000000000000000000000000000000000060",
// programOutput offset (96
// bytes) "0x64f464be0437d366556e4fe7cfc0fc8d2eec0ed531050137ca44052de9c97219",
// onchainDataHash
// "0x0000000000000000000000000000000000000000000000000000000000000816",
// onchainDataSize
// "0x0000000000000000000000000000000000000000000000000000000000000016",
// programOutput length
const SNOS_PROGRAM_OUTPUT_HEX: &str = "\
    00bf8721ac2af6f7f40155c973c2bf5c15b7e0ed790b0865af20bf25ab57e9ff\
    03d46b43f31ccfed7ce09a0e318f1f98f59b28f4527ea01de34382ab8c7f2a26\
    00000000000000000000000000000000000000000000000000000000000441ee\
    0770ab05ba02edc49516cebde84bbffb76da74cdc98fd142c9c703ab871c4c7a\
    017c0bc29d31e9a7d14671610a7626264ce9ce8e3ed066a4775adf9b123de9dd\
    0000000000000000000000000000000000000000000000000000000000000007\
    073314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82\
    000000000000000000000000ae0ee0a63a2ce6baeeffe56e7714fb4efe48d419\
    0000000000000000000000000000000000000000000000000000000000000004\
    0000000000000000000000000000000000000000000000000000000000000000\
    000000000000000000000000def47ac573dd080526c2e6dd3bc8b4d66e9c6a77\
    00000000000000000000000000000000000000000000000000009184e72a0000\
    0000000000000000000000000000000000000000000000000000000000000000\
    0000000000000000000000000000000000000000000000000000000000000008\
    000000000000000000000000ae0ee0a63a2ce6baeeffe56e7714fb4efe48d419\
    073314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82\
    000000000000000000000000000000000000000000000000000000000014de2c\
    02d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5\
    0000000000000000000000000000000000000000000000000000000000000003\
    015342c9b50c5eed063ef19efb9a57ad10c30d1d39f1f1977f48bcc7199e91e0\
    0000000000000000000000000000000000000000000000000429d069189e0000\
    0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn test_snos_output_codec() {
    let output_bytes = hex::decode(SNOS_PROGRAM_OUTPUT_HEX).unwrap();
    let output: Vec<StarkFelt> = output_bytes.chunks(32).map(|chunk| StarkFelt(chunk.try_into().unwrap())).collect();
    let mut reader = FeltReader::new(&output);

    let snos_output = StarknetOsOutput::decode(&mut reader).unwrap();

    let mut actual: Vec<u8> = Vec::new();
    snos_output.into_encoded_vec().into_iter().for_each(|felt| actual.extend_from_slice(felt.0.as_slice()));

    pretty_assertions::assert_eq!(output_bytes, actual);
}

#[test]
fn test_eth_address_cast() {
    let felt = StarkFelt::try_from("0x000000000000000000000000ae0ee0a63a2ce6baeeffe56e7714fb4efe48d419").unwrap();
    let eth_address = EthAddress::try_from(felt).unwrap();
    let actual = eth_address_to_felt(&eth_address);
    assert_eq!(felt, actual);
}

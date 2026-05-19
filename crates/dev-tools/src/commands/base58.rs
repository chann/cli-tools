use anyhow::Result;
use crate::commands::encoding::{encode_data, decode_data};

pub fn encode_base58(input: &str, is_file: bool) -> Result<()> {
    encode_data(
        input,
        is_file,
        |data| bs58::encode(data).into_string(),
        "Base58",
    )
}

pub fn decode_base58(input: &str, output_file: Option<String>) -> Result<()> {
    decode_data(
        input,
        output_file,
        |s| bs58::decode(s).into_vec().ok(),
        "Failed to decode Base58. Ensure input is valid base58.",
    )
}

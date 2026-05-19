use anyhow::Result;
use base32::{Alphabet, encode, decode};
use crate::commands::encoding::{encode_data, decode_data};

pub fn encode_base32(input: &str, is_file: bool) -> Result<()> {
    encode_data(
        input,
        is_file,
        |data| encode(Alphabet::Rfc4648 { padding: true }, data),
        "Base32",
    )
}

pub fn decode_base32(input: &str, output_file: Option<String>) -> Result<()> {
    decode_data(
        input,
        output_file,
        |s| decode(Alphabet::Rfc4648 { padding: true }, s),
        "Failed to decode base32. Ensure input is valid RFC4648 base32.",
    )
}

use anyhow::Result;
use ascii85::{encode, decode};
use crate::commands::encoding::{encode_data, decode_data};

pub fn encode_base85(input: &str, is_file: bool) -> Result<()> {
    encode_data(
        input,
        is_file,
        |data| encode(data),
        "Base85",
    )
}

pub fn decode_base85(input: &str, output_file: Option<String>) -> Result<()> {
    decode_data(
        input,
        output_file,
        |s| decode(s).ok(),
        "Failed to decode Base85. Ensure input is valid base85.",
    )
}

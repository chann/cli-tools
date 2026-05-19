use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};

pub fn gzip(input: &str, output: &str) -> Result<()> {
    let mut input_file = BufReader::new(File::open(input)?);
    let output_file = File::create(output)?;
    let mut encoder = GzEncoder::new(BufWriter::new(output_file), Compression::default());
    copy(&mut input_file, &mut encoder)?;
    encoder.finish()?;
    println!("Compressed {} to {}", input, output);
    Ok(())
}

pub fn gunzip(input: &str, output: &str) -> Result<()> {
    let input_file = File::open(input)?;
    let mut decoder = GzDecoder::new(BufReader::new(input_file));
    let mut output_file = BufWriter::new(File::create(output)?);
    copy(&mut decoder, &mut output_file)?;
    println!("Decompressed {} to {}", input, output);
    Ok(())
}

use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::io::{Read, Result, Write};

pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut c = DeflateEncoder::new(Vec::new(), Compression::best());
    c.write_all(data)?;
    c.finish()
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

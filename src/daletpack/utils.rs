use std::io::{self, Read};
use zstd::stream::read::Decoder;

pub fn compress_zstd(data: &[u8]) -> io::Result<Vec<u8>> {
    zstd::bulk::compress(data, 22)
}

pub fn decompress_zstd(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = Decoder::new(data)?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

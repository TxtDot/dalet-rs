use crate::typed::Page;

use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::io::{Read, Write};

pub fn compress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut c = DeflateEncoder::new(Vec::new(), Compression::best());
    c.write_all(data)?;
    c.finish()
}

pub fn decompress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

#[derive(Debug)]
pub enum SerializeError {
    Serialize(bitcode::Error),
    Compress(std::io::Error),
}

#[derive(Debug)]
pub enum DeserializeError {
    Deserialize(bitcode::Error),
    Decompress(std::io::Error),
}

pub fn serialize(page: Page) -> Result<Vec<u8>, SerializeError> {
    match bitcode::serialize(&page) {
        Ok(r) => match compress(&r) {
            Ok(r) => Ok(r),
            Err(e) => Err(SerializeError::Compress(e)),
        },
        Err(e) => Err(SerializeError::Serialize(e)),
    }
}

pub fn deserialize(data: &[u8]) -> Result<Page, DeserializeError> {
    match decompress(data) {
        Ok(r) => match bitcode::deserialize(&r) {
            Ok(r) => Ok(r),
            Err(e) => Err(DeserializeError::Deserialize(e)),
        },
        Err(e) => Err(DeserializeError::Decompress(e)),
    }
}

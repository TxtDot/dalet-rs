use std::u32::MAX;

pub fn compress_zstd(data: &[u8]) -> std::io::Result<Vec<u8>> {
    zstd::bulk::compress(data, 22)
}

pub fn decompress_zstd(data: &[u8]) -> std::io::Result<Vec<u8>> {
    zstd::bulk::decompress(data, MAX as usize)
}

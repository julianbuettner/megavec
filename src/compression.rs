use lz4_flex::{compress_prepend_size, decompress_size_prepended};

pub struct Lz4;

pub trait Compression {
    fn compress(block: &[u8]) -> Vec<u8>;
    fn decompress(block: &[u8]) -> Vec<u8>;
}

impl Compression for Lz4 {
    fn compress(block: &[u8]) -> Vec<u8> {
        compress_prepend_size(block)
    }
    fn decompress(block: &[u8]) -> Vec<u8> {
        decompress_size_prepended(block).expect("decompression should never fail")
    }
}

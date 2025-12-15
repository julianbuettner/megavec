use std::marker::PhantomData;

use bincode::{Decode, Encode};

use crate::compression::Compression;

pub(crate) struct CompressedBlock<T, C> {
    compressed_bytes: Vec<u8>,
    _marker: PhantomData<(T, C)>,
}

impl<C: Compression, T: Encode + Decode<()>> CompressedBlock<T, C> {
    pub fn new(v: &[T]) -> Self {
        let bytes = bincode::encode_to_vec(v, bincode::config::standard())
            .expect("all data should always be serializable");
        Self {
            compressed_bytes: C::compress(&bytes),
            _marker: PhantomData,
        }
    }
    pub fn decompress(&self) -> Vec<T> {
        let data = C::decompress(&self.compressed_bytes);
        bincode::decode_from_slice(&data, bincode::config::standard())
            .expect("all data should always be deserializable")
            .0
    }
}

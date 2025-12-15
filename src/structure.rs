mod inner_ops;
mod methods;


use bincode::{Decode, Encode};

use crate::{compressed_block::CompressedBlock, compression::Compression, Lz4};

pub struct Megavec<T, C = Lz4> {
    compressed_count: usize,
    target_block_size: usize,
    compressed_blocks: Vec<CompressedBlock<T, C>>,
    working_block: Vec<T>,
}

impl<T: Encode + Decode<()> + Clone, C: Compression> Megavec<T, C> {
    pub fn new(_compression: C, block_size: usize) -> Self {
        Self {
            compressed_count: 0,
            target_block_size: block_size,
            compressed_blocks: Vec::new(),
            working_block: Vec::new(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = T> + use<'_, T, C> {
        self.compressed_blocks
            .iter()
            .flat_map(|b| b.decompress())
            .chain(self.working_block.iter().cloned())
    }
}

use bincode::{Decode, Encode};

use crate::compression::Compression;

impl<C: Compression, T: Encode + Decode<()>> super::Megavec<T, C> {
    pub fn push(&mut self, elem: T) {
        self.working_block.push(elem);
        self.autoflush();
    }
    pub fn pop(&mut self) -> Option<T> {
        self.autodeflush();
        self.working_block.pop()
    }
    pub fn len(&self) -> usize {
        self.compressed_count + self.working_block.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

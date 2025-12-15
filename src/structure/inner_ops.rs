use bincode::{Decode, Encode};

use crate::{compressed_block::CompressedBlock, compression::Compression};

impl<T: Encode + Decode<()>, C: Compression> super::Megavec<T, C> {
    pub(crate) fn autoflush(&mut self) {
        if self.working_block.len() >= self.target_block_size {
            self.flush();
        }
    }

    fn flush(&mut self) {
        self.compressed_blocks
            .push(CompressedBlock::new(&self.working_block));
        self.compressed_count += self.working_block.len();
        self.working_block.clear();
    }

    pub(crate) fn autodeflush(&mut self) {
        if self.working_block.is_empty() {
            self.deflush();
        }
    }

    fn deflush(&mut self) {
        let Some(block) = self.compressed_blocks.pop() else {
            return;
        };
        let mut decompressed = block.decompress();
        self.compressed_count -= decompressed.len();
        decompressed.append(&mut self.working_block);
        self.working_block = decompressed;
    }
}

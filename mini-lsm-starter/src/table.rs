#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

use std::path::Path;
use std::sync::Arc;

use anyhow::{Error, Result};
pub use builder::SsTableBuilder;
use bytes::{Buf, Bytes};
pub use iterator::SsTableIterator;

use crate::block::Block;
use crate::lsm_storage::BlockCache;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockMeta {
    /// Offset of this data block.
    pub offset: usize,
    /// The first key of the data block.
    pub first_key: Bytes,
}

impl BlockMeta {
    /// Encode block meta to a buffer.
    pub fn encode_block_meta(
        block_meta: &[BlockMeta],
        #[allow(clippy::ptr_arg)] // remove this allow after you finish
        buf: &mut Vec<u8>,
    ) {
        unimplemented!()
    }

    /// Decode block meta from a buffer.
    pub fn decode_block_meta(buf: impl Buf) -> Vec<BlockMeta> {
        unimplemented!()
    }
}

/// A file object.
pub struct FileObject(Bytes);

impl FileObject {
    pub fn read(&self, offset: u64, len: u64) -> Result<Vec<u8>> {
        Ok(self.0[offset as usize..(offset + len) as usize].to_vec())
    }

    pub fn size(&self) -> u64 {
        self.0.len() as u64
    }

    /// Create a new file object (day 2) and write the file to the disk (day 4).
    pub fn create(path: &Path, data: Vec<u8>) -> Result<Self> {
        Ok(FileObject(Bytes::from(data)))
    }

    pub fn open(path: &Path) -> Result<Self> {
        Ok(FileObject(Bytes::new()))
    }
}

pub struct SsTable {
    file: FileObject,
    block_metas: Vec<BlockMeta>,
    block_meta_offset: usize,
}

impl SsTable {
    #[cfg(test)]
    pub(crate) fn open_for_test(file: FileObject) -> Result<Self> {
        Self::open(0, None, file)
    }

    /// Open SSTable from a file.
    pub fn open(id: usize, block_cache: Option<Arc<BlockCache>>, file: FileObject) -> Result<Self> {
        Ok(SsTable {
            file,
            block_metas: Vec::new(),
            block_meta_offset: 0,
        })
    }

    pub fn set_meta(&mut self, block_metas: Vec<BlockMeta>, block_meta_offset: usize) {
        self.block_metas = block_metas;
        self.block_meta_offset = block_meta_offset;
    }

    /// Read a block from the disk.
    pub fn read_block(&self, block_idx: usize) -> Result<Arc<Block>> {
        if block_idx >= self.block_metas.len() {
            return Err(Error::msg(format!(
                "block_idx : {} num_of_blocks : {}",
                block_idx,
                self.num_of_blocks()
            )));
        }
        let meta = &self.block_metas[block_idx];
        let next_meta_offset = {
            if block_idx + 1 == self.block_metas.len() {
                self.block_meta_offset
            } else {
                self.block_metas[block_idx + 1].offset
            }
        };
        match self
            .file
            .read(meta.offset as u64, (next_meta_offset - meta.offset) as u64)
        {
            Ok(data) => {
                return Ok(Arc::new(Block::decode(data.as_slice())));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    /// Read a block from disk, with block cache. (Day 4)
    pub fn read_block_cached(&self, block_idx: usize) -> Result<Arc<Block>> {
        unimplemented!()
    }

    /// Find the block that may contain `key`.
    pub fn find_block_idx(&self, key: &[u8]) -> usize {
        let mut low = 0;
        let mut high = self.num_of_blocks();
        while low < high {
            let mid = low + (low + high) / 2;
            match self.block_metas[mid].first_key.as_ref().cmp(&key) {
                std::cmp::Ordering::Equal => {
                    return low;
                }
                std::cmp::Ordering::Greater => {
                    high = mid;
                }
                std::cmp::Ordering::Less => {
                    low = mid + 1;
                }
            }
        }
        low
    }

    /// Get number of data blocks.
    pub fn num_of_blocks(&self) -> usize {
        self.block_metas.len()
    }
}

#[cfg(test)]
mod tests;

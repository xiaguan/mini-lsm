#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use bytes::{Buf, Bytes};

use super::{BlockMeta, FileObject, SsTable};
use crate::block::{Block, BlockBuilder};
use crate::lsm_storage::BlockCache;

/// Builds an SSTable from key-value pairs.
pub struct SsTableBuilder {
    pub(super) meta: Vec<BlockMeta>,
    // Add other fields you need.
    builders: Vec<BlockBuilder>,
    block_size: usize,
    offset: usize,
}

impl SsTableBuilder {
    /// Create a builder based on target block size.
    pub fn new(block_size: usize) -> Self {
        SsTableBuilder {
            meta: Vec::new(),
            builders: Vec::new(),
            block_size: block_size,
            offset: 0,
        }
    }

    fn append_new_builder(&mut self, key: &[u8]) {
        self.builders.push(BlockBuilder::new(self.block_size));
        self.meta.push(BlockMeta {
            offset: self.offset,
            first_key: Bytes::copy_from_slice(key),
        })
    }

    fn get_builder(&mut self, key: &[u8]) -> &mut BlockBuilder {
        if self.builders.len() == 0 {
            self.append_new_builder(key);
        }
        self.builders.last_mut().unwrap()
    }

    /// Adds a key-value pair to SSTable
    pub fn add(&mut self, key: &[u8], value: &[u8]) {
        let latest_builder = self.get_builder(key);
        match latest_builder.add(key, value) {
            true => {
                return;
            }
            false => {
                // add new builder
                self.offset += latest_builder.current_size();
                self.append_new_builder(key);
                let latest_builder = self.get_builder(key);
                assert_eq!(latest_builder.add(key, value), true);
            }
        }
    }

    /// Get the estimated size of the SSTable.
    pub fn estimated_size(&self) -> usize {
        return 0;
    }

    /// Builds the SSTable and writes it to the given path. No need to actually write to disk until
    /// chapter 4 block cache.
    pub fn build(
        self,
        id: usize,
        block_cache: Option<Arc<BlockCache>>,
        path: impl AsRef<Path>,
    ) -> Result<SsTable> {
        let mut result =
            SsTable::open(id, block_cache, FileObject::open(path.as_ref()).unwrap()).unwrap();
        result.set_meta(self.meta.clone(), self.offset);
        Ok(result)
    }

    #[cfg(test)]
    pub(crate) fn build_for_test(self, path: impl AsRef<Path>) -> Result<SsTable> {
        self.build(0, None, path)
    }
}

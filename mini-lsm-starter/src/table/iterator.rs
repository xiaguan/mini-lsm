#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use std::sync::Arc;

use anyhow::{Error, Result};

use super::{Block, SsTable};
use crate::iterators::StorageIterator;

/// An iterator over the contents of an SSTable.
pub struct SsTableIterator {
    table: Arc<SsTable>,
    block: Arc<Block>,
    idx: usize,
}

impl SsTableIterator {
    /// Create a new iterator and seek to the first key-value pair.
    pub fn create_and_seek_to_first(table: Arc<SsTable>) -> Result<Self> {
        match table.read_block(0) {
            Ok(block) => {
                return Ok(SsTableIterator {
                    table,
                    block,
                    idx: 0,
                })
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    /// Seek to the first key-value pair.
    pub fn seek_to_first(&mut self) -> Result<()> {
        if self.table.num_of_blocks() == 0 {
            return Err(Error::msg("the table size is 0"));
        }
        self.block = self.table.read_block(0).unwrap();
        Ok(())
    }

    /// Create a new iterator and seek to the first key-value pair which >= `key`.
    pub fn create_and_seek_to_key(table: Arc<SsTable>, key: &[u8]) -> Result<Self> {
        let match_idx = table.find_block_idx(key);
        match table.read_block(match_idx) {
            Ok(block) => Ok(SsTableIterator {
                table,
                block,
                idx: match_idx,
            }),
            Err(err) => Err(err),
        }
    }

    /// Seek to the first key-value pair which >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) -> Result<()> {
        let match_idx = self.table.find_block_idx(key);
        match self.table.read_block(match_idx) {
            Ok(block) => {
                self.block = block;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl StorageIterator for SsTableIterator {
    fn value(&self) -> &[u8] {
        unimplemented!()
    }

    fn key(&self) -> &[u8] {
        unimplemented!()
    }

    fn is_valid(&self) -> bool {
        unimplemented!()
    }

    fn next(&mut self) -> Result<()> {
        unimplemented!()
    }
}

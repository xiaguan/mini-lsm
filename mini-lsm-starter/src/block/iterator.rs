#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use std::sync::Arc;

use super::Block;

/// Iterates on a block.
pub struct BlockIterator {
    block: Arc<Block>,
    key: Vec<u8>,
    value: Vec<u8>,
    idx: usize,
}

fn parse_entry(data: &[u8], mut index: usize, key_vec: &mut Vec<u8>, value_vec: &mut Vec<u8>) {
    // read the key_len
    let key_len = u16::from_be_bytes([data[index], data[index + 1]]) as usize;
    index += 2;
    key_vec.extend_from_slice(&data[index..index + key_len]);
    // read teh value_len
    let value_len = u16::from_be_bytes([data[index + key_len], data[index + key_len + 1]]);
    index += 2;
    value_vec.extend_from_slice(&data[index..index + value_len as usize]);
}

impl BlockIterator {
    fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            key: Vec::new(),
            value: Vec::new(),
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first entry.
    pub fn create_and_seek_to_first(block: Arc<Block>) -> Self {
        let mut key_vec = Vec::new();
        let mut value_vec = Vec::new();
        for i in block.offsets.iter() {
            parse_entry(&block.data, *i as usize, &mut key_vec, &mut value_vec);
        }
        BlockIterator {
            block,
            key: key_vec,
            value: value_vec,
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first key that >= `key`.
    pub fn create_and_seek_to_key(block: Arc<Block>, key: &[u8]) -> Self {
        let mut key_vec = Vec::new();
        let mut value_vec = Vec::new();
        let mut index = 0;
        for i in block.offsets.iter() {
            let before_key_vec = key_vec.len();
            parse_entry(&block.data, *i as usize, &mut key_vec, &mut value_vec);
            let after_key_vec = key_vec.len();
            let compare_key = &key_vec[before_key_vec..after_key_vec];
            if compare_key >= key && index == 0 {
                index = *i as usize;
            }
        }
        BlockIterator {
            block,
            key: key_vec,
            value: value_vec,
            idx: index,
        }
    }

    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {
        return &self.key;
    }

    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {
        return &self.value;
    }

    /// Returns true if the iterator is valid.
    pub fn is_valid(&self) -> bool {
        return self.idx < self.block.offsets.len();
    }

    /// Seeks to the first key in the block.
    pub fn seek_to_first(&mut self) {
        self.idx = 0;
    }

    /// Move to the next key in the block.
    pub fn next(&mut self) {
        self.idx += 1;
    }

    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        unimplemented!()
    }
}

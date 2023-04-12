use super::{Block, SIZE_OF_U16};
use bytes::BufMut;

/// Builds a block.
pub struct BlockBuilder {
    offset_vec: Vec<u16>,
    block_data: Vec<u8>,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        BlockBuilder {
            block_data: Vec::with_capacity(block_size),
            offset_vec: Vec::new(),
        }
    }

    pub fn current_size(&self) -> usize {
        // offset array , num of elements, data
        self.offset_vec.len() * SIZE_OF_U16 + SIZE_OF_U16 + self.block_data.len()
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");
        // why *3 : key_len , value_len , offset
        let entry_size = SIZE_OF_U16 * 3 + key.len() + value.len();
        if self.current_size() + entry_size >= self.block_data.capacity() && !self.is_empty() {
            return false;
        }
        self.offset_vec.push(self.block_data.len() as u16);
        self.block_data.put_u16(key.len() as u16);
        self.block_data.put(key);
        self.block_data.put_u16(value.len() as u16);
        self.block_data.put(value);
        true
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        self.offset_vec.is_empty()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        Block {
            data: self.block_data,
            offsets: self.offset_vec,
        }
    }
}

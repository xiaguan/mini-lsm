use super::Block;

const KEY_LEN_SIZE: usize = 2;
const VALUE_LEN_SIZE: usize = 2;
const OFFSET_LEN_SIZE: usize = 2;
const MAX_SIZE: usize = 65536;

/// Builds a block.
pub struct BlockBuilder {
    offset_vec: Vec<u16>,
    block_data: Vec<u8>,
    write_index: usize,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        BlockBuilder {
            block_data: Vec::with_capacity(block_size),
            offset_vec: Vec::new(),
            write_index: 0,
        }
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        let entry_size = KEY_LEN_SIZE + VALUE_LEN_SIZE + key.len() + value.len();
        if self.write_index + entry_size >= self.block_data.capacity() {
            return false;
        }
        if key.len() >= MAX_SIZE || value.len() >= MAX_SIZE {
            panic!("key or value is too big to fit in");
        }
        let key_len: u16 = key.len() as u16;
        let bytes = key_len.to_be_bytes();
        // push it tot the block_data
        self.block_data.extend_from_slice(&bytes);
        // push the key to the block_data
        self.block_data.extend_from_slice(key);
        // push the value to the block_data
        let value_len: u16 = value.len() as u16;
        let bytes = value_len.to_be_bytes();
        self.block_data.extend_from_slice(&bytes);
        self.block_data.extend_from_slice(value);
        self.write_index += entry_size;
        // set offset
        self.offset_vec.push(self.write_index as u16);
        return true;
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        return self.write_index == 0;
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        Block {
            data: self.block_data,
            offsets: self.offset_vec,
        }
    }
}

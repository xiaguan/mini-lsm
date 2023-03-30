#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub const SIZE_OF_U16: usize = std::mem::size_of::<u16>();

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        let offsets_len = self.offsets.len();
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        buf.put_u16(offsets_len as u16);
        buf.into()
    }

    pub fn decode(data: &[u8]) -> Self {
        let num_of_elements = (&data[data.len() - SIZE_OF_U16..]).get_u16() as usize;
        // it's the end of data , and also the beginning of the offest array
        let data_len = data.len() - SIZE_OF_U16 - SIZE_OF_U16 * num_of_elements;
        // don't  forget the `num of elements`
        let offsets_array_raw = &data[data_len..data.len() - SIZE_OF_U16];
        let offsets = offsets_array_raw
            .chunks(SIZE_OF_U16)
            .map(|mut x| x.get_u16())
            .collect();
        let data = data[0..data_len].to_vec();
        Self { data, offsets }
    }
}

#[cfg(test)]
mod tests;

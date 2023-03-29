#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::Bytes;
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        // num : num of elements
        // offset array : each entry's offset
        // entry_array : each entry's data
        let mut buf = Vec::new();
        // format the entry array
        buf.extend_from_slice(&self.data);
        // format the offset array
        for offset in &self.offsets {
            buf.extend_from_slice(&offset.to_be_bytes());
        }
        let num_of_elements = self.offsets.len() as u16;
        // format the num of elements
        buf.extend_from_slice(&num_of_elements.to_be_bytes());
        println!(
            "encode buf size is {} entry_size is {} num is {}",
            buf.len(),
            self.data.len(),
            num_of_elements
        );
        return Bytes::copy_from_slice(&buf);
    }

    pub fn decode(data: &[u8]) -> Self {
        // the num of elements is the last two character in data
        let num_of_elements = u16::from_be_bytes([data[data.len() - 2], data[data.len() - 1]]);
        println!(
            "num of elements: {} data_size {}",
            num_of_elements,
            data.len()
        );
        let entry_array_size = data.len() - num_of_elements as usize * 2 - 2;
        println!("entry array size: {}", entry_array_size);
        let mut offset_array = Vec::new();

        for i in 0..num_of_elements {
            let offset = u16::from_be_bytes([
                data[entry_array_size + i as usize * 2],
                data[entry_array_size + i as usize * 2 + 1],
            ]);
            println!("offset: {}", offset);
            offset_array.push(offset);
        }
        return Self {
            data: data[..entry_array_size].to_vec(),
            offsets: offset_array,
        };
    }
}

#[cfg(test)]
mod tests;

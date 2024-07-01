use proc_mem::{Process, ProcMemError};
use crate::memory_reader::MemoryReader;

// an actual pointer, that can do all the heavy lifting once it has been constructed properly
//  don't necessarily move the process into here, just implement reading to take the process
pub struct Pointer {
    pub(crate) base_address: usize,
    pub(crate) offsets: Vec<usize>,
}

impl Pointer {
    pub fn read_float(&self, offset: usize, memory_reader: &dyn MemoryReader) -> f32 {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        let read_float = memory_reader.read_f32(
            self.resolve_offsets(
                offsets_copy,
                &memory_reader,
            )).unwrap();
        return read_float;
    }

    pub fn read_i32(&self, offset: usize, memory_reader: &dyn MemoryReader) -> i32 {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        return memory_reader.read_i32(
            self.resolve_offsets(
                offsets_copy,
                &memory_reader,
            )).unwrap();
    }

    // Start at base, walk offset, read as new base, repeat
    pub fn resolve_offsets(&self, offsets: Vec<usize>, memory_reader: &dyn MemoryReader) -> usize {
        let mut ptr = self.base_address;
        for (index, offset) in offsets.iter().enumerate() {
            let address = ptr + offset;
            if index + 1 < offsets.len() {
                ptr = memory_reader.read_usize(address).unwrap();
                if ptr == 0 {
                    return 0;
                }
            } else {
                ptr = address;
            }
        }
        return ptr;
    }

    // Creates a new pointer with the address of the old pointer as base address
    pub fn create_pointer_from_address(&self, offset: usize, memory_reader: &dyn MemoryReader) -> Pointer {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);
        offsets_copy.push(0);

        let new_base_address = self.resolve_offsets(offsets_copy, &memory_reader);
        return Pointer{
            base_address: new_base_address,
            offsets: vec![],
        };
    }
}

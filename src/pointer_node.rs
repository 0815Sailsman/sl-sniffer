use std::process::exit;
use log::debug;
use proc_mem::{Module, Process, Signature};
use crate::memory_reader::MemoryReader;

// recipe to construct actual pointer
pub struct PointerNode {
    pub(crate) name: String,
    pub(crate) pattern: String,
    pub(crate) address_offset: usize,
    pub(crate) instruction_size: usize,
}

impl PointerNode {
    pub fn resolve_to_address_for(&self, memory_reader: &dyn MemoryReader) -> usize {
        let initial_search = match memory_reader.scan_for_pattern(self.pattern.clone()) {
            Some(v) => v,
            None => exit(69)
        };
        debug!("initial search result: {:#x?}", initial_search);

        let address_at_initial_search = memory_reader.read_i32(
            memory_reader.base_address() + initial_search + self.address_offset
        ).unwrap();
        debug!("address at initial search: {:#x?}", address_at_initial_search);

        let target_address = memory_reader.base_address() + initial_search + address_at_initial_search as usize + self.instruction_size;
        debug!("Target address: {:#x?}", target_address);

        return target_address;
    }
}
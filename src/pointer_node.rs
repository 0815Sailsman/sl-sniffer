use std::process::exit;
use proc_mem::{Module, Process, Signature};

// recipe to construct actual pointer
pub struct PointerNode {
    pub(crate) name: String,
    pub(crate) pattern: String,
    pub(crate) address_offset: usize,
    pub(crate) instruction_size: usize,
}

impl PointerNode {
    pub fn resolve_to_address_for(&self, process: &Process, module: &Module) -> usize {
        let signature = Signature {
            name: self.name.clone(),
            pattern: self.pattern.clone(),
            offsets: vec![],
            extra: 0,
            relative: true,
            rip_relative: false,
            rip_offset: 0,
        };

        let initial_search = module.find_signature(&signature).unwrap_or_else(|e| {
            eprintln!("Error obtaining player_pos_address: {:?}", e);
            exit(3);
        });
        println!("initial search result: {:#x?}", initial_search);

        let address_at_initial_search = process.read_mem::<i32>(
            process.process_base_address + initial_search + self.address_offset
        ).unwrap();
        println!("address at initial search: {:#x?}", address_at_initial_search);

        let target_address = process.process_base_address + initial_search + address_at_initial_search as usize + self.instruction_size;
        println!("Target address: {:#x?}", target_address);

        return target_address;
    }
}
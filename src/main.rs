use std::process::exit;
use proc_mem::{Module, Process, Signature};

// recipe to construct actual pointer
struct PointerNode {
    name: String,
    pattern: String,
    address_offset: usize,
    instruction_size: usize,
}

// an actual pointer, that can do all the heavy lifting once it has been constructed properly
//  don't necessarily move the process into here, just implement reading to take the process
pub struct Pointer {
    base_address: usize,
    offsets: Vec<usize>,
}

impl Pointer {
    pub fn read_float(&self, offset: usize, process: &Process) -> f32 {
        let mut offsets_copy = self.offsets.clone();
        offsets_copy.push(offset);

        let read_float = process.read_mem::<f32>(
            self.resolve_offsets(
                offsets_copy,
                &process,
            )).unwrap();
        return read_float;
    }

    // Start at base, walk offset, read as new base, repeat
    fn resolve_offsets(&self, offsets: Vec<usize>, process: &Process) -> usize {
        let mut ptr = self.base_address;
        for (index, offset) in offsets.iter().enumerate() {
            let address = ptr + offset;
            if index + 1 < offsets.len() {
                ptr = process.read_mem::<usize>(address).unwrap();
                if ptr == 0 {
                    return 0;
                }
            } else {
                ptr = address;
            }
        }
        return ptr;
    }

}

struct DarkSoulsRemastered {
    process: Process,
    module: Module,
    player_position: Option<Pointer>,
}

impl DarkSoulsRemastered {
    pub fn read_player_position(&self) -> Vec<f32> {
        match &self.player_position {
            None => panic!("Position pointer uninitialized"),
            Some(pointer) => {
                return vec![
                    pointer.read_float(0x10, &self.process),
                    pointer.read_float(0x14, &self.process),
                    pointer.read_float(0x18, &self.process),
                ];
            }
        }
    }

    pub fn resolve_pointers(&mut self) {
        self.player_position = Some(Pointer {
            base_address: init_address_from_node(PointerNode {
                name: String::from("player_pos"),
                pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
                address_offset: 3,
                instruction_size: 7,
            }, &self),
            offsets: vec![0, 0x68, 0x68, 0x28],
        });
    }
}

fn main() {
    let process = Process::with_name("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
        eprintln!("Error opening process: {:?}", e);
        exit(1);
    });
    println!("Found process: {} {}", process.process_id, process.process_name);

    let module = process.module("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
        eprintln!("Error opening module: {:?}", e);
        exit(2);
    });

    // note can I create this struct in a nicer way? temporarily initializing the 2 beforehand seems uncool
    let mut dark_souls_remastered: DarkSoulsRemastered = DarkSoulsRemastered {
        process,
        module,
        player_position: None,
    };

    // I like this even more
    dark_souls_remastered.resolve_pointers();

    let coords = dark_souls_remastered.read_player_position();
    println!("Player pos: (x:{:#?}, y:{:#?}, z:{:#?})", coords[0], coords[1], coords[2]);
}

fn init_address_from_node(pointer_node: PointerNode, dark_souls_remastered: &DarkSoulsRemastered) -> usize {
    let signature = Signature {
        name: pointer_node.name,
        pattern: pointer_node.pattern,
        offsets: vec![],
        extra: 0,
        relative: true,
        rip_relative: false,
        rip_offset: 0,
    };

    let initial_search = dark_souls_remastered.module.find_signature(&signature).unwrap_or_else(|e| {
        eprintln!("Error obtaining player_pos_address: {:?}", e);
        exit(3);
    });
    println!("initial search result: {:#x?}", initial_search);

    let address_at_initial_search = dark_souls_remastered.process.read_mem::<i32>(
        dark_souls_remastered.process.process_base_address + initial_search + pointer_node.address_offset
    ).unwrap();
    println!("address at initial search: {:#x?}", address_at_initial_search);

    let target_address = dark_souls_remastered.process.process_base_address + initial_search + address_at_initial_search as usize + pointer_node.instruction_size;
    println!("Target address: {:#x?}", target_address);

    return target_address;
}
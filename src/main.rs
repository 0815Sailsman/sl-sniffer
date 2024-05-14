use std::process::exit;
use proc_mem::{Module, Process, Signature};

struct PointerNode {
    name: String,
    pattern: String,
    address_offset: usize,
    instruction_size: usize,
}

struct Pointer {
    base_address: usize,
    offsets: Vec<usize>,
}

// todo maybe add fields here for module and base address, so they dont have to be recomputed all the time?
struct DarkSoulsRemastered {
    process: Process,
    player_position: Option<Pointer>,
}

fn main() {
    let mut dark_souls_remastered: DarkSoulsRemastered = DarkSoulsRemastered {
        process: Process::with_name("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
            eprintln!("Error opening process: {:?}", e);
            exit(1);
        }),
        player_position: None,
    };
    println!("Found process: {} {}", dark_souls_remastered.process.process_id, dark_souls_remastered.process.process_name);

    resolve_pointers(&mut dark_souls_remastered);

    let coords = read_player_position(&dark_souls_remastered.player_position.unwrap(), &dark_souls_remastered.process);
    println!("Player pos: (x:{:#?}, y:{:#?}, z:{:#?})", coords[0], coords[1], coords[2]);
}

fn resolve_pointers(dark_souls_remastered: &mut DarkSoulsRemastered) {
    dark_souls_remastered.player_position = Some(Pointer {
        base_address: init_address_from_node(PointerNode {
            name: String::from("player_pos"),
            pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
            address_offset: 3,
            instruction_size: 7,
        }, &dark_souls_remastered),
        offsets: vec![0, 0x68, 0x68, 0x28],
    });
}

fn read_player_position(position_pointer: &Pointer, process: &Process) -> Vec<f32> {
    return vec![
        read_float(&position_pointer, 0x10, &process),
        read_float(&position_pointer, 0x14, &process),
        read_float(&position_pointer, 0x18, &process),
    ];
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

    let module = dark_souls_remastered.process.module("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
        eprintln!("Error opening module: {:?}", e);
        exit(2);
    });

    let initial_search = module.find_signature(&signature).unwrap_or_else(|e| {
        eprintln!("Error obtaining player_pos_address: {:?}", e);
        exit(3);
    });
    println!("initial search result: {:#x?}", initial_search);

    let base_address = dark_souls_remastered.process.process_base_address;
    let address_at_initial_search = dark_souls_remastered.process.read_mem::<i32>(
        base_address + initial_search + pointer_node.address_offset
    ).unwrap();
    println!("address at initial search: {:#x?}", address_at_initial_search);

    let target_address = base_address + initial_search + address_at_initial_search as usize + pointer_node.instruction_size;
    println!("Target address: {:#x?}", target_address);

    return target_address;
}

fn read_float(pointer: &Pointer, offset: usize, process: &Process) -> f32 {
    let mut offsets_copy = pointer.offsets.clone();
    offsets_copy.push(offset);

    let read_float = process.read_mem::<f32>(
        resolve_offsets(
            pointer.base_address,
            offsets_copy,
            &process,
        )).unwrap();
    return read_float;
}

// Start at base, walk offset, read as new base, repeat
fn resolve_offsets(base: usize, offsets: Vec<usize>, process: &Process) -> usize {
    let mut ptr = base;
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

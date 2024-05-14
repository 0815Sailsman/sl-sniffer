use std::process::exit;
use proc_mem::{Module, Process, Signature};

struct Pointer {
    base_address: usize,
    offsets: Vec<usize>,
}

fn main() {
    let proc_name = "DarkSoulsRemastered.exe";

    let dark_souls_remastered = Process::with_name(proc_name).unwrap_or_else(|e| {
        eprintln!("Error opening process: {:?}", e);
        exit(1);
    });

    println!("Found process: {} {}", dark_souls_remastered.process_id, dark_souls_remastered.process_name);

    resolve_pointers(dark_souls_remastered);
}

fn resolve_pointers(process: Process) {
    let base_address = process.process_base_address;
    println!("Base address: {:#x?}", base_address);
    let module = process.module("DarkSoulsRemastered.exe").unwrap_or_else(|e| {
        eprintln!("Error opening module: {:?}", e);
        exit(2);
    });
    println!("Module base size: {:?}", module.base_size());

    let player_position = init_position_pointer(&process, module, base_address);

    read_player_position(&player_position, &process);
}

fn init_position_pointer(process: &Process, module: Module, base_address: usize) -> Pointer {
    let player_pos_signature = Signature {
        name: String::from("player_pos"),
        pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
        offsets: vec![],
        extra: 0,
        relative: true,
        rip_relative: false,
        rip_offset: 0,
    };
    let player_pos = module.find_signature(&player_pos_signature).unwrap_or_else(|e| {
        eprintln!("Error obtaining player_pos_address: {:?}", e);
        exit(3);
    });
    println!("player_pos: {:#x?}", player_pos);

    // read pattern
    let address_offset = 3;
    let address_at_player_pos = process.read_mem::<i32>(base_address + player_pos + address_offset).unwrap();
    println!("address at player_pos: {:#x?}", address_at_player_pos);

    // combine
    // i expect 		result	0x0000000141c77e50	long
    let instruction_size = 7;
    let combined_pointer = base_address + player_pos + address_at_player_pos as usize + instruction_size;
    println!("Player pos address: {:#x?}", combined_pointer);

    return Pointer {
        base_address: combined_pointer,
        offsets: vec![0, 0x68, 0x68, 0x28]
    };
}

fn read_player_position(position_pointer: &Pointer, process: &Process) {
    let player_x = read_float(&position_pointer, 0x10, &process);
    let player_y = read_float(&position_pointer, 0x14, &process);
    let player_z = read_float(&position_pointer, 0x18, &process);

    println!("Player pos: (x:{:#?}, y:{:#?}, z:{:#?})", player_x, player_y, player_z);
}

fn read_float(pointer: &Pointer, offset: usize, process: &Process) -> f32 {
    let mut offsets_copy = pointer.offsets.clone();
    offsets_copy.push(offset);

    let read_float = process.read_mem::<f32>(
        resolve_offsets(
            pointer.base_address,
            offsets_copy,
            &process
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

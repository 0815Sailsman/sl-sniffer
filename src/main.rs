use std::process::exit;
use proc_mem::{Process, Signature};

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

/*    let x_buffer = process.read_mem::<f32>(0x0C75B788).unwrap();*/
/*    let x_buffer = process.read_mem::<f32>(0x23BA65A0).unwrap();
    println!("Player pos: (x:{:#?}, y:_, z:_)", x_buffer);*/

   /* let game_man_signature = Signature {
        name: String::from("GameMan"),
        pattern: String::from("48 8b 05 ? ? ? ? c6 40 18 00"),
        offsets: vec![],
        extra: 3,
        relative: true,
        rip_relative: false,
        rip_offset: 0,
    };
    let game_man = module.find_signature(&game_man_signature).unwrap_or_else(|e1| {
        eprintln!("Error obtaining game_man_address: {:?}", e1);
        exit(3);
    });
    println!("Game man: {:#x?}", game_man);
*/
    // find pattern
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
    let combined_pointer = base_address as i64 + (player_pos as i64) + address_at_player_pos as i64 + instruction_size;
    println!("Player pos address: {:#x?}", combined_pointer);

    // jetzt offsets aus remastered addPointer adden (/)
    // NICHT EINFACH ADDIEREN SONDERN WALKEN UND RESOLVEN
    // expect final address to be 		address	0x000000000da3d8a0	long
    let final_address = resolve_offsets(combined_pointer, vec![0, 0x68, 0x68, 0x28, 0x10], &process);
    println!("final resolved address: {:#x?}", final_address);

    // read at final address
    let x_buffer = process.read_mem::<f32>((final_address) as usize).unwrap();
    println!("Player pos: (x:{:#?}, y:_, z:_)", x_buffer);
}

fn resolve_offsets(base: i64, offsets: Vec<i64>, process: &Process) -> i64 {
    let mut ptr = base;
    for (index, offset) in offsets.iter().enumerate() {
        let address = ptr + offset;
        if (index + 1 < offsets.len()) {
            ptr = process.read_mem::<i64>(address as usize).unwrap();
            if (ptr == 0) {
                return 0;
            }
        } else {
            ptr = address;
        }
    }
    return ptr;
}

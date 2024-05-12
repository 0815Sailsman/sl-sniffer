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
    let game_man_signature = Signature {
        name: String::from("GameMan"),
        pattern: String::from("48 8b 05 ? ? ? ? c6 40 18 00"),
        offsets: vec![3],
        extra: 7,
        relative: true,
        rip_relative: false,
        rip_offset: 0,
    };
    let game_man = module.find_signature(&game_man_signature).unwrap_or_else(|e1| {
        eprintln!("Error obtaining game_man_address: {:?}", e1);
        exit(3);
    });
    println!("Game man: {:#x?}", game_man);
}

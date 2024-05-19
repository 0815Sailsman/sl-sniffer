use std::process::exit;
use proc_mem::{Module, Process};

use crate::pointer::Pointer;
use crate::pointer_node::PointerNode;
use crate::attribute::Attribute;

pub mod pointer;
pub mod pointer_node;
pub mod attribute;
pub mod item;

struct DarkSoulsRemastered {
    process: Process,
    module: Module,
    player_game_data: Option<Pointer>,
    player_position: Option<Pointer>,
    player_ins: Option<Pointer>
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

    pub fn read_player_health(&self) -> i32 {
        match &self.player_ins {
            None => panic!("player_ins pointer uninitialized"),
            Some(pointer) => {
                return pointer.read_i32(0x3e8, &self.process);
            }
        }
    }

    pub fn read_player_attribute(&self, attribute: Attribute) -> i32 {
        match &self.player_game_data {
            None => panic!("player_game_data pointer uninitialized"),
            Some(pointer) => {
                return pointer.read_i32(0x8 + attribute as usize, &self.process);
            }
        }
    }
    pub fn resolve_pointers(&mut self) {
        self.player_game_data = Some(Pointer {
            base_address: PointerNode {
                name: String::from("game_data_man"),
                pattern: String::from("48 8b 05 ? ? ? ? 48 8b 50 10 48 89 54 24 60"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x10],
        });

        self.player_position = Some(Pointer {
            base_address: PointerNode {
                name: String::from("player_pos"),
                pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x68, 0x68, 0x28],
        });

        self.player_ins = Some(Pointer {
            base_address: PointerNode {
                name: String::from("player_ins"),
                pattern: String::from("48 8b 0d ? ? ? ? 0f 28 f1 48 85 c9 74 ? 48 89 7c"),
                address_offset: 3,
                instruction_size: 7,
            }.resolve_to_address_for(&self.process, &self.module),
            offsets: vec![0, 0x68],
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
        player_game_data: None,
        player_position: None,
        player_ins: None
    };

    // I like this even more
    dark_souls_remastered.resolve_pointers();

    let coords = dark_souls_remastered.read_player_position();
    println!("Player pos: (x:{:#?}, y:{:#?}, z:{:#?})", coords[0], coords[1], coords[2]);

    let player_health = dark_souls_remastered.read_player_health();
    println!("Player health: {:#?}", player_health);

    let strength_attribute = dark_souls_remastered.read_player_attribute(Attribute::Strength);
    println!("Strength level: {:#?}", strength_attribute);
}
use std::process::exit;
use proc_mem::Process;
use remastered::DarkSoulsRemastered;

use crate::attribute::Attribute;
use crate::bonfire::Bonfire;

pub mod pointer;
pub mod pointer_node;
pub mod attribute;
pub mod item;
mod bonfire;
mod remastered;

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
        player_ins: None,
        bonfire_db: None
    };

    // I like this even more
    dark_souls_remastered.resolve_pointers();

    let coords = dark_souls_remastered.read_player_position();
    println!("Player pos: (x:{:#?}, y:{:#?}, z:{:#?})", coords[0], coords[1], coords[2]);

    let player_health = dark_souls_remastered.read_player_health();
    println!("Player health: {:#?}", player_health);

    let strength_attribute = dark_souls_remastered.read_player_attribute(Attribute::Strength);
    println!("Strength level: {:#?}", strength_attribute);

    let inventory = dark_souls_remastered.read_inventory();
    println!("Inventory: {:#?}", inventory);

    let bonfire_state_asylum_courtcard = dark_souls_remastered.get_bonfire_state(Bonfire::UndeadAsylumCourtyard);
    println!("Bonfire asylum courtyard: {:#?}", bonfire_state_asylum_courtcard);

    let bonfire_state_firelink_shrine = dark_souls_remastered.get_bonfire_state(Bonfire::FirelinkShrine);
    println!("Bonfire firelink: {:#?}", bonfire_state_firelink_shrine);
}

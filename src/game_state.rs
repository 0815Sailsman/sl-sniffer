use crate::attribute::PlayerStats;
use crate::bonfire::AllBonfireStates;
use crate::item::Item;
use crate::remastered::DarkSoulsRemastered;

pub struct GameState {
    player_position: Vec<f32>,
    player_health: i32,
    player_attributes: PlayerStats,
    inventory: Vec<Item>,
    all_bonfire_states: AllBonfireStates
}

impl GameState {
    // Implement serialize trait or something like that for every relevant key
    pub fn as_json(&self) -> String {
        return String::from("");
    }

    pub fn from(remastered: &DarkSoulsRemastered) -> Self {
        return Self{
            player_position: remastered.read_player_position(),
            player_health: remastered.read_player_health(),
            player_attributes: remastered.read_player_attributes(),
            inventory: remastered.read_inventory(),
            all_bonfire_states: remastered.all_bonfire_states(),
        }
    }
}
use crate::remastered::DarkSoulsRemastered;

pub enum Attribute {
    Vitality = 0x38,
    Attunement = 0x40,
    Endurance = 0x48,
    Strength = 0x50,
    Dexterity = 0x58,
    Resistance = 0x80,
    Intelligence = 0x60,
    Faith = 0x68,
    Humanity = 0x7C,
    SoulLevel = 0x88,
}

pub struct PlayerStats {
    vitality: i32,
    attunement: i32,
    endurance: i32,
    strength: i32,
    dexterity: i32,
    resistance: i32,
    intelligence: i32,
    faith: i32,
    humanity: i32,
    soul_level: i32
}

impl PlayerStats {
    pub fn from(remastered: &DarkSoulsRemastered) -> Self {
        return Self {
            vitality: remastered.read_player_attribute(Attribute::Vitality),
            attunement: remastered.read_player_attribute(Attribute::Attunement),
            endurance: remastered.read_player_attribute(Attribute::Endurance),
            strength: remastered.read_player_attribute(Attribute::Strength),
            dexterity: remastered.read_player_attribute(Attribute::Dexterity),
            resistance: remastered.read_player_attribute(Attribute::Resistance),
            intelligence: remastered.read_player_attribute(Attribute::Intelligence),
            faith: remastered.read_player_attribute(Attribute::Faith),
            humanity: remastered.read_player_attribute(Attribute::Humanity),
            soul_level: remastered.read_player_attribute(Attribute::SoulLevel),
        };
    }
}
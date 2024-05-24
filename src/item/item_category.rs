#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ItemCategory {
    Armor,
    Consumables,
    Key,
    MeleeWeapons,
    RangedWeapons,
    Ammo,
    Rings,
    Shields,
    Spells,
    SpellTools,
    UpgradeMaterials,
    UsableItems,
}

impl ItemCategory {
    pub fn by_prefix(value:i32) -> Vec<ItemCategory> {
        match value {
            0 => {
                return vec![
                    ItemCategory::MeleeWeapons,
                    ItemCategory::RangedWeapons,
                    ItemCategory::Ammo,
                    ItemCategory::Shields,
                    ItemCategory::SpellTools
                ];
            }
            1 => {
                return vec![
                    ItemCategory::Armor
                ];
            }

            2 => {
                return vec![
                    ItemCategory::Rings
                ];
            }

            4 => {
                return vec![
                    ItemCategory::Consumables,
                    ItemCategory::Key,
                    ItemCategory::Spells,
                    ItemCategory::UpgradeMaterials,
                    ItemCategory::UsableItems
                ];
            }

            other => {
                panic!("Illegal item category code: {:#?}", other);
            }
        }
    }
}
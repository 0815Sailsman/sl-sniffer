#[derive(Clone, Debug)]
pub enum ItemInfusion {
    Normal = 0,
    Crystal = 1,
    Lightning = 2,
    Raw = 3,
    Magic = 4,
    Enchanted = 5,
    Divine = 6,
    Occult = 7,
    Fire = 8,
    Chaos = 9
}

impl ItemInfusion {
    pub fn from_value(value: i32) -> ItemInfusion {
        match value {
            0 => ItemInfusion::Normal,
            1 => ItemInfusion::Crystal,
            2 => ItemInfusion::Lightning,
            3 => ItemInfusion::Raw,
            4 => ItemInfusion::Magic,
            5 => ItemInfusion::Enchanted,
            6 => ItemInfusion::Divine,
            7 => ItemInfusion::Occult,
            8 => ItemInfusion::Fire,
            9 => ItemInfusion::Chaos,
            other => panic!("Illegal value for reverse item_infusion enum: {:#?}", other)
        }
    }
}
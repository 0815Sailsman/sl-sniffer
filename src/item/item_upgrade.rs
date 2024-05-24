#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ItemUpgrade {
    None = 0,
    Unique = 1,
    Armor = 2,
    Infusable = 3,
    InfusableRestricted = 4,
    PyroFlame = 5,
    PyroFlameAscended = 6,
}
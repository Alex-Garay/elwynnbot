pub enum Offsets {
    PlayerGuid = 0x00468550,
    EnumerateVisibleObjects = 0x00468380,
}

pub enum ObjectType {
    None,
    Item,
    Container,
    Unit,
    Player,
    GameObject,
    DynamicObject,
    Corpse,
}
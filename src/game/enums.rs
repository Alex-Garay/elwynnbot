pub enum Offsets {
    PlayerGuid = 0x00468550,
    EnumerateVisibleObjects = 0x00468380,
    GetObjectPointer = 0x00464870,
}

#[derive(Debug)]
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

pub static OBJECT_TYPE_OFFSET: usize = 0x14;

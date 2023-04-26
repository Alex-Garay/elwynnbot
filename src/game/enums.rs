use tracing::info;

pub enum Offsets {
    PlayerGuid = 0x00468550,
    EnumerateVisibleObjects = 0x00468380,
    GetObjectPointer = 0x00464870,
    ObjectType = 0x14,
    UnitDescriptorOffset = 0x8,
    UnitHealthOffset = 0x58,
    UnitNameOffset = 0xB30,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum WoWObjectVariants {
    WoWNone(WoWNone),
    WoWItem(WoWItem),
    WoWContainer(WoWContainer),
    WoWUnit(WoWUnit),
    WoWPlayer(WoWPlayer),
    WoWGameObject(WoWGameObject),
    WoWDynamicObject(WoWDynamicObject),
    WoWCorpse(WoWCorpse),
}

#[derive(Clone, Debug)]
pub struct WoWNone {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWItem {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWContainer {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWPlayer {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWGameObject {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWCorpse {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWDynamicObject {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

#[derive(Clone, Debug)]
pub struct WoWUnit {
    pub pointer: usize,
    pub guid: u64,
    pub object_type: ObjectType,
}

impl WoWUnit {
    pub fn get_descriptor_address(&self) -> usize {
        let descriptor_address = self.pointer + Offsets::UnitDescriptorOffset as usize;
        let descriptor = unsafe { core::ptr::read_volatile(descriptor_address as *const usize) };
        descriptor
    }
    // Will only ever return a percentage. The client in 1.12.1 is unaware of the actual health value of a WoWUnit.
    pub fn health(&self) -> i32 {
        let health_address = self.get_descriptor_address() + Offsets::UnitHealthOffset as usize;
        let health = unsafe { core::ptr::read_volatile(health_address as *const i32) };

        health
    }

    pub fn name(&self) -> String { 
        let pointer_one_address = self.pointer + Offsets::UnitNameOffset as usize;

        let pointer_one =  unsafe { core::ptr::read_volatile( pointer_one_address as *const usize) };
        let pointer_two = unsafe { core::ptr::read_volatile( pointer_one as *const usize) };

        let name = unsafe { core::ptr::read_volatile( pointer_two as *const [u8; 512]) };

        let name_binding = String::from_utf8_lossy(&name);

        let name_sanitized = name_binding.split('\0').next().unwrap();

        name_sanitized.to_string()
    }
}
// pub static OBJECT_TYPE_OFFSET: usize = 0x14;

pub enum ClientOffsets {
    StaticClientConnection = 0x00C79CE0,
    ObjectManagerOffset = 0x2ED0,
    FirstObjectOffset = 0xAC,
    LocalGuidOffset = 0xC0,
    NextObjectOffset = 0x3C,
    LocalTargetGuid = 0x00BD07B0,
    LocalPlayerGuid = 0xBD07A8,
}

pub enum NameOffsets {
    NameStore = 0x00C5D938 + 0x8,
    NameMask = 0x24,
    NameBase = 0x1C,
    NameString = 0x20
}

pub enum ObjectOffets {
    Type = 0x14,
    PositionX = 0x79C,
    PositionY = 0x798,
    PositionZ = 0x7A0,
    Rot = 0x7A8,
    Guid = 0x30,
    UnitFields = 0x8
}

 pub enum UnitOffsets {
    Level = 0x36 * 4,
    Health = 0x18 * 4,
    Energy = 0x19 * 4,
    MaxHealth = 0x20 * 4,
    SummonedBy = 0xE * 4,
    MaxEnergy = 0x21 * 4
}
#[derive(Debug)]
pub enum LoadAddresses {
    ClientConnection = ClientOffsets::StaticClientConnection as isize,
    ObjectManager = (ClientOffsets::StaticClientConnection as isize) + (ClientOffsets::ObjectManagerOffset as isize),
    FirstObject = (ClientOffsets::StaticClientConnection as isize + ClientOffsets::ObjectManagerOffset as isize) + (ClientOffsets::FirstObjectOffset as isize),
    LocalTargetGuid =  ClientOffsets::LocalTargetGuid as isize,
    LocalPlayerGuid =  ClientOffsets::LocalPlayerGuid as isize,
}

pub struct WoWObject {
    pub guid: u32,
    pub summoned_by: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub rotation: f32,
    pub base_address: isize,
    pub unit_field_address: u32,
    pub object_type: i16,
    pub name: String,
    pub current_health: u32,
    pub max_health: u32,
    pub current_energy: u32,
    pub max_energy: u32,
    pub level: u32,
    pub is_dead: bool
}

impl Default for WoWObject {
    fn default() -> Self {
        WoWObject {
            guid: 0,
            summoned_by: 0,
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            rotation: 0.0,
            base_address: 0,
            unit_field_address: 0,
            object_type: 0,
            name: String::new(),
            current_health: 0,
            max_health: 0,
            current_energy: 0,
            max_energy: 0,
            level: 0,
            is_dead: false,
        }
    }
}
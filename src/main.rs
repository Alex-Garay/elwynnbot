mod memory;
use memory::get_pid;
use memory::byte_to_string;
use process_memory::*;
fn main() -> std::io::Result<()> {
    let pid = get_pid("Wow.exe");    

    let load_address = LoadAddress {
            client_connection: ClientOffsets::StaticClientConnection as u32,
            object_manager: (ClientOffsets::StaticClientConnection as u32) + (ClientOffsets::ObjectManagerOffset as u32),
            first_object:  (ClientOffsets::StaticClientConnection as u32 + ClientOffsets::ObjectManagerOffset as u32) + (ClientOffsets::FirstObjectOffset as u32),
            local_target_guid: ClientOffsets::LocalTargetGuid as u64,
            local_player_guid: ClientOffsets::LocalPlayerGuid as u64,
    };
    println!("LocalTargetGuid: {:#X}", load_address.local_target_guid);

    let process_handle;
    match pid {
        Ok(process) => process_handle = Some(process.try_into_process_handle()?),
        Err(_err) => {
            println!("Error: Not a valid process.");
            return  Ok(());
        }
    }

    if let Some(handle) = process_handle {
        let mut player_name_memory = DataMember::<[u8; 8]>::new(handle);
        let mut local_player = DataMember::<u32>::new(handle);
        // // Sets the memory address for a player's name.
        player_name_memory.set_offset(vec![0xC79D18]);
        local_player.set_offset(vec![load_address.local_target_guid.try_into().unwrap()]);
        // // Read name of player in bytes.
        let player_name = match unsafe {player_name_memory.read()} {
            Ok(name) => byte_to_string(&name),
            _ => {
                println!("Error: Unable to retrieve player's health from process.");
                return Ok(());
            }
        };

        let guid = match unsafe {local_player.read()} {
            Ok(val) => val,
            _ => 0,
        };
    
        println!("Name: {:?}", player_name);
        println!("Target Guid: {:?}", guid);
    }
    Ok(())
}


enum ClientOffsets {
    StaticClientConnection = 0x00C79CE0,
    ObjectManagerOffset = 0x2ED0,
    FirstObjectOffset = 0xAC,
    LocalGuidOffset = 0xC0,
    NextObjectOffset = 0x3C,
    LocalTargetGuid = 0x00BD07B0,
    LocalPlayerGuid = 0xBD07A8,
}

enum NameOffsets {
    NameStore = 0x00C5D938 + 0x8,
    NameMask = 0x24,
    NameBase = 0x1C,
    NameString = 0x20
}

enum ObjectOffets {
    Type = 0x14,
    PositionX = 0x79C,
    PositionY = 0x798,
    PositionZ = 0x7A0,
    Rot = 0x7A8,
    Guid = 0x30,
    UnitFields = 0x8
}

 enum UnitOffsets {
    Level = 0x36 * 4,
    Health = 0x18 * 4,
    Energy = 0x19 * 4,
    MaxHealth = 0x20 * 4,
    SummonedBy = 0xE * 4,
    MaxEnergy = 0x21 * 4
}

struct LoadAddress {
    client_connection: u32,
    object_manager: u32,
    first_object: u32,
    local_target_guid: u64,
    local_player_guid: u64
}

struct WoWObject {
    guid: u32,
    summoned_by: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    rotation: f32,
    base_address: u32,
    unit_field_address: u32,
    object_type: i16,
    name: String,

    // Player or Mob Specific
    current_health: u32,
    max_health: u32,
    current_energy: u32,
    max_energy: u32,
    level: u32,
    is_dead: bool
}
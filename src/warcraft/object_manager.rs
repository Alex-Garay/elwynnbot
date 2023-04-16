use process_memory::*;
use crate::{memory::byte_to_string};
pub fn player_instance(handle: ProcessHandle) {
    let mut player_name_memory = DataMember::<[u8; 8]>::new(handle);
    // Sets the memory address for a player's name.

    player_name_memory.set_offset(vec![0x00C79D18]);
    
    // Read name of player in bytes.
    let player_name = match unsafe {player_name_memory.read()} {
        Ok(name) => byte_to_string(&name),
        _ => String::from("value")
    };
    println!("Name: {:#?}",  player_name);
}

pub fn _player_name(handle: ProcessHandle) {
    let mut player_name_memory = DataMember::<[u8; 8]>::new(handle);
    // // Sets the memory address for a player's name.
    player_name_memory.set_offset(vec![0xC79D18]);
    // // Read name of player in bytes.
    let player_name = match unsafe {player_name_memory.read()} {
        Ok(name) => byte_to_string(&name),
        _ => String::from("value")
    };
    println!("Name: {:?}", player_name);
}

/*
    player_name_memory.set_offset(vec![0x18F4AF6C]);
    let bytes: [u8; 8] = unsafe {
        player_name_memory.read().unwrap()
    };
    let u32_slice = &bytes[0..4];
    let u32_array: [u8; 4] = u32_slice.try_into().expect("invalid slice length");
    println!("Name: {:#?}",  u32::from_le_bytes(u32_array));
 */
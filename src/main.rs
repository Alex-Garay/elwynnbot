mod memory;
use memory::get_pid;
use memory::byte_to_string;
use process_memory::*;
fn main() -> std::io::Result<()> {
    let pid = get_pid("Wow.exe");

    let process_handle;
    match pid {
        Ok(process) => process_handle = Some(process.try_into_process_handle()?),
        Err(_err) => {
            println!("Error: Not a valid process.");
            return  Ok(());
        }
    }

    if let Some(handle) = process_handle {
        let mut player_name = DataMember::<[u8; 8]>::new(handle);
    
        // // Sets the memory address for a player's name.
        player_name.set_offset(vec![0x00C79D18]);
        // // Read name of player in bytes.
        let string_bytes = match unsafe {player_name.read()} {
            Ok(name) => name,
            _ => {
                println!("Error: Unable to retrieve player's health from process.");
                return Ok(());
            }
        };
    
        println!("Name: {:?}", byte_to_string(&string_bytes));
    }
    Ok(())
}
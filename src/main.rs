mod memory;
use memory::get_pid;
use process_memory::*;

fn main() -> std::io::Result<()> {

    let process_handle = get_pid("Wow.exe").try_into_process_handle()?;

    let mut player_name = DataMember::<[u8; 8]>::new(process_handle);

    // Sets the memory address for a player's name.
    player_name.set_offset(vec![0x00C79D18]);

    // Read name of player in bytes.
    let string_bytes = unsafe {
        player_name.read().unwrap()
    };

    println!("String Number: {:?}", string_bytes);

    // Turns Bytes into a String
    let binding = String::from_utf8_lossy(&string_bytes);

    // Removes null 0 bytes from the string.
    let result_string = binding.split('\0').next().unwrap();

    println!("Name: {:?}", &result_string);
    Ok(())
}

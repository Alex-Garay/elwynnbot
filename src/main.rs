mod warcraft;
mod memory;
use process_memory::*;
fn main() -> std::io::Result<()> {
    let process_id = memory::get_process_id_by_name("WoW.exe");
    println!("Process ID: {:?}", process_id);
    if let Some(process_id) = process_id {
        // let mut my_buffer: Vec<u8> = vec![];
        // memory::read_process_memory(process_id, 0x00C79D18, &mut my_buffer);
        // println!("OUTSIDE: {:?}", my_buffer);
        memory::read_process_memory();
        // let handle = process_id.try_into_process_handle()?;
        // warcraft::object_manager::player_instance(handle);
    }
    Ok(())
}
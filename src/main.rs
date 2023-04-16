mod warcraft;
mod memory;
use process_memory::*;
fn main() -> std::io::Result<()> {
    let process_id = memory::get_process_id_by_name("Wow.exe");

    if let Some(process_id) = process_id {
        let handle = process_id.try_into_process_handle()?;
        warcraft::object_manager::player_instance(handle);
    }
    Ok(())
}
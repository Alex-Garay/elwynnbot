mod warcraft;
mod memory;
use process_memory::*;
fn main() -> std::io::Result<()> {
    let pid = memory::get_process_id_by_name("Wow.exe");

    if let Some(pid) = pid {
        let handle = pid.try_into_process_handle()?;
        warcraft::object_manager::player_instance(handle);
    }
    Ok(())
}
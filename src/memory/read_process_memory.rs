// TODO: Using Windows's win32 api.
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_VM_READ
};
pub fn read_process_memory (process_id: u32, address: usize, buffer: &mut [u8]) {
    let process_handle = unsafe {
        OpenProcess(PROCESS_VM_READ, false, process_id)
    };
}
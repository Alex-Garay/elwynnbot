use crate::memory::byte_to_string;
mod windows {
    pub(crate) use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    };
}

pub fn get_process_id_by_name(process_name: &str) -> Option<u32> {
    // CreateToolhelp32Snapshot: Takes a snapshot of the specified processes, as well as the heaps, modules, and threads used by these processes.
    // dwFlags: The portions of the system to be included in the snapshot. This parameter can be one or more of the following values.
    // TH32CS_SNAPPROCESS: Includes all processes in the system in the snapshot.
    let snapshot = match unsafe {
        windows::CreateToolhelp32Snapshot(windows::TH32CS_SNAPPROCESS, 0)
    } {
        Ok(handle) => handle,
        Err(_) => return None
    };
    // PROCESSENTRY32: Describes an entry from a list of the processes residing in the system address space when a snapshot was taken.
    // Default::default(): Sets all values to it's default.
    // dwSize: The size of the structure, in bytes. Before calling the Process32First function, set this member to sizeof(PROCESSENTRY32). If you do not initialize dwSize, Process32First fails.
    let mut process_entry: windows::PROCESSENTRY32 = Default::default();
    process_entry.dwSize = std::mem::size_of::<windows::PROCESSENTRY32>() as u32;

    // Process32First: Retrieves information about the first process encountered in a system snapshot.
    if unsafe {windows::Process32First(snapshot, &mut process_entry) == false} {
        return None
    }

    loop {
        // Compare our current process's name with the wanted process's name.
        if byte_to_string(&process_entry.szExeFile) == process_name {
            // Returns the process id (PID)
            return Some(process_entry.th32ProcessID)
        }

        if unsafe { windows::Process32Next(snapshot, &mut process_entry) == false} {
            return None
        }
    }
}

/*
Original: https://github.com/Tommoa/rs-process-memory/blob/master/examples/fastyboy.rs
Windows Documentation: https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/
 */
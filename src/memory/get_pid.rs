use std::mem::MaybeUninit;
mod windows {
    pub(crate) use windows::Win32::{
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
    };
}

pub fn get_pid(process_name: &str) -> process_memory::Pid {
    // PROCESSENTRY32: Describes an entry from a list of the processes residing in the system address space when a snapshot was taken.
    // MaybeUninit::zeroed().assume_init() makes 0 as the default value for all properties.
    // dwSize: The size of the structure, in bytes. Before calling the Process32First function, set this member to sizeof(PROCESSENTRY32). If you do not initialize dwSize, Process32First fails.
    let mut entry: windows::PROCESSENTRY32 = unsafe {
        let mut entry: windows::PROCESSENTRY32 = MaybeUninit::zeroed().assume_init();
        entry.dwSize = std::mem::size_of::<windows::PROCESSENTRY32>() as u32;
        entry
    };

    // CreateToolhelp32Snapshot: Takes a snapshot of the specified processes, as well as the heaps, modules, and threads used by these processes.
    // dwFlags: The portions of the system to be included in the snapshot. This parameter can be one or more of the following values.
    // TH32CS_SNAPPROCESS: Includes all processes in the system in the snapshot.
    let snapshot = unsafe {
        match windows::CreateToolhelp32Snapshot(windows::TH32CS_SNAPPROCESS, 0) {
        Ok(shot) =>  shot,
        _ => return 0,
        }
    };

    // Process32First: Retrieves information about the first process encountered in a system snapshot.
    if unsafe {windows::Process32First(snapshot, &mut entry) == false} {
        return 0
    }

    loop {
        // Converts the current process's name from bytes to string.
        let binding = String::from_utf8_lossy(&entry.szExeFile);
        // Removes all the extra null zeros in the string. Example: "Wow.exe/0/0/0/0/0/0/0/0"
        let santized_binding = binding.split("\0").next().unwrap();
        // Compare our current process's name with the wanted process's name.
        if santized_binding == process_name {
            // Returns the process id (PID)
            return entry.th32ProcessID;
        }

        if unsafe { windows::Process32Next(snapshot, &mut entry) == false} {
            return 0;
        }
    }
}
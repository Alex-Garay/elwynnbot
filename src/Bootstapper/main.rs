use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use windows::{
    core::{ 
        PSTR,
        PCSTR,
        PWSTR
    },
    Win32::{
        Foundation::{
            CloseHandle,
            HANDLE
        },
        System::{
            Memory::{
                VirtualAllocEx,
                MEM_COMMIT,
                PAGE_EXECUTE_READWRITE
            },
            Threading::{
                Sleep,
                CreateProcessA,
                PROCESS_INFORMATION,
                PROCESS_CREATION_FLAGS,
                STARTUPINFOA,
                GetProcessId,
            },
            LibraryLoader::{
                GetModuleHandleW
            },
            Diagnostics::Debug::WriteProcessMemory,
        }
    }
};

pub fn bootstrap() {
    println!("Bootstrapping...");

    let application_path: &str = r"O:\Warcraft Development\Vanilla Client\WoW.exe";

    let application_path_ansi: CString = CString::new(application_path).unwrap();

    let mut startup_information: STARTUPINFOA = Default::default();

    let mut process_information: PROCESS_INFORMATION = Default::default();

    startup_information.cb = std::mem::size_of::<STARTUPINFOA> as u32;

    let success = unsafe {
        CreateProcessA(
            PCSTR(application_path_ansi.as_ptr() as *mut u8),
            PSTR(std::ptr::null_mut()),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &mut startup_information,
            &mut process_information,
        )
    };
    unsafe {
        Sleep(1000);
        // println!("Sleeping: 1s");
    }

    let process_handle = process_information.hProcess;
    
    let current_directory = std::env::current_dir().unwrap();

    let loader_path = current_directory.join("Loader.dll");

    let loader_path_wide: Vec<u16> = loader_path.as_os_str().encode_wide().chain(Some(0)).collect();

    // println!("Loader Path: {:?}", loader_path);
    // println!("Loader Path Wide: {:?}", loader_path_wide);
    // println!("Loader Path Wide Length: {:?}", loader_path_wide.len());

    let load_path_pointer = unsafe {
        VirtualAllocEx(
            process_handle,
            None,
            loader_path_wide.len() * 2,
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE
        )
    };
    
    unsafe {
        Sleep(500);
    }

    let bytes: Vec<u8> = loader_path_wide.iter().map(|&x| x as u8).collect();

    let mut bytes_written = 0;

    let write_loader_path_result = unsafe {
        WriteProcessMemory(
            process_handle,
            load_path_pointer,
            bytes.as_ptr() as _,
            bytes.len(),
            Some(bytes_written as *mut usize),
        )
    };

    // println!("hProcess: {:?}\nlpBaseAddress: {:?}\nlpBuffer: {:?}\nnSize: {:?}", process_handle, load_path_pointer, bytes.as_ptr(), bytes.len());
    // println!("Bytes: {:?}", bytes);
    // println!("Write Loader: {:?}", write_loader_path_result);

    // println!("Current Directory: {:?}\nLoader Path: {:?}", current_directory, loader_path);
    if write_loader_path_result.as_bool() {
        println!("WriteProcessMemory succeeded");
    } else {
        let error_code = unsafe { windows::imp::GetLastError() };
        
        println!("WriteProcessMemory failed with error code: {}", error_code);
    }
    println!("Bootstrap Success!");
}
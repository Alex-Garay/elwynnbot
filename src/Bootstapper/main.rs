use std::ffi::CString;
use windows::{
    core::{ 
        PSTR,
        PCSTR,
    },
    Win32::{
        Foundation::{
            CloseHandle,
            HANDLE
        },
        System::Threading::{
            Sleep,
            CreateProcessA,
            PROCESS_INFORMATION,
            PROCESS_CREATION_FLAGS,
            STARTUPINFOA,
            GetProcessId,
}}};

pub fn bootstrap() {
    println!("Hello from read_process_memory");
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
        println!("Sleeping: 1s");
    }

    let process_handle = unsafe {
        HANDLE(GetProcessId(process_information.hProcess) as isize)
    };

    let current_directory = std::env::current_dir().unwrap();
    let loader_path = current_directory.join("Loader.dll");
    println!("Current Directory: {:?}\nLoader Path: {:?}", current_directory, loader_path);
    println!("PROCESS HANDLE: {:?}", process_handle);
}
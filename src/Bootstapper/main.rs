use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::ffi::c_void;
use windows::{
    core::{ 
        PSTR,
        PCSTR,
        PCWSTR,
        PWSTR,
        IntoParam
    },
    Win32::{
        Foundation::{
            CloseHandle,
            HANDLE,
            HMODULE,
            FARPROC
        },
        System::{
            Memory::{
                VirtualAllocEx,
                VirtualFreeEx,
                MEM_COMMIT,
                PAGE_EXECUTE_READWRITE,
                MEM_RELEASE
            },
            Threading::{
                Sleep,
                CreateProcessA,
                PROCESS_INFORMATION,
                PROCESS_CREATION_FLAGS,
                STARTUPINFOA,
                GetProcessId,
                CreateRemoteThread,
                LPTHREAD_START_ROUTINE
            },
            LibraryLoader::{
                GetModuleHandleW,
                GetProcAddress
            },
            Diagnostics::Debug::WriteProcessMemory,
        },
        Security::SECURITY_ATTRIBUTES,
    }
};

type LoadLibraryFunctionType = unsafe extern "system" fn(lib: PWSTR) -> HMODULE;

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
        println!("WriteProcessMemory for Loader.dll filepath succeeded");
    } else {
        let error_code = unsafe { windows::imp::GetLastError() };

        println!("WriteProcessMemory failed with error code: {}", error_code);
    }

    let kernel32_module: &str = "kernel32.dll";

    let kernal32_module_wide: Vec<u16> = kernel32_module.encode_utf16().chain(Some(0)).collect();

    let kernal32_module_handle = unsafe { 
        GetModuleHandleW(pwstr_to_pcwstr(PWSTR(kernal32_module_wide.as_ptr() as _)))
    };

    let library_name = CString::new("LoadLibraryW").unwrap();
    let loader_dll_pointer = unsafe {
        GetProcAddress(kernal32_module_handle.unwrap(), PCSTR(library_name.as_ptr() as _))
    };

    if let Some(_) = loader_dll_pointer {
        println!("Load Library: Success! ");
    } else {
        println!("Failed to load library!");
    }

    unsafe {
        Sleep(500)
    }

    let thread_handle = unsafe {
        CreateRemoteThread(
            process_handle,
            None,
            0,
            std::mem::transmute::<_, LPTHREAD_START_ROUTINE>(loader_dll_pointer),
            Some(load_path_pointer as *const c_void),
            0,
            None
        )
    };

    match thread_handle {
        Ok(thread) => println!("Thread: {:?}", thread),
        Err(error) => println!("Error: {:?}", error)
    };

    unsafe {
        Sleep(500)
    }

    let disallocate_virtual_free_ex_memory = unsafe {
        VirtualFreeEx(
            process_handle,
            load_path_pointer,
            0,
            MEM_RELEASE
        )
    };

    if disallocate_virtual_free_ex_memory.as_bool() {
            println!("Memory Freed successfully");
        } else {
            println!("Failed to free memory with error: {:?}", unsafe { windows::Win32::Foundation::GetLastError() });
    }

    println!("Bootstrap Success!");
}

fn pwstr_to_pcwstr(pwstr: PWSTR) -> PCWSTR {
    unsafe { std::mem::transmute::<PWSTR, PCWSTR>(pwstr) }
}

use crate::game::enums::Offsets;
use detour::RawDetour;
use tracing::{error, info};

use std::sync::Once;
static mut INSTANCE: Option<Functions> = None;
static INIT: Once = Once::new();

pub fn get_hooks_instance() -> &'static mut Functions {
    unsafe {
        INIT.call_once(|| {
            INSTANCE = Some(Functions::new());
            INSTANCE.as_mut().unwrap().create_player_guid_hook();
            info!("Object Manager Initialized");
        });
        INSTANCE.as_mut().unwrap()
    }
}

type TypePlayerGuidFunction = unsafe extern "C" fn() -> u64;
type TypeEnumerateVisibleObjects =
    unsafe extern "fastcall" fn(callback: *const extern "fastcall" fn(i32, u64), filter: i32);

unsafe extern "fastcall" fn callback_enumerate_visible_objects(filter: i32, guid: u64) {
    info!("EnumerateVisibleObjects: {:?}, {:?}", filter, guid);
}

pub struct Functions {
    pub player_guid_hook: Option<TypePlayerGuidFunction>,
    pub enumerate_visible_objects_hook: Option<TypeEnumerateVisibleObjects>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            player_guid_hook: None,
            enumerate_visible_objects_hook: None,
        }
    }
    // Creates a detour to the GetPlayerGuid function and stores it to our Object struct.
    pub fn create_player_guid_hook(&mut self) {
        // Function that will replace and call the GetPlayerGuid function.
        unsafe extern "C" fn replacement_function() -> u64 {
            info!("Hitting Replacement Functioun");
            // Creates a raw pointer to the GetPlayerGuid function.
            let raw_pointer: *const TypePlayerGuidFunction =
                Offsets::PlayerGuid as usize as *const TypePlayerGuidFunction;
            // Calls the GetPlayerGuid function stores at the raw pointer.
            (*raw_pointer)()
        }
        // Creates a detour to the GetPlayerGuid memory address and replaces it with our replacement function.
        match unsafe {
            RawDetour::new(
                Offsets::PlayerGuid as usize as *const (),
                replacement_function as *const (),
            )
        } {
            Ok(detour) => {
                info!("Successful GetPlayerGuid Hook Creation");
                // Enables the detour.
                unsafe { detour.enable().unwrap() };

                // Transmutes our detour trampoline into our TypePlayerGuidFunction.
                let get_player_guid: TypePlayerGuidFunction =
                    unsafe { std::mem::transmute(detour.trampoline()) };

                // Stores the GetPlayerGuid function inside our object so that it can be called later.
                self.player_guid_hook = Some(get_player_guid);

                // Disables the detour.
                unsafe { detour.disable().unwrap() };
            }
            _ => error!("Failed GetPlayerGuid Hook Creation"),
        }
    }
    // Will crash "WoW.exe" if a player is not logged into the game. Can use GetPlayerGuid to check if logged in.
    pub fn create_enumerate_visible_objects_hook(&mut self) {
        unsafe {
            unsafe extern "fastcall" fn replacement_function(
                callback: *const extern "fastcall" fn(i32, u64),
                filter: i32,
            ) {
                let original_function: *const TypeEnumerateVisibleObjects =
                    Offsets::EnumerateVisibleObjects as usize as *const ()
                        as *const TypeEnumerateVisibleObjects;

                (*original_function)(callback as *const extern "fastcall" fn(i32, u64), filter)
            }

            match RawDetour::new(
                Offsets::EnumerateVisibleObjects as usize as *const (),
                replacement_function as *const (),
            ) {
                Ok(detour) => {
                    info!("Successful EnumerateVisibleObjects Hook Creation");

                    detour.enable().unwrap();

                    let original_function: TypeEnumerateVisibleObjects =
                        std::mem::transmute(detour.trampoline());

                    detour.disable().unwrap();

                    self.enumerate_visible_objects_hook = Some(original_function);
                }
                _ => info!("Failed EnumerateVisibleObjects Hook Creation"),
            }
        }
    }
}

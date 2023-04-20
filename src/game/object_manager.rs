use detour::{RawDetour};
use std::sync::Once;
use tracing::{error, info};

static mut INSTANCE: Option<Objects> = None;
static INIT: Once = Once::new();

pub fn get_instance() -> &'static mut Objects {
    unsafe {
        INIT.call_once(|| {
            INSTANCE = Some(Objects::new());
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
    // TODO
    info!("EnumerateVisibleObjects: {:?}, {:?}", filter, guid);
}

pub struct Objects {
    pub player_guid: u64,
    player_guid_hook: Option<TypePlayerGuidFunction>,
    pub enumerate_visible_objects_hook: Option<TypeEnumerateVisibleObjects>,
}

impl Objects {
    fn new() -> Objects {
        Objects {
            player_guid: 0,
            player_guid_hook: None,
            enumerate_visible_objects_hook: None,
        }
    }
    // Creates a detour to the GetPlayerGuid function and stores it to our Object struct.
    fn create_player_guid_hook(&mut self) {
        // Function that will replace and call the GetPlayerGuid function.
        unsafe extern "C" fn replacement_function() -> u64 {
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

    pub fn debug_enumerate_visible_objects_hook(&self) {
        if let Some(hook) = self.enumerate_visible_objects_hook {
            unsafe {
                hook(
                    callback_enumerate_visible_objects as *const extern "fastcall" fn(i32, u64), 0
                )
            };
        }
    }
    pub fn get_player_guid(&mut self) -> Result<u64, u64> {
        if let Some(hook) = self.player_guid_hook {
            self.player_guid = unsafe { hook() };
            info!("Guid: {:?}", self.player_guid);
            return Ok(self.player_guid);
        }
        Err(0)
    }
}
enum Offsets {
    PlayerGuid = 0x00468550,
    EnumerateVisibleObjects = 0x00468380,
}

use detour::{GenericDetour, RawDetour};
use std::sync::Once;
use tracing::{debug, error, info};

static mut INSTANCE: Option<Objects> = None;
static INIT: Once = Once::new();

pub fn get_instance() -> &'static mut Objects {
    unsafe {
        INIT.call_once(|| {
            INSTANCE = Some(Objects::new());
            // INSTANCE.as_mut().unwrap().initialize_player_guid_detour();
            // INSTANCE
            //     .as_mut()
            //     .unwrap()
            //     .initialize_enumerate_visible_objects_detour();
            info!("Object Manager Initialized");
        });
        INSTANCE.as_mut().unwrap()
    }
}


type TypeEnumerateVisibleObjects = unsafe extern "fastcall" fn(
                callback: *const extern "fastcall" fn(i32, u64),
                filter: i32,
) -> ();


 unsafe extern "fastcall" fn callback_enumerate_visible_objects(filter: i32, guid: u64) -> () {
                info!("CALLBACK FUNCTION");
            }
pub struct Objects {
    initialized: bool,
    pub player_guid: u64,
    player_guid_hook: fn() -> u64,
    pub enumerate_visible_objects_hook: Option<TypeEnumerateVisibleObjects>,
}

impl Objects {
    fn new() -> Objects {
        Objects {
            player_guid: 0,
            player_guid_hook: || 0,
            initialized: false,
            enumerate_visible_objects_hook: None,
        }
    }
    fn initialize_player_guid_detour(&mut self) {
        unsafe {
            type PlayerGuidFunctionType = unsafe extern "C" fn() -> u64;
            unsafe extern "C" fn player_guid_hook() -> u64 {
                // info!("CALLING PLAYER GUID HOOK");
                let function_pointer: *const PlayerGuidFunctionType =
                    Offsets::PlayerGuid as usize as *const PlayerGuidFunctionType;
                (*function_pointer)()
            }

            let detour: RawDetour = RawDetour::new(
                Offsets::PlayerGuid as usize as *const (),
                player_guid_hook as *const (),
            )
            .unwrap();
            detour.enable().unwrap();
            self.initialized = true;
            let original_function: fn() -> u64 = std::mem::transmute(detour.trampoline());
            self.player_guid = original_function();
            self.player_guid_hook = original_function;
            // detour.disable().unwrap();
        }
    }

    pub fn initialize_enumerate_visible_objects_detour(&mut self) {
        unsafe {

            unsafe extern "fastcall" fn replacement_fn(
                callback: *const extern "fastcall" fn(i32, u64),
                filter: i32,
            ) -> () {
                info!("REPLACEMENT FUNCTION");
                let original_fn: *const TypeEnumerateVisibleObjects =
                    Offsets::EnumerateVisibleObjects as usize as *const ()
                        as *const TypeEnumerateVisibleObjects;

                (*original_fn)(callback_enumerate_visible_objects as *const extern "fastcall" fn(i32, u64), filter)
            }

            match RawDetour::new(
                Offsets::EnumerateVisibleObjects as usize as *const (),
                replacement_fn as *const (),
            ) {
                Ok(detour) => {
                    info!("SUCCESSFUL DETOUR");
                    detour.enable().unwrap();
                    let original_function: TypeEnumerateVisibleObjects =
                        std::mem::transmute(detour.trampoline());
                    info!("REPLACEMENT FUNCTION: {:?}", replacement_fn as *const ());
                    info!("ORIGINAL FUNCTION: {:?}", original_function);
                    detour.disable().unwrap();
                    original_function(callback_enumerate_visible_objects as *const extern "fastcall" fn(i32, u64), 0);
                    self.enumerate_visible_objects_hook = Some(original_function);
                }
                _ => info!("FAILED DETOUR"),
            }
        }
    }
    pub fn get_player_guid(&mut self) -> Result<u64, String> {
        info!("Get Player Guid Called");
        if self.initialized {
            self.player_guid = (self.player_guid_hook)();
            // self.enumerate_visible_objects_hook();
            Ok(self.player_guid)
        } else {
            Err(String::from("Object Manager: Not Initialized"))
        }
    }
}
enum Offsets {
    PlayerGuid = 0x00468550,
    EnumerateVisibleObjects = 0x00468380,
}

// fn callback() {
//     info!("Hello from callback!");
// }

/*
// type EnumerateVisibleObjectsCallback =
//     unsafe extern "fastcall" fn(filter: i32, guid: u64);

// unsafe extern "fastcall" fn my_callback(filter: i32, guid: u64) {
//     info!("MY CALLBACK - filter: {}, guid: {}", filter, guid);
//     // info!("MY CALLBACK");
//     // 1
// }

// let callback_fn: EnumerateVisibleObjectsCallback = my_callback;
// let callback_fn_ptr =
//     std::mem::transmute::<EnumerateVisibleObjectsCallback, *const ()>(callback_fn);
 */

// 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

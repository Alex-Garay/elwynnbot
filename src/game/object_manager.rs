use tracing::{info, error};
use detour::RawDetour;
use std::sync::Once;

static mut INSTANCE: Option<Objects> = None;
static INIT: Once = Once::new();

pub fn get_instance() -> &'static mut Objects {
    unsafe {
      INIT.call_once(|| {
            INSTANCE = Some(Objects::new());
            INSTANCE.as_mut().unwrap().initialize_player_guid();
            info!("Object Manager Initialized");
        });
        INSTANCE.as_mut().unwrap()
    }
}

pub struct Objects {
    initialized: bool,
    pub player_guid: u64,
    player_guid_hook: fn() -> u64,
}

impl Objects {
    fn new() -> Objects {
        Objects { player_guid: 0, player_guid_hook: || 0, initialized: true}
    }
    fn initialize_player_guid(&mut self) {
        unsafe {
            type PlayerGuidFUnctionType = unsafe extern "C" fn() -> u64;
            unsafe extern "C" fn player_guid_hook() -> u64 {
                let function_pointer: *const PlayerGuidFUnctionType = Offsets::PlayerGuid as usize as *const PlayerGuidFUnctionType;
                (*function_pointer)()
            }

            let detour: RawDetour = RawDetour::new(
                Offsets::PlayerGuid as usize as *const (),
                 player_guid_hook as *const ())
                 .unwrap();
            detour.enable().unwrap();
            let original_function: fn() -> u64 = std::mem::transmute(detour.trampoline());
            self.player_guid = original_function();
            self.player_guid_hook = original_function;
        }
    }
    pub fn get_player_guid(&mut self) -> Result<u64, String> {
        if self.initialized {
            self.player_guid = (self.player_guid_hook)();
            Ok(self.player_guid)
        } else {
            Err(String::from("Object Manager: Not Initialized"))
        }
    }
}
enum Offsets {
    PlayerGuid = 0x00468550,
}
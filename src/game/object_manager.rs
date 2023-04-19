use tracing::info;
use detour::RawDetour;
use std::sync::Once;

static mut INSTANCE: Option<Objects> = None;
static INIT: Once = Once::new();

pub fn get_instance() -> &'static mut Objects {
    unsafe {
      INIT.call_once(|| {
            INSTANCE = Some(Objects::new());
            INSTANCE.as_mut().unwrap().initialize_player_guid();
        });
        INSTANCE.as_mut().unwrap()
    }
}

pub struct Objects {
    initialized: bool,
    pub player_guid: u64,
    update_player_guid_hook: fn() -> u64,
}

impl Objects {
    fn new() -> Objects {
        Objects { player_guid: 0, update_player_guid_hook: || 0, initialized: true}
    }
    fn initialize_player_guid(&mut self) {
        unsafe {
            type PlayerGuidFUnctionType = unsafe extern "C" fn() -> u64;
            unsafe extern "C" fn player_guid_hook() -> u64 {
                let function_pointer: *const PlayerGuidFUnctionType = 0x00468550 as *const PlayerGuidFUnctionType;
                (*function_pointer)()
            }

            let detour: RawDetour = RawDetour::new(
                0x00468550 as *const (),
                 player_guid_hook as *const ())
                 .unwrap();
            detour.enable().unwrap();
            let original_function: fn() -> u64 = std::mem::transmute(detour.trampoline());
            info!("Player Guid: {:?}", original_function);
            self.player_guid = original_function();
            self.update_player_guid_hook = original_function;
        }
    }
    pub fn update_player_guid(&self) {
        if self.initialized {
            info!("Player Guid: {:?}", self.player_guid);
            info!("Updated Player Guid: {:?}", (self.update_player_guid_hook)());
        } else {
            panic!("Object not initialized!");
        }
    }
}

use crate::game::enums::ObjectType;
use crate::game::enums::{Offsets, OBJECT_TYPE_OFFSET};
use detour::RawDetour;
use std::sync::Once;
use tracing::{error, info};
use crate::game::get_object_manager_instance;
use crate::game::WoWObject;

static mut INSTANCE: Option<Functions> = None;

static INIT_HOOKS: Once = Once::new();

pub fn get_hooks_instance() -> &'static mut Functions {
    unsafe {
        INIT_HOOKS.call_once(|| {
            INSTANCE = Some(Functions::new());
            INSTANCE.as_mut().unwrap().create_player_guid_hook();
        });
        INSTANCE.as_mut().unwrap()
    }
}

type TypeGetPlayerGuidFunction = unsafe extern "C" fn() -> u64;
type TypeEnumerateVisibleObjects =
    unsafe extern "fastcall" fn(callback: *const extern "fastcall" fn(i32, u64), filter: i32);

type TypeGetObjectPointerFunction = unsafe extern "stdcall" fn(guid: u64) -> usize;

pub unsafe extern "fastcall" fn callback_enumerate_visible_objects(_filter: i32, guid: u64) {
    let instance = get_object_manager_instance();
    // Crashes if the instance is not created here due to Mutex locking tracing_subscriber.
    if instance.functions.get_object_pointer_hook.is_none() {
        instance.functions.create_get_object_pointer_hook();
    } else {
        let get_object_pointer = instance.functions.get_object_pointer_hook.unwrap();
        let address = get_object_pointer(guid) + OBJECT_TYPE_OFFSET;
        let byte_object_type = *(address as *const u8);

        let object_type = match byte_object_type {
        0 => ObjectType::None,
        1 => ObjectType::Item,
        2 => ObjectType::Container,
        3 => ObjectType::Unit,
        4 => ObjectType::Player,
        5 => ObjectType::GameObject,
        6 => ObjectType::DynamicObject,
        7 => ObjectType::Corpse,
        _ => ObjectType::None
        };

        info!("ObjectType: {:?}", object_type);
        
        if instance.objects.is_none() {
            instance.objects = Some(vec![]);
        }

        let wow_obects = instance.objects.as_mut().unwrap();
        wow_obects.push(WoWObject{
            pointer: get_object_pointer(guid),
            guid,
            object_type
        });

    }
}

pub struct Functions {
    pub player_guid_hook: Option<TypeGetPlayerGuidFunction>,
    pub enumerate_visible_objects_hook: Option<TypeEnumerateVisibleObjects>,
    pub get_object_pointer_hook: Option<TypeGetObjectPointerFunction>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            player_guid_hook: None,
            enumerate_visible_objects_hook: None,
            get_object_pointer_hook: None,
        }
    }

    pub fn create_get_object_pointer_hook(&mut self) {
        type TypeGetObjectPointerFunction = unsafe extern "stdcall" fn(guid: u64) -> usize;
        unsafe extern "stdcall" fn replacement_function(guid: u64) -> usize {
            let raw_pointer: *const TypeGetObjectPointerFunction =
                Offsets::GetObjectPointer as usize as *const TypeGetObjectPointerFunction;
            (*raw_pointer)(guid)
        }

        unsafe {
            match RawDetour::new(
                Offsets::GetObjectPointer as usize as *const (),
                replacement_function as *const (),
            ) {
                Ok(detour) => {
                    info!("Successful GetObjectPointer hook creation");

                    detour.enable().unwrap();

                    let original_function: TypeGetObjectPointerFunction =
                        std::mem::transmute(detour.trampoline());

                    self.get_object_pointer_hook = Some(original_function);

                    detour.disable().unwrap();
                }
                Err(_) => info!("Failed GetObjectPointer hook creation"),
            }
        }
    }
    // Creates a detour to the GetPlayerGuid function and stores it to our Object struct.
    pub fn create_player_guid_hook(&mut self) {
        // Function that will replace and call the GetPlayerGuid function.
        unsafe extern "C" fn replacement_function() -> u64 {
            // Creates a raw pointer to the GetPlayerGuid function.
            let raw_pointer: *const TypeGetPlayerGuidFunction =
                Offsets::PlayerGuid as usize as *const TypeGetPlayerGuidFunction;
            // Calls the GetPlayerGuid function stores at the raw pointer.
            (*raw_pointer)()
        }
        // Creates a detour to the GetPlayerGuid memory address and replaces it with our replacement function.
        unsafe {
            match RawDetour::new(
                Offsets::PlayerGuid as usize as *const (),
                replacement_function as *const (),
            ) {
                Ok(detour) => {
                    info!("Successful GetPlayerGuid Hook Creation");
                    // Enables the detour.
                    detour.enable().unwrap();

                    // Transmutes our detour trampoline into our TypeGetPlayerGuidFunction.
                    let get_player_guid: TypeGetPlayerGuidFunction =
                        std::mem::transmute(detour.trampoline());

                    // Stores the GetPlayerGuid function inside our object so that it can be called later.
                    self.player_guid_hook = Some(get_player_guid);

                    // Disables the detour.
                    detour.disable().unwrap();
                }
                _ => error!("Failed GetPlayerGuid Hook Creation"),
            }
        }
    }
    // Will crash "WoW.exe" if a player is not logged into the game. Can use GetPlayerGuid to check if logged in.
    pub fn create_enumerate_visible_objects_hook(&mut self) {
        unsafe extern "fastcall" fn replacement_function(
            callback: *const extern "fastcall" fn(i32, u64),
            filter: i32,
        ) {
            let original_function: *const TypeEnumerateVisibleObjects =
                Offsets::EnumerateVisibleObjects as usize as *const ()
                    as *const TypeEnumerateVisibleObjects;

            (*original_function)(callback as *const extern "fastcall" fn(i32, u64), filter)
        }

        unsafe {
            match RawDetour::new(
                Offsets::EnumerateVisibleObjects as usize as *const (),
                replacement_function as *const (),
            ) {
                Ok(detour) => {
                    info!("Successful EnumerateVisibleObjects Hook Creation");

                    detour.enable().unwrap();

                    let original_function: TypeEnumerateVisibleObjects =
                        std::mem::transmute(detour.trampoline());

                    self.enumerate_visible_objects_hook = Some(original_function);

                    detour.disable().unwrap();
                }
                _ => info!("Failed EnumerateVisibleObjects Hook Creation"),
            }
        }
    }

    pub fn get_player_guid(&self) -> u64 {
        if let Some(player_guid_hook) = self.player_guid_hook {
            unsafe { player_guid_hook() }
        } else {
            0
        }
    }
}

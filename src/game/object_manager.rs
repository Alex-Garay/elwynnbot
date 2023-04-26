use crate::game::callback_enumerate_visible_objects;
use crate::game::enums::ObjectType;
use crate::game::enums::WoWObjectVariants;
use crate::game::{get_hooks_instance, Functions};
use std::sync::Once;
use tracing::{error, info};

static mut INSTANCE: Option<ObjectManager> = None;
static INIT_OBJECT_MANAGER: Once = Once::new();

pub fn get_object_manager_instance() -> &'static mut ObjectManager<'static> {
    unsafe {
        INIT_OBJECT_MANAGER.call_once(|| {
            INSTANCE = Some(ObjectManager::new());
        });
        INSTANCE.as_mut().unwrap()
    }
}
pub struct ObjectManager<'a> {
    pub objects: Option<Vec<WoWObjectVariants>>,
    pub functions: &'a mut Functions,
}

impl ObjectManager<'_> {
    pub fn new() -> ObjectManager<'static> {
        ObjectManager {
            objects: None,
            functions: get_hooks_instance(),
        }
    }

    pub fn enumerate_visible_objects(&mut self) {
        if self.is_logged_in() {
            if let Some(wow_objects_vector) = &mut self.objects {
                wow_objects_vector.clear();
            }

            if self.functions.enumerate_visible_objects_hook.is_none() {
                self.functions.create_enumerate_visible_objects_hook();
            }
            let enumerate_hook = self.functions.enumerate_visible_objects_hook.unwrap();
            unsafe {
                enumerate_hook(
                    callback_enumerate_visible_objects as *const extern "fastcall" fn(i32, u64),
                    0,
                )
            };
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.functions.get_player_guid() > 0
    }
}

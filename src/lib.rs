use std::{net::TcpStream, sync::Mutex, thread, time::Duration};
use tracing::info;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;

mod game;

#[ctor::ctor]
fn ctor() {
    info!("Payload Injection: Success!");
    let stream: TcpStream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let _gui_thread = thread::spawn(|| {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let _app = ElwynnBot::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    });
}

#[derive(Default, NwgUi)]
pub struct ElwynnBot {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "elwynnbot", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ElwynnBot::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Call Player GUID Function")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [ElwynnBot::call_object_manager] )]
    hello_button: nwg::Button,
}

impl ElwynnBot {
    fn call_object_manager(&self) {
        thread::spawn(move || {
            let guid = game::get_hooks_instance();
            guid.create_player_guid_hook();
            if let Some(player_guid_hook) = guid.player_guid_hook {
                if unsafe { player_guid_hook() } > 0 {
                    info!("Player currently online.");
                    let mut initiated_objects_hook = false;
                    if initiated_objects_hook == false {
                        guid.create_enumerate_visible_objects_hook();
                        initiated_objects_hook = true;
                    }

                    info!("CREATING GET OBJECT POINTER HOOK");
                    guid.create_get_object_pointer_hook();
                    if let Some(hook) = guid.enumerate_visible_objects_hook {
                        info!("CALLING ENUMERATEVISIBLEOBJECTS HOOK");
                            unsafe {
                                hook(
                                    game::callback_enumerate_visible_objects
                                    as *const extern "fastcall" fn(i32, u64),
                                    0,
                                )
                            };
                    }
                } else {
                    info!("Player currently offline.");
                }
            };
        });
    }

    fn say_goodbye(&self) {
        info!("Goodbye");
        nwg::stop_thread_dispatch();
    }
}
// DOCUMENTATION FOR GUI: https://github.com/gabdube/native-windows-gui

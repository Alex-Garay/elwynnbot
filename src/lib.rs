use std::{net::TcpStream, sync::Mutex, thread, time::Duration};
use tracing::{info};
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
        let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    });
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "elwynnbot", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Call Player GUID Function")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::call_object_manager] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn call_object_manager(&self) {
        let guid = game::object_manager::get_instance();
        match guid.get_player_guid() {
            Ok(response) => {
                info!(%response);
                if response > 0 {
                    guid.create_enumerate_visible_objects_hook();
                    loop {
                        guid.debug_enumerate_visible_objects_hook();
                        thread::sleep(Duration::from_millis(50));
                    }
                }
            }
            Err(e) => info!(%e),
        }
    }

    fn say_goodbye(&self) {
        info!("Goodbye");
        nwg::stop_thread_dispatch();
    }
}
// DOCUMENTATION FOR GUI: https://github.com/gabdube/native-windows-gui
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::thread;
use nwd::NwgUi;
use nwg::NativeUi;

pub fn run() {
    thread::spawn(|| {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let _app = ElwynnBot::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    });
}

#[derive(Default, NwgUi)]
pub struct ElwynnBot {
    #[nwg_control(size: (600, 900), position: (300, 300), title: "elwynnbot", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ElwynnBot::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events( OnButtonClick: [ElwynnBot::say_hello] )]
    hello_button: nwg::Button
}

impl ElwynnBot {

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}
use std::{net::TcpStream, sync::Mutex, thread, time::Duration};
use tracing::info;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;

use detour::RawDetour;

type GetPlayerGuidFunctionType = unsafe extern "C" fn() -> u64;

unsafe extern "C" fn my_function() -> u64 {
    let function_pointer: *const GetPlayerGuidFunctionType = 0x00468550 as *const GetPlayerGuidFunctionType;

    (*function_pointer)()
}

#[ctor::ctor]
fn ctor() {
    let stream: TcpStream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("Hit the library!");
    let _counter_thread = thread::spawn(|| {
        let mut seconds: u32 = 0;
        loop {
            if seconds > 30 {
                break;
            }
            info!("Timer: {:?}s", seconds);
            seconds = seconds + 1;
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    let _gui_thread = thread::spawn(|| {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
        let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    });

}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Call Player GUID Function")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        info!("GUI Function Called!");

        unsafe {
            let detour =
                RawDetour::new(0x00468550 as *const (), my_function as *const ()).unwrap();

            info!("Enabling Detour: {:?}", detour);
            detour.enable().unwrap();
            info!("Enabled Detour? {:?}", detour);
            info!("Creating Original Function");
            let original_function: fn() -> u64 = std::mem::transmute(detour.trampoline());
            info!("Calling Original Function: {:?}", original_function);
            let result = original_function();
            info!("ORIGINAL FUNCTION RETURNED: {:?}", result);
            detour.disable().unwrap();
        }
    }

    fn say_goodbye(&self) {
        info!("GUI: Goodbye");
        nwg::stop_thread_dispatch();
    }
}
// DOCUMENTATION FOR GUI: https://github.com/gabdube/native-windows-gui

// let _message_box_thread = thread::spawn(|| {
//     let message = "DLL Injection Successful";
//     let caption = "DLL Injection";

//     let message_wide: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();
//     let caption_wide: Vec<u16> = caption.encode_utf16().chain(Some(0)).collect();

//     unsafe {
//         info!("MessageBoxW Launched");
//         MessageBoxW(
//             None,
//             PCWSTR::from_raw(message_wide.as_ptr()),
//             PCWSTR::from_raw(caption_wide.as_ptr()),
//             MB_OK
//         );
//     }
// });
mod game;

use std::{net::TcpStream, sync::Mutex, thread, time::Duration};
use tracing::info;

#[ctor::ctor]
fn main() {
    let stream: TcpStream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("Payload Injection: Success!");

    // let instance = game::get_hooks_instance();
    // instance.get_player_guid();
    // instance.create_enumerate_visible_objects_hook();
    // // instance.create_get_object_pointer_hook();
    // let enumerate_hook = instance.enumerate_visible_objects_hook.unwrap();
    // unsafe {
    //     enumerate_hook(
    //         game::callback_enumerate_visible_objects as *const extern "fastcall" fn(i32, u64),
    //         0,
    //     );
    // }
    let manager = game::get_object_manager_instance();
    info!("Are we logged in? {:?}", manager.is_logged_in());
    manager.enumerate_visible_objects();
}

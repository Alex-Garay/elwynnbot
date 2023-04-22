mod game;
use std::thread;

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};

#[ctor::ctor]
fn ctor() {
    thread::spawn(|| {
        let main_window = WindowDesc::new(ui_builder());
        let instance = game::get_hooks_instance();
        let data = instance.get_player_guid();
        AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data)
    });
}

fn ui_builder() -> impl Widget<u64> {
    let instance = game::get_hooks_instance();
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |guid: &u64, _env| (*guid).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data = instance.get_player_guid())
        .padding(5.0);
    
    Flex::column().with_child(label).with_child(button)
}
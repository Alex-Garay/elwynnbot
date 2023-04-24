mod game;
mod gui;

#[ctor::ctor]
fn ctor() {
    gui::run();
}
use gui::GUI;

mod gui;

fn main() {
    // setup gui
    let mut gui = GUI::initialize();

    while gui.wait() {

    }
}

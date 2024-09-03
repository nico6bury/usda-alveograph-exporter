use gui::GUI;

mod gui;

fn main() {
    // setup gui
    let gui = GUI::initialize();
    let recv = gui.get_receiver();

    while gui.wait() {
        match recv.recv() {
            Some(gui::InterfaceMessage::AppClosing) => GUI::quit(),
            Some(gui::InterfaceMessage::ConfigReset) => println!("ConfigReset not implemented yet."),
            Some(gui::InterfaceMessage::Process) => println!("Process not implemented yet."),
            None => {},
        }
    }
}

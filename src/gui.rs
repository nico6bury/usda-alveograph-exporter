use fltk::{app::{self, App, Receiver, Sender}, enums::{Align, Color, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Tile}, prelude::{GroupExt, WidgetExt, WindowExt}, window::{self, Window}};

/// Width in pixels of the main window
const WINDOW_WIDTH: i32 = 700;
/// Height in pixels of the main window
const WINDOW_HEIGHT: i32 = 435;

/// FrameType to use for all major groups of widgets
const GROUP_FRAME: FrameType = FrameType::GtkThinUpBox;
/// Background color (set_color()) for the major group of headers information
const HEADER_GROUP_COLOR: Color = Color::from_rgb(255,250,240);
/// Background color (set_color()) for the major group of io controls
const IO_CONTROLS_GROUP_COLOR: Color = Color::from_rgb(245,255,250);
/// Background color (set_color()) for the major group of config settings
const CONFIG_GROUP_COLOR: Color = Color::from_rgb(220,239,220);
/// Background color (set_color()) for the major group of integrated dialog
const DIALOG_GROUP_COLOR: Color = Color::from_rgb(255,248,220);

/// Alignment to use for labels in the header group
const HEADER_LABEL_ALIGN: Align = Align::Inside.union(Align::Left);
/// Color (set_label_color()) to use for labels in the header group
const HEADER_LABEL_COLOR: Color = Color::from_rgb(0,0,64);

/// This enum is specifically intended for message passing from
/// the GUI to the main function. This is done with Sender and 
/// Receiver objects created in initialize().
#[derive(Clone,Copy,PartialEq,Debug)]
pub enum InterfaceMessage {
    /// Indicates that the user wants to process a selected input and output file
    Process,
    /// Indicates that the user wants to close the program
    AppClosing,
    /// Indicates that the user wants to reset the config to the default value
    ConfigReset
}//end enum InterfaceMessage

pub struct GUI {
    app: App,
    ux_main_window: Window,
    msg_sender: Sender<InterfaceMessage>,
    msg_receiver: Receiver<InterfaceMessage>,
}//end struct GUI

impl GUI {
    /// Returns a clone of the receiver so you can
    /// react to messages sent by gui.
    pub fn get_receiver(&self) -> Receiver<InterfaceMessage> {
        return self.msg_receiver.clone();
    }//end get_receiver(self)

    /// Gives a small visual indication that the program is doing something in the background.
    pub fn start_wait(&mut self) {
        self.ux_main_window.set_cursor(fltk::enums::Cursor::Wait);
    }//end start_wait(self)

    /// Clears the visual indication from start_wait()
    pub fn end_wait(&mut self) {
        self.ux_main_window.set_cursor(fltk::enums::Cursor::Default);
    }//end end_wait(self)

    /// Closes the application.
    pub fn quit() {
        app::App::default().quit();
    }//end show(self)

    /// Wraps app.wait().  
    /// To run main app loop, use while(gui.wait()){}.
    pub fn wait(&self) -> bool {
        self.app.wait()
    }//end wait(&self)

    /// Sets up all the properties and appearances of
    /// various widgets and UI settings.
    pub fn initialize() -> GUI {
        let alveo_app = app::App::default();
        let mut main_window = window::Window::default()
            .with_size(WINDOW_WIDTH,WINDOW_HEIGHT)
            .with_label("USDA Alveograph Exporter");
        main_window.make_resizable(true);
        main_window.end();

        let (s,r) = app::channel();

        let mut tile_group = Tile::default()
            .with_pos(0,0)
            .with_size(main_window.w(), main_window.h());
        tile_group.end();
        main_window.add(&tile_group);

        // set up header information
        let mut header_group = Flex::default()
            .with_pos(0,0)
            .with_size(tile_group.w() / 7 * 4, 90);
        header_group.end();
        header_group.set_frame(GROUP_FRAME);
        header_group.set_color(HEADER_GROUP_COLOR);
        header_group.set_type(FlexType::Column);
        tile_group.add(&header_group);

        let mut header_label1 = Frame::default()
            .with_label("USDA Alveograph Exporter")
            .with_align(HEADER_LABEL_ALIGN);
        header_label1.set_label_size(18);
        header_label1.set_label_type(fltk::enums::LabelType::Embossed);
        header_label1.set_label_color(HEADER_LABEL_COLOR);
        header_group.add(&header_label1);
        let mut header_label2 = Frame::default()
            .with_label("Processes txt files from the Alveograph Program\nNicholas Sixbury/Dan Brabec\tUSDA-ARS Manhattan,KS")
            .with_align(HEADER_LABEL_ALIGN);
        header_label2.set_label_color(HEADER_LABEL_COLOR);
        header_group.add(&header_label2);

        // set up group with input and output controls, processing stuff
        let mut io_controls_group = Group::default()
            .with_pos(0, header_group.y() + header_group.h())
            .with_size(tile_group.w() / 7 * 4, tile_group.h() - header_group.h() - 125);
        io_controls_group.end();
        io_controls_group.set_frame(GROUP_FRAME);
        io_controls_group.set_color(IO_CONTROLS_GROUP_COLOR);
        tile_group.add(&io_controls_group);

        // set up group with configuration options
        let mut config_group = Group::default()
            .with_pos(io_controls_group.x() + io_controls_group.w(), 0)
            .with_size(tile_group.width() - io_controls_group.width(), tile_group.height());
        config_group.end();
        config_group.set_frame(GROUP_FRAME);
        config_group.set_color(CONFIG_GROUP_COLOR);
        tile_group.add(&config_group);

        // set up group for integrated dialog
        let mut dialog_group = Group::default()
            .with_pos(io_controls_group.x(), io_controls_group.y() + io_controls_group.h())
            .with_size(io_controls_group.w(), tile_group.h() - (io_controls_group.y() + io_controls_group.h()));
        dialog_group.end();
        dialog_group.set_frame(GROUP_FRAME);
        dialog_group.set_color(DIALOG_GROUP_COLOR);
        tile_group.add(&dialog_group);

        main_window.show();
        GUI {
            app: alveo_app,
            ux_main_window: main_window,
            msg_sender: s,
            msg_receiver: r,
        }//end struct construction
    }//end initialize()
}//end impl for GUI
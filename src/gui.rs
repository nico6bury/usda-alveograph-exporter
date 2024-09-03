use fltk::{app::{self, App}, enums::{Align, Color, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Tile}, prelude::{GroupExt, WidgetExt, WindowExt}, window::{self, Window}};


pub struct GUI {
    app: App,
    ux_main_window: Window,
}//end struct GUI

impl GUI {
    
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
            .with_size(700,435)
            .with_label("USDA Alveograph Exporter");
        main_window.make_resizable(true);
        main_window.end();

        let mut tile_group = Tile::default()
            .with_pos(0,0)
            .with_size(main_window.w(), main_window.h());
        tile_group.end();
        tile_group.set_frame(FrameType::BorderBox);
        main_window.add(&tile_group);

        // set up header information
        let mut header_group = Flex::default()
            .with_pos(0,0)
            .with_size(tile_group.w() / 7 * 4, 90);
        header_group.end();
        header_group.set_frame(FrameType::BorderBox);
        header_group.set_color(Color::from_rgb(255, 250, 240));
        header_group.set_type(FlexType::Column);
        tile_group.add(&header_group);

        let header_label_align = Align::Inside.union(Align::Left);
        let header_label_frame = FrameType::NoBox;
        let header_label_color = Color::from_rgb(0,0,64);
        let mut header_label1 = Frame::default()
            .with_label("USDA Alveograph Exporter")
            .with_align(header_label_align);
        header_label1.set_label_size(18);
        header_label1.set_label_type(fltk::enums::LabelType::Embossed);
        header_label1.set_label_color(header_label_color);
        header_group.add(&header_label1);
        let mut header_label2 = Frame::default()
            .with_label("Processes txt files from the Alveograph Program\nNicholas Sixbury/Dan Brabec\tUSDA-ARS Manhattan,KS")
            .with_align(header_label_align);
        header_label2.set_frame(header_label_frame);
        header_label2.set_label_color(header_label_color);
        header_group.add(&header_label2);

        // set up group with input and output controls, processing stuff
        let mut io_controls_group = Group::default()
            .with_pos(0, header_group.y() + header_group.h())
            .with_size(tile_group.w() / 7 * 4, tile_group.h() - header_group.h() - 125);
        io_controls_group.end();
        io_controls_group.set_frame(FrameType::BorderBox);
        io_controls_group.set_color(Color::from_rgb(245,255,250));
        tile_group.add(&io_controls_group);

        // set up group with configuration options
        let mut config_group = Group::default()
            .with_pos(io_controls_group.x() + io_controls_group.w(), 0)
            .with_size(tile_group.width() - io_controls_group.width(), tile_group.height());
        config_group.end();
        config_group.set_frame(FrameType::BorderBox);
        config_group.set_color(Color::from_rgb(220,239,220));
        tile_group.add(&config_group);

        // set up group for integrated dialog
        let mut dialog_group = Group::default()
            .with_pos(io_controls_group.x(), io_controls_group.y() + io_controls_group.h())
            .with_size(io_controls_group.w(), tile_group.h() - (io_controls_group.y() + io_controls_group.h()));
        dialog_group.end();
        dialog_group.set_frame(FrameType::BorderBox);
        dialog_group.set_color(Color::from_rgb(255,248,220));
        tile_group.add(&dialog_group);

        main_window.show();
        GUI {
            app: alveo_app,
            ux_main_window: main_window,
        }
    }//end initialize()
}//end impl for GUI
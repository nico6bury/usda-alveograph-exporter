use std::{cell::RefCell, path::PathBuf, rc::Rc};

use fltk::{app::{self, App, Receiver, Sender}, button::Button, dialog::{FileDialogOptions, FileDialogType, NativeFileChooser}, enums::{Align, Color, FrameType}, frame::Frame, group::{Flex, FlexType, Group, Tile}, prelude::{ButtonExt, DisplayExt, GroupExt, WidgetExt, WindowExt}, text::{TextBuffer, TextDisplay, TextEditor}, window::{self, Window}};

/// Width in pixels of the main window
const WINDOW_WIDTH: i32 = 700;
/// Height in pixels of the main window
const WINDOW_HEIGHT: i32 = 380;

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

/// The width in pixels of each fileIO button in the fileIO section.
const IO_BTN_WIDTH: i32 = 150;
/// The height in pixels of each fileIO button in the fileIO section.
const IO_BTN_HEIGHT: i32 = 30;
/// The amount of padding in pixels to put around each fileIO button in the fileIO section.
const IO_BTN_PADDING: i32 = 10;
/// The FrameType to use with each fileIO button in the fileIO section.
const IO_BTN_FRAME: FrameType = FrameType::GtkRoundUpFrame;
/// The Down FrameType to use with each fileIO button in the fileIO section. 
/// This is the Frame used when the button is pressed down.
const IO_BTN_DOWN_FRAME: FrameType = FrameType::GtkRoundDownFrame;
/// The color to use with each fileIO button in the fileIO section.
const IO_BTN_COLOR: Color = Color::from_rgb(248,248,255);
/// The down color to use with each fileIO button in the fileIO section. 
/// This is the color when the button is pressed down.
const IO_BTN_DOWN_COLOR: Color = Color::from_rgb(240,255,240);
/// The height in pixels of each TextBox in the fileIO section. 
/// The width is calculated based on the space available and the padding.
const IO_BOX_HEIGHT: i32 = 30;
/// The amount of padding in pixels to put around each TextBox in the fileIO section.
const IO_BOX_PADDING: i32 = 10;
/// The FrameType to use for each TextBox in the fileIO section.
const IO_BOX_FRAME: FrameType = FrameType::GtkDownFrame;
/// The amount of padding in pixels to use around the process button in the fileIO section.
const IO_PRC_BTN_PADDING: i32 = 10;
/// The width in pixels of the process button in the fileIO section. 
/// The height is calculated based on the space available and the padding.
const IO_PRC_BTN_WIDTH: i32 = 250;
/// The Color to use for the textbox for input files in the fileIO section. 
/// A gray color is recommended in order to indicate that it cannot be edited by the user.
const IO_INPUT_BOX_COLOR: Color = Color::from_rgb(240,240,240);

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

/// This struct holds together all the objects and functions for manipulating and using the GUI.
#[allow(dead_code)]
pub struct GUI {
    /// The main app object. Has some misc useful functions.
    app: App,
    /// The main window of the application.
    ux_main_window: Window,
    /// The sender used for sending messages back to main.
    msg_sender: Sender<InterfaceMessage>,
    /// The receiver handed to main in order to receive messages from the sender.
    msg_receiver: Receiver<InterfaceMessage>,
    /// A reference to the TextBox that shows the input files chosen by the user.
    ux_input_box: Rc<RefCell<TextDisplay>>,
    /// A reference to a vec containing the paths of any input files chosen by the user.
    last_input_paths: Rc<RefCell<Vec<PathBuf>>>,
    /// A reference to the TextBox that shows the output file chosen by the user.
    ux_output_box: Rc<RefCell<TextEditor>>,
    /// A reference to the path of a potential output path chosen by the user.
    last_output_path: Rc<RefCell<Option<PathBuf>>>,
}//end struct GUI

impl GUI {
    /// Returns a clone of the receiver so you can
    /// react to messages sent by gui.
    pub fn get_receiver(&self) -> Receiver<InterfaceMessage> {
        return self.msg_receiver.clone();
    }//end get_receiver(self)

    /// Gets the last set of input file paths from the gui.  
    /// If there weren't any, it might be empty.  
    /// Uses clone to avoid references.
    pub fn get_last_input_paths(&self) -> Vec<PathBuf> {
        let last_input_paths_ref = (&self.last_input_paths).clone();
        let last_input_paths = last_input_paths_ref.as_ref().borrow();
        last_input_paths.to_vec()
    }//end get_last_input_paths()

    /// Gets the last output file path from the gui.  
    /// If there isn't anything, it might be None.  
    /// Uses clone to avoid references.
    pub fn get_last_output_paths(&self) -> Option<PathBuf> {
        let last_output_path_ref = (&self.last_output_path).clone();
        let last_output_path = last_output_path_ref.as_ref().borrow();
        last_output_path.clone()
    }//end get_last_output_paths()

    /// Clears all memory or display of currently stored input paths.
    pub fn clear_last_input_paths(&mut self) {
        let last_input_paths_ref = (&self.last_input_paths).clone();
        let mut last_input_paths = last_input_paths_ref.as_ref().borrow_mut();
        let input_box_ref = (&self.ux_input_box).clone();
        let mut input_box = input_box_ref.as_ref().borrow_mut();
        let mut input_buf = input_box.buffer().unwrap_or_else(|| TextBuffer::default());
        input_buf.set_text("");
        input_box.set_buffer(input_buf);
        last_input_paths.clear();
    }//end clear_last_input_paths()

    /// Clears all memory or display of currently stored output path.
    pub fn clear_last_output_path(&mut self) {
        let last_output_path_ref = (&self.last_output_path).clone();
        let mut last_output_path = last_output_path_ref.as_ref().borrow_mut();
        let output_box_ref = (&self.ux_output_box).clone();
        let mut output_box = output_box_ref.as_ref().borrow_mut();
        let mut output_buf = output_box.buffer().unwrap_or_else(|| TextBuffer::default());
        output_buf.set_text("");
        output_box.set_buffer(output_buf);
        *last_output_path = None;
    }//end clear_last_output_path()

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

        let mut io_controls_label = Frame::default()
            .with_pos(io_controls_group.x(), io_controls_group.y() + 10)
            .with_size(io_controls_group.w(), 20)
            .with_label("Input and Output Controls")
            .with_align(Align::Center);
        io_controls_label.set_label_size(16);
        io_controls_group.add(&io_controls_label);

        let mut input_btn = Button::default()
            .with_pos(io_controls_label.x() + IO_BTN_PADDING, io_controls_label.y() + io_controls_label.h() + IO_BTN_PADDING)
            .with_size(IO_BTN_WIDTH, IO_BTN_HEIGHT)
            .with_label("Select Input File");
        input_btn.set_frame(IO_BTN_FRAME);
        input_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        input_btn.set_tooltip("Click this button to choose an input file.");
        input_btn.clear_visible_focus();
        input_btn.set_color(IO_BTN_COLOR);
        input_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        io_controls_group.add(&input_btn);

        let input_buf = TextBuffer::default();
        let mut input_box = TextDisplay::default()
            .with_pos(input_btn.x() + input_btn.w() + IO_BOX_PADDING, input_btn.y())
            .with_size(io_controls_group.w() - (input_btn.w() + (3 * IO_BOX_PADDING)), IO_BOX_HEIGHT);
        input_box.set_frame(IO_BOX_FRAME);
        input_box.set_scrollbar_align(Align::Bottom);
        input_box.set_scrollbar_size(7);
        input_box.set_color(IO_INPUT_BOX_COLOR);
        input_box.set_buffer(input_buf);
        io_controls_group.add_resizable(&input_box);

        let mut output_btn = Button::default()
            .with_pos(input_btn.x(), input_btn.y() + input_btn.h() + IO_BTN_PADDING)
            .with_size(IO_BTN_WIDTH, IO_BTN_HEIGHT)
            .with_label("Select Output File");
        output_btn.set_frame(IO_BTN_FRAME);
        output_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        output_btn.set_tooltip("Click this button to set where the output file will be located.\nOr, just type a name in the box to the right.");
        output_btn.clear_visible_focus();
        output_btn.set_color(IO_BTN_COLOR);
        output_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        io_controls_group.add(&output_btn);

        let output_buf = TextBuffer::default();
        let mut output_box = TextEditor::default()
            .with_pos(output_btn.x() + output_btn.w() + IO_BOX_PADDING, output_btn.y())
            .with_size(io_controls_group.w() - (output_btn.w() + (3 * IO_BOX_PADDING)), IO_BOX_HEIGHT);
        output_box.set_frame(IO_BOX_FRAME);
        output_box.set_scrollbar_align(Align::Bottom);
        output_box.set_scrollbar_size(7);
        output_box.set_buffer(output_buf);
        io_controls_group.add_resizable(&output_box);

        let mut process_btn = Button::default()
            .with_pos(io_controls_group.x() + (io_controls_group.w() / 2) - (IO_PRC_BTN_WIDTH / 2), output_btn.y() + output_btn.h() + IO_PRC_BTN_PADDING)
            .with_size(IO_PRC_BTN_WIDTH,(io_controls_group.y() + io_controls_group.h()) - (output_btn.y() + output_btn.h()) - (2 * IO_PRC_BTN_PADDING))
            .with_label("Process Data");
        process_btn.emit(s, InterfaceMessage::Process);
        process_btn.set_frame(IO_BTN_FRAME);
        process_btn.set_down_frame(IO_BTN_DOWN_FRAME);
        process_btn.clear_visible_focus();
        process_btn.set_color(IO_BTN_COLOR);
        process_btn.set_selection_color(IO_BTN_DOWN_COLOR);
        io_controls_group.add_resizable(&process_btn);

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

        // set up callbacks and reference stuff
        let input_box_ref = Rc::from(RefCell::from(input_box));
        let last_input_path_ref = Rc::from(RefCell::from(Vec::new()));
        let output_box_ref = Rc::from(RefCell::from(output_box));
        let last_output_path_ref = Rc::from(RefCell::from(None));

        input_btn.set_callback({
            let input_box_ref = (&input_box_ref).clone();
            let last_input_path_ref = (&last_input_path_ref).clone();
            move |_| {
                // get valid references to everything we need from outside
                let mut input_box = input_box_ref.as_ref().borrow_mut();
                let mut last_input_path = last_input_path_ref.as_ref().borrow_mut();
                let mut input_buf = input_box.buffer().unwrap_or_else(|| TextBuffer::default());
                // create a dialog to show
                let mut dialog = NativeFileChooser::new(FileDialogType::BrowseMultiFile);
                dialog.set_option(FileDialogOptions::UseFilterExt);
                dialog.set_filter("*.txt");
                dialog.set_title("Please Select an Input File");
                dialog.show();
                let dialog_error = dialog.error_message().unwrap_or_else(|| "".to_string()).replace("No error","");
                if dialog_error != "" {println!("We encountered a dialog error while getting input file:\n{}", dialog_error)}
                *last_input_path = dialog.filenames();
                let mut name_vec = Vec::new();
                for path in last_input_path.iter() {
                    match path.file_name() {
                        None => name_vec.push("FilenameInvalid".to_string()),
                        Some(name) => name_vec.push(name.to_string_lossy().to_string()),
                    }//end matching whether we can get the filename
                }//end putting filename of each file in the input_box buf
                input_buf.set_text(&name_vec.join(", "));
                drop(dialog);
                // make sure we still have our buffer
                input_box.set_buffer(input_buf);
            }//end closure
        });

        output_btn.set_callback({
            let output_box_ref = (&output_box_ref).clone();
            let last_output_path_ref = (&last_output_path_ref).clone();
            move |_| {
                // get valid references to everything we need from outside
                let mut output_box = output_box_ref.as_ref().borrow_mut();
                let mut last_output_path = last_output_path_ref.as_ref().borrow_mut();
                let mut output_buf = output_box.buffer().unwrap_or_else(|| TextBuffer::default());
                // create a dialog to show
                let mut dialog = NativeFileChooser::new(FileDialogType::BrowseSaveFile);
                dialog.set_option(FileDialogOptions::SaveAsConfirm);
                dialog.set_filter("*.xlsx");
                dialog.set_title("Please select a path for the output file.");
                dialog.show();
                let dialog_error = dialog.error_message().unwrap_or_else(|| "".to_string()).replace("No error", "");
                if dialog_error != "" {
                    println!("We encountered a dialog error while getting the output file path:\n{}", dialog_error);
                    *last_output_path = None;
                    return;
                }//end if we cauldn't get dialog
                *last_output_path = Some(dialog.filename());
                match dialog.filename().file_name() {
                    Some(name) => output_buf.set_text(&name.to_string_lossy().to_string()),
                    None => output_buf.set_text("Invalid output filename"),
                }//end matching whether we can get the filename and update buffer
                // make sure we still have our buffer
                output_box.set_buffer(output_buf);
            }//end closure
        });

        main_window.show();
        main_window.emit(s, InterfaceMessage::AppClosing);
        GUI {
            app: alveo_app,
            ux_main_window: main_window,
            msg_sender: s,
            msg_receiver: r,
            ux_input_box: input_box_ref,
            last_input_paths: last_input_path_ref,
            ux_output_box: output_box_ref,
            last_output_path: last_output_path_ref,
        }//end struct construction
    }//end initialize()
}//end impl for GUI
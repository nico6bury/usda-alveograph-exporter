use std::path::PathBuf;

use gui::GUI;

mod gui;

fn main() {
    // setup gui
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();

    while gui.wait() {
        match recv.recv() {
            Some(gui::InterfaceMessage::AppClosing) => GUI::quit(),
            Some(gui::InterfaceMessage::ConfigReset) => println!("ConfigReset not implemented yet."),
            Some(gui::InterfaceMessage::Process) => {
                // get input and output paths from gui/user
                let input_paths = gui.get_last_input_paths();
                let output_path = gui.get_last_output_paths();
                // make sure we have valid input and output paths
                let input_valid = validate_input_paths(&input_paths, &mut gui);
                let output_path = validate_output_path(output_path, &mut gui);
                if !input_valid || output_path.is_err() {continue;}
                let _output_path = output_path.expect("We already checked it wasn't an error.");
                // proceed with processing calls
                gui.start_wait();
                println!("//TODO: Processing stuff");

                // perform cleanup after finishing processing
                gui.clear_last_input_paths();
                gui.clear_last_output_path();
                gui.end_wait();
            },
            None => {},
        }//end matching message received
    }//end main application loop
}//end main function

/// Returns true if the input paths are more than 0 and valid for processing.  
/// If invalid, shows dialog message about issue.
fn validate_input_paths(input_paths: &Vec<PathBuf>, gui: &mut GUI) -> bool {
    if input_paths.len() > 0 {true}
    else {
        gui.integrated_dialog_alert("There are no input files selected. Please select one before processing.");
        false
    }
}//end validate_input_paths()

/// Returns true if the output_path given is valid for processing.  
/// If invalid, shows dialog message about issue.
fn validate_output_path(output_path: Option<PathBuf>, gui: &mut GUI) -> Result<PathBuf,()> {
    let output_txt = gui.get_output_path_text();
    if output_txt.len() == 0 {
        gui.integrated_dialog_alert("No output path selected. Please select one before processing.");
        return Err(());
    }//end if no selected file OR user deleted selection
    else if output_path.is_some() {
        return Ok(output_path.expect("Already checked that output_path is_some()"));
    }//end else case that both txt and path are valid, all seems good
    else {
        let input_paths = gui.get_last_input_paths();
        let input_dir = match input_paths.first() {
            Some(first_input_path) => match first_input_path.parent() {
                Some(parent_path) => parent_path.to_string_lossy().to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        if input_dir != "" {
            let mut output_pathbuf = PathBuf::new();
            output_pathbuf.push(input_dir);
            output_pathbuf.push(output_txt);
            output_pathbuf.set_extension("xlsx");
            if !output_pathbuf.exists() || gui.integrated_dialog_yes_no("The output file you specified already exists. Are you sure you want to overwrite it?") {
                return Ok(output_pathbuf);
            } else {return Err(());}
        } else {
            gui.integrated_dialog_alert("Couldn't use input paths to determine output path for typed name. Please select valid input files.");
            return Err(());
        }//end else we couldn't figure out input dir
    }//end else case that txt is valid, but path is not, must generate path
}//end validate_output_path()


use std::{fs, path::PathBuf};

use alveograph_exporter::{config_store::{self, ConfigStore}, data};
use gui::GUI;

mod gui;

fn main() {
    // setup gui
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();

    // get config information
    let config_name = "config";
    let mut config_path: Option<PathBuf> = None;
    let mut config_store: ConfigStore = ConfigStore::default();

    // make sure we get config information, update gui, walk user through fix if necessary
    ensure_config_valid(&mut gui, &mut config_store, &mut config_path, config_name);
    // update gui with given config store
    let _ = gui.set_config_store(&config_store);

    while gui.wait() {
        match recv.recv() {
            Some(gui::InterfaceMessage::AppClosing) => {
                if let Some(config_path_v) = config_path {
                    match gui.get_config_store() {
                        Err(msg) => gui.integrated_dialog_alert(&format!("Couldn't get save config store because:\n{}", msg)),
                        Ok(config) => {
                            if let Err(msg) = config_store::try_write_config(&config_path_v, &config) {
                                gui.integrated_dialog_alert(&format!("We weren't able to save the config file. Error message is:\n{}", msg));
                            }//end if writing is not successful
                        },
                    }//end matching whether or not we can get the config store
                    // move this back after we're done with it
                    config_path = Some(config_path_v);
                }//end if we have valid config_path
                GUI::quit();
            },
            Some(gui::InterfaceMessage::ConfigReset) => {
                if let Err(msg) = gui.set_config_store(&ConfigStore::default()) {
                    gui.integrated_dialog_alert(&format!("There was an issue resetting the config!:\n{}", msg));
                }//end if we had an error while trying to reset config store
            },
            Some(gui::InterfaceMessage::Process) => {
                // get input and output paths from gui/user
                let input_paths = gui.get_last_input_paths();
                let output_path = gui.get_last_output_paths();
                // make sure we have valid input and output paths
                let input_valid = validate_input_paths(&input_paths, &mut gui);
                let output_path = validate_output_path(output_path, &mut gui);
                if !input_valid || output_path.is_err() {continue;}
                let _output_path = output_path.expect("We already checked it wasn't an error.");
                // grab configuration details from the gui
                config_store = gui.get_config_store().unwrap();
                // proceed with processing calls
                gui.start_wait();
                println!("//TODO: Processing stuff");
                println!("{}", config_store.read_start_header);
                let file_contents = fs::read_to_string(input_paths.first().unwrap()).unwrap();
                let data = data::read_data_from_file(input_paths.first().unwrap().file_name().unwrap().to_str().unwrap(), &file_contents, &config_store);
                if let Err(msg) = data {println!("{msg}");}

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

/// Gets the config information from the config file.
/// If we encounter issues with that, lets the user know through the gui.
fn ensure_config_valid(
    gui: &mut GUI,
    config_store: &mut ConfigStore,
    config_path: &mut Option<PathBuf>,
    config_name: &str
) {
    *config_store = ConfigStore::default();
    *config_path = None;

    match config_store::try_read_config_path(config_name, false) {
        Ok(config_path_tmp) => {
            if !config_path_tmp.exists() {
                match config_store::try_write_config(&config_path_tmp, &config_store) {
                    Ok(_) => {
                        *config_path = Some(config_path_tmp);
                    },
                    Err(msg) => gui.integrated_dialog_alert(&format!("I couldn't find an exisitng configuration file, so I tried creating one, but that also failed...\nYou can use the default config, but it won't be saved when you exit.\nIf you contine seeing this message, please contact the developer. Error message below:\n{}", msg)),
                }//end matching whether we can write the default config
            }//end if the config file does not already exist
            else {
                match config_store::try_read_config(&config_path_tmp) {
                    Ok(config_store_tmp) => *config_store = config_store_tmp,
                    Err(msg) => {
                        gui.integrated_dialog_alert(&format!("I found a config file, but I couldn't read it. Things like this can happen during version changes or if the file is edited incorrectly. I'm going to go ahead and create a new file with the default settings for you. Here's the error message:\n{}",msg));
                        match config_store::try_write_config(&config_path_tmp, config_store) {
                            Ok(_) => {},
                            Err(msg) => gui.integrated_dialog_alert(&format!("Ok, so I tried writing a new config file, but I wasn't able to. Was it open? Either way, if you keep seeing messages like this, please contact the developer. You can still use the program with the default config and even edit the settings while you use it, but I can't keep track of those changes after you close the program. Error message below:\n{}", msg)),
                        }
                    },
                }//end matching whether or not we can read from the config file we have
                *config_path = Some(config_path_tmp);
            }//end else we do have a config file to read
        },
        Err(msg) => gui.integrated_dialog_alert(&format!("Could not determine the path to a config file:\n{}", msg)),
    }//end matching whether or not we can get config path
}//end ensure_config_valid()

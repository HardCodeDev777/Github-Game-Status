use std::fs::{File, remove_file};
use std::io::Write;
use std::error::Error;
use std::process::Command;
use std::{thread, time};
use std::sync::{Arc, atomic};
use slint::{SharedString, ToSharedString};
use sysinfo::System;
use crate::{app_data::{SetupData, ProcessData, PROCESSES_DATA_FILE_NAME, SETUP_FILE_NAME, STATUS_LOGIC_BATCH_FILE_NAME, DEFAULT_STATUS_BATCH_FILE_NAME}, json_utils::{write_setup_data_to_json, read_setup_data_from_json}};

// Create new SetupData and saves it as JSON
pub fn set_setup(path_to_cli: SharedString, default_status_text: SharedString, 
    default_status_emoji: SharedString) -> Result<(), Box<dyn Error>> {
    // Creates a new SetupData instance from the provided parameters
    let setup_data = SetupData::new(path_to_cli.to_string(), 
    default_status_text.to_string(), default_status_emoji.to_string());

    // Write it to JSON
    write_setup_data_to_json(SETUP_FILE_NAME, setup_data).expect("Error saving setup in Json in core logic!");

    Ok(())
}

// Delete all data files
pub fn delete_all_data() -> Result<(), Box<dyn Error>> {
    // Remove batch files
    remove_file(STATUS_LOGIC_BATCH_FILE_NAME)?;
    remove_file(DEFAULT_STATUS_BATCH_FILE_NAME)?;

    // Remove JSON data files
    remove_file(SETUP_FILE_NAME)?;
    remove_file(PROCESSES_DATA_FILE_NAME)?;

    Ok(())
}

// Create a batch file that updates the user status
pub fn make_process_batch(status_text: SharedString, status_emoji: SharedString) -> Result<(), Box<dyn Error>> {

    // Create the batch file itself
    let mut file = File::create(STATUS_LOGIC_BATCH_FILE_NAME)?;

    // Read SetupData from JSON
    let readed_setup_data = read_setup_data_from_json(SETUP_FILE_NAME).expect("Error reading setup data from Json in core logic!");

    // Get GitHub CLI path from setup data
    let cli_path = readed_setup_data.cli_path;

    // Batch file command content
    let bat_message = String::from(format!(
        "@echo off \n cd {} \n gh user-status set --emoji \"{}\" \"{}\"", 
        cli_path, status_emoji, status_text));

    // Write commands to the batch file
    file.write_all(bat_message.as_bytes())?;

    // Run the batch file using cmd
    let status = Command::new("cmd").args(["/C", STATUS_LOGIC_BATCH_FILE_NAME]).status()?;

    // Check if the batch file executed successfully
    if status.success() {
        println!("Process batch ran successfully in core logic");
    } else {
        println!("Process batch failed to run correctly in core logic");
    }

    // Delete the batch file after execution
    remove_file(STATUS_LOGIC_BATCH_FILE_NAME)?;

    Ok(())
}

// Set the default user status if no matching process is found (via batch file)
pub fn set_default_status_batch() -> Result<(), Box<dyn Error>> {
    // Create the batch file
    let mut file = File::create(DEFAULT_STATUS_BATCH_FILE_NAME)?;

    // Read SetupData from JSON
    let readed_setup_data = read_setup_data_from_json(SETUP_FILE_NAME).expect("Error reading setup data from Json in core logic!");

    // Get required data
    let cli_path = readed_setup_data.cli_path;
    let default_status_text = readed_setup_data.default_status_text;
    let default_status_emoji = readed_setup_data.default_status_emoji;

    // Batch file command content
    let bat_message = String::from(format!(
        "@echo off \n cd {} \n gh user-status set --emoji \"{}\" \"{}\"", 
        cli_path, default_status_emoji, default_status_text));

    // Write commands to the batch file
    file.write_all(bat_message.as_bytes())?;

    // Run the batch file using cmd
    let status = Command::new("cmd").args(["/C", DEFAULT_STATUS_BATCH_FILE_NAME]).status()?;

    // Check if the batch file executed successfully
    if status.success() {
        println!("Default batch ran successfully in core logic");
    } else {
        println!("Default batch failed to run correctly in core logic");
    }

    // Delete the batch file after execution
    remove_file(DEFAULT_STATUS_BATCH_FILE_NAME)?;

    Ok(())
}

// Monitor running processes
#[allow(unused_assignments)]
pub fn monitor_processes(running: Arc<atomic::AtomicBool>, readed_data: Vec<ProcessData>) -> Result<(), Box<dyn Error>> {  
    // Clone the shared running flag(Rust moment)
    let running_clone = running.clone();
    let running = running_clone.clone();

    // Clone game process data
    let readed_data_clone = readed_data.clone();

    // Start a new thread
    thread::spawn(move || {
        // Initialize system information
        let mut sys = System::new_all();

        // Runs while the flag is set to true
        while running.load(atomic::Ordering::Relaxed) {
            // Refreshes system information
            sys.refresh_all();

            // Check if at least one monitored process is running
            let matching_process_data = readed_data_clone.iter().find(|data| {
                sys.processes().values().any(|p| {
                    p.exe()
                        .and_then(|path| path.file_name())
                        .and_then(|name| name.to_str())
                        .map(|name| name.to_lowercase() == data.game_file_name.to_lowercase())
                        .unwrap_or(false)
                })
            });

            // React based on whether a matching process is found
            match matching_process_data {
                Some(process_data) => {
                    // Update status according to the running process
                    make_process_batch(process_data.status_text.to_shared_string(), 
                    process_data.status_emoji.to_shared_string())
                    .expect("Error creating batch file in core logic!");
                },
                None => {
                    // Set default status if no matching process is found
                    set_default_status_batch().expect("Error setting default status in core logic!");
                },
            }

            // Wait for 10 seconds before the next check
            thread::sleep(time::Duration::from_secs(10));
        }
    });
    Ok(())
}

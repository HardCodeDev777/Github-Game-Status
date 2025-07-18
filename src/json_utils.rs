use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;
use serde_json::{to_string_pretty, from_str};
use slint::SharedString;
use crate::app_data::{SetupData, ProcessData};

// Core function for writing to a JSON file
fn write_to_json_core(path: &str, data: String) -> Result<(), Box<dyn Error>> {
    let mut file: File;

    // If the file exists
    if Path::new(path).exists() {
        // Open without overwriting
        file = OpenOptions::new().write(true).open(path)?;
    } else {
        // Create a new file
        file = File::create(path)?;
    }
    // Write all data to it
    file.write_all(data.as_bytes())?;

    Ok(())
}

// Write SetupData to JSON
pub fn write_setup_data_to_json(path: &str, setup_data: SetupData) -> Result<(), Box<dyn Error>> {
    // Serialize SetupData
    let serialized_data = to_string_pretty(&setup_data).unwrap();
    
    // If the file already exists
    if Path::new(path).exists() {
        // Delete it first
        remove_file(path)?;
    }

    // Write to JSON
    write_to_json_core(path, serialized_data).expect("Error saving to JSON in core logic!");

    Ok(())
}

// Write ProcessData to JSON
#[allow(unused_assignments)]
pub fn write_process_data_to_json(path: &str, game_file_name: SharedString, status_text: SharedString, 
    status_emoji: SharedString) -> Result<(), Box<dyn Error>> {
    // Variable to store the final JSON string
    let mut serialized_data = String::new();

    // Create new ProcessData from received parameters
    let new_process_data = ProcessData::new(game_file_name.to_string(), status_text.to_string(), 
    status_emoji.to_string());

    // Clone it
    let new_process_data_clone = new_process_data.clone();

    // If the file exists
    if Path::new(path).exists() {
        // Read Vec<ProcessData> from it
        let mut read_vec = read_processes_data_from_json(path)
            .expect("Error reading game file names from JSON in core logic!");

        // Push new ProcessData
        read_vec.push(new_process_data);

        // Serialize the vector
        serialized_data = to_string_pretty(&read_vec).unwrap();
    } else {
        // Create a new vector of ProcessData
        let mut vec_of_game_process_data = Vec::<ProcessData>::new();

        // Add the newly created ProcessData
        vec_of_game_process_data.push(new_process_data_clone);

        // Serialize the vector
        serialized_data = to_string_pretty(&vec_of_game_process_data).unwrap();
    }

    // Write to JSON
    write_to_json_core(path, serialized_data).expect("Error saving to JSON in core logic!");

    Ok(())
}

// Reads SetupData from JSON
pub fn read_setup_data_from_json(path: &str) -> Result<SetupData, Box<dyn Error>> {
    // Open the file at the given path
    let mut file = File::open(path)?;

    // String to store JSON data
    let mut path_to_cli = String::new();

    // Read JSON into the string
    file.read_to_string(&mut path_to_cli)?;

    // Deserialize into SetupData
    let result: SetupData = from_str(&path_to_cli).unwrap();

    Ok(result)
}

// Read ProcessData from JSON
pub fn read_processes_data_from_json(path: &str) -> Result<Vec<ProcessData>, Box<dyn Error>> {
    // Open the file at the given path
    let mut file = File::open(path)?;

    // String to store JSON data
    let mut process_data_read = String::new();

    // Read JSON into the string
    file.read_to_string(&mut process_data_read)?;

    // Deserialize into Vec<ProcessData>
    let result: Vec<ProcessData> = from_str(&process_data_read).unwrap();

    Ok(result)
}

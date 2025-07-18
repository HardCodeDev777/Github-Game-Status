slint::include_modules!();
use std::error::Error;
use std::sync::{Arc, atomic};
use crate::core_logic::delete_all_data;
use crate::{json_utils::{write_process_data_to_json, read_processes_data_from_json}, core_logic::{set_setup, monitor_processes}, app_data::PROCESSES_DATA_FILE_NAME};

pub fn make_app() -> Result<(), Box<dyn Error>>{
    // Create the application window
    let app = MainWindow::new()?;
    
    // Create a shared flag for monitoring processes
    let running = Arc::new(atomic::AtomicBool::new(true));

    // Clone the flag
    let running_clone = running.clone();

    // When setup is saved in the application
    app.on_save_setup(move |cli_path, default_status_text, default_status_emoji| {
        // Save setup with the provided data
        set_setup(cli_path, default_status_text, default_status_emoji).expect("Error making setup in UI module!");
    });

    // When a game status is saved in the application
    app.on_save_status(move |game_file_name, status_text, status_emoji| {
        // Save the process status data to JSON
        write_process_data_to_json(PROCESSES_DATA_FILE_NAME, game_file_name, status_text, 
            status_emoji).expect("Error while saving process data to JSON in UI module!");
    });

    // When process monitoring is started from the application
    app.on_start_monitoring(move || {
        // Read saved processes data
        let readed_vec_data = read_processes_data_from_json(PROCESSES_DATA_FILE_NAME).
        expect("Error reading process data from JSON in UI module!");

        // Start monitoring processes
        monitor_processes(running_clone.clone(), readed_vec_data).expect("Error monitoring processes");
    });

    // When user requests to delete all data
    app.on_delete_all_data(move || {
        delete_all_data().expect("Error deleting data!");
    });

    // Start the application loop
    app.run()?;

    // If the application is closed, stop the monitoring thread
    running.store(false, atomic::Ordering::Relaxed);

    Ok(())
}

#![windows_subsystem = "windows"]

mod ui_logic;
mod core_logic;
mod json_utils;
mod app_data;

use std::error::Error;
use ui_logic::make_app;

fn main() -> Result<(), Box<dyn Error>> {
    // Run the app
    make_app().expect("Error in app work... How is this even happened?");
    Ok(())
}
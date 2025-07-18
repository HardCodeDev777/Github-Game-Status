use serde::{Serialize, Deserialize};

pub const SETUP_FILE_NAME: &str = "setupdata.json";
pub const PROCESSES_DATA_FILE_NAME: &str = "processesdata.json";
pub const DEFAULT_STATUS_BATCH_FILE_NAME: &str = "default_status_logic.bat";
pub const STATUS_LOGIC_BATCH_FILE_NAME: &str = "status_logic.bat";

#[derive(Serialize, Deserialize)]
pub struct SetupData {
    pub cli_path: String,
    pub default_status_text: String,
    pub default_status_emoji: String
}

impl SetupData{
    pub fn new(cli_path: String, default_status_text: String, default_status_emoji: String) -> Self{
        Self {cli_path, default_status_text, default_status_emoji}
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessData{
    pub game_file_name: String,
    pub status_text: String,
    pub status_emoji: String
}

impl ProcessData{
    pub fn new(game_file_name: String, status_text: String, status_emoji: String) -> Self{
        Self {game_file_name, status_text, status_emoji}
    }
}
use std::path::Path;
use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};

const DATA_PATH: &'static str = "~/.episodic.json";

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub tmdb_api_key: Option<String>,
    pub shows: Vec<Show>,
    pub watch_list: Vec<WatchItem>,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            tmdb_api_key: None,
            shows: vec![],
            watch_list: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Show {
    pub tmdb_id: u32,
    pub name: String,
    pub current_season: u32,
    pub current_episode: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WatchItem {
    pub name: String,
    pub show: Option<Show>,
    pub season: Option<u32>,
}

pub fn save_data(data: AppData) -> Result<AppData> {
    let data_str = serde_json::to_string(&data)?;
    std::fs::write(DATA_PATH, data_str).map_err(|_| anyhow!("Failed to write data"))?;
    Ok(data)
}

pub fn load_data() -> Result<AppData> {
    if !Path::new(DATA_PATH).exists() {
        let data = save_data(AppData::default())?;
        Ok(data)
    } else {
        let data_str = std::fs::read_to_string(DATA_PATH)
            .map_err(|_| anyhow!("Failed to read data file"))?;
        let data = serde_json::from_str(data_str.as_str())
            .map_err(|_| anyhow!("Failed to parse data file as JSON"))?;
        Ok(data)
    }
}

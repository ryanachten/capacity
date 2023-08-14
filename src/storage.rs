use std::{
    fs::{self, File},
    io::{Read, Write},
};

use crate::config::SprintConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StorageConfig {
    pub team_members: i32,
    pub total_sprint_points: f32,
    pub days_per_sprint: f32,
}

const FILE_NAME: &str = "config.json";

pub fn load_config() -> Option<StorageConfig> {
    let mut file_contents = String::new();

    match File::open(FILE_NAME) {
        Ok(mut file) => {
            file.read_to_string(&mut file_contents)
                .expect("Error reading file contents");
        }
        Err(_err) => return None,
    }

    let storage_config = serde_json::from_str::<StorageConfig>(&file_contents);
    match storage_config {
        Ok(res) => return Some(res),
        Err(_e) => return None,
    }
}

pub fn store_config(config: &SprintConfig) {
    let storage_config = StorageConfig {
        team_members: config.team_members,
        total_sprint_points: config.total_sprint_points,
        days_per_sprint: config.days_per_sprint,
    };
    let json_config =
        serde_json::to_string(&storage_config).expect("Error serializing config for storage");

    let mut json_file = File::create(FILE_NAME).expect("Error creating config file for storage");

    match json_file.write_all(json_config.as_bytes()) {
        Ok(_res) => return,
        Err(e) => println!("Error writing config file {}", e),
    };
}

pub fn reset_config() {
    let result = fs::remove_file(FILE_NAME);
    if result.is_ok() {
        println!("Reset configuration successful");
    }
}

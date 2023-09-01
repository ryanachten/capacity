use std::{
    fs::{self, File},
    io::{Error, Read, Write},
    path::PathBuf,
};

use crate::config::SprintConfig;
use directories::ProjectDirs;
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
    let config_path = get_config_directory().join(FILE_NAME);

    match File::open(config_path) {
        Ok(mut file) => {
            file.read_to_string(&mut file_contents)
                .expect("Error reading file contents");
        }
        Err(_err) => return None,
    }

    let storage_config = serde_json::from_str::<StorageConfig>(&file_contents);
    match storage_config {
        Ok(res) => Some(res),
        Err(_e) => None,
    }
}

pub fn store_config(config: &SprintConfig) -> Result<PathBuf, Error> {
    let storage_config = StorageConfig {
        team_members: config.team_members,
        total_sprint_points: config.total_sprint_points,
        days_per_sprint: config.days_per_sprint,
    };
    let json_config = serde_json::to_string(&storage_config)?;

    let config_directory = get_config_directory();
    if !config_directory.exists() {
        fs::create_dir_all(&config_directory)?;
    }

    let config_file_path = config_directory.join(FILE_NAME);
    let mut json_file = File::create(config_file_path)?;
    json_file.write_all(json_config.as_bytes())?;

    Ok(config_directory)
}

pub fn reset_config() {
    let config_path = get_config_directory().join(FILE_NAME);

    let result = fs::remove_file(config_path);
    if result.is_ok() {
        println!("Reset configuration successful");
    }
}

fn get_config_directory() -> PathBuf {
    /*
       Project directories are specific to each environment:
       - Lin: /home/alice/.config/sprint
       - Win: C:\Users\Alice\AppData\Roaming\sprint\capacity\config
       - Mac: /Users/Alice/Library/Application Support/com.sprint.capacity
    */
    let project_directory =
        ProjectDirs::from("com", "sprint", "capacity").expect("Couldn't get project directory");

    return project_directory.config_dir().to_path_buf();
}

use std::io;

use crate::storage::{load_config, store_config};

pub struct SprintConfig {
    pub team_members: i32,
    pub total_sprint_points: f32,
    pub days_per_sprint: f32,
    pub days_of_leave: f32,
}

pub fn get_config() -> SprintConfig {
    let stored_config = load_config();
    match stored_config {
        Some(config) => {
            let days_of_leave = get_days_of_leave();
            return SprintConfig {
                days_of_leave,
                team_members: config.team_members,
                total_sprint_points: config.total_sprint_points,
                days_per_sprint: config.days_per_sprint,
            };
        }
        None => return get_input_config(),
    }
}

fn get_input_config() -> SprintConfig {
    println!("Configuring Capacity...");

    let team_members = get_user_input::<i32>("Number of team members");
    let total_sprint_points = get_user_input::<f32>("Sprint points at full capacity");
    let days_per_sprint = get_user_input::<f32>("Total days this sprint");
    let days_of_leave = get_days_of_leave();

    let config = SprintConfig {
        team_members,
        total_sprint_points,
        days_of_leave,
        days_per_sprint,
    };

    let store_result = get_user_confirmation("Store configuration for next time? (y/n)");
    if store_result {
        store_config(&config);
    }

    return config;
}

fn get_user_input<T: std::str::FromStr>(prompt_msg: &str) -> T {
    println!("{}:", prompt_msg);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    match input.trim().parse::<T>() {
        Ok(parsed_input) => return parsed_input,
        Err(_error) => {
            println!("Error parsing '{}', try again...", input.trim());
            return get_user_input(prompt_msg);
        }
    }
}

fn get_user_confirmation(prompt_msg: &str) -> bool {
    println!("{}:", prompt_msg);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    match input.trim() {
        "yes" => return true,
        "y" => return true,
        "no" => return false,
        "n" => return false,
        _ => {
            println!("Invalid input '{}', try again...", input.trim());
            return get_user_confirmation(prompt_msg);
        }
    }
}

fn get_days_of_leave() -> f32 {
    return get_user_input::<f32>("Total days of leave this sprint");
}

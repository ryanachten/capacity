use colored::Colorize;
use std::io::{stdin, stdout, Write};

use crate::{
    print::{subheading, warning},
    storage::{load_config, store_config},
};

const TEAM_MEMBERS_PREFIX: &str = "🙂 Number of team members";
const SPRINT_POINTS_PREFIX: &str = "✋ Sprint points at full capacity";
const DAYS_PER_SPRINT_PREFIX: &str = "📆 Total days this sprint";

pub struct SprintConfig {
    pub team_members: i32,
    pub total_sprint_points: f32,
    pub days_per_sprint: f32,
    pub days_of_leave: f32,
}

pub fn get_config() -> SprintConfig {
    match load_config() {
        Some(config) => {
            subheading("Using stored configuration");
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

pub fn print_config() {
    match load_config() {
        Some(res) => {
            subheading("Current stored configuration");
            println!(
                "{}: {}",
                TEAM_MEMBERS_PREFIX,
                res.team_members.to_string().bold()
            );
            println!(
                "{}: {}",
                SPRINT_POINTS_PREFIX,
                res.total_sprint_points.to_string().bold()
            );
            println!(
                "{}: {}",
                DAYS_PER_SPRINT_PREFIX,
                res.days_per_sprint.to_string().bold()
            );
        }
        None => warning("No stored configuration"),
    }
}

fn get_input_config() -> SprintConfig {
    subheading("Configuring capacity");

    let team_members = get_user_input::<i32>(TEAM_MEMBERS_PREFIX);
    let total_sprint_points = get_user_input::<f32>(SPRINT_POINTS_PREFIX);
    let days_per_sprint = get_user_input::<f32>(DAYS_PER_SPRINT_PREFIX);
    let days_of_leave = get_days_of_leave();

    let config = SprintConfig {
        team_members,
        total_sprint_points,
        days_of_leave,
        days_per_sprint,
    };

    let store_result = get_user_confirmation("💾 Store configuration for next time? (y/n)");
    if store_result {
        store_config(&config);
    }

    return config;
}

fn get_user_input<T: std::str::FromStr>(prompt_msg: &str) -> T {
    print!("{}: ", prompt_msg);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    match input.trim().parse::<T>() {
        Ok(parsed_input) => return parsed_input,
        Err(_error) => {
            warning(&format!("Invalid input '{}', try again...", input.trim()));
            return get_user_input(prompt_msg);
        }
    }
}

fn get_user_confirmation(prompt_msg: &str) -> bool {
    print!("{}: ", prompt_msg);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    match input.to_lowercase().trim() {
        "yes" => return true,
        "y" => return true,
        "no" => return false,
        "n" => return false,
        _ => {
            warning(&format!("Invalid input '{}', try again...", input.trim()));
            return get_user_confirmation(prompt_msg);
        }
    }
}

fn get_days_of_leave() -> f32 {
    return get_user_input::<f32>("📆 Total days of leave this sprint");
}

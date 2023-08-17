use crate::config::{get_config, SprintConfig};
use colored::Colorize;

#[cfg(test)]
mod tests;

pub struct SprintCapacity {
    proposed_sprint_points: f32,
    sprint_capacity: f32,
}

pub fn get_capacity() {
    let config = get_config();
    let capacity = calculate_capacity(&config);
    println!(
        "👉 {} {:.1} sprint points, at {:.1}% team capacity",
        "Capacity:".bold(),
        capacity.proposed_sprint_points,
        capacity.sprint_capacity
    )
}

pub fn calculate_capacity(config: &SprintConfig) -> SprintCapacity {
    let total_days = config.days_per_sprint * config.team_members as f32;
    let capacity = 1.0 - config.days_of_leave / total_days;
    let proposed_sprint_points = capacity * config.total_sprint_points;
    let sprint_capacity = capacity * 100.0;

    return SprintCapacity {
        proposed_sprint_points,
        sprint_capacity,
    };
}

use super::*;

#[test]
fn determines_full_capacity() {
    let result = calculate_capacity(&SprintConfig {
        team_members: 1,
        total_sprint_points: 2.0,
        days_per_sprint: 1.0,
        days_of_leave: 0.0,
    });
    assert_eq!(result.proposed_sprint_points, 2.0);
    assert_eq!(result.sprint_capacity, 100.0);
}

#[test]
fn determines_no_capacity() {
    let result = calculate_capacity(&SprintConfig {
        team_members: 1,
        total_sprint_points: 2.0,
        days_per_sprint: 1.0,
        days_of_leave: 1.0,
    });
    assert_eq!(result.proposed_sprint_points, 0.0);
    assert_eq!(result.sprint_capacity, 0.0);
}

#[test]
fn determines_reduced_capacity() {
    let result = calculate_capacity(&SprintConfig {
        team_members: 4,
        total_sprint_points: 10.0,
        days_per_sprint: 14.0,
        days_of_leave: 28.0,
    });
    assert_eq!(result.proposed_sprint_points, 5.0);
    assert_eq!(result.sprint_capacity, 50.0);
}

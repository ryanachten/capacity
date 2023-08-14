use capacity::get_capacity;
use cli::{get_run_mode, RunMode};
use config::print_config;
use storage::reset_config;

mod capacity;
mod cli;
mod config;
mod storage;

pub fn run() {
    match get_run_mode() {
        RunMode::ResetConfig => reset_config(),
        RunMode::PrintConfig => print_config(),
        _ => get_capacity(),
    }
}

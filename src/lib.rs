use args::RunMode;
use capacity::get_capacity;
use storage::reset_config;

use crate::args::get_run_mode;

mod args;
mod capacity;
mod config;
mod storage;

pub fn run() {
    let mode = get_run_mode();
    match mode {
        RunMode::ResetConfig => reset_config(),
        _ => get_capacity(),
    }
}

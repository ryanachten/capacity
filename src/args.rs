use std::env;

pub enum RunMode {
    GetCapacity,
    ResetConfig,
}

enum Command {
    Config,
    Reset,
}

const DEFAULT_MODE: RunMode = RunMode::GetCapacity;

pub fn get_run_mode() -> RunMode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return DEFAULT_MODE;
    }

    let first_command = get_command(&args[1]);

    if args.len() > 2 {
        let second_command = get_command(&args[2]);
        match (first_command, second_command) {
            (Some(Command::Config), Some(Command::Reset)) => return RunMode::ResetConfig,
            _ => return DEFAULT_MODE,
        }
    }

    return DEFAULT_MODE;
}

fn get_command(arg: &String) -> Option<Command> {
    match arg.to_lowercase().trim() {
        "config" => return Some(Command::Config),
        "reset" => return Some(Command::Reset),
        _ => None,
    }
}

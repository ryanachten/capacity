use clap::{arg, command, ArgAction, Command};

pub enum RunMode {
    GetCapacity,
    PrintConfig,
    ResetConfig,
}

pub fn get_run_mode() -> RunMode {
    let matches = command!()
        .subcommand(
            Command::new("config")
                .about("View and modify sprint configuration")
                .arg(arg!(-r --reset "Resets stored configuration").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        if matches.get_flag("reset") {
            return RunMode::ResetConfig;
        }
        return RunMode::PrintConfig;
    }

    return RunMode::GetCapacity;
}

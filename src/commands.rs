mod install;
mod remove;

use clap::{Arg, Command};

// Project info
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

// Commands Manager
pub fn commandsRun() {
    let matches = Command::new("MewManager")
        .version(VERSION)
        .author("https://github.com/n3k0girl/MewManager")
        .about(NAME)
        .arg(Arg::new("install").short('i').long("install"))
        .arg(Arg::new("remove").short('r').long("remove"))
        .arg_required_else_help(true)
        .get_matches();

    // Context of commands
    install::main(matches.get_one::<String>("install").cloned());
    remove::main(matches.get_one::<String>("remove").cloned());
}
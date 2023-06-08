mod install;
mod remove;
mod checkfmt;

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
    let install_pkg: String = checkfmt::checkFormat(matches.get_one::<String>("install").cloned());
    let remove_pkg: String = checkfmt::checkFormat(matches.get_one::<String>("remove").cloned());
    
    if install_pkg != "NoPKG" {
        install::main(install_pkg);
    }

    if remove_pkg != "NoPKG" {
        remove::main(remove_pkg);
    }
}
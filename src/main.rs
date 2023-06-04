#![allow(non_snake_case)]
use nix::unistd::Uid;

mod commands;

fn main() {
    
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions.");
    }

    commands::commandsRun();
}

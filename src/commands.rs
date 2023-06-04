use clap::{arg, ArgAction, Command};

pub fn commandsRun() {
    let matches = Command::new("MewManager")
        .version("1.0")
        .author("https://github.com/n3k0girl/MewManager")
        .about("MewManager")
        .arg(arg!(--install <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!("install: {:?}", matches.get_one::<String>("install").expect("required"));
    println!("one: {:?}", matches.get_one::<String>("one").expect("required"));
}
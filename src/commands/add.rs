use clap::{Command, ArgMatches};

pub fn register() -> Command {
    Command::new("add")
        .about("Add a new item")
        .arg(Arg::new("name")
        .help("Name of the item")
        .required(true))
}

pub fn execute(sub_m: &ArgMatches) {
    let name = sub_m.value_of("name").unwrap();
    println!("Adding {}", name);
}
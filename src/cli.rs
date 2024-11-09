use clap::{Arg, Command};
pub fn build_cli() -> Command {
    Command::new("cli_basics")
        .version("1.0")
        .author("rhanspal <rhanspal25@gmail.com>")
        .about("A description of your CLI")
        .subcommand(
            Command::new("add")
                .subcommand(Command::new("new").arg(Arg::new("force")).about("asosalkdsakljdklasjd"))
                .about("Add a new item")
                .arg(Arg::new("name").help("Name of the item").required(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an item")
                .arg(Arg::new("id").help("ID of the item").required(true)),
        )
}
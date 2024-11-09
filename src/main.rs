mod cli;
use cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            println!("Adding - {}", name);
        }
        Some(("remove", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            println!("Removing - {}", id);
        }
        _ => println!("No valid subcommand used"),
    }
}

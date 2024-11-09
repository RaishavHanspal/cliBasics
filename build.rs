use clap_markdown::help_markdown_command;
use std::env;
use std::path::Path;

include!("src/cli.rs");

fn main() {
    let release_enabled = env::var("PROFILE").map_or(false, |profile| profile == "release");
    if release_enabled {
        // let out_dir = env::var("OUT_DIR").unwrap();
        let out_path = Path::new("./");
        
        let mut app = build_cli();
    
    // Generate markdown documentation
    let markdown = help_markdown_command(&mut app);
    println!("cargo:warning=markdown={}", markdown);
        std::fs::write(out_path.join("cli-docs.md"), markdown).expect("Could not write markdown docs");

        println!("Documentation and completions generated at {}", out_path.display());
    }
}

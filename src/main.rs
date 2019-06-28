// src/main.rs

mod cli;

use liboverdrop::FragmentScanner;

fn main() {
    let matches = cli::build_cli().get_matches();
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("scan") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }

        let scan_dir = matches.value_of("directory").unwrap_or("./");
        println!("Scanning directory {}", scan_dir);

        let base_dirs = vec![scan_dir.to_string()];
        let allowed_extensions = vec![];
        let od_cfg = FragmentScanner::new(base_dirs, "", false, allowed_extensions);

        let fragments = od_cfg.scan();
        for (filename, filepath) in fragments {
            println!(
                "fragment '{}' located at '{}'",
                filename,
                filepath.display()
            );
        }
    }
}

use std::path::Path;

use blocked_bsgs_rho_wall_verifier::{verify_all, verify_fixtures, verify_seed_mapping};

fn usage() -> ! {
    eprintln!(
        "usage: blocked-bsgs-rho-wall-verifier \
         <verify-fixtures|verify-seeds DERIVED_SEEDS_JSON|verify-all STUDY_DIR>"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, command] if command == "verify-fixtures" => verify_fixtures(),
        [_, command, path] if command == "verify-seeds" => verify_seed_mapping(Path::new(path)),
        [_, command, path] if command == "verify-all" => verify_all(Path::new(path)),
        _ => usage(),
    };
    match result {
        Ok(report) => println!("{}", serde_json::to_string_pretty(&report).unwrap()),
        Err(error) => {
            eprintln!("verification failed: {error}");
            std::process::exit(1);
        }
    }
}

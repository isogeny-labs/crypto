fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "smith".into());
    let result = match mode.as_str() {
        "smith" => p256_cover_classification::actual_audit(),
        "controls" => p256_cover_classification::reference_controls(),
        "catalogue" => {
            let v = p256_cover_classification::catalogue();
            p256_cover_classification::validate_worked_catalogue(&v).unwrap();
            v
        }
        _ => {
            eprintln!("usage: p256-cover-classification [smith|controls|catalogue]");
            std::process::exit(2)
        }
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn main() {
    let result = match std::env::args().nth(1).as_deref() {
        None | Some("toy") => p256_folded_walk::correctness_report(),
        Some("trace") => p256_folded_walk::full_field_trace(),
        _ => {
            eprintln!("usage: p256-folded-walk [toy|trace]");
            std::process::exit(2)
        }
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

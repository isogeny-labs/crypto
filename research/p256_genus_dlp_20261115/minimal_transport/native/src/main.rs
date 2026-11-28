fn main() {
    if std::env::args().nth(1).as_deref() != Some("qualify") {
        eprintln!("usage: p256-minimal-transport qualify (requires admitted immutable qualifier)");
        std::process::exit(2);
    }
    match p256_minimal_transport::qualification() {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).unwrap()),
        Err(error) => {
            eprintln!("qualification failed: {error}");
            std::process::exit(1);
        }
    }
}

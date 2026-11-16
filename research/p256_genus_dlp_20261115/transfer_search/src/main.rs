fn main() {
    let output = match std::env::args().nth(1).as_deref() {
        None | Some("audit") => p256_transfer_search::audit(),
        Some("identities") => p256_transfer_search::identity_certificates(),
        _ => {
            eprintln!("usage: p256-transfer-search [audit|identities]");
            std::process::exit(2)
        }
    };
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

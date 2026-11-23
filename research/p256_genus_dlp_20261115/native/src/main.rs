fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "verify".into());
    let result = match mode.as_str() {
        "verify" => p256_genus_check::verify_actual(),
        "toy-correctness" => p256_genus_check::toy_correctness(),
        "toy-zeta" => p256_genus_check::toy_zeta(),
        "certificates" => p256_genus_check::certificates(),
        _ => {
            eprintln!("usage: p256-genus-check [verify|toy-correctness|toy-zeta|certificates]");
            std::process::exit(2)
        }
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

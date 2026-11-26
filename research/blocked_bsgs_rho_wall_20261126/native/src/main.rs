fn main() {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("usage: blocked-bsgs-rho-wall <verify-fixtures|emit-seeds|run-cold|run-aa|run-amortized>");
        std::process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("external parameters are not accepted");
        std::process::exit(2);
    }
    match blocked_bsgs_rho_wall::dispatch(&command) {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON output")),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

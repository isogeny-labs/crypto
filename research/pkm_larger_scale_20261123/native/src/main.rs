#[path = "../../../../src/bin/ic/prime_workload.rs"]
mod workload;

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("generate") if args.len() == 6 => {
            let spec = serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let count = args[3].parse().map_err(|_| "count must be an integer")?;
            let seed = args[4].parse().map_err(|_| "seed must be a u64")?;
            let doc = workload::Workload::generate(spec, count, seed, &args[5])?;
            let sha = doc.write_new(std::path::Path::new(&args[2]))?;
            println!("{}", serde_json::json!({"schema":"ic.prime-workload-generation/v1","sha256":sha,"targets":doc.targets.len(),"measured":false}));
        }
        Some("validate") if args.len() == 2 => {
            let validated = workload::load(std::path::Path::new(&args[1]))?;
            println!("{}", serde_json::json!({"schema":"ic.prime-workload-validation/v1","sha256":validated.sha256,"targets":validated.document.targets.len(),"curve_validation":validated.curve.validation,"measured":false}));
        }
        _ => return Err("usage: pkm-prime-workload generate CURVE.json NEW_WORKLOAD.json COUNT SEED DOMAIN | validate WORKLOAD.json".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

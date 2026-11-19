use ghs_matched_frontier_gate::{
    read_json, selftest, validate_protocol, validate_results, validate_templates,
};
use std::env;
use std::path::Path;

fn run() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    match args.as_slice() {
        [_, command] if command == "selftest" => {
            selftest()?;
            println!("GHS M3 native validator self-test PASS");
        }
        [_, command, protocol, templates] if command == "protocol" => {
            validate_protocol(&read_json(Path::new(protocol))?)?;
            validate_templates(&read_json(Path::new(templates))?)?;
            println!(
                "{{\"calibrations\":33,\"diagnostic_runs\":27,\"grid_sizes\":11,\"launch_ready\":false,\"primary_runs\":141,\"status\":\"contract_valid\"}}"
            );
        }
        [_, command, protocol, manifest] if command == "results" => {
            let summary = validate_results(Path::new(protocol), Path::new(manifest))?;
            println!(
                "{}",
                serde_json::to_string(&summary).map_err(|error| error.to_string())?
            );
        }
        _ => {
            return Err(
                "usage: ghs-m3-validate selftest | protocol PROTOCOL TEMPLATES | results PROTOCOL MANIFEST"
                    .into(),
            );
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ghs-m3-validate: refused: {error}");
        std::process::exit(2);
    }
}

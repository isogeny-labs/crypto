use fall_degree_replay::large_curves;
use std::{env, fs, path::PathBuf};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.as_slice() {
        [command] if command=="list" => {
            for name in large_curves::curve_names() { println!("{name}"); }
        }
        [command,name,out] if command=="issue" => {
            let (problem,witness) = large_curves::fixture_json(name)?;
            let dir = PathBuf::from(out);
            fs::create_dir_all(&dir)?;
            let witness_json = serde_json::to_string_pretty(&witness)?+"\n";
            let checked = large_curves::verify_fixture_json(&problem,&witness_json)?;
            fs::write(dir.join("problem.json"),problem)?;
            fs::write(dir.join("curve-witness.json"),witness_json)?;
            fs::write(dir.join("curve-check.json"),serde_json::to_string_pretty(&checked)?+"\n")?;
            println!("{}",serde_json::to_string_pretty(&checked)?);
        }
        [command,problem,witness] if command=="check" => {
            let checked = large_curves::verify_fixture_json(&fs::read_to_string(problem)?,&fs::read_to_string(witness)?)?;
            println!("{}",serde_json::to_string_pretty(&checked)?);
        }
        _ => return Err("usage: large_curve_fixtures list | issue CURVE OUTDIR | check PROBLEM.json CURVE-WITNESS.json".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("large_curve_fixtures: {error}");
        std::process::exit(1);
    }
}

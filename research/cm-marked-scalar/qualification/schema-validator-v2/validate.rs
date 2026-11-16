use std::{env,fs,process};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args:Vec<String>=env::args().collect();
    if args.len()!=3 { eprintln!("validate SCHEMA INSTANCE"); process::exit(2); }
    let schema:serde_json::Value=serde_json::from_slice(&fs::read(&args[1])?)?;
    let instance:serde_json::Value=serde_json::from_slice(&fs::read(&args[2])?)?;
    let validator=jsonschema::validator_for(&schema)?;
    let mut errors=0;
    for error in validator.iter_errors(&instance) { eprintln!("{}: {}",error.instance_path,error); errors+=1; }
    if errors>0 { process::exit(1); }
    println!("SCHEMA_VALIDATION_PASS");
    Ok(())
}

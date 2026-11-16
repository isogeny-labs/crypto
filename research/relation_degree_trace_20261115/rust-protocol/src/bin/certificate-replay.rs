//! Construct and export proof witnesses. Timing orchestration is a separate task.
use certified_relations::{replay, System};
use std::{error::Error, fs, io::Write, path::Path};
fn read(path: &str, cap: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    if fs::metadata(path)?.len() > cap {
        return Err("file byte cap".into());
    }
    Ok(fs::read(path)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: certificate-replay produce INPUT compact|expanded NEW_WITNESS OR certificate-replay script INPUT WITNESS NEW_SCRIPT".into());
    }
    let system: System = serde_json::from_slice(&read(&args[2], 4 * 1024 * 1024)?)?;
    let bytes = match args[1].as_str() {
        "produce" => {
            let compact = match args[3].as_str() {
                "compact" => true,
                "expanded" => false,
                _ => return Err("unknown constructor".into()),
            };
            serde_json::to_vec_pretty(&replay::produce(&system, compact)?)?
        }
        "script" => replay::singular_script(
            &system,
            &serde_json::from_slice(&read(&args[3], 32 * 1024 * 1024)?)?,
        )?
        .into_bytes(),
        _ => return Err("unknown mode".into()),
    };
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(Path::new(&args[4]))?
        .write_all(&bytes)?;
    Ok(())
}

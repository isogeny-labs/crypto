use std::{error::Error, fs, path::Path};
fn read(path: &str, cap: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    if fs::metadata(Path::new(path))?.len() > cap {
        return Err("file byte cap".into());
    }
    Ok(fs::read(path)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: relation-replay-checker SYSTEM WITNESS".into());
    }
    let checked = relation_replay_checker::verify_json(
        &read(&a[1], 4 * 1024 * 1024)?,
        &read(&a[2], 16 * 1024 * 1024)?,
    )?;
    println!("{}", serde_json::to_string(&checked)?);
    Ok(())
}

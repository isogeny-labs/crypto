//! Owner-side public-input separation; preserve u64 values exactly.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("usage: PREPARE PRIVATE_FIXTURE NEW_PUBLIC_JSON NEW_MANIFEST".into());
    }
    let fixture: Value = serde_json::from_slice(&fs::read(&a[0])?)?;
    let public = &fixture["public"];
    let object = public.as_object().ok_or("public object")?;
    if object.len() != 5
        || object
            .keys()
            .any(|k| !["p", "l", "alpha", "g", "q"].contains(&k.as_str()))
        || fixture["expected_scalar"].as_u64().is_none()
    {
        return Err("fixture registration".into());
    }
    let mut bytes = serde_json::to_vec(public)?;
    bytes.push(b'\n');
    let sha = format!("{:x}", Sha256::digest(&bytes));
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[1])?;
    f.write_all(&bytes)?;
    let manifest = json!({"schema":"nagao.e2e.public-input/v1","source_revision":fixture["source_revision"],"generator_seed":fixture["seed"],"public_input_sha256":sha,"public_input_bytes":bytes.len(),"p":public["p"],"l":public["l"],"worker_receives":"public input only"});
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[2])?;
    serde_json::to_writer_pretty(&mut f, &manifest)?;
    f.write_all(b"\n")?;
    Ok(())
}

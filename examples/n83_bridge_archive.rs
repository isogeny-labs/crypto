//! Bind and extract a raw GitHub Actions N83 bridge artifact with native SHA-256 receipts.
//! Usage: n83_bridge_archive <artifact.zip> <new-extraction-dir> <manifest.json>

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path};
use std::process::Command;

use crypto_lib::hash::sha256::Sha256;
use serde_json::{json, Value};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn fail(message: impl Into<String>) -> Box<dyn std::error::Error> {
    message.into().into()
}

fn file_sha(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut block = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut block)?;
        if count == 0 {
            break;
        }
        hasher.update(&block[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}
fn run_command(args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("/usr/bin/unzip")
        .args(args)
        .env_clear()
        .output()?;
    if !output.status.success() {
        return Err(fail(format!(
            "unzip {:?}: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}
fn archive(zip: &Path, out: &Path, manifest: &Path) -> Result<Value> {
    if out.exists() || manifest.exists() {
        return Err(fail("extraction or manifest already exists"));
    }
    let zip_text = zip.to_str().ok_or("artifact path is not UTF-8")?;
    let names = String::from_utf8(run_command(&["-Z1", zip_text])?)?;
    let mut safe = Vec::new();
    let mut seen = HashSet::new();
    for name in names.lines() {
        if name.ends_with('/') {
            continue;
        }
        let path = Path::new(name);
        if name.contains('\\')
            || !seen.insert(name.to_owned())
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(fail(format!("unsafe or duplicate archive member: {name}")));
        }
        safe.push(name.to_owned());
    }
    if safe.is_empty() || !safe.iter().any(|name| name == "results/analysis.json") {
        return Err(fail("artifact lacks analysis or members"));
    }
    run_command(&["-tqq", zip_text])?;
    fs::create_dir(out)?;
    let out_text = out.to_str().ok_or("extraction path is not UTF-8")?;
    run_command(&["-q", zip_text, "-d", out_text])?;
    let mut members = Vec::new();
    let mut total_bytes = 0u64;
    let canonical_out = out.canonicalize()?;
    for name in safe {
        let path = out.join(&name);
        if path.symlink_metadata()?.file_type().is_symlink()
            || !path.canonicalize()?.starts_with(&canonical_out)
        {
            return Err(fail(format!("archive member escaped extraction: {name}")));
        }
        let size = path.metadata()?.len();
        total_bytes += size;
        members.push(json!({"path":name,"bytes":size,"sha256":file_sha(&path)?}));
    }
    let result = json!({"schema":"n83.native-bridge-archive/v1",
                        "artifact_sha256":file_sha(zip)?,
                        "artifact_bytes":zip.metadata()?.len(),
                        "member_count":members.len(),"member_bytes":total_bytes,
                        "members":members});
    let mut bytes = serde_json::to_vec_pretty(&result)?;
    bytes.push(b'\n');
    let mut stream = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(manifest)?;
    stream.write_all(&bytes)?;
    stream.sync_all()?;
    Ok(result)
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err(fail(
            "usage: n83_bridge_archive <artifact.zip> <new-extraction-dir> <manifest.json>",
        ));
    }
    let result = archive(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
    )?;
    println!(
        "{}",
        json!({"status":"PASS_BOUND_ARCHIVE",
                           "artifact_sha256":result["artifact_sha256"],
                           "member_count":result["member_count"]})
    );
    Ok(())
}

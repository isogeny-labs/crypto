//! Small, explicit process/file boundary. No shell evaluation of request data.
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("UTC clock")
        .as_secs()
}
pub fn error(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub fn atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(error)?;
    let temp = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut f = File::create(&temp).map_err(error)?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(error)?;
    fs::rename(&temp, path).map_err(error)?;
    File::open(parent).and_then(|f| f.sync_all()).map_err(error)
}

pub fn save(path: &Path, v: &impl Serialize) -> Result<(), String> {
    atomic(path, &serde_json::to_vec_pretty(v).map_err(error)?)
}
pub fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    serde_json::from_slice(&fs::read(path).map_err(error)?).map_err(error)
}
pub fn sha_file(path: &Path) -> Result<(String, u64), String> {
    let mut file = File::open(path).map_err(error)?;
    let mut sha = Sha256::new();
    let mut count = 0;
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf).map_err(error)?;
        if n == 0 {
            break;
        }
        sha.update(&buf[..n]);
        count += n as u64;
    }
    Ok((format!("{:x}", sha.finalize()), count))
}
pub fn output(cmd: &mut Command) -> Result<Output, String> {
    let result = cmd.output().map_err(error)?;
    if !result.status.success() {
        return Err(format!(
            "command {:?} failed ({}): {}",
            cmd.get_program(),
            result.status,
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(result)
}
pub fn json(cmd: &mut Command) -> Result<serde_json::Value, String> {
    serde_json::from_slice(&output(cmd)?.stdout).map_err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_atomic_file_has_a_durable_current_directory_parent() {
        let name = format!("isolab-atomic-regression-{}.json", std::process::id());
        let path = Path::new(&name);
        atomic(path, b"first").unwrap();
        atomic(path, b"replacement").unwrap();
        assert_eq!(fs::read(path).unwrap(), b"replacement");
        fs::remove_file(path).unwrap();
    }
}

//! Build and verify the committed source archive used by both N83 hosts.
use super::Result;
use flate2::{read::GzDecoder, Compression, GzBuilder};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path};
use std::process::{Command, Stdio};

pub const PATHS: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "src",
    "examples",
    "docs/curves/registry.json",
    "docs/curves/traits.json",
    "docs/ic/boundary_targets.json",
    "docs/ic/calibration.json",
    "docs/ecbench/schema.sql",
    "docs/identity-certificates/schoenhage-10mult.json",
    "tests/fixtures/ecbench_manifest_small.json",
    "research/n83_cold_factor_base_20261016",
    "research/n83_compact4_crosslayout_20261016",
    "research/n83_compact4_crosslayout_results_20261016",
    "research/n83_crossfamily_current_20261025",
    "research/n83_dense_root_20261013",
    "research/n83_e2e_factor_base_20261016",
    "research/n83_e2e_final_holdout_20261016",
    "research/n83_e2e_finegrid_20261016",
    "research/n83_e2e_holdout_20261016",
    "research/n83_factor_base_100_results_20261014",
    "research/n83_highk_beyond4405_20261021",
    "research/n83_indexed5_20261013",
    "research/n83_k3072_current_source_20261017",
    "research/n83_k3300_current_source_prefix_20261025",
    "research/n83_k3596_current_source_full_20261025",
    "research/n83_nearby_full_grid_20261024",
    "research/n83_nearby_grid_confirm16_20261024",
    "research/n83_nearby_grid_tier1_20261024",
    "research/n83_online_finalists_20261018",
    "research/n83_root11_dynamic_20261016",
    "research/n83_root4_20261014",
    "research/n83_smallk_challenge_20261017",
];

fn digest(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut state = Sha256::new();
    let mut block = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut block)?;
        if count == 0 {
            break;
        }
        state.update(&block[..count]);
    }
    Ok(hex::encode(state.finalize()))
}

fn members(archive: &Path) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    let mut tar = tar::Archive::new(GzDecoder::new(File::open(archive)?));
    for entry in tar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        if path.as_os_str().is_empty()
            || !path.components().all(|c| matches!(c, Component::Normal(_)))
        {
            return Err("unsafe source archive path".into());
        }
        if entry.header().entry_type().is_dir() {
            continue;
        }
        // Git archives carry the commit ID in a PAX global header. Python's
        // historical tar reader hides this metadata entry from its member map.
        if matches!(entry.header().entry_type().as_byte(), b'g' | b'x') {
            continue;
        }
        if !entry.header().entry_type().is_file() {
            return Err(format!(
                "unsupported source archive member {} type {}",
                path.display(),
                entry.header().entry_type().as_byte()
            )
            .into());
        }
        let name = path.to_str().ok_or("non-UTF8 archive member")?.to_owned();
        let mut state = Sha256::new();
        let mut block = [0u8; 1024 * 1024];
        loop {
            let count = entry.read(&mut block)?;
            if count == 0 {
                break;
            }
            state.update(&block[..count]);
        }
        if result.insert(name, hex::encode(state.finalize())).is_some() {
            return Err("duplicate source archive member".into());
        }
    }
    Ok(result)
}

pub fn verify(directory: &Path, root: Option<&Path>) -> Result<Value> {
    let archive = directory.join("source.tar.gz");
    let manifest_path = directory.join("manifest.json");
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let expected = manifest["members_sha256"]
        .as_object()
        .ok_or("member map absent")?;
    let actual = members(&archive)?;
    if manifest["schema"] != "n83.crossfamily-source-bundle/v1"
        || manifest["source_head_sha"]
            .as_str()
            .is_none_or(|x| x.len() != 40)
        || manifest["source_archive_sha256"] != digest(&archive)?
        || manifest["paths"] != json!(PATHS)
        || expected.len() != actual.len()
        || actual
            .iter()
            .any(|(name, sha)| expected.get(name).and_then(Value::as_str) != Some(sha))
    {
        return Err("source archive or manifest differs".into());
    }
    if let Some(root) = root {
        for (name, sha) in actual {
            let path = root.join(&name);
            if !path.is_file() || path.is_symlink() || digest(&path)? != sha {
                return Err(format!("extracted source differs: {name}").into());
            }
        }
    }
    Ok(json!({"source_head_sha":manifest["source_head_sha"],
        "source_archive_sha256":manifest["source_archive_sha256"],
        "source_manifest_sha256":digest(&manifest_path)?,"file_count":expected.len()}))
}

pub fn build(root: &Path, output: &Path) -> Result<Value> {
    if output.exists() {
        return Err("source bundle output already exists".into());
    }
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()?;
    if !status.status.success() || !status.stdout.is_empty() {
        return Err("source checkout must be clean".into());
    }
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()?;
    if !head.status.success() {
        return Err("source head absent".into());
    }
    let head = String::from_utf8(head.stdout)?.trim().to_owned();
    fs::create_dir(output)?;
    let archive_path = output.join("source.tar.gz");
    let mut command = Command::new("git");
    command
        .args(["archive", "--format=tar", "HEAD", "--"])
        .args(PATHS)
        .current_dir(root)
        .stdout(Stdio::piped());
    let mut child = command.spawn()?;
    let mut source = child.stdout.take().ok_or("git archive pipe absent")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&archive_path)?;
    let mut encoder = GzBuilder::new()
        .mtime(0)
        .filename("")
        .write(file, Compression::new(6));
    std::io::copy(&mut source, &mut encoder)?;
    encoder.finish()?.sync_all()?;
    if !child.wait()?.success() {
        return Err("git source archive failed".into());
    }
    let manifest = json!({"schema":"n83.crossfamily-source-bundle/v1",
        "source_head_sha":head,"source_archive_sha256":digest(&archive_path)?,
        "paths":PATHS,"members_sha256":members(&archive_path)?});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join("manifest.json"))?;
    file.write_all(&serde_json::to_vec(&manifest)?)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    verify(output, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_paths_are_unique_and_normal() {
        let mut names = std::collections::BTreeSet::new();
        for name in PATHS {
            assert!(names.insert(name));
            assert!(Path::new(name)
                .components()
                .all(|c| matches!(c, Component::Normal(_))));
        }
    }
}

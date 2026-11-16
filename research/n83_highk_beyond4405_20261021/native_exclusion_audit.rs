//! Native replay of the sealed N83 fixed31 point exclusion audit.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const HERE: &str = "research/n83_highk_beyond4405_20261021";
const PRIOR: &str = "research/n83_highk_online_tiebreak_20261020/fresh_exclusion_audit.json";
const PRIOR_SHA: &str = "82919779c004b8ed6b703bbfa5d4bd24b2b24f38e8fa3b4e8fb1b10f78b42b9e";
const MIGRATION_PARENT: &str = "ced161f948f0aac06de2354e757e92b5735bedcb";
const EXTRA: &[(&str, &str, &str)] = &[
    (
        "HEAD",
        "research/n83_highk_online_tiebreak_20261020/screen_targets.jsonl",
        "41b160dc56fadb0e8beec4c86bfee47457c6ade8290dd17cfa30c728787d5582",
    ),
    (
        "753b6223493688aaff3b3c8881cb487228ef90e6",
        "research/n83_cold_base_confirm_20261021/targets.jsonl",
        "613bb1ad135f3bfc4632fdc3543eb91953571f1ac91fd36b5d31fece5a04f67c",
    ),
    (
        "48501c90100223ebf9597ed223692c0f24ce1f8f",
        "research/n83_fresh_cold_binary_retest_20261021/fresh_targets.jsonl",
        "c604d7d42468d16636f4d0a8f6fe4caa75b91104d85287743b868e04ce5098e4",
    ),
    (
        "HEAD",
        "research/n83_fresh_cold_finalists_20261019/fresh_targets.jsonl",
        "174a6c56bbd54d419df3363e700660a08578612fbb5b8d7f25ed54c5eb622e4a",
    ),
    (
        "HEAD",
        "research/n83_fresh_cold_finalists_20261019/historical_finalist_targets.jsonl",
        "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58",
    ),
    (
        "HEAD",
        "research/n83_global_bridge_20261018/fresh_targets.jsonl",
        "a767c6acb37174593b8dd50cddfac7ed5be02498a00d199754a234606a8cbd64",
    ),
    (
        "HEAD",
        "research/n83_k5086_base_20261021/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_load80_online_pair_20261020/targets.jsonl",
        "5d5482355731bc2362f7a3b612c70fba13ba326f97e153a4610138bda5a14abd",
    ),
    (
        "HEAD",
        "research/n83_root4_load80_20261020/reports/run-20261020T1604Z/raw/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/maxk_holdout_targets.jsonl",
        "96375c178e8315c136f190927dc3470fc84f1b7096c2e563387808928aa56d6a",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/maxk_screen_targets.jsonl",
        "b99b196b43e69414c101e948ca74ea32703d1ea87ad71e0109e01b37f8cafe38",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/runs/base_only_k128_20261021/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/runs/base_only_k4405_20261021/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/runs/base_only_k5500_20261021/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/runs/k4405_monolithic_local_7200/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
    (
        "HEAD",
        "research/n83_two_tier_root_20261021/runs/k4405_split_shared_local_7200/target.jsonl",
        "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656",
    ),
];

type Result<T> = std::result::Result<T, String>;
type Point = (u128, u128);

fn digest(blob: &[u8]) -> String {
    format!("{:x}", Sha256::digest(blob))
}

fn git_show(root: &Path, revision: &str, path: &str) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("show")
        .arg(format!("{revision}:{path}"))
        .output()
        .map_err(|e| format!("git show {revision}:{path}: {e}"))?;
    if !output.status.success() {
        return Err(format!("git show {revision}:{path} failed"));
    }
    Ok(output.stdout)
}

fn migrated_blob_from(
    root: &Path,
    path: &str,
    expected_sha: &str,
    parent: &str,
) -> Result<Vec<u8>> {
    let fixture = Path::new(path);
    let manifest_path = fixture
        .parent()
        .ok_or_else(|| format!("fixture has no parent: {path}"))?
        .join(".artifacts.json");
    let manifest_path = manifest_path.to_string_lossy();
    let manifest: Value = serde_json::from_slice(&git_show(root, "HEAD", &manifest_path)?)
        .map_err(|e| format!("artifact manifest parse failed for {path}: {e}"))?;
    let filename = fixture
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid fixture name")?;
    let entry = &manifest["files"][filename];
    if manifest["schema"] != "artifacts.manifest/v1"
        || entry["sha256"].as_str() != Some(expected_sha)
    {
        return Err(format!("artifact manifest differs: {path}"));
    }
    let bytes = entry["bytes"]
        .as_u64()
        .ok_or_else(|| format!("artifact byte count differs: {path}"))?;
    let blob = git_show(root, parent, path)?;
    if blob.len() as u64 != bytes {
        return Err(format!("artifact size differs: {path}"));
    }
    Ok(blob)
}

fn migrated_blob(root: &Path, path: &str, expected_sha: &str) -> Result<Vec<u8>> {
    migrated_blob_from(root, path, expected_sha, MIGRATION_PARENT)
}

fn verified_blob(root: &Path, rev: &str, path: &str, expected_sha: &str) -> Result<Vec<u8>> {
    let blob = match git_show(root, rev, path) {
        Ok(blob) => blob,
        Err(e) if rev == "HEAD" => migrated_blob(root, path, expected_sha)
            .map_err(|fallback| format!("{e}; {fallback}"))?,
        Err(e) => return Err(e),
    };
    if digest(&blob) != expected_sha {
        return Err(format!("sealed fixture differs: {rev}:{path}"));
    }
    Ok(blob)
}

fn coordinate(v: &Value) -> Option<u128> {
    match v {
        Value::String(s) => s.parse().ok(),
        Value::Number(n) => n.to_string().parse().ok(),
        Value::Bool(b) => Some(u128::from(*b)),
        _ => None,
    }
}

fn point_rows(blob: &[u8]) -> Vec<Point> {
    String::from_utf8_lossy(blob)
        .lines()
        .filter_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            let pair = value.as_array()?;
            if pair.len() != 2 {
                return None;
            }
            Some((coordinate(&pair[0])?, coordinate(&pair[1])?))
        })
        .collect()
}

fn repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| format!("git rev-parse failed: {e}"))?;
    if !output.status.success() {
        return Err("run from the crypto checkout".into());
    }
    let root = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    Ok(PathBuf::from(root.trim()))
}

fn audit(root: &Path) -> Result<Value> {
    let prior_blob = fs::read(root.join(PRIOR)).map_err(|e| format!("prior audit: {e}"))?;
    if digest(&prior_blob) != PRIOR_SHA {
        return Err("prior exclusion inventory differs".into());
    }
    let prior: Value = serde_json::from_slice(&prior_blob).map_err(|e| e.to_string())?;
    let old = prior["prior_files"]
        .as_array()
        .ok_or("prior_files missing")?;
    let mut inventory: Vec<(String, String, String)> = old
        .iter()
        .map(|row| {
            Ok((
                "HEAD".into(),
                row["path"].as_str().ok_or("prior path missing")?.into(),
                row["sha256"].as_str().ok_or("prior digest missing")?.into(),
            ))
        })
        .collect::<Result<_>>()?;
    inventory.extend(
        EXTRA
            .iter()
            .map(|(r, p, h)| (r.to_string(), p.to_string(), h.to_string())),
    );
    let mut seen = HashSet::new();
    let mut checked = Vec::new();
    for (rev, path, sha) in inventory {
        let points = point_rows(&verified_blob(root, &rev, &path, &sha)?);
        seen.extend(points.iter().copied());
        checked.push(
            json!({"revision": rev, "path": path, "sha256": sha, "point_rows": points.len()}),
        );
    }
    let prior_unique_point_count = seen.len();
    let mut new_seen = HashSet::new();
    let mut fresh = Vec::new();
    for (kind, expected_count) in [("screen", 1536), ("holdout", 16)] {
        let blob = fs::read(root.join(HERE).join(format!("{kind}_targets.jsonl")))
            .map_err(|e| format!("{kind} targets: {e}"))?;
        let points = point_rows(&blob);
        let distinct: HashSet<_> = points.iter().copied().collect();
        if points.len() != expected_count || distinct.len() != expected_count {
            return Err(format!("{kind} point count or distinctness differs"));
        }
        let prior_overlap_count = distinct.intersection(&seen).count();
        let other_fresh_overlap_count = distinct.intersection(&new_seen).count();
        new_seen.extend(distinct);
        fresh.push(
            json!({"point_count": expected_count, "targets_sha256": digest(&blob),
            "prior_overlap_count": prior_overlap_count,
            "other_fresh_overlap_count": other_fresh_overlap_count}),
        );
    }
    let status = if fresh
        .iter()
        .all(|row| row["prior_overlap_count"] == 0 && row["other_fresh_overlap_count"] == 0)
    {
        "PASS_ZERO_OVERLAP"
    } else {
        "OVERLAP_FOUND"
    };
    let report = json!({"schema": "n83.fixed31-exclusion/v2", "prior_audit_sha256": PRIOR_SHA,
        "prior_files": checked, "prior_unique_point_count": prior_unique_point_count,
        "screen": fresh[0], "holdout": fresh[1], "status": status});
    let encoded = format!(
        "{}\n",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    let output = root.join(HERE).join("fresh_exclusion_audit_v2.json");
    if output.exists() {
        if fs::read(&output).map_err(|e| e.to_string())? != encoded.as_bytes() {
            return Err(format!("sealed audit differs: {}", output.display()));
        }
    } else {
        fs::write(&output, encoded).map_err(|e| e.to_string())?;
    }
    Ok(report)
}

fn main() {
    let result = repo_root().and_then(|root| audit(&root));
    match result {
        Ok(report) => println!(
            "{}",
            json!({"status": report["status"],
            "prior_unique_point_count": report["prior_unique_point_count"],
            "screen": report["screen"], "holdout": report["holdout"]})
        ),
        Err(e) => {
            eprintln!("n83-exclusion-audit: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn git(root: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().into()
    }

    #[test]
    fn migrated_blob_requires_sealed_digest_and_byte_count() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("n83-exclusion-{stamp}-{}", std::process::id()));
        fs::create_dir_all(root.join("fixtures")).unwrap();
        git(&root, &["init", "-q"]);
        git(&root, &["config", "user.name", "N83 Test"]);
        git(&root, &["config", "user.email", "n83-test@example.invalid"]);
        let path = "fixtures/target.jsonl";
        let blob = b"[\"1\",\"2\"]\n";
        fs::write(root.join(path), blob).unwrap();
        git(&root, &["add", "."]);
        git(&root, &["commit", "-qm", "fixture"]);
        let parent = git(&root, &["rev-parse", "HEAD"]);
        fs::remove_file(root.join(path)).unwrap();
        let sha = digest(blob);
        let manifest = json!({"schema":"artifacts.manifest/v1", "files": {
            "target.jsonl": {"sha256": sha, "bytes": blob.len()}}});
        fs::write(root.join("fixtures/.artifacts.json"), manifest.to_string()).unwrap();
        git(&root, &["add", "-A"]);
        git(&root, &["commit", "-qm", "migrate"]);
        assert_eq!(
            migrated_blob_from(&root, path, &sha, &parent).unwrap(),
            blob
        );
        assert!(migrated_blob_from(&root, path, &"0".repeat(64), &parent)
            .unwrap_err()
            .contains("manifest differs"));

        let bad_size = json!({"schema":"artifacts.manifest/v1", "files": {
            "target.jsonl": {"sha256": sha, "bytes": blob.len() + 1}}});
        fs::write(root.join("fixtures/.artifacts.json"), bad_size.to_string()).unwrap();
        git(&root, &["add", "."]);
        git(&root, &["commit", "-qm", "alter byte count"]);
        assert!(migrated_blob_from(&root, path, &sha, &parent)
            .unwrap_err()
            .contains("size differs"));
        fs::remove_dir_all(root).unwrap();
    }
}

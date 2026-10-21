//! Native fixture test of the real shell card's sequencing and cap handling.
//! The fixture commands copy receipts; no cryptographic worker is simulated.
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{error::Error, fs, os::unix::fs::PermissionsExt, path::Path, process::Command};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn executable(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}
fn check(wall_cap: bool) -> Result<()> {
    let root = std::env::temp_dir().join(format!(
        "n83-full-card-check-{}-{wall_cap}",
        std::process::id()
    ));
    fs::create_dir(&root)?;
    let result = (|| -> Result<()> {
        fs::create_dir(root.join("fixtures"))?;
        fs::create_dir(root.join("bin"))?;
        fs::write(
            root.join("card.sh"),
            include_bytes!("../research/n83_prebuild_cost_m23_20261019/run-full-m4-card.sh"),
        )?;
        fs::write(
            root.join("source-commit.txt"),
            "0000000000000000000000000000000000000000\n",
        )?;
        fs::write(root.join("sentinel"), b"fixture\n")?;
        let mut prerequisite = json!({"schema":"n83.xor-cache-card-audit/v1","status":"PASS_RECEIPT_AUDIT","natural_attempts":vec![serde_json::Value::Null;16],"comparisons":vec![json!({"status":"PASS_MATCHED_SEARCH_ENDPOINT"});4]});
        // A censored prerequisite must stop before any output/run is made.
        prerequisite["comparisons"][0]["status"] = json!("CENSORED_UNMATCHED_ENDPOINT");
        fs::write(
            root.join("cache-audit.json"),
            serde_json::to_vec(&prerequisite)?,
        )?;
        fs::write(
            root.join("inputs.sha256"),
            format!("{:x}  sentinel\n", Sha256::digest(b"fixture\n")),
        )?;
        // These replace host-inventory commands only inside this child process.
        for name in ["sudo", "lscpu"] {
            executable(&root.join("bin").join(name), "#!/bin/sh\nexit 0\n")?;
        }
        if cfg!(target_os = "macos") {
            executable(
                &root.join("bin/sha256sum"),
                "#!/bin/sh\nexec shasum -a 256 \"$@\"\n",
            )?;
        }
        fs::write(
            root.join("fixtures/replay.json"),
            b"{\"status\":\"PASS\"}\n",
        )?;
        fs::write(
            root.join("fixtures/cap.json"),
            b"{\"status\":\"UNKNOWN_budget\"}\n",
        )?;
        for arm in ["U", "D"] {
            for cache in ["off", "on"] {
                for target in 0..8 {
                    for mode in ["positive", "natural"] {
                        let prefix = format!("{arm}-{cache}-{target}-{mode}");
                        let status = if mode == "positive" {
                            "PASS_CONSTRUCTED_PENDING_REPLAY"
                        } else {
                            "UNKNOWN_search_cap"
                        };
                        fs::write(
                            root.join(format!("fixtures/{prefix}.json")),
                            serde_json::to_vec(
                                &json!({"status":status,"counters":{"completed_conflicts":10000}}),
                            )?,
                        )?;
                        let mut progress = serde_json::to_vec(
                            &json!({"schema":"n83.small-m4-progress/v1","identity":{"arm":arm,"mode":mode,"target_index":target,"experiment_policy":"full-m4-common-public/v1","xor_decided_cache":cache=="on","conflict_budget":10000},"counters":{"completed_conflicts":10000}}),
                        )?;
                        progress.push(b'\n');
                        fs::write(root.join(format!("fixtures/{prefix}.jsonl")), progress)?;
                    }
                }
            }
        }
        executable(
            &root.join("run-full-m4-docker.sh"),
            r##"#!/bin/sh
set -eu
mode=$1 cache=$2 arm=$3 target=$4 output=$5
shift 9
label=$1
printf '%s\n' "$label" >> "$N83_MOCK_ROOT/order.txt"
mkdir "$output"
if [ "$mode" = replay ]; then
    cp "$N83_MOCK_ROOT/fixtures/replay.json" "$output/worker.json"
else
    prefix="$N83_MOCK_ROOT/fixtures/$arm-$cache-$target-$mode"
    cp "$prefix.jsonl" "$output/worker.json.progress.jsonl"
    if [ "$label" = "$N83_MOCK_CAP_LABEL" ]; then
        cp "$N83_MOCK_ROOT/fixtures/cap.json" "$output/worker.json.cap.json"
        exit 124
    fi
    cp "$prefix.json" "$output/worker.json"
fi
"##,
        )?;
        executable(
            &root.join("worker"),
            r##"#!/bin/sh
set -eu
test "$1" = full-m4-control-check
printf '%s\n' "$6" >> "$N83_MOCK_ROOT/gates.txt"
cp "$N83_MOCK_ROOT/fixtures/replay.json" "$6"
"##,
        )?;
        let path = format!("{}:{}", root.join("bin").display(), std::env::var("PATH")?);
        let blocked = Command::new("sh")
            .arg(root.join("card.sh"))
            .arg(&root)
            .env("PATH", &path)
            .output()?;
        assert!(!blocked.status.success());
        assert!(!root.join("runs").exists());
        prerequisite["comparisons"][0]["status"] = json!("PASS_MATCHED_SEARCH_ENDPOINT");
        fs::write(
            root.join("cache-audit.json"),
            serde_json::to_vec(&prerequisite)?,
        )?;
        let output = Command::new("sh")
            .arg(root.join("card.sh"))
            .arg(&root)
            .env("PATH", path)
            .env("N83_MOCK_ROOT", &root)
            .env("N83_MOCK_CAP_LABEL", if wall_cap { "t0-u1" } else { "" })
            .output()?;
        if !output.status.success() {
            return Err(format!("card failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        assert_eq!(
            fs::read_to_string(root.join("card-status.txt"))?.trim(),
            "COMPLETE_BOUNDED"
        );
        let mut expected = Vec::new();
        for arm in ["U", "D"] {
            for cache in ["off", "on"] {
                expected.push(format!("positive-{arm}-{cache}"));
                expected.push(format!("positive-{arm}-{cache}-replay"));
            }
        }
        expected.extend(["t0-u1", "t0-d1", "t0-d2", "t0-u2"].map(str::to_owned));
        for target in 1..8 {
            for arm in if target % 2 == 1 {
                ["D", "U"]
            } else {
                ["U", "D"]
            } {
                expected.push(format!("t{target}-{arm}"));
            }
        }
        let actual = fs::read_to_string(root.join("order.txt"))?;
        assert_eq!(actual.lines().collect::<Vec<_>>(), expected);
        assert_eq!(
            fs::read_to_string(root.join("gates.txt"))?,
            "control-pair-U.json\ncontrol-pair-D.json\n"
        );
        // A completed tree cannot be reused, even by this fixture harness.
        let second = Command::new("sh")
            .arg(root.join("card.sh"))
            .arg(&root)
            .output()?;
        assert!(!second.status.success());
        Ok(())
    })();
    if result.is_ok() {
        fs::remove_dir_all(&root)?;
    }
    result
}
fn main() -> Result<()> {
    check(false)?;
    check(true)?;
    println!("PASS: 26 container calls, two native control checks, eighteen searches, prerequisite rejection, wall-cap retention, no output reuse");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_card_preserves_caller_labels_and_all_cells() {
        check(false).unwrap();
    }
    #[test]
    fn wall_capped_arm_is_retained_and_remaining_cells_run() {
        check(true).unwrap();
    }
}

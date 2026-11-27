//! Main-panel successor: foreground timeout preserves the outer job process group.
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, io::Write, path::Path, process::Command};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const TARGET_SEEDS: [u64; 6] = [303, 404, 505, 606, 707, 808];
fn check(ok: bool, msg: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn check_identity(v: &[&str], rounds: u64, ids: &mut BTreeSet<String>) -> Result<()> {
    check(
        (1..=4).contains(&rounds) && v.len() == 5,
        "schedule width/rounds",
    )?;
    let case = v[2].parse::<usize>()?;
    let seed = v[3].parse::<u64>()?;
    let round = v[4].parse::<u64>()?;
    check(
        case < 6
            && TARGET_SEEDS.contains(&seed)
            && ["a", "b", "c"].contains(&v[1])
            && round < rounds,
        "schedule bounds",
    )?;
    check(
        v[0] == format!("{}-{case}-{seed}-{round}", v[1]) && ids.insert(v[0].to_owned()),
        "unique bound schedule id",
    )
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    check(a.len() == 3, "OUT FROZEN_OPENSSL_CHECKER")?;
    let out = Path::new(&a[1]);
    let cfg: Value = serde_json::from_slice(&fs::read(out.join("config.json"))?)?;
    let rounds = cfg["rounds"].as_u64().ok_or("rounds")?;
    check((1..=4).contains(&rounds), "round bound")?;
    let records = fs::read_to_string(out.join("schedule.tsv"))?;
    let rows = records.lines().collect::<Vec<_>>();
    check(
        rows.len() == 6 * TARGET_SEEDS.len() * 3 * rounds as usize,
        "full schedule count",
    )?;
    let audit = out.join("point-audit");
    fs::create_dir(&audit)?;
    let mut progress = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(audit.join("progress.tsv"))?;
    let mut ids = BTreeSet::new();
    let mut complete = 0;
    for row in rows {
        let v = row.split('\t').collect::<Vec<_>>();
        check_identity(&v, rounds, &mut ids)?;
        let case = v[2].parse::<usize>()?;
        let seed = v[3].parse::<u64>()?;
        let cert: Value = serde_json::from_slice(&fs::read(
            out.join(v[0]).join(format!("scale-u-{case}-{seed}.json")),
        )?)?;
        let curve = cert["curve"].as_array().ok_or("curve")?;
        let g = cert["generator"].as_array().ok_or("generator")?;
        let q = cert["target"].as_array().ok_or("target")?;
        check(
            curve.len() == 4 && g.len() == 2 && q.len() == 2,
            "affine input shape",
        )?;
        let mut args = vec![case.to_string(), seed.to_string()];
        for v in curve.iter().chain(g).chain(q) {
            args.push(
                v.as_u64()
                    .ok_or("canonical unsigned coordinate")?
                    .to_string(),
            );
        }
        let r = curve[3].as_u64().ok_or("order")?;
        let p = curve[0].as_u64().ok_or("field")?;
        let y = q[1].as_u64().ok_or("target y")?;
        let k = cert["scalar"].as_u64().ok_or("scalar")?;
        check(k > 0 && k < r && y > 0 && y < p, "nontrivial control range")?;
        args.push(k.to_string());
        for mode in ["positive", "wrong-scalar", "substituted-target"] {
            let mut argv = args.clone();
            if mode == "wrong-scalar" {
                argv[10] = ((k + 1) % r).to_string();
            } else if mode == "substituted-target" {
                argv[9] = (p - y).to_string();
                argv[10] = (r - k).to_string();
            }
            writeln!(progress, "BEGIN\t{}\t{mode}", v[0])?;
            progress.flush()?;
            let result = Command::new("timeout")
                .args([
                    "--foreground",
                    "--signal=TERM",
                    "--kill-after=5",
                    "60",
                    &a[2],
                ])
                .args(argv)
                .output()?;
            let prefix = format!("{}.{mode}", v[0]);
            fs::write(audit.join(format!("{prefix}.stdout")), &result.stdout)?;
            fs::write(audit.join(format!("{prefix}.stderr")), &result.stderr)?;
            let valid = if mode == "positive" {
                result.status.success()
                    && String::from_utf8_lossy(&result.stdout).trim()
                        == "OPENSSL VERIFIED public-target binding, nonidentity rP=O and kP=Q"
            } else {
                result.status.code() == Some(1)
                    && String::from_utf8_lossy(&result.stderr).trim()
                        == if mode == "wrong-scalar" {
                            "REJECTED kP != Q"
                        } else {
                            "REJECTED public target mismatch"
                        }
            };
            writeln!(
                progress,
                "END\t{}\t{mode}\t{}\t{valid}",
                v[0],
                result.status.code().unwrap_or(-1)
            )?;
            progress.flush()?;
            check(
                valid,
                "OpenSSL positive/control failed; preserve all output",
            )?;
        }
        complete += 1;
    }
    let summary = json!({"schema":"native-public-point-audit/v1","cells":complete,"positive_checks_passed":complete,"wrong_scalar_rejected":complete,"consistent_substitution_rejected":complete,"scope":"C/OpenSSL target derivation and raw add/double scalar replay, reusing frozen C trial-prime/Hasse admission; unsigned producer evidence, no relation/performance/execution attestation"});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(audit.join("summary.json"))?;
    f.write_all(&serde_json::to_vec_pretty(&summary)?)?;
    println!(
        "PASS {complete} OpenSSL positives and {} required negative controls",
        2 * complete
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_bound_cohort_identity_and_rejects_duplicate() {
        let mut ids = BTreeSet::new();
        let row = ["b-0-303-1", "b", "0", "303", "1"];
        assert!(check_identity(&row, 4, &mut ids).is_ok());
        assert!(check_identity(&row, 4, &mut ids).is_err());
    }
    #[test]
    fn rejects_old_seed_substitution_and_out_of_range_cells() {
        for row in [
            ["b-0-101-1", "b", "0", "101", "1"],
            ["b-0-303-1", "b", "0", "404", "1"],
            ["b-6-303-1", "b", "6", "303", "1"],
            ["b-0-303-4", "b", "0", "303", "4"],
            ["x-0-303-1", "x", "0", "303", "1"],
        ] {
            assert!(check_identity(&row, 4, &mut BTreeSet::new()).is_err());
        }
        assert!(check_identity(&["short"], 4, &mut BTreeSet::new()).is_err());
    }
}

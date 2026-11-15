//! Bounded Singular comparison with exact output replay against native results.
use relation_degree_trace::{workload::Fixture, Poly, Term};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};
fn parse_singular(text: &str, p: u64, n: usize) -> Result<(u64, BTreeSet<Poly>), Box<dyn Error>> {
    let mut basis = BTreeSet::new();
    let mut current = Poly::new();
    let mut time = None;
    let mut done = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("CPU_MICROSECONDS:") {
            time = Some(value.parse()?);
        } else if line == "POLY" {
            current.clear();
        } else if let Some(term) = line.strip_prefix("TERM:") {
            let (c, m) = term.split_once(':').ok_or("bad TERM")?;
            let c: i128 = c.parse()?;
            let m: Vec<usize> = m
                .split(',')
                .map(|e| e.trim().parse())
                .collect::<Result<_, _>>()?;
            if m.len() != n {
                return Err("bad term dimensions".into());
            }
            let c = c.rem_euclid(p as i128) as u64;
            if c != 0 {
                current.insert(m, c);
            }
        } else if line == "ENDPOLY" {
            if !current.is_empty() {
                basis.insert(current.clone());
            }
        } else if line == "DONE" {
            done = true;
        }
    }
    if !done || basis.is_empty() {
        return Err("missing completion/basis".into());
    }
    Ok((time.ok_or("missing CPU timer")?, basis))
}
fn expected(path: &Path) -> Result<Option<BTreeSet<Poly>>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(None);
    }
    let summary: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let Some(array) = summary["samples"].as_array() else {
        return Ok(None);
    };
    for sample in array {
        let b = &sample["arms"]["baseline"];
        if b["status"] == "complete" {
            let terms: Vec<Vec<Term>> = serde_json::from_value(b["basis"].clone())?;
            return Ok(Some(
                terms
                    .into_iter()
                    .map(|terms| {
                        terms
                            .into_iter()
                            .map(|t| (t.exponents, t.coefficient as u64))
                            .collect()
                    })
                    .collect(),
            ));
        }
    }
    Ok(None)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let (mut fixtures, mut native, mut out, mut singular) = (None, None, None, None);
    let mut cpu = "0".to_string();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixtures" => fixtures = Some(PathBuf::from(args.next().ok_or("missing fixtures")?)),
            "--native-results" => {
                native = Some(PathBuf::from(args.next().ok_or("missing native results")?))
            }
            "--out" => out = Some(PathBuf::from(args.next().ok_or("missing output")?)),
            "--singular" => singular = Some(PathBuf::from(args.next().ok_or("missing Singular")?)),
            "--cpu" => cpu = args.next().ok_or("missing CPU")?,
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    let fixtures = fixtures.ok_or("fixtures required")?;
    let native = native.ok_or("native results required")?;
    let out = out.ok_or("out required")?;
    let singular = singular.ok_or("Singular required")?;
    if out.exists() {
        return Err("output exists".into());
    }
    fs::create_dir_all(&out)?;
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(fixtures.join("manifest.json"))?)?;
    let mut rows = Vec::new();
    for item in manifest["cases"].as_array().ok_or("bad manifest")? {
        let id = item["id"].as_str().ok_or("bad ID")?;
        let root = fixtures.join(id);
        let fixture: Fixture =
            serde_json::from_str(&fs::read_to_string(root.join("fixture.json"))?)?;
        let expected = expected(&native.join(id).join("summary.json"))?;
        let mut first = None;
        for arm in ["baseline", "candidate"] {
            for algorithm in ["std", "slimgb"] {
                // slimgb is documented for fields; do not pretend Z/pZ ring support.
                if fixture.system.prime >= 2u64.pow(31) && algorithm == "slimgb" {
                    rows.push(json!({"id":id,"arm":arm,"algorithm":algorithm,"status":"unsupported_coefficient_ring"}));
                    continue;
                }
                let script = root.join(format!("{arm}-{algorithm}.sing"));
                let start = Instant::now();
                let result = Command::new("timeout")
                    .args([
                        "30s",
                        "prlimit",
                        "--as=2147483648",
                        "--",
                        "taskset",
                        "-c",
                        &cpu,
                    ])
                    .arg(&singular)
                    .arg("-q")
                    .arg(&script)
                    .env("OMP_NUM_THREADS", "1")
                    .output()?;
                let wall_ns = start.elapsed().as_nanos();
                let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
                fs::write(out.join(format!("{id}-{arm}-{algorithm}.stdout")), &stdout)?;
                fs::write(out.join(format!("{id}-{arm}-{algorithm}.stderr")), &stderr)?;
                let parsed = parse_singular(
                    &stdout,
                    fixture.system.prime,
                    fixture.system.variables.len(),
                );
                let row = if let Ok((micros, basis)) = parsed {
                    let verified = expected.as_ref().is_some_and(|v| *v == basis);
                    let pair_equal = first.as_ref().map(|v| v == &basis);
                    if first.is_none() {
                        first = Some(basis.clone());
                    }
                    json!({"id":id,"arm":arm,"algorithm":algorithm,"status":if result.status.success()&&verified{"checked_against_native_basis"}else if result.status.success(){"output_unverified"}else{"process_failed"},
                "native_basis_equal":verified,"other_external_basis_equal":pair_equal,"solver_cpu_microseconds_100_solves":micros,"process_wall_ns":wall_ns,"basis_polynomials":basis.len(),"preprocessing_and_external_membership_cost":"not charged by this solver-only diagnostic; not an end-to-end speed ratio"})
                } else {
                    json!({"id":id,"arm":arm,"algorithm":algorithm,"status":"failure_or_unparseable_output","exit_code":result.status.code(),"process_wall_ns":wall_ns})
                };
                println!("{id} {arm} {algorithm}: {}", row["status"]);
                rows.push(row);
            }
        }
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(
            &json!({"schema":"external-solver-diagnostic/v1","rows":rows,"scope":"single 100-solve CPU batches; no native wall/CPU mixed ratios; no fastest claim"}),
        )? + "\n",
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_completion_marker_is_not_a_result() {
        assert!(parse_singular("CPU_MICROSECONDS:10\nPOLY\nTERM:1:1,0\nENDPOLY", 101, 2).is_err());
    }
    #[test]
    fn signed_coefficient_replay() {
        let (_, b) = parse_singular(
            "CPU_MICROSECONDS:10\nPOLY\nTERM:-1:1,0\nENDPOLY\nDONE",
            101,
            2,
        )
        .unwrap();
        assert_eq!(b.first().unwrap()[&vec![1, 0]], 100);
    }
}

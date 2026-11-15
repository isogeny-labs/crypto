//! Native solver controller; producer observations, never independent standing.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    env, fs,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
mod export;
mod protocol;
fn check(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn sha(path: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}
fn write_new(path: &Path, value: &Value) -> Result<()> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(serde_json::to_string_pretty(value)?.as_bytes())?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}
fn member(root: &Path, name: &str) -> Result<PathBuf> {
    let rel = Path::new(name);
    check(
        !rel.as_os_str().is_empty() && rel.components().all(|c| matches!(c, Component::Normal(_))),
        "unsafe manifest path",
    )?;
    let root = root.canonicalize()?;
    let path = root.join(rel).canonicalize()?;
    check(path.starts_with(root), "manifest symlink escapes input")?;
    Ok(path)
}
#[derive(Debug)]
struct Query {
    fields: Vec<String>,
}
fn queries(root: &Path) -> Result<Vec<Query>> {
    let text = fs::read_to_string(root.join("queries.tsv"))?;
    let mut lines = text.lines();
    check(
        lines.next() == Some("file\tarm\tencoding\tp\ta\tb\tell\tbase\tx\ty\tinf\texpected"),
        "query manifest schema",
    )?;
    let mut seen = BTreeSet::new();
    let mut result = vec![];
    for line in lines {
        let f: Vec<_> = line.split('\t').map(str::to_owned).collect();
        check(f.len() == 12, "query manifest arity")?;
        check(seen.insert(f[0].clone()), "duplicate query")?;
        member(root, &f[0])?;
        member(root, &f[7])?;
        check(
            matches!(f[2].as_str(), "mod" | "carry") && matches!(f[11].as_str(), "sat" | "unsat"),
            "manifest encoding/status",
        )?;
        for i in [3, 4, 5, 6, 8, 9, 10] {
            f[i].parse::<u64>()?;
        }
        check(
            matches!(f[10].as_str(), "0" | "1"),
            "manifest infinity flag",
        )?;
        result.push(Query { fields: f });
    }
    check(!result.is_empty(), "empty query manifest")?;
    Ok(result)
}
fn artifact(path: &Path, base: &Path) -> Result<Value> {
    Ok(
        json!({"path":path.strip_prefix(base)?.to_str().ok_or("non-UTF8 artifact")?,"sha256":sha(path)?,"bytes":fs::metadata(path)?.len()}),
    )
}
fn rlimit(text: &str) -> Result<u64> {
    let tokens: Vec<_> = text.split_whitespace().collect();
    let values: Vec<_> = tokens
        .windows(2)
        .filter(|w| w[0].trim_start_matches('(') == ":rlimit-count")
        .collect();
    check(
        values.len() == 1,
        "missing or duplicate solver resource counter",
    )?;
    Ok(values[0][1].trim_end_matches(')').parse()?)
}
fn execute(
    input: &Path,
    out: &Path,
    study: &Path,
    solver: &Path,
    timeout: u32,
    external: &Path,
) -> Result<bool> {
    check(timeout > 0 && timeout <= 300, "solver deadline domain")?;
    let input = input.canonicalize()?;
    let study = study.canonicalize()?;
    let solver = solver.canonicalize()?;
    let external = external.canonicalize()?;
    let qs = queries(&input)?;
    check(
        !out.exists(),
        "result directory exists; never overwrite or retry in place",
    )?;
    let configuration: Value =
        serde_json::from_slice(&fs::read(input.join("configuration.json"))?)?;
    check(
        configuration["schema"] == "atlas.input/v1",
        "input configuration schema",
    )?;
    fs::create_dir(out)?;
    let out = out.canonicalize()?;
    let source = json!({"study_binary_sha256":sha(&study)?,"external_replay_binary_sha256":sha(&external)?,"solver_binary_sha256":sha(&solver)?,"query_manifest_sha256":sha(&input.join("queries.tsv"))?,"timeout_seconds":timeout,"configuration":configuration});
    write_new(&out.join("identity.json"), &source)?;
    let mut rows = vec![];
    let mut all_valid = true;
    for (index, q) in qs.iter().enumerate() {
        let f = &q.fields;
        let query = member(&input, &f[0])?;
        let base = member(&input, &f[7])?;
        let model = out.join(format!("{index:04}.solver.out"));
        let errors = out.join(format!("{index:04}.solver.err"));
        let argv = vec![
            format!("-T:{timeout}"),
            "-st".into(),
            query.to_string_lossy().into_owned(),
        ];
        let status = Command::new(&solver)
            .args(&argv)
            .stdin(Stdio::null())
            .stdout(fs::File::create(&model)?)
            .stderr(fs::File::create(&errors)?)
            .status()?;
        let text = fs::read_to_string(&model)?;
        let observed = text.lines().next().unwrap_or("empty").to_owned();
        let mut valid = observed == f[11];
        let mut replay_exit = None;
        let mut replay_artifacts = vec![];
        if observed == "sat" {
            let replay = Command::new(&study)
                .arg("--replay")
                .args(&f[3..7])
                .arg(&base)
                .args(&f[8..11])
                .arg(&model)
                .output()?;
            replay_exit = replay.status.code();
            valid &= replay.status.success() && status.success();
            let rp = out.join(format!("{index:04}.replay.out"));
            let re = out.join(format!("{index:04}.replay.err"));
            fs::write(&rp, replay.stdout)?;
            fs::write(&re, replay.stderr)?;
            replay_artifacts = vec![artifact(&rp, &out)?, artifact(&re, &out)?];
        } else if observed == "unsat" {
            // Z3 exits 1 solely because the input requests get-model after UNSAT.
            // Accept that exact diagnostic only; other errors invalidate the row.
            let remaining: Vec<_> = text.lines().filter(|l| l.starts_with("(error ")).collect();
            let expected_error = remaining.len() == 1
                && remaining[0].starts_with("(error \"line ")
                && remaining[0].ends_with("model is not available\")");
            valid &= status.success() || (status.code() == Some(1) && expected_error);
            valid &= fs::metadata(&errors)?.len() == 0;
        } else {
            valid = false;
        }
        let mut external_exit = None;
        if observed == "sat" || observed == "unsat" {
            let replay = Command::new(&external)
                .args(&f[3..7])
                .arg(&base)
                .args(&f[8..11])
                .arg(&model)
                .output()?;
            external_exit = replay.status.code();
            valid &= replay.status.success();
            let rp = out.join(format!("{index:04}.openssl.out"));
            let re = out.join(format!("{index:04}.openssl.err"));
            fs::write(&rp, replay.stdout)?;
            fs::write(&re, replay.stderr)?;
            replay_artifacts.push(artifact(&rp, &out)?);
            replay_artifacts.push(artifact(&re, &out)?);
        }
        let resource = rlimit(&text).ok();
        valid &= resource.is_some();
        all_valid &= valid;
        let row = json!({"schema":"atlas.solver-row/v1","index":index,"arm":f[1],"encoding":f[2],"query":f[0],"query_sha256":sha(&query)?,"base_sha256":sha(&base)?,"expected":f[11],"observed":observed,"solver_exit":status.code(),"replay_exit":replay_exit,"external_replay_exit":external_exit,"solver_rlimit":resource,"producer_check_passed":valid,"independent_standing":"unverified","argv":argv,"artifacts":[artifact(&model,&out)?,artifact(&errors,&out)?],"replay_artifacts":replay_artifacts});
        write_new(&out.join(format!("{index:04}.json")), &row)?;
        rows.push(row);
    }
    let summary = json!({"schema":"atlas.solver-run/v1","identity":source,"flow":{"planned":qs.len(),"attempted":rows.len(),"completed":rows.iter().filter(|r|r["producer_check_passed"]==true).count(),"invalid_or_incomplete":rows.iter().filter(|r|r["producer_check_passed"]!=true).count()},"producer_check_passed":all_valid,"independent_standing":"unverified","rows":rows});
    write_new(&out.join("summary.json"), &summary)?;
    Ok(all_valid)
}
fn main() {
    let a: Vec<String> = env::args().collect();
    let result = (|| -> Result<bool> {
        if a.len() == 7 && a[1] == "protocol" {
            check(matches!(a[6].as_str(), "unit" | "panel"), "protocol scope")?;
            write_new(
                Path::new(&a[2]),
                &protocol::draft(a[3].parse()?, a[4].parse()?, a[5].parse()?, a[6] == "unit"),
            )?;
            return Ok(true);
        }
        if a.len() == 6 && a[1] == "export" {
            export::write(
                Path::new(&a[2]),
                Path::new(&a[3]),
                Path::new(&a[4]),
                Path::new(&a[5]),
            )?;
            return Ok(true);
        }
        check(
            a.len() == 8 && a[1] == "run",
            "usage: atlas-control run INPUT FRESH_OUTPUT STUDY Z3 TIMEOUT_SECONDS OPENSSL_REPLAY",
        )?;
        execute(
            Path::new(&a[2]),
            Path::new(&a[3]),
            Path::new(&a[4]),
            Path::new(&a[5]),
            a[6].parse()?,
            Path::new(&a[7]),
        )
    })();
    match result {
        Ok(true) => (),
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsafe_manifest_paths_are_rejected() {
        let root = env::temp_dir();
        assert!(member(&root, "../escape").is_err());
        assert!(member(&root, "/absolute").is_err());
    }
    #[test]
    fn results_are_write_once() {
        let path = env::temp_dir().join(format!("atlas-control-test-{}.json", std::process::id()));
        let _ = fs::remove_file(&path);
        write_new(&path, &json!({"a":1})).unwrap();
        assert!(write_new(&path, &json!({"a":2})).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "{\n  \"a\": 1\n}\n");
        fs::remove_file(path).unwrap();
    }
    #[test]
    fn resource_counter_is_unambiguous() {
        assert_eq!(rlimit("(:rlimit-count 123)\n").unwrap(), 123);
        assert!(rlimit(":rlimit-count 1 :rlimit-count 2").is_err());
        assert!(rlimit("sat\n").is_err());
    }
}

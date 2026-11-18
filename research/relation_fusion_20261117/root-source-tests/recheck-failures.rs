//! Diagnose every failure observed in a retained partial full-suite run.
//! No timing statistics; never treats a filtered pass as full-suite qualification.
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type R<T> = Result<T, Box<dyn Error>>;
fn save(p: &Path, b: &[u8]) -> R<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn main() -> R<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage:recheck-failures FROZEN_UNIT_ELF PARTIAL_STDOUT NEW_OUT".into());
    }
    let names: Vec<String> = fs::read_to_string(&a[2])?
        .lines()
        .filter_map(|l| {
            l.strip_prefix("test ")?
                .strip_suffix(" ... FAILED")
                .map(str::to_owned)
        })
        .collect();
    if names.is_empty()
        || names.iter().any(|n| {
            !n.bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b':')
        })
    {
        return Err("failure roster empty or invalid".into());
    }
    let out = Path::new(&a[3]);
    fs::create_dir(out)?;
    let start = Instant::now();
    let mut summary = String::from("test\texit\n");
    for name in &names {
        let dir = out.join(name.replace("::", "__"));
        fs::create_dir(&dir)?;
        if start.elapsed() > Duration::from_secs(600) {
            save(
                &dir.join("not-run.txt"),
                b"Original global600s deadline; no retry.\n",
            )?;
            continue;
        }
        let status = Command::new("taskset")
            .args([
                "-c",
                "0",
                "timeout",
                "--kill-after=1s",
                "30s",
                "prlimit",
                "--as=2147483648",
                "--fsize=16777216",
                "--",
            ])
            .arg(&a[1])
            .args(["--exact", name, "--nocapture", "--test-threads=1"])
            .env("RUST_TEST_THREADS", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::from(
                fs::File::options()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("stdout"))?,
            ))
            .stderr(Stdio::from(
                fs::File::options()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("stderr"))?,
            ))
            .status()?;
        save(
            &dir.join("exit.txt"),
            format!("{:?}\n", status.code()).as_bytes(),
        )?;
        summary += &format!("{name}\t{:?}\n", status.code());
    }
    save(&out.join("summary.tsv"), summary.as_bytes())?;
    Ok(())
}

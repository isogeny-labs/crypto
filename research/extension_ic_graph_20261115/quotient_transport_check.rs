//! Local transport-only create-new control; no cryptographic computation.
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    process::{self, Command},
    time::{SystemTime, UNIX_EPOCH},
};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 4 && args[3].len() == 40 {
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[2])?;
        writeln!(out, "{{\"transport_control_only\":true}}")?;
        println!("transport-control stdout");
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: quotient-transport-check WRAPPER".into());
    }
    let run = env::temp_dir().join(format!(
        "quotient-transport-{}-{}",
        process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    fs::create_dir(&run)?;
    fs::copy(env::current_exe()?, run.join("driver"))?;
    fs::create_dir(run.join("originals"))?;
    for mode in [
        "s75-r32", "s75-r128", "s85-r32", "s85-r128", "s95-r32", "s95-r128",
    ] {
        fs::write(
            run.join("originals").join(format!("{mode}.raw.json")),
            "{}\n",
        )?;
    }
    let status = fs::read_to_string("/proc/self/status")?;
    let cpu = status
        .lines()
        .find_map(|s| s.strip_prefix("Cpus_allowed_list:"))
        .ok_or("affinity")?
        .trim()
        .split([',', '-'])
        .next()
        .ok_or("CPU")?;
    fs::write(run.join("runtime-plan.json"),format!("{{\"memory_gib\":8,\"source_revision\":\"{}\",\"pinned_cpu\":\"{cpu}\",\"worker_timeout_seconds\":10}}","a".repeat(40)))?;
    let status = Command::new("bash")
        .arg(fs::canonicalize(&args[1])?)
        .arg("--execute")
        .arg(&run)
        .status()?;
    if !status.success() {
        return Err(format!(
            "capture failed; control files retained at {}",
            run.display()
        )
        .into());
    }
    if fs::read_to_string(run.join("raw.json"))? != "{\"transport_control_only\":true}\n"
        || fs::read_to_string(run.join("replay.summary.jsonl"))? != "transport-control stdout\n"
        || fs::read_to_string(run.join("replay.exit-code.txt"))?.trim() != "0"
        || fs::read_to_string(run.join("flow.jsonl"))?.lines().count() != 2
    {
        return Err("capture contents/flow/status".into());
    }
    fs::remove_dir_all(run)?;
    println!("recorded-matrix transport-only raw/stdout create-new control passed");
    Ok(())
}

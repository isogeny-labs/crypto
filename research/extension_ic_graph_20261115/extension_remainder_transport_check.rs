//! Local transport controls only: create-new raw ownership and all-arm failure flow.
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
    if let Some(i) = args.iter().position(|s| s == "--out") {
        let mut out = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&args[i + 1])?;
        writeln!(out, "{{\"transport_control_only\":true}}")?;
        println!("transport stdout");
        let mode = args.iter().position(|s| s == "--mode").ok_or("mode")?;
        if env::var_os("IC_TRANSPORT_FAIL_FIRST").is_some() && args[mode + 1] == "full-smooth" {
            return Err("deliberate first-arm failure".into());
        }
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: TRANSPORT_CHECK WRAPPER".into());
    }
    for negative in [false, true] {
        let run = env::temp_dir().join(format!(
            "extension-remainder-transport-{}-{}",
            process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        ));
        fs::create_dir(&run)?;
        fs::copy(env::current_exe()?, run.join("driver"))?;
        fs::write(run.join("public.json"), "{}\n")?;
        let status = fs::read_to_string("/proc/self/status")?;
        let cpu = status
            .lines()
            .find_map(|s| s.strip_prefix("Cpus_allowed_list:"))
            .ok_or("CPU affinity")?
            .trim()
            .split([',', '-'])
            .next()
            .ok_or("CPU")?;
        fs::write(run.join("runtime-plan.json"),format!("{{\"memory_gib\":8,\"pinned_cpu\":\"{cpu}\",\"source_revision\":\"{}\",\"worker_timeout_seconds\":5,\"mode_order\":[\"full-smooth\",\"quadratic-remainders\"]}}","a".repeat(40)))?;
        let mut command = Command::new("bash");
        command
            .arg(fs::canonicalize(&args[1])?)
            .arg("--execute")
            .arg(&run);
        if negative {
            command.env("IC_TRANSPORT_FAIL_FIRST", "1");
        } else {
            command.env_remove("IC_TRANSPORT_FAIL_FIRST");
        }
        if !command.status()?.success() {
            return Err(format!("wrapper failed: {}", run.display()).into());
        }
        for mode in ["full-smooth", "quadratic-remainders"] {
            if fs::read_to_string(run.join(format!("{mode}.raw.json")))?
                != "{\"transport_control_only\":true}\n"
                || fs::read_to_string(run.join(format!("{mode}.stdout.txt")))?
                    != "transport stdout\n"
            {
                return Err("raw/stdout ownership failed".into());
            }
            let code = fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?;
            if (code.trim() == "0") != (mode != "full-smooth" || !negative) {
                return Err("worker status/failure continuation".into());
            }
        }
        if fs::read_to_string(run.join("flow.jsonl"))?.lines().count() != 4 {
            return Err("all-arm flow failed".into());
        }
        // Deliberately repeat: the stub must refuse an existing scientific raw path.
        if Command::new(run.join("driver"))
            .arg("--out")
            .arg(run.join("full-smooth.raw.json"))
            .status()?
            .success()
        {
            return Err("create-new collision accepted".into());
        }
        println!(
            "{} transport-only control passed; originals retained at {}",
            if negative {
                "first-arm-failure"
            } else {
                "positive"
            },
            run.display()
        );
    }
    Ok(())
}

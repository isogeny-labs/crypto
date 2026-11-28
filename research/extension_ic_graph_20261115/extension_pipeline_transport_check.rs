//! Native transport-only fixture/seven-arm control; no cryptographic computation.
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{self, Command},
    time::{SystemTime, UNIX_EPOCH},
};
const MODES: [&str; 7] = [
    "ic",
    "extension-full-smooth",
    "extension-remainders",
    "rho-plain-1",
    "rho-negation-1",
    "rho-plain-16",
    "rho-negation-16",
];
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if let Some(i) = args.iter().position(|s| s == "--out") {
        let mut out = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&args[i + 1])?;
        if args.iter().any(|s| s == "--generate") {
            writeln!(out, "{{\"public\":{{\"transport_control_only\":true}}}}")?;
        } else {
            writeln!(out, "{{\"transport_control_only\":true}}")?;
        }
        println!("transport stdout");
        if env::var_os("IC_TRANSPORT_FAIL_FIRST").is_some()
            && Path::new(&args[i + 1])
                .file_name()
                .is_some_and(|s| s == "ic.raw.json")
        {
            return Err("deliberate first-worker failure".into());
        }
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: TRANSPORT WRAPPER".into());
    }
    for negative in [false, true] {
        let run = env::temp_dir().join(format!(
            "extension-pipeline-transport-{}-{}",
            process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        ));
        fs::create_dir(&run)?;
        fs::copy(env::current_exe()?, run.join("driver"))?;
        let status = fs::read_to_string("/proc/self/status")?;
        let cpu = status
            .lines()
            .find_map(|s| s.strip_prefix("Cpus_allowed_list:"))
            .ok_or("CPU affinity")?
            .trim()
            .split([',', '-'])
            .next()
            .ok_or("CPU")?;
        let modes = MODES
            .iter()
            .map(|m| format!("\"{m}\""))
            .collect::<Vec<_>>()
            .join(",");
        fs::write(run.join("runtime-plan.json"),format!("{{\"memory_gib\":8,\"pinned_cpu\":\"{cpu}\",\"source_revision\":\"{}\",\"fixture_timeout_seconds\":5,\"worker_timeout_seconds\":5,\"p\":1009,\"seed\":389,\"b_count\":1018081,\"max_descent_residuals\":8192,\"max_accepted_descents\":32,\"mode_order\":[{modes}]}}","a".repeat(40)))?;
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
            return Err(format!("wrapper failed; originals at {}", run.display()).into());
        }
        for mode in MODES {
            if fs::read_to_string(run.join(format!("{mode}.raw.json")))?
                != "{\"transport_control_only\":true}\n"
                || fs::read_to_string(run.join(format!("{mode}.summary.jsonl")))?
                    != "transport stdout\n"
            {
                return Err("raw/stdout ownership".into());
            }
            let code = fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?;
            if (code.trim() == "0") != (mode != "ic" || !negative) {
                return Err("all-arm failure continuation".into());
            }
        }
        if fs::read_to_string(run.join("fixture.exit-code.txt"))?.trim() != "0"
            || fs::read_to_string(run.join("flow.jsonl"))?.lines().count() != 16
        {
            return Err("fixture/seven-arm flow".into());
        }
        if Command::new(run.join("driver"))
            .arg("--out")
            .arg(run.join("ic.raw.json"))
            .status()?
            .success()
        {
            return Err("create-new collision accepted".into());
        }
        println!(
            "{} fixture/seven-arm transport-only control passed; originals at {}",
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

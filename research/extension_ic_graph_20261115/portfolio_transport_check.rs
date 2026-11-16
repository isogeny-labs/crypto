//! Transport-only native control: no cryptographic computation or scientific timing.
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{self, Command},
    time::{SystemTime, UNIX_EPOCH},
};
const MODES: [&str; 6] = [
    "ic",
    "rank-stop",
    "rho-plain-1",
    "rho-negation-1",
    "rho-plain-16",
    "rho-negation-16",
];
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if let Some(i) = args.iter().position(|v| v == "--out") {
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(args.get(i + 1).ok_or("out path")?)?;
        if args.iter().any(|v| v == "--generate") {
            writeln!(out, "{{\"public\":{{\"transport_control_only\":true}}}}")?;
        } else {
            writeln!(out, "{{\"transport_control_only\":true}}")?;
        }
        println!("transport-control stdout");
        return Ok(());
    }
    if !(2..=3).contains(&args.len()) {
        return Err(
            "usage: portfolio-transport-check WRAPPER [--expect-failure|--structural]".into(),
        );
    }
    let negative = args.get(2).is_some_and(|s| s == "--expect-failure");
    let structural = args.get(2).is_some_and(|s| s == "--structural");
    if args.len() == 3 && !negative && !structural {
        return Err("unknown control option".into());
    }
    let wrapper = fs::canonicalize(Path::new(&args[1]))?;
    let run = env::temp_dir().join(format!(
        "ic-transport-control-{}-{}",
        process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    ));
    fs::create_dir(&run)?;
    fs::copy(env::current_exe()?, run.join("driver"))?;
    let status = fs::read_to_string("/proc/self/status")?;
    let allowed = status
        .lines()
        .find_map(|s| s.strip_prefix("Cpus_allowed_list:"))
        .ok_or("CPU affinity")?
        .trim();
    let cpu = allowed.split([',', '-']).next().ok_or("CPU")?;
    let modes: &[&str] = if structural {
        &["rank-stop", "structural-gate"]
    } else {
        &MODES
    };
    let modes_json = modes
        .iter()
        .map(|m| format!("\"{m}\""))
        .collect::<Vec<_>>()
        .join(",");
    let plan=format!("{{\"pinned_cpu\":\"{cpu}\",\"source_revision\":\"{}\",\"memory_gib\":8,\"p\":1009,\"seed\":347,\"fixture_timeout_seconds\":10,\"worker_timeout_seconds\":10,\"small_percent\":75,\"b_count\":1018081,\"collection_rank_stride\":128,\"max_descent_residuals\":8192,\"max_accepted_descents\":32,\"mode_order\":[{modes_json}]}}","a".repeat(40));
    fs::write(run.join("runtime-plan.json"), plan)?;
    let status = Command::new("bash")
        .arg(wrapper)
        .arg("--execute")
        .arg(&run)
        .status()?;
    if status.success() == negative {
        return Err(format!(
            "unexpected wrapper status; controls retained at {}",
            run.display()
        )
        .into());
    }
    for mode in modes {
        let raw = fs::read_to_string(run.join(format!("{mode}.raw.json")))?;
        let exit = fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?;
        if negative {
            if !raw.is_empty()
                || exit.trim() != "1"
                || !fs::read_to_string(run.join(format!("{mode}.stderr")))?
                    .contains("AlreadyExists")
            {
                return Err("negative control failed for a different reason".into());
            }
        } else if raw != "{\"transport_control_only\":true}\n"
            || exit.trim() != "0"
            || fs::read_to_string(run.join(format!("{mode}.summary.jsonl")))?
                != "transport-control stdout\n"
        {
            return Err("raw/stdout separation or contents failed".into());
        }
    }
    fs::remove_dir_all(run)?;
    println!(
        "transport-only {} control passed",
        if negative {
            "old-wrapper rejection"
        } else if structural {
            "new-wrapper two-arm capture"
        } else {
            "new-wrapper six-arm capture"
        }
    );
    Ok(())
}

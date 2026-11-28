use ic_hky_replay_records::*;
use serde_json::json;
use std::{
    fs::{self, File},
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

fn pause_requested(context: &Context) -> Result<bool> {
    Ok(Path::new(
        context.native()["pause_request_path"]
            .as_str()
            .ok_or("missing native pause selector")?,
    )
    .exists())
}

fn child(context: &Context, case: &Case, arm: &str) -> Result<bool> {
    context.guard()?;
    let folder = context.run.join(arm).join(&case.id);
    if folder.join("ended.txt").exists() {
        return Ok(true);
    }
    let _busy = Lock::acquire(Path::new(GLOBAL_LOCK), true)?;
    if pause_requested(context)? {
        return Ok(false);
    }
    fs::create_dir_all(&folder)?;
    fs::copy(
        context.run.join("current-host.txt"),
        folder.join("host-record.txt"),
    )?;
    let binary = context.protocol["extensions"]["frozen_binary_path"]
        .as_str()
        .ok_or("missing scientific executable")?;
    let argv = json!([
        binary,
        "case",
        STUDY,
        case.draw.to_string(),
        case.guesses.to_string(),
        case.branch.to_string()
    ]);
    atomic_json(&folder.join("command.json"), &argv)?;
    atomic_bytes(
        &folder.join("started.txt"),
        format!("{}\n", now()?).as_bytes(),
    )?;
    context.checkpoint()?;
    let output = File::create(folder.join("output.json"))?;
    let errors = File::create(folder.join("stderr.txt"))?;
    // Arguments are separate argv elements; no protocol/user string is interpolated as shell code.
    let status=Command::new("bash").arg("-c").arg(
        "ulimit -v 8388608; exec /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o \"$1\" timeout -k 30 900 \"$2\" case \"$3\" \"$4\" \"$5\" \"$6\""
    ).arg("bash").arg(folder.join("resources.txt")).arg(binary).arg(STUDY)
        .arg(case.draw.to_string()).arg(case.guesses.to_string()).arg(case.branch.to_string())
        .current_dir(&context.root).env("RAYON_NUM_THREADS","1").env("OMP_NUM_THREADS","1").env("HKY_CHUNK","1024")
        .stdin(Stdio::null()).stdout(Stdio::from(output)).stderr(Stdio::from(errors)).status()?;
    let code = status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(0));
    File::open(folder.join("output.json"))?.sync_all()?;
    File::open(folder.join("stderr.txt"))?.sync_all()?;
    atomic_json(
        &folder.join("status.json"),
        &json!({"exit_code":code,"address_space_bytes":8589934592u64,"timeout_seconds":900,"wall_valid":false,"native_controller":true}),
    )?;
    atomic_bytes(
        &folder.join("ended.txt"),
        format!("{}\n", now()?).as_bytes(),
    )?;
    context.refresh()?;
    context.checkpoint()?;
    println!(
        "arm={arm} draw={} k={} branch={} exit={code}",
        case.draw, case.guesses, case.branch
    );
    Ok(true)
}

fn run(context: &Context, resume: bool) -> Result<()> {
    context.guard()?;
    if resume {
        require(
            context.run.is_dir(),
            "native continuation directory does not exist",
        )?;
    } else {
        require(
            !context.run.exists(),
            "native continuation already exists; inspect then use --resume",
        )?;
        fs::create_dir_all(&context.run)?;
    }
    let _owner = Lock::acquire(&context.run.join(".batch.lock"), false)?;
    if resume {
        verify_manifest(&context.run)?;
        let directory = context.export_directory()?;
        let export = directory.join("export.json");
        if export.exists() {
            require(
                context
                    .case_paths()
                    .iter()
                    .all(|p| p.join("ended.txt").exists()),
                "a final export exists beside unfinished execution cells",
            )?;
            context.validate_export(&read_json(&export)?, &directory)?;
            println!("All native cells and the immutable final export already exist; no metadata overwritten.");
            return Ok(());
        }
    }
    // SAFETY: getpgrp has no pointer arguments and only reads this process's group.
    let pgid = unsafe { libc::getpgrp() };
    if resume {
        // This function verifies the OLD SHA manifest before changing process metadata.
        restore_records(
            &context.run,
            &context.case_paths(),
            std::process::id(),
            pgid,
        )?;
    } else {
        atomic_bytes(
            &context.run.join("runner.pid"),
            format!("{}\n", std::process::id()).as_bytes(),
        )?;
        atomic_bytes(
            &context.run.join("runner.pgid"),
            format!("{pgid}\n").as_bytes(),
        )?;
    }
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    atomic_bytes(
        &context.run.join(format!("runner-started-{nonce}.txt")),
        format!("{}\n", now()?).as_bytes(),
    )?;
    context.host()?;
    atomic_json(
        &context.run.join("instrument-pins.json"),
        &json!({"scientific":context.protocol["arms"][0]["instrument"],"native_controller_sha256":context.native()["binary_sha256"],"protocol_sha256":context.protocol["freeze"]["protocol_sha256"],"input_sha256":context.protocol["extensions"]["input_sha256"]}),
    )?;
    context.checkpoint()?;
    for case in &context.cases {
        let arms = if case.ordinal.is_multiple_of(2) {
            ["hky", "hky-aa"]
        } else {
            ["hky-aa", "hky"]
        };
        for arm in arms {
            if pause_requested(context)? || !child(context, case, arm)? {
                atomic_bytes(
                    &context.run.join("paused.txt"),
                    format!(
                        "{}\nControlled pause before preparing another scientific case.\n",
                        now()?
                    )
                    .as_bytes(),
                )?;
                context.checkpoint()?;
                return Ok(());
            }
        }
    }
    let rows = context.refresh()?;
    atomic_bytes(
        &context.run.join("ended.txt"),
        format!("{}\n", now()?).as_bytes(),
    )?;
    atomic_bytes(&context.run.join("REVIEW_REQUIRED.txt"),b"All244 prospective execution cells have outcomes; caps and interruptions remain censored. Four legacy timeouts remain separate. Inspect raw rows, A/A pairs, the complete-price gaps and the native export; archive all raw records, obtain independent review and inspect report/graph/PDF. No automatic claim promotion.\n")?;
    let path = context.final_export(&rows)?;
    context.checkpoint()?;
    println!("Native export written once: {}", path.display());
    Ok(())
}

fn freeze(root: &Path, protocol_path: &Path) -> Result<()> {
    let mut context = Context::open(root, protocol_path, false)?;
    require(
        !context.run.exists(),
        "new native scientific data already exist; cannot prospectively freeze",
    )?;
    let digest = protocol_hash(&context.protocol)?;
    context.protocol["freeze"] = json!({"status":"frozen","protocol_sha256":digest,"commit":git_head(&context.root)?,"frozen_at":now()?,"filed_before_affected_data":true});
    context.guard()?;
    validate_schema(
        &context.protocol,
        Path::new(
            context.native()["protocol_schema"]["path"]
                .as_str()
                .unwrap(),
        ),
    )?;
    atomic_json(&context.protocol_path, &context.protocol)?;
    println!(
        "Frozen before244 new native cases: {}",
        context.protocol["freeze"]["protocol_sha256"]
    );
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    require(
        (args.len() == 3) || (args.len() == 4 && args[0] == "run" && args[3] == "--resume"),
        "usage: ic-hky-replay-records guard|freeze|run ROOT PROTOCOL [--resume]",
    )?;
    let root = PathBuf::from(&args[1]);
    let protocol = PathBuf::from(&args[2]);
    if args[0] == "freeze" {
        return freeze(&root, &protocol);
    }
    let context = Context::open(&root, &protocol, true)?;
    context.guard()?;
    match args[0].as_str() {
        "guard" => {
            println!("Native frozen protocol, schema, source and binary guards pass.");
            Ok(())
        }
        "run" => run(
            &context,
            args.get(3).map(String::as_str) == Some("--resume"),
        ),
        _ => Err("unknown native replay operation".into()),
    }
}

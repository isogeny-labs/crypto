//! Command line: the same subcommands and flags as the tool's first version.
use crate::cmd::{self, fail, Fail, Res};
use crate::kernels;
use std::collections::BTreeMap;

pub const USAGE: &str = "optlevel: how close does an implementation run to what its machine can do?

    optlevel probe                              what this host can measure
    optlevel calibrate --out CAL.json           machine ceiling (roofline) of this host
    optlevel measure --out M.json [opts] -- CMD measure a kernel (perf stat with a PMU, else timing)
    optlevel score --calibration CAL.json --measurement M.json --out R.json
    optlevel plot --calibration CAL.json --out roofline.svg R.json [R2.json ...]
    optlevel controls --out DIR                 the synthetic control kernels, end to end
    optlevel vector-share BINARY SYMBOL...      static share of packed-vector instructions
    optlevel gpu probe|calibrate|measure ...    GPU path; refuses without nvidia-smi
    optlevel validate R.json                    check a record against the schema and tier rules
    optlevel benchrec R.json                    the record's values as `benchrec.py metric` arguments
    optlevel summary R.json [R2.json ...]       one JSON line per record (tier, attainment, grade, flags)
    optlevel kernel NAME [--isa I] [--n N] [--seconds S]   one calibration kernel (what calibrate runs)

Definition, formulas, tiers and failure modes: docs/OPTIMIZATION_LEVEL.md.
The calibration kernels are inside this binary (src/kernels.rs), one variant
per instruction set, selected at run time; `calibrate --c-kernels` adds the C
builds of kernels/optk.c to the best-of.  OPTLEVEL_CPUS=N declares a CPU
allotment that neither the affinity mask nor a cgroup quota shows.
Exit status: 0 ok, 1 a record failed validation, 2 usage, 3 unsupported on this host, 4 the command failed.";

const CALIBRATE: &str = "optlevel calibrate --out CAL.json [--seconds 0.5] [--repeat 3] [--triad-bytes N] [--c-kernels]
  --seconds S      timed seconds per kernel run
  --triad-bytes N  bytes per triad array (default 1.5 x LLC)
  --c-kernels      also build kernels/optk.c with the system C compiler and take the best of both";
const MEASURE_OPTS: &str = "  --out M.json            (required)
  --label TEXT
  --repeat N              A/B pairs of the single-thread point (default 5; >= 5 for a full noise floor)
  --ops N                 operation count of one run, when the command prints none
  --ops-regex RE          regex with one group; matches in stdout+stderr are summed
  --op-class CLASS
  --bytes-per-op X        declared memory traffic per operation
  --working-set-bytes N   bytes the kernel touches repeatedly; at or below the LLC, declared traffic is not charged to DRAM
  --cwd DIR";
const MEASURE: &str = "optlevel measure --out M.json [options] -- CMD...
  --scale SPEC            copies:1,4 (pinned concurrent copies) or env:VAR:1,2,4 (thread-count variable)
  --timing-only           do not use perf even when a PMU exists
  --no-pin
  --warmup                one unrecorded run first
  --timeout SECONDS";
const SCORE: &str = "optlevel score --calibration CAL.json --measurement M.json --out R.json
  [--topdown topdown.json] [--vector-share X] [--ncu kernels.json] [--nsys summary.json]
  [--counted-ops N] [--best-known-ops N] [--best-known-source TEXT]
  [--repository R] [--path P] [--registry-id ID]";
const CONTROLS: &str = "optlevel controls --out DIR [--repeat 5] [--seconds 1.0] [--cal-seconds 0.5] [--cal-repeat 3]
  [--recalibrate] [--rescore] [--timing-only] [--c-kernels]
  --rescore   reuse the measurement files already in --out; only score and plot again";
const GPU: &str = "optlevel gpu probe [--no-ncu-check]
optlevel gpu calibrate --out CAL.json [--repeat 3]
optlevel gpu measure --out M.json [options] -- CMD...";

/// Parsed arguments: valued options, boolean flags, positionals.
pub struct Parsed {
    opts: BTreeMap<String, String>,
    flags: Vec<String>,
    pub pos: Vec<String>,
    help: bool,
}

/// `values` and `bools` name the accepted options (without the dashes).  With
/// `remainder`, everything from the first positional or `--` on is the command.
pub fn parse(args: &[String], values: &[&str], bools: &[&str], remainder: bool) -> Res<Parsed> {
    let mut p = Parsed { opts: BTreeMap::new(), flags: Vec::new(), pos: Vec::new(), help: false };
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a == "--" {
            p.pos.extend(args[i + 1..].iter().cloned());
            break;
        }
        if a == "--help" || a == "-h" {
            p.help = true;
            i += 1;
            continue;
        }
        if let Some(name) = a.strip_prefix("--") {
            let (name, inline) = match name.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (name, None),
            };
            if bools.contains(&name) && inline.is_none() {
                p.flags.push(name.to_string());
                i += 1;
                continue;
            }
            if values.contains(&name) {
                let v = match inline {
                    Some(v) => v,
                    None => {
                        i += 1;
                        args.get(i).cloned().ok_or_else(|| Fail { code: 2, msg: format!("--{name} needs a value") })?
                    }
                };
                p.opts.insert(name.to_string(), v);
                i += 1;
                continue;
            }
            return fail(2, format!("unknown option --{name}"));
        }
        if remainder {
            p.pos.extend(args[i..].iter().cloned());
            break;
        }
        p.pos.push(a.clone());
        i += 1;
    }
    Ok(p)
}
impl Parsed {
    pub fn flag(&self, name: &str) -> bool {
        self.flags.iter().any(|f| f == name)
    }
    pub fn opt(&self, name: &str) -> Option<String> {
        self.opts.get(name).cloned()
    }
    pub fn req(&self, name: &str) -> Res<String> {
        self.opt(name).ok_or_else(|| Fail { code: 2, msg: format!("--{name} is required") })
    }
    pub fn num<T: std::str::FromStr>(&self, name: &str) -> Res<Option<T>> {
        match self.opts.get(name) {
            None => Ok(None),
            Some(v) => v.parse().map(Some).map_err(|_| Fail { code: 2, msg: format!("--{name}: bad value {v:?}") }),
        }
    }
    pub fn num_or<T: std::str::FromStr>(&self, name: &str, default: T) -> Res<T> {
        Ok(self.num(name)?.unwrap_or(default))
    }
}

const MEASURE_VALUES: [&str; 11] = ["out", "label", "repeat", "ops", "ops-regex", "op-class", "bytes-per-op", "working-set-bytes", "cwd", "scale", "timeout"];

fn measure_args(p: &Parsed) -> Res<cmd::MeasureArgs> {
    Ok(cmd::MeasureArgs {
        out: p.req("out")?, label: p.opt("label").unwrap_or_default(), repeat: p.num_or("repeat", 5)?, ops: p.num("ops")?,
        ops_regex: p.opt("ops-regex"), op_class: p.opt("op-class"), bytes_per_op: p.num("bytes-per-op")?,
        working_set_bytes: p.num("working-set-bytes")?, cwd: p.opt("cwd"), cmd: p.pos.clone(), scale: p.opt("scale"),
        timing_only: p.flag("timing-only"), no_pin: p.flag("no-pin"), warmup: p.flag("warmup"), timeout: p.num("timeout")?,
    })
}

fn dispatch(args: &[String]) -> Res<()> {
    let Some(sub) = args.first() else { return fail(2, USAGE) };
    let rest = &args[1..];
    let help = |p: &Parsed, text: &str| -> bool {
        if p.help {
            println!("{text}");
        }
        p.help
    };
    match sub.as_str() {
        "-h" | "--help" | "help" => {
            println!("{USAGE}");
            Ok(())
        }
        "kernel" => match kernels::main(rest) {
            0 => Ok(()),
            c => fail(c, String::new()),
        },
        "probe" => {
            let p = parse(rest, &[], &[], false)?;
            if help(&p, "optlevel probe") { return Ok(()) }
            cmd::cmd_probe()
        }
        "calibrate" => {
            let p = parse(rest, &["out", "seconds", "repeat", "triad-bytes"], &["c-kernels"], false)?;
            if help(&p, CALIBRATE) { return Ok(()) }
            cmd::cmd_calibrate(&cmd::CalibrateArgs { out: p.req("out")?, seconds: p.num_or("seconds", 0.5)?, repeat: p.num_or("repeat", 3)?,
                                                     triad_bytes: p.num("triad-bytes")?, c_kernels: p.flag("c-kernels") })
        }
        "measure" => {
            let p = parse(rest, &MEASURE_VALUES, &["timing-only", "no-pin", "warmup"], true)?;
            if help(&p, &format!("{MEASURE}\n{MEASURE_OPTS}")) { return Ok(()) }
            cmd::cmd_measure(&measure_args(&p)?)
        }
        "score" => {
            let p = parse(rest, &["calibration", "measurement", "out", "topdown", "vector-share", "ncu", "nsys", "counted-ops", "best-known-ops",
                                  "best-known-source", "repository", "path", "registry-id"], &[], false)?;
            if help(&p, SCORE) { return Ok(()) }
            cmd::cmd_score(&cmd::ScoreArgs {
                calibration: p.req("calibration")?, measurement: p.req("measurement")?, out: p.req("out")?, topdown: p.opt("topdown"),
                vector_share: p.num("vector-share")?, ncu: p.opt("ncu"), nsys: p.opt("nsys"), counted_ops: p.num("counted-ops")?,
                best_known_ops: p.num("best-known-ops")?, best_known_source: p.opt("best-known-source"), repository: p.opt("repository"),
                path: p.opt("path"), registry_id: p.opt("registry-id"),
            })
        }
        "plot" => {
            let p = parse(rest, &["calibration", "out", "title"], &[], false)?;
            if help(&p, "optlevel plot --calibration CAL.json --out roofline.svg [--title T] R.json [R2.json ...]") { return Ok(()) }
            if p.pos.is_empty() {
                return fail(2, "plot needs at least one record");
            }
            cmd::cmd_plot(&p.req("calibration")?, &p.req("out")?, &p.opt("title").unwrap_or_else(|| "Roofline".into()), &p.pos)
        }
        "controls" => {
            let p = parse(rest, &["out", "repeat", "seconds", "cal-seconds", "cal-repeat"], &["recalibrate", "rescore", "timing-only", "c-kernels"], false)?;
            if help(&p, CONTROLS) { return Ok(()) }
            cmd::cmd_controls(&cmd::ControlsArgs { out: p.req("out")?, repeat: p.num_or("repeat", 5)?, seconds: p.num_or("seconds", 1.0)?,
                                                   cal_seconds: p.num_or("cal-seconds", 0.5)?, cal_repeat: p.num_or("cal-repeat", 3)?,
                                                   recalibrate: p.flag("recalibrate"), rescore: p.flag("rescore"), timing_only: p.flag("timing-only"),
                                                   c_kernels: p.flag("c-kernels") })
        }
        "vector-share" => {
            let p = parse(rest, &[], &[], false)?;
            if help(&p, "optlevel vector-share BINARY SYMBOL...") { return Ok(()) }
            if p.pos.len() < 2 {
                return fail(2, "vector-share BINARY SYMBOL...");
            }
            print!("{}", crate::util::to_pretty(&cmd::vector_share(&p.pos[0], &p.pos[1..])?));
            Ok(())
        }
        "gpu" => {
            let Some(g) = rest.first() else { return fail(2, GPU) };
            let rest = &rest[1..];
            match g.as_str() {
                "probe" => {
                    let p = parse(rest, &[], &["no-ncu-check"], false)?;
                    if help(&p, GPU) { return Ok(()) }
                    cmd::cmd_gpu_probe(!p.flag("no-ncu-check"))
                }
                "calibrate" => {
                    let p = parse(rest, &["out", "repeat"], &[], false)?;
                    if help(&p, GPU) { return Ok(()) }
                    cmd::cmd_gpu_calibrate(&cmd::GpuCalibrateArgs { out: p.req("out")?, repeat: p.num_or("repeat", 3)? })
                }
                "measure" => {
                    let p = parse(rest, &MEASURE_VALUES[..9], &[], true)?;
                    if help(&p, &format!("optlevel gpu measure --out M.json [options] -- CMD...\n{MEASURE_OPTS}")) { return Ok(()) }
                    cmd::cmd_gpu_measure(&measure_args(&p)?)
                }
                "-h" | "--help" => {
                    println!("{GPU}");
                    Ok(())
                }
                _ => fail(2, GPU),
            }
        }
        "validate" => {
            let p = parse(rest, &[], &[], false)?;
            if help(&p, "optlevel validate R.json [R2.json ...]") { return Ok(()) }
            if p.pos.is_empty() {
                return fail(2, "validate needs at least one record");
            }
            cmd::cmd_validate(&p.pos)
        }
        "benchrec" => {
            let p = parse(rest, &[], &[], false)?;
            if help(&p, "optlevel benchrec R.json") { return Ok(()) }
            match p.pos.as_slice() {
                [r] => cmd::cmd_benchrec(r),
                _ => fail(2, "benchrec R.json"),
            }
        }
        "summary" => {
            let p = parse(rest, &[], &[], false)?;
            if help(&p, "optlevel summary R.json [R2.json ...]") { return Ok(()) }
            cmd::cmd_summary(&p.pos)
        }
        other => fail(2, format!("unknown command {other:?}\n\n{USAGE}")),
    }
}

pub fn main(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(()) => 0,
        Err(Fail { code, msg }) => {
            if !msg.is_empty() {
                eprintln!("optlevel: {msg}");
            }
            code
        }
    }
}

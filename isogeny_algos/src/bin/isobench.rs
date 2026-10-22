//! `isobench`: the isogeny benchmark harness and frontier (see `docs/isobench/README.md`).
use isogeny_algos::bigint::Big;
use isogeny_algos::field::{Rng, Zp};
use isogeny_algos::fpm::FpM;
use isogeny_algos::isobench::bounds;
use isogeny_algos::isobench::counted::{calibrate_field, Calibration, FieldPrices};
use isogeny_algos::isobench::harness::{self, Spec};
use isogeny_algos::isobench::problems::{self, Params};
use isogeny_algos::json::Json;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const HELP: &str = "isobench <command> [options]

  problems                                  the problems, their units, methods and parameters (JSON)
  calibrate --out FILE [--max-log K]        measure the kernel prices (field-multiplication equivalents)
  plan SPEC                                 the deterministic execution plan of a spec (JSON)
  run SPEC --out DIR --cal FILE [--isolate CPU --bench-tool PATH [--max-other X]]
                                            run a session: one child process per execution, verified,
                                            sealed records; --isolate pins each child through
                                            tools/isolated_bench.py
  verify DIR --cal FILE [--replay N | --replay-all]
                                            audit a session: hashes, seals, plan, answers, replays
  bound fit --arm NAME --out FILE DIR...    fit one arm of one or more sessions into a sealed bound
  bound check FILE --sessions ROOT          re-derive a committed bound from its sessions
  frontier build --bounds DIR --out MD --json FILE [--check]
                                            the frontier page; --check fails if the files are stale
  challenge seal --problem P [--params 'k=v;..'] --incumbent M --candidate M --sizes a,b,.. \\
                 --instances N --rounds R --seed S --out FILE
  challenge spec FILE                       print the challenge's frozen spec (to run it)
  challenge verdict --challenge FILE --session DIR --out FILE
  exec ...                                  (the child of `run`; not for direct use)";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli(&args) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("isobench: {e}");
            std::process::exit(if e == "help" { 0 } else { 2 });
        }
    }
}

struct Opts {
    flags: BTreeMap<String, String>,
    pos: Vec<String>,
}

fn opts(args: &[String], switches: &[&str]) -> Result<Opts, String> {
    let mut flags = BTreeMap::new();
    let mut pos = vec![];
    let mut i = 0;
    while i < args.len() {
        if let Some(k) = args[i].strip_prefix("--") {
            if switches.contains(&k) {
                flags.insert(k.to_string(), String::new());
                i += 1;
            } else {
                let v = args
                    .get(i + 1)
                    .ok_or_else(|| format!("--{k} needs a value"))?;
                flags.insert(k.to_string(), v.clone());
                i += 2;
            }
        } else {
            pos.push(args[i].clone());
            i += 1;
        }
    }
    Ok(Opts { flags, pos })
}

impl Opts {
    fn need(&self, k: &str) -> Result<&str, String> {
        self.flags
            .get(k)
            .map(|s| s.as_str())
            .ok_or_else(|| format!("--{k} is required"))
    }
    fn u64(&self, k: &str) -> Result<u64, String> {
        self.need(k)?
            .parse()
            .map_err(|_| format!("--{k}: not an integer"))
    }
}

fn read(p: &str) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))
}

fn write(p: &Path, s: &str) -> Result<(), String> {
    std::fs::write(p, s).map_err(|e| format!("{}: {e}", p.display()))
}

fn parse_params(s: Option<&str>) -> Result<Params, String> {
    let mut m = Params::new();
    if let Some(s) = s {
        for kv in s.split(';').filter(|t| !t.is_empty()) {
            let (k, v) = kv.split_once('=').ok_or("params: expected k=v;...")?;
            m.insert(k.to_string(), v.to_string());
        }
    }
    Ok(m)
}

fn params_arg(p: &Params) -> String {
    p.iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn cli(args: &[String]) -> Result<(), String> {
    let Some(cmd) = args.first() else {
        println!("{HELP}");
        return Ok(());
    };
    match cmd.as_str() {
        "help" | "--help" | "-h" => {
            println!("{HELP}");
            Ok(())
        }
        "problems" => {
            let v: Vec<Json> = problems::registry()
                .iter()
                .map(|p| {
                    Json::obj(vec![
                        ("id", Json::str(p.id)),
                        ("summary", Json::str(p.summary)),
                        ("size", Json::str(p.size)),
                        ("unit", Json::str(p.unit)),
                        ("declared_alpha", Json::Float(p.declared_alpha)),
                        (
                            "methods",
                            Json::Arr(p.methods.iter().map(Json::str).collect()),
                        ),
                        (
                            "params",
                            Json::Obj(
                                p.params
                                    .iter()
                                    .map(|(k, d)| (k.to_string(), Json::str(d)))
                                    .collect(),
                            ),
                        ),
                    ])
                })
                .collect();
            println!("{}", Json::Arr(v).dump());
            Ok(())
        }
        "calibrate" => {
            let o = opts(&args[1..], &[])?;
            let max_log = o
                .flags
                .get("max-log")
                .map_or(Ok(13), |s| s.parse().map_err(|_| "--max-log"))?;
            let out = PathBuf::from(o.need("out")?);
            let mut rng = Rng::new(1);
            let mut fields: BTreeMap<String, FieldPrices> = BTreeMap::new();
            eprintln!("calibrating zp");
            fields.insert(
                "zp".into(),
                calibrate_field(&Zp::new(2305843009213693951), max_log, &mut rng),
            );
            macro_rules! fpm {
                ($n:expr) => {{
                    eprintln!("calibrating fpm{}", $n);
                    let p = calibration_prime($n);
                    let f = FpM::<$n>::new(&p);
                    fields.insert(format!("fpm{}", $n), calibrate_field(&f, max_log, &mut rng));
                }};
            }
            fpm!(1);
            fpm!(2);
            fpm!(3);
            fpm!(4);
            fpm!(5);
            fpm!(6);
            fpm!(7);
            fpm!(8);
            let cal = Calibration {
                id: String::new(),
                fields,
            };
            write(&out, &(cal.to_json().dump() + "\n"))?;
            let back = Calibration::from_json(&read(out.to_str().unwrap())?)?;
            eprintln!("wrote {} ({})", out.display(), back.id);
            Ok(())
        }
        "plan" => {
            let o = opts(&args[1..], &[])?;
            let spec = Spec::parse(&read(o.pos.first().ok_or("plan SPEC")?)?)?;
            println!(
                "{}",
                harness::plan_json(&spec, &harness::plan(&spec)).dump()
            );
            Ok(())
        }
        "exec" => {
            let o = opts(&args[1..], &[])?;
            let cal = Calibration::from_json(&read(o.need("cal")?)?)?;
            let out = harness::child(
                o.need("problem")?,
                o.need("method")?,
                &parse_params(o.flags.get("params").map(|s| s.as_str()))?,
                o.u64("size")?,
                o.u64("index")?,
                o.u64("seed")?,
                &cal,
            );
            println!("{}", harness::child_json(&out).dump());
            Ok(())
        }
        "run" => {
            let o = opts(&args[1..], &[])?;
            let spec_text = read(o.pos.first().ok_or("run SPEC --out DIR --cal FILE")?)?;
            let spec = Spec::parse(&spec_text)?;
            let cal_path = PathBuf::from(o.need("cal")?);
            let cal = Calibration::from_json(&read(o.need("cal")?)?)?;
            let dir = PathBuf::from(o.need("out")?);
            if dir.join("records.jsonl").exists() {
                return Err(format!("{} already holds a session", dir.display()));
            }
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let isolate = o.flags.get("isolate").cloned();
            let tool = o.flags.get("bench-tool").cloned();
            let max_other = o
                .flags
                .get("max-other")
                .cloned()
                .unwrap_or_else(|| "0.5".into());
            if isolate.is_some() && tool.is_none() {
                return Err("--isolate needs --bench-tool (tools/isolated_bench.py)".into());
            }
            write(&dir.join("spec.json"), &(spec.doc.dump() + "\n"))?;
            let p = harness::plan(&spec);
            write(
                &dir.join("plan.json"),
                &(harness::plan_json(&spec, &p).dump() + "\n"),
            )?;
            write(
                &dir.join("host.json"),
                &(harness::host_json().dump() + "\n"),
            )?;
            let mut rec_file =
                std::fs::File::create(dir.join("records.jsonl")).map_err(|e| e.to_string())?;
            let mut records = vec![];
            for e in &p {
                let iso_out = dir.join("isolation.jsonl");
                let (child, isolation) = spawn_child(
                    &spec,
                    e,
                    &cal_path,
                    isolate.as_deref().map(|c| {
                        (
                            c,
                            tool.as_deref().unwrap(),
                            max_other.as_str(),
                            iso_out.as_path(),
                        )
                    }),
                )?;
                let r = harness::record(&spec, e, &child, &cal, isolation);
                eprintln!(
                    "[{}/{}] size {} inst {} round {} {}: {} cost {} wall {} ms",
                    e.no + 1,
                    p.len(),
                    e.size,
                    e.index,
                    e.round,
                    spec.arms[e.arm].name,
                    r.get("status").and_then(|s| s.as_str()).unwrap_or("?"),
                    r.get("cost").and_then(|c| c.as_f64()).unwrap_or(f64::NAN),
                    r.get("wall_ms")
                        .and_then(|c| c.as_f64())
                        .unwrap_or(f64::NAN),
                );
                writeln!(rec_file, "{}", r.dump()).map_err(|e| e.to_string())?;
                records.push(r);
            }
            drop(rec_file);
            let session = harness::session_json(&dir, &spec, &cal, &records)?;
            write(&dir.join("session.json"), &(session.dump() + "\n"))?;
            println!("{}", session.dump());
            Ok(())
        }
        "verify" => {
            let o = opts(&args[1..], &["replay-all"])?;
            let dir = PathBuf::from(o.pos.first().ok_or("verify DIR --cal FILE")?);
            let cal_path = PathBuf::from(o.need("cal")?);
            let cal = Calibration::from_json(&read(o.need("cal")?)?)?;
            let replay = if o.flags.contains_key("replay-all") {
                None
            } else {
                Some(
                    o.flags
                        .get("replay")
                        .map_or(Ok(0), |s| s.parse().map_err(|_| "--replay"))?,
                )
            };
            let rerun =
                |spec: &Spec, e: &harness::Exec| spawn_child(spec, e, &cal_path, None).map(|r| r.0);
            let (bad, replayed) = harness::audit(&dir, &cal, replay, &rerun)?;
            for b in &bad {
                eprintln!("FAIL {b}");
            }
            println!(
                "{}",
                Json::obj(vec![
                    ("session", Json::str(dir.display())),
                    ("replayed", Json::Num(replayed as i64)),
                    ("findings", Json::Num(bad.len() as i64)),
                    (
                        "status",
                        Json::str(if bad.is_empty() { "PASS" } else { "FAIL" })
                    ),
                ])
                .dump()
            );
            if bad.is_empty() {
                Ok(())
            } else {
                Err(format!("{} findings", bad.len()))
            }
        }
        "bound" => {
            let sub = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let o = opts(&args[2..], &[])?;
            match sub {
                "fit" => {
                    let dirs: Vec<PathBuf> = o.pos.iter().map(PathBuf::from).collect();
                    let refs: Vec<&Path> = dirs.iter().map(|d| d.as_path()).collect();
                    let b = bounds::bound(&refs, o.need("arm")?)?;
                    write(Path::new(o.need("out")?), &(b.dump() + "\n"))?;
                    println!("{}", b.get("id").and_then(|i| i.as_str()).unwrap_or(""));
                    Ok(())
                }
                "check" => {
                    let root = PathBuf::from(o.need("sessions")?);
                    for f in &o.pos {
                        bounds::check_bound(&read(f)?, &root).map_err(|e| format!("{f}: {e}"))?;
                        eprintln!("ok {f}");
                    }
                    Ok(())
                }
                _ => Err("bound fit | bound check".into()),
            }
        }
        "frontier" => {
            let o = opts(&args[2..], &["check"])?;
            let (md, j) = bounds::frontier(Path::new(o.need("bounds")?))?;
            let (mdp, jp) = (
                PathBuf::from(o.need("out")?),
                PathBuf::from(o.need("json")?),
            );
            let js = j.dump() + "\n";
            if o.flags.contains_key("check") {
                let stale = std::fs::read_to_string(&mdp).ok() != Some(md.clone())
                    || std::fs::read_to_string(&jp).ok() != Some(js.clone());
                if stale {
                    return Err("the frontier page is stale: run `isobench frontier build`".into());
                }
                eprintln!("frontier up to date");
                return Ok(());
            }
            write(&mdp, &md)?;
            write(&jp, &js)?;
            Ok(())
        }
        "challenge" => {
            let sub = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let o = opts(&args[2..], &[])?;
            match sub {
                "seal" => {
                    let problem = o.need("problem")?;
                    let decl = problems::problem(problem)?;
                    let params = problems::resolve_params(
                        decl,
                        &parse_params(o.flags.get("params").map(|s| s.as_str()))?,
                    )?;
                    let sizes: Vec<u64> = o
                        .need("sizes")?
                        .split(',')
                        .map(|t| t.parse::<u64>().map_err(|_| "--sizes".to_string()))
                        .collect::<Result<_, _>>()?;
                    let c = bounds::challenge(
                        problem,
                        &params,
                        o.need("incumbent")?,
                        o.need("candidate")?,
                        &sizes,
                        o.u64("instances")?,
                        o.u64("rounds")?,
                        o.u64("seed")?,
                    )?;
                    write(Path::new(o.need("out")?), &(c.dump() + "\n"))?;
                    println!("{}", c.get("id").and_then(|i| i.as_str()).unwrap_or(""));
                    Ok(())
                }
                "spec" => {
                    let c = Json::parse(&read(o.pos.first().ok_or("challenge spec FILE")?)?)?;
                    println!("{}", c.get("spec").ok_or("no spec")?.dump());
                    Ok(())
                }
                "verdict" => {
                    let v = bounds::verdict(
                        &read(o.need("challenge")?)?,
                        Path::new(o.need("session")?),
                    )?;
                    write(Path::new(o.need("out")?), &(v.dump() + "\n"))?;
                    println!("{}", v.dump());
                    Ok(())
                }
                _ => Err("challenge seal | spec | verdict".into()),
            }
        }
        other => Err(format!("unknown command {other}\n\n{HELP}")),
    }
}

/// A prime for the calibration of N-limb fields: the first probable prime above 2^(64N - 1).
fn calibration_prime(n: usize) -> Big {
    let mut v = Big::from_u64(1).shl(64 * n - 1).add_small(1);
    loop {
        let i = isogeny_algos::int::Int::from_big(&v);
        if i.is_probable_prime() {
            return v;
        }
        v = v.add_small(2);
    }
}

/// Run one execution in a fresh child (optionally pinned by isolated_bench), returning its
/// output and the isolation summary.
fn spawn_child(
    spec: &Spec,
    e: &harness::Exec,
    cal: &Path,
    isolate: Option<(&str, &str, &str, &Path)>,
) -> Result<(Json, Json), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut child_args: Vec<String> = vec![
        exe.display().to_string(),
        "exec".into(),
        "--problem".into(),
        spec.problem.clone(),
        "--method".into(),
        spec.arms[e.arm].method.clone(),
        "--params".into(),
        params_arg(&spec.params),
        "--size".into(),
        e.size.to_string(),
        "--index".into(),
        e.index.to_string(),
        "--seed".into(),
        e.seed.to_string(),
        "--cal".into(),
        cal.display().to_string(),
    ];
    let mut isolation = Json::Null;
    if let Some((cpu, tool, max_other, out)) = isolate {
        let mut v = vec![
            "python3".to_string(),
            tool.to_string(),
            "run".into(),
            "--wait".into(),
            "--cpus".into(),
            cpu.to_string(),
            "--max-other-cpu".into(),
            max_other.to_string(),
            "--out".into(),
            out.display().to_string(),
            "--label".into(),
            format!("{} exec {}", spec.id, e.no),
            "--".into(),
        ];
        v.extend(child_args);
        child_args = v;
    }
    let output = Command::new(&child_args[0])
        .args(&child_args[1..])
        .output()
        .map_err(|e| format!("spawn: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "child failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let line = text
        .lines()
        .rev()
        .find(|l| l.starts_with('{'))
        .ok_or("child printed no record")?;
    if let Some((cpu, _, _, out)) = isolate {
        let last = std::fs::read_to_string(out)
            .ok()
            .and_then(|t| t.lines().last().map(String::from))
            .and_then(|l| Json::parse(&l).ok());
        let contended = last
            .as_ref()
            .and_then(|j| j.get("run"))
            .and_then(|r| r.get("contended"))
            .cloned()
            .unwrap_or(Json::Null);
        isolation = Json::obj(vec![("cpus", Json::str(cpu)), ("contended", contended)]);
    }
    Ok((Json::parse(line)?, isolation))
}

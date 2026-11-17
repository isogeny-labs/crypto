//! The subcommands: probe, calibrate, measure, score, plot, controls,
//! vector-share, gpu, validate, benchrec, summary.
use crate::host::{self, conditions_begin, conditions_end, host_info};
use crate::kernels;
use crate::plot;
use crate::run::{run_once, RunSpec};
use crate::score::{self as S, f, stats, stats_opt, truthy, Extras};
use crate::util::*;
use regex::Regex;
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A failure with the process exit status it maps to:
/// 2 usage, 3 unsupported on this host, 4 the command failed.
#[derive(Debug)]
pub struct Fail {
    pub code: i32,
    pub msg: String,
}
impl From<String> for Fail {
    fn from(msg: String) -> Self {
        Fail { code: 2, msg }
    }
}
pub type Res<T> = Result<T, Fail>;
pub fn fail<T>(code: i32, msg: impl Into<String>) -> Res<T> {
    Err(Fail { code, msg: msg.into() })
}

pub const SCHEMA_TEXT: &str = include_str!("../schema/optlevel.record.v1.schema.json");
const OPTK_C: &str = include_str!("../kernels/optk.c");
const GPU_CALIB_CU: &str = include_str!("../kernels/gpu_calib.cu");
const KERNELS_RS: &str = include_str!("kernels.rs");

/// C builds of kernels/optk.c (`calibrate --c-kernels`): name -> extra flags; every build also gets C_BASE_FLAGS.
pub const C_BUILDS: [(&str, &[&str]); 3] = [
    ("native", &[]),
    ("w512", &["-mprefer-vector-width=512"]),
    ("scalar", &["-fno-tree-vectorize", "-fno-tree-slp-vectorize"]),
];
pub const C_BASE_FLAGS: [&str; 3] = ["-O3", "-march=native", "-ffp-contract=fast"];
/// operation class -> kernels that realise it (array loops, plus a
/// register-resident variant where the array loop is load/store bound)
pub const CLASS_KERNELS: [(&str, &[&str]); 9] = [
    ("muladd_u32", &["muladd_u32"]), ("muladd_u64", &["muladd_u64"]), ("mulwide_u64", &["mulwide_u64"]),
    ("modmuladd_p31", &["modmuladd_p31"]), ("modmuladd_p62", &["modmuladd_p62"]),
    ("xor_u64", &["xor_u64"]), ("popcnt_u64", &["popcnt_u64"]),
    ("fma_f64", &["fma_f64", "fma_f64_reg"]), ("ptr_hop", &["chase"]),
];
pub fn class_doc(c: &str) -> &'static str {
    match c {
        "muladd_u32" => "a <- a*m + c on 32-bit words (wrapping)",
        "muladd_u64" => "a <- a*m + c on 64-bit words (wrapping)",
        "mulwide_u64" => "a <- hi64(a*m) + c: one 64x64->128 multiply per operation",
        "modmuladd_p31" => "a <- a + m*b mod p, p < 2^31 (Montgomery, 32x32->64 multiplies)",
        "modmuladd_p62" => "a <- a + m*b mod p, p < 2^62 (Montgomery, 64x64->128 multiplies)",
        "xor_u64" => "a <- a xor b on 64-bit words (one F_2 row-add word)",
        "popcnt_u64" => "popcount(a and b) on 64-bit words",
        "fma_f64" => "a <- a*s + b in binary64 (one fused multiply-add; float-encoded modular arithmetic)",
        "ptr_hop" => "one dependent pointer dereference",
        "triad" => "a <- b + s*c streamed over arrays (one multiply-add per element); compute roof = the fma_f64 ceiling, memory roof = bandwidth / bytes per element",
        _ => "",
    }
}
pub const PMU_CORE: &str = "cycles:u,instructions:u,branches:u,branch-misses:u";
pub const PMU_MEM: &str = "cycles:u,instructions:u,cache-misses:u,L1-dcache-load-misses:u,dTLB-load-misses:u";
const CACHE_LINE: f64 = 64.0;

fn note(msg: &str) {
    eprintln!("optlevel: {msg}");
}
fn schema() -> Value {
    serde_json::from_str(SCHEMA_TEXT).expect("the embedded schema is valid JSON")
}
fn flags_text(flags: &[String]) -> String {
    if flags.is_empty() { "none".into() } else { flags.join(", ") }
}

// ---- kernel builds -------------------------------------------------------------
pub fn cache_dir() -> Res<PathBuf> {
    let d = match std::env::var_os("OPTLEVEL_CACHE").filter(|v| !v.is_empty()) {
        Some(d) => PathBuf::from(d),
        None => match std::env::var_os("XDG_CACHE_HOME").filter(|v| !v.is_empty()) {
            Some(x) => PathBuf::from(x).join("optlevel"),
            None => PathBuf::from(std::env::var_os("HOME").unwrap_or_else(|| "/tmp".into())).join(".cache").join("optlevel"),
        },
    };
    std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    Ok(d)
}

/// The kernel builds a calibration can run: the Rust kernels inside this
/// binary (one per isa), and with `c_kernels` the C builds of optk.c.
pub struct Builds {
    pub exe: String,
    /// build name -> command prefix (before the kernel name)
    pub builds: Vec<(String, Vec<String>)>,
    pub doc: Value,
}
impl Builds {
    pub fn names(&self) -> Vec<&str> {
        self.builds.iter().map(|b| b.0.as_str()).collect()
    }
    pub fn is_scalar(name: &str) -> bool {
        name == "scalar" || name == "c-scalar"
    }
    /// argv of `kernel` in `build` (Rust: `optlevel kernel K --isa I`, C: `optk-I K`).
    pub fn cmd(&self, build: &str, kernel: &str) -> Res<Vec<String>> {
        let Some((_, pre)) = self.builds.iter().find(|b| b.0 == build) else {
            return fail(3, format!("kernel build {build:?} is not available here (builds: {})", self.names().join(" ")));
        };
        let mut v = pre.clone();
        if build.starts_with("c-") {
            v.push(kernel.into());
        } else {
            v.extend(["kernel".into(), kernel.into(), "--isa".into(), build.into()]);
        }
        Ok(v)
    }
    /// (binary, symbol) of a kernel's pass, for vector-share.
    pub fn symbol(&self, build: &str, kernel: &str) -> Res<(String, String)> {
        let pre = &self.builds.iter().find(|b| b.0 == build).ok_or_else(|| format!("no build {build}"))?.1;
        Ok(if build.starts_with("c-") { (pre[0].clone(), format!("pass_{kernel}")) } else { (self.exe.clone(), kernels::symbol(build, kernel)) })
    }
}

pub fn build_kernels(c_kernels: bool) -> Res<Builds> {
    let exe = std::env::current_exe().map_err(|e| format!("cannot find this executable: {e}"))?.to_string_lossy().into_owned();
    let isas = kernels::available_isas();
    let mut builds: Vec<(String, Vec<String>)> = isas.iter().map(|i| (i.to_string(), vec![exe.clone()])).collect();
    let mut doc = json!({"kernels": "rust (tools/optlevel/src/kernels.rs, inside the optlevel binary; one variant per isa, selected at run time)",
                         "source_sha256": sha256_hex(KERNELS_RS.as_bytes()), "isas": isas,
                         "exe": exe, "exe_sha256": sha256_file(Path::new(&exe)).ok(),
                         "c": null});
    if c_kernels {
        let cc = std::env::var("CC").ok().filter(|c| !c.is_empty()).or_else(|| which("cc")).or_else(|| which("gcc")).or_else(|| which("clang"));
        let Some(cc) = cc else { return fail(3, "--c-kernels: no C compiler (set CC)") };
        let ccv = Command::new(&cc).arg("--version").output().ok()
            .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().next().map(String::from)).unwrap_or_default();
        let fp = host_info()["fingerprint"].as_str().unwrap_or("").to_string();
        let tag = &sha256_hex(format!("{OPTK_C}{ccv}{fp}{C_BUILDS:?}{C_BASE_FLAGS:?}").as_bytes())[..12];
        let dir = cache_dir()?;
        let src = dir.join(format!("optk-{tag}.c"));
        std::fs::write(&src, OPTK_C).map_err(|e| format!("{}: {e}", src.display()))?;
        let mut cdoc = Map::new();
        for (name, extra) in C_BUILDS {
            let path = dir.join(format!("optk-{tag}-{name}"));
            let flags: Vec<&str> = C_BASE_FLAGS.iter().chain(extra.iter()).copied().collect();
            if !path.exists() {
                let tmp = dir.join(format!("optk-{tag}-{name}.tmp{}", std::process::id()));
                let r = Command::new(&cc).args(&flags).arg("-o").arg(&tmp).arg(&src).output().map_err(|e| format!("{cc}: {e}"))?;
                if !r.status.success() {
                    let err = String::from_utf8_lossy(&r.stderr).trim().to_string();
                    if !extra.is_empty() {
                        // an optional flag this compiler does not know: skip the build, say so
                        note(&format!("build c-{name} skipped: {}", err.lines().last().unwrap_or("compiler error")));
                        continue;
                    }
                    return fail(3, format!("cannot compile optk.c: {err}"));
                }
                std::fs::rename(&tmp, &path).map_err(|e| format!("{}: {e}", path.display()))?;
            }
            let p = path.to_string_lossy().into_owned();
            cdoc.insert(format!("c-{name}"), json!({"path": p, "flags": flags, "sha256": sha256_file(&path).ok()}));
            builds.push((format!("c-{name}"), vec![p]));
        }
        doc["c"] = json!({"compiler": cc, "compiler_version": ccv, "source_sha256": sha256_hex(OPTK_C.as_bytes()), "builds": cdoc});
    }
    Ok(Builds { exe, builds, doc })
}

/// The k highest-numbered allowed CPUs.
pub fn pick_cpus(k: usize) -> Res<Vec<usize>> {
    let mut allowed = host::affinity();
    allowed.reverse();
    if k > allowed.len() {
        return fail(2, format!("{k} copies requested but only {} CPUs are allowed", allowed.len()));
    }
    allowed.truncate(k);
    Ok(allowed)
}

// ---- calibrate -------------------------------------------------------------------
pub struct CalibrateArgs {
    pub out: String,
    pub seconds: f64,
    pub repeat: usize,
    pub triad_bytes: Option<u64>,
    pub c_kernels: bool,
}

pub fn cmd_calibrate(a: &CalibrateArgs) -> Res<()> {
    let host = host_info();
    let builds = build_kernels(a.c_kernels)?;
    let ncpu = host["usable_cpus"].as_u64().unwrap_or(1) as usize;
    let mut scale: Vec<usize> = [2usize, 4, 8, 16, 32, 64].into_iter().filter(|k| *k < ncpu).chain([1, ncpu]).collect();
    scale.sort();
    scale.dedup();
    let last = *scale.last().unwrap();
    let mut flags: Vec<String> = Vec::new();
    let begin = conditions_begin();
    let own = std::cell::Cell::new(0.0f64);
    let secs = a.seconds.to_string();
    let wide = builds.builds[0].0.clone(); // the widest Rust isa: streams, latency and the clock run in it

    let run = |build: &str, kernel: &str, k: usize, extra: &[String]| -> Res<Value> {
        let mut cmd = builds.cmd(build, kernel)?;
        cmd.extend(["--seconds".to_string(), secs.clone()]);
        cmd.extend(extra.iter().cloned());
        let r = run_once(&RunSpec { cmd: &cmd, copies: k, cpus: Some(pick_cpus(k)?), ..Default::default() })?;
        own.set(own.get() + f(&r, "cpu_s").unwrap_or(0.0));
        let failed = r["exit_status"].as_array().map(|a| a.iter().any(|x| x.as_i64() != Some(0))).unwrap_or(true);
        if r["ops"].is_null() || failed {
            return fail(4, format!("calibration kernel {kernel} ({build}) failed: {}", r["stderr_tail"].as_str().unwrap_or("")));
        }
        Ok(r)
    };
    let rate = |r: &Value| f(r, "ops_per_s").unwrap_or(f64::NAN);

    let mut clock = Vec::new();
    for _ in 0..a.repeat {
        clock.push(rate(&run(&wide, "clock", 1, &[])?));
    }
    let mut compute = Map::new();
    for (cls, kerns) in CLASS_KERNELS {
        let mut variants: Vec<(String, Vec<f64>)> = Vec::new();
        for _ in 0..a.repeat {
            for kern in kerns {
                for b in builds.names() {
                    if kern.ends_with("_reg") && Builds::is_scalar(b) {
                        continue;
                    }
                    let name = format!("{kern}@{b}");
                    let v = rate(&run(b, kern, 1, &[])?);
                    match variants.iter_mut().find(|x| x.0 == name) {
                        Some(x) => x.1.push(v),
                        None => variants.push((name, vec![v])),
                    }
                }
            }
        }
        let vstats: Vec<(String, Value)> = variants.iter().map(|(k, v)| (k.clone(), stats(v))).collect();
        let vmax = |v: &Value| f(v, "max").unwrap_or(f64::NEG_INFINITY);
        let mut best = &vstats[0];
        for v in &vstats {
            if vmax(&v.1) > vmax(&best.1) {
                best = v;
            }
        }
        let (bk, bb) = best.0.split_once('@').unwrap();
        let mut total = Map::new();
        total.insert("1".into(), best.1.clone());
        for &k in &scale[1..] {
            let mut rs = Vec::new();
            for _ in 0..a.repeat {
                rs.push(rate(&run(bb, bk, k, &[])?));
            }
            total.insert(k.to_string(), stats(&rs));
        }
        let side = |scalar: bool| vstats.iter().filter(|(k, _)| Builds::is_scalar(k.split_once('@').unwrap().1) == scalar).map(|(_, v)| vmax(v)).reduce(f64::max);
        note(&format!("{cls:14} peak {} ops/s/core ({}); all {last} cores {}", py_e(vmax(&best.1), 3), best.0, py_e(vmax(&total[&last.to_string()]), 3)));
        compute.insert(cls.into(), json!({"definition": class_doc(cls), "unit": "operations per second", "best_variant": best.0,
                                          "variants": vstats.iter().cloned().collect::<Map<String, Value>>(), "total_ops_per_s": total,
                                          "scalar_ops_per_s": side(true), "simd_ops_per_s": side(false)}));
    }

    // memory: triad arrays and the chase set each exceed the last-level cache
    let llc = host["caches_bytes"].as_object().and_then(|c| c.values().filter_map(Value::as_u64).max()).unwrap_or(32 << 20);
    let avail = Regex::new(r"MemAvailable:\s*(\d+)").unwrap().captures(&std::fs::read_to_string("/proc/meminfo").unwrap_or_default())
        .and_then(|m| m[1].parse::<u64>().ok()).map(|kb| kb * 1024).unwrap_or(4 << 30);
    let mut arr = (64u64 << 20).max((1.5 * llc as f64) as u64);
    let cap = (0.4 * avail as f64 / (3.0 * last as f64)) as u64;
    if let Some(t) = a.triad_bytes {
        arr = t;
    } else if arr > cap {
        arr = cap;
    }
    if arr < llc {
        flags.push("TRIAD_ARRAY_NOT_ABOVE_LLC".into());
    }
    let n_el = (arr / 8).to_string();
    // 32, 24 and 16 bytes per element
    let mut streams: Vec<(String, &str, &str)> = ["triad", "xor_u64", "popcnt_u64"].iter().map(|k| (k.to_string(), wide.as_str(), *k)).collect();
    if builds.names().contains(&"c-native") {
        streams.extend(["triad", "xor_u64", "popcnt_u64"].iter().map(|k| (format!("{k}@c-native"), "c-native", *k)));
    }
    let (mut bw, mut bw_variants) = (Map::new(), Map::new());
    for &k in &scale {
        let mut per: Vec<(String, Vec<f64>)> = streams.iter().map(|s| (s.0.clone(), Vec::new())).collect();
        for _ in 0..a.repeat {
            for (i, (_, build, kern)) in streams.iter().enumerate() {
                let r = run(build, kern, k, &["--n".into(), n_el.clone()])?;
                per[i].1.push(f(&r, "bytes_declared").unwrap_or(f64::NAN) / f(&r, "ops").unwrap_or(f64::NAN) * rate(&r));
            }
        }
        let st: Vec<(String, Value)> = per.iter().map(|(k, v)| (k.clone(), stats(v))).collect();
        let mut best = &st[0];
        for s in &st {
            if f(&s.1, "max") > f(&best.1, "max") {
                best = s;
            }
        }
        let mut b = best.1.clone();
        b["kernel"] = json!(best.0);
        note(&format!("bandwidth {k} core(s): {:.2} GB/s ({}; {})", f(&b, "max").unwrap_or(f64::NAN) / 1e9, best.0,
                      st.iter().map(|(n, v)| format!("{n} {:.2}", f(v, "max").unwrap_or(f64::NAN) / 1e9)).collect::<Vec<_>>().join(", ")));
        bw.insert(k.to_string(), b);
        bw_variants.insert(k.to_string(), Value::Object(st.into_iter().collect()));
    }
    let mut lat = Vec::new();
    for _ in 0..a.repeat {
        lat.push(1e9 / rate(&run(&wide, "chase", 1, &["--n".into(), (arr / 64).to_string()])?));
    }
    let cond = conditions_end(&begin, own.get());
    if f(&cond, "other_cpu_cores").unwrap_or(0.0) > S::CONTENDED_CORES {
        flags.push("CONTENDED".into());
    }
    if host["hypervisor"] == true {
        flags.push("VIRTUALIZED".into());
    }
    let cl = stats(&clock);
    if (f(&cl, "max").unwrap() - f(&cl, "min").unwrap()) / f(&cl, "median").unwrap() > 0.03 {
        flags.push("CLOCK_UNSTABLE".into());
    }
    let doc = json!({
        "schema": S::CALIBRATION_SCHEMA, "tool_version": S::TOOL_VERSION, "created_utc": utcnow(), "device": "cpu",
        "host": host, "build": builds.doc,
        "settings": {"seconds_per_kernel": a.seconds, "repeats": a.repeat, "scale_points": scale, "c_kernels": a.c_kernels},
        "ceiling_rule": "max over repeats and over builds of the summed rate of k pinned copies; the median is kept as the low end",
        "clock_hz_effective": cl, "compute": compute,
        "memory": {"bandwidth_bytes_per_s": bw, "bandwidth_variants": bw_variants,
                   "bytes_convention": "bytes moved over arrays above the LLC: loads + stores + write-allocate reads \
(triad 32 B/element, xor 24, popcount 16); the ceiling is the best of the three",
                   "triad_array_bytes": arr, "llc_bytes": llc, "latency_ns": stats(&lat),
                   "latency_definition": "ns per dependent load over a random cycle of 64-byte nodes the size of one triad array"},
        "conditions": cond, "flags": flags,
    });
    write_json(Path::new(&a.out), &doc)?;
    note(&format!("wrote {} (host {}, {} usable CPUs: {}, flags {})", a.out, host["fingerprint"].as_str().unwrap_or(""), ncpu,
                  host["usable_cpus_basis"].as_str().unwrap_or(""), flags_text(&flags)));
    Ok(())
}

// ---- measure -----------------------------------------------------------------------
/// "copies:1,4" or "env:VAR:1,2,4" -> (mode, var, [k...]).
pub fn parse_scale(spec: Option<&str>) -> Res<(String, Option<String>, Vec<usize>)> {
    let Some(spec) = spec.filter(|s| !s.is_empty()) else { return Ok(("copies".into(), None, vec![1])) };
    let parts: Vec<&str> = spec.split(':').collect();
    let ks = |s: &str| -> Option<Vec<usize>> {
        let mut v: Vec<usize> = s.split(',').map(|x| x.trim().parse().ok()).collect::<Option<_>>()?;
        v.sort();
        v.dedup();
        Some(v)
    };
    match parts.as_slice() {
        ["copies", k] => ks(k).map(|k| ("copies".to_string(), None, k)),
        ["env", var, k] => ks(k).map(|k| ("env".to_string(), Some(var.to_string()), k)),
        _ => None,
    }.ok_or_else(|| Fail { code: 2, msg: "--scale wants copies:1,4 or env:VAR:1,2,4".into() })
}

/// Per-operation PMU components from the single-thread runs
/// (`core`: the runs counted with PMU_CORE, `mem`: with PMU_MEM).
pub fn summarise_pmu(core: &[Value], mem: &[Value]) -> (Value, Vec<String>) {
    let med = |runs: &[Value], g: &dyn Fn(&Value) -> Option<f64>| f(&stats_opt(runs.iter().map(g)), "median");
    let counter = |r: &Value, ev: &str| r.get("counters").and_then(|c| f(c, ev));
    let per_op = |ev: &'static str| move |r: &Value| -> Option<f64> {
        let ops = f(r, "ops").filter(|o| *o != 0.0)?;
        Some(r.get("counters").and_then(|c| f(c, ev))? / ops)
    };
    let mut mux: Vec<String> = core.iter().chain(mem).flat_map(|r| r.get("multiplexed").and_then(Value::as_array).cloned().unwrap_or_default())
        .filter_map(|v| v.as_str().map(String::from)).collect();
    mux.sort();
    mux.dedup();
    let any = if !core.is_empty() { core } else { mem };
    let ratio = |num: &'static str, den: &'static str| move |r: &Value| -> Option<f64> {
        let d = counter(r, den).filter(|d| *d != 0.0)?;
        Some(counter(r, num)? / d)
    };
    let out = json!({
        "ipc": med(any, &ratio("instructions", "cycles")),
        "cycles_per_op": med(any, &per_op("cycles")),
        "instructions_per_op": med(any, &per_op("instructions")),
        "branch_miss_rate": med(core, &ratio("branch-misses", "branches")),
        "llc_misses_per_op": med(mem, &per_op("cache-misses")),
        "l1d_misses_per_op": med(mem, &per_op("L1-dcache-load-misses")),
        "dtlb_misses_per_op": med(mem, &per_op("dTLB-load-misses")),
    });
    (out, mux)
}

#[derive(Default, Clone)]
pub struct MeasureArgs {
    pub out: String,
    pub label: String,
    pub repeat: usize,
    pub ops: Option<i64>,
    pub ops_regex: Option<String>,
    pub op_class: Option<String>,
    pub bytes_per_op: Option<f64>,
    pub working_set_bytes: Option<i64>,
    pub cwd: Option<String>,
    pub cmd: Vec<String>,
    pub scale: Option<String>,
    pub timing_only: bool,
    pub no_pin: bool,
    pub warmup: bool,
    pub timeout: Option<f64>,
}

fn compile_regex(r: &Option<String>) -> Res<Option<Regex>> {
    match r {
        None => Ok(None),
        Some(r) => Regex::new(r).map(Some).map_err(|e| Fail { code: 2, msg: format!("--ops-regex: {e}") }),
    }
}
fn any_failed(r: &Value) -> bool {
    r["exit_status"].as_array().map(|a| a.iter().any(|x| x.as_i64() != Some(0))).unwrap_or(false)
}
fn cond_f(r: &Value, k: &str) -> Option<f64> {
    f(&r["conditions"], k)
}

pub fn cmd_measure(a: &MeasureArgs) -> Res<()> {
    if a.cmd.is_empty() {
        return fail(2, "no command (put it after --)");
    }
    let (mode, var, mut ks) = parse_scale(a.scale.as_deref())?;
    if !ks.contains(&1) {
        ks.insert(0, 1);
    }
    let host = host_info();
    let (perf, why) = if a.timing_only {
        (None, Some("disabled by --timing-only".to_string()))
    } else {
        match host::pmu_probe() {
            Ok(p) => (Some(p), None),
            Err(e) => (None, Some(e)),
        }
    };
    let tier = if perf.is_some() { "T-pmu" } else { "T-timing" };
    let mut flags: Vec<String> = Vec::new();
    if host["hypervisor"] == true {
        flags.push("VIRTUALIZED".into());
    }
    if host["smt_active"] == "1" {
        flags.push("SMT_ACTIVE".into());
    }
    if host["numa_nodes"].as_u64().unwrap_or(1) > 1 {
        flags.push("NUMA_MULTI_NODE".into());
    }
    let usable = host["usable_cpus"].as_u64().unwrap_or(1) as usize;
    if ks.iter().any(|k| *k > usable) {
        flags.push("SCALE_EXCEEDS_USABLE_CPUS".into());
        note(&format!("a scale point exceeds the {usable} usable CPUs ({}); that point will be throttled or oversubscribed", host["usable_cpus_basis"].as_str().unwrap_or("")));
    }
    let re = compile_regex(&a.ops_regex)?;
    let one = |k: usize, events: Option<&str>| -> Res<Value> {
        let mut spec = RunSpec { cmd: &a.cmd, copies: 1, cwd: a.cwd.as_deref(), perf: events.and(perf.as_deref()), events,
                                 ops: a.ops, ops_regex: re.as_ref(), timeout: a.timeout, ..Default::default() };
        if mode == "env" {
            spec.env = vec![(var.clone().unwrap_or_default(), k.to_string())];
        } else {
            spec.copies = k;
            spec.cpus = if a.no_pin { None } else { Some(pick_cpus(k)?) };
        }
        run_once(&spec).map_err(|e| Fail { code: 4, msg: e })
    };
    let core_events = perf.as_ref().map(|_| PMU_CORE);
    let mem_events = perf.as_ref().map(|_| PMU_MEM);
    let (mut runs_a, mut runs_b) = (Vec::new(), Vec::new());
    let mut scaled: Vec<(usize, Vec<Value>)> = ks.iter().filter(|k| **k > 1).map(|k| (*k, Vec::new())).collect();
    if a.warmup {
        one(1, None)?;
    }
    for _ in 0..a.repeat {
        // interleaved: A(1), each k > 1, B(1)
        runs_a.push(one(1, core_events)?);
        for (k, v) in scaled.iter_mut() {
            v.push(one(*k, None)?);
        }
        runs_b.push(one(1, mem_events)?);
    }
    let single: Vec<Value> = runs_a.iter().chain(&runs_b).cloned().collect();
    let allruns: Vec<Value> = single.iter().cloned().chain(scaled.iter().flat_map(|s| s.1.iter().cloned())).collect();
    if allruns.iter().any(|r| r["ops"].is_null()) {
        flags.push("OPS_MISSING".into());
        let tail = allruns.iter().find(|r| r["ops"].is_null()).and_then(|r| r["stderr_tail"].as_str()).unwrap_or("").to_string();
        let err = "no operation count: print an optlevel.ops/v1 line, or pass --ops or --ops-regex";
        let doc = json!({"schema": S::MEASUREMENT_SCHEMA, "tier": tier, "flags": flags, "host": host, "command": a.cmd, "label": a.label,
                         "created_utc": utcnow(), "points": {}, "op_class": a.op_class, "error": err, "stderr_tail": tail});
        write_json(Path::new(&a.out), &doc)?;
        return fail(4, format!("{err}\n{tail}"));
    }
    if allruns.iter().any(any_failed) {
        flags.push("COMMAND_FAILED".into());
    }
    let mut distinct: Vec<i64> = single.iter().filter_map(|r| r["ops"].as_i64()).collect();
    distinct.sort();
    distinct.dedup();
    if distinct.len() > 1 {
        flags.push("OPS_VARY".into()); // legal for time-bounded kernels; recorded
    }
    let ksec = |r: &Value| f(r, "kernel_seconds");
    let wall = |r: &Value| f(r, "wall_s").unwrap_or(f64::NAN);
    if single.iter().map(|r| ksec(r).unwrap_or(wall(r))).fold(f64::INFINITY, f64::min) < S::MIN_KERNEL_SECONDS {
        flags.push("TOO_SHORT".into());
    }
    if single.iter().any(|r| ksec(r).is_none()) {
        flags.push("WALL_INCLUDES_STARTUP".into());
    }
    let startup = stats_opt(single.iter().map(|r| ksec(r).map(|k| 1.0 - k / wall(r))));
    let other = stats_opt(allruns.iter().map(|r| cond_f(r, "other_cpu_cores")));
    if f(&other, "median").unwrap_or(0.0) > S::CONTENDED_CORES {
        flags.push("CONTENDED".into());
    }
    if allruns.iter().any(|r| cond_f(r, "thermal_throttle_events").unwrap_or(0.0) > 0.0) {
        flags.push("THERMAL_THROTTLE".into());
    }
    let op_class = a.op_class.clone().or_else(|| single.iter().find_map(|r| r["op_class"].as_str().filter(|s| !s.is_empty()).map(String::from)));
    let Some(op_class) = op_class else { return fail(2, "no operation class: pass --op-class (one of the calibrated classes)") };

    let point = |rs: &[Value], k: usize| -> Value {
        json!({"threads": k, "mode": if k > 1 { mode.as_str() } else { "single" },
               "ops_per_s": stats_opt(rs.iter().map(|r| f(r, "ops_per_s"))), "ops": stats_opt(rs.iter().map(|r| f(r, "ops"))),
               "wall_s": stats_opt(rs.iter().map(|r| f(r, "wall_s"))), "cpu_s": stats_opt(rs.iter().map(|r| f(r, "cpu_s"))),
               "cpu_utilisation": stats_opt(rs.iter().map(|r| Some(f(r, "cpu_s")? / (wall(r) * k as f64)))),
               "other_cpu_cores": stats_opt(rs.iter().map(|r| cond_f(r, "other_cpu_cores"))),
               "loadavg_1m_before": stats_opt(rs.iter().map(|r| cond_f(r, "loadavg_1m_before")))})
    };
    let mut points = Map::new();
    points.insert("1".into(), point(&single, 1));
    for (k, rs) in &scaled {
        points.insert(k.to_string(), point(rs, *k));
    }
    let mut declared = a.bytes_per_op;
    if declared.is_none() && single.iter().all(|r| !r["bytes_declared"].is_null()) {
        declared = f(&stats_opt(single.iter().map(|r| Some(f(r, "bytes_declared")? / f(r, "ops")?))), "median");
    }
    let rates = |rs: &[Value]| rs.iter().filter_map(|r| f(r, "ops_per_s")).collect::<Vec<_>>();
    let mut doc = json!({
        "schema": S::MEASUREMENT_SCHEMA, "tool_version": S::TOOL_VERSION, "created_utc": utcnow(), "label": a.label,
        "tier": tier, "pmu_unavailable_reason": why, "host": host, "command": a.cmd, "cwd": a.cwd,
        "op_class": op_class, "ops_source": single[0]["ops_source"],
        "scale": {"mode": mode, "env_var": var, "points": ks, "pinned": mode == "copies" && !a.no_pin},
        "repeats": a.repeat, "points": points,
        "noise_floor": S::noise_floor(&rates(&runs_a), &rates(&runs_b)),
        "declared_bytes_per_op": declared, "startup_share": f(&startup, "median"),
        "working_set_bytes": a.working_set_bytes.map(|w| json!(w)).unwrap_or_else(|| single[0].get("working_set_bytes").cloned().unwrap_or(Value::Null)),
        "conditions": {"other_cpu_cores": other, "loadavg_1m": stats_opt(allruns.iter().map(|r| cond_f(r, "loadavg_1m_before"))),
                       "steal_cores": stats_opt(allruns.iter().map(|r| cond_f(r, "steal_cores"))),
                       "psi_cpu_some_avg10": stats_opt(allruns.iter().map(|r| cond_f(r, "psi_cpu_some_avg10_before")))},
        "flags": flags,
        "runs": allruns.iter().map(|r| { let mut r = r.clone(); r.as_object_mut().map(|o| o.remove("stderr_tail")); r }).collect::<Vec<_>>(),
    });
    if perf.is_some() {
        let (pmu, mux) = summarise_pmu(&runs_a, &runs_b);
        if !mux.is_empty() {
            doc["flags"].as_array_mut().unwrap().push(json!("MULTIPLEXED"));
            doc["pmu_multiplexed"] = json!(mux);
        }
        if let Some(m) = f(&pmu, "llc_misses_per_op") {
            if !mux.iter().any(|x| x == "cache-misses") {
                doc["memory_traffic"] = json!({"source": "llc-miss", "bytes_per_op": m * CACHE_LINE,
                    "note": "user-space last-level cache misses x 64 B; a lower bound when the last-level prefetcher hides traffic"});
            }
        }
        doc["pmu"] = pmu;
    }
    write_json(Path::new(&a.out), &doc)?;
    let s = &doc["points"]["1"]["ops_per_s"];
    let fl = strs(&doc["flags"]);
    note(&format!("{tier} {op_class} single {} ops/s [{}, {}] noise {:.3} flags {} -> {}", py_e(f(s, "median").unwrap_or(f64::NAN), 3),
                  py_e(f(s, "min").unwrap_or(f64::NAN), 3), py_e(f(s, "max").unwrap_or(f64::NAN), 3),
                  f(&doc["noise_floor"], "value").unwrap_or(f64::NAN), flags_text(&fl), a.out));
    Ok(())
}
fn strs(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
}

// ---- score ---------------------------------------------------------------------------
/// profile/v1 topdown.json (tools/profile.sh topdown) -> fractions.
pub fn normalise_topdown(doc: &Value) -> Value {
    let nonalnum = Regex::new(r"[^a-z0-9]+").unwrap();
    let prefix = Regex::new(r"^(tma_|td_|topdown_)").unwrap();
    let mut out: Map<String, Value> = Map::new();
    for m in doc.get("metrics").and_then(Value::as_array).into_iter().flatten() {
        let raw = match m.get("metric") {
            Some(Value::String(s)) => s.to_lowercase(),
            Some(Value::Null) | None => String::new(),
            Some(v) => v.to_string().to_lowercase(),
        };
        let name = nonalnum.replace_all(&raw, "_").trim_matches('_').to_string();
        let name = prefix.replace(&name, "").to_string();
        let v = match m.get("value") {
            Some(Value::Number(n)) => n.as_f64(),
            Some(Value::String(s)) => s.trim().parse().ok(),
            _ => None,
        };
        let Some(v) = v else { continue };
        for key in ["frontend_bound", "bad_speculation", "backend_bound", "retiring", "memory_bound", "core_bound"] {
            if name == key || name.ends_with(&format!("_{key}")) || name.starts_with(key) {
                out.insert(key.into(), json!(v));
            }
        }
    }
    if out.values().filter_map(Value::as_f64).fold(f64::NEG_INFINITY, f64::max) > 1.5 {
        // perf prints percentages
        for v in out.values_mut() {
            *v = json!(v.as_f64().unwrap_or(0.0) / 100.0);
        }
    }
    out.insert("multiplexed".into(), doc.get("multiplexed").cloned().unwrap_or(json!([])));
    out.insert("source".into(), json!("tools/profile.sh topdown (profile/v1)"));
    Value::Object(out)
}

/// GPU components from tools/profile.sh output: `ncu` kernels.json (the
/// longest launch) and `nsys` summary.json (host idle share).
pub fn gpu_from_profile(ncu: Option<&Path>, nsys: Option<&Path>) -> Res<Map<String, Value>> {
    let mut g = Map::new();
    if let Some(p) = ncu {
        let d = read_json(p)?;
        let ls = d.get("launches").and_then(Value::as_array).cloned().unwrap_or_default();
        let dur = |x: &Value| f(x, "duration_ns").unwrap_or(0.0);
        let Some(mut top) = ls.first() else { return fail(2, format!("{} holds no ncu launches", p.display())) };
        for l in &ls {
            if dur(l) > dur(top) {
                top = l;
            }
        }
        let mut n: Map<String, Value> = ["kernel", "sm_throughput_pct", "memory_throughput_pct", "dram_throughput_pct", "achieved_occupancy_pct",
                                         "theoretical_occupancy_pct", "issue_active_pct", "registers_per_thread", "stalls_per_issue", "sm_clock_hz"]
            .iter().map(|k| (k.to_string(), top.get(*k).cloned().unwrap_or(Value::Null))).collect();
        n.insert("source".into(), json!("tools/profile.sh ncu (profile/v1 kernels.json), longest launch"));
        g.insert("ncu".into(), Value::Object(n));
    }
    if let Some(p) = nsys {
        let d = read_json(p)?;
        let cov = d.get("timeline").and_then(|t| f(t, "kernel_coverage"));
        g.insert("host_idle_share".into(), json!(cov.map(|c| 1.0 - c)));
        g.insert("nsys_source".into(), json!("tools/profile.sh nsys (summary.json timeline.kernel_coverage)"));
    }
    Ok(g)
}

#[derive(Default)]
pub struct ScoreArgs {
    pub calibration: String,
    pub measurement: String,
    pub out: String,
    pub topdown: Option<String>,
    pub vector_share: Option<f64>,
    pub ncu: Option<String>,
    pub nsys: Option<String>,
    pub counted_ops: Option<f64>,
    pub best_known_ops: Option<f64>,
    pub best_known_source: Option<String>,
    pub repository: Option<String>,
    pub path: Option<String>,
    pub registry_id: Option<String>,
}

pub fn score_documents(calib: &Value, meas: &Value, a: &ScoreArgs) -> Res<Value> {
    if calib["schema"] != S::CALIBRATION_SCHEMA || meas["schema"] != S::MEASUREMENT_SCHEMA {
        return fail(2, "score needs an optlevel.calibration/v1 and an optlevel.measurement/v1 document");
    }
    let mut meas = meas.clone();
    let mut extras = Extras::default();
    let tier = meas["tier"].as_str().unwrap_or("").to_string();
    if let Some(td) = &a.topdown {
        if tier != "T-pmu" {
            return fail(2, "a top-down profile can only be attached to a T-pmu measurement");
        }
        extras.extra_diag.insert("topdown".into(), normalise_topdown(&read_json(Path::new(td))?));
    }
    if let Some(vs) = a.vector_share {
        extras.extra_diag.insert("vector_share_static".into(), json!(vs));
    }
    if a.ncu.is_some() || a.nsys.is_some() {
        if !tier.starts_with("T-gpu") {
            return fail(2, "--ncu/--nsys belong to a GPU measurement");
        }
        let mut g = meas.get("gpu").and_then(Value::as_object).cloned().unwrap_or_default();
        g.extend(gpu_from_profile(a.ncu.as_deref().map(Path::new), a.nsys.as_deref().map(Path::new))?);
        meas["gpu"] = Value::Object(g);
        if a.ncu.is_some() {
            meas["tier"] = json!("T-gpu-ncu");
        }
    }
    if let Some(best) = a.best_known_ops {
        let counted = a.counted_ops.or_else(|| f(&meas["points"]["1"]["ops"], "median")).unwrap_or(f64::NAN);
        extras.algorithmic = Some(json!({"counted_ops": counted, "best_known_ops": best, "ratio": counted / best,
                                         "source": a.best_known_source.clone().unwrap_or_else(|| "unstated".into()),
                                         "note": "separate axis; never folded into the score"}));
    }
    for (k, v) in [("repository", &a.repository), ("path", &a.path), ("registry_id", &a.registry_id)] {
        if let Some(v) = v.as_ref().filter(|v| !v.is_empty()) {
            extras.kernel.insert(k.into(), json!(v));
        }
    }
    let rec = S::build_record(calib, &meas, extras)?;
    let errs = S::validate(&rec, &schema());
    if !errs.is_empty() {
        return fail(2, format!("record violates the schema: {}", errs.iter().take(5).cloned().collect::<Vec<_>>().join("; ")));
    }
    Ok(rec)
}

pub fn cmd_score(a: &ScoreArgs) -> Res<()> {
    let rec = score_documents(&load_json(Path::new(&a.calibration))?, &load_json(Path::new(&a.measurement))?, a)?;
    write_json(Path::new(&a.out), &rec)?;
    println!("{}", summary_text(&rec));
    Ok(())
}

pub fn summary_text(rec: &Value) -> String {
    let (g, att) = (&rec["grade"], &rec["attainment"]);
    let cls = rec["kernel"]["op_class"].as_str().unwrap_or("");
    let fx = |v: &Value, k: &str| f(v, k).unwrap_or(f64::NAN);
    let mut lines = vec![format!("{}  [{}]  class {cls}", rec["label"].as_str().filter(|l| !l.is_empty()).unwrap_or(cls), rec["tier"].as_str().unwrap_or(""))];
    for name in ["single", "full"] {
        if let Some(a) = att.get(name) {
            let r = &rec["roofline"][name];
            lines.push(format!("  attainment {name:6} {:.3} [{:.3}, {:.3}]  of {} ops/s ({} roof, {} thread(s))", fx(a, "value"), fx(a, "low"), fx(a, "high"),
                               py_e(fx(r, "bound_ops_per_s"), 3), r["binding"].as_str().unwrap_or(""), r["threads"]));
        }
    }
    if truthy(rec.get("parallel")) {
        let p = &rec["parallel"];
        lines.push(format!("  parallel   raw {:.3}, roofline-normalised {:.3} at {} threads", fx(p, "raw_efficiency"), fx(p, "roofline_normalised_efficiency"), p["threads"]));
    }
    lines.push(match f(g, "score") {
        Some(s) => format!("  grade      {} (score {s:.3}, scope {})", g["letter_range"].as_str().unwrap_or("None"), g["scope"].as_str().unwrap_or("")),
        None => format!("  grade      none ({})", strs(&g["invalid"]).join("; ")),
    });
    lines.push(format!("  limiting   {}  [{}]", g["limiting_factor"].as_str().unwrap_or(""), g["limiting_basis"].as_str().unwrap_or("")));
    lines.push(if truthy(rec.get("noise_floor")) {
        format!("  noise      A/A {:.3} over {} pairs", fx(&rec["noise_floor"], "value"), rec["noise_floor"]["pairs"])
    } else {
        "  noise      not measured".into()
    });
    let fl = strs(&rec["flags"]);
    lines.push(format!("  flags      {}", flags_text(&fl)));
    lines.push("  This is not a speed claim or a benchmark row (docs/OPTIMIZATION_LEVEL.md).".into());
    lines.join("\n")
}

/// One JSON line with the fields a job log wants from a record
/// (`optlevel summary R.json`; replaces the pods' inline Python).
pub fn cmd_summary(records: &[String]) -> Res<()> {
    for p in records {
        let d = read_json(Path::new(p))?;
        let g = &d["grade"];
        println!("{}", json!({"record": p, "tier": d["tier"], "op_class": d["kernel"]["op_class"], "attainment": d["attainment"],
                              "score": g["score"], "grade": g["letter_range"], "scope": g["scope"],
                              "limiting_factor": g["limiting_factor"], "flags": d["flags"]}));
    }
    Ok(())
}

// ---- static vector share ---------------------------------------------------------------
/// Share of instructions in the named symbols that use packed-vector registers
/// (ymm/zmm, or xmm with a packed mnemonic), from `objdump -d` text.  Static:
/// it counts instructions in the text, not instructions executed.
pub fn vector_share_of(disassembly: &str, symbols: &[String]) -> Option<(Vec<String>, u64, u64)> {
    let head = Regex::new(r"^[0-9a-f]+ <(.+)>:$").unwrap();
    let insn = Regex::new(r"^\s+[0-9a-f]+:\s+\S").unwrap();
    let wide = Regex::new(r"%[yz]mm\d").unwrap();
    let xmm = Regex::new(r"%xmm\d").unwrap();
    let packed = Regex::new(r"^(?:v?p[a-z]+|v?[a-z]+p[sd]$)").unwrap();
    let (mut total, mut vec, mut found, mut cur) = (0u64, 0u64, Vec::<String>::new(), false);
    for line in disassembly.lines() {
        if let Some(m) = head.captures(line) {
            let name = &m[1];
            cur = symbols.iter().any(|s| name == s || name.starts_with(&format!("{s}.")));
            if cur && !found.iter().any(|x| x == name) {
                found.push(name.to_string());
            }
            continue;
        }
        if cur && insn.is_match(line) {
            let ins = line.split_once(':').map(|x| x.1).unwrap_or("").trim();
            let mnem = ins.split_whitespace().next().unwrap_or("");
            if ["nop", "nopw", "nopl", "xchg", "data16", "cs"].contains(&mnem) {
                continue;
            }
            total += 1;
            if wide.is_match(ins) || (xmm.is_match(ins) && packed.is_match(mnem)) {
                vec += 1;
            }
        }
    }
    found.sort();
    (total > 0).then_some((found, total, vec))
}

pub fn vector_share(binary: &str, symbols: &[String]) -> Res<Value> {
    let Some(od) = which("objdump") else { return fail(3, "vector-share needs objdump") };
    let out = Command::new(od).args(["-d", "--no-show-raw-insn", binary]).output().map_err(|e| format!("objdump: {e}"))?;
    let Some((found, total, vec)) = vector_share_of(&String::from_utf8_lossy(&out.stdout), symbols) else {
        return fail(2, format!("none of {symbols:?} found in {binary}"));
    };
    Ok(json!({"binary": binary, "symbols": found, "instructions": total, "vector_instructions": vec,
              "vector_share_static": vec as f64 / total as f64, "kind": "static (instructions in the text, not executed)"}))
}

// ---- controls -----------------------------------------------------------------------------
pub struct ControlsArgs {
    pub out: String,
    pub repeat: usize,
    pub seconds: f64,
    pub cal_seconds: f64,
    pub cal_repeat: usize,
    pub recalibrate: bool,
    pub rescore: bool,
    pub timing_only: bool,
    pub c_kernels: bool,
}

/// Run the synthetic controls end to end in whatever tier the host allows.
pub fn cmd_controls(a: &ControlsArgs) -> Res<()> {
    let out = PathBuf::from(&a.out);
    std::fs::create_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    let cal = out.join("calibration.json");
    if !cal.exists() || a.recalibrate {
        if a.rescore {
            return fail(2, "--rescore needs the calibration.json of the original run");
        }
        cmd_calibrate(&CalibrateArgs { out: cal.to_string_lossy().into_owned(), seconds: a.cal_seconds, repeat: a.cal_repeat, triad_bytes: None, c_kernels: a.c_kernels })?;
    }
    let calib = load_json(&cal)?;
    let big = calib["memory"]["triad_array_bytes"].as_u64().unwrap_or(0);
    let ncpu = host::usable_cpu_count();
    let best = calib["compute"]["modmuladd_p31"]["best_variant"].as_str().and_then(|b| b.split_once('@')).map(|x| x.1.to_string()).unwrap_or_default();
    let sec = ["--seconds".to_string(), a.seconds.to_string()];
    // (name, expectation, build, kernel, extra args, disassemble the pass?)
    let controls: [(&str, &str, &str, &str, Vec<String>, bool); 6] = [
        ("positive-simd-incache", "modular multiply-add row operation, vectorized, in cache (expected: near the compute roof)", &best, "modmuladd_p31", vec![], true),
        ("scalar-incache", "the same loop compiled with vectorization disabled (expected: clearly lower, core bound / not vectorized)", "scalar", "modmuladd_p31", vec![], true),
        ("serial-dependency", "the same arithmetic as a Horner chain (expected: clearly lower, core bound)", &best, "horner_p31", vec![], true),
        ("stream-daxpy", "a <- a*s + b in binary64 over arrays above the last-level cache (expected: on the bandwidth slope, high attainment)", "", "fma_f64", vec!["--n".into(), (big / 8).to_string()], false),
        ("pointer-chase", "dependent loads over a random cycle above the last-level cache (expected: low, memory latency bound)", "", "chase", vec!["--n".into(), (big / 64).to_string()], false),
        ("aa-repeat", "the positive control again, an independent A/A measurement (expected: equal to the first within the noise floor)", &best, "modmuladd_p31", vec![], true),
    ];
    let builds = if a.rescore { None } else { Some(build_kernels(a.c_kernels || best.starts_with("c-"))?) };
    let mut recs = Vec::new();
    for (name, expect, build, kernel, extra, disasm) in controls {
        let m = out.join(format!("{name}.measurement.json"));
        let rec_path = out.join(format!("{name}.record.json"));
        let mut extras = Extras::default();
        extras.kernel = [("repository", "isogeny-labs/crypto"), ("path", "tools/optlevel/src/kernels.rs"), ("expectation", expect)]
            .iter().map(|(k, v)| (k.to_string(), json!(v))).collect();
        if let Some(b) = &builds {
            let build = if build.is_empty() { b.builds[0].0.as_str() } else { build };
            let mut cmd = b.cmd(build, kernel)?;
            cmd.extend(extra);
            cmd.extend(sec.iter().cloned());
            cmd_measure(&MeasureArgs { cmd, scale: (ncpu > 1).then(|| format!("copies:1,{ncpu}")), timing_only: a.timing_only, repeat: a.repeat,
                                       out: m.to_string_lossy().into_owned(), label: name.into(), ..Default::default() })?;
            if disasm {
                let (binary, sym) = b.symbol(build, kernel)?;
                extras.extra_diag.insert("vector_share_static".into(), vector_share(&binary, &[sym])?["vector_share_static"].clone());
            }
        } else {
            // --rescore: the measured binaries may be gone; keep what the original record says about them
            if !m.exists() {
                return fail(2, format!("--rescore: {} is missing", m.display()));
            }
            if let Ok(old) = read_json(&rec_path) {
                if let Some(vs) = old["diagnosis"].get("vector_share_static").filter(|v| !v.is_null()) {
                    extras.extra_diag.insert("vector_share_static".into(), vs.clone());
                }
                for k in ["repository", "path", "expectation"] {
                    if let Some(v) = old["kernel"].get(k) {
                        extras.kernel.insert(k.into(), v.clone());
                    }
                }
            }
        }
        let rec = S::build_record(&calib, &load_json(&m)?, extras)?;
        write_json(&rec_path, &rec)?;
        eprintln!("{}", summary_text(&rec));
        recs.push(rec);
    }
    plot::write_roofline(&calib, &recs, &out.join("controls-roofline.svg"), "optlevel controls")?;
    let table = controls_table(&recs);
    std::fs::write(out.join("controls.md"), format!("{table}\n")).map_err(|e| format!("controls.md: {e}"))?;
    println!("{table}");
    Ok(())
}

pub fn controls_table(recs: &[Value]) -> String {
    let fx = |v: &Value, k: &str| f(v, k).unwrap_or(f64::NAN);
    let opt3 = |v: &Value, k: &str| f(v, k).map(|x| format!("{x:.3}")).unwrap_or_else(|| "n/a".into());
    let mut rows = vec!["| control | tier | class | binding roof | A single [low, high] | A full | raw par. eff. | norm. par. eff. | score | grade | A/A noise | other CPU (cores) | limiting factor |".to_string(),
                        "|:--|:--|:--|:--|:--|--:|--:|--:|--:|:--|--:|--:|:--|".to_string()];
    for r in recs {
        let (s, p, g) = (&r["attainment"]["single"], &r["parallel"], &r["grade"]);
        let fullv = r["attainment"].get("full").map(|x| opt3(x, "value")).unwrap_or_else(|| "n/a".into());
        let score = f(g, "score").map(|x| py_repr(format!("{x:.3}").parse().unwrap_or(x))).unwrap_or_else(|| "None".into());
        rows.push(format!("| {} | {} | {} | {} | {:.3} [{:.3}, {:.3}] | {fullv} | {} | {} | {score} | {} | {:.3} | {:.2} | {} |",
                          r["label"].as_str().unwrap_or(""), r["tier"].as_str().unwrap_or(""), r["kernel"]["op_class"].as_str().unwrap_or(""),
                          r["roofline"]["single"]["binding"].as_str().unwrap_or(""), fx(s, "value"), fx(s, "low"), fx(s, "high"),
                          opt3(p, "raw_efficiency"), opt3(p, "roofline_normalised_efficiency"), g["letter_range"].as_str().unwrap_or("None"),
                          fx(&r["noise_floor"], "value"), fx(&r["conditions"]["other_cpu_cores"], "median"), g["limiting_factor"].as_str().unwrap_or("")));
    }
    rows.join("\n")
}

// ---- GPU path ---------------------------------------------------------------------------------
pub fn gpu_list() -> Result<Vec<String>, String> {
    let Some(smi) = which("nvidia-smi") else { return Err("nvidia-smi is not installed: no NVIDIA GPU is visible to this host".into()) };
    let r = Command::new(smi).args(["--query-gpu=name,memory.total,driver_version,compute_cap", "--format=csv,noheader"]).output();
    let gpus: Vec<String> = r.as_ref().ok().map(|o| String::from_utf8_lossy(&o.stdout).lines().map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()).unwrap_or_default();
    if !r.map(|o| o.status.success()).unwrap_or(false) || gpus.is_empty() {
        return Err("nvidia-smi lists no GPU".into());
    }
    Ok(gpus)
}

/// Rows of `--query-gpu=utilization.gpu,utilization.memory,power.draw,temperature.gpu,clocks_throttle_reasons.active
/// --format=csv,noheader,nounits`.
pub fn parse_smi_samples(text: &str) -> Value {
    let (mut util, mut mem, mut power, mut temp, mut thr) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), 0u64);
    for line in text.lines() {
        let fl: Vec<&str> = line.split(',').map(str::trim).collect();
        if fl.len() < 5 {
            continue;
        }
        let v: Vec<f64> = fl[..4].iter().filter_map(|x| x.parse().ok()).collect();
        if v.len() < 4 {
            continue;
        }
        util.push(v[0]);
        mem.push(v[1]);
        power.push(v[2]);
        temp.push(v[3]);
        // 0x1 = GPU idle
        if !["0x0000000000000000", "0x0000000000000001", "[N/A]", "N/A", ""].contains(&fl[4]) {
            thr += 1;
        }
    }
    let n = util.len();
    json!({"samples": n, "utilization_gpu_pct": stats(&util), "utilization_memory_pct": stats(&mem), "power_w": stats(&power),
           "temperature_c": stats(&temp), "throttled_sample_share": if n > 0 { json!(thr as f64 / n as f64) } else { Value::Null },
           "note": "nvidia-smi utilisation is the share of samples with a kernel resident; it is not SM activity"})
}

/// What an `ncu` run's output says about the hardware counters.
/// Returns (status, text): status is "available", "withheld" or "failed".
pub fn classify_ncu_output(text: &str, exit_ok: bool) -> (&'static str, String) {
    if text.contains("ERR_NVGPUCTRPERM") {
        return ("withheld", "counters withheld by the driver (ERR_NVGPUCTRPERM): ncu ran but this user may not read GPU performance counters; \
                 it needs NVreg_RestrictProfilingToAdminUsers=0 on the host or the provider's profiling capability, neither of which a container can set".into());
    }
    if exit_ok && !text.contains("==ERROR==") {
        return ("available", "ncu profiled the control kernel".into());
    }
    let last = text.lines().rev().find(|l| l.contains("==ERROR==") || !l.trim().is_empty()).unwrap_or("no output").trim().to_string();
    ("failed", format!("ncu did not profile the control kernel: {last}"))
}

fn build_gpu_calib() -> Res<(PathBuf, String, Vec<&'static str>)> {
    let Some(nvcc) = which("nvcc") else { return fail(3, "this needs nvcc to build kernels/gpu_calib.cu") };
    let flags = vec!["-O3", "-std=c++17"];
    let dir = cache_dir()?;
    let tag = &sha256_hex(GPU_CALIB_CU.as_bytes())[..12];
    let (src, exe) = (dir.join(format!("gpu_calib-{tag}.cu")), dir.join(format!("gpu_calib-{tag}")));
    if !exe.exists() {
        std::fs::write(&src, GPU_CALIB_CU).map_err(|e| format!("{}: {e}", src.display()))?;
        let r = Command::new(&nvcc).args(&flags).arg("-o").arg(&exe).arg(&src).output().map_err(|e| format!("nvcc: {e}"))?;
        if !r.status.success() {
            return fail(4, format!("nvcc failed: {}", String::from_utf8_lossy(&r.stderr)));
        }
    }
    Ok((exe, nvcc, flags))
}

/// Everything `gpu probe` reports beyond the device list: tools, missing
/// helpers, and whether ncu can read the hardware counters here.
pub fn gpu_probe_doc(gpus: &[String], ncu_check: bool) -> Value {
    let (nvcc, ncu, nsys) = (which("nvcc"), which("ncu"), which("nsys"));
    let helpers = host::helper_tools();
    let mut notes: Vec<String> = Vec::new();
    let missing = strs(&helpers["missing"]);
    if !missing.is_empty() {
        notes.push(format!("missing helper tools: {} (tools/profile.sh ncu and nsys exit 2 without them; install them before profiling, e.g. apt-get install -y {})",
                           missing.join(", "), missing.join(" ")));
    }
    let (status, detail): (&str, String) = match (&ncu, ncu_check) {
        (None, _) => ("absent", "ncu is not installed".into()),
        (Some(_), false) => ("not tested", "skipped by --no-ncu-check".into()),
        (Some(ncu), true) => match build_gpu_calib() {
            Err(e) => ("not tested", format!("the control kernel could not be built: {}", e.msg.trim())),
            Ok((exe, ..)) => match Command::new(ncu).args(["--target-processes", "all", "--launch-count", "1"]).arg(&exe).arg("muladd_u32").output() {
                Err(e) => ("failed", format!("ncu could not be run: {e}")),
                Ok(o) => {
                    let text = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
                    let (s, d) = classify_ncu_output(&text, o.status.success());
                    (s, d)
                }
            },
        },
    };
    let tier = if status == "available" { "T-gpu-ncu" } else { "T-gpu-timing" };
    if status != "available" {
        notes.push(format!("ncu counters: {detail}; records on this host are tier T-gpu-timing"));
    }
    json!({"gpus": gpus, "nvcc": nvcc, "ncu": ncu, "nsys": nsys, "helper_tools": helpers,
           "ncu_counters": {"status": status, "detail": detail}, "gpu_tier": tier, "notes": notes})
}

pub struct GpuCalibrateArgs {
    pub out: String,
    pub repeat: usize,
}
fn gpu_host(gpus: &[String]) -> Value {
    let mut host = host_info();
    let fp = sha256_hex(format!("{}{}", host["fingerprint"].as_str().unwrap_or(""), gpus.join("|")).as_bytes())[..16].to_string();
    host["gpus"] = json!(gpus);
    host["fingerprint"] = json!(fp);
    host
}
/// Refuse (exit 3) on a host without a GPU, writing the reason to `out`.
fn require_gpu(out: Option<&str>) -> Res<Vec<String>> {
    match gpu_list() {
        Ok(g) => Ok(g),
        Err(why) => {
            let doc = json!({"schema": "optlevel.unsupported/v1", "status": "unsupported", "reason": why, "host": host_info(),
                             "helper_tools": host::helper_tools()});
            if let Some(o) = out {
                write_json(Path::new(o), &doc)?;
            }
            println!("{}", Value::String(why.clone()));
            fail(3, format!("GPU path unsupported on this host: {why}"))
        }
    }
}

pub fn cmd_gpu_probe(ncu_check: bool) -> Res<()> {
    let gpus = require_gpu(None)?;
    let doc = gpu_probe_doc(&gpus, ncu_check);
    for n in strs(&doc["notes"]) {
        note(&n);
    }
    print!("{}", to_pretty(&doc));
    Ok(())
}

/// Calibration document from the `optlevel.ops/v1` lines of gpu_calib runs.
pub fn gpu_calibration_doc(lines: &[Value], host: Value, build: Value) -> Value {
    let mut per: Vec<(String, Vec<f64>)> = Vec::new();
    let mut bw = Vec::new();
    for d in lines {
        let (Some(ops), Some(secs), Some(cls)) = (f(d, "ops"), f(d, "kernel_seconds"), d["op_class"].as_str()) else { continue };
        if cls == "triad" {
            bw.push(f(d, "bytes_declared").unwrap_or(f64::NAN) / secs);
        } else {
            match per.iter_mut().find(|x| x.0 == cls) {
                Some(x) => x.1.push(ops / secs),
                None => per.push((cls.to_string(), vec![ops / secs])),
            }
        }
    }
    let mut compute: Map<String, Value> = per.iter().map(|(c, v)| {
        (c.clone(), json!({"definition": if class_doc(c).is_empty() { c.as_str() } else { class_doc(c) }, "unit": "operations per second",
                           "best_variant": "gpu_calib.cu", "variants": {}, "total_ops_per_s": {"1": stats(v)},
                           "scalar_ops_per_s": null, "simd_ops_per_s": null}))
    }).collect();
    // the memory-bound class: a streamed triad is scored against the fma_f64
    // compute ceiling and the device bandwidth at its declared 24 B/element
    if let (Some(fma), false) = (compute.get("fma_f64").cloned(), bw.is_empty()) {
        let mut t = fma;
        t["definition"] = json!(class_doc("triad"));
        t["alias_of"] = json!("fma_f64");
        t["best_variant"] = json!("gpu_calib.cu (fma_f64 stream)");
        compute.insert("triad".into(), t);
    }
    json!({"schema": S::CALIBRATION_SCHEMA, "tool_version": S::TOOL_VERSION, "created_utc": utcnow(), "device": "gpu", "host": host, "build": build,
           "ceiling_rule": "max over repeats of the device rate with every SM occupied; threads '1' means one whole device",
           "compute": compute,
           "memory": {"bandwidth_bytes_per_s": {"1": stats(&bw)}, "latency_ns": null, "bytes_convention": "device triad, 24 bytes per element"},
           "conditions": null, "flags": ["GPU_CEILING_UNVALIDATED_BY_AUTHOR"]})
}

pub fn cmd_gpu_calibrate(a: &GpuCalibrateArgs) -> Res<()> {
    let gpus = require_gpu(Some(&a.out))?;
    let (exe, nvcc, flags) = build_gpu_calib()?;
    let mut lines = Vec::new();
    for _ in 0..a.repeat {
        let out = Command::new(&exe).output().map_err(|e| format!("{}: {e}", exe.display()))?;
        if !out.status.success() {
            return fail(4, format!("gpu_calib failed (its host re-check of results is part of the run): {}{}",
                                   String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)));
        }
        for line in String::from_utf8_lossy(&out.stdout).lines().filter(|l| l.contains(S::OPS_SCHEMA)) {
            if let Ok(d) = serde_json::from_str::<Value>(line) {
                lines.push(d);
            }
        }
    }
    let build = json!({"compiler": nvcc, "flags": flags, "source_sha256": sha256_hex(GPU_CALIB_CU.as_bytes())});
    write_json(Path::new(&a.out), &gpu_calibration_doc(&lines, gpu_host(&gpus), build))?;
    Ok(())
}

/// Measurement document of a GPU kernel from its interleaved A/B runs.
pub fn gpu_measurement_doc(a: &MeasureArgs, runs_a: &[Value], runs_b: &[Value], samples: Value, host: Value) -> Res<Value> {
    let allr: Vec<&Value> = runs_a.iter().chain(runs_b).collect();
    if allr.iter().any(|r| r["ops"].is_null()) {
        return fail(4, "the command reported no operation count (optlevel.ops/v1 line, --ops or --ops-regex)");
    }
    let mut flags: Vec<&str> = Vec::new();
    if f(&samples, "throttled_sample_share").unwrap_or(0.0) != 0.0 {
        flags.push("GPU_THROTTLED");
    }
    let wall = |r: &Value| f(r, "wall_s").unwrap_or(f64::NAN);
    if allr.iter().map(|r| f(r, "kernel_seconds").filter(|k| *k != 0.0).unwrap_or(wall(r))).fold(f64::INFINITY, f64::min) < S::MIN_KERNEL_SECONDS {
        flags.push("TOO_SHORT");
    }
    if allr.iter().any(|r| r["kernel_seconds"].is_null()) {
        flags.push("WALL_INCLUDES_STARTUP");
    }
    if allr.iter().any(|r| any_failed(r)) {
        flags.push("COMMAND_FAILED");
    }
    // memory traffic: --bytes-per-op, else what the kernel's own ops lines declare (as on CPU)
    let mut declared = a.bytes_per_op;
    if declared.is_none() && allr.iter().all(|r| !r["bytes_declared"].is_null()) {
        declared = f(&stats_opt(allr.iter().map(|r| Some(f(r, "bytes_declared")? / f(r, "ops")?))), "median");
    }
    let rates = |rs: &[Value]| rs.iter().filter_map(|r| f(r, "ops_per_s")).collect::<Vec<_>>();
    let st = |k: &str| stats_opt(allr.iter().map(|r| f(r, k)));
    Ok(json!({
        "schema": S::MEASUREMENT_SCHEMA, "tool_version": S::TOOL_VERSION, "created_utc": utcnow(), "label": a.label, "tier": "T-gpu-timing",
        "host": host, "command": a.cmd,
        "op_class": a.op_class.clone().map(Value::String).unwrap_or_else(|| allr[0]["op_class"].clone()),
        "ops_source": allr[0]["ops_source"], "repeats": a.repeat,
        "points": {"1": {"threads": 1, "mode": "device", "ops_per_s": st("ops_per_s"), "ops": st("ops"), "wall_s": st("wall_s")}},
        "noise_floor": S::noise_floor(&rates(runs_a), &rates(runs_b)),
        "declared_bytes_per_op": declared, "startup_share": null,
        "working_set_bytes": a.working_set_bytes.map(|w| json!(w)).unwrap_or_else(|| allr[0].get("working_set_bytes").cloned().unwrap_or(Value::Null)),
        "gpu": {"nvidia_smi": samples, "host_cpu_utilisation": stats_opt(allr.iter().map(|r| Some(f(r, "cpu_s")? / wall(r))))},
        "conditions": {"other_cpu_cores": stats_opt(allr.iter().map(|r| cond_f(r, "other_cpu_cores")))},
        "flags": flags,
    }))
}

pub fn cmd_gpu_measure(a: &MeasureArgs) -> Res<()> {
    let gpus = require_gpu(Some(&a.out))?;
    if a.cmd.is_empty() {
        return fail(2, "no command (put it after --)");
    }
    let re = compile_regex(&a.ops_regex)?;
    let log = std::env::temp_dir().join(format!("optlevel-smi-{}.csv", std::process::id()));
    let mut smi = Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu,utilization.memory,power.draw,temperature.gpu,clocks_throttle_reasons.active", "--format=csv,noheader,nounits", "-lms", "200"])
        .stdin(Stdio::null()).stdout(std::fs::File::create(&log).map_err(|e| format!("{}: {e}", log.display()))?).stderr(Stdio::null())
        .spawn().map_err(|e| format!("nvidia-smi: {e}"))?;
    let (mut runs_a, mut runs_b) = (Vec::new(), Vec::new());
    let mut err = None;
    'outer: for _ in 0..a.repeat {
        for slot in [&mut runs_a, &mut runs_b] {
            match run_once(&RunSpec { cmd: &a.cmd, copies: 1, ops: a.ops, ops_regex: re.as_ref(), cwd: a.cwd.as_deref(), ..Default::default() }) {
                Ok(r) => slot.push(r),
                Err(e) => {
                    err = Some(e);
                    break 'outer;
                }
            }
        }
    }
    // SAFETY: smi is our own child and has not been reaped.
    unsafe { libc::kill(smi.id() as i32, libc::SIGTERM) };
    let _ = smi.wait();
    let samples = parse_smi_samples(&std::fs::read_to_string(&log).unwrap_or_default());
    let _ = std::fs::remove_file(&log);
    if let Some(e) = err {
        return fail(4, e);
    }
    write_json(Path::new(&a.out), &gpu_measurement_doc(a, &runs_a, &runs_b, samples, gpu_host(&gpus))?)?;
    Ok(())
}

// ---- small commands ---------------------------------------------------------------------------------
pub fn cmd_probe() -> Res<()> {
    let host = host_info();
    let (perf, why) = match host::pmu_probe() {
        Ok(p) => (Some(p), None),
        Err(e) => (None, Some(e)),
    };
    let (gpus, gwhy) = match gpu_list() {
        Ok(g) => (Some(g), None),
        Err(e) => (None, Some(e)),
    };
    let imc: Vec<String> = strs(&host["event_sources"]).into_iter().filter(|s| s.starts_with("uncore_imc")).collect();
    let helpers = host::helper_tools();
    for m in strs(&helpers["missing"]) {
        note(&format!("helper tool {m} is missing (tools/profile.sh needs it for ncu and nsys)"));
    }
    print!("{}", to_pretty(&json!({"host": host, "cpu_tier": if perf.is_some() { "T-pmu" } else { "T-timing" }, "perf": perf,
                                    "pmu_unavailable_reason": why, "gpu": gpus, "gpu_unavailable_reason": gwhy, "ncu": which("ncu"),
                                    "uncore_imc": imc, "helper_tools": helpers, "kernel_isas": kernels::available_isas()})));
    Ok(())
}

/// Schema and tier errors of one record.
pub fn record_errors(rec: &Value) -> Vec<String> {
    let mut errs = S::validate(rec, &schema());
    if let Err(e) = S::enforce_tier(rec) {
        errs.push(e);
    }
    errs
}

pub fn cmd_validate(records: &[String]) -> Res<()> {
    let mut bad = 0;
    for p in records {
        let errs = match read_json(Path::new(p)) {
            Ok(rec) => record_errors(&rec),
            Err(e) => vec![e],
        };
        bad += !errs.is_empty() as i32;
        println!("{p}: {}", if errs.is_empty() { "ok".to_string() } else { errs.join("; ") });
    }
    if bad > 0 { fail(1, String::new()) } else { Ok(()) }
}

/// The record's values as `tools/benchrec.py metric` argument lines.
pub fn benchrec_lines(rec: &Value) -> Vec<String> {
    let num = |v: &Value| match v {
        Value::Number(n) if n.is_f64() => py_repr(n.as_f64().unwrap_or(f64::NAN)),
        other => other.to_string(),
    };
    S::benchrec_metrics(rec).iter().map(|m| {
        let mut args: Vec<String> = vec!["--name".into(), m["name"].as_str().unwrap_or("").into(), "--value".into(), num(&m["value"]),
                                         "--unit".into(), m["unit"].as_str().unwrap_or("").into(), "--better".into(), m["better"].as_str().unwrap_or("").into(),
                                         "--reference-kind".into(), "none".into(), "--note".into(),
                                         format!("optlevel {}; not a speed claim", rec["tier"].as_str().unwrap_or(""))];
        for (k, flag) in [("n", "--n"), ("spread_low", "--spread-low"), ("spread_high", "--spread-high")] {
            if let Some(v) = m.get(k).filter(|v| !v.is_null()) {
                args.push(flag.into());
                args.push(num(v));
            }
        }
        format!("python3 tools/benchrec.py metric {} <record>", args.iter().map(|x| sh_quote(x)).collect::<Vec<_>>().join(" "))
    }).collect()
}

pub fn cmd_benchrec(record: &str) -> Res<()> {
    for line in benchrec_lines(&read_json(Path::new(record))?) {
        println!("{line}");
    }
    Ok(())
}

pub fn cmd_plot(calibration: &str, out: &str, title: &str, records: &[String]) -> Res<()> {
    let recs: Vec<Value> = records.iter().map(|r| read_json(Path::new(r))).collect::<Result<_, _>>()?;
    plot::write_roofline(&load_json(Path::new(calibration))?, &recs, Path::new(out), title)?;
    Ok(())
}

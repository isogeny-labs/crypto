//! CLI, target generation and the per-(target, arm) runner.

use crate::curve::{mix, Curve, Oracle, Pt, Rng};
use crate::enc::{build, Arm, Cell};
use crate::gf::rot;
use crate::satlib::SolveResult;
use std::collections::HashSet;
use std::io::Write;
use std::time::{Duration, Instant};

pub const MASTER_SEED: u64 = 2026_1027_03;

pub const HEADER: &str = "cell\tn\tl\tm\tvsel\tcurve_a\tb\tround\ttidx\tkind\txr\ttruth\tarm\tslot\tseed\tresult\tverdict_ok\tcert_ok\tconflicts\tdecisions\tpropagations\trestarts\txor_props\txor_conflicts\tsearch_ops\tencode_ops\ttotal_ops\twall_ns\tbuild_ns\tnv\tclauses\txor_rows\txor_lits\tand_aux\tcomparators\trss_kb";

#[derive(Clone, Copy, Debug)]
pub struct ArmTok {
    pub arm: Arm,
    pub slot: u32,
}

pub fn parse_arms(s: &str) -> Result<Vec<ArmTok>, String> {
    s.split(',')
        .filter(|t| !t.is_empty())
        .map(|t| {
            let (name, slot) = match t.split_once('@') {
                Some((a, b)) => (a, b.parse::<u32>().map_err(|e| e.to_string())?),
                None => (t, 1),
            };
            Arm::parse(name)
                .map(|arm| ArmTok { arm, slot })
                .ok_or_else(|| format!("unknown arm {name}"))
        })
        .collect()
}

pub struct Target {
    pub xr: u64,
    pub truth_sat: bool,
    pub kind: &'static str, // unsat | sat_incidental | planted
}

fn orbit_key(cx: &Cell, x: u64) -> u64 {
    let c = cx.nb.coords(x);
    (0..cx.n).map(|k| rot(c, k, cx.n)).min().unwrap()
}

/// Targets: distinct sigma-orbits (conjugates are the same problem, so they
/// would be pseudo-replicates), E_a-abscissas, labelled by the exact oracle.
pub fn gen_targets(
    cx: &Cell,
    o: &mut Oracle,
    curve_a: u64,
    want_unsat: usize,
    keep_sat: usize,
    planted: usize,
    seed: u64,
) -> Vec<Target> {
    let curve = Curve {
        f: &cx.f,
        a: curve_a,
        b: cx.b,
    };
    let mut rng = Rng(seed);
    let mut seen: HashSet<u64> = HashSet::new();
    let mut out = vec![];
    let (mut nu, mut ns) = (0, 0);
    let mut guard = 0u64;
    while nu < want_unsat {
        guard += 1;
        assert!(guard < 50_000_000, "target generation did not terminate");
        let x = 1 + rng.below((1u64 << cx.n) - 1);
        if !curve.is_abscissa(x) || !seen.insert(orbit_key(cx, x)) {
            continue;
        }
        let sat = o.exists(cx.m, x);
        if !sat && nu < want_unsat {
            out.push(Target {
                xr: x,
                truth_sat: false,
                kind: "unsat",
            });
            nu += 1;
        } else if sat && ns < keep_sat {
            out.push(Target {
                xr: x,
                truth_sat: true,
                kind: "sat_incidental",
            });
            ns += 1;
        }
    }
    // planted: x_i in V with E-lifts, xR = x(sum of lifts); the oracle must agree
    let mut np = 0;
    guard = 0;
    while np < planted {
        guard += 1;
        assert!(
            guard < 5_000_000,
            "planting found no unused sigma-orbit: cell too small for this many distinct targets"
        );
        let mut sum = Pt::Inf;
        let mut ok = true;
        for _ in 0..cx.m {
            let x = o.velems[rng.below(o.velems.len() as u64) as usize];
            match curve.lift(x) {
                Some(y) => {
                    let y = if rng.next() & 1 == 1 { y ^ x } else { y };
                    sum = curve.add(sum, Pt::Aff(x, y));
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        if let Pt::Aff(xr, _) = sum {
            if xr != 0 && seen.insert(orbit_key(cx, xr)) {
                assert!(o.exists(cx.m, xr), "planted target must be SAT");
                out.push(Target {
                    xr,
                    truth_sat: true,
                    kind: "planted",
                });
                np += 1;
            }
        }
    }
    out
}

fn rss_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines().find(|l| l.starts_with("VmHWM:")).map(|l| {
                l.split_whitespace()
                    .nth(1)
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0)
            })
        })
        .unwrap_or(0)
}

pub struct RunParams {
    pub round: u32,
    pub conflict_cap: u64,
    pub deadline_s: u64,
}

pub fn run_one(
    cx: &Cell,
    curve_a: u64,
    vsel: usize,
    t: &Target,
    tidx: usize,
    tok: ArmTok,
    p: &RunParams,
) -> String {
    let seed = mix(
        mix(
            MASTER_SEED,
            ((cx.n as u64) << 32) | ((cx.l as u64) << 16) | cx.m as u64,
        ),
        mix(
            mix(tidx as u64, p.round as u64),
            (tok.arm as u64) * 16 + tok.slot as u64,
        ),
    );
    let t0 = Instant::now();
    let built = build(tok.arm, cx, t.xr, true);
    let (mut s, pi) = built.load(seed);
    s.scramble_saved_phases(seed ^ 0xA5A5);
    s.conflict_budget = p.conflict_cap;
    s.deadline = Some(Instant::now() + Duration::from_secs(p.deadline_s));
    let build_ns = t0.elapsed().as_nanos() as u64;
    let t1 = Instant::now();
    let res = s.solve();
    let wall = t1.elapsed().as_nanos() as u64;
    let (rs, cert_ok, verdict_ok) = match res {
        SolveResult::Sat => {
            let model = s.model();
            let v = built.verify(cx, &model, &pi, t.xr);
            ("SAT", if v.is_ok() { "1" } else { "0" }, t.truth_sat)
        }
        SolveResult::Unsat => ("UNSAT", "NA", !t.truth_sat),
        SolveResult::Unknown => ("UNKNOWN", "NA", true),
    };
    let st = &s.stats;
    let total = s.word_ops();
    let sz = built.cnf.sizes();
    format!(
        "{n}_{l}_{m}_{vsel}{bsuf}\t{n}\t{l}\t{m}\t{vsel}\t{a}\t{b}\t{round}\t{tidx}\t{kind}\t{xr}\t{truth}\t{arm}\t{slot}\t{seed}\t{rs}\t{vok}\t{cok}\t{conf}\t{dec}\t{prop}\t{rst}\t{xp}\t{xc}\t{sops}\t{eops}\t{tops}\t{wall}\t{bns}\t{nv}\t{ncl}\t{nx}\t{xl}\t{na}\t{ncmp}\t{rss}",
        bsuf = if cx.b == 1 { String::new() } else { format!("_b{}", cx.b) },
        n = cx.n,
        l = cx.l,
        m = cx.m,
        vsel = vsel,
        a = curve_a,
        b = cx.b,
        round = p.round,
        tidx = tidx,
        kind = t.kind,
        xr = t.xr,
        truth = if t.truth_sat { "SAT" } else { "UNSAT" },
        arm = tok.arm.name(),
        slot = tok.slot,
        seed = seed,
        rs = rs,
        vok = verdict_ok as u8,
        cok = cert_ok,
        conf = s.conflicts(),
        dec = st.decisions,
        prop = st.propagations,
        rst = st.restarts,
        xp = st.xor_propagations,
        xc = st.xor_conflicts,
        sops = total - st.encode_word_ops,
        eops = st.encode_word_ops,
        tops = total,
        wall = wall,
        bns = build_ns,
        nv = sz.vars,
        ncl = sz.clauses,
        nx = sz.xor_rows,
        xl = sz.xor_lits,
        na = sz.and_aux,
        ncmp = built.n_comparators,
        rss = rss_kb(),
    )
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn flag_u<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    flag(args, name)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

pub fn cli(args: &[String]) -> i32 {
    match args.first().map(|s| s.as_str()) {
        Some("run") => cmd_run(&args[1..]),
        Some("sizes") => cmd_sizes(&args[1..]),
        Some("targets") => cmd_targets(&args[1..]),
        Some("summarize") => crate::stats::cmd_summarize(&args[1..]),
        Some("gate") => crate::stats::cmd_gate(&args[1..]),
        Some("manifest") => cmd_manifest(),
        _ => {
            eprintln!(
                "g3census <run|sizes|targets|summarize|manifest> ...\n\
                 run --n N --l L --m M [--vsel 0] [--curve-a 0] [--b 1] --unsat K [--sat-keep S] [--planted P] \
                 --arms C,Cpoly,A,B,... [--planted-only-arms U] [--unsat-only-arms Chalf] [--tag T] [--round 1] [--conflicts CAP] [--deadline-s S] [--seed-offset 0] --out DIR\n\
                 sizes --n N --l L --m M [--vsel 0]   (encoding size table, no solving)\n\
                 targets --n N --l L --m M ...         (exact-oracle census of the target pool, no solving)\n\
                 summarize DIR [DIR...] --out OUT\n\
                 gate DIR [DIR...]                     (pilot read-out: conflicts and ns per conflict per cell and arm)\n\
                 manifest                               (hashes of the sources and of the repo solver)"
            );
            2
        }
    }
}

fn cell_from(args: &[String]) -> (Cell, usize) {
    let n: u32 = flag_u(args, "--n", 17);
    let l: u32 = flag_u(args, "--l", 8);
    let m: usize = flag_u(args, "--m", 2);
    let vsel: usize = flag_u(args, "--vsel", 0);
    let b: u64 = flag_u(args, "--b", 1);
    (Cell::new(n, l, m, vsel, b), vsel)
}

fn cmd_run(args: &[String]) -> i32 {
    let (cx, vsel) = cell_from(args);
    let curve_a: u64 = flag_u(args, "--curve-a", 0);
    let want: usize = flag_u(args, "--unsat", 25);
    let keep: usize = flag_u(args, "--sat-keep", 5);
    let planted: usize = flag_u(args, "--planted", 10);
    let arms = match parse_arms(flag(args, "--arms").unwrap_or("C,A,B")) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let only_planted = match parse_arms(flag(args, "--planted-only-arms").unwrap_or("")) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let unsat_only = match parse_arms(flag(args, "--unsat-only-arms").unwrap_or("")) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let tag = flag(args, "--tag").unwrap_or("");
    let out = flag(args, "--out").expect("--out DIR required");
    let params = RunParams {
        round: flag_u(args, "--round", 1),
        conflict_cap: flag_u(args, "--conflicts", 20_000_000),
        deadline_s: flag_u(args, "--deadline-s", 900),
    };
    let seed_off: u64 = flag_u(args, "--seed-offset", 0);
    std::fs::create_dir_all(out).expect("mkdir out");
    let file = format!(
        "{out}/rows_n{}_l{}_m{}_v{}_b{}_a{}_r{}{}.tsv",
        cx.n,
        cx.l,
        cx.m,
        vsel,
        cx.b,
        curve_a,
        params.round,
        if tag.is_empty() {
            String::new()
        } else {
            format!("_{tag}")
        }
    );
    if std::path::Path::new(&file).exists() {
        eprintln!("refusing to overwrite {file}");
        return 3;
    }
    let mut o = Oracle::new(&cx.f, &cx.nb, &cx.vb_n, cx.g, cx.b);
    let tseed = mix(
        MASTER_SEED ^ seed_off,
        ((cx.n as u64) << 40)
            | ((cx.l as u64) << 32)
            | ((cx.m as u64) << 24)
            | (vsel as u64) << 16
            | curve_a,
    );
    let t0 = Instant::now();
    let targets = gen_targets(&cx, &mut o, curve_a, want, keep, planted, tseed);
    eprintln!(
        "targets: {} generated in {:.1}s",
        targets.len(),
        t0.elapsed().as_secs_f64()
    );
    let mut w = std::io::BufWriter::new(std::fs::File::create(&file).expect("create rows"));
    writeln!(w, "{HEADER}").unwrap();
    for (ti, t) in targets.iter().enumerate() {
        // interleave: arm order shuffled per target from the master seed
        let mut todo: Vec<ArmTok> = arms.clone();
        if t.kind == "planted" {
            todo.extend(only_planted.iter().copied());
        }
        if t.kind == "unsat" {
            todo.extend(unsat_only.iter().copied());
        }
        let mut order: Vec<usize> = (0..todo.len()).collect();
        let mut rng = Rng(mix(tseed, ti as u64 + 1));
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i as u64 + 1) as usize);
        }
        for &ai in &order {
            let row = run_one(&cx, curve_a, vsel, t, ti, todo[ai], &params);
            writeln!(w, "{row}").unwrap();
        }
        w.flush().unwrap();
    }
    0
}

fn cmd_sizes(args: &[String]) -> i32 {
    let (cx, _) = cell_from(args);
    let xr = 3 | 1 << (cx.n - 2);
    println!("arm\tn\tl\tm\tvars\tclauses\tclause_lits\txor_rows\txor_lits\tand_aux\tcomparators");
    for arm in [
        Arm::C,
        Arm::CPoly,
        Arm::CNoBreak,
        Arm::A,
        Arm::B,
        Arm::BNoGauge,
        Arm::G,
        Arm::U,
        Arm::CHalf,
    ] {
        let b = build(arm, &cx, xr, true);
        let s = b.cnf.sizes();
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            arm.name(),
            cx.n,
            cx.l,
            cx.m,
            s.vars,
            s.clauses,
            s.clause_lits,
            s.xor_rows,
            s.xor_lits,
            s.and_aux,
            b.n_comparators
        );
    }
    0
}

fn cmd_targets(args: &[String]) -> i32 {
    let (cx, _) = cell_from(args);
    let mut o = Oracle::new(&cx.f, &cx.nb, &cx.vb_n, cx.g, cx.b);
    let want: usize = flag_u(args, "--unsat", 25);
    let a: u64 = flag_u(args, "--curve-a", 0);
    let t0 = Instant::now();
    let t = gen_targets(&cx, &mut o, a, want, 1000, 0, MASTER_SEED);
    let su = t.iter().filter(|x| !x.truth_sat).count();
    println!(
        "n={} l={} m={} unsat={} sat_incidental={} secs={:.1}",
        cx.n,
        cx.l,
        cx.m,
        su,
        t.len() - su,
        t0.elapsed().as_secs_f64()
    );
    0
}

/// FNV-1a 64 of a byte slice (provenance of sources; not a security claim).
pub fn fnv(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn cmd_manifest() -> i32 {
    let sat = include_bytes!("../../../../src/cryptanalysis/sat.rs");
    println!("master_seed\t{MASTER_SEED}");
    println!(
        "repo_solver_sat_rs_fnv64\t{:016x}\tlen={}",
        fnv(sat),
        sat.len()
    );
    println!("enc_rs_fnv64\t{:016x}", fnv(include_bytes!("enc.rs")));
    println!("gf_rs_fnv64\t{:016x}", fnv(include_bytes!("gf.rs")));
    println!("curve_rs_fnv64\t{:016x}", fnv(include_bytes!("curve.rs")));
    println!("run_rs_fnv64\t{:016x}", fnv(include_bytes!("run.rs")));
    println!("stats_rs_fnv64\t{:016x}", fnv(include_bytes!("stats.rs")));
    0
}

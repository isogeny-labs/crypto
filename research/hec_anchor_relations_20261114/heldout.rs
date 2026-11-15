//! Frozen held-out screen. See HELDOUT-PROTOCOL.md for cohort, caps and scope.
mod anchor;
mod linear;
#[path = "../hec_table_oracle_20261114/pipeline.rs"]
mod pipeline;
mod rational;
mod rational_solve;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    fp::Fp,
    ic::{self, IcParams},
    instance::{factor_base, hasse_interval, order_bsgs, prime_factors},
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
    solve::Solver,
};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
const BUILD: &str = match option_env!("HEC_SOURCE") {
    Some(s) => s,
    None => "UNFROZEN",
};
const ARMS: [&str; 15] = [
    "n1fr", "n1fd", "n1hr", "n1hd", "n64fr", "n64fd", "n64hr", "n64hd", "t1f", "t1h", "t64f",
    "t64h", "orbit", "compact", "t1faa",
];

fn cohort() -> Vec<(usize, u64, u64)> {
    let mut out = Vec::new();
    for (g, ps) in [
        (2, &[1009, 4001, 16381, 65537][..]),
        (3, &[127, 503, 1009][..]),
        (4, &[31, 61, 127][..]),
    ] {
        for &p in ps {
            for family in 0..2 {
                out.push((g, p, family));
            }
        }
    }
    out
}
fn prime(n: u64) -> bool {
    n >= 2 && !(2..).take_while(|d| d * d <= n).any(|d| n % d == 0)
}
fn upper(p: u64, g: usize) -> u128 {
    let s = (p as f64).sqrt() as u128 + 2;
    s.pow((2 * g) as u32)
}
struct Fixture {
    c: Curve,
    d1: Div,
    ell: u64,
    family: u64,
    attempt: u64,
    selection_ns: u64,
}
impl Fixture {
    fn encode(&self) -> String {
        let mut v = vec![
            "FIXTURE".to_string(),
            self.c.g.to_string(),
            self.c.fp.p.to_string(),
            self.family.to_string(),
            self.attempt.to_string(),
            self.ell.to_string(),
        ];
        for p in [&self.c.f, &self.d1.u, &self.d1.v] {
            v.push(p.n.to_string());
            v.extend(p.c[..p.n].iter().map(u64::to_string));
        }
        v.push(self.selection_ns.to_string());
        v.join(" ")
    }
    fn parse(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next(), Some("FIXTURE"));
        let mut take = || it.next().unwrap().parse::<u64>().unwrap();
        let g = take() as usize;
        let p = take();
        let family = take();
        let attempt = take();
        let ell = take();
        assert!(prime(p) && p > 3 && p <= 1_000_000);
        assert!(prime(ell));
        let mut polys = Vec::new();
        for _ in 0..3 {
            let n = take() as usize;
            assert!(n <= 24);
            let cs: Vec<_> = (0..n).map(|_| take()).collect();
            assert!(cs.iter().all(|&v| v < p));
            polys.push(Poly::from_coeffs(&cs));
        }
        let selection_ns = take();
        assert!(it.next().is_none());
        let c = Curve::new(Fp::new(p), polys[0]);
        let d1 = Div {
            u: polys[1],
            v: polys[2],
        };
        assert_eq!(c.g, g);
        assert_eq!(c.f.lc(), 1);
        assert!(c.is_valid(&d1) && !c.is_zero(&d1));
        assert!(c.is_zero(&c.mul(&d1, ell)));
        assert!((ell as u128).pow(2) > upper(p, g));
        Self {
            c,
            d1,
            ell,
            family,
            attempt,
            selection_ns,
        }
    }
    fn target(&self, s: u64) -> (u64, Div) {
        let seed =
            9000001 ^ ((self.c.g as u64) << 48) ^ (self.c.fp.p << 16) ^ (self.family << 8) ^ s;
        let k = 1 + Rng::new(seed).below(self.ell - 1);
        (k, self.c.mul(&self.d1, k))
    }
}
fn select(g: usize, p: u64, family: u64) -> Option<Fixture> {
    assert!(prime(p));
    let now = Instant::now();
    let fp = Fp::new(p);
    let mut rng = Rng::new((2026111501 + family) ^ p.rotate_left(17) ^ (g as u64).rotate_left(40));
    for attempt in 0..128 {
        let mut cs: Vec<_> = (0..=2 * g).map(|_| rng.below(p)).collect();
        cs.push(1);
        let f = Poly::from_coeffs(&cs);
        if Poly::gcd(&f, &f.deriv(&fp), &fp).deg() != 0 {
            eprintln!("REJECT {attempt} singular");
            continue;
        }
        let c = Curve::new(fp, f);
        let fb = factor_base(&c);
        let (lo, _) = hasse_interval(p, g);
        for (point, &(x, y)) in fb.iter().take(8).enumerate() {
            let d = c.place(x, y);
            let Some(order) = order_bsgs(&c, &d, 200_000) else {
                eprintln!("REJECT {attempt} {point} bsgs_budget");
                continue;
            };
            let ell = *prime_factors(order).last().unwrap();
            if ell < 1000 || (ell as u128) * 4 < lo as u128 || (ell as u128).pow(2) <= upper(p, g) {
                eprintln!("REJECT {attempt} {point} order={order} ell={ell}");
                continue;
            }
            let d1 = c.mul(&d, order / ell);
            if c.is_zero(&d1) || !c.is_valid(&d1) || !c.is_zero(&c.mul(&d1, ell)) {
                continue;
            }
            return Some(Fixture {
                c,
                d1,
                ell,
                family,
                attempt,
                selection_ns: now.elapsed().as_nanos() as u64,
            });
        }
    }
    None
}
fn worker(path: &str, s: u64, arm: &str) {
    let f = Fixture::parse(&fs::read_to_string(path).unwrap());
    let (expected, target) = f.target(s);
    let (g, p) = (f.c.g, f.c.fp.p);
    if arm.starts_with("rational-") {
        let requested = if g == 2 {
            let mut h = 6;
            loop {
                let mut v = 1u64;
                for j in 0..6 {
                    v = v * (h - j) / (j + 1);
                }
                if 32 * v >= 4 * p {
                    break h as usize;
                }
                h += 1;
            }
        } else {
            (p as f64).powf(1.0 - 1.0 / g as f64).ceil() as usize
        };
        let gate = arm == "rational-linear-gate";
        let report = if arm == "rational-generic" {
            rational_solve::solve(
                &f.c,
                &f.d1,
                &target,
                f.ell,
                20260916 + s,
                requested,
                2,
                gate,
            )
        } else {
            assert!(matches!(
                arm,
                "rational-linear" | "rational-linear-aa" | "rational-linear-gate"
            ));
            rational_solve::solve_linear(&f.c, &f.d1, &target, f.ell, 20260916 + s, requested, gate)
        };
        let r = &report.base;
        let check = Instant::now();
        let correct = r.k == Some(expected) && r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":{gate},\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"anchors\":{},\"requested_h\":{},\"m\":{},\"trials\":{},\"constructed\":{},\"tuples\":{},\"splits\":{},\"admitted\":{},\"duplicates\":{},\"pure_rows\":{},\"unions\":{},\"pins\":{},\"descent_steps\":{},\"common\":{},\"normalized_admitted\":{},\"raw_checks\":{},\"pure_checks\":{},\"status\":\"{}\"}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,requested,r.m,r.draws,report.constructed,r.unique_tuples,r.splits,r.admitted,r.duplicate_relations,r.pure_rows,r.unions,r.pins,r.descent_steps,report.common,report.normalized_admitted,r.raw_checks,r.pure_checks,report.status);
        if !correct {
            std::process::exit(3);
        }
        return;
    }
    let gate = arm == "compact-gate";
    if matches!(arm, "compact" | "compact-gate" | "orbit") {
        let r = if arm == "orbit" {
            anchor::solve_orbit(&f.c, &f.d1, &target, f.ell, 20260916 + s, false)
        } else {
            anchor::solve_compact(&f.c, &f.d1, &target, f.ell, 20260916 + s, gate)
        };
        let check = Instant::now();
        let group = r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let correct = r.k == Some(expected) && group;
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":{gate},\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"anchors\":{},\"m\":{},\"trials\":{},\"tuples\":{},\"admitted\":{},\"pure_rows\":{},\"raw_checks\":{},\"pure_checks\":{},\"reported_bytes\":{}}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,r.m,r.draws,r.unique_tuples,r.admitted,r.pure_rows,r.raw_checks,r.pure_checks,r.memory_bytes);
        if !correct {
            std::process::exit(3);
        }
    } else {
        let (table, branches, solver, reference) = match arm {
            "n1fr" => (false, 1, Solver::Fast, true),
            "n1fd" => (false, 1, Solver::Fast, false),
            "n1hr" => (false, 1, Solver::Hybrid, true),
            "n1hd" => (false, 1, Solver::Hybrid, false),
            "n64fr" => (false, 64, Solver::Fast, true),
            "n64fd" => (false, 64, Solver::Fast, false),
            "n64hr" => (false, 64, Solver::Hybrid, true),
            "n64hd" => (false, 64, Solver::Hybrid, false),
            "t1f" | "t1faa" => (true, 1, Solver::Fast, true),
            "t1h" => (true, 1, Solver::Hybrid, true),
            "t64f" => (true, 64, Solver::Fast, true),
            "t64h" => (true, 64, Solver::Hybrid, true),
            _ => panic!("unknown arm"),
        };
        let params = IcParams {
            branches,
            solver,
            reference_oracle: reference,
            max_trials: 2_000_000,
            seed: 20260916 + s,
            ..IcParams::default()
        };
        let r = if table {
            pipeline::solve_table(&f.c, &f.d1, &target, f.ell, &params)
        } else {
            ic::solve_dlp(&f.c, &f.d1, &target, f.ell, &params)
        };
        let check = Instant::now();
        let correct = r.k == Some(expected) && r.k.is_some_and(|k| f.c.mul(&f.d1, k) == target);
        let extra = check.elapsed().as_nanos();
        let got = r.k.map_or("null".to_string(), |k| k.to_string());
        println!("CELL {{\"g\":{g},\"p\":{p},\"family\":{},\"attempt\":{},\"ell\":{},\"seed\":{s},\"arm\":\"{arm}\",\"gate\":false,\"expected_k\":{expected},\"k\":{got},\"target_u\":{:?},\"target_v\":{:?},\"correct\":{correct},\"total_ns\":{},\"fb_ns\":{},\"walk_ns\":{},\"oracle_ns\":{},\"row_ns\":{},\"solve_ns\":{},\"extra_check_ns\":{extra},\"m\":{},\"trials\":{},\"relations\":{},\"solve_rounds\":{}}}",f.family,f.attempt,f.ell,&target.u.c[..target.u.n],&target.v.c[..target.v.n],r.total_ns,r.fb_ns,r.walk_ns,r.oracle_ns,r.row_ns,r.solve_ns,r.m,r.trials,r.relations,r.solve_rounds);
        if !correct {
            std::process::exit(3);
        }
    }
}
fn sha(path: &Path) -> String {
    let out = Command::new("/usr/bin/shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
struct Receipt {
    status: String,
    wall: u128,
    cached: bool,
}
fn receipt(path: &Path, header: Option<&str>) -> Receipt {
    let s = fs::read_to_string(path).unwrap();
    let mut lines = s.lines();
    let first = lines.next().unwrap();
    if let Some(h) = header {
        assert_eq!(first, h, "resume identity mismatch");
    }
    let fields: Vec<_> = lines.next().unwrap().split_whitespace().collect();
    Receipt {
        status: fields[0].to_string(),
        wall: fields[1].parse().unwrap(),
        cached: true,
    }
}
fn cell(dir: &Path, key: &str, args: &[String], hash: &str, cap: u64, skip: bool) -> Receipt {
    let header = format!("HEC-HELDOUT-v1 {BUILD} {hash} {args:?}");
    let rp = dir.join(format!("{key}.receipt"));
    if rp.exists() {
        return receipt(&rp, Some(&header));
    }
    let out = dir.join(format!("{key}.out"));
    let err = dir.join(format!("{key}.err"));
    let pid = dir.join(format!("{key}.pid"));
    assert!(
        !out.exists() && !err.exists() && !pid.exists(),
        "incomplete cell {key}: inspect process before resume"
    );
    let now = Instant::now();
    let (status, wall) = if skip {
        ("gate_failed".to_string(), 0)
    } else {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(args)
            .stdout(Stdio::from(File::create(&out).unwrap()))
            .stderr(Stdio::from(File::create(&err).unwrap()))
            .spawn()
            .unwrap();
        fs::write(&pid, format!("{}\n{header}\n", child.id())).unwrap();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break format!("exit_{}", status.code().map_or(-1, |n| n));
            }
            if now.elapsed() >= Duration::from_secs(cap) {
                if let Err(error) = child.kill() {
                    if let Some(status) = child.try_wait().unwrap() {
                        break format!("exit_{}", status.code().map_or(-1, |n| n));
                    }
                    panic!("could not stop owned child: {error}");
                }
                child.wait().unwrap();
                break "timeout".to_string();
            }
            thread::sleep(Duration::from_millis(5));
        };
        fs::remove_file(pid).unwrap();
        (status, now.elapsed().as_nanos())
    };
    let temp = dir.join(format!("{key}.receipt.tmp"));
    fs::write(&temp, format!("{header}\n{status} {wall}\n")).unwrap();
    fs::rename(temp, rp).unwrap();
    Receipt {
        status,
        wall,
        cached: false,
    }
}
fn stage(mode: &str, root: &Path) {
    assert_ne!(BUILD, "UNFROZEN");
    fs::create_dir_all(root).unwrap();
    let stage = match mode {
        "--select" => "select",
        "--gate" => "gate",
        "--measure" => "measure",
        _ => panic!("unknown stage"),
    };
    let mut used: u128 = fs::read_dir(root)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(&format!("{stage}-"))
                && p.extension().is_some_and(|e| e == "receipt")
        })
        .map(|p| receipt(&p, None).wall)
        .sum();
    let budget = if stage == "select" { 600 } else { 1800 };
    for (index, (g, p, family)) in cohort().into_iter().enumerate() {
        let base = format!("g{g}-p{p}-f{family}");
        let fixture = root.join(format!("select-{base}.out"));
        if stage == "select" {
            let key = format!("select-{base}");
            if !root.join(format!("{key}.receipt")).exists()
                && used + 30_000_000_000 > budget * 1_000_000_000
            {
                println!("BUDGET {stage} {used}");
                std::process::exit(5);
            }
            let r = cell(
                root,
                &key,
                &[
                    "--select-worker".into(),
                    g.to_string(),
                    p.to_string(),
                    family.to_string(),
                ],
                "none",
                30,
                false,
            );
            if !r.cached {
                used += r.wall;
            }
            println!("STATUS {key} {} {}", r.status, r.wall);
            continue;
        }
        let sr = root.join(format!("select-{base}.receipt"));
        if !sr.exists() || receipt(&sr, None).status != "exit_0" {
            println!("UNAVAILABLE {base}");
            continue;
        }
        let hash = sha(&fixture);
        for s in 0..5u64 {
            let mut arms = if stage == "gate" {
                vec!["compact-gate"]
            } else {
                ARMS.to_vec()
            };
            if stage == "measure" {
                arms.rotate_left((index * 5 + s as usize) % 15);
                if s % 2 == 1 {
                    arms.reverse();
                }
            }
            for arm in arms {
                let key = format!("{stage}-{base}-s{s}-{arm}");
                let cap = if stage == "gate" { 60 } else { 30 };
                if !root.join(format!("{key}.receipt")).exists()
                    && used + (cap as u128) * 1_000_000_000 > budget * 1_000_000_000
                {
                    println!("BUDGET {stage} {used}");
                    std::process::exit(5);
                }
                let gate = root.join(format!("gate-{base}-s{s}-compact-gate.receipt"));
                let skip = stage == "measure"
                    && matches!(arm, "orbit" | "compact")
                    && (!gate.exists() || receipt(&gate, None).status != "exit_0");
                let args = vec![
                    "--worker".to_string(),
                    fixture.to_str().unwrap().to_string(),
                    s.to_string(),
                    arm.to_string(),
                ];
                let r = cell(root, &key, &args, &hash, cap, skip);
                if !r.cached {
                    used += r.wall;
                }
                println!("STATUS {key} {} {}", r.status, r.wall);
            }
        }
    }
    println!("TERMINAL {stage} child_wall_ns={used}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "--linear-cold" => linear_stage(Path::new(&a[2]), Path::new(&a[3])),
        "--sleep-ms" => thread::sleep(Duration::from_millis(a[2].parse().unwrap())),
        "--runner-self-test" => {
            let dir = Path::new(&a[2]);
            fs::create_dir_all(dir).unwrap();
            let args = vec!["--sleep-ms".to_string(), "1".to_string()];
            let first = cell(dir, "success", &args, "self-test", 2, false);
            let replay = cell(dir, "success", &args, "self-test", 2, false);
            assert_eq!(first.status, "exit_0");
            assert!(replay.cached);
            assert_eq!(first.wall, replay.wall);
            let mismatch = std::panic::catch_unwind(|| {
                cell(dir, "success", &args, "different-input", 2, false)
            });
            assert!(mismatch.is_err(), "changed input must fail closed");
            let timed = cell(
                dir,
                "timeout",
                &["--sleep-ms".into(), "2000".into()],
                "self-test",
                1,
                false,
            );
            assert_eq!(timed.status, "timeout");
            println!("SELFTEST success_replay_identity_timeout_passed");
        }
        "--select-worker" => {
            match select(
                a[2].parse().unwrap(),
                a[3].parse().unwrap(),
                a[4].parse().unwrap(),
            ) {
                Some(f) => println!("{}", f.encode()),
                None => {
                    println!("UNAVAILABLE selection_cap");
                    std::process::exit(4);
                }
            }
        }
        "--worker" => worker(&a[2], a[3].parse().unwrap(), &a[4]),
        mode => stage(mode, &PathBuf::from(&a[2])),
    }
}

const LINEAR_ARMS: [&str; 16] = [
    "n1fr",
    "n1fd",
    "n1hr",
    "n1hd",
    "n64fr",
    "n64fd",
    "n64hr",
    "n64hd",
    "t1f",
    "t1h",
    "t64f",
    "t64h",
    "compact",
    "rational-generic",
    "rational-linear",
    "rational-linear-aa",
];
fn linear_stage(root: &Path, origin: &Path) {
    assert_ne!(BUILD, "UNFROZEN");
    fs::create_dir_all(root).unwrap();
    let mut used: u128 = fs::read_dir(root)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "receipt"))
        .map(|p| receipt(&p, None).wall)
        .sum();
    for (index, (g, p)) in [
        (2, 1009),
        (2, 16381),
        (3, 127),
        (3, 1009),
        (4, 31),
        (4, 127),
    ]
    .into_iter()
    .enumerate()
    {
        let name = format!("select-g{g}-p{p}-f0.out");
        let fixture = root.join(&name);
        let source = origin.join(&name);
        if fixture.exists() {
            assert_eq!(sha(&fixture), sha(&source));
        } else {
            fs::copy(&source, &fixture).unwrap();
        }
        let input = sha(&fixture);
        for s in 5..8u64 {
            let gate_key = format!("gate-g{g}-p{p}-f0-s{s}-rational-linear-gate");
            let args = vec![
                "--worker".into(),
                fixture.to_str().unwrap().into(),
                s.to_string(),
                "rational-linear-gate".into(),
            ];
            if !root.join(format!("{gate_key}.receipt")).exists()
                && used + 60_000_000_000 > 1_800_000_000_000
            {
                println!("BUDGET {used}");
                std::process::exit(5);
            }
            let gate = cell(root, &gate_key, &args, &input, 60, false);
            if !gate.cached {
                used += gate.wall;
            }
            println!("STATUS {gate_key} {} {}", gate.status, gate.wall);
            let mut arms = LINEAR_ARMS.to_vec();
            arms.rotate_left((index * 3 + (s - 5) as usize) % 16);
            if s % 2 == 1 {
                arms.reverse();
            }
            for arm in arms {
                let key = format!("measure-g{g}-p{p}-f0-s{s}-{arm}");
                if !root.join(format!("{key}.receipt")).exists()
                    && used + 30_000_000_000 > 1_800_000_000_000
                {
                    println!("BUDGET {used}");
                    std::process::exit(5);
                }
                let skip = gate.status != "exit_0" && arm.starts_with("rational-");
                let args = vec![
                    "--worker".into(),
                    fixture.to_str().unwrap().into(),
                    s.to_string(),
                    arm.into(),
                ];
                let result = cell(root, &key, &args, &input, 30, skip);
                if !result.cached {
                    used += result.wall;
                }
                println!("STATUS {key} {} {}", result.status, result.wall);
            }
        }
    }
    println!("TERMINAL linear-cold child_wall_ns={used}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cohort_has_twenty_prime_field_slots() {
        assert_eq!(cohort().len(), 20);
        for (_, p, _) in cohort() {
            assert!(prime(p));
        }
        assert!(!prime(65535));
    }
    #[test]
    fn fixture_roundtrip_and_target_replay() {
        let i = hec_native_ic::instance::pick_instance(101, 2).unwrap();
        let f = Fixture {
            c: i.curve,
            d1: i.d1,
            ell: i.ell,
            family: 1,
            attempt: 3,
            selection_ns: 123,
        };
        let text = f.encode();
        let restored = Fixture::parse(&text);
        assert_eq!(text, restored.encode());
        assert_eq!(f.target(0), restored.target(0));
        assert_ne!(f.target(0).0, f.target(1).0);
    }
}

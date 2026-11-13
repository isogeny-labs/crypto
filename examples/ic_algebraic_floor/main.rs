//! `ic_algebraic_floor`: cost per relation of algebraic and SAT
//! decompositions for `m ≥ 3` summands against the meet-in-the-middle
//! (pair-table) floor, on the same binary Koblitz cells and targets
//! (`research/ic_m3_algebraic_vs_mitm_20261014/`).
//!
//! ```bash
//! cargo run --release --example ic_algebraic_floor -- run --m 3 \
//!     --cells 1:13:6,1:17:7 --arms f4-chain,f6-ic,sat,s4-hybrid-g4 \
//!     --pool 2048 --kyes 6 --kno 6 --budget-secs 120 --seed 20261014 --out DIR
//! cargo run --release --example ic_algebraic_floor -- analyze --dir DIR --out ANALYSIS.json
//! ```
//!
//! A cell is `a:n:d`: the Koblitz curve `K_a / GF(2^n)` with the standard
//! subspace factor base `V = span(1, z, …, z^{d−1})` (every point with
//! abscissa in `V`, both signs).  A relation is a target `R = [k]G` that
//! decomposes as `R = P_1 + … + P_m` over the base; every arm answers the
//! decision "does `R` decompose, and with which summands", stopping at the
//! first decomposition.
//!
//! * The floor (`truth`) is `PairSumTable` probing (`m = 3`: `|F|` probes a
//!   target, `m = 4`: `|F|²`), run over a pool of seeded uniform targets.
//!   It is exact and complete, so it is also the ground truth; on small
//!   cells it is itself checked against exhaustive enumeration
//!   (`pdp_split::exhaustive_decompositions`).
//! * The algebraic arms run on `kyes` decomposable and `kno` non-decomposable
//!   targets drawn in pool order, each solve in its own child process under
//!   a wall budget and an address-space cap, so a solve that does not finish
//!   is recorded as censored at the budget, never dropped.
//!
//! A stage diagnostic (AGENTS.md §8): decomposition cost per relation,
//! never an end-to-end ECDLP.

mod linear21;

use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, BinaryInstance};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, FieldStructure, SolverEngine,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, groebner_decompose, groebner_decompose_f6_ic,
    sat_decompose_with, FrobeniusFactorBase, KoblitzCurve, PairSumTable, SatDecompositionOptions,
};
use crypto_lib::cryptanalysis::pdp_split::{
    build_for_instance, exhaustive_decompositions, solve_branch, summands_of, verify_decomposition,
};
use crypto_lib::cryptanalysis::wide_groebner::wide_groebner_decompose;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn num<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    flag(args, name)
        .map(|v| {
            v.parse()
                .unwrap_or_else(|_| panic!("{name} takes a number, got {v}"))
        })
        .unwrap_or(default)
}

/// One cell's curve, base and fast views, built the same way in the
/// orchestrator and in every child.
struct Cell {
    a: u8,
    n: u32,
    d: u32,
    kc: KoblitzCurve,
    fb: FrobeniusFactorBase,
    inst: BinaryInstance,
    r: u64,
}

impl Cell {
    fn new(a: u8, n: u32, d: u32) -> Self {
        let kc = KoblitzCurve::new(a, n).expect("Koblitz curve with a usable subgroup");
        let fb = build_standard_subspace_factor_base(&kc, d).expect("standard subspace base");
        let inst = koblitz_instance(a, n).expect("binary instance");
        assert!(
            inst.irreducible.degree == kc.curve.irreducible.degree
                && inst.irreducible.low_terms == kc.curve.irreducible.low_terms,
            "instance and curve must share the field representation"
        );
        let r = kc.subgroup_order.to_u64().expect("r fits u64");
        Self {
            a,
            n,
            d,
            kc,
            fb,
            inst,
            r,
        }
    }

    fn target(&self, k: u64) -> BinaryPoint {
        self.kc.mul(self.kc.generator(), &BigUint::from(k))
    }

    fn x_of(&self, p: &BinaryPoint) -> u64 {
        self.inst.fast.lift(p).x
    }

    fn basis_words(&self) -> Vec<u64> {
        (0..self.d).map(|i| 1u64 << i).collect()
    }

    /// Columns of the relation matrix: points up to sign.
    fn columns(&self) -> usize {
        self.fb.signed_orbits.len()
    }

    fn check_witness(&self, idx: &[usize], target: &BinaryPoint) -> bool {
        let sum = idx.iter().fold(BinaryPoint::Infinity, |s, &i| {
            self.kc.add(&s, &self.fb.points[i])
        });
        &sum == target
    }

    /// Sorted abscissae of a decomposition, the form `pdp_split` compares.
    fn abscissae(&self, idx: &[usize]) -> Vec<u64> {
        let mut xs: Vec<u64> = idx.iter().map(|&i| self.x_of(&self.fb.points[i])).collect();
        xs.sort_unstable();
        xs
    }
}

/// Peak resident set of this process in kB (`VmHWM`), 0 when unreadable.
fn vm_hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        })
        .unwrap_or(0)
}

fn parse_cell(s: &str) -> (u8, u32, u32) {
    let p: Vec<&str> = s.split(':').collect();
    assert_eq!(p.len(), 3, "a cell is a:n:d, got {s}");
    (
        p[0].parse().unwrap(),
        p[1].parse().unwrap(),
        p[2].parse().unwrap(),
    )
}

fn cell_seed(seed: u64, m: usize, n: u32, d: u32) -> u64 {
    seed ^ (m as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (n as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ (d as u64).wrapping_mul(0x1656_67b1_9e37_79f9)
}

/// Group additions the sorted pair-table scan performs up to and
/// including outer row `row` (`m = 3`: one `R − P_k` a row; `m = 4`: the
/// row's `l + 1` pair sums and as many rests).
fn floor_adds(m: usize, rows_done: u64) -> u64 {
    match m {
        3 => rows_done,
        4 => rows_done * (rows_done + 1),
        _ => unreachable!(),
    }
}

// ── truth: the floor and the ground truth ─────────────────────────────

fn truth(
    cell: &Cell,
    m: usize,
    pool: usize,
    kyes: usize,
    kno: usize,
    seed: u64,
    exhaustive_cap: f64,
) -> Value {
    let t0 = Instant::now();
    let table = PairSumTable::build_full(&cell.kc, &cell.fb).expect("pair table");
    let build_ns = t0.elapsed().as_nanos() as u64;
    let pts = cell.fb.points.len();
    let build_adds = (pts as u64) * (pts as u64 + 1) / 2;
    let mut rng = StdRng::seed_from_u64(cell_seed(seed, m, cell.n, cell.d));
    let mut rows = Vec::with_capacity(pool);
    let mut yes_sel = Vec::new();
    let mut no_sel = Vec::new();
    let (mut sum_ns, mut sum_adds, mut decomposable, mut total_decomps) = (0u64, 0u64, 0u64, 0u64);
    for _ in 0..pool {
        let k = rng.gen_range(1..cell.r);
        let target = cell.target(k);
        let fast = table.curve().lift(&target);
        // Decision mode, timed: the sorted scan, stopping at the first
        // witness, exactly what a relation search pays.
        let mut first: Option<Vec<usize>> = None;
        let t = Instant::now();
        table.witnesses_fast(fast, m, &mut |w| {
            first = Some(w.to_vec());
            false
        });
        let ns = t.elapsed().as_nanos() as u64;
        let rows_done = match &first {
            Some(w) => *w.iter().max().unwrap() as u64 + 1,
            None => pts as u64,
        };
        let adds = floor_adds(m, rows_done);
        // Counting mode, untimed: every sorted decomposition.
        let mut all: Vec<Vec<usize>> = Vec::new();
        table.witnesses_fast(fast, m, &mut |w| {
            let mut v = w.to_vec();
            v.sort_unstable();
            all.push(v);
            true
        });
        all.sort();
        all.dedup();
        for w in &all {
            assert!(
                cell.check_witness(w, &target),
                "pair-table witness does not add up"
            );
        }
        assert_eq!(
            first.is_some(),
            !all.is_empty(),
            "decision and count disagree"
        );
        sum_ns += ns;
        sum_adds += adds;
        total_decomps += all.len() as u64;
        let yes = !all.is_empty();
        if yes {
            decomposable += 1;
            if yes_sel.len() < kyes {
                yes_sel.push(k);
            }
        } else if no_sel.len() < kno {
            no_sel.push(k);
        }
        rows.push(json!({"k": k, "yes": yes, "decomps": all.len(), "ns": ns, "adds": adds}));
    }
    // Exhaustive cross-check of the floor on the selected targets, where
    // enumeration is affordable.
    let liftable = (0..1u64 << cell.d)
        .filter(|&x| crypto_lib::cryptanalysis::pdp_split::lift_x(&cell.inst, x).is_some())
        .count();
    let tuples = (0..m).fold(1f64, |acc, i| acc * (liftable + i) as f64 / (i + 1) as f64);
    let mut exhaustive = Vec::new();
    if tuples <= exhaustive_cap {
        for &k in yes_sel.iter().chain(no_sel.iter()) {
            let target = cell.target(k);
            let xr = cell.x_of(&target);
            let ex = exhaustive_decompositions(&cell.inst, &cell.basis_words(), m, xr);
            let fast = table.curve().lift(&target);
            let mut mitm: BTreeSet<Vec<u64>> = BTreeSet::new();
            table.witnesses_fast(fast, m, &mut |w| {
                mitm.insert(cell.abscissae(w));
                true
            });
            exhaustive.push(
                json!({"k": k, "exhaustive": ex.len(), "mitm": mitm.len(), "equal": ex == mitm}),
            );
        }
    }
    // Interleave yes/no so the early-stop rule sees both strata first.
    let mut selected = Vec::new();
    for i in 0..yes_sel.len().max(no_sel.len()) {
        if let Some(&k) = yes_sel.get(i) {
            selected.push(json!({"k": k, "yes": true}));
        }
        if let Some(&k) = no_sel.get(i) {
            selected.push(json!({"k": k, "yes": false}));
        }
    }
    json!({
        "cmd": "truth", "m": m, "a": cell.a, "n": cell.n, "d": cell.d,
        "curve": cell.kc.label(), "r": cell.r, "seed": seed,
        "fb_points": pts, "fb_columns": cell.columns(), "liftable_abscissae": liftable,
        "table_build_ns": build_ns, "table_build_adds": build_adds,
        "vm_hwm_kb": vm_hwm_kb(),
        "pool": pool, "decomposable": decomposable, "total_decomps": total_decomps,
        "floor_sum_ns": sum_ns, "floor_sum_adds": sum_adds,
        "exhaustive_tuples": tuples, "exhaustive_check": exhaustive,
        "selected": selected, "rows": rows,
    })
}

// ── solve: one algebraic decision in a child process ──────────────────

fn limit_address_space(gib: u64) {
    let bytes = gib << 30;
    let lim = libc::rlimit {
        rlim_cur: bytes,
        rlim_max: bytes,
    };
    // SAFETY: setrlimit reads the struct it is handed.
    unsafe {
        libc::setrlimit(libc::RLIMIT_AS, &lim);
    }
}

/// `…-gG` guesses `G` unknowns; `…-fK` guesses all but `K` of the `m·d`
/// summand unknowns (`g = max(0, m·d − K)`), so the branch systems keep
/// the same size as `d` grows.
fn hybrid_g(arm: &str, m: usize, d: u32) -> Option<usize> {
    if let Some((_, g)) = arm.rsplit_once("-g") {
        return g.parse().ok();
    }
    let (_, k) = arm.rsplit_once("-f")?;
    let k: usize = k.parse().ok()?;
    Some((m * d as usize).saturating_sub(k))
}

fn solve(cell: &Cell, m: usize, arm: &str, k: u64, budget: Duration) -> Value {
    let target = cell.target(k);
    let index_of = cell.fb.index_map();
    let st = FieldStructure::new(cell.kc.n, &cell.kc.curve.irreducible);
    let engine = SolverEngine::InheritedF4 { max_degree: 3 };
    let node_budget = usize::MAX / 2;
    let x0 = f4_word_ops_thread();
    let mut t0 = Instant::now();
    let mut extra = json!({});
    let (hit, exhausted, unsupported): (Option<Vec<usize>>, bool, bool) = match arm {
        "f4-chain" | "f6-ic" => {
            let (h, s) = if arm == "f4-chain" {
                groebner_decompose(
                    &cell.kc,
                    &cell.fb,
                    &index_of,
                    &st,
                    &target,
                    m,
                    engine,
                    node_budget,
                )
            } else {
                groebner_decompose_f6_ic(
                    &cell.kc,
                    &cell.fb,
                    &index_of,
                    &st,
                    &target,
                    m,
                    engine,
                    node_budget,
                )
            };
            extra = json!({"reductions": s.reductions, "splits": s.splits,
                "max_degree_built": s.max_degree_built});
            (h, s.exhausted, s.unsupported)
        }
        "wide-chain" => {
            // The u128 chained-S3 solver (`wide_groebner`, up to 128 unknowns):
            // whole-summand branching with Macaulay linearisation at each node.
            let (h, s) = wide_groebner_decompose(
                &cell.kc,
                &cell.fb,
                &index_of,
                &st,
                &target,
                m,
                node_budget,
            );
            extra = json!({"nodes": s.nodes, "reductions": s.reductions, "refuted": s.refuted,
                "leaves": s.leaves, "max_rows": s.max_rows, "max_cols": s.max_cols});
            (h, s.exhausted, s.unsupported)
        }
        "lin21" | "lin21-ht" => {
            // Table-free 2 + 1: for every third summand, the linear S3
            // solve on (x1 + x2, x1 x2).  Setup (the base's index map and
            // the linear-map tables, O(|F|)) is timed apart from the
            // decision, as the floor's table build is.
            let variant = if arm == "lin21" {
                linear21::Variant::Full
            } else {
                linear21::Variant::HalfTrace
            };
            let base: Vec<_> = cell
                .fb
                .points
                .iter()
                .map(|p| cell.inst.fast.lift(p))
                .collect();
            let lin = linear21::Linear21::new(
                &cell.inst.gf,
                &cell.inst.fast,
                cell.n,
                cell.d as usize,
                cell.inst.b,
                base,
            );
            let setup_ns = t0.elapsed().as_nanos() as u64;
            let r = cell.inst.fast.lift(&target);
            let mut c = linear21::Counts::default();
            t0 = Instant::now();
            let res = lin.decide(r, variant, &mut c, || t0.elapsed() >= budget);
            extra = json!({"setup_ns": setup_ns, "trials": c.trials, "consistent": c.consistent,
                "splits": c.splits, "gauss_xors": c.gauss_xors, "field_muls": c.field_muls,
                "field_invs": c.field_invs, "verify_adds": c.verify_adds,
                "truncated": c.truncated});
            match res {
                Ok(Some(w)) => (Some(w.to_vec()), false, false),
                Ok(None) => (None, c.truncated, false),
                Err(()) => (None, true, false),
            }
        }
        "sat" => {
            let options = SatDecompositionOptions {
                restrict_to_factor_base: true,
                wall_budget: Some(budget),
                ..SatDecompositionOptions::default()
            };
            let (h, s) = sat_decompose_with(
                &cell.kc,
                &cell.fb,
                &index_of,
                &st,
                &target,
                m,
                1 << 24,
                None,
                options,
            );
            extra = json!({"conflicts": s.conflicts, "models": s.models,
                "solver_calls": s.solver_calls, "spurious": s.spurious, "refuted": s.refuted});
            (h, s.exhausted, s.unsupported)
        }
        a if a.starts_with("s4-hybrid") || a.starts_with("s22-hybrid") => {
            let g = hybrid_g(a, m, cell.d).expect("hybrid arm names its g");
            let plan: Vec<usize> = if a.starts_with("s4") {
                vec![3]
            } else {
                vec![2, 2]
            };
            assert_eq!(plan.iter().sum::<usize>(), m, "{a} does not match m = {m}");
            let xr = cell.x_of(&target);
            match build_for_instance(&cell.inst, &plan, cell.d as usize, xr) {
                Err(e) => {
                    extra = json!({"error": e});
                    (None, true, true)
                }
                Ok((sys, _)) => {
                    let words = cell.basis_words();
                    let (mut xors, mut branches, mut unresolved, mut timed_out) =
                        (0u64, 0u64, 0u64, false);
                    let mut found: Option<Vec<u64>> = None;
                    let mut max_sd = 0u32;
                    'branches: for guess in 0..(1u64 << g) {
                        let left = budget.saturating_sub(t0.elapsed());
                        if left.is_zero() {
                            timed_out = true;
                            break;
                        }
                        let bo = solve_branch(&sys, g, guess, Some(left), 24);
                        branches += 1;
                        xors += bo.word_xors;
                        max_sd = max_sd.max(bo.solving_degree);
                        if bo.timed_out {
                            timed_out = true;
                            break;
                        }
                        match bo.solutions {
                            None => unresolved += 1,
                            Some(sols) => {
                                for a in sols {
                                    let xs = summands_of(&sys, &words, a);
                                    if verify_decomposition(&cell.inst, &xs, xr) == Some(true) {
                                        found = Some(xs);
                                        break 'branches;
                                    }
                                }
                            }
                        }
                    }
                    extra = json!({"g": g, "n_vars": sys.n_vars, "branches": branches,
                        "unresolved": unresolved, "split_word_xors": xors,
                        "solving_degree_max": max_sd, "witness_abscissae": found});
                    // The abscissae verified on the curve; map back to base
                    // indices only to report a hit.
                    let hit = found.map(|_| Vec::new());
                    (hit, timed_out || (unresolved > 0), false)
                }
            }
        }
        other => panic!("unknown arm {other}"),
    };
    let wall_ns = t0.elapsed().as_nanos() as u64;
    let word_xors = f4_word_ops_thread() - x0;
    let witness_ok = match &hit {
        Some(idx) if !idx.is_empty() => cell.check_witness(idx, &target),
        Some(_) => true, // hybrid: verified on the curve by verify_decomposition
        None => true,
    };
    // A completed "no" is a refutation; an exhausted "no" is no verdict.
    let verdict = if hit.is_some() {
        "yes"
    } else if exhausted || unsupported {
        "none"
    } else {
        "no"
    };
    json!({"cmd": "solve", "arm": arm, "m": m, "a": cell.a, "n": cell.n, "d": cell.d, "k": k,
        "verdict": verdict, "unsupported": unsupported, "wall_ns": wall_ns, "word_xors": word_xors,
        "witness_ok": witness_ok, "vm_hwm_kb": vm_hwm_kb(), "extra": extra})
}

// ── run: orchestrate cells, arms and per-target timeouts ──────────────

fn run_child(
    exe: &Path,
    m: usize,
    arm: &str,
    cell: (u8, u32, u32),
    k: u64,
    budget_secs: u64,
    mem_gib: u64,
) -> Value {
    let (a, n, d) = cell;
    let grace = Duration::from_secs(budget_secs + 15);
    let t0 = Instant::now();
    let mut child = Command::new(exe)
        .args([
            "solve",
            "--m",
            &m.to_string(),
            "--arm",
            arm,
            "--cell",
            &format!("{a}:{n}:{d}"),
            "--k",
            &k.to_string(),
            "--budget-secs",
            &budget_secs.to_string(),
            "--mem-gib",
            &mem_gib.to_string(),
        ])
        // One core per solve, so wall time compares like with like.
        .env("RAYON_NUM_THREADS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn solve child");
    loop {
        match child.try_wait().expect("wait") {
            Some(status) => {
                let out = child.wait_with_output().expect("collect");
                let text = String::from_utf8_lossy(&out.stdout);
                if let Some(line) = text.lines().rev().find(|l| l.starts_with('{')) {
                    if let Ok(v) = serde_json::from_str::<Value>(line) {
                        return v;
                    }
                }
                let err = String::from_utf8_lossy(&out.stderr);
                let oom = err.contains("memory allocation") || err.contains("alloc");
                return json!({"cmd": "solve", "arm": arm, "m": m, "a": a, "n": n, "d": d, "k": k,
                    "verdict": "crash", "oom": oom, "status": status.to_string(),
                    "wall_ns": t0.elapsed().as_nanos() as u64,
                    "stderr_tail": err.chars().rev().take(400).collect::<String>().chars().rev().collect::<String>()});
            }
            None => {
                if t0.elapsed() > grace {
                    let _ = child.kill();
                    let _ = child.wait();
                    return json!({"cmd": "solve", "arm": arm, "m": m, "a": a, "n": n, "d": d, "k": k,
                        "verdict": "timeout", "wall_ns": t0.elapsed().as_nanos() as u64});
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

fn censored(v: &Value) -> bool {
    if v["unsupported"].as_bool() == Some(true) {
        return false;
    }
    matches!(
        v["verdict"].as_str(),
        Some("timeout") | Some("crash") | Some("none")
    ) || v["wall_ns"].as_u64().unwrap_or(0) as f64 / 1e9
        >= v["budget_secs"].as_f64().unwrap_or(f64::MAX)
}

fn cmd_run(args: &[String]) {
    let m: usize = num(args, "--m", 3);
    let cells: Vec<(u8, u32, u32)> = flag(args, "--cells")
        .expect("--cells a:n:d,...")
        .split(',')
        .map(parse_cell)
        .collect();
    let arms: Vec<String> = flag(args, "--arms")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let pool: usize = num(args, "--pool", 2048);
    let kyes: usize = num(args, "--kyes", 6);
    let kno: usize = num(args, "--kno", 6);
    let budget: u64 = num(args, "--budget-secs", 120);
    let mem: u64 = num(args, "--mem-gib", 4);
    let seed: u64 = num(args, "--seed", 20261014);
    let stop_after: usize = num(args, "--stop-after", 4);
    let exhaustive_cap: f64 = num(args, "--exhaustive-cap", 5e6);
    let out = PathBuf::from(flag(args, "--out").expect("--out DIR"));
    std::fs::create_dir_all(&out).unwrap();
    let exe = std::env::current_exe().unwrap();
    let mut stopped: BTreeSet<String> = BTreeSet::new();
    for &(a, n, d) in &cells {
        let cell = Cell::new(a, n, d);
        let tr = truth(&cell, m, pool, kyes, kno, seed, exhaustive_cap);
        let tpath = out.join(format!("truth-m{m}-n{n}-d{d}.json"));
        std::fs::write(&tpath, serde_json::to_string(&tr).unwrap()).unwrap();
        eprintln!(
            "m={m} n={n} d={d}: |F|={} decomposable {}/{} floor {:.1} µs/target, exhaustive checks {}",
            tr["fb_points"], tr["decomposable"], pool,
            tr["floor_sum_ns"].as_u64().unwrap() as f64 / pool as f64 / 1e3,
            tr["exhaustive_check"].as_array().unwrap().len()
        );
        drop(cell);
        let selected: Vec<(u64, bool)> = tr["selected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| (s["k"].as_u64().unwrap(), s["yes"].as_bool().unwrap()))
            .collect();
        for arm in &arms {
            let path = out.join(format!("arm-{arm}-m{m}-n{n}-d{d}.jsonl"));
            let mut f = std::fs::File::create(&path).unwrap();
            if stopped.contains(arm) {
                writeln!(
                    f,
                    "{}",
                    json!({"cmd": "skipped", "arm": arm, "m": m, "n": n, "d": d,
                    "reason": "an earlier cell of this arm met the early-stop rule"})
                )
                .unwrap();
                eprintln!("  {arm}: skipped (stopped at a smaller cell)");
                continue;
            }
            let mut lead_censored = 0usize;
            for (i, &(k, yes)) in selected.iter().enumerate() {
                let mut v = run_child(&exe, m, arm, (a, n, d), k, budget, mem);
                v["truth_yes"] = json!(yes);
                v["budget_secs"] = json!(budget);
                v["censored"] = json!(censored(&v));
                let agrees = match v["verdict"].as_str() {
                    Some("yes") => Some(yes),
                    Some("no") => Some(!yes),
                    _ => None,
                };
                v["agrees"] = json!(agrees);
                writeln!(f, "{v}").unwrap();
                f.flush().unwrap();
                eprintln!(
                    "  {arm} k={k} truth={yes} verdict={} {:.3}s",
                    v["verdict"],
                    v["wall_ns"].as_u64().unwrap_or(0) as f64 / 1e9
                );
                if v["unsupported"].as_bool() == Some(true) {
                    eprintln!("  {arm}: unsupported at n={n} (encoder or variable limit)");
                    break;
                }
                if i < stop_after && v["censored"].as_bool().unwrap() {
                    lead_censored += 1;
                }
                if i + 1 == stop_after && lead_censored == stop_after {
                    writeln!(
                        f,
                        "{}",
                        json!({"cmd": "stopped", "arm": arm, "m": m, "n": n, "d": d,
                        "reason": format!("the first {stop_after} targets were all censored")})
                    )
                    .unwrap();
                    stopped.insert(arm.clone());
                    eprintln!("  {arm}: early stop at n={n}");
                    break;
                }
            }
        }
    }
}

// ── linpool: the table-free linear arm over the whole seeded pool ─────

/// `lin21-ht` (or `lin21`) in decision mode on the same seeded pool the
/// floor decides, in this process and with no pair table, so it also runs
/// on cells whose table does not fit in memory.  Where a `truth-*.json` of
/// the same cell and seed is present in `--out`, the verdicts are compared
/// target by target.
fn cmd_linpool(args: &[String]) {
    let m: usize = 3;
    let cells: Vec<(u8, u32, u32)> = flag(args, "--cells")
        .expect("--cells a:n:d,...")
        .split(',')
        .map(parse_cell)
        .collect();
    let pool: usize = num(args, "--pool", 2048);
    let seed: u64 = num(args, "--seed", 20261014);
    let arm = flag(args, "--arm").unwrap_or_else(|| "lin21-ht".into());
    let variant = match arm.as_str() {
        "lin21" => linear21::Variant::Full,
        "lin21-ht" => linear21::Variant::HalfTrace,
        other => panic!("linpool arm is lin21 or lin21-ht, got {other}"),
    };
    let out = PathBuf::from(flag(args, "--out").expect("--out DIR"));
    std::fs::create_dir_all(&out).unwrap();
    for &(a, n, d) in &cells {
        let cell = Cell::new(a, n, d);
        let t0 = Instant::now();
        let base: Vec<_> = cell
            .fb
            .points
            .iter()
            .map(|p| cell.inst.fast.lift(p))
            .collect();
        let pts = base.len();
        let lin = linear21::Linear21::new(
            &cell.inst.gf,
            &cell.inst.fast,
            n,
            d as usize,
            cell.inst.b,
            base,
        );
        let setup_ns = t0.elapsed().as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(cell_seed(seed, m, n, d));
        let mut rows = Vec::with_capacity(pool);
        let (mut sum_ns, mut decomposable) = (0u64, 0u64);
        let mut total = linear21::Counts::default();
        for _ in 0..pool {
            let k = rng.gen_range(1..cell.r);
            let target = cell.target(k);
            let r = cell.inst.fast.lift(&target);
            let mut c = linear21::Counts::default();
            let t = Instant::now();
            let res = lin.decide(r, variant, &mut c, || false);
            let ns = t.elapsed().as_nanos() as u64;
            let w = res.expect("no deadline");
            assert!(!c.truncated, "kernel cap exceeded at n={n} k={k}");
            if let Some(w) = &w {
                assert!(cell.check_witness(w, &target), "witness does not add up");
                decomposable += 1;
            }
            sum_ns += ns;
            total.trials += c.trials;
            total.consistent += c.consistent;
            total.splits += c.splits;
            total.gauss_xors += c.gauss_xors;
            total.field_muls += c.field_muls;
            total.field_invs += c.field_invs;
            total.verify_adds += c.verify_adds;
            rows.push(json!({"k": k, "yes": w.is_some(), "ns": ns, "trials": c.trials}));
        }
        // Target-by-target comparison with the floor's verdicts, if present.
        let tpath = out.join(format!("truth-m{m}-n{n}-d{d}.json"));
        let compared = std::fs::read_to_string(&tpath).ok().map(|txt| {
            let tr: Value = serde_json::from_str(&txt).unwrap();
            assert_eq!(tr["seed"].as_u64(), Some(seed), "truth of another seed");
            let trows = tr["rows"].as_array().unwrap();
            let common = trows.len().min(rows.len());
            let disagree = (0..common)
                .filter(|&i| {
                    assert_eq!(trows[i]["k"], rows[i]["k"], "pools differ");
                    trows[i]["yes"] != rows[i]["yes"]
                })
                .count();
            json!({"targets": common, "disagreements": disagree})
        });
        let rec = json!({
            "cmd": "linpool", "arm": arm, "m": m, "a": a, "n": n, "d": d,
            "curve": cell.kc.label(), "r": cell.r, "seed": seed,
            "fb_points": pts, "fb_columns": cell.columns(),
            "pair_table_entries": (pts as u64) * (pts as u64 + 1) / 2,
            "setup_ns": setup_ns, "vm_hwm_kb": vm_hwm_kb(),
            "pool": pool, "decomposable": decomposable, "sum_ns": sum_ns,
            "trials": total.trials, "consistent": total.consistent, "splits": total.splits,
            "gauss_xors": total.gauss_xors, "field_muls": total.field_muls,
            "field_invs": total.field_invs, "verify_adds": total.verify_adds,
            "floor_comparison": compared, "rows": rows,
        });
        std::fs::write(
            out.join(format!("linpool-{arm}-m{m}-n{n}-d{d}.json")),
            serde_json::to_string(&rec).unwrap(),
        )
        .unwrap();
        eprintln!(
            "linpool {arm} n={n} d={d}: |F|={pts} decomposable {decomposable}/{pool} {:.1} µs/target, {:.0} ns/trial, floor comparison {}",
            sum_ns as f64 / pool as f64 / 1e3,
            sum_ns as f64 / total.trials as f64,
            rec["floor_comparison"]
        );
    }
}

fn cmd_solve(args: &[String]) {
    let m: usize = num(args, "--m", 3);
    let arm = flag(args, "--arm").expect("--arm");
    let (a, n, d) = parse_cell(&flag(args, "--cell").expect("--cell"));
    let k: u64 = num(args, "--k", 1);
    let budget = Duration::from_secs(num(args, "--budget-secs", 120));
    let mem: u64 = num(args, "--mem-gib", 4);
    let cell = Cell::new(a, n, d);
    limit_address_space(mem);
    let v = solve(&cell, m, &arm, k, budget);
    println!("{v}");
}

fn cmd_truth(args: &[String]) {
    let m: usize = num(args, "--m", 3);
    let (a, n, d) = parse_cell(&flag(args, "--cell").expect("--cell"));
    let cell = Cell::new(a, n, d);
    let mut v = truth(
        &cell,
        m,
        num(args, "--pool", 512),
        num(args, "--kyes", 6),
        num(args, "--kno", 6),
        num(args, "--seed", 1),
        num(args, "--exhaustive-cap", 5e6),
    );
    v.as_object_mut().unwrap().remove("rows");
    println!("{v}");
}

// ── analyze ───────────────────────────────────────────────────────────

mod analysis;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") => cmd_run(&args),
        Some("solve") => cmd_solve(&args),
        Some("truth") => cmd_truth(&args),
        Some("linpool") => cmd_linpool(&args),
        Some("linsummary") => cmd_linsummary(&args),
        Some("s4check") => cmd_s4check(&args),
        Some("analyze") => analysis::cmd_analyze(&args),
        _ => eprintln!("usage: ic_algebraic_floor run|solve|truth|linpool|linsummary|s4check|analyze …"),
    }
}

// ── s4check: the symmetrised fourth summation polynomial on witnesses ──

/// Machine check of the desk derivation in
/// `research/groebner_free_matched_20261113/README.md`: with
/// `e₁, e₂, e₃` the elementary symmetric functions of the three summand
/// abscissae and `u = x(R)`,
///
/// ```text
/// S₄ = e₃⁴ + u⁴e₂⁴ + b²e₁⁴ + b²u⁴ + (b + u⁴)e₃² + b·u²·e₂² + b·u³·e₃      (F₂-affine in e)
///    + u·e₃³ + u³·e₂²·e₃ + u²·e₁²·e₃² + b·u·e₁²·e₃                        (the rest)
/// ```
///
/// Evaluated on every witness the linear arm finds in the seeded pool
/// (expected: zero on all), and on the same triples with one abscissa
/// replaced by another base abscissa (negative control: zero only by
/// chance or when the replacement is itself a decomposition).
fn cmd_s4check(args: &[String]) {
    let cells: Vec<(u8, u32, u32)> = flag(args, "--cells")
        .expect("--cells a:n:d,...")
        .split(',')
        .map(parse_cell)
        .collect();
    let pool: usize = num(args, "--pool", 2048);
    let seed: u64 = num(args, "--seed", 20261014);
    let out = PathBuf::from(flag(args, "--out").expect("--out FILE.json"));
    let mut recs = Vec::new();
    for &(a, n, d) in &cells {
        let cell = Cell::new(a, n, d);
        let gf = &cell.inst.gf;
        let b = cell.inst.b;
        let base: Vec<_> = cell
            .fb
            .points
            .iter()
            .map(|p| cell.inst.fast.lift(p))
            .collect();
        let xs_base: Vec<u64> = base.iter().map(|p| p.x).collect();
        let lin =
            linear21::Linear21::new(gf, &cell.inst.fast, n, d as usize, b, base);
        let s4 = |x: [u64; 3], u: u64| -> (u64, u64) {
            let e1 = x[0] ^ x[1] ^ x[2];
            let e2 = gf.mul(x[0], x[1]) ^ gf.mul(x[0], x[2]) ^ gf.mul(x[1], x[2]);
            let e3 = gf.mul(gf.mul(x[0], x[1]), x[2]);
            let sq = |v| gf.sqr(v);
            let (u2, e12, e22, e32) = (sq(u), sq(e1), sq(e2), sq(e3));
            let (u3, u4) = (gf.mul(u2, u), sq(u2));
            let affine = sq(e32)
                ^ gf.mul(u4, sq(e22))
                ^ gf.mul(sq(b), sq(e12))
                ^ gf.mul(sq(b), u4)
                ^ gf.mul(b ^ u4, e32)
                ^ gf.mul(gf.mul(b, u2), e22)
                ^ gf.mul(gf.mul(b, u3), e3);
            let rest = gf.mul(u, gf.mul(e32, e3))
                ^ gf.mul(u3, gf.mul(e22, e3))
                ^ gf.mul(u2, gf.mul(e12, e32))
                ^ gf.mul(gf.mul(b, u), gf.mul(e12, e3));
            (affine, rest)
        };
        let mut rng = StdRng::seed_from_u64(cell_seed(seed, 3, n, d));
        let mut ctl = StdRng::seed_from_u64(cell_seed(seed ^ 0x5a5a, 3, n, d));
        let (mut witnesses, mut zero, mut ctl_zero, mut rest_nonzero) = (0u64, 0u64, 0u64, 0u64);
        for _ in 0..pool {
            let k = rng.gen_range(1..cell.r);
            let target = cell.target(k);
            let r = cell.inst.fast.lift(&target);
            let mut c = linear21::Counts::default();
            let w = lin
                .decide(r, linear21::Variant::HalfTrace, &mut c, || false)
                .unwrap();
            let Some(w) = w else { continue };
            witnesses += 1;
            let x = [xs_base[w[0]], xs_base[w[1]], xs_base[w[2]]];
            let (aff, rest) = s4(x, r.x);
            zero += u64::from(aff ^ rest == 0);
            rest_nonzero += u64::from(rest != 0);
            let mut y = x;
            loop {
                let alt = xs_base[ctl.gen_range(0..xs_base.len())];
                if alt != x[2] {
                    y[2] = alt;
                    break;
                }
            }
            let (aff, rest) = s4(y, r.x);
            ctl_zero += u64::from(aff ^ rest == 0);
        }
        eprintln!(
            "s4check n={n} d={d}: witnesses {witnesses}, S4 = 0 on {zero}, control zeros {ctl_zero}, non-affine part non-zero on {rest_nonzero}"
        );
        recs.push(json!({"a": a, "n": n, "d": d, "pool": pool, "seed": seed,
            "witnesses": witnesses, "s4_zero": zero, "control_zero": ctl_zero,
            "nonaffine_part_nonzero": rest_nonzero}));
    }
    std::fs::write(&out, serde_json::to_string_pretty(&json!({"cmd": "s4check", "cells": recs})).unwrap())
        .unwrap();
}

// ── linsummary: the pool-level table of the linear arm against the floor ──

fn ols(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    sxy / sxx
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// Reads every `linpool-*.json` of `--dir` and, where present, the
/// `truth-*.json` of the same cell; writes `--out` (JSON) and a Markdown
/// table beside it.  Per cell, over the whole pool: cost per relation
/// (`Σ wall / #decomposable`) of the linear arm and of the floor, their
/// ratio with a paired bootstrap over targets, the counted units (trials
/// and group additions per relation), and peak memory.  Across the cells
/// that have a floor: OLS slopes of `log₂` counted work per relation on
/// `n`, and their difference, bootstrapped the same way.
fn cmd_linsummary(args: &[String]) {
    let dir = PathBuf::from(flag(args, "--dir").expect("--dir DIR"));
    let out = PathBuf::from(flag(args, "--out").expect("--out SUMMARY.json"));
    let boot: usize = num(args, "--boot", 2000);
    let seed: u64 = num(args, "--seed", 20261113);
    let arm = flag(args, "--arm").unwrap_or_else(|| "lin21-ht".into());
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.starts_with(&format!("linpool-{arm}-m3-")) && n.ends_with(".json")
        })
        .collect();
    files.sort();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut cells = Vec::new();
    // Per floor cell: n and the bootstrap replicates of (log2 trials/rel, log2 adds/rel).
    let mut slope_cells: Vec<(f64, f64, f64, Vec<(f64, f64)>)> = Vec::new();
    for f in &files {
        let lp: Value = serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap();
        let (n, d) = (lp["n"].as_u64().unwrap(), lp["d"].as_u64().unwrap());
        let rows = lp["rows"].as_array().unwrap();
        let pool = rows.len();
        let cols = lp["fb_columns"].as_f64().unwrap();
        let lin_ns: Vec<f64> = rows.iter().map(|r| r["ns"].as_f64().unwrap()).collect();
        let lin_tr: Vec<f64> = rows.iter().map(|r| r["trials"].as_f64().unwrap()).collect();
        let yes: Vec<bool> = rows.iter().map(|r| r["yes"].as_bool().unwrap()).collect();
        let dec = yes.iter().filter(|&&y| y).count() as f64;
        let setup = lp["setup_ns"].as_f64().unwrap();
        let lin_online = lin_ns.iter().sum::<f64>() / dec;
        let lin_trials = lin_tr.iter().sum::<f64>() / dec;
        let mut cell = json!({
            "n": n, "d": d, "a": lp["a"], "curve": lp["curve"], "r": lp["r"],
            "fb_points": lp["fb_points"], "pair_table_entries": lp["pair_table_entries"],
            "pool": pool, "decomposable": dec, "p": dec / pool as f64,
            "lin_ns_per_relation_online": lin_online,
            "lin_ns_per_relation_amortized": lin_online + setup / cols,
            "lin_trials_per_relation": lin_trials,
            "lin_ns_per_trial": lin_ns.iter().sum::<f64>() / lin_tr.iter().sum::<f64>(),
            "lin_setup_ns": setup, "lin_vm_hwm_kb": lp["vm_hwm_kb"],
            "floor": Value::Null,
        });
        let tpath = dir.join(format!("truth-m3-n{n}-d{d}.json"));
        if let Ok(txt) = std::fs::read_to_string(&tpath) {
            let tr: Value = serde_json::from_str(&txt).unwrap();
            let trows = tr["rows"].as_array().unwrap();
            assert_eq!(trows.len(), pool, "pool sizes differ at n={n}");
            let fl_ns: Vec<f64> = trows.iter().map(|r| r["ns"].as_f64().unwrap()).collect();
            let fl_ad: Vec<f64> = trows.iter().map(|r| r["adds"].as_f64().unwrap()).collect();
            let disagree = (0..pool)
                .filter(|&i| {
                    assert_eq!(trows[i]["k"], rows[i]["k"]);
                    trows[i]["yes"].as_bool().unwrap() != yes[i]
                })
                .count();
            let build = tr["table_build_ns"].as_f64().unwrap();
            let fl_online = fl_ns.iter().sum::<f64>() / dec;
            let fl_adds = fl_ad.iter().sum::<f64>() / dec;
            // Paired bootstrap over targets.
            let (mut ro, mut ra, mut reps) = (Vec::new(), Vec::new(), Vec::new());
            for _ in 0..boot {
                let (mut l, mut fl, mut t, mut ad, mut dy) = (0.0, 0.0, 0.0, 0.0, 0.0);
                for _ in 0..pool {
                    let i = rng.gen_range(0..pool);
                    l += lin_ns[i];
                    fl += fl_ns[i];
                    t += lin_tr[i];
                    ad += fl_ad[i];
                    dy += f64::from(u8::from(yes[i]));
                }
                if dy == 0.0 {
                    continue;
                }
                ro.push(l / fl);
                ra.push((l / dy + setup / cols) / (fl / dy + build / cols));
                reps.push(((t / dy).log2(), (ad / dy).log2()));
            }
            cell["floor"] = json!({
                "disagreements": disagree,
                "ns_per_relation_online": fl_online,
                "ns_per_relation_amortized": fl_online + build / cols,
                "adds_per_relation_online": fl_adds,
                "table_build_ns": build, "vm_hwm_kb": tr["vm_hwm_kb"],
                "lin_over_floor_online": lin_online / fl_online,
                "lin_over_floor_online_ci95": [pct(&mut ro, 0.025), pct(&mut ro, 0.975)],
                "lin_over_floor_amortized": (lin_online + setup / cols) / (fl_online + build / cols),
                "lin_over_floor_amortized_ci95": [pct(&mut ra, 0.025), pct(&mut ra, 0.975)],
                "trials_over_adds": lin_trials / fl_adds,
            });
            slope_cells.push((n as f64, lin_trials.log2(), fl_adds.log2(), reps));
        }
        cells.push(cell);
    }
    let xs: Vec<f64> = slope_cells.iter().map(|c| c.0).collect();
    let slopes = if xs.len() >= 3 {
        let lt = ols(&xs, &slope_cells.iter().map(|c| c.1).collect::<Vec<_>>());
        let fa = ols(&xs, &slope_cells.iter().map(|c| c.2).collect::<Vec<_>>());
        let nrep = slope_cells.iter().map(|c| c.3.len()).min().unwrap();
        let (mut dl, mut sl, mut sf) = (Vec::new(), Vec::new(), Vec::new());
        for b in 0..nrep {
            let a = ols(
                &xs,
                &slope_cells.iter().map(|c| c.3[b].0).collect::<Vec<_>>(),
            );
            let f = ols(
                &xs,
                &slope_cells.iter().map(|c| c.3[b].1).collect::<Vec<_>>(),
            );
            sl.push(a);
            sf.push(f);
            dl.push(a - f);
        }
        json!({
            "cells_n": xs, "unit": "log2 counted work per relation per unit n",
            "lin_trials_slope": lt, "lin_trials_slope_ci95": [pct(&mut sl, 0.025), pct(&mut sl, 0.975)],
            "floor_adds_slope": fa, "floor_adds_slope_ci95": [pct(&mut sf, 0.025), pct(&mut sf, 0.975)],
            "delta": lt - fa, "delta_ci95": [pct(&mut dl, 0.025), pct(&mut dl, 0.975)],
        })
    } else {
        Value::Null
    };
    let summary = json!({"cmd": "linsummary", "arm": arm, "boot": boot, "seed": seed,
        "cells": cells, "counted_slopes": slopes});
    std::fs::write(&out, serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    let mut md = String::from(
        "| n | d | \\|F\\| | p | floor ns/rel (online) | lin ns/rel (online) | lin ÷ floor online [95%] | lin ÷ floor amortized [95%] | trials ÷ adds | ns/trial | floor peak MB | lin peak MB | disagreements |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n",
    );
    for c in summary["cells"].as_array().unwrap() {
        let f = &c["floor"];
        let g = |v: &Value| v.as_f64().map(|x| format!("{x:.3e}")).unwrap_or("—".into());
        let ci = |v: &Value, k: &str| match (v[k].as_f64(), v[format!("{k}_ci95")].as_array()) {
            (Some(x), Some(c)) => format!(
                "{x:.2} [{:.2}, {:.2}]",
                c[0].as_f64().unwrap(),
                c[1].as_f64().unwrap()
            ),
            _ => "—".into(),
        };
        md.push_str(&format!(
            "| {} | {} | {} | {:.3} | {} | {} | {} | {} | {} | {:.0} | {} | {:.1} | {} |\n",
            c["n"],
            c["d"],
            c["fb_points"],
            c["p"].as_f64().unwrap(),
            g(&f["ns_per_relation_online"]),
            g(&c["lin_ns_per_relation_online"]),
            ci(f, "lin_over_floor_online"),
            ci(f, "lin_over_floor_amortized"),
            f["trials_over_adds"]
                .as_f64()
                .map(|x| format!("{x:.3}"))
                .unwrap_or("—".into()),
            c["lin_ns_per_trial"].as_f64().unwrap(),
            f["vm_hwm_kb"]
                .as_f64()
                .map(|x| format!("{:.1}", x / 1e3))
                .unwrap_or("—".into()),
            c["lin_vm_hwm_kb"].as_f64().unwrap() / 1e3,
            f["disagreements"]
                .as_u64()
                .map(|x| x.to_string())
                .unwrap_or("—".into()),
        ));
    }
    md.push_str(&format!(
        "\nCounted slopes: `{}`\n",
        summary["counted_slopes"]
    ));
    std::fs::write(out.with_extension("md"), &md).unwrap();
    print!("{md}");
}

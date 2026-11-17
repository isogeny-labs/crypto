//! Solving degree at **fixed surplus**: the `m = 3` ladder
//! `RESEARCH_DESCENT_CROSSOVER.md` §7 asks for.
//!
//! `dreg_sweep` takes `ℓ` from the Frobenius-invariant subspaces, so its
//! surplus `S = n − 3ℓ` jumps with `n`.  This takes each cell's `(n, ℓ)`
//! explicitly, over random subspaces, so pairs of cells can share `S`.
//! Every draw's solutions are counted exactly first; only draws with none
//! are measured, so a degree that does not resolve on a fully built matrix
//! is a lower bound on the refutation degree and not a satisfiable system.
//!
//! ```sh
//! F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 \
//!   cargo run --release --example dreg_ladder -- \
//!     --cells 7:3:7,13:5:6 --unsat 4 --ffd-max 5 --controls 1 --seed 20260923
//! ```
//!
//! Each cell's draws come from their own seed (`seed`, `n`, `ℓ`), so a cell
//! reproduces run alone or in any order.  One JSON line per draw on stdout;
//! the summary table on stderr at the end.
//!
//! `--m M` (default 3) measures the chained system with `M` summands:
//! `M − 2` intermediate points, `(M − 2)·n + M·ℓ` unknowns, surplus
//! `n − M·ℓ`.  At `M = 3` everything is as before; otherwise the cell seed is
//! offset by `M << 48`, the JSON line carries `"m"`, and controls are off
//! (their density is defined for `M = 3`).
//!
//! `--min-sat K` keeps walking the same deterministic stream after the
//! requested UNSAT quota until at least `K` natural satisfiable draws have
//! also been emitted (or `--max-draws` is reached).  Extra UNSAT draws seen
//! while waiting are emitted and retained but are not added to the requested
//! UNSAT panel.  This lets a preregistered stream supply natural soundness
//! controls without changing seeds or selecting on a later closure outcome.
//!
//! `--audit-jsonl FILE` independently replays a committed complete stream
//! prefix and compares every generated field except elapsed time.  Audit mode
//! requires explicit `--expect-m`, `--expect-n`, `--expect-lambda`,
//! `--expect-seed`, `--expect-count`, `--expect-d-max`, and
//! `--expect-ffd-max` values, so no scientific parameter is inferred from the
//! file being audited.
//!
//! `--shard I/J` (with `--first-draw F`, default 0) measures a fixed draw
//! range in parallel processes: process `I` of `J` walks the cell's one
//! sequential stream to `--max-draws`, sampling every draw (cheap) and
//! measuring only the draws `k ≥ F` with `k mod J = I`.  `--unsat` does not
//! stop it.  Each line is the sequential run's line for that draw (same
//! `draw` index, same fields), so the `J` outputs sorted by `draw` are the
//! sequential file from `F` on, up to `secs`.

use crypto_lib::cryptanalysis::koblitz_bench::{
    ladder_control, ladder_draw_m, ladder_measure_m, ladder_sample, LadderDraw, LadderOutcome,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn outcome_json(o: &LadderOutcome) -> String {
    match o {
        LadderOutcome::Satisfiable => r#"{"kind":"satisfiable"}"#.into(),
        LadderOutcome::Resolved { degree, refuted } => {
            format!(r#"{{"kind":"resolved","degree":{degree},"refuted":{refuted}}}"#)
        }
        LadderOutcome::AtLeast(d) => format!(r#"{{"kind":"at_least","degree":{d}}}"#),
        LadderOutcome::CapsHit { built } => format!(
            r#"{{"kind":"caps_hit","built":{}}}"#,
            built.map_or("null".to_string(), |b| b.to_string())
        ),
    }
}

fn outcome_short(o: &LadderOutcome) -> String {
    match o {
        LadderOutcome::Satisfiable => "sat".into(),
        LadderOutcome::Resolved {
            degree,
            refuted: true,
        } => format!("{degree}"),
        LadderOutcome::Resolved {
            degree,
            refuted: false,
        } => format!("{degree}p"),
        LadderOutcome::AtLeast(d) => format!("≥{d}"),
        LadderOutcome::CapsHit { built } => {
            format!("caps@{}", built.map_or("-".into(), |b| b.to_string()))
        }
    }
}

fn outcome_value(o: &LadderOutcome) -> Value {
    match o {
        LadderOutcome::Satisfiable => json!({"kind": "satisfiable"}),
        LadderOutcome::Resolved { degree, refuted } => {
            json!({"kind": "resolved", "degree": degree, "refuted": refuted})
        }
        LadderOutcome::AtLeast(degree) => json!({"kind": "at_least", "degree": degree}),
        LadderOutcome::CapsHit { built } => json!({"kind": "caps_hit", "built": built}),
    }
}

#[allow(clippy::too_many_arguments)]
fn expected_draw_value(
    draw: &LadderDraw,
    draw_index: usize,
    m: usize,
    n: u32,
    ell: usize,
    d_max: u32,
) -> Value {
    let mut value = json!({
        "cell": cell_name(n, ell, m),
        "n": n,
        "ell": ell,
        "surplus": i64::from(n) - (m * ell) as i64,
        "d_max": d_max,
        "draw": draw_index,
        "n_vars": draw.n_vars,
        "n_eqs": draw.n_eqs,
        "v_basis": draw.v_basis,
        "x_r": draw.x_r,
        "solutions": draw.solutions,
        "outcome": outcome_value(&draw.outcome),
        "ffd": draw.ffd,
    });
    if m != 3 {
        value
            .as_object_mut()
            .expect("record is an object")
            .insert("m".into(), Value::from(m));
    }
    value
}

/// Independently replay and verify a generator JSONL.  Every record is the
/// next draw in the cell's deterministic RNG stream, and every scientific
/// field is compared with a fresh [`ladder_draw_m`] result.  Only `secs` is
/// excluded: it is required to be a finite nonnegative number, but its value
/// is deliberately not compared.
#[allow(clippy::too_many_arguments)]
fn audit_jsonl(
    path: &str,
    m: usize,
    n: u32,
    ell: usize,
    seed: u64,
    expected_count: usize,
    d_max: u32,
    ffd_max: u32,
) -> Result<(), String> {
    if m < 3 {
        return Err("--expect-m must be at least 3".into());
    }
    if ell == 0 || ell as u32 >= n || n >= 64 {
        return Err("expected cell must satisfy 1 <= lambda < n < 64".into());
    }
    if expected_count == 0 {
        return Err("--expect-count must be positive".into());
    }
    if d_max == 0 || ffd_max == 0 {
        return Err("expected degree bounds must be positive".into());
    }

    let file = File::open(path).map_err(|e| format!("open audit JSONL {path}: {e}"))?;
    let mut rng = StdRng::seed_from_u64(cell_seed(seed, n, ell, m));
    let mut seen = 0usize;
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line_number = line_index + 1;
        let line = line.map_err(|e| format!("read audit JSONL line {line_number}: {e}"))?;
        if line.trim().is_empty() {
            return Err(format!("blank audit JSONL line {line_number}"));
        }
        if seen >= expected_count {
            return Err(format!(
                "extra audit JSONL record at line {line_number}; expected exactly {expected_count}"
            ));
        }
        let mut actual: Value = serde_json::from_str(&line)
            .map_err(|e| format!("malformed audit JSONL line {line_number}: {e}"))?;
        let object = actual
            .as_object_mut()
            .ok_or_else(|| format!("audit JSONL line {line_number} is not an object"))?;
        let secs = object
            .remove("secs")
            .ok_or_else(|| format!("audit JSONL line {line_number} lacks secs"))?;
        secs.as_f64()
            .filter(|secs| secs.is_finite() && *secs >= 0.0)
            .ok_or_else(|| format!("audit JSONL line {line_number} has invalid non-timing secs"))?;

        let draw = ladder_draw_m(m, n, ell, d_max, ffd_max, &mut rng)
            .ok_or_else(|| format!("could not rebuild draw {seen} at line {line_number}"))?;
        let expected = expected_draw_value(&draw, seen, m, n, ell, d_max);
        if actual != expected {
            return Err(format!(
                "audit JSONL line {line_number} differs from deterministic draw {seen}\nexpected: {expected}\nactual:   {actual}"
            ));
        }
        seen += 1;
    }
    if seen != expected_count {
        return Err(format!(
            "audit JSONL ended after {seen} records; expected exactly {expected_count}"
        ));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let num = |name: &str, default: u64| flag(name).and_then(|v| v.parse().ok()).unwrap_or(default);
    if let Some(path) = flag("--audit-jsonl") {
        let required = |name: &str| -> Result<u64, String> {
            let value = flag(name).ok_or_else(|| format!("audit mode requires {name}"))?;
            value
                .parse()
                .map_err(|_| format!("audit mode requires an integer {name}"))
        };
        let result = (|| {
            let m = usize::try_from(required("--expect-m")?)
                .map_err(|_| "--expect-m does not fit usize".to_string())?;
            let n = u32::try_from(required("--expect-n")?)
                .map_err(|_| "--expect-n does not fit u32".to_string())?;
            let ell = usize::try_from(required("--expect-lambda")?)
                .map_err(|_| "--expect-lambda does not fit usize".to_string())?;
            let seed = required("--expect-seed")?;
            let count = usize::try_from(required("--expect-count")?)
                .map_err(|_| "--expect-count does not fit usize".to_string())?;
            let d_max = u32::try_from(required("--expect-d-max")?)
                .map_err(|_| "--expect-d-max does not fit u32".to_string())?;
            let ffd_max = u32::try_from(required("--expect-ffd-max")?)
                .map_err(|_| "--expect-ffd-max does not fit u32".to_string())?;
            audit_jsonl(&path, m, n, ell, seed, count, d_max, ffd_max)
        })();
        match result {
            Ok(()) => {
                eprintln!("deterministic draw audit passed: {path}");
                return;
            }
            Err(error) => {
                eprintln!("deterministic draw audit failed: {error}");
                std::process::exit(65);
            }
        }
    }
    let cells: Vec<(u32, usize, u32)> = flag("--cells")
        .unwrap_or_else(|| "7:3:7".into())
        .split(',')
        .map(|c| {
            let p: Vec<u64> = c
                .split(':')
                .map(|x| x.parse().expect("cell is n:ell:d_max"))
                .collect();
            assert_eq!(p.len(), 3, "cell is n:ell:d_max");
            (p[0] as u32, p[1] as usize, p[2] as u32)
        })
        .collect();
    let want_unsat = num("--unsat", 4) as usize;
    let min_sat = num("--min-sat", 0) as usize;
    let max_draws = num("--max-draws", 4096) as usize;
    let ffd_max = num("--ffd-max", 5) as u32;
    let controls = num("--controls", 0) as usize;
    // `--d-min D` (with `--unsat-index`): start the degree scan at D.  Only
    // for a draw whose degrees below D are already measured not to resolve,
    // in a committed run of the same draw; the JSON line records `d_min`.
    let d_min = num("--d-min", 1) as u32;
    let seed = num("--seed", 0x5EED);
    let m = num("--m", 3) as usize;
    assert!(m >= 3, "--m is at least 3 (the chained system)");

    // `--unsat-index K`: measure only the cell's K-th unsatisfiable draw
    // (from 0), replaying the draws before it through the exact solution
    // count alone.  `--control-only`: run only the cell's control.  Both give
    // what the sequential run gives at that position, from the same seed, so a
    // cell can be measured one process per draw and a restart loses only the
    // draws in flight.
    let only_unsat = flag("--unsat-index").map(|v| v.parse::<usize>().expect("--unsat-index K"));
    let control_only = args.iter().any(|a| a == "--control-only");
    assert!(
        (only_unsat.is_none() && !control_only) || min_sat == 0,
        "--min-sat applies only to a complete sequential cell stream"
    );
    assert!(
        m == 3 || (controls == 0 && !control_only),
        "controls are defined for m = 3 only"
    );
    let shard: Option<(usize, usize)> = flag("--shard").map(|v| {
        let (i, j) = v.split_once('/').expect("--shard I/J");
        let (i, j) = (i.parse().expect("I"), j.parse().expect("J"));
        assert!(j >= 1 && i < j, "--shard I/J with 0 <= I < J");
        (i, j)
    });
    if let Some((i, j)) = shard {
        assert_eq!(cells.len(), 1, "one cell per process in this mode");
        assert!(only_unsat.is_none() && !control_only && controls == 0 && min_sat == 0);
        let (n, ell, d_max) = cells[0];
        let first = num("--first-draw", 0) as usize;
        shard_run(
            m,
            n,
            ell,
            d_max,
            ffd_max,
            cell_seed(seed, n, ell, m),
            first,
            max_draws,
            i,
            j,
        );
        return;
    }
    if only_unsat.is_some() || control_only {
        assert_eq!(cells.len(), 1, "one cell per process in this mode");
        let (n, ell, d_max) = cells[0];
        let cell_seed = cell_seed(seed, n, ell, m);
        if let Some(k) = only_unsat {
            measure_one(m, n, ell, d_min, d_max, ffd_max, cell_seed, k, max_draws);
        }
        if control_only {
            let n_vars = 3 * ell + n as usize;
            for t in 0..controls.max(1) {
                let terms = ladder_terms_per_eq(
                    n,
                    ell,
                    cell_seed ^ 0x0C01_7201_u64.wrapping_mul(t as u64 + 1),
                );
                let started = std::time::Instant::now();
                let o = ladder_control(
                    n_vars,
                    3,
                    terms,
                    d_max,
                    cell_seed
                        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                        .wrapping_add(t as u64),
                );
                println!(
                    r#"{{"cell":"n{n}l{ell}","control":{t},"n_vars":{n_vars},"n_eqs":{},"terms_per_eq":{terms},"d_max":{d_max},"outcome":{},"secs":{:.3}}}"#,
                    n_vars + 4,
                    outcome_json(&o),
                    started.elapsed().as_secs_f64()
                );
                eprintln!("n={n} ℓ={ell}: control {t} -> {}", outcome_short(&o));
            }
        }
        return;
    }

    let mut table = Vec::new();
    for &(n, ell, d_max) in &cells {
        let cell_seed = cell_seed(seed, n, ell, m);
        let mut rng = StdRng::seed_from_u64(cell_seed);
        let mut unsat: Vec<LadderDraw> = Vec::new();
        let (mut drawn, mut sat) = (0usize, 0usize);
        let mut shape = None;
        while (unsat.len() < want_unsat || sat < min_sat) && drawn < max_draws {
            let d = ladder_draw_m(m, n, ell, d_max, ffd_max, &mut rng).expect("cell builds");
            drawn += 1;
            shape = Some((d.n_vars, d.n_eqs));
            println!(
                r#"{{"cell":"{}","n":{n},"ell":{ell},"surplus":{},"d_max":{d_max},"draw":{},"n_vars":{},"n_eqs":{},"v_basis":{:?},"x_r":{},"solutions":{},"outcome":{},"ffd":{},"secs":{:.3}{}}}"#,
                cell_name(n, ell, m),
                n as i64 - (m * ell) as i64,
                drawn - 1,
                d.n_vars,
                d.n_eqs,
                d.v_basis,
                d.x_r,
                d.solutions,
                outcome_json(&d.outcome),
                d.ffd.map_or("null".to_string(), |f| f.to_string()),
                d.secs,
                m_json(m)
            );
            if d.outcome == LadderOutcome::Satisfiable {
                sat += 1;
            } else if unsat.len() < want_unsat {
                eprintln!(
                    "n={n} ℓ={ell}: unsat draw {} -> {} ({:.1}s)",
                    unsat.len(),
                    outcome_short(&d.outcome),
                    d.secs
                );
                unsat.push(d);
            } else {
                eprintln!(
                    "n={n} ℓ={ell}: retained extra unsat draw {} while awaiting natural SAT control",
                    drawn - 1
                );
            }
        }
        let (n_vars, n_eqs) = shape.unwrap_or((0, 0));
        let mut ctrl = Vec::new();
        for t in 0..controls {
            // Degree 3 and the systems' own term density, as `dreg_summary` does.
            let terms = ladder_terms_per_eq(
                n,
                ell,
                cell_seed ^ 0x0C01_7201_u64.wrapping_mul(t as u64 + 1),
            );
            let started = std::time::Instant::now();
            let o = ladder_control(
                n_vars,
                3,
                terms,
                d_max,
                cell_seed
                    .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    .wrapping_add(t as u64),
            );
            println!(
                r#"{{"cell":"n{n}l{ell}","control":{t},"n_vars":{n_vars},"n_eqs":{},"terms_per_eq":{terms},"d_max":{d_max},"outcome":{},"secs":{:.3}}}"#,
                n_vars + 4,
                outcome_json(&o),
                started.elapsed().as_secs_f64()
            );
            eprintln!("n={n} ℓ={ell}: control {t} -> {}", outcome_short(&o));
            ctrl.push(o);
        }
        table.push((n, ell, d_max, n_vars, n_eqs, drawn, sat, unsat, ctrl));
    }

    eprintln!();
    eprintln!("| n | ℓ | S | vars | eqs | d_max | draws | sat | unsat outcomes | FFD | control |");
    eprintln!("|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|:--|");
    for (n, ell, d_max, n_vars, n_eqs, drawn, sat, unsat, ctrl) in &table {
        let outs: Vec<String> = unsat.iter().map(|d| outcome_short(&d.outcome)).collect();
        let ffds: Vec<String> = unsat
            .iter()
            .map(|d| d.ffd.map_or(">".into(), |f| f.to_string()))
            .collect();
        let cs: Vec<String> = ctrl.iter().map(outcome_short).collect();
        eprintln!(
            "| {n} | {ell} | {} | {n_vars} | {n_eqs} | {d_max} | {drawn} | {sat} | {} | {} | {} |",
            *n as i64 - (m * *ell) as i64,
            outs.join(" "),
            ffds.join(" "),
            if cs.is_empty() {
                "-".into()
            } else {
                cs.join(" ")
            }
        );
    }
}

/// `--shard I/J`: the draws `first ≤ k < max_draws` with `k mod J = I`,
/// each measured exactly as the sequential loop measures it.  The stream is
/// sampled for every draw so the measured ones sit where the sequential run
/// puts them; [`ladder_draw_m`] is [`ladder_sample`] then
/// [`ladder_measure_m`] from degree 1, and the measurement takes no
/// randomness.
#[allow(clippy::too_many_arguments)]
fn shard_run(
    m: usize,
    n: u32,
    ell: usize,
    d_max: u32,
    ffd_max: u32,
    cell_seed: u64,
    first: usize,
    max_draws: usize,
    i: usize,
    j: usize,
) {
    let mut rng = StdRng::seed_from_u64(cell_seed);
    for drawn in 0..max_draws {
        let (basis, x_r) = ladder_sample(n, ell, &mut rng);
        if drawn < first || drawn % j != i {
            continue;
        }
        let d = ladder_measure_m(m, n, &basis, &x_r, 1, d_max, ffd_max).expect("cell builds");
        println!(
            r#"{{"cell":"{}","n":{n},"ell":{ell},"surplus":{},"d_max":{d_max},"draw":{drawn},"n_vars":{},"n_eqs":{},"v_basis":{:?},"x_r":{},"solutions":{},"outcome":{},"ffd":{},"secs":{:.3}{}}}"#,
            cell_name(n, ell, m),
            n as i64 - (m * ell) as i64,
            d.n_vars,
            d.n_eqs,
            d.v_basis,
            d.x_r,
            d.solutions,
            outcome_json(&d.outcome),
            d.ffd.map_or("null".to_string(), |f| f.to_string()),
            d.secs,
            m_json(m)
        );
        if d.outcome != LadderOutcome::Satisfiable {
            eprintln!(
                "n={n} ℓ={ell}: shard {i}/{j} draw {drawn} unsatisfiable -> {} ({:.1}s)",
                outcome_short(&d.outcome),
                d.secs
            );
        }
    }
}

/// A cell's seed: `seed`, `n` and `ℓ` as before; `m` enters only when it is
/// not 3, so every `m = 3` cell keeps the seed it was committed with.
fn cell_seed(seed: u64, n: u32, ell: usize, m: usize) -> u64 {
    let base = seed ^ (u64::from(n) << 40) ^ ((ell as u64) << 32);
    if m == 3 {
        base
    } else {
        base ^ ((m as u64) << 48)
    }
}

fn cell_name(n: u32, ell: usize, m: usize) -> String {
    if m == 3 {
        format!("n{n}l{ell}")
    } else {
        format!("n{n}l{ell}m{m}")
    }
}

/// The JSON field naming `m`, empty at `m = 3` so those lines are unchanged.
fn m_json(m: usize) -> String {
    if m == 3 {
        String::new()
    } else {
        format!(r#","m":{m}"#)
    }
}

/// This process's peak resident set (`VmHWM`, kB), where `/proc` has it.
fn peak_rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

/// The cell's `k`-th unsatisfiable draw, measured; the draws before it only
/// counted.  Prints the draw's JSON line exactly as the sequential run does.
#[allow(clippy::too_many_arguments)]
fn measure_one(
    m: usize,
    n: u32,
    ell: usize,
    d_min: u32,
    d_max: u32,
    ffd_max: u32,
    cell_seed: u64,
    k: usize,
    max_draws: usize,
) {
    use crypto_lib::binary_ecc::F2mElement;
    use crypto_lib::cryptanalysis::koblitz_bench::{
        chained_s3_solution_count, chained_solution_count, ladder_measure_m, ladder_sample,
    };
    use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    let irr = find_irreducible_sparse(n).expect("field");
    let b = F2mElement::one(n);
    let mut rng = StdRng::seed_from_u64(cell_seed);
    let mut unsat_seen = 0usize;
    for drawn in 0..max_draws {
        let (basis, x_r) = ladder_sample(n, ell, &mut rng);
        let solutions = if m == 3 {
            chained_s3_solution_count(&basis, &x_r, &b, &irr)
        } else {
            chained_solution_count(m, &basis, &x_r, &b, &irr)
        };
        if solutions > 0 {
            continue;
        }
        if unsat_seen < k {
            unsat_seen += 1;
            continue;
        }
        let d = ladder_measure_m(m, n, &basis, &x_r, d_min, d_max, ffd_max).expect("cell builds");
        println!(
            r#"{{"cell":"{}","n":{n},"ell":{ell},"surplus":{},"d_max":{d_max},"draw":{drawn},"n_vars":{},"n_eqs":{},"v_basis":{:?},"x_r":{},"solutions":{},"outcome":{},"ffd":{},"secs":{:.3},"unsat_index":{k}{}{}}}"#,
            cell_name(n, ell, m),
            n as i64 - (m * ell) as i64,
            d.n_vars,
            d.n_eqs,
            d.v_basis,
            d.x_r,
            d.solutions,
            outcome_json(&d.outcome),
            d.ffd.map_or("null".to_string(), |f| f.to_string()),
            d.secs,
            if d_min > 1 {
                format!(r#","d_min":{d_min}"#)
            } else {
                String::new()
            },
            m_json(m)
        );
        eprintln!(
            "n={n} ℓ={ell}: unsat draw {k} (draw {drawn}) -> {} ({:.1}s)",
            outcome_short(&d.outcome),
            d.secs
        );
        if let Some(kb) = peak_rss_kb() {
            eprintln!("peak_rss_kB {kb}");
        }
        return;
    }
    eprintln!("n={n} ℓ={ell}: no unsatisfiable draw {k} within {max_draws} draws");
}

/// Mean terms per equation of one fresh draw of the cell, for the control's
/// density -- the same statistic `dreg_summary` feeds its control.
fn ladder_terms_per_eq(n: u32, ell: usize, seed: u64) -> usize {
    use crypto_lib::binary_ecc::F2mElement;
    use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
    use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
    use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use num_bigint::BigUint;
    use rand::Rng;
    let mut rng = StdRng::seed_from_u64(seed);
    let irr = find_irreducible_sparse(n).unwrap();
    let st = FieldStructure::new(n, &irr);
    let basis = random_subspace_basis(n, ell, &mut rng);
    let x_r = F2mElement::from_biguint(&BigUint::from(rng.gen::<u64>() & ((1u64 << n) - 1)), n);
    let sys = build_decomposition_system(&basis, &x_r, &F2mElement::one(n), 3, &st).unwrap();
    sys.equations.iter().map(|e| e.terms.len()).sum::<usize>() / sys.equations.len().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_jsonl(label: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "dreg_ladder_{label}_{}_{}.jsonl",
            std::process::id(),
            nonce
        ))
    }

    #[test]
    fn deterministic_audit_rejects_a_changed_scientific_field() {
        let (m, n, ell, seed, d_max, ffd_max) = (3, 5, 1, 77, 1, 1);
        let mut rng = StdRng::seed_from_u64(cell_seed(seed, n, ell, m));
        let draw = ladder_draw_m(m, n, ell, d_max, ffd_max, &mut rng).expect("small draw");
        let mut record = expected_draw_value(&draw, 0, m, n, ell, d_max);
        record
            .as_object_mut()
            .expect("record object")
            .insert("secs".into(), json!(0.0));
        let path = temporary_jsonl("changed_field");
        std::fs::write(&path, format!("{record}\n")).expect("write valid fixture");
        assert!(audit_jsonl(
            path.to_str().expect("UTF-8 path"),
            m,
            n,
            ell,
            seed,
            1,
            d_max,
            ffd_max
        )
        .is_ok());

        record
            .as_object_mut()
            .expect("record object")
            .insert("solutions".into(), Value::from(draw.solutions + 1));
        std::fs::write(&path, format!("{record}\n")).expect("write changed fixture");
        let error = audit_jsonl(
            path.to_str().expect("UTF-8 path"),
            m,
            n,
            ell,
            seed,
            1,
            d_max,
            ffd_max,
        )
        .expect_err("changed solution count must fail");
        assert!(error.contains("differs from deterministic draw"));
        std::fs::remove_file(path).expect("remove fixture");
    }

    #[test]
    fn deterministic_audit_rejects_blank_records() {
        let path = temporary_jsonl("blank");
        std::fs::write(&path, "\n").expect("write blank fixture");
        let error = audit_jsonl(path.to_str().expect("UTF-8 path"), 3, 5, 1, 77, 1, 1, 1)
            .expect_err("blank record must fail");
        assert!(error.contains("blank audit JSONL line"));
        std::fs::remove_file(path).expect("remove fixture");
    }
}

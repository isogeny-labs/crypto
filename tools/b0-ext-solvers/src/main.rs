//! B0 external SAT arms. Reads a DIMACS file (CryptoMiniSat extended format with
//! `x` lines allowed for the `cms` arm; plain CNF for `cadical`), solves it under a
//! conflict budget with one thread, and prints one JSON line:
//! status (sat|unsat|unknown), conflicts, decisions, propagations, wall_ms, and the
//! first `--model-vars` variables of the model as a 0/1 string (empty unless sat).
//! It verifies nothing: the caller checks the model against the algebra.
use std::fs;
use std::time::Instant;

struct Args {
    solver: String,
    file: String,
    budget: i64,
    model_vars: usize,
}

fn parse_args() -> Args {
    let mut a = Args {
        solver: String::new(),
        file: String::new(),
        budget: 1_000_000,
        model_vars: 0,
    };
    let v: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < v.len() {
        match v[i].as_str() {
            "--solver" => {
                a.solver = v[i + 1].clone();
                i += 2;
            }
            "--file" => {
                a.file = v[i + 1].clone();
                i += 2;
            }
            "--budget" => {
                a.budget = v[i + 1].parse().expect("budget");
                i += 2;
            }
            "--model-vars" => {
                a.model_vars = v[i + 1].parse().expect("model-vars");
                i += 2;
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if a.solver.is_empty() || a.file.is_empty() {
        eprintln!("usage: --solver cms|cadical --file F [--budget N] [--model-vars K]");
        std::process::exit(2);
    }
    a
}

struct Instance {
    nvars: usize,
    clauses: Vec<Vec<i32>>,
    xors: Vec<Vec<i32>>,
    empty_clause: bool,
}

fn parse_dimacs(text: &str) -> Instance {
    let mut inst = Instance {
        nvars: 0,
        clauses: Vec::new(),
        xors: Vec::new(),
        empty_clause: false,
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        if line.starts_with('p') {
            let t: Vec<&str> = line.split_whitespace().collect();
            inst.nvars = t[2].parse().expect("nvars");
            continue;
        }
        let (is_xor, body) = if let Some(rest) = line.strip_prefix('x') {
            (true, rest)
        } else {
            (false, line)
        };
        let mut lits: Vec<i32> = Vec::new();
        for tok in body.split_whitespace() {
            let l: i32 = tok.parse().expect("literal");
            if l == 0 {
                break;
            }
            lits.push(l);
            let v = l.unsigned_abs() as usize;
            if v > inst.nvars {
                inst.nvars = v;
            }
        }
        if is_xor {
            inst.xors.push(lits);
        } else if lits.is_empty() {
            inst.empty_clause = true;
        } else {
            inst.clauses.push(lits);
        }
    }
    inst
}

struct Output<'a> {
    status: &'a str,
    conflicts: i64,
    decisions: i64,
    propagations: i64,
    wall_ms: f64,
    model: &'a str,
    solver: &'a str,
    version: &'a str,
    peak_rss_bytes: Option<u64>,
    requested_conflict_budget: i64,
    effective_conflict_budget: i64,
    variables: usize,
    ordinary_clauses: usize,
    xor_rows: usize,
}

fn out(output: Output<'_>) {
    let Output {
        status,
        conflicts,
        decisions,
        propagations,
        wall_ms,
        model,
        solver,
        version,
        peak_rss_bytes,
        requested_conflict_budget,
        effective_conflict_budget,
        variables,
        ordinary_clauses,
        xor_rows,
    } = output;
    let peak_rss = peak_rss_bytes
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_owned());
    println!(
        "{{\"schema\":\"prime31.selector-external-solver-record/v1\",\"solver\":\"{solver}\",\"version\":\"{version}\",\"status\":\"{status}\",\"variables\":{variables},\"ordinary_clauses\":{ordinary_clauses},\"xor_rows\":{xor_rows},\"requested_conflict_budget\":{requested_conflict_budget},\"effective_conflict_budget\":{effective_conflict_budget},\"conflicts\":{conflicts},\"decisions\":{decisions},\"propagations\":{propagations},\"wall_ms\":{wall_ms:.3},\"peak_rss_bytes\":{peak_rss},\"model\":\"{model}\",\"forbidden\":{{\"secret_scalars_consumed\":0,\"witness_indices_or_signs_consumed\":0,\"pair_table_entries_consumed\":0,\"half_table_entries_consumed\":0,\"complete_selector_assignments_enumerated\":0,\"grader_records_consumed_after_target_extraction\":0}},\"pcc\":\"unverified\"}}"
    );
}

fn peak_rss_bytes() -> Option<u64> {
    #[cfg(unix)]
    {
        // SAFETY: `rusage` is plain old data and `getrusage` initializes it.
        let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
            return None;
        }
        let raw = u64::try_from(usage.ru_maxrss).ok()?;
        #[cfg(target_os = "macos")]
        return Some(raw);
        #[cfg(not(target_os = "macos"))]
        return raw.checked_mul(1024);
    }
    #[cfg(not(unix))]
    None
}

fn main() {
    let a = parse_args();
    let text = fs::read_to_string(&a.file).expect("read instance");
    let inst = parse_dimacs(&text);
    match a.solver.as_str() {
        "cms" => {
            use cryptominisat::{Lbool, Lit, Solver};
            let mut s = Solver::new();
            s.set_num_threads(1);
            s.new_vars(inst.nvars);
            let to_lit = |l: i32| Lit::new(l.unsigned_abs() - 1, l < 0).expect("lit");
            if inst.empty_clause {
                s.add_clause(&[]);
            }
            for c in &inst.clauses {
                let ls: Vec<Lit> = c.iter().map(|&l| to_lit(l)).collect();
                s.add_clause(&ls);
            }
            // An `x` line is an XOR of its literals equal to true (CryptoMiniSat convention).
            for x in &inst.xors {
                let ls: Vec<Lit> = x.iter().map(|&l| to_lit(l)).collect();
                s.add_xor_literal_clause(&ls, true);
            }
            s.set_max_confl(a.budget);
            let t = Instant::now();
            let r = s.solve();
            let wall = t.elapsed().as_secs_f64() * 1e3;
            let status = match r {
                Lbool::True => "sat",
                Lbool::False => "unsat",
                Lbool::Undef => "unknown",
            };
            let mut model = String::new();
            if matches!(r, Lbool::True) {
                let m = s.get_model();
                for value in m.iter().take(a.model_vars) {
                    model.push(match value {
                        Lbool::True => '1',
                        Lbool::False => '0',
                        Lbool::Undef => '?',
                    });
                }
            }
            out(Output {
                status,
                conflicts: s.sum_conflicts() as i64,
                decisions: s.sum_decisions() as i64,
                propagations: s.sum_propagations() as i64,
                wall_ms: wall,
                model: &model,
                solver: "cryptominisat",
                version: "5.8.0-rust1",
                peak_rss_bytes: peak_rss_bytes(),
                requested_conflict_budget: a.budget,
                effective_conflict_budget: a.budget,
                variables: inst.nvars,
                ordinary_clauses: inst.clauses.len() + usize::from(inst.empty_clause),
                xor_rows: inst.xors.len(),
            });
        }
        "cadical" => {
            use cadical_sys::{CaDiCal, Status};
            if !inst.xors.is_empty() {
                eprintln!(
                    "cadical arm needs plain CNF; this file has {} xor lines",
                    inst.xors.len()
                );
                std::process::exit(3);
            }
            let mut c = CaDiCal::new();
            if inst.empty_clause {
                c.add(0);
            }
            for cl in &inst.clauses {
                for &l in cl {
                    c.add(l);
                }
                c.add(0);
            }
            let budget: i32 = a.budget.min(i32::MAX as i64) as i32;
            c.limit("conflicts", budget);
            let t = Instant::now();
            let r = c.solve();
            let wall = t.elapsed().as_secs_f64() * 1e3;
            let status = match r {
                Status::SATISFIABLE => "sat",
                Status::UNSATISFIABLE => "unsat",
                Status::UNKNOWN => "unknown",
            };
            let mut model = String::new();
            if matches!(r, Status::SATISFIABLE) {
                for v in 1..=a.model_vars.min(inst.nvars) {
                    model.push(if c.val(v as i32) > 0 { '1' } else { '0' });
                }
            }
            let (cf, dc, pr) = (
                c.get_statistic_value("conflicts"),
                c.get_statistic_value("decisions"),
                c.get_statistic_value("propagations"),
            );
            out(Output {
                status,
                conflicts: cf,
                decisions: dc,
                propagations: pr,
                wall_ms: wall,
                model: &model,
                solver: "cadical",
                version: &CaDiCal::version(),
                peak_rss_bytes: peak_rss_bytes(),
                requested_conflict_budget: a.budget,
                effective_conflict_budget: i64::from(budget),
                variables: inst.nvars,
                ordinary_clauses: inst.clauses.len() + usize::from(inst.empty_clause),
                xor_rows: inst.xors.len(),
            });
        }
        other => {
            eprintln!("unknown solver {other}");
            std::process::exit(2);
        }
    }
}

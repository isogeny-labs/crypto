//! Thin source-linked adapter around the repository's exact native CDCL/XOR
//! solver.  The solver source is included by path rather than copied.

#[allow(dead_code, clippy::needless_range_loop)]
#[path = "../../../../src/cryptanalysis/sat.rs"]
mod sat;

use sat::{parse_dimacs, parse_dimacs_xor, SolveResult};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::time::{Duration, Instant};

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn semantic_inputs(formula: &str) -> Result<Vec<u32>, String> {
    let mut variables = Vec::new();
    for line in formula.lines().filter(|line| line.starts_with("c slot ")) {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 7
            || fields[0] != "c"
            || fields[1] != "slot"
            || fields[3] != "selector_bits"
            || fields[5] != "sign"
        {
            return Err(format!("malformed selector metadata line: {line}"));
        }
        for field in fields[4].split(',') {
            variables.push(
                field
                    .parse::<u32>()
                    .map_err(|_| format!("invalid selector variable {field:?}"))?,
            );
        }
        variables.push(
            fields[6]
                .parse::<u32>()
                .map_err(|_| format!("invalid sign variable {:?}", fields[6]))?,
        );
    }
    if variables.is_empty() {
        return Err("formula has no selector metadata comments".into());
    }
    variables.sort_unstable();
    variables.dedup();
    Ok(variables)
}

fn peak_rss_bytes() -> Option<u64> {
    // SAFETY: `rusage` is plain old data and `getrusage` initializes it.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return None;
    }
    let raw = u64::try_from(usage.ru_maxrss).ok()?;
    #[cfg(target_os = "macos")]
    return Some(raw);
    #[cfg(not(target_os = "macos"))]
    raw.checked_mul(1024)
}

fn model_text(model: &[bool]) -> String {
    let mut output = String::from("s SATISFIABLE\nv ");
    for (index, value) in model.iter().copied().enumerate() {
        let variable = (index + 1) as i64;
        output.push_str(&(if value { variable } else { -variable }).to_string());
        output.push(' ');
        if (index + 1) % 20 == 0 && index + 1 != model.len() {
            output.push_str("0\nv ");
        }
    }
    output.push_str("0\n");
    output
}

fn main() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 7 {
        return Err(
            "usage: prime31-selector-native-solver <cnf|xnf> <formula> <model.out> <record.json> <timeout-seconds> <conflict-budget> <priority|plain>"
                .into(),
        );
    }
    let formula_bytes = fs::read(&arguments[1])
        .map_err(|error| format!("read formula {}: {error}", arguments[1]))?;
    let formula = std::str::from_utf8(&formula_bytes)
        .map_err(|error| format!("formula is not UTF-8: {error}"))?;
    let timeout_seconds = arguments[4]
        .parse::<u64>()
        .map_err(|_| format!("invalid timeout {:?}", arguments[4]))?;
    if timeout_seconds == 0 {
        return Err("timeout must be positive".into());
    }
    let conflict_budget = arguments[5]
        .parse::<u64>()
        .map_err(|_| format!("invalid conflict budget {:?}", arguments[5]))?;
    if conflict_budget == 0 {
        return Err("conflict budget must be positive".into());
    }
    let native_xor = match arguments[0].as_str() {
        "cnf" => false,
        "xnf" => true,
        value => return Err(format!("unknown formula kind {value:?}")),
    };
    let priority = match arguments[6].as_str() {
        "priority" => true,
        "plain" => false,
        value => return Err(format!("unknown branching mode {value:?}")),
    };

    let parse_started = Instant::now();
    let mut solver = if native_xor {
        parse_dimacs_xor(formula)?
    } else {
        parse_dimacs(formula)?
    };
    let parse_ns = parse_started.elapsed().as_nanos();
    let semantic_variables = semantic_inputs(formula)?;
    if priority {
        solver.set_branch_priority(&semantic_variables);
    }
    solver.deadline = Some(Instant::now() + Duration::from_secs(timeout_seconds));
    solver.conflict_budget = conflict_budget;
    let variables = solver.n_vars();
    let original_clauses = solver.n_original_clauses();
    let xor_rows = solver.n_xors();

    let solve_started = Instant::now();
    let outcome = solver.solve();
    let solve_ns = solve_started.elapsed().as_nanos();
    let status = match outcome {
        SolveResult::Sat => "sat",
        SolveResult::Unsat => "unsat",
        SolveResult::Unknown => "unknown",
    };
    let model_verified = if outcome == SolveResult::Sat {
        let model = solver.model();
        if !solver.check_original_assignment(&model) {
            return Err("native SAT model fails original formula replay".into());
        }
        fs::write(&arguments[2], model_text(&model))
            .map_err(|error| format!("write model {}: {error}", arguments[2]))?;
        true
    } else {
        let text = if outcome == SolveResult::Unsat {
            "s UNSATISFIABLE\n"
        } else {
            "s UNKNOWN\n"
        };
        fs::write(&arguments[2], text)
            .map_err(|error| format!("write status {}: {error}", arguments[2]))?;
        false
    };
    let stats = solver.stats;
    let record = json!({
        "schema":"prime31.selector-native-solver-record/v1",
        "solver":"crypto_lib::cryptanalysis::sat (source-linked)",
        "formula_kind":if native_xor {"cnf_xor"} else {"ordinary_cnf"},
        "formula_sha256":sha256(&formula_bytes),
        "formula_bytes":formula_bytes.len(),
        "variables":variables,
        "original_clauses":original_clauses,
        "xor_rows":xor_rows,
        "semantic_input_variables":semantic_variables.len(),
        "semantic_branch_priority":priority,
        "timeout_seconds":timeout_seconds,
        "conflict_budget":conflict_budget,
        "status":status,
        "model_formula_replay":model_verified,
        "parse_ns":parse_ns.to_string(),
        "solve_ns":solve_ns.to_string(),
        "peak_rss_bytes_at_end":peak_rss_bytes(),
        "stats":{
            "decisions":stats.decisions,
            "conflicts":stats.conflicts,
            "restarts":stats.restarts,
            "propagations":stats.propagations,
            "learnt_clauses":stats.learnt_clauses,
            "clause_visits":stats.clause_visits,
            "clause_lit_visits":stats.clause_lit_visits,
            "analyze_lit_visits":stats.analyze_lit_visits,
            "max_level":stats.max_level,
            "xor_passes":stats.xor_passes,
            "xor_propagations":stats.xor_propagations,
            "xor_conflicts":stats.xor_conflicts,
            "xor_row_ops":stats.xor_row_ops,
            "xor_row_scans":stats.xor_row_scans,
            "xor_word_updates":stats.xor_word_updates,
            "xor_support_visits":stats.xor_support_visits,
            "word_ops":solver.word_ops()
        },
        "forbidden":{
            "secret_scalars_consumed":0,
            "witness_indices_or_signs_consumed":0,
            "pair_table_entries_consumed":0,
            "half_table_entries_consumed":0,
            "complete_selector_assignments_enumerated":0,
            "grader_records_consumed_after_target_extraction":0
        },
        "pcc":"unverified"
    });
    let mut bytes = serde_json::to_vec_pretty(&record).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    fs::write(&arguments[3], bytes)
        .map_err(|error| format!("write record {}: {error}", arguments[3]))?;
    println!("{status}");
    Ok(())
}

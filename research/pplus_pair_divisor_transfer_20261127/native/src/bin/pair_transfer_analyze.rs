use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Cell {
    kind: &'static str,
    depth: usize,
    state: u64,
    arithmetic: u64,
}

fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&format!("{key}=")))
}

fn parse_u64(line: &str, key: &str) -> Result<u64, String> {
    field(line, key)
        .ok_or_else(|| format!("missing {key}"))?
        .parse()
        .map_err(|_| format!("invalid {key}"))
}

fn main() {
    let mut args = env::args().skip(1);
    let candidate_path = args.next().unwrap_or_else(|| {
        eprintln!("usage: pair-transfer-analyze CANDIDATE.txt VERIFIER.txt");
        std::process::exit(2);
    });
    let verifier_path = args.next().unwrap_or_else(|| {
        eprintln!("usage: pair-transfer-analyze CANDIDATE.txt VERIFIER.txt");
        std::process::exit(2);
    });
    let candidate = fs::read_to_string(&candidate_path).unwrap_or_else(|error| {
        eprintln!("cannot read {candidate_path}: {error}");
        std::process::exit(2);
    });
    let verifier = fs::read_to_string(&verifier_path).unwrap_or_else(|error| {
        eprintln!("cannot read {verifier_path}: {error}");
        std::process::exit(2);
    });

    let verifier_summary = verifier
        .lines()
        .find(|line| line.starts_with("VERIFY schema=pplus-pair-divisor-transfer-verify-v"));
    let exactness_ok = verifier_summary.and_then(|line| field(line, "exactness_ok")) == Some("1");
    let verifier_errors = verifier_summary
        .and_then(|line| field(line, "errors"))
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(u64::MAX);

    let mut cells = Vec::new();
    let mut parse_errors = Vec::new();
    let mut poly_stop = "none".to_owned();
    let mut dag_stop = "none".to_owned();
    for line in candidate.lines() {
        if line.starts_with("POLY_BEGIN label=second_pair_norm") {
            match (
                parse_u64(line, "depth"),
                parse_u64(line, "slots"),
                parse_u64(line, "add"),
                parse_u64(line, "mul"),
            ) {
                (Ok(depth), Ok(state), Ok(add), Ok(mul)) => cells.push(Cell {
                    kind: "poly",
                    depth: depth as usize,
                    state,
                    arithmetic: add.saturating_add(mul),
                }),
                _ => parse_errors.push("invalid polynomial summary"),
            }
        } else if line.starts_with("DAG_BEGIN label=second_pair_norm") {
            match (
                parse_u64(line, "depth"),
                parse_u64(line, "nodes"),
                parse_u64(line, "add"),
                parse_u64(line, "mul"),
                parse_u64(line, "neg"),
            ) {
                (Ok(depth), Ok(state), Ok(add), Ok(mul), Ok(neg)) => cells.push(Cell {
                    kind: "dag",
                    depth: depth as usize,
                    state,
                    arithmetic: add.saturating_add(mul).saturating_add(neg),
                }),
                _ => parse_errors.push("invalid DAG summary"),
            }
        } else if line.starts_with("POLY_STOP label=second_pair_norm") {
            poly_stop = line.to_owned();
        } else if line.starts_with("DAG_STOP label=second_pair_norm") {
            dag_stop = line.to_owned();
        }
    }

    cells.sort_by_key(|cell| (cell.kind, cell.depth));
    for kind in ["poly", "dag"] {
        let selected: Vec<_> = cells.iter().filter(|cell| cell.kind == kind).collect();
        for window in selected.windows(2) {
            let state_ratio = window[1].state as f64 / window[0].state as f64;
            let arithmetic_ratio = window[1].arithmetic as f64 / window[0].arithmetic as f64;
            println!(
                "RATIO kind={kind} from_depth={} to_depth={} state={state_ratio:.9} arithmetic={arithmetic_ratio:.9}",
                window[0].depth, window[1].depth
            );
        }
    }

    let candidate_complete = candidate
        .lines()
        .last()
        .is_some_and(|line| line == "FOOTER status=PRODUCER_COMPLETE");
    let coverage_complete = false;
    let forbidden_aggregate = false;
    let recurrence_proved_below_four = false;
    let verdict =
        if !candidate_complete || !exactness_ok || verifier_errors != 0 || !parse_errors.is_empty()
        {
            "INVALID_STOP"
        } else if forbidden_aggregate {
            "PAIR_AGGREGATE_STOP"
        } else if recurrence_proved_below_four && coverage_complete {
            "ALGORITHM_PASS"
        } else if poly_stop != "none" || dag_stop != "none" {
            "INDETERMINATE_CAP"
        } else {
            "BOUNDED_NO_FOLD"
        };

    println!(
        "ANALYSIS schema=pplus-pair-divisor-transfer-analysis-v1 exactness_ok={} coverage_complete={} candidate_complete={} cross_target_amortization=0 pair_aggregate=0 recurrence_rho_lt_4=0 poly_cells={} dag_cells={} poly_stop={} dag_stop={} errors={} verdict={verdict}",
        usize::from(exactness_ok),
        usize::from(coverage_complete),
        usize::from(candidate_complete),
        cells.iter().filter(|cell| cell.kind == "poly").count(),
        cells.iter().filter(|cell| cell.kind == "dag").count(),
        usize::from(poly_stop != "none"),
        usize::from(dag_stop != "none"),
        parse_errors.len(),
    );
    if poly_stop != "none" {
        println!("STOP_DETAIL {poly_stop}");
    }
    if dag_stop != "none" {
        println!("STOP_DETAIL {dag_stop}");
    }
    for error in parse_errors {
        println!("ERROR {error}");
    }

    if verdict == "INVALID_STOP" {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_key_value_fields() {
        let line = "CELL depth=3 nodes=17";
        assert_eq!(field(line, "depth"), Some("3"));
        assert_eq!(parse_u64(line, "nodes").unwrap(), 17);
    }
}

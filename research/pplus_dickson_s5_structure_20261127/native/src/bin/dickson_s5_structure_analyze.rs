use std::collections::BTreeMap;
use std::env;
use std::fs;

type Fields = BTreeMap<String, String>;

fn records(text: &str) -> Vec<(String, Fields)> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut words = line.split_whitespace();
            let tag = words.next().unwrap_or_default().to_owned();
            let fields = words
                .filter_map(|word| word.split_once('='))
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            (tag, fields)
        })
        .collect()
}

fn find<'a>(records: &'a [(String, Fields)], tag: &str) -> Vec<&'a Fields> {
    records
        .iter()
        .filter_map(|(record_tag, fields)| (record_tag == tag).then_some(fields))
        .collect()
}

fn value<'a>(fields: &'a Fields, key: &str) -> Option<&'a str> {
    fields.get(key).map(String::as_str)
}

fn number(fields: &Fields, key: &str) -> Option<usize> {
    value(fields, key)?.parse().ok()
}

fn main() {
    let arguments: Vec<_> = env::args().collect();
    assert_eq!(
        arguments.len(),
        3,
        "usage: dickson-s5-structure-analyze CANDIDATE.txt VERIFIER.txt"
    );
    let candidate_text = fs::read_to_string(&arguments[1]).expect("read candidate evidence");
    let verifier_text = fs::read_to_string(&arguments[2]).expect("read verifier evidence");
    let candidate = records(&candidate_text);
    let verifier = records(&verifier_text);

    let candidate_final = find(&candidate, "FINAL");
    let verifier_final = find(&verifier, "VERIFY");
    let exactness_ok = candidate_final.len() == 1
        && verifier_final.len() == 1
        && value(candidate_final[0], "algebra_ok") == Some("1")
        && value(candidate_final[0], "production_authorized") == Some("0")
        && value(verifier_final[0], "candidate_match") == Some("1")
        && value(verifier_final[0], "exactness_ok") == Some("1")
        && value(verifier_final[0], "errors") == Some("0");
    let exact_zero_oracle = candidate_final
        .first()
        .and_then(|fields| value(fields, "exact_zero_oracle"))
        == Some("1");

    let mut structure_cells = 0;
    let cells = find(&candidate, "CELL");
    for cell in &cells {
        let n = number(cell, "n").unwrap_or(0);
        let pair_dimension = number(cell, "pair_dimension").unwrap_or(0);
        let kernel_rank = number(cell, "kernel_rank").unwrap_or(usize::MAX);
        let delta_s_rank = number(cell, "delta_s_rank").unwrap_or(usize::MAX);
        let delta_p_rank = number(cell, "delta_p_rank").unwrap_or(usize::MAX);
        let low_displacement = delta_s_rank < pair_dimension || delta_p_rank < pair_dimension;
        structure_cells += usize::from(low_displacement);
        println!(
            "ANALYSIS_CELL depth={} n={n} label={} pair_dimension={pair_dimension} kernel_rank={kernel_rank} delta_s_rank={delta_s_rank} delta_p_rank={delta_p_rank} structure_present={} zero_oracle=0",
            value(cell, "depth").unwrap_or("missing"),
            value(cell, "label").unwrap_or("missing"),
            usize::from(low_displacement)
        );
    }

    for state in find(&candidate, "STATE") {
        println!(
            "ANALYSIS_STATE depth={} n={} four_multisets={} pair_square={} compact_circuit_coefficients_max={} explicit_state_subquadratic=0",
            value(state, "depth").unwrap_or("missing"),
            value(state, "n").unwrap_or("missing"),
            value(state, "four_multisets").unwrap_or("missing"),
            value(state, "pair_square").unwrap_or("missing"),
            value(state, "compact_circuit_coefficients_max").unwrap_or("missing")
        );
    }

    let oracle_contract_ok = false;
    let verdict = if !exactness_ok {
        "INVALID_STOP"
    } else if !exact_zero_oracle {
        "STRUCTURE_ONLY_STOP"
    } else if oracle_contract_ok {
        "ALGORITHM_PASS"
    } else {
        "INVALID_STOP"
    };
    println!(
        "ANALYSIS schema=pplus-dickson-s5-analysis-v1 exactness_ok={} cells={} structure_cells={structure_cells} exact_zero_oracle={} oracle_contract_ok={} cross_target_amortization=0 production_authorized=0 verdict={verdict}",
        usize::from(exactness_ok),
        cells.len(),
        usize::from(exact_zero_oracle),
        usize::from(oracle_contract_ok)
    );

    if verdict == "INVALID_STOP" {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_keeps_frozen_key_values() {
        let parsed = records("FINAL algebra_ok=1 exact_zero_oracle=0\n");
        assert_eq!(parsed.len(), 1);
        assert_eq!(value(&parsed[0].1, "algebra_ok"), Some("1"));
        assert_eq!(number(&parsed[0].1, "exact_zero_oracle"), Some(0));
    }
}

//! Producer consistency/readout for the frozen chart panel; not an independent
//! replay of the curve arithmetic or the solver's oracle.
use serde_json::{json, Value};
use std::collections::BTreeMap;

const TARGETS: [&str; 2] = ["common0", "planted[5, 6, 12, 19]"];

fn number(v: &Value, key: &str) -> Result<u64, String> {
    v[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer {key}"))
}

fn chart_readout(rows: &[Value]) -> Result<Value, String> {
    let mut index = BTreeMap::new();
    for v in rows {
        if v["kind"] != "inf" {
            return Err("unexpected solve kind".into());
        }
        let target = v["target"].as_str().ok_or("missing target")?;
        let transformed = v["transform"].as_bool().ok_or("missing transform")?;
        let seed = number(v, "chart_seed")?;
        let set = if transformed { "R2" } else { "R1" };
        if !TARGETS.contains(&target)
            || number(v, "p")? != 29
            || number(v, "curve")? != 1
            || v["arm"] != "conj"
            || v["set"] != set
            || number(v, "macaulay_degree")? != 29
            || number(v, "solver_seed")? != 0xdec1
            || seed > 2
            || (!transformed && seed != 0)
        {
            return Err("row outside frozen chart population".into());
        }
        for key in ["solved", "match"] {
            if v[key].as_bool().is_none() {
                return Err(format!("missing boolean {key}"));
            }
        }
        for key in [
            "dim",
            "total_muls",
            "solver_decomps",
            "oracle_decomps",
            "at_infinity",
        ] {
            number(v, key)?;
        }
        let decomps: Vec<[u64; 4]> =
            serde_json::from_str(v["decomps"].as_str().ok_or("missing decomps")?)
                .map_err(|e| e.to_string())?;
        if decomps.len() as u64 != number(v, "solver_decomps")? {
            return Err("decomposition count mismatch".into());
        }
        if decomps
            .iter()
            .any(|d| d.iter().any(|x| *x >= 29) || d.windows(2).any(|w| w[0] > w[1]))
        {
            return Err("noncanonical factor-base parameters".into());
        }
        if transformed {
            let matrix: [[u64; 5]; 5] =
                serde_json::from_value(v["chart_matrix"].clone()).map_err(|e| e.to_string())?;
            if matrix.iter().flatten().any(|x| *x >= 29) || !invertible(matrix) {
                return Err("noncanonical or singular chart".into());
            }
            if index.insert((target, seed), v).is_some() {
                return Err("duplicate chart".into());
            }
        } else {
            if !v["chart_matrix"].is_null() {
                return Err("plain solve has chart matrix".into());
            }
            // Plain calls intentionally repeat per offset; compare every repeat
            // against the first, including failed plain solves.
            if let Some(old) = index.get(&(target, 3)) {
                for key in [
                    "dim",
                    "total_muls",
                    "solved",
                    "match",
                    "decomps",
                    "solver_decomps",
                    "oracle_decomps",
                ] {
                    if old[key] != v[key] {
                        return Err(format!("plain A/A mismatch: {key}"));
                    }
                }
            } else {
                index.insert((target, 3), v);
            }
        }
    }
    if rows.len() != 12 {
        return Err("expected twelve calls".into());
    }
    let mut result = Vec::new();
    for target in TARGETS {
        if rows
            .iter()
            .filter(|v| v["target"] == target && v["transform"] == false)
            .count()
            != 3
        {
            return Err("expected three plain A/A calls per target".into());
        }
        let plain = index.get(&(target, 3)).ok_or("missing plain solve")?;
        for seed in 0..3 {
            let v = index
                .get(&(target, seed))
                .ok_or("missing transformed call")?;
            if v["oracle_decomps"] != plain["oracle_decomps"] {
                return Err("oracle population mismatch".into());
            }
            if v["solved"] == true && v["match"] != true {
                return Err("completed transformed solve disagrees with oracle".into());
            }
            result.push(json!({"target":target,"chart_seed":seed,"status":if v["solved"]==true {"complete"} else {"unknown"},
                "dim":v["dim"],"at_infinity":v["at_infinity"],"oracle_decomps":v["oracle_decomps"],
                "shortfall_removed":v["solved"]==true && v["match"]==true && v["dim"]==4096,
                "total_muls":v["total_muls"]}));
        }
    }
    Ok(
        json!({"schema":"chart-panel-readout/v1","standing":"producer-unverified","plain_aa":true,"clock_used":false,"rows":result}),
    )
}

fn invertible(mut a: [[u64; 5]; 5]) -> bool {
    for col in 0..5 {
        let Some(row) = (col..5).find(|r| a[*r][col] != 0) else {
            return false;
        };
        a.swap(row, col);
        // Small public F_29 arithmetic; exact nonzero pivot inverse.
        let inverse = (1..29).find(|x| a[col][col] * x % 29 == 1).unwrap();
        for row in col + 1..5 {
            let k = a[row][col] * inverse % 29;
            for j in col..5 {
                a[row][j] = (a[row][j] + 29 - k * a[col][j] % 29) % 29;
            }
        }
    }
    true
}

fn main() {
    let paths: Vec<_> = std::env::args().skip(1).collect();
    let result = (|| {
        if paths.len() != 6 {
            return Err("usage: chart_compare SIX_RAW_JSONL_FILES".into());
        }
        let mut rows = Vec::new();
        for path in paths {
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
            let mut calls = Vec::new();
            let mut controls = 0;
            for line in text.lines() {
                let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
                match v["kind"].as_str() {
                    Some("inf") => calls.push(v),
                    Some("control") if v["std_H_equals_precompute"] == true => {
                        controls += 1;
                    }
                    Some("curve") => {}
                    _ => return Err("unexpected row or failed interpolation control".into()),
                }
            }
            if controls != 1
                || calls.len() != 2
                || calls[0]["target"] != calls[1]["target"]
                || calls[0]["transform"] != false
                || calls[1]["transform"] != true
            {
                return Err("expected one plain/transformed pair per file".into());
            }
            rows.extend(calls);
        }
        chart_readout(&rows)
    })();
    match result {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            eprintln!("refused chart readout: {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn panel() -> Vec<Value> {
        let identity: [[u64; 5]; 5] =
            core::array::from_fn(|i| core::array::from_fn(|j| u64::from(i == j)));
        TARGETS.iter().flat_map(|target|(0..3).flat_map(move |seed|[false,true].map(move |tf|json!({
            "kind":"inf","p":29,"curve":1,"arm":"conj","target":target,"transform":tf,"set":if tf {"R2"} else {"R1"},
            "chart_seed":if tf {seed} else {0},"macaulay_degree":29,"solver_seed":0xdec1,
            "chart_matrix":if tf {json!(identity)} else {Value::Null},"solved":true,"match":true,"dim":4096,
            "total_muls":100,"solver_decomps":0,"oracle_decomps":0,"decomps":"[]","at_infinity":0})))).collect()
    }
    #[test]
    fn empty_match_is_unknown_when_solver_failed() {
        let mut rows = panel();
        rows[1]["solved"] = false.into();
        assert_eq!(
            chart_readout(&rows).unwrap()["rows"][0]["status"],
            "unknown"
        );
    }
    #[test]
    fn census_aa_and_completed_mismatches_are_refused() {
        let mut rows = panel();
        rows.pop();
        assert!(chart_readout(&rows).is_err());
        let mut rows = panel();
        rows[2]["total_muls"] = 101.into();
        assert!(chart_readout(&rows).is_err());
        let mut rows = panel();
        rows[1]["match"] = false.into();
        assert!(chart_readout(&rows).is_err());
    }
    #[test]
    fn chart_requires_a_nonsingular_canonical_matrix() {
        assert!(chart_readout(&panel()).is_ok());
        let mut rows = panel();
        rows[1]["chart_matrix"] = serde_json::to_value([[0u64; 5]; 5]).unwrap();
        assert!(chart_readout(&rows).is_err());
    }
}

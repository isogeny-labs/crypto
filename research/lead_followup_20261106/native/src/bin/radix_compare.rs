//! Compare one frozen depth, never infer completion from a smaller subset.
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn uint(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or_else(|| format!("missing integer {k}"))
}

fn index(rows: &[Value], radix: u64, depth: u64) -> Result<BTreeMap<String, &Value>, String> {
    let mut out = BTreeMap::new();
    for v in rows {
        if v.get("summary").is_some() {
            continue; // main separately checks the one terminal producer summary
        }
        if uint(v, "p")? != 2013265921
            || uint(v, "m")? != 2
            || uint(v, "t")? != depth
            || uint(v, "radix")? != radix
            || v["kind"] != "kummer"
            || v["engine"] != "f4_fp_tower"
            || v["target"] != "random"
            || uint(v, "target_index")? > 1
            || !["tower", "null"].iter().any(|c| v["control"] == *c)
        {
            return Err("row outside the frozen depth/arm/domain".into());
        }
        let n = uint(v, "n_vars")?;
        for x in [
            uint(&v["curve"], "a")?,
            uint(&v["curve"], "b")?,
            uint(&v["tower"], "g")?,
            uint(&v["tower"], "zeta")?,
            uint(v, "g")?,
            uint(v, "x_r")?,
        ] {
            if x >= 2013265921 {
                return Err("noncanonical field coefficient".into());
            }
        }
        if n != 2 * depth || uint(v, "N")? != n {
            return Err("wrong dimension".into());
        }
        let unit = vec![json!(1); n as usize];
        let weights = if radix == 2 && v.get("weights") == Some(&Value::Null) {
            &unit // the pilot's canonical unit-grading representation
        } else {
            v["weights"].as_array().ok_or("missing weights")?
        };
        let block = radix.ilog2() as u64;
        let expected: Vec<Value> = (0..2)
            .flat_map(|_| (0..depth).map(|i| json!(1u64 << (i % block))))
            .collect();
        if weights != &expected {
            return Err("wrong grading weights".into());
        }
        for field in ["timed_out", "oversize", "inconsistent"] {
            if v[field].as_bool().is_none() {
                return Err(format!("missing boolean {field}"));
            }
        }
        uint(v, "muladds")?;
        uint(v, "max_cols")?;
        let key = json!([v["control"], v["target_index"]]).to_string();
        if out.insert(key, v).is_some() {
            return Err("duplicate draw".into());
        }
    }
    if out.len() != 4 {
        return Err("incomplete census: expected tower/null times two draws".into());
    }
    Ok(out)
}

fn compare(arms: [&[Value]; 3], depth: u64) -> Result<Value, String> {
    if !(10..=12).contains(&depth) {
        return Err("depth outside protocol".into());
    }
    let tables = [
        index(arms[0], 2, depth)?,
        index(arms[1], 4, depth)?,
        index(arms[2], 8, depth)?,
    ];
    let mut rows = Vec::new();
    for (key, base) in &tables[0] {
        for (j, radix) in [4, 8].iter().enumerate() {
            let candidate = tables[j + 1].get(key).ok_or("missing paired draw")?;
            for field in [
                "curve",
                "tower",
                "x_r",
                "g",
                "p",
                "m",
                "t",
                "target",
                "target_index",
                "control",
                "kind",
                "N",
                "n_vars",
            ] {
                if base.get(field).is_none() || base[field] != candidate[field] {
                    return Err(format!("input mismatch: {field}"));
                }
            }
            let censored = [*base, *candidate]
                .iter()
                .any(|v| v["timed_out"] == true || v["oversize"] == true);
            if !censored
                && (base["inconsistent"] != candidate["inconsistent"]
                    || base["inconsistent"] != true)
            {
                return Err("completed refutation disagrees or lacks a unit-ideal outcome".into());
            }
            let b = uint(base, "muladds")?;
            let c = uint(candidate, "muladds")?;
            if !censored && b == 0 {
                return Err("zero reference cost".into());
            }
            rows.push(json!({"control":base["control"],"target_index":base["target_index"],"t":depth,"radix":radix,
                "status":if censored {"censored"} else {"complete"},"reference_muladds":b,"candidate_muladds":c,
                "ratio":if censored {Value::Null} else {json!(c as f64 / b as f64)},
                "reference_max_cols":base["max_cols"],"candidate_max_cols":candidate["max_cols"]}));
        }
    }
    Ok(
        json!({"schema":"radix-depth-comparison/v1","standing":"producer-unverified","clock_used":false,"t":depth,"rows":rows}),
    )
}

fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let result = (|| {
        if a.len() != 4 {
            return Err("usage: radix_compare DEPTH R2.jsonl R4.jsonl R8.jsonl".into());
        }
        let depth = a[0].parse().map_err(|_| "invalid depth")?;
        let mut inputs = Vec::new();
        for path in &a[1..] {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            let rows = text
                .lines()
                .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            inputs.push(rows);
        }
        let mut summary_pin = None;
        for rows in &inputs {
            let summaries: Vec<&Value> = rows
                .iter()
                .filter_map(|v: &Value| v.get("summary"))
                .collect();
            if summaries.len() != 1 || rows.last().and_then(|v| v.get("summary")).is_none() {
                return Err("expected exactly one terminal summary".into());
            }
            let s = summaries[0];
            if uint(s, "rows")? != 4
                || uint(s, "p")? != 2013265921
                || uint(s, "random_per_cell")? != 2
                || uint(s, "planted_violations")? != 0
            {
                return Err("invalid producer census/controls".into());
            }
            let pin = json!([
                uint(s, "seed")?,
                uint(s, "budget_seconds")?,
                uint(s, "planted_per_cell")?
            ]);
            if summary_pin.as_ref().is_some_and(|p| p != &pin) {
                return Err("construction seed/budget mismatch".into());
            }
            summary_pin = Some(pin);
        }
        compare([&inputs[0], &inputs[1], &inputs[2]], depth)
    })();
    match result {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            eprintln!("refused comparison: {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn arm(radix: u64) -> Vec<Value> {
        ["tower","null"].iter().flat_map(|c|(0..2).map(move |i|json!({"p":2013265921u64,"m":2,"t":10,"N":20,"n_vars":20,
            "radix":radix,"kind":"kummer","engine":"f4_fp_tower","target":"random","target_index":i,"control":c,
            "curve":{"a":1,"b":2},"tower":{"g":3,"zeta":4},"g":1,"x_r":42+i,
            "weights":(0..2).flat_map(|_|(0..10).map(|j|1u64<<(j%radix.ilog2()))).collect::<Vec<_>>(),
            "timed_out":false,"oversize":false,"inconsistent":true,"muladds":100,"max_cols":10}))).collect()
    }
    #[test]
    fn pairs_by_input_not_row_order() {
        let a = arm(2);
        let mut b = arm(4);
        b.reverse();
        let c = arm(8);
        assert_eq!(
            compare([&a, &b, &c], 10).unwrap()["rows"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
    }
    #[test]
    fn censored_costs_never_become_ratios() {
        let a = arm(2);
        let b = arm(4);
        let mut c = arm(8);
        c[0]["timed_out"] = true.into();
        let v = compare([&a, &b, &c], 10).unwrap();
        assert_eq!(
            v["rows"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["ratio"].is_null())
                .count(),
            1
        );
    }
    #[test]
    fn refuses_changed_targets_missing_cells_or_false_grading() {
        let a = arm(2);
        let b = arm(4);
        let mut c = arm(8);
        c[0]["x_r"] = 99.into();
        assert!(compare([&a, &b, &c], 10).is_err());
        let mut c = arm(8);
        c.pop();
        assert!(compare([&a, &b, &c], 10).is_err());
        let mut c = arm(8);
        c[0]["weights"] = json!(vec![1; 20]);
        assert!(compare([&a, &b, &c], 10).is_err());
        let mut c = arm(8);
        c[0]["inconsistent"] = false.into();
        assert!(compare([&a, &b, &c], 10).is_err());
    }
}

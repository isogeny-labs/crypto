//! Reprice observed primitives, check coverage labels, never promote a total.
// Keep the parent library's naming convention for the included unit source.
#[allow(dead_code, clippy::enum_variant_names)]
#[path = "../../../../src/cryptanalysis/bitops.rs"]
mod bitops;
use bitops::{Field, Prim, Widths};
use serde_json::Value;
use std::collections::BTreeMap;

const KINDS: [&str; 12] = [
    "field_add",
    "field_mul",
    "field_sqr",
    "field_inv",
    "field_sqrt",
    "legendre",
    "modr_add",
    "modr_mul",
    "modr_inv",
    "word_op",
    "hash_probe",
    "rng_words",
];
const PHASES: [&str; 9] = [
    "unattributed",
    "walk",
    "transport",
    "model",
    "factor_base",
    "relations",
    "descent",
    "linear_algebra",
    "verification",
];
const COMPONENTS: [&str; 6] = [
    "mapping",
    "filter",
    "replay",
    "output",
    "original_solve",
    "projected_solve",
];

fn zero_counts() -> BTreeMap<String, u64> {
    KINDS.into_iter().map(|k| (k.to_string(), 0)).collect()
}

fn counts(v: &Value) -> Option<BTreeMap<String, u64>> {
    let m = v.as_object()?;
    if m.len() != KINDS.len() {
        return None;
    }
    let out: BTreeMap<_, _> = KINDS
        .into_iter()
        .map(|k| Some((k.to_string(), m.get(k)?.as_u64()?)))
        .collect::<Option<_>>()?;
    (out["rng_words"] <= out["word_op"]).then_some(out)
}
fn price(m: &BTreeMap<String, u64>, w: &Widths) -> Option<u128> {
    m.iter()
        .filter(|(k, _)| k.as_str() != "rng_words")
        .try_fold(0u128, |sum, (k, &n)| {
            sum.checked_add(w.gates(Prim::from_name(k)?).ok()?.checked_mul(n as u128)?)
        })
}
fn declared_price(v: &Value, expected: u128) -> bool {
    v.as_str() == Some(expected.to_string().as_str())
}

pub(super) fn valid(v: &Value, family: &Value) -> bool {
    validate(v, family).is_some()
}
fn validate(v: &Value, family: &Value) -> Option<()> {
    let p = family["p"].as_u64()?;
    let l = family["l"].as_u64()?;
    let w = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    if p == 0
        || l == 0
        || v["schema"] != "cover.graph.accounting-diagnostic/v1"
        || v["unit"] != "bitop/v1"
        || v["complete"] != false
        || v["comparison_eligible"] != false
        || v.get("total_bitops") != Some(&Value::Null)
        || v["foreign_hook_calls"] != 0
        || v["other_modulus_ops"] != 0
        || v["widths"]["field_bits"] != w.element_bits()
        || v["widths"]["r_bits"] != w.r_bits
        || v["pricing"] != "executed modular hooks and observed native events"
        || v["known_uncounted"].as_array()?.is_empty()
        || v["known_uncounted"].as_array()?.iter().any(|g| {
            g["name"].as_str().is_none_or(str::is_empty)
                || g.get("count") != Some(&Value::Null)
                || g["status"] != "coverage_or_emission_pending"
        })
    {
        return None;
    }
    let phases = v["phases"].as_array()?;
    if phases.len() != PHASES.len() {
        return None;
    }
    let mut sum = zero_counts();
    for (row, name) in phases.iter().zip(PHASES) {
        if row["phase"] != name {
            return None;
        }
        let c = counts(&row["primitives"])?;
        if !declared_price(&row["diagnostic_priced_bitops"], price(&c, &w)?) {
            return None;
        }
        for (k, n) in c {
            let slot = sum.get_mut(&k)?;
            *slot = slot.checked_add(n)?;
        }
    }
    let cost = price(&sum, &w)?;
    (cost > 0
        && counts(&v["observed_primitives"])? == sum
        && declared_price(&v["diagnostic_priced_bitops"], cost))
    .then_some(())
}

fn add_counts(sum: &mut BTreeMap<String, u64>, next: &Value) -> Option<()> {
    for (k, n) in counts(next)? {
        let slot = sum.get_mut(&k)?;
        *slot = slot.checked_add(n)?;
    }
    Some(())
}

/// Check and reprice projection/solver observations inside the LA phase.
/// Unknown native work remains explicit; no complete total is manufactured.
pub(super) fn details(work: &Value, outer: &Value, family: &Value) -> Option<Value> {
    let p = family["p"].as_u64()?;
    let l = family["l"].as_u64()?;
    if p < 3 || l < 3 {
        return None;
    }
    let w = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let attempts = work["la_attempts"].as_u64()?;
    if attempts == 0 || attempts > 4096 {
        return None;
    }
    let solves = work["work_native"].as_array()?;
    let graph = work.get("graph_filter").filter(|g| !g.is_null());
    let mut sum = zero_counts();
    let mut components: BTreeMap<_, _> = COMPONENTS
        .into_iter()
        .map(|name| (name, zero_counts()))
        .collect();
    let mut solve_rows = Vec::new();
    for solve in solves {
        let rows = solve["rows"].as_u64()?;
        if solve["schema"] != "cover.sieve.solve-work/v1"
            || solve["complete"] != false
            || solve["marker"] != "counted:cover_solve_subtotals"
            || rows == 0
            || rows > 4096
            || solve["columns"] != rows
            || solve["nonzeros"].as_u64()? > rows.checked_mul(4096)?
            || !solve["returned_solution"].is_boolean()
        {
            return None;
        }
        add_counts(&mut sum, &solve["primitives"])?;
        let component = match solve["kind"].as_str()? {
            "original" => "original_solve",
            "projected" => "projected_solve",
            _ => return None,
        };
        add_counts(components.get_mut(component)?, &solve["primitives"])?;
        let mut row = solve.clone();
        row["diagnostic_priced_bitops"] =
            Value::from(price(&counts(&solve["primitives"])?, &w)?.to_string());
        solve_rows.push(row);
    }
    let mut projection_rows = Vec::new();
    let mut next_solve = 0;
    let mut input_rows = 0u64;
    let mut output_rows = 0u64;
    let mut accepted_calls = 0u64;
    let mut fallback_calls = 0u64;
    let calls: &[Value] = if let Some(graph) = graph {
        graph["work_native"].as_array()?
    } else {
        &[]
    };
    if graph.is_some() && calls.len() as u64 != attempts {
        return None;
    }
    for attempt in 1..=attempts {
        let call = graph.map(|_| &calls[(attempt - 1) as usize]);
        let accepted = if let Some(call) = call {
            let input = call["input_rows"].as_u64()?;
            if call["schema"] != "cover.graph.projection-work/v1"
                || call["complete"] != false
                || call["marker"] != "counted:graph_projection_subtotals"
                || call["call_index"] != attempt
                || input == 0
                || input > 4096
                || call["input_nonzeros"].as_u64()? > input.checked_mul(4096)?
            {
                return None;
            }
            input_rows = input_rows.checked_add(input)?;
            let accepted = call["accepted"].as_bool()?;
            match (
                call["candidate_rows"].as_u64(),
                call["candidate_nonzeros"].as_u64(),
            ) {
                (Some(rows), Some(nnz)) if rows <= input && nnz <= rows.checked_mul(4096)? => (),
                (None, None)
                    if call.get("candidate_rows") == Some(&Value::Null)
                        && call.get("candidate_nonzeros") == Some(&Value::Null)
                        && !accepted => {}
                _ => return None,
            }
            let stages = call["stages"].as_array()?;
            let expected = ["mapping", "filter", "replay", "output"];
            if stages.is_empty()
                || stages.len() > 4
                || (accepted && stages.len() != 4)
                || (!accepted && stages.len() == 4)
            {
                return None;
            }
            if accepted
                && (price(&counts(&stages[1]["primitives"])?, &w)? == 0
                    || price(&counts(&stages[2]["primitives"])?, &w)? == 0)
            {
                return None;
            }
            let mut call_row = call.clone();
            for (i, (stage, name)) in stages.iter().zip(expected).enumerate() {
                if stage["stage"] != name {
                    return None;
                }
                add_counts(&mut sum, &stage["primitives"])?;
                add_counts(components.get_mut(name)?, &stage["primitives"])?;
                call_row["stages"][i]["diagnostic_priced_bitops"] =
                    Value::from(price(&counts(&stage["primitives"])?, &w)?.to_string());
            }
            if accepted {
                let rows = call["candidate_rows"].as_u64()?;
                if rows == 0
                    || rows > input
                    || call["candidate_nonzeros"].as_u64()? > rows.checked_mul(4096)?
                {
                    return None;
                }
                output_rows = output_rows.checked_add(rows)?;
                accepted_calls += 1;
            }
            projection_rows.push(call_row);
            accepted
        } else {
            false
        };
        if accepted {
            let solve = solves.get(next_solve)?;
            let call = call?;
            if solve["attempt"] != attempt
                || solve["kind"] != "projected"
                || solve["rows"] != call["candidate_rows"]
                || solve["nonzeros"] != call["candidate_nonzeros"]
            {
                return None;
            }
            next_solve += 1;
            if solve["returned_solution"] == true
                && solves
                    .get(next_solve)
                    .is_none_or(|s| s["attempt"] != attempt)
            {
                continue;
            }
        }
        let solve = solves.get(next_solve)?;
        if solve["attempt"] != attempt || solve["kind"] != "original" {
            return None;
        }
        if let Some(call) = call {
            if solve["rows"] != call["input_rows"] || solve["nonzeros"] != call["input_nonzeros"] {
                return None;
            }
            fallback_calls += 1;
        }
        next_solve += 1;
    }
    if next_solve != solves.len() || solves.last()?["returned_solution"] != true {
        return None;
    }
    if let Some(g) = graph {
        if g["calls"] != attempts
            || g["projected"] != accepted_calls
            || g["refusals"] != attempts - accepted_calls
            || g["fallback_solves"] != fallback_calls
            || g["input_rows"] != input_rows
            || g["output_rows"] != output_rows
            || g["replayed_rows"] != output_rows
        {
            return None;
        }
    }
    let la = outer["phases"]
        .as_array()?
        .iter()
        .find(|p| p["phase"] == "linear_algebra")?;
    let mut rest = counts(&la["primitives"])?;
    for (k, &n) in &sum {
        let slot = rest.get_mut(k)?;
        *slot = slot.checked_sub(n)?;
    }
    let mut component_totals = serde_json::Map::new();
    for (name, c) in components {
        component_totals.insert(name.into(),serde_json::json!({"primitives":c,"diagnostic_priced_bitops":price(&c,&w)?.to_string()}));
    }
    Some(
        serde_json::json!({"schema":"cover.graph.checked-subtotals/v1", "complete":false,"component_totals":component_totals,
        "projection_calls":projection_rows, "solve_calls":solve_rows,
        "observed_component_primitives":sum, "diagnostic_component_bitops":price(&sum,&w)?.to_string(),
        "unallocated_linear_algebra":{"primitives":rest,"diagnostic_priced_bitops":price(&rest,&w)?.to_string()},
        "known_uncounted":"native bookkeeping remains incomplete"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn tally(words: u64) -> Value {
        let mut m = serde_json::Map::new();
        for k in KINDS {
            m.insert(k.into(), json!(if k == "word_op" { words } else { 0 }));
        }
        Value::Object(m)
    }
    fn fixture(graph: bool) -> (Value, Value, Value) {
        let family = json!({"p":1009,"l":(1u64<<61)-1});
        let outer = json!({"phases":[{"phase":"linear_algebra","primitives":tally(100)}]});
        let call = json!({"schema":"cover.graph.projection-work/v1","complete":false,"marker":"counted:graph_projection_subtotals","call_index":1,"input_rows":4,"input_nonzeros":6,"candidate_rows":3,"candidate_nonzeros":3,"accepted":true,"stages":[{"stage":"mapping","primitives":tally(1)},{"stage":"filter","primitives":tally(2)},{"stage":"replay","primitives":tally(3)},{"stage":"output","primitives":tally(4)}]});
        let g = json!({"calls":1,"projected":1,"refusals":0,"fallback_solves":0,"input_rows":4,"output_rows":3,"replayed_rows":3,"work_native":[call]});
        let solve = json!({"schema":"cover.sieve.solve-work/v1","complete":false,"marker":"counted:cover_solve_subtotals","attempt":1,"kind":if graph {"projected"} else {"original"},"rows":if graph {3} else {4},"columns":if graph {3} else {4},"nonzeros":if graph {3} else {6},"returned_solution":true,"primitives":tally(if graph {5} else {10})});
        (
            json!({"la_attempts":1,"graph_filter":if graph {g} else {Value::Null},"work_native":[solve]}),
            outer,
            family,
        )
    }
    #[test]
    fn projection_and_solve_counts_partition_the_observed_phase() {
        for graph in [false, true] {
            let (work, outer, family) = fixture(graph);
            let d = details(&work, &outer, &family).unwrap();
            assert_eq!(d["complete"], false);
            assert_eq!(
                d["diagnostic_component_bitops"],
                if graph { "960" } else { "640" }
            );
            assert_eq!(
                d["unallocated_linear_algebra"]["primitives"]["word_op"],
                if graph { 85 } else { 90 }
            );
        }
    }
    #[test]
    fn refused_projection_keeps_the_original_solve_and_spent_work() {
        let (mut work, outer, family) = fixture(true);
        let g = &mut work["graph_filter"];
        g["projected"] = json!(0);
        g["refusals"] = json!(1);
        g["fallback_solves"] = json!(1);
        g["output_rows"] = json!(0);
        g["replayed_rows"] = json!(0);
        let c = &mut g["work_native"][0];
        c["accepted"] = json!(false);
        c["candidate_rows"] = Value::Null;
        c["candidate_nonzeros"] = Value::Null;
        c["stages"].as_array_mut().unwrap().truncate(2);
        let s = &mut work["work_native"][0];
        s["kind"] = json!("original");
        s["rows"] = json!(4);
        s["columns"] = json!(4);
        s["nonzeros"] = json!(6);
        assert!(details(&work, &outer, &family).is_some());
        work["work_native"][0]["kind"] = json!("projected");
        assert!(details(&work, &outer, &family).is_none());
    }
    #[test]
    fn detail_tampering_and_counter_overflow_are_rejected() {
        let (work, outer, family) = fixture(true);
        for bad in 0..8 {
            let mut w = work.clone();
            match bad {
                0 => w["work_native"][0]["attempt"] = json!(2),
                1 => w["graph_filter"]["work_native"][0]["candidate_nonzeros"] = json!(4),
                2 => w["graph_filter"]["work_native"][0]["stages"][2]["stage"] = json!("filter"),
                3 => w["work_native"][0]["primitives"] = tally(101),
                4 => w["work_native"][0]["complete"] = json!(true),
                5 => w["graph_filter"]["output_rows"] = json!(4),
                6 => w["work_native"][0]["primitives"] = tally(u64::MAX),
                _ => w["graph_filter"]["work_native"][0]["call_index"] = json!(2),
            }
            assert!(details(&w, &outer, &family).is_none(), "case {bad}");
        }
        let mut bad_family = family.clone();
        bad_family["p"] = json!(0);
        assert!(details(&work, &outer, &bad_family).is_none());
    }
}

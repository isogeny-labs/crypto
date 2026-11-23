//! Reprice observed primitives, check coverage labels, never promote a total.
// Keep the parent library's naming convention for the included unit source.
#[allow(dead_code, clippy::enum_variant_names)]
#[path = "../../../../src/cryptanalysis/bitops.rs"]
mod bitops;
use bitops::{Field, Prim, Widths};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

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

fn uniform_event(v: &Value) -> Option<[u64; 4]> {
    let c = v["calls"].as_u64()?;
    let d = v["raw_draws"].as_u64()?;
    let r = v["rejections"].as_u64()?;
    let other = v["other_modulus_calls"].as_u64()?;
    if v["schema"] != "modular-uniform-events/v1"
        || v["complete"] != false
        || other != 0
        || d != c.checked_add(r)?
        || (c == 0 && d != 0)
        || v["priced_native"]["integer_mul64"] != d
        || v["priced_native"]["integer_add64"] != c
        || v["unpriced_native"]["integer_sub64"] != c.checked_mul(2)?
        || v["unpriced_native"]["integer_clz64"] != c
        || v["unpriced_native"]["integer_shl64"] != c
        || !declared_price(
            &v["known_native_priced_bitops"],
            (d as u128)
                .checked_mul(bitops::int_mul(64))?
                .checked_add((c as u128).checked_mul(bitops::int_add(64))?)?,
        )
    {
        return None;
    }
    Some([c, d, r, other])
}

fn uniform_sampling(v: &Value, outer: &Value) -> Option<()> {
    if v["schema"] != "cover.uniform-sampling-diagnostic/v1" || v["complete"] != false {
        return None;
    }
    let rows = v["phases"].as_array()?;
    if rows.len() != PHASES.len() {
        return None;
    }
    let primitive_phases = outer["phases"].as_array()?;
    let mut sum = [0u64; 4];
    for (i, (row, name)) in rows.iter().zip(PHASES).enumerate() {
        if row["phase"] != name {
            return None;
        }
        let c = uniform_event(&row["events"])?;
        if c[1] > primitive_phases.get(i)?["primitives"]["rng_words"].as_u64()? {
            return None;
        }
        for (slot, n) in sum.iter_mut().zip(c) {
            *slot = slot.checked_add(n)?;
        }
    }
    (uniform_event(&v["total"])? == sum).then_some(())
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
    if let Some(sampling) = v.get("uniform_sampling") {
        uniform_sampling(sampling, v)?;
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

pub(super) fn required_items(work: &Value, outer: &Value, required: &[&str]) -> bool {
    fn check(work: &Value, outer: &Value, required: &[&str]) -> Option<()> {
        let markers = outer["counted_items"].as_array()?;
        let actual: BTreeSet<_> = markers.iter().map(Value::as_str).collect::<Option<_>>()?;
        if actual.len() != markers.len() || required.iter().any(|m| !actual.contains(m)) {
            return None;
        }
        if required.contains(&"counted:modular_la_native_scans") {
            let solves = work["work_native"].as_array()?;
            if solves.is_empty()
                || solves.iter().any(|s| {
                    let p = &s["primitives"];
                    p["word_op"]
                        .as_u64()
                        .zip(p["rng_words"].as_u64())
                        .is_none_or(|(words, rng)| words <= rng)
                })
            {
                return None;
            }
        }
        if required.contains(&"counted:cover_residual_scalar_additions") {
            let additions = work["descent_residuals"].as_u64()?.checked_mul(2)?;
            if additions == 0 {
                return None;
            }
            let descent = outer["phases"]
                .as_array()?
                .iter()
                .find(|p| p["phase"] == "descent")?;
            if descent["primitives"]["modr_add"].as_u64()? < additions {
                return None;
            }
        }
        if required.contains(&"counted:selected_modular_uniform_draws") {
            let sampling = &outer["uniform_sampling"];
            uniform_sampling(sampling, outer)?;
            if sampling["total"]["calls"].as_u64()? == 0
                || sampling["total"]["rejections"].as_u64()? == 0
            {
                return None;
            }
        }
        Some(())
    }
    required.is_empty() || check(work, outer, required).is_some()
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
    fn required_scan_and_scalar_items_need_executed_counter_coverage() {
        let (mut work, mut outer, _) = fixture(false);
        let required = [
            "counted:modular_la_native_scans",
            "counted:cover_residual_scalar_additions",
        ];
        assert!(!required_items(&work, &outer, &required));
        outer["counted_items"] = json!(required);
        work["descent_residuals"] = json!(4);
        let mut scalar = tally(0);
        scalar["modr_add"] = json!(8);
        outer["phases"]
            .as_array_mut()
            .unwrap()
            .push(json!({"phase":"descent","primitives":scalar}));
        assert!(required_items(&work, &outer, &required));
        let mut zero = work.clone();
        zero["descent_residuals"] = json!(0);
        assert!(!required_items(&zero, &outer, &required));
        let mut bad = outer.clone();
        bad["phases"][1]["primitives"]["modr_add"] = json!(7);
        assert!(!required_items(&work, &bad, &required));
        let mut bad = work.clone();
        bad["work_native"][0]["primitives"]["rng_words"] = json!(10);
        assert!(!required_items(&bad, &outer, &required));
        let mut bad = outer.clone();
        bad["counted_items"] = json!([required[0], required[0]]);
        assert!(!required_items(&work, &bad, &required));
    }

    #[test]
    fn selected_uniform_events_reprice_and_keep_unpriced_counts() {
        fn event(c: u64, d: u64) -> Value {
            json!({"schema":"modular-uniform-events/v1","complete":false,"calls":c,"raw_draws":d,"rejections":d-c,"other_modulus_calls":0,
                "priced_native":{"integer_mul64":d,"integer_add64":c},"known_native_priced_bitops":(d as u128*24256+c as u128*320).to_string(),
                "unpriced_native":{"integer_sub64":2*c,"integer_clz64":c,"integer_shl64":c}})
        }
        let mut rows: Vec<_> = PHASES
            .into_iter()
            .map(|name| json!({"phase":name,"events":event(0,0)}))
            .collect();
        rows[6]["events"] = event(1, 2);
        let sampling = json!({"schema":"cover.uniform-sampling-diagnostic/v1","complete":false,"phases":rows,"total":event(1,2)});
        let mut phases: Vec<_> = PHASES
            .into_iter()
            .map(|name| json!({"phase":name,"primitives":tally(0)}))
            .collect();
        phases[6]["primitives"]["word_op"] = json!(4);
        phases[6]["primitives"]["rng_words"] = json!(2);
        let outer = json!({"phases":phases,"uniform_sampling":sampling,"counted_items":["counted:selected_modular_uniform_draws"]});
        assert!(uniform_sampling(&sampling, &outer).is_some());
        assert!(required_items(
            &Value::Null,
            &outer,
            &["counted:selected_modular_uniform_draws"]
        ));
        for bad in 0..6 {
            let mut s = sampling.clone();
            match bad {
                0 => s["total"]["known_native_priced_bitops"] = json!("0"),
                1 => s["phases"][6]["events"]["rejections"] = json!(0),
                2 => s["total"]["unpriced_native"]["integer_clz64"] = json!(0),
                3 => s["total"]["complete"] = json!(true),
                4 => s["phases"][6]["events"]["other_modulus_calls"] = json!(1),
                _ => s["phases"][6]["phase"] = json!("model"),
            }
            assert!(uniform_sampling(&s, &outer).is_none(), "case {bad}");
        }
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

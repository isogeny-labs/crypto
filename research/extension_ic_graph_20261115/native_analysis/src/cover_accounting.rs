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
    let mut sum: BTreeMap<_, u64> = KINDS.into_iter().map(|k| (k.to_string(), 0)).collect();
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

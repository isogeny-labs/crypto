//! Word-RAM reference pricing: every native counter at a fixed,
//! host-independent price.
//!
//! This adopts the accounting of CurveSmash
//! (<https://github.com/zac-williamson/curvesmash/blob/main/docs/COST_MODEL.md>),
//! itself HashSmash's: one reference operation, the target's affine point
//! addition, costs 1; any other work costs its operation count on a
//! 256-bit word RAM divided by the fixed word-operation budget `C` of the
//! reference addition.  The budgets are written down, not measured, so
//! a record's price is a function of its counts and the formulas below
//! alone, on every host.  They are conservative portable allowances, not
//! timings or the fastest implementations; they define the unit.
//!
//! The `*_wordram` methods run the same solver as their `*_full` twins
//! and charge the same counters by the same rules
//! ([`full_price::reprice_with`]); only the price of each unit differs.
//! `full_prices.json` and every method that reads it are untouched.
//!
//! The 256-bit word RAM: load, store, add and subtract modulo `2^256`,
//! one bitwise AND/OR/XOR/NOT, shift or rotate, compare, conditional
//! branch, and one uniform 256-bit word each cost one operation.  For a
//! `k`-bit field in `s` words CurveSmash fixes
//!
//! ```text
//! A = 128(s+1)                    reduced addition or subtraction
//! M = k(2A+64) + 128(s+1)         double-and-add multiplication
//! I = 2k(M+64) + 128(s+1)         Fermat inversion, binary powering
//! C = I + 16M + 32A + 4096        affine addition, doubling, exceptions
//! ```
//!
//! and those are the prime-field budgets here, unchanged
//! (`reference_costs` reproduces CurveSmash's secp256k1 `C = 78,037,248`).
//! In characteristic 2 a squaring is linear: spread `k` bits and fold
//! `k` high bits back, at most eight operations per bit, so
//! `S = 8k + 128(s+1)`, and Fermat inversion `x^(2^k−2)` takes one
//! squaring and one multiplication per exponent bit,
//! `I = k(S+M+128) + 128(s+1)`.  `A`, `M` and `C` keep CurveSmash's form.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::full_price::{self, UNITS};
use crate::cryptanalysis::ecbench::methods::SolveReport;
use crate::cryptanalysis::ecbench::workload::CurveFacts;

/// The model's identity, written into every repriced record.
pub const MODEL: &str = "ecbench.wordram/v1";

/// Bits per machine word, CurveSmash's.
pub const WORD_BITS: u32 = 256;

/// The method a `*_wordram` id runs before repricing.
pub fn base_of(id: &str) -> Option<&'static str> {
    match id {
        "ic.pipeline_wordram" => Some("ic.pipeline"),
        "rho.signed_frobenius_wordram" => Some("rho.signed_frobenius"),
        "rho.signed_frobenius_strong_wordram" => Some("rho.signed_frobenius_strong"),
        _ => None,
    }
}

/// The field a curve's arithmetic runs in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    /// `GF(p)` with `p` of this many bits.
    Prime(u32),
    /// `GF(2^m)`.
    Binary(u32),
}

/// Reference word-operation budgets for one field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budgets {
    pub add: u64,
    pub mul: u64,
    pub sqr: u64,
    pub inv: u64,
    /// The reference affine addition, `C`.
    pub point: u64,
}

fn words(bits: u32) -> u64 {
    u64::from(bits.div_ceil(WORD_BITS).max(1))
}

/// The budgets of [`Field`], by the formulas in the module comment.
pub fn reference_costs(field: Field) -> Budgets {
    let (k, binary) = match field {
        Field::Prime(k) => (u64::from(k), false),
        Field::Binary(m) => (u64::from(m), true),
    };
    let s = words(k as u32);
    let base = 128 * (s + 1);
    let add = base;
    let mul = k * (2 * add + 64) + base;
    let (sqr, inv) = if binary {
        let sqr = 8 * k + base;
        (sqr, k * (sqr + mul + 128) + base)
    } else {
        (mul, 2 * k * (mul + 64) + base)
    };
    Budgets {
        add,
        mul,
        sqr,
        inv,
        point: inv + 16 * mul + 32 * add + 4096,
    }
}

/// The word-operation budget of every unit in [`UNITS`] for a curve over
/// `field` whose subgroup order has `r_bits` bits.
///
/// - `double`: the reference operation covers doubling, so `C`.
/// - `frobenius`: `(x, y) ↦ (x², y²)`, two squarings.
/// - `canon`: the signed-Frobenius class minimum of `2m` points: `m−1`
///   Frobenius maps, `m` negations `y ↦ x + y`, and a compare and branch
///   for each of the `2m` candidates; on a prime field the negation
///   class: one negation, a compare and a branch.
/// - `as_solve`: `z² + z = c` by the half-trace, `m−1` squarings and
///   `(m+1)/2` additions, plus the trace test, one addition.
/// - `lookup`: four additions' allowance: hash the key, form the slot
///   address, load and compare the key, branch and move the value.  The
///   word RAM charges one operation per transfer whatever the cache
///   level; a table's memory traffic is not priced beyond that.
/// - `row_op`: one entry multiply-subtract modulo `r`: a multiplication
///   and a reduced subtraction at `r_bits`, two loads and a store.
/// - `word_xor`: one XOR.
pub fn unit_word_ops(field: Field, r_bits: u32) -> BTreeMap<&'static str, u64> {
    let b = reference_costs(field);
    let rb = reference_costs(Field::Prime(r_bits));
    let canon = match field {
        Field::Binary(m) => {
            let m = u64::from(m);
            (m - 1) * 2 * b.sqr + m * b.add + 2 * m * 2
        }
        Field::Prime(_) => b.add + 2,
    };
    let as_solve = match field {
        Field::Binary(m) => {
            let m = u64::from(m);
            (m - 1) * b.sqr + m.div_ceil(2) * b.add + b.add
        }
        // No Artin–Schreier equation in odd characteristic; a square
        // root by Fermat powering is the nearest budget.
        Field::Prime(_) => b.inv,
    };
    let out: BTreeMap<&'static str, u64> = [
        ("double", b.point),
        ("as_solve", as_solve),
        ("lookup", 4 * b.add),
        ("row_op", rb.mul + rb.add + 3),
        ("frobenius", 2 * b.sqr),
        ("canon", canon),
        ("field_mul", b.mul),
        ("field_sqr", b.sqr),
        ("field_inv", b.inv),
        ("word_xor", 1),
    ]
    .into_iter()
    .collect();
    debug_assert!(UNITS.iter().all(|u| out.contains_key(u)));
    out
}

/// The field of a curve, from its facts.
pub fn field_of(curve: &CurveFacts) -> Result<Field, String> {
    match (curve.field_degree, curve.field_bits) {
        (Some(m), _) => Ok(Field::Binary(m)),
        (None, Some(k)) => Ok(Field::Prime(k)),
        (None, None) => Err(format!("{}: no field size to price", curve.slug)),
    }
}

fn bits(r: u128) -> u32 {
    128 - r.leading_zeros()
}

/// Every unit of [`UNITS`] as its budget over `C`.
pub fn ratios(field: Field, r_bits: u32) -> BTreeMap<String, f64> {
    let c = reference_costs(field).point as f64;
    unit_word_ops(field, r_bits)
        .into_iter()
        .map(|(u, w)| (u.to_string(), w as f64 / c))
        .collect()
}

/// Every budget, `C` and ratio for one field, as records carry them.
pub fn describe(field: Field, r_bits: u32) -> Value {
    let b = reference_costs(field);
    let (kind, k) = match field {
        Field::Prime(k) => ("prime", k),
        Field::Binary(m) => ("binary", m),
    };
    json!({
        "model": MODEL,
        "source": "src/cryptanalysis/ecbench/wordram_price.rs",
        "after": "https://github.com/zac-williamson/curvesmash/blob/main/docs/COST_MODEL.md",
        "word_bits": WORD_BITS,
        "field": {"kind": kind, "bits": k, "words": words(k)},
        "r_bits": r_bits,
        "reference_operation": "affine point addition",
        "reference_word_ops": b.point,
        "field_word_ops": {"add": b.add, "mul": b.mul, "sqr": b.sqr, "inv": b.inv},
        "unit_word_ops": unit_word_ops(field, r_bits),
        "units": ratios(field, r_bits),
    })
}

/// [`describe`] for a built curve.
pub fn describe_curve(curve: &CurveFacts) -> Result<Value, String> {
    Ok(describe(field_of(curve)?, bits(curve.r)))
}

/// Reprice a report of the base method in place, as
/// [`full_price::reprice`] does with the measured table.
pub fn reprice(rep: &mut SolveReport, family: &str, curve: &CurveFacts) -> Result<(), String> {
    let field = field_of(curve)?;
    let r_bits = bits(curve.r);
    let units = ratios(field, r_bits);
    let by_counter = full_price::reprice_with(rep, family, &units);
    let mut pricing = describe(field, r_bits);
    pricing["gae_by_counter"] = json!(by_counter);
    if let Value::Object(d) = &mut rep.detail {
        d.insert("wordram_pricing".into(), pricing);
    } else {
        let old = std::mem::take(&mut rep.detail);
        rep.detail = json!({"base": old, "wordram_pricing": pricing});
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::methods::PhaseRecord;

    #[test]
    fn curvesmash_reference_prices_reproduce() {
        // docs/COST_MODEL.md: secp256k1's affine addition.
        assert_eq!(reference_costs(Field::Prime(256)).point, 78_037_248);
        // And its Fp12 product, 144M + 188A + 65536 at k=381, s=2.
        let b = reference_costs(Field::Prime(381));
        assert_eq!(144 * b.mul + 188 * b.add + 65_536, 45_839_872);
    }

    #[test]
    fn binary_budgets_follow_the_formulas() {
        let b = reference_costs(Field::Binary(31));
        assert_eq!(b.add, 256);
        assert_eq!(b.mul, 31 * 576 + 256);
        assert_eq!(b.sqr, 8 * 31 + 256);
        assert_eq!(b.inv, 31 * (b.sqr + b.mul + 128) + 256);
        assert_eq!(b.point, b.inv + 16 * b.mul + 32 * b.add + 4096);
        let r = ratios(Field::Binary(31), 30);
        assert_eq!(r.len(), UNITS.len());
        assert_eq!(r["double"], 1.0);
        assert!(r.values().all(|v| *v > 0.0 && *v <= 1.0));
    }

    #[test]
    fn every_wordram_id_has_a_base_shared_with_full() {
        for (w, f) in [
            ("ic.pipeline_wordram", "ic.pipeline_full"),
            ("rho.signed_frobenius_wordram", "rho.signed_frobenius_full"),
            (
                "rho.signed_frobenius_strong_wordram",
                "rho.signed_frobenius_strong_full",
            ),
        ] {
            assert_eq!(base_of(w), full_price::base_of(f));
        }
    }

    #[test]
    fn reprice_charges_by_the_same_rules() {
        let mut rep = SolveReport {
            recovered: None,
            exhausted: false,
            phases: vec![PhaseRecord {
                name: "relations".into(),
                adds: 10,
                doubles: 2,
                native: [
                    ("lookups".to_string(), 4u64),
                    ("mystery_work".to_string(), 3),
                ]
                .into_iter()
                .collect(),
                ..Default::default()
            }],
            total_gae: 0.0,
            automorphisms_used: 2,
            counters: BTreeMap::new(),
            unpriced: vec!["lookups_uncharged".into()],
            deterministic: true,
            nondeterminism: vec![],
            solve_wall_ns: 0,
            factor_base: None,
            detail: json!({}),
            online: None,
            online_error: None,
            solver: None,
            field_ops: None,
        };
        let units = ratios(Field::Binary(31), 30);
        full_price::reprice_with(&mut rep, "ic", &units);
        assert_eq!(rep.unpriced, vec!["mystery_work_uncharged".to_string()]);
        assert!((rep.total_gae - (12.0 + 4.0 * units["lookup"])).abs() < 1e-12);
    }
}

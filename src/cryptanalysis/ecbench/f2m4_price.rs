//! Full pricing for curves over `F_{(2^m)^4}`: every native counter of
//! `ic.pipeline` (with `gaudry-d4-bin`) and of the two negation-map rho
//! methods charged in one unit, so that no record on either side of the
//! comparison is a lower bound unless a counter has no rule here.
//!
//! **The unit.**  The primary unit of the k = 4 comparison is the counted
//! `F_q` multiplication: one affine group addition is `fq_muls_per_add`
//! of them (the `Gf2q` counter's delta over one addition), the solver's
//! cost is its counted multiplications, and `GAE = F_q multiplications /
//! fq_muls_per_add`.  Native work that is not a counted multiplication
//! (canonicalisations, key and hash evaluations, table probes and inserts,
//! scalar arithmetic modulo `r`, the solver's table inversions and the
//! matrix elements its passes touch, large-prime arithmetic, the
//! recent-point window, ...) is priced here in *`F_q`-multiplication
//! equivalents* (FME): the time of the operation divided by the time of
//! one `F_q` multiplication in the solver's own inner loop (a dense
//! `axpy` element), measured on the same field by `ecbench
//! price-units-f2m4` and committed in `f2m4_full_prices.json`.  A record's
//! price is therefore a function of its counts and that file alone.
//!
//! **Convention, applied identically to both arms.**  A counted `F_q`
//! multiplication, and an affine group operation of `fq_muls_per_add`
//! multiplications, absorb the `F_q` additions and the loop control of
//! their own formulas; the repository's unit has always done so and the
//! rho and index-calculus arms are charged alike.  Everything else is a
//! native counter and has a rule below.
//!
//! **Lower bounds.**  A counter that is nonzero and has no rule stays
//! unpriced (`<name>_uncharged`), so the record is still marked a lower
//! bound rather than silently dropping work.
//!
//! **The §5a band.**  Every full record exposes, in
//! `detail.full_pricing`, the per-counter counts and charges and the
//! total of native operations, so the same record can be read with its
//! native work at 0, 0.1 and 1 GAE per operation: see [`band`].

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hint::black_box;
use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::canonical::sha256_hex;
use crate::cryptanalysis::ecbench::methods::{PhaseRecord, SolveReport};
use crate::cryptanalysis::gaudry_quartic_bin::{wide_key, BinCurve, Fe, PtB};
use crate::cryptanalysis::ic_boundary::{fast_map, CountedGroup, FastMap};
use crate::cryptanalysis::koblitz_strong_rho::RhoScalar;

/// The measured table, compiled in.
pub const F2M4_PRICES: &str = include_str!("f2m4_full_prices.json");

/// The whole native price table re-measured in one interleaved pass on one
/// host, with the three mod-`r` units taken as the smaller of the code's and
/// a Montgomery kernel's price (see [`Mulmod::Onepass`]), compiled in.  The
/// older `f2m4_mulmod_prices.json` (Montgomery units only, measured on
/// another host and with a different kernel) is kept as a record and read by
/// nothing.
pub const F2M4_ONEPASS_PRICES: &str = include_str!("f2m4_onepass_prices.json");

/// The method ids that reprice, with the base each wraps.
pub fn base_of(id: &str) -> Option<&'static str> {
    match id {
        "ic.pipeline_f2m4_full" => Some("ic.pipeline"),
        "rho.negation_batched_full" => Some("rho.negation_batched"),
        "rho.negation_f2m4_full" => Some("rho.negation"),
        _ => None,
    }
}

/// Every unit of a table row, as the ratio of its time to one counted
/// `F_q` multiplication (a dense `axpy` element) on the same field.
pub const UNITS: &[&str] = &[
    "fq_inv_tab",
    "elem_fill",
    "elem_scan",
    "axpy_skip",
    "par_region",
    "monomial_hash_op",
    "rng_seed",
    "rng_draw",
    "fast_probe",
    "fast_insert",
    "guard_insert",
    "key_eval",
    "neg_eval",
    "mix_eval",
    "pack_eval",
    "canon_batched",
    "hash_x",
    "window_scan_elem",
    "window_push",
    "dp_probe",
    "dp_insert",
    "scalar_mod_op",
    "scalar_mulmod",
    "scalar_invmod",
    "row_op",
    "lp_mulmod",
    "lp_addsub",
    "lp_entry",
    "btree_op",
    "relation_row",
    "jump_shuffle",
    "round_job",
    "walk_start",
];

/// The key of a table row: `m{m}`, or `m{m}_u128` for a degree measured on
/// both widths of subgroup order (only a degree whose orders straddle a
/// word has two).
pub fn row_key(m: u32, wide: bool) -> String {
    if wide {
        format!("m{m}_u128")
    } else {
        format!("m{m}")
    }
}

/// The table row for `m` and the width of the subgroup order, as unit →
/// FME.  A row measured on a curve whose order has the other width is
/// refused: the arithmetic modulo `r` costs several times more past a word.
pub fn row(m: u32, wide: bool) -> Result<BTreeMap<String, f64>, String> {
    let doc: Value = serde_json::from_str(F2M4_PRICES).map_err(|e| e.to_string())?;
    let want = if wide { 128 } else { 64 };
    let instances = doc.get("instances").ok_or("the table has no instances")?;
    let (key, inst) = [row_key(m, true), row_key(m, false)]
        .into_iter()
        .filter_map(|k| instances.get(&k).map(|v| (k, v)))
        .find(|(_, v)| v.get("scalar_width_bits").and_then(Value::as_u64) == Some(want))
        .ok_or_else(|| {
            format!(
                "src/cryptanalysis/ecbench/f2m4_full_prices.json has no row for m = {m} with \
                 {want}-bit orders; measure it with `ecbench price-units-f2m4 --m {m}{}`",
                if wide { " --wide" } else { "" }
            )
        })?;
    let r = inst
        .get("row")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("row `{key}` has no units"))?;
    let mut out = BTreeMap::new();
    for u in UNITS {
        let v = r
            .get(*u)
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("f2m4_full_prices.json row {key} lacks `{u}`"))?;
        out.insert((*u).to_string(), v);
    }
    Ok(out)
}

/// Which table, and which multiplication modulo `r`, prices the units.
///
/// `Code` is the committed `f2m4_full_prices.json` (the older host), with the
/// repository's own mod-`r` arithmetic: `koblitz_strong_rho`'s `mul_mod_u128`
/// is a chunked `u128 %` that costs 19 times more at 68-bit orders than at
/// 64-bit ones (an implementation step, not the problem's).  Every legacy
/// and `*_ref` id uses it.
///
/// The three `Onepass` variants read `f2m4_onepass_prices.json` instead: the
/// whole table (every unit, not only the mod-`r` ones) measured in one
/// interleaved pass on one host, with a warm-up pass, so no row mixes hosts
/// or kernels.  The mod-`r` units (`scalar_mulmod`, `row_op`, `lp_mulmod`)
/// are measured there on the code's kernel and on a Montgomery kernel (u64
/// storage and one limb for `r < 2^64`, u128 storage and two limbs
/// otherwise), side by side:
/// * `Onepass` prices each of the three as the smaller of the two (what a
///   competent implementation reaches, with no step where the order crosses
///   a word that the other kernel does not also have);
/// * `OnepassCode` prices them on the code's kernel only;
/// * `OnepassMont` on the Montgomery kernel only.
///
/// Counts never change; only prices do.  The three are recorded side by side
/// by the `*_refmm` ids so that neither choice is hidden.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mulmod {
    Code,
    Onepass,
    OnepassCode,
    OnepassMont,
}

impl Mulmod {
    /// The same table with the code's mod-`r` kernel.
    pub fn code_of(self) -> Mulmod {
        match self {
            Mulmod::Code => Mulmod::Code,
            _ => Mulmod::OnepassCode,
        }
    }
}

/// The units the mod-`r` kernel choice moves.
pub const MULMOD_UNITS: &[&str] = &["scalar_mulmod", "row_op", "lp_mulmod"];

/// [`row`], or the one-pass table's row for `mulmod`: unit → FME.
pub fn row_mm(m: u32, wide: bool, mulmod: Mulmod) -> Result<BTreeMap<String, f64>, String> {
    let field = match mulmod {
        Mulmod::Code => return row(m, wide),
        Mulmod::Onepass => "row",
        Mulmod::OnepassCode => "row_code",
        Mulmod::OnepassMont => "row_montgomery",
    };
    let doc: Value = serde_json::from_str(F2M4_ONEPASS_PRICES).map_err(|e| e.to_string())?;
    let want = if wide { 128 } else { 64 };
    let key = row_key(m, wide);
    let inst = doc
        .get("instances")
        .and_then(|i| i.get(&key))
        .filter(|v| v.get("scalar_width_bits").and_then(Value::as_u64) == Some(want))
        .ok_or_else(|| {
            format!(
                "src/cryptanalysis/ecbench/f2m4_onepass_prices.json has no row `{key}`; measure it \
                 with `ecbench price-units-f2m4 --onepass --m {m}`",
            )
        })?;
    let r = inst
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| format!("f2m4_onepass_prices.json row {key} has no `{field}`"))?;
    let mut out = BTreeMap::new();
    for u in UNITS {
        let v = r
            .get(*u)
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("f2m4_onepass_prices.json row {key} `{field}` lacks `{u}`"))?;
        out.insert((*u).to_string(), v);
    }
    Ok(out)
}

// ── Rules ───────────────────────────────────────────────────────────────

/// How one counter is charged.
#[derive(Clone, Copy)]
enum Rule {
    /// A count of outcomes or shape, no work of its own (or work charged
    /// through another counter, named in the rule table's comment).
    Outcome,
    /// Work already in the group operations or counted `F_q`
    /// multiplications of the phase.
    Counted,
    /// A sum of units: FME = Σ k · unit · count.
    Units(&'static [(&'static str, f64)]),
}

/// Every counter of `rho.negation_batched` and `rho.negation` (as
/// `rho.negation_f2m4_full` reports it) that names work or its absence.
fn rho_rule(counter: &str, batched: bool) -> Option<Rule> {
    use Rule::*;
    Some(match counter {
        // Counts of outcomes, parameters, and group operations that the
        // F_q counter (batched) or the group-operation ledger
        // (unbatched) already prices.
        "walks_parallel"
        | "rounds"
        | "distinguished_points"
        | "useless_collisions"
        | "steps"
        | "look_ahead_additions"
        | "cycle_escape_doublings"
        | "cycles_length_2"
        | "cycles_length_4"
        | "cycles_other_length"
        | "walks_abandoned_in_a_cycle"
        | "walks_capped"
        | "batched_additions"
        | "unbatched_operations"
        | "batch_inversions"
        | "batch_fq4_multiplications"
        | "jumps"
        | "jump_coefficient_bits"
        | "distinguished_point_bits"
        | "setup_additions"
        | "setup_doublings"
        | "start_additions"
        | "walk_operations"
        | "verification_operations"
        | "fq_muls_per_add"
        | "canonical_flips"
        | "walks"
        // A canonicalisation's four F_q additions are inside its
        // `canon_batched` unit; the unbatched walk's are inside `neg_eval`
        // and `key_eval`.
        | "canonicalisation_fq_additions" => Outcome,
        "fq_operations_total"
        | "fq_operations_setup"
        | "fq_operations_batch_inversion"
        | "fq_operations_batched_formula"
        | "fq_operations_unbatched"
        | "fq_operations_verification" => Counted,
        // Batched: one canonicalisation compares two packed ordinates; the
        // unbatched walk's is charged through its key and negation
        // evaluations below.
        "canonicalisations" if batched => Units(&[("canon_batched", 1.0)]),
        "canonicalisations" => Outcome,
        "hash_evaluations" => Units(&[("hash_x", 1.0)]),
        "walks_started" => Units(&[("walk_start", 1.0)]),
        // One job per accepted step or look-ahead retry.
        "round_jobs" => Units(&[("round_job", 1.0)]),
        // A batched scan packs the abscissa once and a push packs it again;
        // the unbatched walk's keys are counted in `key_evaluations`.
        "window_scans" if batched => Units(&[("pack_eval", 1.0)]),
        "window_scans" => Outcome,
        "window_scan_elements" => Units(&[("window_scan_elem", 1.0)]),
        "window_pushes" if batched => Units(&[("window_push", 1.0), ("pack_eval", 1.0)]),
        "window_pushes" => Units(&[("window_push", 1.0)]),
        // Unbatched: the keys, mixes and negations the walk evaluates.
        "key_evaluations" => Units(&[("key_eval", 1.0)]),
        "mix_evaluations" => Units(&[("mix_eval", 1.0)]),
        "negation_evaluations" => Units(&[("neg_eval", 1.0)]),
        // Both.
        "distinguished_point_table_queries" => Units(&[("dp_probe", 1.0)]),
        "distinguished_point_table_inserts" => Units(&[("dp_insert", 1.0)]),
        "scalar_mod_r_operations" => Units(&[("scalar_mod_op", 1.0)]),
        "scalar_mulmods" => Units(&[("scalar_mulmod", 1.0)]),
        "scalar_invmods" => Units(&[("scalar_invmod", 1.0)]),
        "rng_draws" => Units(&[("rng_draw", 1.0)]),
        _ => return None,
    })
}

/// Every counter of the `ic.pipeline` records on a curve over
/// `F_{(2^m)^4}` (both factor-base bases, both linear algebras, with and
/// without large primes), plus the counters the full method injects.
fn ic_rule(counter: &str) -> Option<Rule> {
    use Rule::*;
    Some(match counter {
        // Outcomes and shapes.
        "relations"
        | "columns"
        | "rank"
        | "rows"
        | "single_column_rows"
        | "two_column_rows"
        | "repeated_column_rows"
        | "pinned_by_repeated_row"
        | "direct_relations_skipped"
        | "repeated_targets_skipped"
        | "lift_failures"
        | "unliftable_systems"
        | "frobfold_mismatches"
        | "s4_hits"
        | "trials"
        | "solver_budget_exceeded"
        | "walk_pool_offsets"
        | "walk_pool_restarts"
        | "walk_pool_ops"
        | "lp_decompositions_0_large"
        | "lp_decompositions_1_large"
        | "lp_decompositions_2_large"
        | "lp_discarded"
        | "lp_full_direct"
        | "lp_full_merged"
        | "lp_full_weight"
        | "lp_merges"
        | "lp_small_columns"
        | "lp_stored_1_large"
        | "lp_stored_2_large"
        | "lp_stored_pivots"
        | "inconsistent"
        | "full_rank_unpinned"
        | "verification_failures"
        | "solver_calls" => Outcome,
        // Counted F_q multiplications (the solver's, including its
        // set-up) and the base lifts (their counted multiplications and
        // the `x_of_t` before each).
        "solver_ops" | "sqrt_solves" => Counted,
        // Each trial inserts its target's key into the guard set.
        "target_guard_probes" => Units(&[("guard_insert", 1.0), ("key_eval", 1.0)]),
        "lookups" => Units(&[("fast_probe", 1.0)]),
        "row_ops" => Units(&[("row_op", 1.0)]),
        // One key, one hash and two coefficient additions per walk step.
        "walk_steps" => Units(&[("key_eval", 1.0), ("mix_eval", 1.0), ("scalar_mod_op", 2.0)]),
        // A restart permutes the jump table and adds an offset.
        "walk_restarts" => Units(&[("jump_shuffle", 1.0), ("scalar_mod_op", 2.0)]),
        // Two draws per jump or pool offset.
        "walk_jumps" => Units(&[("rng_draw", 2.0)]),
        // Counters the full method injects.
        "relation_rows_built" => Units(&[("relation_row", 1.0)]),
        "solver_table_inversions" => Units(&[("fq_inv_tab", 1.0)]),
        "solver_element_fills" => Units(&[("elem_fill", 1.0)]),
        "solver_element_scans" => Units(&[("elem_scan", 1.0)]),
        "solver_element_skips" => Units(&[("axpy_skip", 1.0)]),
        "solver_parallel_regions" => Units(&[("par_region", 1.0)]),
        "solver_monomial_hash_ops" => Units(&[("monomial_hash_op", 1.0)]),
        "solver_systems_built" => Units(&[("rng_seed", 1.0)]),
        "witness_lookups" => Units(&[("fast_probe", 1.0), ("key_eval", 1.0)]),
        "lp_modular_multiplications" => Units(&[("lp_mulmod", 1.0)]),
        "lp_additions_subtractions" => Units(&[("lp_addsub", 1.0)]),
        "lp_entries_merged" => Units(&[("lp_entry", 1.0)]),
        "lp_pivot_probes" => Units(&[("dp_probe", 1.0)]),
        "lp_pivot_inserts" => Units(&[("dp_insert", 1.0)]),
        "lp_ordered_map_operations" => Units(&[("btree_op", 1.0)]),
        "factor_base_points" => {
            Units(&[("fast_insert", 2.0), ("fast_probe", 1.0), ("key_eval", 3.0)])
        }
        "factor_base_columns" => Units(&[("key_eval", 1.0), ("neg_eval", 4.0)]),
        "factor_base_sort_comparisons" => Units(&[("elem_scan", 1.0)]),
        _ => return None,
    })
}

/// One charge: a counter, its count, and what it costs.
#[derive(Clone, Debug)]
pub struct Charge {
    pub counter: String,
    pub count: u64,
    /// FME; `None` when the counter is a count of outcomes or work already
    /// in the counted unit.
    pub fme: f64,
    /// Whether the counter is native work priced by a unit (so it counts
    /// toward the band's native operations).
    pub native: bool,
    /// Rule applications per count: the sum of the rule's multipliers (a
    /// `walk_steps` count is one key, one mix and two mod-`r` operations,
    /// four applications; a `row_ops` count is one).
    pub applications_per_count: f64,
}

fn charge(rule: Rule, counter: &str, count: u64, u: &BTreeMap<String, f64>) -> Option<Charge> {
    match rule {
        Rule::Outcome | Rule::Counted => None,
        Rule::Units(us) => Some(Charge {
            counter: counter.to_string(),
            count,
            fme: us.iter().map(|(n, k)| k * u[*n] * count as f64).sum(),
            native: true,
            applications_per_count: us.iter().map(|(_, k)| *k).sum(),
        }),
    }
}

/// Reprice a batched- or unbatched-rho report in place.  Counters are
/// read from `rep.counters`; the group operations or `F_q` operations
/// already in `rep.total_gae` stay; native work becomes a `natives` phase.
pub fn reprice_rho(
    rep: &mut SolveReport,
    m: u32,
    wide: bool,
    per_add: f64,
    batched: bool,
) -> Result<BTreeMap<String, f64>, String> {
    reprice_rho_mm(rep, m, wide, per_add, batched, Mulmod::Code)
}

/// [`reprice_rho`], with the mod-`r` multiplications priced as `mulmod`.
pub fn reprice_rho_mm(
    rep: &mut SolveReport,
    m: u32,
    wide: bool,
    per_add: f64,
    batched: bool,
    mulmod: Mulmod,
) -> Result<BTreeMap<String, f64>, String> {
    let units = row_mm(m, wide, mulmod)?;
    let mut charges = Vec::new();
    let mut unknown = Vec::new();
    for (name, &count) in &rep.counters {
        if count == 0 {
            continue;
        }
        match rho_rule(name, batched) {
            Some(rule) => charges.extend(charge(rule, name, count, &units)),
            None => unknown.push(format!("{}_uncharged", name.trim_end_matches("_uncharged"))),
        }
    }
    finish(rep, per_add, &units, charges, unknown, "natives");
    Ok(units)
}

fn finish(
    rep: &mut SolveReport,
    per_add: f64,
    units: &BTreeMap<String, f64>,
    charges: Vec<Charge>,
    mut unknown: Vec<String>,
    phase: &str,
) {
    let fme: f64 = charges.iter().map(|c| c.fme).sum();
    let native_ops: u64 = charges.iter().map(|c| c.count).sum();
    let counted_gae = rep.total_gae;
    let mut p = PhaseRecord {
        name: phase.into(),
        gae: fme / per_add,
        ..Default::default()
    };
    for c in &charges {
        p.native.insert(c.counter.clone(), c.count);
    }
    rep.phases.push(p);
    rep.total_gae += fme / per_add;
    unknown.sort();
    unknown.dedup();
    rep.unpriced = unknown;
    let by_counter: BTreeMap<String, Value> = charges
        .iter()
        .map(|c| {
            (
                c.counter.clone(),
                json!({"count": c.count, "fme": c.fme, "gae": c.fme / per_add}),
            )
        })
        .collect();
    let block = json!({
        "unit": "GAE = (counted F_q multiplications + native work in F_q-multiplication equivalents) / fq_muls_per_add",
        "table": "src/cryptanalysis/ecbench/f2m4_full_prices.json",
        "table_sha256": sha256_hex(F2M4_PRICES.as_bytes()),
        "units_fme": units,
        "fq_muls_per_add": per_add,
        "counted_gae": counted_gae,
        "native_gae": fme / per_add,
        "native_operations": native_ops,
        "by_counter": by_counter,
        "band": band(counted_gae, native_ops),
        "band_rule_applications": band_rule_applications(counted_gae, per_add, &charges),
        "convention": "a counted F_q multiplication and an affine group operation absorb the F_q additions and loop control of their own formulas; every other native counter has a rule or the record stays a lower bound",
    });
    match &mut rep.detail {
        Value::Object(d) => {
            d.insert("full_pricing".into(), block);
        }
        other => {
            let old = std::mem::take(other);
            *other = json!({"base": old, "full_pricing": block});
        }
    }
}

/// §5a: the total with the native work at 0, 0.1 and 1 GAE per operation
/// instead of its measured price: `counted_gae + rate · native_operations`.
/// `counted_gae` is everything the unit counted (group operations,
/// counted `F_q` multiplications); native operations are the counts of the
/// counters priced by a unit.
pub fn band(counted_gae: f64, native_operations: u64) -> Value {
    json!({
        "native_gae_per_operation_0": counted_gae,
        "native_gae_per_operation_0.1": counted_gae + 0.1 * native_operations as f64,
        "native_gae_per_operation_1": counted_gae + native_operations as f64,
    })
}

/// §5a, comparable between the arms: the unit is one *rule application*,
/// a single priced unit invoked once (a key evaluation, a mix, one
/// mod-`r` operation, a row operation, a probe), the same on both arms.
/// A counter that bundles several (`walk_steps`: key, mix and two mod-`r`
/// operations) counts as the sum of its rule's multipliers, so the result
/// does not depend on how finely an arm splits its counters.  (The older
/// [`band`] adds the raw counts and is kept only so old readers parse.)
///
/// The record carries, per counter, the count, its applications, its FME
/// and GAE, so the band is recomputable for any rate or weighting:
/// `counted_gae + rate * applications`, or `counted_gae + native_gae`
/// (the weighted point, each application at its measured unit).
///
/// A flat rate per application is still blunt: the units span 0.1 to 100+
/// FME, so the weighted point is the estimate and the 0 / 0.1 / 1 points
/// bound how wrong a flat rate would be.  Legacy `ic.pipeline` records
/// (not `*_full`) never counted their unpriced work and have no band; a
/// record whose `unpriced` list is non-empty is a lower bound at every rate.
pub fn band_rule_applications(counted_gae: f64, per_add: f64, charges: &[Charge]) -> Value {
    let mut by: BTreeMap<String, (u64, f64, f64, f64)> = BTreeMap::new();
    for c in charges {
        let e = by.entry(c.counter.clone()).or_default();
        e.0 += c.count;
        e.1 += c.count as f64 * c.applications_per_count;
        e.2 += c.fme;
        e.3 = c.applications_per_count;
    }
    let apps: f64 = by.values().map(|e| e.1).sum();
    let native_gae: f64 = by.values().map(|e| e.2).sum::<f64>() / per_add;
    let counters: BTreeMap<String, Value> = by
        .into_iter()
        .map(|(k, (n, a, f, per))| {
            (
                k,
                json!({
                    "count": n,
                    "applications_per_count": per,
                    "applications": a,
                    "fme": f,
                    "gae": f / per_add,
                    "fme_per_application": if a > 0.0 { f / a } else { 0.0 },
                }),
            )
        })
        .collect();
    json!({
        "unit": "one rule application = one priced unit invoked once; a counter's applications = count x the sum of its rule's multipliers",
        "native_rule_applications": apps,
        "native_gae_per_application_0": counted_gae,
        "native_gae_per_application_0.1": counted_gae + 0.1 * apps,
        "native_gae_per_application_1": counted_gae + apps,
        "native_gae_weighted": counted_gae + native_gae,
        "by_counter": counters,
        "legacy_ic": "legacy ic.pipeline records never counted their unpriced work and have no band; use ic.pipeline_f2m4_full or its _ref/_refmm variants",
    })
}

// ── IC ──────────────────────────────────────────────────────────────────

/// What the index-calculus repricing needs besides the report.
pub struct IcContext<'a> {
    pub m: u32,
    /// The subgroup order does not fit a word.
    pub wide: bool,
    pub per_add: f64,
    /// Counted `F_q` multiplications of one base lift plus the `x_of_t`
    /// before it, per attempt.
    pub lift_muls: f64,
    pub x_of_t_muls: f64,
    pub oracle: &'a crate::cryptanalysis::gaudry_quartic_bin::OracleNative,
    /// Factor-base points, columns and abscissae.
    pub fb_points: u64,
    pub fb_columns: u64,
    pub fb_abscissae: u64,
    /// The legacy estimate the pipeline added to the solver's operations
    /// for large-prime merges (`lp_merge_mulmods`), replaced here by the
    /// counted arithmetic.
    pub lp_estimate: u64,
    /// The Weil-restriction system build and the interpolation
    /// precomputation, as the pipeline counted them inside `solver_ops`
    /// (this file's schoolbook `F_{q⁴}` arithmetic), and what the same
    /// work costs in this context's unit.  Both 0 for the current unit,
    /// which keeps the count as it is.
    pub f4_in_solver: u64,
    pub f4_in_solver_priced: f64,
    /// Which multiplication modulo `r` prices `row_op`, `lp_mulmod` and
    /// `scalar_mulmod`.
    pub mulmod: Mulmod,
}

/// Reprice an `ic.pipeline` report of a curve over `F_{(2^m)^4}` in place:
/// every phase is rebuilt from its group operations, its counted
/// multiplications and lifts, and a charge for every other native counter.
pub fn reprice_ic(
    rep: &mut SolveReport,
    ctx: &IcContext<'_>,
) -> Result<BTreeMap<String, f64>, String> {
    let units = row_mm(ctx.m, ctx.wide, ctx.mulmod)?;
    let per_add = ctx.per_add;
    let mut charges: Vec<Charge> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    // Counters the full method injects, by phase.
    let o = ctx.oracle;
    let mut extra: BTreeMap<&str, Vec<(&str, u64)>> = BTreeMap::new();
    let a = ctx.fb_abscissae.max(2);
    extra.insert(
        "factor_base",
        vec![
            ("factor_base_points", ctx.fb_points),
            ("factor_base_columns", ctx.fb_columns),
            (
                "factor_base_sort_comparisons",
                (a as f64 * (a as f64).log2()).ceil() as u64,
            ),
        ],
    );
    extra.insert(
        "relations",
        vec![
            ("solver_systems_built", o.solves),
            ("solver_table_inversions", o.table_inversions),
            ("solver_element_fills", o.touch.fills),
            ("solver_element_scans", o.touch.scans),
            ("solver_element_skips", o.touch.skips),
            ("solver_parallel_regions", o.touch.regions),
            ("solver_monomial_hash_ops", o.monomial_hash_ops),
            ("witness_lookups", o.witness_lookups),
            ("lp_modular_multiplications", o.lp_mulmods),
            ("lp_additions_subtractions", o.lp_addsubs),
            ("lp_entries_merged", o.lp_entries),
            ("lp_pivot_probes", o.lp_map_ops[0]),
            ("lp_pivot_inserts", o.lp_map_ops[1]),
            ("lp_ordered_map_operations", o.lp_map_ops[2]),
        ],
    );
    let mut total = 0.0;
    let mut counted_total = 0.0;
    for p in rep.phases.iter_mut() {
        // Group operations and counted multiplications.
        let mut counted = (p.adds + p.doubles) as f64;
        // `scalar_mults` is a tally of multiplications whose additions are
        // already in `adds` and `doubles`.
        let mut natives = p.native.clone();
        for (k, v) in extra.get(p.name.as_str()).into_iter().flatten() {
            if *v > 0 {
                natives.insert((*k).to_string(), *v);
            }
        }
        let mut native_fme = 0.0;
        for (name, &count) in &natives {
            if count == 0 {
                continue;
            }
            match ic_rule(name) {
                Some(Rule::Counted) => match name.as_str() {
                    "solver_ops" => {
                        // The pipeline's operations include its estimate for
                        // the large-prime merges; those are counted below.
                        let f4 = ctx.f4_in_solver.min(count - ctx.lp_estimate.min(count));
                        counted += ((count - ctx.lp_estimate.min(count) - f4) as f64
                            + ctx.f4_in_solver_priced)
                            / per_add;
                    }
                    "sqrt_solves" => {
                        counted += count as f64 * (ctx.lift_muls + ctx.x_of_t_muls) / per_add;
                    }
                    _ => {}
                },
                Some(rule) => {
                    if let Some(c) = charge(rule, name, count, &units) {
                        native_fme += c.fme;
                        charges.push(c);
                    }
                }
                None => unknown.push(format!("{}_uncharged", name.trim_end_matches("_uncharged"))),
            }
        }
        p.native = natives;
        p.gae = counted + native_fme / per_add;
        counted_total += counted;
        total += p.gae;
    }
    // The pipeline's top-level counters are shapes (matrix rank and rows,
    // targets tried, relations found), not work; anything else is not
    // known to be.
    for (name, &count) in &rep.counters {
        let shape =
            name.starts_with("matrix_") || name == "targets_tried" || name == "relations_found";
        if count > 0 && !shape {
            unknown.push(format!("{}_uncharged", name.trim_end_matches("_uncharged")));
        }
    }
    rep.total_gae = total;
    // Solver wall-time items stay what they were; native counters get rules.
    let mut kept: Vec<String> = rep
        .unpriced
        .iter()
        .filter(|u| u.starts_with("solver_") || u.starts_with("counts_lost"))
        .cloned()
        .collect();
    kept.extend(unknown);
    kept.sort();
    kept.dedup();
    rep.unpriced = kept;
    let native_ops: u64 = charges.iter().map(|c| c.count).sum();
    let native_gae: f64 = charges.iter().map(|c| c.fme).sum::<f64>() / per_add;
    let by_counter: BTreeMap<String, Value> = {
        let mut m: BTreeMap<String, (u64, f64)> = BTreeMap::new();
        for c in &charges {
            let e = m.entry(c.counter.clone()).or_default();
            e.0 += c.count;
            e.1 += c.fme;
        }
        m.into_iter()
            .map(|(k, (n, f))| (k, json!({"count": n, "fme": f, "gae": f / per_add})))
            .collect()
    };
    let block = json!({
        "unit": "GAE = (counted F_q multiplications + native work in F_q-multiplication equivalents) / fq_muls_per_add",
        "table": "src/cryptanalysis/ecbench/f2m4_full_prices.json",
        "table_sha256": sha256_hex(F2M4_PRICES.as_bytes()),
        "units_fme": units,
        "fq_muls_per_add": per_add,
        "counted_gae": counted_total,
        "native_gae": native_gae,
        "native_operations": native_ops,
        "large_prime_estimate_replaced": ctx.lp_estimate,
        "by_counter": by_counter,
        "band": band(counted_total, native_ops),
        "band_rule_applications": band_rule_applications(counted_total, per_add, &charges),
        "convention": "a counted F_q multiplication and an affine group operation absorb the F_q additions and loop control of their own formulas; every other native counter has a rule or the record stays a lower bound",
    });
    match &mut rep.detail {
        Value::Object(d) => {
            d.insert("full_pricing".into(), block);
        }
        other => {
            let old = std::mem::take(other);
            *other = json!({"base": old, "full_pricing": block});
        }
    }
    Ok(units)
}

// ── Measuring the units ─────────────────────────────────────────────────

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn ns_per<F: FnMut(u64) -> u64>(iters: u64, mut f: F) -> f64 {
    let start = Instant::now();
    let mut acc = 0u64;
    for i in 0..iters {
        acc = acc.wrapping_add(f(i));
    }
    black_box(acc);
    start.elapsed().as_nanos() as f64 / iters as f64
}

/// `ic_boundary::mix`, the walk's hash (private there).
fn mix(v: u64) -> u64 {
    let mut h = v ^ 0x2545_F491_4F6C_DD1D;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    h
}

fn scalar_units<S: RhoScalar>(r: S, rng: &mut StdRng, out: &mut BTreeMap<&'static str, f64>) {
    let draw = |rng: &mut StdRng| -> S {
        let v = rng.gen::<u64>() as u128 | ((rng.gen::<u64>() as u128) << 64);
        let v = v % r.to_u128().max(2);
        S::from_biguint(&num_bigint::BigUint::from(v.max(1))).expect("below r")
    };
    let xs: Vec<S> = (0..1024).map(|_| draw(rng)).collect();
    let iters = 4_000_000u64;
    let mut acc = xs[0];
    out.insert(
        "scalar_mod_op",
        ns_per(iters, |i| {
            let x = xs[(i % 1024) as usize];
            acc = if i & 1 == 0 {
                S::add_mod(acc, x, r)
            } else {
                S::sub_mod(acc, x, r)
            };
            acc.to_u128() as u64
        }),
    );
    let mut acc = xs[1];
    out.insert(
        "scalar_mulmod",
        ns_per(1_000_000, |i| {
            acc = S::mul_mod(acc, xs[(i % 1024) as usize], r);
            acc.to_u128() as u64
        }),
    );
    out.insert(
        "scalar_invmod",
        ns_per(20_000, |i| {
            S::inverse_mod(xs[(i % 1024) as usize], r).map_or(0, |v| v.to_u128() as u64)
        }),
    );
    // A relation-matrix multiply-subtract: row[j] -= f * pivot[j] (mod r).
    let cols = 2048usize;
    let pivot: Vec<S> = (0..cols).map(|_| draw(rng)).collect();
    let mut row: Vec<S> = (0..cols).map(|_| draw(rng)).collect();
    let passes = 300u64;
    let start = Instant::now();
    for k in 0..passes {
        let f = xs[(k % 1024) as usize];
        for j in 0..cols {
            row[j] = S::sub_mod(row[j], S::mul_mod(f, pivot[j], r), r);
        }
    }
    black_box(&row);
    out.insert(
        "row_op",
        start.elapsed().as_nanos() as f64 / (passes * cols as u64) as f64,
    );
    // One relation's row: four summands into an ordered map, the column
    // hash, the right-hand side, the sparse vector.
    let cols4: Vec<usize> = (0..1024).map(|_| rng.gen_range(0..4096)).collect();
    out.insert(
        "relation_row",
        ns_per(200_000, |i| {
            let mut row: std::collections::BTreeMap<usize, S> = std::collections::BTreeMap::new();
            for k in 0..4usize {
                let c = cols4[((i as usize) * 4 + k) % 1024];
                let e = row.entry(c).or_insert(S::zero());
                *e = S::add_mod(*e, xs[(k + i as usize) % 1024], r);
            }
            row.retain(|_, v| *v != S::zero());
            let mut h = 0x9E37_79B9_7F4A_7C15u64;
            for (&c, &v) in &row {
                let v = v.to_u128();
                let v64 = (v as u64) ^ ((v >> 64) as u64).rotate_left(29);
                h = mix(h ^ mix(((c as u64) << 32) ^ v64));
            }
            let dv = S::sub_mod(S::zero(), S::mul_mod(xs[0], xs[1], r), r);
            let rhs = S::mul_mod(xs[2], xs[3], r);
            let mut sparse: Vec<(usize, S)> = row.into_iter().collect();
            sparse.push((4096, dv));
            h ^ rhs.to_u128() as u64 ^ sparse.len() as u64
        }),
    );
}

/// Measure every unit on `inst`'s field once and return unit → ns.
fn measure_once(inst: &BinCurve, seed: u64) -> BTreeMap<&'static str, f64> {
    let c = &inst.curve;
    let f = &c.f;
    let g = &f.f.tab;
    let q = f.f.q;
    let m = f.f.m;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out: BTreeMap<&'static str, f64> = BTreeMap::new();

    // The reference: a dense axpy element (log-table multiply and XOR).
    let n = 4096usize;
    let piv: Vec<Fe> = (0..n).map(|_| rng.gen_range(1..q)).collect();
    let mut row: Vec<Fe> = (0..n).map(|_| rng.gen_range(0..q)).collect();
    let passes = 600u64;
    let start = Instant::now();
    for k in 0..passes {
        g.axpy(
            &mut row,
            &piv,
            1 + ((k as u32).wrapping_mul(7919) % (q - 1)),
        );
    }
    black_box(&row);
    out.insert(
        "fq_mul",
        start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64,
    );
    let xs: Vec<Fe> = (0..1024).map(|_| rng.gen_range(1..q)).collect();
    out.insert(
        "fq_inv_tab",
        ns_per(4_000_000, |i| u64::from(g.inv(xs[(i % 1024) as usize]))),
    );
    // Passes that multiply nothing, which the solver's elimination makes
    // over its rows: count a row's nonzero entries, scan it for its
    // leading entry, and step over its zero entries in a dense axpy (a
    // quarter of the pivot's entries nonzero).  The unit is their mean.
    let mut zero_tail = vec![0u32; n];
    zero_tail[n - 1] = 1;
    let sparse_piv: Vec<Fe> = (0..n)
        .map(|_| {
            if rng.gen_range(0..4) == 0 {
                rng.gen_range(1..q)
            } else {
                0
            }
        })
        .collect();
    let start = Instant::now();
    let mut s1 = 0usize;
    for _ in 0..passes {
        s1 += black_box(&row).iter().filter(|&&x| x != 0).count();
    }
    black_box(s1);
    let count_ns = start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64;
    let start = Instant::now();
    let mut s2 = 0usize;
    for _ in 0..passes {
        s2 += black_box(&zero_tail)
            .iter()
            .position(|&x| x != 0)
            .unwrap_or(0);
    }
    black_box(s2);
    let scan_ns = start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64;
    let start = Instant::now();
    for k in 0..passes {
        g.axpy(
            &mut row,
            &sparse_piv,
            1 + ((k as u32).wrapping_mul(104_729) % (q - 1)),
        );
    }
    black_box(&row);
    let sparse_ns = start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64;
    out.insert("elem_scan", (count_ns + scan_ns) / 2.0);
    // A dense axpy over a pivot with a quarter of its entries nonzero costs
    // its nonzero entries a multiplication each and its zero entries a
    // skip.
    let frac = sparse_piv.iter().filter(|&&x| x != 0).count() as f64 / n as f64;
    out.insert(
        "axpy_skip",
        ((sparse_ns - frac * out["fq_mul"]) / (1.0 - frac)).max(0.0),
    );
    // One fork and join of a small parallel region (four rows, a trivial
    // body) on a one-thread pool: the dispatch work itself (the job, the
    // latch, the split), which every run pays.  What a multi-thread pool
    // adds is waiting for sleeping workers to be scheduled, a property of
    // the host's load and placement that a count cannot carry; evidence
    // runs pin the solve to one core (`ecbench run --cpus auto`).
    {
        use rayon::prelude::*;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("a one-thread pool");
        let mut small: Vec<(u32, Vec<Fe>)> = (0..4).map(|k| (k, vec![1u32; 8])).collect();
        out.insert(
            "par_region",
            pool.install(|| {
                ns_per(400_000, |_| {
                    small.par_iter_mut().for_each(|(k, v)| {
                        v[(*k as usize) % 8] ^= 1;
                    });
                    u64::from(small[0].1[0])
                })
            }),
        );
    }
    // Zero-filling a new row.
    let start = Instant::now();
    for k in 0..passes {
        let mut v = vec![0u32; black_box(n)];
        v[(k as usize) % n] = 1;
        black_box(&v);
    }
    out.insert(
        "elem_fill",
        start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64,
    );
    let keys4: Vec<[u8; 4]> = (0..600)
        .map(|_| core::array::from_fn(|_| rng.gen_range(0..12)))
        .collect();
    let mono: HashMap<[u8; 4], usize> = keys4.iter().enumerate().map(|(i, k)| (*k, i)).collect();
    out.insert(
        "monomial_hash_op",
        ns_per(2_000_000, |i| {
            mono.get(&keys4[(i % 600) as usize]).copied().unwrap_or(0) as u64
        }),
    );
    out.insert(
        "rng_seed",
        ns_per(200_000, |i| {
            StdRng::seed_from_u64(i ^ 0x9e37_79b9).gen::<u64>()
        }),
    );
    let mut r2 = StdRng::seed_from_u64(seed ^ 1);
    out.insert("rng_draw", ns_per(4_000_000, |_| r2.gen::<u64>()));
    out.insert("mix_eval", ns_per(8_000_000, mix));

    // Points.
    let mut pts: Vec<PtB> = Vec::with_capacity(256);
    while pts.len() < 256 {
        let x = f.random(&mut rng);
        if let Some(p) = c.lift_x(&x, &mut rng) {
            pts.push(p);
        }
    }
    out.insert(
        "key_eval",
        ns_per(2_000_000, |i| {
            CountedGroup::key(c, &pts[(i % 256) as usize])
        }),
    );
    out.insert(
        "neg_eval",
        ns_per(2_000_000, |i| {
            u64::from(c.neg(&pts[(i % 256) as usize]).y.0[0])
        }),
    );
    out.insert(
        "pack_eval",
        ns_per(4_000_000, |i| f.pack(&pts[(i % 256) as usize].x) as u64),
    );
    out.insert(
        "canon_batched",
        ns_per(2_000_000, |i| {
            let p = pts[(i % 256) as usize];
            let ny = p.x.add(&p.y);
            let q = if f.pack(&ny) < f.pack(&p.y) {
                PtB {
                    x: p.x,
                    y: ny,
                    inf: false,
                }
            } else {
                p
            };
            u64::from(q.y.0[0])
        }),
    );
    out.insert(
        "hash_x",
        ns_per(2_000_000, |i| {
            mix(wide_key(f.pack(&pts[(i % 256) as usize].x)))
        }),
    );

    // The recent-point window: 16 entries of (key, point, a, b).
    let mut recent: VecDeque<(u128, PtB, u128, u128)> = VecDeque::with_capacity(17);
    for k in 0..16u128 {
        recent.push_back((k + 1000, pts[k as usize], k, k));
    }
    let scan = ns_per(2_000_000, |i| {
        let key = f.pack(&pts[(i % 256) as usize].x) | (1u128 << 100);
        recent.iter().position(|e| e.0 == key).unwrap_or(16) as u64
    });
    // Subtract the pack that the scan loop includes, then per element.
    out.insert(
        "window_scan_elem",
        ((scan - out["pack_eval"]).max(0.0)) / 16.0,
    );
    out.insert(
        "window_push",
        ns_per(4_000_000, |i| {
            recent.push_back((i as u128, pts[(i % 256) as usize], 1, 2));
            if recent.len() > 16 {
                recent.pop_front();
            }
            recent.len() as u64
        }),
    );
    out.insert(
        "walk_start",
        ns_per(1_000_000, |i| {
            let d: VecDeque<(u128, PtB, u128, u128)> = VecDeque::with_capacity(black_box(17));
            black_box(&d);
            i
        }),
    );

    // Hash tables of the sizes the methods use.
    // At least 2^14 entries, so that the allocation of a small table is
    // not charged to its few inserts.
    let size = (1usize << (m + 1)).clamp(1 << 14, 1 << 22);
    let mut fm: FastMap<u32> = fast_map(size);
    let mut insert_rng = StdRng::seed_from_u64(seed ^ 2);
    let start = Instant::now();
    for i in 0..size {
        fm.insert(insert_rng.gen::<u64>(), i as u32);
    }
    out.insert(
        "fast_insert",
        start.elapsed().as_nanos() as f64 / size as f64,
    );
    let hit_keys: Vec<u64> = fm.keys().copied().step_by(97).take(1 << 13).collect();
    let probes: Vec<u64> = (0..(1usize << 14))
        .map(|i| {
            if i % 2 == 0 {
                hit_keys[(i / 2) % hit_keys.len()]
            } else {
                insert_rng.gen::<u64>()
            }
        })
        .collect();
    out.insert(
        "fast_probe",
        ns_per(2_000_000, |i| {
            u64::from(fm.contains_key(&probes[(i as usize) % probes.len()]))
        }),
    );
    // The guard set of the relation loop, grown to a million entries.
    let mut guard: HashMap<u64, ()> = HashMap::with_capacity(1 << 21);
    for _ in 0..(1u32 << 20) {
        guard.insert(insert_rng.gen::<u64>(), ());
    }
    out.insert(
        "guard_insert",
        ns_per(1 << 18, |_| {
            u64::from(guard.insert(insert_rng.gen::<u64>(), ()).is_some())
        }),
    );
    // The distinguished-point table, growing from 4096.
    let mut dp: HashMap<u128, (u128, u128)> = HashMap::with_capacity(4096);
    let start = Instant::now();
    let mut dp_keys = Vec::with_capacity(size);
    for i in 0..size {
        let k = (insert_rng.gen::<u64>() as u128) << 40 | i as u128;
        dp_keys.push(k);
        dp.insert(k, (i as u128, 1));
    }
    out.insert("dp_insert", start.elapsed().as_nanos() as f64 / size as f64);
    out.insert(
        "dp_probe",
        ns_per(2_000_000, |i| {
            let k = if i % 2 == 0 {
                dp_keys[(i as usize / 2) % dp_keys.len()]
            } else {
                (insert_rng.gen::<u64>() as u128) << 40
            };
            u64::from(dp.contains_key(&k))
        }),
    );
    let bt_keys: Vec<usize> = (0..1024).map(|_| insert_rng.gen_range(0..4096)).collect();
    out.insert(
        "btree_op",
        ns_per(500_000, |i| {
            let mut b: std::collections::BTreeMap<usize, u128> = Default::default();
            for k in 0..4 {
                *b.entry(bt_keys[(i as usize + k) % 1024]).or_insert(0) += 1;
            }
            b.len() as u64
        }) / 4.0,
    );
    // The shuffle of the sixteen-entry jump table.
    let mut perm: Vec<usize> = (0..16).collect();
    out.insert(
        "jump_shuffle",
        ns_per(500_000, |_| {
            for i in (1..16).rev() {
                perm.swap(i, insert_rng.gen_range(0..i + 1));
            }
            perm[0] as u64
        }),
    );
    // A round's bookkeeping for one walk: the job, the denominator, the
    // take and put of the walk's state.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct Job {
        slot: usize,
        jump: usize,
        denom: Option<crate::cryptanalysis::gaudry_quartic_bin::Eb>,
        sum: Option<PtB>,
    }
    #[derive(Clone)]
    #[allow(dead_code)]
    struct Walk {
        x: PtB,
        a: u128,
        b: u128,
        tried: usize,
        j0: usize,
        len: u64,
        last: Option<u128>,
        recent: VecDeque<(u128, PtB, u128, u128)>,
    }
    let mut pool: Vec<Option<Walk>> = (0..256)
        .map(|k| {
            Some(Walk {
                x: pts[k],
                a: 1,
                b: 2,
                tried: 0,
                j0: k % 16,
                len: 0,
                last: None,
                recent: VecDeque::new(),
            })
        })
        .collect();
    let jump_pt = pts[7];
    let mut jobs: Vec<Job> = Vec::with_capacity(256);
    let rounds = 4_000u64;
    let start = Instant::now();
    let mut acc = 0u64;
    for _ in 0..rounds {
        jobs.clear();
        for (slot, w) in pool.iter().enumerate() {
            let Some(w) = w else { continue };
            let j = (w.j0 + w.tried) % 16;
            let d = w.x.x.add(&jump_pt.x);
            jobs.push(Job {
                slot,
                jump: j,
                denom: if d.is_zero() { None } else { Some(d) },
                sum: None,
            });
        }
        for job in jobs.drain(..) {
            let mut w = pool[job.slot].take().unwrap();
            w.len += 1;
            w.tried = 0;
            acc = acc.wrapping_add(job.jump as u64 + w.len);
            pool[job.slot] = Some(w);
        }
    }
    black_box(acc);
    out.insert(
        "round_job",
        start.elapsed().as_nanos() as f64 / (rounds * 256) as f64,
    );

    // Arithmetic modulo r, at the width of r; the large-prime code always
    // works in u128.
    if let Ok(r64) = u64::try_from(inst.r) {
        scalar_units::<u64>(r64, &mut rng, &mut out);
    } else {
        scalar_units::<u128>(inst.r, &mut rng, &mut out);
    }
    {
        let r = inst.r;
        let xs: Vec<u128> = (0..1024).map(|_| rng.gen::<u128>() % r.max(2)).collect();
        let mut acc = xs[0].max(1);
        out.insert(
            "lp_mulmod",
            ns_per(1_000_000, |i| {
                acc = <u128 as RhoScalar>::mul_mod(acc, xs[(i % 1024) as usize].max(1), r);
                acc as u64
            }),
        );
        let mut acc = xs[1];
        out.insert(
            "lp_addsub",
            ns_per(4_000_000, |i| {
                let x = xs[(i % 1024) as usize];
                acc = if acc >= x { acc - x } else { r - (x - acc) };
                acc as u64
            }),
        );
        // The merge loop of two sparse vectors, per entry, with the
        // arithmetic replaced by a wrapping add (it is priced apart).
        let xv: Vec<(usize, u128)> = (0..64).map(|k| (2 * k, xs[k] | 1)).collect();
        let yv: Vec<(usize, u128)> = (0..64).map(|k| (3 * k, xs[k + 64] | 1)).collect();
        let per_merge = ns_per(100_000, |_| {
            let mut o: Vec<(usize, u128)> = Vec::with_capacity(128);
            let (mut i, mut j) = (0, 0);
            while i < xv.len() || j < yv.len() {
                let take_x = j >= yv.len() || (i < xv.len() && xv[i].0 < yv[j].0);
                let take_y = i >= xv.len() || (j < yv.len() && yv[j].0 < xv[i].0);
                if take_x {
                    o.push(xv[i]);
                    i += 1;
                } else if take_y {
                    o.push((yv[j].0, yv[j].1.wrapping_add(1)));
                    j += 1;
                } else {
                    o.push((xv[i].0, xv[i].1.wrapping_add(yv[j].1)));
                    i += 1;
                    j += 1;
                }
            }
            o.len() as u64
        });
        out.insert("lp_entry", per_merge / 128.0);
    }
    out
}

/// Measure every unit for `inst` `reps` times on this host and return the
/// median ratio to one `F_q` multiplication per unit, with the per-rep
/// ratios and the multiplication's own time.
pub fn measure(inst: &BinCurve, reps: usize) -> Result<Value, String> {
    let mut ratios: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut mul_ns = Vec::new();
    for k in 0..reps.max(1) {
        let t = measure_once(inst, 0xF2 + k as u64);
        let mul = t["fq_mul"];
        if mul <= 0.0 {
            return Err("an F_q multiplication measured at zero nanoseconds".into());
        }
        mul_ns.push(mul);
        for (u, ns) in t {
            if u != "fq_mul" {
                ratios.entry(u).or_default().push(ns / mul);
            }
        }
    }
    let mut row = serde_json::Map::new();
    let mut per_rep = serde_json::Map::new();
    for u in UNITS {
        let v = ratios
            .get(u)
            .ok_or_else(|| format!("unit {u} was not measured"))?;
        if v.iter().any(|x| !x.is_finite() || *x < 0.0) {
            return Err(format!("unit {u} did not measure"));
        }
        row.insert((*u).into(), json!((median(v.clone()) * 1e6).round() / 1e6));
        per_rep.insert((*u).into(), json!(v));
    }
    Ok(json!({
        "row": row,
        "scalar_width_bits": if u64::try_from(inst.r).is_ok() { 64 } else { 128 },
        "reps": reps,
        "ns_per_fq_mul_median": median(mul_ns.clone()),
        "ns_per_fq_mul": mul_ns,
        "ratios_per_rep": per_rep,
    }))
}

// ── Montgomery multiplication modulo r (the Montgomery kernel of the one-pass table) ───

/// `a·b` as a 256-bit product `(hi, lo)`.
fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    let (a0, a1) = (a as u64 as u128, a >> 64);
    let (b0, b1) = (b as u64 as u128, b >> 64);
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    let mid = (p00 >> 64) + (p01 as u64 as u128) + (p10 as u64 as u128);
    let lo = (p00 as u64 as u128) | (mid << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (hi, lo)
}

/// `m⁻¹ mod 2^128` for odd `m` (Newton iteration).
fn inv_pow2_128(m: u128) -> u128 {
    let mut x = m;
    for _ in 0..7 {
        x = x.wrapping_mul(2u128.wrapping_sub(m.wrapping_mul(x)));
    }
    x
}

/// Montgomery multiplication modulo an odd `r`, in one limb (`r < 2^64`) or
/// two (`r` wider), with `R = 2^64` or `2^128`.  `mul` of a plain and a
/// Montgomery-form operand is the plain product, so a multiplier fixed
/// across a vector operation is converted once (`to_mont`) and each
/// element then costs one reduction.
#[derive(Clone, Copy)]
pub struct Mont {
    m: u128,
    minv: u128,
    r2: u128,
    wide: bool,
}

impl Mont {
    /// `None` for an even or trivial modulus.
    pub fn new(r: u128) -> Option<Mont> {
        if r < 3 || r & 1 == 0 {
            return None;
        }
        let wide = r >> 64 != 0;
        let minv = inv_pow2_128(r);
        let rmod = if wide {
            (u128::MAX % r + 1) % r
        } else {
            (1u128 << 64) % r
        };
        let r2 = <u128 as RhoScalar>::mul_mod(rmod, rmod, r);
        Some(Mont {
            m: r,
            minv,
            r2,
            wide,
        })
    }

    /// The reduction.  The final correction is a select, not a branch: the
    /// operand is random, a branch there mispredicts half the time and
    /// costs more than the multiplications.
    #[inline(always)]
    fn redc(&self, hi: u128, lo: u128) -> u128 {
        use std::hint::select_unpredictable as sel;
        if self.wide {
            let u = lo.wrapping_mul(self.minv);
            let (uh, _) = mul_wide(u, self.m);
            let (d, borrow) = hi.overflowing_sub(uh);
            d.wrapping_add(sel(borrow, self.m, 0))
        } else {
            // One limb: the product fits a u128 (`hi` is zero).
            let m = self.m as u64;
            let u = (lo as u64).wrapping_mul(self.minv as u64);
            let uh = ((u as u128 * m as u128) >> 64) as u64;
            let (d, borrow) = ((lo >> 64) as u64).overflowing_sub(uh);
            d.wrapping_add(sel(borrow, m, 0)) as u128
        }
    }

    /// `a·b·R⁻¹ mod r`.
    #[inline(always)]
    pub fn mul(&self, a: u128, b: u128) -> u128 {
        if self.wide {
            let (hi, lo) = mul_wide(a, b);
            self.redc(hi, lo)
        } else {
            self.redc(0, (a as u64 as u128) * (b as u64 as u128))
        }
    }

    /// `a·R mod r`.
    pub fn to_mont(&self, a: u128) -> u128 {
        self.mul(a, self.r2)
    }

    /// `a·b mod r` for plain `a, b < r`: two reductions.
    #[inline(always)]
    pub fn mulmod(&self, a: u128, b: u128) -> u128 {
        self.mul(self.mul(a, b), self.r2)
    }
}

fn push<'a>(m: &mut BTreeMap<&'a str, Vec<f64>>, k: &'a str, v: f64) {
    m.entry(k).or_default().push(v);
}

/// The repository's `scalar_mulmod` and `row_op` loops at the width of `r`
/// (the same code as [`scalar_units`], these two units only), in ns.
fn code_scalar_units(
    r: u128,
    narrow: bool,
    xs: &[u128],
    rng: &mut StdRng,
    out: &mut BTreeMap<&'static str, f64>,
) {
    fn go<S: RhoScalar>(
        r: S,
        xs: &[u128],
        rng: &mut StdRng,
        out: &mut BTreeMap<&'static str, f64>,
    ) {
        let conv = |v: u128| S::from_biguint(&num_bigint::BigUint::from(v)).expect("below r");
        let xs: Vec<S> = xs.iter().map(|&v| conv(v)).collect();
        let mut acc = xs[1];
        out.insert(
            "scalar_mulmod",
            ns_per(1_000_000, |i| {
                acc = S::mul_mod(acc, xs[(i % 1024) as usize], r);
                acc.to_u128() as u64
            }),
        );
        let cols = 2048usize;
        let rr = r.to_u128();
        let mut draw = || conv((rng.gen::<u128>() % rr).max(1));
        let pivot: Vec<S> = (0..cols).map(|_| draw()).collect();
        let mut row: Vec<S> = (0..cols).map(|_| draw()).collect();
        let passes = 300u64;
        let start = Instant::now();
        for k in 0..passes {
            let f = xs[(k % 1024) as usize];
            for j in 0..cols {
                row[j] = S::sub_mod(row[j], S::mul_mod(f, pivot[j], r), r);
            }
        }
        black_box(&row);
        out.insert(
            "row_op",
            start.elapsed().as_nanos() as f64 / (passes * cols as u64) as f64,
        );
    }
    if narrow {
        go::<u64>(r as u64, xs, rng, out);
    } else {
        go::<u128>(r, xs, rng, out);
    }
}

/// Measure `scalar_mulmod`, `row_op` and `lp_mulmod` on the repository's
/// code path and on [`Mont`], in the same pass as one counted `F_q`
/// multiplication, and return the median ratios per unit.  Montgomery:
/// two reductions for an isolated product (`scalar_mulmod`, `lp_mulmod`),
/// one for a multiplier fixed over a vector (`row_op`, whose multiplier is
/// converted once per row and the conversion charged).
pub fn measure_mulmod(inst: &BinCurve, reps: usize) -> Result<Value, String> {
    let r = inst.r;
    let mont = Mont::new(r).ok_or("the subgroup order is not odd")?;
    let narrow = u64::try_from(r).is_ok();
    let mut cur: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut fast: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut extra: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut mul_ns = Vec::new();
    for k in 0..reps.max(1) {
        let mut rng = StdRng::seed_from_u64(0xF2 + k as u64);
        let g = &inst.curve.f.f.tab;
        let q = inst.curve.f.f.q;
        let n = 4096usize;
        let piv: Vec<Fe> = (0..n).map(|_| rng.gen_range(1..q)).collect();
        let mut frow: Vec<Fe> = (0..n).map(|_| rng.gen_range(0..q)).collect();
        let passes = 600u64;
        let start = Instant::now();
        for k in 0..passes {
            g.axpy(
                &mut frow,
                &piv,
                1 + ((k as u32).wrapping_mul(7919) % (q - 1)),
            );
        }
        black_box(&frow);
        let fq = start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64;
        if fq <= 0.0 {
            return Err("an F_q multiplication measured at zero nanoseconds".into());
        }
        mul_ns.push(fq);
        let xs: Vec<u128> = (0..1024).map(|_| (rng.gen::<u128>() % r).max(1)).collect();
        let xm: Vec<u128> = xs.iter().map(|&x| mont.to_mont(x)).collect();
        // The repository's path: the scalar units at the width of r, the
        // large-prime code always u128.
        let mut cu: BTreeMap<&'static str, f64> = BTreeMap::new();
        code_scalar_units(r, narrow, &xs, &mut rng, &mut cu);
        let mut acc = xs[0];
        cu.insert(
            "lp_mulmod",
            ns_per(1_000_000, |i| {
                acc = <u128 as RhoScalar>::mul_mod(acc, xs[(i % 1024) as usize], r);
                acc as u64
            }),
        );
        for (u, v) in &cu {
            push(&mut cur, u, v / fq);
        }
        let mut acc = xs[1];
        let sm = ns_per(1_000_000, |i| {
            acc = mont.mulmod(acc, xs[(i % 1024) as usize]);
            acc as u64
        });
        push(&mut fast, "scalar_mulmod", sm / fq);
        let mut acc = xs[2];
        let lp = ns_per(1_000_000, |i| {
            acc = mont.mulmod(acc, xs[(i % 1024) as usize]);
            acc as u64
        });
        push(&mut fast, "lp_mulmod", lp / fq);
        let mut acc = xs[3];
        let one = ns_per(1_000_000, |i| {
            acc = mont.mul(acc, xm[(i % 1024) as usize]);
            acc as u64
        });
        push(&mut extra, "lp_mulmod_one_reduction", one / fq);
        let cols = 2048usize;
        let pivot: Vec<u128> = (0..cols).map(|_| (rng.gen::<u128>() % r).max(1)).collect();
        let mut rowv: Vec<u128> = (0..cols).map(|_| (rng.gen::<u128>() % r).max(1)).collect();
        let passes = 300u64;
        let mut row_time = |fixed_mont: bool| -> f64 {
            let start = Instant::now();
            for k in 0..passes {
                let f = xs[(k % 1024) as usize];
                let fm = mont.to_mont(f);
                for j in 0..cols {
                    let p = if fixed_mont {
                        mont.mul(pivot[j], fm)
                    } else {
                        mont.mulmod(f, pivot[j])
                    };
                    let (d, borrow) = rowv[j].overflowing_sub(p);
                    rowv[j] = d.wrapping_add(std::hint::select_unpredictable(borrow, r, 0));
                }
            }
            black_box(&rowv);
            start.elapsed().as_nanos() as f64 / (passes * cols as u64) as f64
        };
        push(&mut fast, "row_op", row_time(true) / fq);
        push(&mut extra, "row_op_two_reductions", row_time(false) / fq);
    }
    let med = |m: &BTreeMap<&str, Vec<f64>>| -> Value {
        let mut o = serde_json::Map::new();
        for (u, v) in m {
            o.insert((*u).into(), json!((median(v.clone()) * 1e6).round() / 1e6));
        }
        Value::Object(o)
    };
    Ok(json!({
        "row": med(&fast),
        "code_row": med(&cur),
        "extra": med(&extra),
        "scalar_width_bits": if narrow { 64 } else { 128 },
        "reps": reps,
        "ns_per_fq_mul_median": median(mul_ns.clone()),
        "ns_per_fq_mul": mul_ns,
        "ratios_per_rep": {"row": fast, "code_row": cur},
    }))
}

// ── The one-pass table ──────────────────────────────────────────────────

/// Montgomery multiplication modulo an odd `r < 2^64` on `u64` storage and
/// one limb (`R = 2^64`): the kernel a competent implementation uses for a
/// narrow order, with no 128-bit subtraction or select.
#[derive(Clone, Copy)]
pub struct Mont64 {
    m: u64,
    minv: u64,
    r2: u64,
}

impl Mont64 {
    /// `None` for an even or trivial modulus.
    pub fn new(r: u64) -> Option<Mont64> {
        if r < 3 || r & 1 == 0 {
            return None;
        }
        let mut x = r;
        for _ in 0..6 {
            x = x.wrapping_mul(2u64.wrapping_sub(r.wrapping_mul(x)));
        }
        let rmod = ((1u128 << 64) % r as u128) as u64;
        let r2 = ((rmod as u128 * rmod as u128) % r as u128) as u64;
        Some(Mont64 { m: r, minv: x, r2 })
    }

    /// `t·R⁻¹ mod r` for `t < r·R`; the final correction is a select.
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let u = (t as u64).wrapping_mul(self.minv);
        let uh = ((u as u128 * self.m as u128) >> 64) as u64;
        let (d, borrow) = ((t >> 64) as u64).overflowing_sub(uh);
        d.wrapping_add(std::hint::select_unpredictable(borrow, self.m, 0))
    }

    /// `a·b·R⁻¹ mod r`.
    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.redc(a as u128 * b as u128)
    }

    /// `a·R mod r`.
    pub fn to_mont(&self, a: u64) -> u64 {
        self.mul(a, self.r2)
    }

    /// `a·b mod r` for plain `a, b < r`: two reductions.
    #[inline(always)]
    pub fn mulmod(&self, a: u64, b: u64) -> u64 {
        self.mul(self.mul(a, b), self.r2)
    }
}

/// The Montgomery kernel at either width, behind one interface.
trait MontKernel: Copy {
    type S: Copy;
    fn from_u128(v: u128) -> Self::S;
    fn to_mont(&self, a: Self::S) -> Self::S;
    fn mul(&self, a: Self::S, b: Self::S) -> Self::S;
    fn mulmod(&self, a: Self::S, b: Self::S) -> Self::S;
    /// `a − b mod r` for `a, b < r`.
    fn sub(&self, a: Self::S, b: Self::S) -> Self::S;
    fn low(a: Self::S) -> u64;
}

impl MontKernel for Mont64 {
    type S = u64;
    fn from_u128(v: u128) -> u64 {
        v as u64
    }
    fn to_mont(&self, a: u64) -> u64 {
        Mont64::to_mont(self, a)
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        Mont64::mul(self, a, b)
    }
    #[inline(always)]
    fn mulmod(&self, a: u64, b: u64) -> u64 {
        Mont64::mulmod(self, a, b)
    }
    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        let (d, borrow) = a.overflowing_sub(b);
        d.wrapping_add(std::hint::select_unpredictable(borrow, self.m, 0))
    }
    fn low(a: u64) -> u64 {
        a
    }
}

impl MontKernel for Mont {
    type S = u128;
    fn from_u128(v: u128) -> u128 {
        v
    }
    fn to_mont(&self, a: u128) -> u128 {
        Mont::to_mont(self, a)
    }
    #[inline(always)]
    fn mul(&self, a: u128, b: u128) -> u128 {
        Mont::mul(self, a, b)
    }
    #[inline(always)]
    fn mulmod(&self, a: u128, b: u128) -> u128 {
        Mont::mulmod(self, a, b)
    }
    #[inline(always)]
    fn sub(&self, a: u128, b: u128) -> u128 {
        let (d, borrow) = a.overflowing_sub(b);
        d.wrapping_add(std::hint::select_unpredictable(borrow, self.m, 0))
    }
    fn low(a: u128) -> u64 {
        a as u64
    }
}

/// The Montgomery kernel's `scalar_mulmod`, `lp_mulmod` and `row_op` on `k`
/// (storage at the kernel's width), in ns: an isolated product costs two
/// reductions; the row operation's multiplier is converted once per row (the
/// conversion charged) and each element then costs one.
fn mont_units<K: MontKernel>(
    k: K,
    r: u128,
    rng: &mut StdRng,
    out: &mut BTreeMap<&'static str, f64>,
) {
    let xs: Vec<K::S> = (0..1024)
        .map(|_| K::from_u128((rng.gen::<u128>() % r).max(1)))
        .collect();
    let mut acc = xs[1];
    out.insert(
        "scalar_mulmod",
        ns_per(1_000_000, |i| {
            acc = k.mulmod(acc, xs[(i % 1024) as usize]);
            K::low(acc)
        }),
    );
    let mut acc = xs[2];
    out.insert(
        "lp_mulmod",
        ns_per(1_000_000, |i| {
            acc = k.mulmod(acc, xs[(i % 1024) as usize]);
            K::low(acc)
        }),
    );
    let cols = 2048usize;
    let pivot: Vec<K::S> = (0..cols)
        .map(|_| K::from_u128((rng.gen::<u128>() % r).max(1)))
        .collect();
    let mut row: Vec<K::S> = (0..cols)
        .map(|_| K::from_u128((rng.gen::<u128>() % r).max(1)))
        .collect();
    let passes = 300u64;
    let start = Instant::now();
    for p in 0..passes {
        let fm = k.to_mont(xs[(p % 1024) as usize]);
        for j in 0..cols {
            row[j] = k.sub(row[j], k.mul(pivot[j], fm));
        }
    }
    black_box(&row);
    out.insert(
        "row_op",
        start.elapsed().as_nanos() as f64 / (passes * cols as u64) as f64,
    );
}

/// The ns of one counted `F_q` multiplication (a dense axpy element), as
/// [`measure_once`] measures it.
fn fq_mul_ns(inst: &BinCurve, rng: &mut StdRng) -> f64 {
    let g = &inst.curve.f.f.tab;
    let q = inst.curve.f.f.q;
    let n = 4096usize;
    let piv: Vec<Fe> = (0..n).map(|_| rng.gen_range(1..q)).collect();
    let mut row: Vec<Fe> = (0..n).map(|_| rng.gen_range(0..q)).collect();
    let passes = 600u64;
    let start = Instant::now();
    for k in 0..passes {
        g.axpy(
            &mut row,
            &piv,
            1 + ((k as u32).wrapping_mul(7919) % (q - 1)),
        );
    }
    black_box(&row);
    start.elapsed().as_nanos() as f64 / (passes * n as u64) as f64
}

/// The code's kernel for the three mod-`r` units (the repository's
/// `scalar_mulmod`/`row_op` loops at the width of `r`, `lp_mulmod` always
/// `u128`), in ns.
fn code_units(r: u128, narrow: bool, rng: &mut StdRng) -> BTreeMap<&'static str, f64> {
    let xs: Vec<u128> = (0..1024).map(|_| (rng.gen::<u128>() % r).max(1)).collect();
    let mut cu: BTreeMap<&'static str, f64> = BTreeMap::new();
    code_scalar_units(r, narrow, &xs, rng, &mut cu);
    let mut acc = xs[0];
    cu.insert(
        "lp_mulmod",
        ns_per(1_000_000, |i| {
            acc = <u128 as RhoScalar>::mul_mod(acc, xs[(i % 1024) as usize], r);
            acc as u64
        }),
    );
    cu
}

/// The Montgomery kernel's three units at the width of `r`, in ns.
fn mont_kernel_units(r: u128, narrow: bool, rng: &mut StdRng) -> BTreeMap<&'static str, f64> {
    let mut o = BTreeMap::new();
    if narrow {
        let k = Mont64::new(r as u64).expect("an odd order");
        mont_units(k, r, rng, &mut o);
    } else {
        let k = Mont::new(r).expect("an odd order");
        mont_units(k, r, rng, &mut o);
    }
    o
}

fn spread(v: &[f64]) -> Value {
    let med = median(v.to_vec());
    let (lo, hi) = v
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, b), &x| (a.min(x), b.max(x)));
    let mad = median(v.iter().map(|x| (x - med).abs()).collect());
    json!({
        "n": v.len(),
        "min": lo,
        "median": med,
        "max": hi,
        "relative_range": (hi - lo) / med,
        "relative_mad": mad / med,
    })
}

/// Measure the whole table for `inst` in one interleaved pass on this host:
/// every unit of [`measure`], and the three mod-`r` units on the code's
/// kernel and on a Montgomery kernel (u64 storage and one limb for a narrow
/// order), side by side.
///
/// Rep 0 is a warm-up and is discarded.  In every other rep the full unit
/// set is measured first (its mod-`r` units are the code's), then one block
/// of `F_q`-multiplication / code kernel / Montgomery kernel / code kernel /
/// `F_q`-multiplication, in the order C M C on even reps and M C M on odd
/// ones, so that neither kernel always runs first.  The block's ratios use
/// the mean of its two flanking `F_q`-multiplication times.  A unit's price
/// is the median over reps; for the three mod-`r` units the code's samples
/// are the full set's and the block's, and `row` takes the smaller of the
/// code's and the Montgomery median.  Spread per unit is recorded.
pub fn measure_onepass(inst: &BinCurve, reps: usize) -> Result<Value, String> {
    let r = inst.r;
    let narrow = u64::try_from(r).is_ok();
    Mont::new(r).ok_or("the subgroup order is not odd")?;
    let reps = reps.max(1);
    let mut all: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut code: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut mont: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    let mut mul_ns = Vec::new();
    for k in 0..=reps {
        let warm = k == 0;
        let seed = 0xF2 + k as u64;
        let t = measure_once(inst, seed);
        let mul = t["fq_mul"];
        if mul <= 0.0 {
            return Err("an F_q multiplication measured at zero nanoseconds".into());
        }
        let mut rng = StdRng::seed_from_u64(seed ^ 0x51);
        let before = fq_mul_ns(inst, &mut rng);
        let order: [bool; 3] = if k % 2 == 0 {
            [true, false, true]
        } else {
            [false, true, false]
        };
        let mut blocks: Vec<(bool, BTreeMap<&'static str, f64>)> = Vec::new();
        for is_code in order {
            let b = if is_code {
                code_units(r, narrow, &mut rng)
            } else {
                mont_kernel_units(r, narrow, &mut rng)
            };
            blocks.push((is_code, b));
        }
        let after = fq_mul_ns(inst, &mut rng);
        if warm {
            continue;
        }
        mul_ns.push(mul);
        let fq_adj = (before + after) / 2.0;
        for (u, ns) in &t {
            if *u != "fq_mul" {
                all.entry(u).or_default().push(ns / mul);
            }
            if MULMOD_UNITS.contains(u) {
                code.entry(u).or_default().push(ns / mul);
            }
        }
        for (is_code, b) in blocks {
            for (u, ns) in b {
                let dst = if is_code { &mut code } else { &mut mont };
                dst.entry(u).or_default().push(ns / fq_adj);
            }
        }
    }
    let mut row = serde_json::Map::new();
    let mut row_code = serde_json::Map::new();
    let mut row_mont = serde_json::Map::new();
    let mut chosen = serde_json::Map::new();
    let mut spreads = serde_json::Map::new();
    let mut mulmod = serde_json::Map::new();
    let rnd = |x: f64| json!((x * 1e6).round() / 1e6);
    for u in UNITS {
        let v = all
            .get(u)
            .ok_or_else(|| format!("unit {u} was not measured"))?;
        if v.iter().any(|x| !x.is_finite() || *x < 0.0) {
            return Err(format!("unit {u} did not measure"));
        }
        if MULMOD_UNITS.contains(u) {
            let (c, m) = (&code[u], &mont[u]);
            if c.iter().chain(m).any(|x| !x.is_finite() || *x <= 0.0) {
                return Err(format!("unit {u} did not measure"));
            }
            let (mc, mm) = (median(c.clone()), median(m.clone()));
            row_code.insert((*u).into(), rnd(mc));
            row_mont.insert((*u).into(), rnd(mm));
            row.insert((*u).into(), rnd(mc.min(mm)));
            chosen.insert(
                (*u).into(),
                json!(if mm < mc { "montgomery" } else { "code" }),
            );
            spreads.insert(format!("{u}_code"), spread(c));
            spreads.insert(format!("{u}_montgomery"), spread(m));
            mulmod.insert(
                (*u).into(),
                json!({"code_samples": c, "montgomery_samples": m}),
            );
        } else {
            let med = median(v.clone());
            for t in [&mut row, &mut row_code, &mut row_mont] {
                t.insert((*u).into(), rnd(med));
            }
            spreads.insert((*u).into(), spread(v));
        }
    }
    Ok(json!({
        "row": row,
        "row_code": row_code,
        "row_montgomery": row_mont,
        "mulmod_chosen": chosen,
        "spread": spreads,
        "scalar_width_bits": if narrow { 64 } else { 128 },
        "montgomery_kernel": if narrow { "u64 storage, one limb" } else { "u128 storage, two limbs" },
        "reps": reps,
        "warmup_reps": 1,
        "ns_per_fq_mul_median": median(mul_ns.clone()),
        "ns_per_fq_mul": mul_ns,
        "mulmod_samples": mulmod,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_parses_and_every_row_has_every_unit() {
        let doc: Value = serde_json::from_str(F2M4_PRICES).unwrap();
        let inst = doc["instances"].as_object().unwrap();
        assert!(!inst.is_empty());
        for (k, v) in inst {
            let wide = v["scalar_width_bits"] == 128;
            let m: u32 = k
                .trim_start_matches('m')
                .trim_end_matches("_u128")
                .parse()
                .unwrap();
            let r = row(m, wide).unwrap();
            assert_eq!(r.len(), UNITS.len());
            assert!(r.values().all(|v| *v > 0.0), "{k}: a unit priced at zero");
        }
    }

    #[test]
    fn the_ids_have_bases() {
        for id in [
            "ic.pipeline_f2m4_full",
            "rho.negation_batched_full",
            "rho.negation_f2m4_full",
        ] {
            assert!(base_of(id).is_some());
        }
    }

    #[test]
    fn the_rule_application_band_counts_a_bundle_by_its_parts() {
        let u = row(8, false).unwrap();
        // IC's one `walk_steps` is a key, a mix and two mod-r operations;
        // rho's unbatched walk counts those four separately.
        let ic = charge(ic_rule("walk_steps").unwrap(), "walk_steps", 10, &u).unwrap();
        let rho: Vec<Charge> = [
            ("key_evaluations", 10),
            ("mix_evaluations", 10),
            ("scalar_mod_op_dummy", 0),
            ("scalar_mod_r_operations", 20),
        ]
        .iter()
        .filter(|(n, _)| rho_rule(n, false).is_some())
        .filter_map(|(n, c)| charge(rho_rule(n, false).unwrap(), n, *c, &u))
        .collect();
        let a = band_rule_applications(1.0, 50.0, std::slice::from_ref(&ic));
        let b = band_rule_applications(1.0, 50.0, &rho);
        assert_eq!(a["native_rule_applications"], 40.0);
        assert_eq!(b["native_rule_applications"], 40.0);
        assert_eq!(a["native_gae_per_application_1"], 41.0);
        let w = a["native_gae_weighted"].as_f64().unwrap();
        assert!((w - b["native_gae_weighted"].as_f64().unwrap()).abs() < 1e-12);
        assert_eq!(a["by_counter"]["walk_steps"]["applications_per_count"], 4.0);
        // The old count-based band sees 10 against 40.
        assert_eq!(ic.count, 10);
    }

    #[test]
    fn the_band_is_linear_in_the_rate() {
        let b = band(100.0, 50);
        assert_eq!(b["native_gae_per_operation_0"], 100.0);
        assert_eq!(b["native_gae_per_operation_0.1"], 105.0);
        assert_eq!(b["native_gae_per_operation_1"], 150.0);
    }

    #[test]
    fn montgomery_agrees_with_the_code_path() {
        let mut rng = StdRng::seed_from_u64(7);
        for bits in [3u32, 20, 40, 63, 64, 65, 68, 80, 100, 127] {
            for _ in 0..20 {
                let r = (rng.gen::<u128>() >> (128 - bits)) | 1 | (1u128 << (bits - 1));
                let Some(mt) = Mont::new(r) else { continue };
                for _ in 0..200 {
                    let a = rng.gen::<u128>() % r;
                    let b = rng.gen::<u128>() % r;
                    let want = <u128 as RhoScalar>::mul_mod(a, b, r);
                    assert_eq!(mt.mulmod(a, b), want, "bits {bits} r {r} a {a} b {b}");
                    assert_eq!(mt.mul(a, mt.to_mont(b)), want);
                }
            }
        }
        assert!(Mont::new(10).is_none());
    }

    #[test]
    fn mul_wide_is_the_256_bit_product() {
        assert_eq!(mul_wide(u128::MAX, u128::MAX), (u128::MAX - 1, 1));
        assert_eq!(mul_wide(1u128 << 100, 1u128 << 100), (1u128 << 72, 0));
    }

    #[test]
    fn montgomery_one_limb_u64_agrees_with_the_code_path() {
        let mut rng = StdRng::seed_from_u64(11);
        for bits in [3u32, 20, 40, 50, 63, 64] {
            for _ in 0..20 {
                let r = ((rng.gen::<u64>() >> (64 - bits)) | 1 | (1u64 << (bits - 1))) as u128;
                let Some(mt) = Mont64::new(r as u64) else {
                    continue;
                };
                for _ in 0..200 {
                    let a = rng.gen::<u64>() % r as u64;
                    let b = rng.gen::<u64>() % r as u64;
                    let want = <u128 as RhoScalar>::mul_mod(a as u128, b as u128, r) as u64;
                    assert_eq!(mt.mulmod(a, b), want, "bits {bits} r {r} a {a} b {b}");
                    assert_eq!(mt.mul(a, mt.to_mont(b)), want);
                    let s = MontKernel::sub(&mt, a, b);
                    assert_eq!(s as u128, (a as u128 + r - b as u128) % r);
                }
            }
        }
        assert!(Mont64::new(10).is_none());
    }

    /// The one-pass table has every unit in every row of the older table,
    /// from one host and one binary; the three variants of the mod-r kernel
    /// differ in the three mod-r units only, and the default is the smaller
    /// of the two kernels in each.
    #[test]
    fn the_onepass_table_covers_every_row_and_moves_three_units() {
        let doc: Value = serde_json::from_str(F2M4_PRICES).unwrap();
        let new: Value = serde_json::from_str(F2M4_ONEPASS_PRICES).unwrap();
        assert!(new["host"]["env_class_id"]
            .as_str()
            .unwrap()
            .starts_with("ECBENV2h"));
        assert_eq!(new["binary_sha256"].as_str().unwrap().len(), 64);
        for (k, v) in doc["instances"].as_object().unwrap() {
            let wide = v["scalar_width_bits"] == 128;
            let m: u32 = k
                .trim_start_matches('m')
                .trim_end_matches("_u128")
                .parse()
                .unwrap();
            let a = row_mm(m, wide, Mulmod::Onepass).unwrap();
            let c = row_mm(m, wide, Mulmod::OnepassCode).unwrap();
            let t = row_mm(m, wide, Mulmod::OnepassMont).unwrap();
            for u in UNITS {
                assert!(a[*u] > 0.0, "{k}: {u} priced at zero");
                if MULMOD_UNITS.contains(u) {
                    assert_eq!(a[*u], c[*u].min(t[*u]), "{k}: {u}");
                } else {
                    assert_eq!(a[*u], c[*u], "{k}: {u} must not move");
                    assert_eq!(a[*u], t[*u], "{k}: {u} must not move");
                }
            }
        }
    }

    fn row_op_ns(r: u128, narrow: bool, seed: u64, production: bool) -> f64 {
        let mut rng = StdRng::seed_from_u64(seed);
        if production {
            // `measure_once`'s own kernel.
            let mut out = BTreeMap::new();
            if narrow {
                scalar_units::<u64>(r as u64, &mut rng, &mut out);
            } else {
                scalar_units::<u128>(r, &mut rng, &mut out);
            }
            out["row_op"]
        } else {
            code_units(r, narrow, &mut rng)["row_op"]
        }
    }

    /// The row-op kernel of the one-pass table's code arm reproduces the
    /// production kernel (`measure_once`'s `row_op`, the committed unit's
    /// definition) on the same host: interleaved, the medians agree within a
    /// factor of 1.5, at one word and at two.
    #[test]
    fn the_code_row_kernel_reproduces_the_production_row_op() {
        for (r, narrow) in [
            (0xFFFF_FFFF_FFFF_FFC5u128, true),
            ((1u128 << 40) + 15, true),
            ((1u128 << 68) + 1_000_003, false),
        ] {
            let (mut a, mut b) = (Vec::new(), Vec::new());
            for k in 0..5u64 {
                // Interleave and alternate which runs first.
                if k % 2 == 0 {
                    a.push(row_op_ns(r, narrow, k, true));
                    b.push(row_op_ns(r, narrow, k, false));
                } else {
                    b.push(row_op_ns(r, narrow, k, false));
                    a.push(row_op_ns(r, narrow, k, true));
                }
            }
            let (pa, pb) = (median(a), median(b));
            let ratio = pb / pa;
            assert!(
                (1.0 / 1.5..=1.5).contains(&ratio),
                "r = {r}: code kernel {pb:.2} ns against production {pa:.2} ns (ratio {ratio:.2})"
            );
        }
    }
}

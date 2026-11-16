//! Producer qualifier: independent modular replay/rank, no DLP/rho admission.
#[allow(dead_code, clippy::enum_variant_names)]
#[path = "../../../../../src/cryptanalysis/bitops.rs"]
mod bitops;
pub(crate) use bitops::{Field, Prim, Widths};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
};
pub(crate) type Terms = Vec<(u64, u64)>;
type Row = (Terms, Terms);
pub(crate) const KINDS: [&str; 12] = [
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
pub(crate) const PHASES: [&str; 9] = [
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
pub(crate) fn n(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or_else(|| format!("invalid {k}"))
}
pub(crate) fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub(crate) fn add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
pub(crate) fn pow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 != 0 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        e >>= 1;
    }
    r
}
// Reuse residual_walk::is_prime_u64's twelve-base deterministic u64 test;
// arithmetic here is independent of the collector and its counters.
pub(crate) fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(p) {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'witness: for a in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}
pub(crate) fn canonical(v: &Value, limit: u64, p: u64) -> Result<Terms, String> {
    let t: Terms = serde_json::from_value(v.clone()).map_err(|_| "invalid sparse terms")?;
    if t.len() > 4096
        || t.iter().any(|&(i, c)| i >= limit || c == 0 || c >= p)
        || t.windows(2).any(|w| w[0].0 >= w[1].0)
    {
        return Err("noncanonical sparse row".into());
    }
    Ok(t)
}
pub(crate) fn accumulate(out: &mut BTreeMap<u64, u64>, row: &Terms, c: u64, p: u64) {
    for &(i, a) in row {
        let slot = out.entry(i).or_default();
        *slot = add(*slot, mul(c, a, p), p);
    }
}
fn rank(rows: &[Terms], columns: usize, p: u64) -> usize {
    let mut a = vec![vec![0u64; columns]; rows.len()];
    for (i, row) in rows.iter().enumerate() {
        for &(j, c) in row {
            a[i][j as usize] = c;
        }
    }
    let mut r = 0;
    for c in 0..columns {
        let Some(pivot) = (r..a.len()).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, pivot);
        let inv = pow(a[r][c], p - 2, p);
        for v in &mut a[r][c..] {
            *v = mul(*v, inv, p);
        }
        let pivot_row = a[r].clone();
        for row in a.iter_mut().skip(r + 1) {
            let factor = row[c];
            if factor == 0 {
                continue;
            }
            for (v, &pivot) in row[c..].iter_mut().zip(&pivot_row[c..]) {
                *v = add(*v, p - mul(factor, pivot, p), p);
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
pub(crate) fn counts(v: &Value) -> Result<BTreeMap<String, u64>, String> {
    if v.as_object().is_none_or(|m| m.len() != KINDS.len()) {
        return Err("counter schema".into());
    }
    let m: BTreeMap<_, _> = KINDS
        .into_iter()
        .map(|k| Ok((k.to_string(), n(v, k)?)))
        .collect::<Result<_, String>>()?;
    if m["rng_words"] > m["word_op"] {
        return Err("RNG exceeds words".into());
    }
    Ok(m)
}
pub(crate) fn price(m: &BTreeMap<String, u64>, w: &Widths) -> Result<u128, String> {
    m.iter()
        .filter(|(k, _)| k.as_str() != "rng_words")
        .try_fold(0u128, |s, (k, &v)| {
            s.checked_add(
                w.gates(Prim::from_name(k).ok_or("unknown primitive")?)
                    .map_err(|_| "unpriced primitive")?
                    .checked_mul(v as u128)
                    .ok_or("price overflow")?,
            )
            .ok_or("price overflow".into())
        })
}
pub(crate) fn bitop_mul64() -> u128 {
    bitops::int_mul(64)
}
pub(crate) fn bitop_add64() -> u128 {
    bitops::int_add(64)
}
fn validate(raw: &Value) -> Result<Value, String> {
    let s = &raw["summary"];
    let p = n(s, "p")?;
    let l = n(s, "l")?;
    let small = n(s, "small_base")?;
    let base = n(s, "full_base")?;
    if n(s, "b_count")? == 0
        || n(s, "b_count")? > p.checked_mul(p).ok_or("B space overflow")?
        || n(s, "line_count")? == 0
        || !(101..=1511).contains(&p)
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || l % 2 == 0
        || small == 0
        || small >= base
        || base > p
        || small
            != base
                .checked_mul(n(s, "small_percent")?)
                .ok_or("base overflow")?
                / 100
        || s["schema"] != "cover.two-large-prime.component/v1"
        || s["m"] != 9
        || s["complete_cost"] != false
        || !s["total_bitops"].is_null()
        || s["comparative_eligibility"] != false
        || s["full_dlp_recovery"] != false
        || s["rho_parity"] != false
        || s["foreign_hook_calls"] != 0
        || s["known_uncounted"].as_array().is_none_or(|a| a.is_empty())
        || s["other_modulus_ops"]
            .as_array()
            .is_none_or(|a| a.len() != 15 || a.iter().any(|v| v != 0))
    {
        return Err("domain/coverage flags".into());
    }
    let mode = s["mode"].as_str().ok_or("mode")?;
    if !["smooth", "two-lp"].contains(&mode) {
        return Err("mode".into());
    }
    let inputs = raw["inputs"].as_array().ok_or("inputs")?;
    if inputs.len() > 4096
        || n(s, "unique_verified")? != inputs.len() as u64
        || n(s, "candidates")?
            != n(s, "refused")?
                .checked_add(n(s, "duplicates")?)
                .and_then(|v| v.checked_add(inputs.len() as u64))
                .ok_or("candidate overflow")?
    {
        return Err("input/candidate counts".into());
    }
    let mut rows: Vec<Row> = Vec::new();
    let mut kinds = [0u64; 3];
    let mut identities = BTreeMap::new();
    let mut reverse = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for (id, r) in inputs.iter().enumerate() {
        let terms: Vec<(u64, i8)> =
            serde_json::from_value(r["full"]["terms"].clone()).map_err(|_| "full terms")?;
        if terms.len() != 9
            || terms
                .iter()
                .any(|&(i, e)| i >= base || ![-1, 1].contains(&e))
            || terms.windows(2).any(|w| w[0].0 >= w[1].0)
        {
            return Err("full relation shape".into());
        }
        let neg: Vec<_> = terms.iter().map(|&(i, e)| (i, -e)).collect();
        if seen.contains(&terms) || seen.contains(&neg) {
            return Err("duplicate original relation".into());
        }
        seen.insert(terms.clone());
        let part = &r["partial"];
        let a = canonical(&part["small"], small, l)?;
        let b = canonical(&part["large"], p, l)?;
        if n(part, "id")? != id as u64
            || part["rhs"] != 0
            || b.len() > 2
            || (mode == "smooth" && !b.is_empty())
        {
            return Err("input identity/large count".into());
        }
        let expected: Terms = terms
            .iter()
            .filter(|&&(i, _)| i < small)
            .map(|&(i, e)| (i, if e == 1 { 1 } else { l - 1 }))
            .collect();
        if a != expected || b.len() != terms.len() - a.len() {
            return Err("small/large binding".into());
        }
        for (&(i, e), &(x, v)) in terms.iter().filter(|&&(i, _)| i >= small).zip(&b) {
            if v != if e == 1 { 1 } else { l - 1 }
                || identities.insert(i, x).is_some_and(|old| old != x)
                || reverse.insert(x, i).is_some_and(|old| old != i)
            {
                return Err("point identity/coefficient binding".into());
            }
        }
        kinds[b.len()] += 1;
        rows.push((a, b));
    }
    if s["by_large_count"] != json!(kinds) {
        return Err("large count histogram".into());
    }
    let certs = raw["certificates"].as_array().ok_or("certificates")?;
    if certs.len() > 4096
        || n(s, "complete_rows")? != certs.len() as u64
        || n(s, "replayed_rows")? != certs.len() as u64
        || n(&s["graph"], "complete_relations")? != certs.len() as u64
        || n(&s["graph"], "input_relations")? != inputs.len() as u64
    {
        return Err("output/graph counts".into());
    }
    let mut complete = Vec::new();
    let mut nonzeros = 0;
    for r in certs {
        let small_row = canonical(&r["small"], small, l)?;
        let sources = canonical(&r["sources"], inputs.len() as u64, l)?;
        if sources.is_empty() || r["rhs"] != 0 {
            return Err("empty certificate or RHS".into());
        }
        let (mut a, mut b) = (BTreeMap::new(), BTreeMap::new());
        for (id, c) in sources {
            accumulate(&mut a, &rows[id as usize].0, c, l);
            accumulate(&mut b, &rows[id as usize].1, c, l);
        }
        let replay: Terms = a.into_iter().filter(|&(_, c)| c != 0).collect();
        if b.values().any(|&v| v != 0) || replay != small_row {
            return Err("independent certificate replay mismatch".into());
        }
        nonzeros += small_row.len();
        complete.push(small_row);
    }
    if n(s, "output_nonzeros")? != nonzeros as u64 {
        return Err("output nonzeros".into());
    }
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let phases = s["phases"].as_array().ok_or("phases")?;
    if phases.len() != 9 {
        return Err("phase count".into());
    }
    let mut sum: BTreeMap<String, u64> = KINDS.into_iter().map(|k| (k.into(), 0)).collect();
    let mut subtotal = 0u128;
    for (v, phase) in phases.iter().zip(PHASES) {
        if v["phase"] != phase {
            return Err("phase order".into());
        }
        let c = counts(&v["primitives"])?;
        let priced = price(&c, &widths)?;
        if v["diagnostic_priced_bitops"] != priced.to_string() {
            return Err("phase repricing".into());
        }
        subtotal = subtotal.checked_add(priced).ok_or("price overflow")?;
        for (k, n) in c {
            let slot = sum.get_mut(&k).ok_or("primitive")?;
            *slot = slot.checked_add(n).ok_or("counter overflow")?;
        }
    }
    if counts(&s["observed_primitives"])? != sum {
        return Err("phase sum".into());
    }
    let u = &s["uniform_sampling"];
    let calls = n(u, "calls")?;
    let draws = n(u, "raw_draws")?;
    let rejected = n(u, "rejections")?;
    let int_price = (draws as u128) * bitop_mul64() + (calls as u128) * bitop_add64();
    if u["schema"] != "modular-uniform-events/v1"
        || u["complete"] != false
        || u["other_modulus_calls"] != 0
        || draws != calls.checked_add(rejected).ok_or("draw overflow")?
        || draws > sum["rng_words"]
        || u["priced_native"]["integer_mul64"] != draws
        || u["priced_native"]["integer_add64"] != calls
        || u["known_native_priced_bitops"] != int_price.to_string()
        || u["unpriced_native"]["integer_sub64"] != calls.checked_mul(2).ok_or("sub overflow")?
        || u["unpriced_native"]["integer_clz64"] != calls
        || u["unpriced_native"]["integer_shl64"] != calls
    {
        return Err("uniform events".into());
    }
    Ok(
        json!({"mode":mode,"recorded_jacobian_verified_inputs":inputs.len(),"partial_by_large_count":kinds,"certificate_replays":complete.len(),"complete_rows":complete.len(),"rank":rank(&complete,small as usize,l),"output_nonzeros":nonzeros,"graph":s["graph"],"phases":phases,"observed_primitive_subtotal_bitops":subtotal.to_string(),"selected_uniform_events":u,"recorded_inner_wall_ns_practicality_only":s["inner_wall_ns_practicality_only"]}),
    )
}
fn analyze(a: &Value, b: &Value, plan: &Value) -> Result<Value, String> {
    if plan["driver_kind"] != "cover-two-large-prime-component"
        || plan["allow_comparative_claims"] != false
        || plan["automatic_retries"] != 0
    {
        return Err("plan".into());
    }
    let sa = &a["summary"];
    let sb = &b["summary"];
    let source = plan["source_revision"].as_str().ok_or("source SHA")?;
    let digest = sa["cohort_digest"].as_str().ok_or("cohort digest")?;
    if source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || digest.len() != 64
        || !digest.bytes().all(|c| c.is_ascii_hexdigit())
        || plan["m"] != 9
        || plan["mode_order"] != json!(["smooth", "two-lp"])
        || plan["worker_timeout_seconds"] != 900
        || plan["timeout_seconds"] != 1900
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
    {
        return Err("frozen source/cohort/protocol".into());
    }
    for k in [
        "p",
        "seed",
        "m",
        "small_percent",
        "b_count",
        "source_revision",
    ] {
        if sa[k] != plan[k] || sb[k] != plan[k] {
            return Err(format!("frozen {k} mismatch"));
        }
    }
    for k in [
        "l",
        "small_base",
        "full_base",
        "line_count",
        "cohort_digest",
    ] {
        if sa[k] != sb[k] {
            return Err(format!("paired {k} mismatch"));
        }
    }
    if sa["mode"] != "smooth" || sb["mode"] != "two-lp" {
        return Err("arm order".into());
    }
    let x = validate(a)?;
    let y = validate(b)?;
    let nontrivial = n(&sb["graph"], "cycles_closed")? > 0
        && n(&y, "rank")? > 0
        && sb["by_large_count"][2].as_u64().is_some_and(|n| n > 0);
    Ok(
        json!({"schema":"cover.two-large-prime.component-analysis/v1","source_revision":plan["source_revision"],"p":sa["p"],"seed":sa["seed"],"m":9,"small_percent":sa["small_percent"],"small_base":sa["small_base"],"full_base":sa["full_base"],"b_count":sa["b_count"],"line_count":sa["line_count"],"cohort_digest":sa["cohort_digest"],"outcomes":[x,y],"binding_replay_price_checks_passed":true,"nontrivial_two_lp_graph":nontrivial,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"independent_signed_verdict":false,"full_dlp_recovery":false,"rho_parity":false,"flow":{"planned":2,"attempted":2,"completed":2,"excluded":0,"automatic_retries":0}}),
    )
}
fn read(path: &str) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(path)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB bound".into());
    }
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err(
            "usage: two-lp-component-analysis SMOOTH.json TWO-LP.json PLAN.json NEW-SUMMARY.json"
                .into(),
        );
    }
    let parent = std::path::Path::new(&a[1]).parent().ok_or("raw parent")?;
    if std::path::Path::new(&a[2]).parent() != Some(parent) {
        return Err("raw parents differ".into());
    }
    for file in [
        "exit-code.txt",
        "smooth.exit-code.txt",
        "two-lp.exit-code.txt",
    ] {
        if fs::read_to_string(parent.join(file))?.trim() != "0" {
            return Err("failed/censored process".into());
        }
    }
    let plan = read(&a[3])?;
    let flow: Vec<Value> = fs::read_to_string(parent.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 4 {
        return Err("incomplete flow".into());
    }
    for (i, row) in flow.iter().enumerate() {
        if row["mode"] != if i < 2 { "smooth" } else { "two-lp" }
            || row["record_type"]
                != if i % 2 == 0 {
                    "execution_start"
                } else {
                    "execution"
                }
            || row["source_revision"] != plan["source_revision"]
            || (i % 2 == 1 && row["exit_code"] != 0)
        {
            return Err("flow schedule/source/exit mismatch".into());
        }
    }
    let s = analyze(&read(&a[1])?, &read(&a[2])?, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[4])?;
    serde_json::to_writer_pretty(&mut out, &s)?;
    writeln!(out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn symbolic_fixture() -> Value {
        let zero = json!(KINDS
            .into_iter()
            .map(|k| (k, 0u64))
            .collect::<BTreeMap<_, _>>());
        let phases: Vec<_> = PHASES
            .into_iter()
            .map(|phase| json!({"phase":phase,"primitives":zero,"diagnostic_priced_bitops":"0"}))
            .collect();
        let full_a: Vec<_> = (0..9).map(|i| (i, 1)).collect();
        let full_b: Vec<_> = (0..9).map(|i| (i, if i < 7 { 1 } else { -1 })).collect();
        let small: Vec<_> = (0..7).map(|i| (i, 1)).collect();
        let out: Vec<_> = (0..7).map(|i| (i, 2)).collect();
        json!({"summary":{"schema":"cover.two-large-prime.component/v1","mode":"two-lp","p":101,"l":103,"m":9,"small_base":7,"full_base":10,"small_percent":75,"b_count":100,"line_count":50,
        "unique_verified":2,"candidates":2,"refused":0,"duplicates":0,"by_large_count":[0,0,2],"complete_rows":1,"replayed_rows":1,"output_nonzeros":7,
        "graph":{"input_relations":2,"complete_relations":1,"cycles_closed":1},"phases":phases,"observed_primitives":zero,
        "uniform_sampling":{"schema":"modular-uniform-events/v1","complete":false,"calls":0,"raw_draws":0,"rejections":0,"other_modulus_calls":0,"priced_native":{"integer_mul64":0,"integer_add64":0},"known_native_priced_bitops":"0","unpriced_native":{"integer_sub64":0,"integer_clz64":0,"integer_shl64":0}},
        "foreign_hook_calls":0,"other_modulus_ops":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"known_uncounted":["symbolic fixture; no group verification"],"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"full_dlp_recovery":false,"rho_parity":false},
        "inputs":[{"full":{"terms":full_a},"partial":{"id":0,"small":small,"large":[[90,1],[91,1]],"rhs":0}},{"full":{"terms":full_b},"partial":{"id":1,"small":small,"large":[[90,102],[91,102]],"rhs":0}}],
        "certificates":[{"small":out,"rhs":0,"sources":[[0,1],[1,1]]}]})
    }
    #[test]
    fn symbolic_fixture_binds_certificates_prices_and_coverage() {
        let v = symbolic_fixture();
        let s = validate(&v).unwrap();
        assert_eq!(s["rank"], 1);
        for damage in 0..7 {
            let mut bad = v.clone();
            match damage {
                0 => bad["certificates"][0]["sources"][0][1] = json!(2),
                1 => bad["inputs"][0]["full"]["terms"][0][1] = json!(-1),
                2 => bad["inputs"][1]["partial"]["large"][0][0] = json!(89),
                3 => bad["summary"]["phases"][0]["diagnostic_priced_bitops"] = json!("1"),
                4 => bad["summary"]["complete_cost"] = json!(true),
                5 => {
                    bad["summary"]["uniform_sampling"]["unpriced_native"]["integer_sub64"] =
                        json!(1)
                }
                _ => bad["summary"]["l"] = json!(105),
            }
            assert!(validate(&bad).is_err(), "damage {damage}");
        }
    }
    #[test]
    fn rank_modulus_is_prime_including_wide_and_pseudoprime_controls() {
        for p in [2, 3, 101, 103, (1 << 61) - 1] {
            assert!(prime(p));
        }
        for n in [0, 1, 4, 561, 341550071728321] {
            assert!(!prime(n));
        }
    }
    #[test]
    fn paired_symbolic_records_bind_frozen_cohort_without_promoting_dlp() {
        let mut b = symbolic_fixture();
        b["summary"]["full_base"] = json!(12);
        b["summary"]["small_base"] = json!(9);
        for row in b["inputs"].as_array_mut().unwrap() {
            row["full"]["terms"][7][0] = json!(9);
            row["full"]["terms"][8][0] = json!(10);
        }
        let mut a = b.clone();
        a["summary"]["mode"] = json!("smooth");
        for (id, row) in a["inputs"].as_array_mut().unwrap().iter_mut().enumerate() {
            row["full"]["terms"][7][0] = json!(7);
            row["full"]["terms"][8][0] = json!(8);
            row["partial"]["small"] = json!((0..9)
                .map(|i| (i, if id == 1 && i >= 7 { 102 } else { 1 }))
                .collect::<Terms>());
            row["partial"]["large"] = json!([]);
        }
        a["certificates"] = json!((0..2)
            .map(
                |id| json!({"small":a["inputs"][id]["partial"]["small"],"rhs":0,"sources":[[id,1]]})
            )
            .collect::<Vec<_>>());
        a["summary"]["by_large_count"] = json!([2, 0, 0]);
        a["summary"]["complete_rows"] = json!(2);
        a["summary"]["replayed_rows"] = json!(2);
        a["summary"]["output_nonzeros"] = json!(18);
        a["summary"]["graph"]["complete_relations"] = json!(2);
        a["summary"]["graph"]["cycles_closed"] = json!(0);
        for raw in [&mut a, &mut b] {
            raw["summary"]["source_revision"] = json!("0".repeat(40));
            raw["summary"]["cohort_digest"] = json!("0".repeat(64));
            raw["summary"]["seed"] = json!(1);
        }
        let plan = json!({"driver_kind":"cover-two-large-prime-component","allow_comparative_claims":false,"automatic_retries":0,"source_revision":"0".repeat(40),"p":101,"seed":1,"m":9,"small_percent":75,"b_count":100,"mode_order":["smooth","two-lp"],"worker_timeout_seconds":900,"timeout_seconds":1900,"rounds":1,"warmup_rounds":0});
        let result = analyze(&a, &b, &plan).unwrap();
        assert_eq!(result["nontrivial_two_lp_graph"], true);
        assert_eq!(result["full_dlp_recovery"], false);
        assert_eq!(result["comparative_eligibility"], false);
        for key in ["cohort_digest", "b_count", "source_revision"] {
            let mut bad = b.clone();
            bad["summary"][key] = json!("changed");
            assert!(analyze(&a, &bad, &plan).is_err());
        }
    }
    #[test]
    fn modular_rank_counts_independence_not_rows() {
        let rows = vec![
            vec![(0, 1), (1, 2)],
            vec![(0, 2), (1, 4)],
            vec![(1, 1), (2, 1)],
        ];
        assert_eq!(rank(&rows, 3, 101), 2);
        assert_eq!(rank(&[], 3, 101), 0);
        let wide = (1u64 << 61) - 1;
        assert_eq!(rank(&[vec![(0, wide - 1)], vec![(0, 2)]], 1, wide), 1);
    }
    #[test]
    fn independent_replay_cancels_large_vertices() {
        let rows = [
            (vec![(0, 1)], vec![(97, 1), (98, 2)]),
            (vec![(1, 3)], vec![(97, 100), (98, 99)]),
        ];
        let (mut small, mut large) = (BTreeMap::new(), BTreeMap::new());
        for (a, b) in &rows {
            accumulate(&mut small, a, 1, 101);
            accumulate(&mut large, b, 1, 101);
        }
        assert!(large.values().all(|&c| c == 0));
        assert_eq!(small, BTreeMap::from([(0, 1), (1, 3)]));
        accumulate(&mut large, &vec![(97, 1)], 1, 101);
        assert!(large.values().any(|&c| c != 0));
    }
}

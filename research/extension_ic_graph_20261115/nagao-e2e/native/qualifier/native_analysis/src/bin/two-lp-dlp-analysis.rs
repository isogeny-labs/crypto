//! Producer scalar/certificate qualifier. No independent signed admission.
#[allow(dead_code)]
#[path = "two-lp-component-analysis.rs"]
pub(crate) mod shared;
use serde_json::{json, Value};
use shared::{
    accumulate, add, canonical, counts, mul, n, pow, price, prime, Field, Terms, Widths, KINDS,
    PHASES,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

/// Independent batch Gaussian elimination, including affine consistency.
/// The protected target is last; a pivot there is a unit target equation.
pub(crate) fn target_solution(
    rows: &[(Terms, u64)],
    columns: usize,
    p: u64,
) -> Result<(usize, Option<u64>), String> {
    let mut a = vec![vec![0u64; columns + 1]; rows.len()];
    for (i, (row, rhs)) in rows.iter().enumerate() {
        for &(c, v) in row {
            a[i][c as usize] = v;
        }
        a[i][columns] = *rhs;
    }
    let (mut r, mut answer) = (0, None);
    for col in 0..columns {
        let Some(pivot) = (r..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(r, pivot);
        let inv = pow(a[r][col], p - 2, p);
        for v in &mut a[r][col..] {
            *v = mul(*v, inv, p);
        }
        let pivot_row = a[r].clone();
        for row in a.iter_mut().skip(r + 1) {
            let factor = row[col];
            if factor == 0 {
                continue;
            }
            for (v, &b) in row[col..].iter_mut().zip(&pivot_row[col..]) {
                *v = add(*v, p - mul(factor, b, p), p);
            }
        }
        if col + 1 == columns {
            answer = Some(a[r][columns]);
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    if a.iter()
        .skip(r)
        .any(|row| row[..columns].iter().all(|&v| v == 0) && row[columns] != 0)
    {
        return Err("inconsistent affine certificate system".into());
    }
    Ok((r, answer))
}
pub(crate) fn counter_check(w: &Value, p: u64, l: u64) -> Result<Value, String> {
    let v = &w["work_native"];
    if v["schema"] != "cover.two-large-prime.dlp-work/v1"
        || v["complete"] != false
        || !v["total_bitops"].is_null()
        || v["comparative_eligibility"] != false
        || v["foreign_hook_calls"] != 0
        || v["other_modulus_ops"]
            .as_array()
            .is_none_or(|a| a.len() != 15 || a.iter().any(|v| v != 0))
        || v["known_uncounted"].as_array().is_none_or(|a| a.is_empty())
    {
        return Err("coverage/ownership".into());
    }
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let phases = v["phases"].as_array().ok_or("phases")?;
    if phases.len() != 9 {
        return Err("phase count".into());
    }
    let mut sum: BTreeMap<String, u64> = KINDS.into_iter().map(|k| (k.into(), 0)).collect();
    let mut subtotal = 0u128;
    for (row, name) in phases.iter().zip(PHASES) {
        if row["phase"] != name {
            return Err("phase order".into());
        }
        let c = counts(&row["primitives"])?;
        let value = price(&c, &widths)?;
        if row["diagnostic_priced_bitops"] != value.to_string() {
            return Err("phase price".into());
        }
        subtotal = subtotal.checked_add(value).ok_or("price overflow")?;
        for (k, v) in c {
            let slot = sum.get_mut(&k).ok_or("counter")?;
            *slot = slot.checked_add(v).ok_or("counter overflow")?;
        }
    }
    if sum != counts(&v["observed_primitives"])? {
        return Err("phase sum".into());
    }
    let harness = counts(&v["excluded_harness_primitives"])?;
    if v["excluded_harness_uniform_events"]["complete"] != false {
        return Err("excluded harness sampler scope".into());
    }
    let u = &v["uniform_sampling"];
    let calls = n(u, "calls")?;
    let draws = n(u, "raw_draws")?;
    let rejections = n(u, "rejections")?;
    if calls == 0
        || u["schema"] != "modular-uniform-events/v1"
        || u["complete"] != false
        || u["other_modulus_calls"] != 0
        || draws != calls.checked_add(rejections).ok_or("RNG overflow")?
        || draws > sum["rng_words"]
        || u["priced_native"]["integer_mul64"] != draws
        || u["priced_native"]["integer_add64"] != calls
        || u["unpriced_native"]["integer_sub64"] != calls.checked_mul(2).ok_or("RNG overflow")?
        || u["unpriced_native"]["integer_clz64"] != calls
        || u["unpriced_native"]["integer_shl64"] != calls
        || u["known_native_priced_bitops"]
            != ((draws as u128) * shared::bitop_mul64() + (calls as u128) * shared::bitop_add64())
                .to_string()
    {
        return Err("uniform conservation/prices".into());
    }
    Ok(
        json!({"observed_primitive_subtotal_bitops":subtotal.to_string(),"phases":phases,"uniform_sampling":u,"excluded_harness_primitives":harness,"excluded_harness_uniform_events":v["excluded_harness_uniform_events"],"complete":false,"total_bitops":null}),
    )
}

fn decode_full(v: &Value, base: u64, len: usize) -> Result<Vec<(u64, i8)>, String> {
    let t: Vec<(u64, i8)> = serde_json::from_value(v.clone()).map_err(|_| "signed source terms")?;
    if t.len() != len
        || t.iter().any(|&(i, e)| i >= base || ![-1, 1].contains(&e))
        || t.windows(2).any(|w| w[0].0 >= w[1].0)
    {
        return Err("signed source shape".into());
    }
    Ok(t)
}
fn bind_split(
    terms: &[(u64, i8)],
    row: &Value,
    small: u64,
    base: u64,
    p: u64,
    l: u64,
    ids: &mut BTreeMap<u64, u64>,
) -> Result<(), String> {
    let a = canonical(&row["small"], small + 1, l)?;
    let b = canonical(&row["large"], p, l)?;
    let expected: Terms = terms
        .iter()
        .filter(|&&(i, _)| i < small)
        .map(|&(i, e)| (i, if e == 1 { 1 } else { l - 1 }))
        .collect();
    let actual: Terms = a.into_iter().filter(|&(i, _)| i < small).collect();
    if actual != expected || b.len() != terms.iter().filter(|&&(i, _)| i >= small).count() {
        return Err("source split".into());
    }
    for (&(i, e), &(x, c)) in terms.iter().filter(|&&(i, _)| i >= small).zip(&b) {
        if i >= base
            || c != if e == 1 { 1 } else { l - 1 }
            || ids.insert(i, x).is_some_and(|old| old != x)
        {
            return Err("LP identity/sign".into());
        }
    }
    Ok(())
}
pub(crate) fn candidate_proof(
    w: &Value,
    plan: &Value,
    l: u64,
    expected: u64,
) -> Result<Value, String> {
    candidate_proof_with_collection_mode(w, plan, l, expected, false)
}
#[allow(dead_code)] // Used by the separate rank-stop qualifier binary.
pub(crate) fn rank_stopped_candidate_proof(
    w: &Value,
    plan: &Value,
    l: u64,
    expected: u64,
) -> Result<Value, String> {
    if w["work"]["options"]["collection_rank_stride"] != plan["collection_rank_stride"]
        || n(&w["work"], "b_count_completed")? > n(plan, "b_count")?
    {
        return Err("rank-stopped collection bounds/options".into());
    }
    candidate_proof_with_collection_mode(w, plan, l, expected, true)
}
fn candidate_proof_with_collection_mode(
    w: &Value,
    plan: &Value,
    l: u64,
    expected: u64,
    rank_stopped: bool,
) -> Result<Value, String> {
    let v = &w["work"];
    let small = n(v, "small_base")?;
    let base = n(v, "full_base")?;
    let p = n(&w["public"], "p")?;
    if v["schema"] != "cover.two-large-prime.dlp/v1"
        || v["p"] != p
        || v["l"] != l
        || v["seed"] != plan["seed"]
        || v["m"] != 9
        || v["options"]["small_percent"] != plan["small_percent"]
        || v["options"]["b_count"] != plan["b_count"]
        || v["options"]["max_descent_residuals"] != plan["max_descent_residuals"]
        || v["options"]["max_accepted_descents"] != plan["max_accepted_descents"]
        || small == 0
        || small >= base
        || base > p
        || small != base * n(plan, "small_percent")? / 100
        || v["b_count_planned"] != plan["b_count"]
        || (!rank_stopped && v["b_count_completed"] != plan["b_count"])
        || v["actual_scalar"] != expected
        || v["verified"] != true
        || v["capped"] != false
        || !v["resource_refusal"].is_null()
        || v["invalid_descents"] != 0
        || v["fallback_solves"] != 0
        || v["complete_cost"] != false
        || !v["total_bitops"].is_null()
        || v["comparative_eligibility"] != false
    {
        return Err("candidate frozen options/endpoint/coverage".into());
    }
    let originals = v["source_relations"].as_array().ok_or("original sources")?;
    let ds = v["descents"].as_array().ok_or("descents")?;
    if originals.len() > 4096
        || ds.len() < 2
        || ds.len() > n(plan, "max_accepted_descents")? as usize
        || n(v, "verified_source_rows")? != originals.len() as u64
        || n(v, "descent_residuals")? > n(plan, "max_descent_residuals")?
    {
        return Err("source/descent counts".into());
    }
    if n(v, "candidates")?
        != n(v, "refused")?
            .checked_add(n(v, "duplicates")?)
            .and_then(|n| n.checked_add(originals.len() as u64))
            .ok_or("candidate overflow")?
    {
        return Err("candidate counts".into());
    }
    let proof = &v["final_projection"];
    if proof["target_column"] != small {
        return Err("protected target".into());
    }
    let inputs = proof["inputs"].as_array().ok_or("affine inputs")?;
    if inputs.len() != originals.len() + ds.len() {
        return Err("affine input count".into());
    }
    let (mut ids, mut seen) = (BTreeMap::new(), BTreeSet::new());
    let mut histogram = [0u64; 3];
    for (i, row) in originals.iter().enumerate() {
        let terms = decode_full(&row["full"]["terms"], base, 9)?;
        let neg: Vec<_> = terms.iter().map(|&(i, e)| (i, -e)).collect();
        if seen.contains(&terms) || seen.contains(&neg) {
            return Err("duplicate original".into());
        }
        seen.insert(terms.clone());
        if row["partial"] != inputs[i] || inputs[i]["id"] != i as u64 || inputs[i]["rhs"] != 0 {
            return Err("original input binding".into());
        }
        bind_split(&terms, &inputs[i], small, base, p, l, &mut ids)?;
        let k = inputs[i]["large"].as_array().ok_or("LP list")?.len();
        if k > 2 {
            return Err("more than two LPs".into());
        }
        histogram[k] += 1;
        if inputs[i]["small"]
            .as_array()
            .ok_or("small list")?
            .iter()
            .any(|t| t[0] == small)
        {
            return Err("homogeneous source targets scalar".into());
        }
    }
    if v["by_large_count"] != json!(histogram) || histogram[2] == 0 {
        return Err("actual two-LP sources absent".into());
    }
    for (i, d) in ds.iter().enumerate() {
        let a = n(d, "a")?;
        let b = n(d, "b")?;
        let terms = decode_full(&d["decomposition"]["terms"], base, 6)?;
        let row = &inputs[originals.len() + i];
        if a >= l
            || b == 0
            || b >= l
            || d["modulus"] != l
            || row["id"] != (originals.len() + i) as u64
            || row["rhs"] != a
        {
            return Err("descent labels/RHS".into());
        }
        bind_split(&terms, row, small, base, p, l, &mut ids)?;
        if canonical(&row["small"], small + 1, l)?.last() != Some(&(small, l - b)) {
            return Err("target coefficient".into());
        }
    }
    let mut reverse = BTreeSet::new();
    if ids.values().any(|&x| !reverse.insert(x)) {
        return Err("two vertices for same point".into());
    }
    let certs = proof["certificates"].as_array().ok_or("certificates")?;
    if certs.is_empty()
        || certs.len() > 4096
        || proof["replayed_rows"] != certs.len() as u64
        || proof["report"]["complete_relations"] != certs.len() as u64
        || proof["report"]["input_relations"] != inputs.len() as u64
        || proof["report"]["rejected_fill"] != 0
        || n(&proof["report"], "cycles_closed")? == 0
    {
        return Err("graph/replay flow".into());
    }
    let mut rows = Vec::new();
    let mut nonzeros = 0;
    for c in certs {
        let small_row = canonical(&c["small"], small + 1, l)?;
        let sources = canonical(&c["sources"], inputs.len() as u64, l)?;
        let rhs = n(c, "rhs")?;
        if sources.is_empty() || rhs >= l {
            return Err("empty/invalid affine certificate".into());
        }
        let (mut a, mut b, mut right) = (BTreeMap::new(), BTreeMap::new(), 0u64);
        for (id, k) in sources {
            let input = &inputs[id as usize];
            accumulate(&mut a, &canonical(&input["small"], small + 1, l)?, k, l);
            accumulate(&mut b, &canonical(&input["large"], p, l)?, k, l);
            right = add(right, mul(k, n(input, "rhs")?, l), l);
        }
        if b.values().any(|&v| v != 0)
            || a.into_iter().filter(|&(_, v)| v != 0).collect::<Terms>() != small_row
            || right != rhs
        {
            return Err("independent affine replay mismatch".into());
        }
        nonzeros += small_row.len();
        rows.push((small_row, rhs));
    }
    let (rank, scalar) = target_solution(&rows, small as usize + 1, l)?;
    if scalar != Some(expected) || n(v, "last_rank")? > rank as u64 {
        return Err("target not independently determined".into());
    }
    let attempts = v["projection_solve_attempts"]
        .as_array()
        .ok_or("attempts")?;
    if attempts.is_empty()
        || attempts.last().ok_or("attempt")?["verified"] != true
        || attempts.iter().any(|a| {
            a.get("projection_error").is_some()
                || a["candidate_scalar"]
                    .as_u64()
                    .is_some_and(|_| a["verified"] != true)
        })
    {
        return Err("failed determined candidate or projection".into());
    }
    Ok(
        json!({"verified_original_inputs":originals.len(),"partial_by_large_count":histogram,"small_base":small,"full_base":base,"accepted_descents":ds.len(),"descent_residuals":v["descent_residuals"],"descent_stream_advances":v["descent_stream_advances"],"projection_attempts":attempts.len(),"certificate_replays":certs.len(),"matrix_rank":rank,"target_scalar":scalar,"output_nonzeros":nonzeros,"graph":proof["report"]}),
    )
}
fn analyze(fixture: &Value, a: &Value, b: &Value, plan: &Value) -> Result<Value, String> {
    let public = &fixture["public"];
    let p = n(public, "p")?;
    let l = n(public, "l")?;
    let expected = n(fixture, "expected_scalar")?;
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if !(101..=1511).contains(&p)
        || !(3..1u64 << 63).contains(&l)
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || plan["m"] != 9
        || plan["fixture_timeout_seconds"] != 60
        || !(1..100).contains(&n(plan, "small_percent")?)
        || n(plan, "b_count")? == 0
        || n(plan, "b_count")? > p * p
        || !(1..=1_000_000).contains(&n(plan, "max_descent_residuals")?)
        || !(2..=32).contains(&n(plan, "max_accepted_descents")?)
        || source.len() != 40
        || !source.bytes().all(|b| b.is_ascii_hexdigit())
        || plan["driver_kind"] != "cover-two-large-prime-dlp-validation"
        || plan["allow_comparative_claims"] != false
        || plan["automatic_retries"] != 0
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["worker_timeout_seconds"] != 900
        || plan["timeout_seconds"] != 1900
        || plan["mode_order"] != json!(["ic", "two-lp"])
        || public["p"] != plan["p"]
        || fixture["seed"] != plan["seed"]
        || fixture["source_revision"] != source
        || !prime(p)
        || !prime(l)
        || l >= 1 << 63
        || expected == 0
        || expected >= l
    {
        return Err("fixture/protocol/domain".into());
    }
    let mut outcomes = Vec::new();
    for (w, mode) in [(a, "ic"), (b, "two-lp")] {
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["seed"] != plan["seed"]
            || w["method"] != mode
            || w["public"] != *public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != expected
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("worker public-input/actual scalar binding".into());
        }
        outcomes.push(json!({"method":mode,"actual_scalar":expected,"verified":true,"recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"],"accounting":counter_check(w,p,l)?}));
    }
    let proof = candidate_proof(b, plan, l, expected)?;
    Ok(
        json!({"schema":"cover.two-large-prime.dlp-validation-analysis/v1","source_revision":source,"p":p,"seed":plan["seed"],"l":l,"expected_scalar":expected,"outcomes":outcomes,"candidate_proof":proof,"full_recovery_validation_passed":true,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"rho_parity":false,"independent_signed_verdict":false,"flow":{"planned":2,"attempted":2,"completed":2,"excluded":0,"automatic_retries":0}}),
    )
}
fn read(p: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(p)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB".into());
    }
    Ok(serde_json::from_reader(File::open(p)?)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: two-lp-dlp-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for file in [
        "exit-code.txt",
        "fixture.exit-code.txt",
        "ic.exit-code.txt",
        "two-lp.exit-code.txt",
    ] {
        if fs::read_to_string(run.join(file))?.trim() != "0" {
            return Err("failed/censored process".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let fixture = read(&run.join("fixture.json"))?;
    if read(&run.join("public.json"))? != fixture["public"] {
        return Err("public fixture mismatch".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 6 {
        return Err("flow incomplete".into());
    }
    for (i, v) in flow.iter().enumerate() {
        let (mode, kind) = match i {
            0 => ("fixture", "fixture_start"),
            1 => ("fixture", "fixture"),
            2 => ("ic", "execution_start"),
            3 => ("ic", "execution"),
            4 => ("two-lp", "execution_start"),
            _ => ("two-lp", "execution"),
        };
        if v["mode"] != mode
            || v["record_type"] != kind
            || v["source_revision"] != plan["source_revision"]
            || (i % 2 == 1 && v["exit_code"] != 0)
        {
            return Err("flow/source/exit mismatch".into());
        }
    }
    let summary = analyze(
        &fixture,
        &read(&run.join("ic.raw.json"))?,
        &read(&run.join("two-lp.raw.json"))?,
        &plan,
    )?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protected_last_target_can_be_determined_without_full_rank() {
        let rows = vec![(vec![(0, 1), (1, 1)], 2), (vec![(0, 1), (1, 2)], 5)];
        assert_eq!(target_solution(&rows, 2, 103).unwrap(), (2, Some(3)));
        assert_eq!(
            target_solution(&[(vec![(2, 7)], 21)], 3, 103).unwrap(),
            (1, Some(3))
        );
        assert_eq!(
            target_solution(&[(vec![(0, 1), (2, 1)], 5)], 3, 103)
                .unwrap()
                .1,
            None
        );
    }
    #[test]
    fn contradictory_affine_rhs_is_rejected() {
        let rows = vec![(vec![(0, 1)], 2), (vec![(0, 1)], 3)];
        assert!(target_solution(&rows, 1, 101).is_err());
    }
}

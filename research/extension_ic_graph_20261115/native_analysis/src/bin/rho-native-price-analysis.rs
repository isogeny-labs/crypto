//! Same-walk native rho arithmetic pricing qualification; no parity claim.
#[allow(dead_code)]
#[path = "two-lp-component-analysis.rs"]
mod shared;
use serde_json::{json, Value};
use shared::{counts, n, price, prime, Field, Widths, KINDS};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};
pub(crate) type Counts = BTreeMap<String, u64>;
const OPS: [&str; 13] = [
    "mul",
    "sq",
    "inv",
    "walk_adds",
    "dbl_adds",
    "setup_adds",
    "fp_adds",
    "canon_cmp",
    "hash",
    "dp_check",
    "scalar_ops",
    "table_ops",
    "cycle_cmp",
];
fn zero() -> Counts {
    KINDS.into_iter().map(|k| (k.into(), 0)).collect()
}
fn plus(c: &mut Counts, key: &str, value: u64) -> Result<(), String> {
    let x = c.get_mut(key).ok_or("primitive key")?;
    *x = x.checked_add(value).ok_or("counter overflow")?;
    Ok(())
}
fn times(a: u64, b: u64) -> Result<u64, String> {
    a.checked_mul(b).ok_or("counter product overflow".into())
}
fn arithmetic(c: &Value, native: bool) -> Result<Counts, String> {
    let (mut out, m, s, i, a) = (
        zero(),
        n(c, "mul")?,
        n(c, "sq")?,
        n(c, "inv")?,
        n(c, "fp_adds")?,
    );
    plus(
        &mut out,
        "field_mul",
        times(m.checked_add(s).ok_or("mul/square sum")?, 55)?,
    )?;
    plus(
        &mut out,
        "field_mul",
        times(i, if native { 80 } else { 150 })?,
    )?;
    plus(
        &mut out,
        "field_add",
        times(m.checked_add(s).ok_or("mul/square sum")?, 44)?,
    )?;
    plus(
        &mut out,
        "field_add",
        times(i, if native { 42 } else { 104 })?,
    )?;
    plus(&mut out, "field_add", a)?;
    plus(&mut out, "field_inv", i)?;
    Ok(out)
}
fn difference(a: &Value, b: &Value) -> Result<Value, String> {
    let mut out = serde_json::Map::new();
    for key in OPS {
        out.insert(
            key.into(),
            json!(n(a, key)?
                .checked_sub(n(b, key)?)
                .ok_or("operation counter regression")?),
        );
    }
    Ok(Value::Object(out))
}
fn expected_phases(w: &Value, native: bool) -> Result<Vec<Counts>, String> {
    let c = &w["work"];
    let walk = difference(&c["run"]["ops"], &c["setup"])?;
    let mut pre = arithmetic(&c["setup"], native)?;
    let rng = n(c, "rng_draws")?;
    plus(&mut pre, "word_op", rng)?;
    plus(&mut pre, "rng_words", rng)?;
    let mut walking = arithmetic(&walk, native)?;
    let flips = times(n(c, "flips")?, 6)?;
    walking.insert(
        "field_add".into(),
        walking["field_add"]
            .checked_sub(flips)
            .ok_or("flip charge exceeds adds")?,
    );
    plus(&mut walking, "field_add", times(n(&walk, "canon_cmp")?, 6)?)?;
    plus(&mut walking, "word_op", n(&walk, "canon_cmp")?)?;
    plus(&mut walking, "word_op", n(&walk, "cycle_cmp")?)?;
    plus(
        &mut walking,
        "word_op",
        times(
            n(&walk, "hash")?
                .checked_sub(n(&walk, "dp_check")?)
                .ok_or("DP hash counter")?,
            6,
        )?,
    )?;
    plus(&mut walking, "modr_add", n(&walk, "scalar_ops")?)?;
    let mut collision = zero();
    plus(&mut collision, "word_op", times(n(&walk, "dp_check")?, 7)?)?;
    plus(
        &mut collision,
        "word_op",
        times(n(&walk, "table_ops")?, 12)?,
    )?;
    plus(
        &mut collision,
        "hash_probe",
        times(n(&walk, "table_ops")?, 2)?,
    )?;
    let solves = n(c, "collision_solves")?;
    plus(&mut collision, "modr_inv", solves)?;
    plus(&mut collision, "modr_mul", solves)?;
    plus(&mut collision, "modr_add", times(solves, 3)?)?;
    let mut verification = arithmetic(&c["verify"], native)?;
    plus(&mut verification, "word_op", 12)?;
    Ok(vec![pre, walking, collision, verification])
}
fn calibration(v: &Value) -> Result<(), String> {
    if v["schema"] != "rho.native-tower-primitives/v1"
        || v["complete"] != false
        || v["legacy_tuned_credits_included"] != false
    {
        return Err("native calibration identity/scope".into());
    }
    for (label, m, a, i) in [
        ("mul", 55, 44, 0),
        ("square", 55, 44, 0),
        ("inverse", 80, 42, 1),
    ] {
        let mut expected = zero();
        expected.insert("field_mul".into(), m);
        expected.insert("field_add".into(), a);
        expected.insert("field_inv".into(), i);
        if counts(&v[label])? != expected {
            return Err(format!("native {label} primitive calibration"));
        }
    }
    Ok(())
}
pub(crate) fn account(w: &Value, native: bool, p: u64, l: u64) -> Result<(Value, Counts), String> {
    let v = &w["work_native"];
    let label = if native {
        "executed native Barrett tower prime-field primitives"
    } else {
        "existing generic Fq3 harness primitive prices"
    };
    let gaps = v["known_uncounted"].as_array().ok_or("gaps")?;
    if v["schema"] != "cover.graph.accounting-diagnostic/v1"
        || v["unit"] != "bitop/v1"
        || v["complete"] != false
        || !v["total_bitops"].is_null()
        || v["comparison_eligible"] != false
        || v["foreign_hook_calls"] != 0
        || v["other_modulus_ops"] != 0
        || v["pricing"] != label
        || gaps.is_empty()
        || !gaps.iter().any(|g| g["name"] == "rng_internal_work")
        || gaps
            .iter()
            .any(|g| g["name"] == "native_extension_formula_vs_generic_harness_pricing")
            == native
    {
        return Err("accounting scheme/coverage/ownership".into());
    }
    if native {
        calibration(&v["native_tower_calibration"])?;
        if v["counted_items"] != json!(["counted:native_rho_tower_prime_field_arithmetic"]) {
            return Err("native coverage marker".into());
        }
    }
    let phases = v["phases"].as_array().ok_or("phases")?;
    let expected = expected_phases(w, native)?;
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let mut sum = zero();
    let mut subtotal = 0u128;
    let mut outer = zero();
    if phases.len() != 4
        || v["widths"]["field_bits"] != 64 - p.leading_zeros()
        || v["widths"]["r_bits"] != 64 - l.leading_zeros()
    {
        return Err("phase count/widths".into());
    }
    for (i, (row, name)) in phases
        .iter()
        .zip(["precompute", "walk", "collision", "verification"])
        .enumerate()
    {
        if row["phase"] != name {
            return Err("phase order".into());
        }
        let actual = counts(&row["primitives"])?;
        if i < 3 && actual != expected[i] {
            return Err(format!("independent explicit counts for {name}"));
        }
        if i == 3 {
            for (k, &value) in &actual {
                outer.insert(
                    k.clone(),
                    value
                        .checked_sub(expected[i][k])
                        .ok_or("verification undercounts explicit work")?,
                );
            }
        }
        let cost = price(&actual, &widths)?;
        if row["diagnostic_priced_bitops"] != cost.to_string() {
            return Err("phase repricing".into());
        }
        subtotal = subtotal.checked_add(cost).ok_or("subtotal overflow")?;
        for (k, value) in actual {
            plus(&mut sum, &k, value)?;
        }
    }
    if (n(w, "recovered_scalar")? > 1 && (outer["field_mul"] == 0 || outer["field_inv"] == 0))
        || sum != counts(&v["observed_primitives"])?
        || v["diagnostic_priced_bitops"] != subtotal.to_string()
    {
        return Err("outer replay/phase sum/subtotal".into());
    }
    Ok((
        json!({"pricing":label,"observed_primitive_subtotal_bitops":subtotal.to_string(),"phases":phases,"explicit_counts_independently_reconstructed":true,"outer_replay_primitives":outer,"native_calibration":if native{v["native_tower_calibration"].clone()}else{Value::Null},"complete":false,"total_bitops":null}),
        outer,
    ))
}
fn analyze(fixture: &Value, workers: &[Value], plan: &Value) -> Result<Value, String> {
    let public = &fixture["public"];
    let (p, l, answer) = (
        n(public, "p")?,
        n(public, "l")?,
        n(fixture, "expected_scalar")?,
    );
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if workers.len() != 2
        || plan["backend"] != "runpod_user_authorized"
        || plan["runpod_compute_type"] != "CPU"
        || plan["cpu_affinity_rule"] != "first_allowed_cpu_bound_before_call"
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|v| v.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || plan["driver_kind"] != "rho-native-price-validation"
        || plan["mode_order"] != json!(["harness", "native"])
        || plan["p"] != p
        || fixture["seed"] != plan["seed"]
        || fixture["source_revision"] != source
        || fixture["workers_receive"] != "public only"
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || source.len() != 40
        || !source.bytes().all(|b| b.is_ascii_hexdigit())
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["allow_comparative_claims"] != false
        || plan["fixture_timeout_seconds"] != 60
        || plan["worker_timeout_seconds"] != 900
        || plan["timeout_seconds"] != 1900
        || !(101..=1511).contains(&p)
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || answer == 0
        || answer >= l
    {
        return Err("fixture/frozen protocol/domain".into());
    }
    let mut outcomes = Vec::new();
    let mut outer = None;
    for (i, w) in workers.iter().enumerate() {
        let r = &w["work"]["run"];
        let dp = 16.min((l.ilog2() / 2).saturating_sub(4));
        if w["method"] != "rho"
            || w["solver_answer_placeholder"] != 0
            || w["recovered_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
            || w["work"]["verified"] != true
            || r["correct"] != false
            || r["p"] != p
            || r["l"] != l
            || r["seed"] != plan["seed"]
            || r["run"] != 0
            || r["negation"] != true
            || r["batch"] != 16
            || r["r"] != 256
            || r["dp_bits"] != dp
            || r["capped"] != false
            || n(r, "steps")? == 0
        {
            return Err("public worker/actual scalar/walk identity".into());
        }
        let (a, residual) = account(w, i == 1, p, l)?;
        if outer.as_ref().is_some_and(|v| v != &residual) {
            return Err("different additional elliptic replay".into());
        }
        outer = Some(residual);
        outcomes.push(json!({"arm":plan["mode_order"][i],"actual_scalar":answer,"verified":true,"accounting":a,"recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"]}));
    }
    if !same_trace(&workers[0], &workers[1]) {
        return Err("walk/high-level semantic trace differs".into());
    }
    Ok(
        json!({"schema":"rho.native-price-validation-analysis/v1","source_revision":source,"p":p,"l":l,"seed":plan["seed"],"actual_scalar":answer,"steps":workers[0]["work"]["run"]["steps"],"same_semantic_work":true,"same_outer_replay_primitives":true,"native_calibration":workers[1]["work_native"]["native_tower_calibration"],"outcomes":outcomes,"native_price_validation_passed":true,"algorithm_speed_improvement":false,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"rho_parity":false,"legacy_tuned_credits_are_separate":true,"independent_signed_verdict":false,"flow":{"planned":2,"attempted":2,"completed":2,"excluded":0,"automatic_retries":0}}),
    )
}
fn read(p: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(p)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB".into());
    }
    Ok(serde_json::from_reader(File::open(p)?)?)
}
fn semantic_work(v: &Value) -> Value {
    match v {
        Value::Object(o) => Value::Object(
            o.iter()
                .filter(|(k, _)| k.as_str() != "wall_ms" && k.as_str() != "filter_call_wall_ns")
                .map(|(k, v)| (k.clone(), semantic_work(v)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.iter().map(semantic_work).collect()),
        _ => v.clone(),
    }
}
fn same_trace(a: &Value, b: &Value) -> bool {
    let x = semantic_work(&a["work"]);
    let y = semantic_work(&b["work"]);
    !x.is_null() && x == y && a["signature"]["work"] == x && b["signature"]["work"] == y
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: rho-native-price-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in [
        "exit-code.txt",
        "fixture.exit-code.txt",
        "harness.exit-code.txt",
        "native.exit-code.txt",
    ] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
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
        return Err("incomplete flow".into());
    }
    for (i, v) in flow.iter().enumerate() {
        let mode = ["fixture", "harness", "native"][i / 2];
        let kind = if i == 0 {
            "fixture_start"
        } else if i == 1 {
            "fixture"
        } else if i % 2 == 0 {
            "execution_start"
        } else {
            "execution"
        };
        if v["mode"] != mode
            || v["record_type"] != kind
            || v["source_revision"] != plan["source_revision"]
            || (i % 2 == 1 && v["exit_code"] != 0)
        {
            return Err("flow/source/exit".into());
        }
    }
    let workers = [
        read(&run.join("harness.raw.json"))?,
        read(&run.join("native.raw.json"))?,
    ];
    let summary = analyze(&fixture, &workers, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    let mut pins = Vec::new();
    for file in [
        "runtime-plan.json",
        "fixture.json",
        "public.json",
        "harness.raw.json",
        "native.raw.json",
        "flow.jsonl",
    ] {
        let bytes = fs::read(run.join(file))?;
        pins.push(json!({"file":file,"bytes":bytes.len(),"blake3":blake3::hash(&bytes).to_hex().to_string()}));
    }
    let export = json!({"schema":"rho.native-price.raw-export/v1","canonical_schema_conformance_claimed":false,"protocol":plan,"fixture":fixture,"workers":workers,"flow":flow,"analysis":summary,"exact_original_files":pins,"independent_signed_verdict":false});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run.join("raw-export.json"))?;
    serde_json::to_writer(&mut out, &export)?;
    writeln!(out)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matching_signatures_cannot_hide_changed_recorded_operations() {
        let signature = json!({"run":{"ops":{"mul":7}}});
        let a =
            json!({"work":{"run":{"ops":{"mul":7},"wall_ms":1.0}},"signature":{"work":signature}});
        let mut b =
            json!({"work":{"run":{"ops":{"mul":7},"wall_ms":2.0}},"signature":{"work":signature}});
        assert!(same_trace(&a, &b));
        b["work"]["run"]["ops"]["mul"] = json!(8);
        assert!(!same_trace(&a, &b));
        b["signature"]["work"]["run"]["ops"]["mul"] = json!(8);
        assert!(!same_trace(&a, &b));
    }
    #[test]
    fn hand_phase_counts_separate_rng_canonicalization_collision_and_replay() {
        let mut setup = serde_json::Map::new();
        let mut total = serde_json::Map::new();
        for key in OPS {
            setup.insert(key.into(), json!(0));
            total.insert(key.into(), json!(0));
        }
        for (key, value) in [("mul", 1), ("sq", 1), ("inv", 1), ("fp_adds", 10)] {
            setup.insert(key.into(), json!(value));
            total.insert(key.into(), json!(value));
        }
        for (key, value) in [
            ("mul", 2),
            ("sq", 1),
            ("inv", 1),
            ("fp_adds", 10),
            ("canon_cmp", 3),
            ("hash", 5),
            ("dp_check", 2),
            ("scalar_ops", 4),
            ("table_ops", 2),
            ("cycle_cmp", 2),
        ] {
            let old = total[key].as_u64().unwrap();
            total.insert(key.into(), json!(old + value));
        }
        let w = json!({"work":{"setup":setup,"run":{"ops":total},"verify":{"mul":2,"sq":1,"inv":1,"fp_adds":3},"rng_draws":7,"flips":1,"collision_solves":1}});
        let phases = expected_phases(&w, true).unwrap();
        assert_eq!(phases[0]["field_mul"], 190);
        assert_eq!(phases[0]["field_add"], 140);
        assert_eq!(phases[0]["rng_words"], 7);
        assert_eq!(phases[0]["word_op"], 7);
        assert_eq!(phases[1]["field_mul"], 245);
        assert_eq!(phases[1]["field_add"], 196);
        assert_eq!(phases[1]["word_op"], 23);
        assert_eq!(phases[1]["modr_add"], 4);
        assert_eq!(phases[2]["word_op"], 38);
        assert_eq!(phases[2]["hash_probe"], 4);
        assert_eq!(phases[2]["modr_inv"], 1);
        assert_eq!(phases[2]["modr_mul"], 1);
        assert_eq!(phases[2]["modr_add"], 3);
        assert_eq!(phases[3]["field_mul"], 245);
        assert_eq!(phases[3]["field_inv"], 1);
        assert_eq!(phases[3]["word_op"], 12);
        let mut forged = w;
        forged["work"]["run"]["ops"]["inv"] = json!(0);
        assert!(expected_phases(&forged, true).is_err());
    }
    #[test]
    fn hand_operation_vector_changes_inverse_arithmetic_without_double_inv() {
        let c = json!({"mul":2,"sq":1,"inv":1,"fp_adds":3});
        let native = arithmetic(&c, true).unwrap();
        let harness = arithmetic(&c, false).unwrap();
        assert_eq!(native["field_mul"], 245);
        assert_eq!(native["field_add"], 177);
        assert_eq!(native["field_inv"], 1);
        assert_eq!(harness["field_mul"], 315);
        assert_eq!(harness["field_add"], 239);
        assert_eq!(harness["field_inv"], 1);
    }
    #[test]
    fn calibration_rejects_double_inversion_and_unknown_primitives() {
        let mut base = zero();
        base.insert("field_mul".into(), 55);
        base.insert("field_add".into(), 44);
        let mut inverse = zero();
        inverse.insert("field_mul".into(), 80);
        inverse.insert("field_add".into(), 42);
        inverse.insert("field_inv".into(), 1);
        let mut v = json!({"schema":"rho.native-tower-primitives/v1","complete":false,"legacy_tuned_credits_included":false,"mul":base,"square":base,"inverse":inverse});
        assert!(calibration(&v).is_ok());
        v["inverse"]["field_inv"] = json!(2);
        assert!(calibration(&v).is_err());
        v["inverse"]["field_inv"] = json!(1);
        v["mul"]["modr_mul"] = json!(1);
        assert!(calibration(&v).is_err());
    }
}

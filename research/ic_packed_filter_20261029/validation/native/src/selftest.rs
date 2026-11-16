//! Negative controls on the validator's own checkers (port of selftest.py).
//! A checker that reports 0 mismatches is only evidence if it can report one:
//! each test perturbs a copy of real records and requires the checker to flag it.

use crate::j3::{cond8_fields, diff, reprice};
use crate::py::{self, int_str, Obj};
use crate::scalars::check_session;
use crate::v;
use serde_json::Value;
use std::path::Path;

fn has_timing(o: &Value) -> bool {
    match o {
        Value::Object(m) => m.iter().filter(|(k, _)| k.as_str() != "isolation").any(|(k, v)| v::is_timing_key(k) || has_timing(v)),
        Value::Array(a) => a.iter().any(has_timing),
        _ => false,
    }
}

pub fn run(out: &Path) {
    let recs = v::load_wall(32);
    let arms = v::all_by_key(&recs);
    let dgbf = &arms[&"dgbf".to_string()];
    let dgb = &arms[&"dgb".to_string()];
    let k = dgbf.key_set().into_iter().nth(10).unwrap();
    let (base, r) = (dgb[&k], dgbf[&k]);
    let d = |a: &Value, b: &Value| diff(&cond8_fields(a, true), &cond8_fields(b, true), "");
    let mut o = Obj::new();
    o.set("identical_pair_no_diff", d(base, r).is_empty());
    // 1. native count off by one
    let mut r2 = r.clone();
    {
        let x = &mut r2["phases"][2]["native"]["lookups"];
        *x = py::int_val(py::int(x) + 1);
    }
    o.set("native_plus_one_detected", !d(base, &r2).is_empty());
    // 2. work_native counter added
    let mut r3 = r.clone();
    r3["phases"][1]["work_native"].as_object_mut().unwrap().insert("extra".into(), Value::from(1));
    o.set("work_native_extra_key_detected", !d(base, &r3).is_empty());
    // 3. recovered log changed
    let mut r4 = r.clone();
    r4["outcome"]["recovered"] = Value::from((int_str(&r4["outcome"]["recovered"]) + 1).to_string());
    o.set("recovered_changed_detected", !d(base, &r4).is_empty());
    // 4. mismatched pairing (another target) must differ
    let k2 = (k.0.clone(), (k.1 + 1).rem_euclid(24), k.2);
    o.set("wrong_pairing_detected", !d(dgb[&k2], r).is_empty());
    // 5. re-pricing catches a one-bitop change
    let mut r5 = r.clone();
    r5["work"]["phases"][1]["bitops"] = Value::from((int_str(&r5["work"]["phases"][1]["bitops"]) + 1).to_string());
    o.set("reprice_detects_one_bitop", !reprice(&r5).is_empty());
    o.set("reprice_clean_record_ok", reprice(r).is_empty());
    // 6. scalar check rejects a wrong log
    let mut sess: Vec<&Value> = vec![&r4];
    sess.extend(recs.iter().filter(|x| x["workload"]["curve"]["slug"].as_str() == Some(k.0.as_str())).take(20));
    let sc = check_session(&sess);
    o.set("scalar_check_rejects_wrong_log", sc["failures"].as_array().unwrap().len() == 1);
    // 7. tfbs on the unfiltered arm would be seen
    let mut r6 = base.clone();
    r6["phases"][1]["work_native"].as_object_mut().unwrap().insert("table_filter_bit_sets".into(), Value::from(5));
    o.set("tfbs_on_dgb_seen", v::tfbs(&r6).1 == vec!["oracle_setup".to_string()]);
    // 8. the counted view removes every wall/timing field
    let cv = v::counted_view(r);
    let iso_keys: Vec<&String> = cv["isolation"].as_object().unwrap().keys().collect();
    let iso_ok = iso_keys.len() == 2 && iso_keys.iter().any(|k| *k == "level") && iso_keys.iter().any(|k| *k == "blockers");
    o.set("counted_view_has_no_timing_field", !has_timing(&cv) && iso_ok);
    let masked = v::mask_blockers(&[Value::from("L2: solve too short to admit wall time: solve 1.0 \u{b5}s")]);
    o.set("counted_view_masks_too_short", masked == vec![Value::from(v::TOO_SHORT)]);
    let all = o.0.values().all(|x| x == &Value::Bool(true));
    o.set("all_pass", all);
    py::dump(&o.val(), out);
    println!("selftest all_pass {all}");
}

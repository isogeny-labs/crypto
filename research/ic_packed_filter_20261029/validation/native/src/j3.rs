//! J3 (port of j3_work.py): charged-work identity.
//!
//! Condition 8 record half: every dgbf and dgbf-edge record, warm-ups
//! included, against the dgb record of the same (curve, target, round). Bitop
//! half: every record's bitop/v1 work re-priced from its primitive counts
//! (docs/units/BITOPS.md at 3271a25d02), and dgbf - dgb = 384 x
//! table_filter_bit_sets, phase by phase. Condition 10 for every IC record.
//! Counted dgb/dgbf records equal the wall records in the condition 8 fields.

use crate::py::{self, int, int_str, py_eq, repr, Num, Obj};
use crate::v::{self, OMap};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const PHASE_FIELDS: [&str; 4] = ["adds", "doubles", "scalar_mults", "gae"];

fn prices(b: i128, w: i128) -> BTreeMap<&'static str, i128> {
    let fa = 13 * b;
    let fm = 2 * (b * b + 5 * b * (b - 1)) + 13 * b;
    let fi = 2 * b * 13 * b;
    let (i, m, s, a) = (fi, fm, fm, fa);
    BTreeMap::from([
        ("field_add", fa),
        ("field_mul", fm),
        ("field_sqr", fm),
        ("field_inv", fi),
        ("field_sqrt", 2 * b * fm),
        ("legendre", fi),
        ("group_add", i + 2 * m + s + 6 * a),
        ("group_double", i + 2 * m + 2 * s + 8 * a),
        ("modr_add", 13 * w),
        ("modr_mul", 2 * (w * w + 5 * w * (w - 1)) + 13 * w),
        ("modr_inv", 2 * w * 13 * w),
        ("word_op", 64),
        ("hash_probe", 4 * b),
    ])
}

/// Re-price a record's bitop/v1 work; the list of (phase, mine, recorded) that differ.
pub fn reprice(r: &Value) -> Vec<Value> {
    let wk = &r["work"];
    let b = int(&wk["widths"]["field"]["bits"]);
    let w = int(&wk["widths"]["r_bits"]);
    if wk["widths"]["field"]["kind"].as_str() != Some("prime") {
        v::stop("non-prime field");
    }
    let p = prices(b, w);
    let (mut bad, mut tot) = (Vec::new(), 0i128);
    for ph in wk["phases"].as_array().unwrap() {
        let mut s = 0i128;
        for (prim, n) in ph["prims"].as_object().unwrap() {
            let price = p.get(prim.as_str()).unwrap_or_else(|| v::stop(format!("unpriced primitive {prim} in {}", r["record_id"])));
            s += price * int(n);
        }
        if s != int_str(&ph["bitops"]) {
            bad.push(Value::Array(vec![ph["name"].clone(), py::int_val(s), ph["bitops"].clone()]));
        }
        tot += s;
    }
    if tot != int_str(&wk["bitops"]) {
        bad.push(Value::Array(vec!["total".into(), py::int_val(tot), wk["bitops"].clone()]));
    }
    bad
}

fn unpriced_counts(r: &Value) -> Value {
    let mut out = Map::new();
    let counters = v::get_or_empty(r, "counters");
    for name in r["cost"]["unpriced"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let base = name.strip_suffix("_uncharged").unwrap_or(name);
        if let Some(c) = counters.get(name) {
            out.insert(name.into(), c.clone());
            continue;
        }
        let (mut t, mut found) = (Num::Int(0), false);
        for p in r["phases"].as_array().unwrap() {
            let nat = v::get_or_empty(p, "native");
            if let Some(x) = nat.get(name) {
                t = t.add(Num::of(x));
                found = true;
            } else if let Some(x) = nat.get(base) {
                t = t.add(Num::of(x));
                found = true;
            }
        }
        out.insert(name.into(), if found { t.val() } else { Value::Null });
    }
    Value::Object(out)
}

pub fn cond8_fields(r: &Value, drop_tfbs: bool) -> Value {
    let mut ph = Map::new();
    for p in r["phases"].as_array().unwrap() {
        let mut wn = v::get_or_empty(p, "work_native").as_object().unwrap().clone();
        if drop_tfbs {
            wn.shift_remove("table_filter_bit_sets");
        }
        let mut o = Map::new();
        for f in PHASE_FIELDS {
            o.insert(f.into(), p.get(f).cloned().unwrap_or_else(|| v::stop(format!("KeyError {f}"))));
        }
        o.insert("native".into(), v::get_or_empty(p, "native").clone());
        o.insert("work_native".into(), Value::Object(wn));
        ph.insert(p["name"].as_str().unwrap().to_string(), Value::Object(o));
    }
    let mut o = Obj::new();
    o.set("recovered", r["outcome"]["recovered"].clone())
        .set("total_gae", r["cost"]["total_gae"].clone())
        .set("unpriced", unpriced_counts(r))
        .set("phases", Value::Object(ph));
    o.val()
}

pub fn diff(a: &Value, b: &Value, path: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let (Value::Object(ma), Value::Object(mb)) = (a, b) {
        let ks: BTreeSet<&String> = ma.keys().chain(mb.keys()).collect();
        for k in ks {
            match (ma.get(k), mb.get(k)) {
                (Some(x), Some(y)) => out.extend(diff(x, y, &format!("{path}/{k}"))),
                (x, _) => out.push(format!("{path}/{k}: missing on {}", if x.is_none() { "A" } else { "B" })),
            }
        }
    } else if !py_eq(a, b) {
        out.push(format!("{path}: {} != {}", repr(a), repr(b)));
    }
    out
}

fn first5(d: &[String]) -> Value {
    Value::Array(d.iter().take(5).map(|s| Value::from(s.clone())).collect())
}

fn key_list(k: &v::Key) -> Value {
    serde_json::json!([k.0, k.1, k.2])
}

fn tfbs_where_is(r: &Value, want: &[&str]) -> bool {
    v::tfbs(r).1.iter().map(|s| s.as_str()).eq(want.iter().copied())
}

fn bitops(r: &Value) -> i128 {
    int_str(&r["work"]["bitops"])
}

fn cond8(recs: &[Value]) -> Value {
    let arms = v::all_by_key(recs);
    let a = |n: &str| &arms[&n.to_string()];
    let mut matched = Obj::new();
    let mut mism = Vec::new();
    for arm in ["dgbf", "dgbf-edge", "dgb-aa"] {
        let mut n = 0u64;
        for (k, r) in a(arm).iter() {
            let base = a("dgb")[k];
            let d = diff(&cond8_fields(base, true), &cond8_fields(r, true), "");
            n += 1;
            if !d.is_empty() {
                let mut o = Obj::new();
                o.set("arm", arm)
                    .set("key", key_list(k))
                    .set("record_id", r["record_id"].clone())
                    .set("dgb_record_id", base["record_id"].clone())
                    .set("diff", first5(&d));
                mism.push(o.val());
            }
        }
        matched.set(arm, n);
    }
    let absent = ["dgb", "dgb-aa"].iter().all(|x| a(x).values().all(|r| tfbs_where_is(r, &[])));
    let present = ["dgbf", "dgbf-edge"].iter().all(|x| a(x).values().all(|r| tfbs_where_is(r, &["oracle_setup"])));
    let mut per_curve: OMap<String, BTreeSet<i128>> = OMap::new();
    for x in ["dgbf", "dgbf-edge"] {
        for (k, r) in a(x).iter() {
            per_curve.entry_or(k.0.clone(), BTreeSet::new).insert(v::tfbs(r).0);
        }
    }
    let mut vpc = Obj::new();
    for (c, s) in per_curve.iter() {
        vpc.set(c.clone(), s.iter().map(|x| py::int_val(*x)).collect::<Vec<_>>());
    }
    let mut tf = Obj::new();
    tf.set("absent_on_dgb_and_dgb_aa", absent)
        .set("present_once_in_oracle_setup_on_filtered", present)
        .set("values_per_curve", vpc.val())
        .set("one_value_per_curve", per_curve.values().all(|s| s.len() == 1));
    // bitop identity, phase by phase
    let (mut bad_ident, mut bad_price, mut n) = (Vec::new(), Vec::new(), 0u64);
    for r in recs {
        let bp = reprice(r);
        if !bp.is_empty() {
            bad_price.push(serde_json::json!({"record_id": r["record_id"].clone(), "bad": bp}));
        }
    }
    for arm in ["dgbf", "dgbf-edge"] {
        for (k, r) in a(arm).iter() {
            let base = a("dgb")[k];
            let (t, _) = v::tfbs(r);
            n += 1;
            let dt = bitops(r) - bitops(base);
            let phs = |x: &Value| -> Vec<(String, Value)> {
                x["work"]["phases"].as_array().unwrap().iter().map(|p| (p["name"].as_str().unwrap().to_string(), p.clone())).collect()
            };
            let mut ph_r: OMap<String, Value> = OMap::new();
            for (nm, p) in phs(r) {
                ph_r.insert(nm, p);
            }
            let mut ph_b: OMap<String, Value> = OMap::new();
            for (nm, p) in phs(base) {
                ph_b.insert(nm, p);
            }
            let mut ok = dt == 384 * t && ph_r.key_set() == ph_b.key_set();
            for (name, pr) in ph_r.iter() {
                let pb = &ph_b[name];
                let d_ph = int_str(&pr["bitops"]) - int_str(&pb["bitops"]);
                let want = if name == "oracle_setup" { 384 * t } else { 0 };
                ok &= d_ph == want;
                let (mr, mb) = (pr["prims"].as_object().unwrap(), pb["prims"].as_object().unwrap());
                let prims: BTreeSet<&String> = mr.keys().chain(mb.keys()).collect();
                let mut dp: BTreeMap<&str, i128> = BTreeMap::new();
                for p in prims {
                    let x = mr.get(p).map(int).unwrap_or(0) - mb.get(p).map(int).unwrap_or(0);
                    if x != 0 {
                        dp.insert(p.as_str(), x);
                    }
                }
                let want_dp: BTreeMap<&str, i128> =
                    if name == "oracle_setup" { BTreeMap::from([("word_op", 6 * t)]) } else { BTreeMap::new() };
                ok &= dp == want_dp;
            }
            if !ok {
                let mut o = Obj::new();
                o.set("arm", arm)
                    .set("key", key_list(k))
                    .set("record_id", r["record_id"].clone())
                    .set("delta", py::int_val(dt))
                    .set("tfbs", py::int_val(t));
                bad_ident.push(o.val());
            }
        }
    }
    let meas = |x: &str| -> i128 { a(x).values().filter(|r| !py::truthy(&r["warmup"])).map(|r| bitops(r)).sum() };
    let mut bo = Obj::new();
    bo.set("records_repriced", recs.len() as u64)
        .set("reprice_mismatches", bad_price)
        .set("identity_checked", n)
        .set("identity_failures", bad_ident)
        .set("measured_sum_dgb", py::int_val(meas("dgb")))
        .set("measured_sum_dgbf", py::int_val(meas("dgbf")))
        .set("measured_ratio_dgbf_over_dgb", py::fval(py::idiv(meas("dgbf"), meas("dgb"))))
        .set("measured_difference", py::int_val(meas("dgbf") - meas("dgb")));
    let mut res = Obj::new();
    res.set("matched", matched.val()).set("mismatches", mism).set("tfbs", tf.val()).set("bitops", bo.val());
    res.val()
}

fn cond10(recs: &[Value]) -> Value {
    let mut bad = Vec::new();
    let (mut pmax, mut spmax) = (0i128, 0i128);
    let mut src = BTreeSet::new();
    let mut n = 0u64;
    for r in recs {
        if r["method"]["id"].as_str() != Some("ic.pipeline") {
            continue;
        }
        n += 1;
        let icv1 = r["workload"]["curve"]["icv1"].as_str().unwrap();
        let p: i128 = icv1.split(':').nth(1).unwrap().split('-').nth(1).unwrap().parse().unwrap();
        let fb = &r["factor_base"];
        let sp = if fb.as_object().unwrap().contains_key("signed_points") {
            src.insert("signed_points");
            int(&fb["signed_points"])
        } else {
            src.insert("2*abscissae");
            2 * int(&fb["abscissae"])
        };
        pmax = pmax.max(p);
        spmax = spmax.max(sp);
        if p > 4294967295 || sp > 65536 {
            bad.push(r["record_id"].clone());
        }
    }
    let mut o = Obj::new();
    o.set("ic_records", n)
        .set("p_max", py::int_val(pmax))
        .set("signed_points_max", py::int_val(spmax))
        .set("source", src.into_iter().collect::<Vec<_>>())
        .set("failures", bad);
    o.val()
}

fn counted_equals_wall(wall: &[Value], counted: &[Value]) -> Value {
    let (w, c) = (v::all_by_key(wall), v::all_by_key(counted));
    let mut res = Obj::new();
    let mut equal = true;
    for arm in ["dgb", "dgbf"] {
        let (wa, ca) = (&w[&arm.to_string()], &c[&arm.to_string()]);
        let mut mism = Vec::new();
        let mut n = 0u64;
        if wa.key_set() != ca.key_set() {
            mism.push(Value::from("key sets differ"));
        }
        for k in ca.key_set() {
            n += 1;
            let (rw, rc) = (wa[&k], ca[&k]);
            let mut d = diff(&cond8_fields(rw, false), &cond8_fields(rc, false), "");
            if bitops(rw) != bitops(rc) {
                d.push("work.bitops differ".into());
            }
            if !d.is_empty() {
                let mut o = Obj::new();
                o.set("key", key_list(&k))
                    .set("wall_record", rw["record_id"].clone())
                    .set("counted_record", rc["record_id"].clone())
                    .set("diff", first5(&d));
                mism.push(o.val());
            }
        }
        equal &= mism.is_empty();
        res.set(arm, serde_json::json!({"matched": n, "mismatches": mism}));
    }
    res.set("equal", equal);
    res.val()
}

pub fn run(out: &Path) {
    let mut o = Obj::new();
    for b in [32u32, 24, 28] {
        let counted = v::load_counted(b, "counted");
        let wall = if b == 28 {
            v::load_wall_counts_only(28) // descriptive session: counts only
        } else {
            let wall = v::load_wall(b);
            o.set(format!("P{b}-wall-cond8"), cond8(&wall));
            o.set(format!("P{b}-wall-cond10"), cond10(&wall));
            wall
        };
        o.set(format!("P{b}-counted-cond10"), cond10(&counted));
        o.set(format!("P{b}-counted-equals-wall"), counted_equals_wall(&wall, &counted));
        let bp: Vec<Value> = counted.iter().filter(|r| !reprice(r).is_empty()).map(|r| r["record_id"].clone()).collect();
        o.set(format!("P{b}-counted-reprice-mismatches"), bp);
        let cw = v::all_by_key(&counted);
        let (cf, cd) = (&cw[&"dgbf".to_string()], &cw[&"dgb".to_string()]);
        let fails: Vec<Value> = cf
            .iter()
            .filter(|(k, r)| bitops(r) - bitops(cd[*k]) != 384 * v::tfbs(r).0)
            .map(|(_, r)| r["record_id"].clone())
            .collect();
        o.set(format!("P{b}-counted-bitop-identity-failures"), fails);
        println!("P{b} j3 done");
    }
    py::dump(&o.val(), out);
}

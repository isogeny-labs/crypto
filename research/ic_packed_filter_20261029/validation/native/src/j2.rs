//! J2 (port of j2_instrument.py): instrument checks.
//!
//! Wall session (PROTOCOL 6.1 conditions 1-7, read after the SHA256SUMS
//! check): exit, audit, counted A/A, spec and build, L2 gate (re-counted and
//! re-graded from each record's isolation block), native A/A admission file,
//! block positions and predecessors. Counted session (conditions 1-6 and the
//! exit-42 clause), read through v::load_counted: no wall field, no rho wall time.

use crate::py::{self, int, int_key, py_eq, truthy, Num, Obj};
use crate::v::{self, OMap};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const REV: &str = "3271a25d02dcef6da148293fb8f9e2149ce59858";
pub const ECB: &str = "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503";
const WEIGHTS: [(f64, &str); 3] = [(0.0, "0.0"), (0.1, "0.1"), (1.0, "1.0")];

fn curves(b: u32) -> BTreeSet<&'static str> {
    match b {
        24 => ["icv1-fp24-t217-e1fbffd2", "icv1-fp24-tm567-4accd0e8"].into(),
        28 => ["icv1-fp28-tm295-85243301", "icv1-fp28-tm8273-f1778659"].into(),
        32 => ["icv1-fp32-t89141-95879fe3", "icv1-fp32-tm68303-f902d2f0"].into(),
        _ => unreachable!(),
    }
}

pub fn job(kind: &str, b: u32) -> &'static str {
    match (kind, b) {
        ("wall", 24) => "ic-dgbf-p24-20261029",
        ("wall", 32) => "ic-dgbf-p32-20261029",
        ("wall", 28) => "ic-dgbf-p28-20261029",
        ("counted", 24) => "ic-dgbf-rho-p24-20261029",
        ("counted", 32) => "ic-dgbf-rho-p32-20261029",
        ("counted", 28) => "ic-dgbf-rho-p28-20261029-r2",
        _ => unreachable!(),
    }
}

pub fn frozen_spec_name(b: u32, kind: &str) -> String {
    if kind == "wall" {
        format!("spec-P{b}.json")
    } else {
        format!("spec-P{b}-rho.json")
    }
}

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or_else(|| v::stop(format!("expected a string, got {v}")))
}

fn level_from_blockers(bl: &[Value]) -> &'static str {
    let has = |p: &str| bl.iter().any(|b| s(b).starts_with(p));
    if has("L1: ") {
        "L0"
    } else if has("L2: ") {
        "L1"
    } else if has("L3: ") {
        "L2"
    } else {
        "L3"
    }
}

/// A Counter rendered as a dict (keys in first-seen order; output is sorted).
fn counter_val(c: &OMap<String, i128>) -> Value {
    let mut o = Obj::new();
    for (k, n) in c.iter() {
        o.set(k.clone(), py::int_val(*n));
    }
    o.val()
}

fn bump(c: &mut OMap<String, i128>, k: impl Into<String>) {
    *c.entry_or(k.into(), || 0) += 1;
}

fn sha_line_map(eks: &Path) -> BTreeMap<String, String> {
    v::sums_lines(eks).into_iter().map(|(h, n)| (n, h)).collect()
}

fn common_file_checks(b: u32, kind: &str, records: &[Value]) -> Map<String, Value> {
    let eks = v::session_dir(b, kind);
    let mut out = Map::new();
    let (ok, n, bad) = v::check_sums(&eks);
    out.insert("sha256sums".into(), serde_json::json!({"ok": ok, "files": n, "bad": bad}));
    let je = v::jload(&eks.join("job-exit.json"));
    let mut jo = Obj::new();
    jo.set("exit_code", je["exit_code"].clone())
        .set("job_name", je["job_name"].clone())
        .set("job_name_expected", je["job_name"].as_str() == Some(job(kind, b)))
        .set("single_pod_uid", je.get("pod_uid").and_then(|x| x.as_str()).is_some_and(|x| x.chars().count() == 36));
    out.insert("job_exit".into(), jo.val());
    let spec_name = frozen_spec_name(b, kind);
    let frozen = v::jload(&v::backlog().join(&spec_name));
    let inspec = v::jload(&eks.join("input-spec.json"));
    let sspec = v::jload(&eks.join("session").join("spec.json"));
    let bi = v::jload(&eks.join("build-info.json"));
    let binsha = py::split_ws(&v::read_text(&eks.join("binary.sha256")), -1)[0].clone();
    let sess = v::jload(&eks.join("session").join("session.json"));
    let slugs: BTreeSet<&str> = records.iter().map(|r| s(&r["workload"]["curve"]["slug"])).collect();
    let cseeds: BTreeSet<i128> = records.iter().map(|r| int(&r["workload"]["curve_spec"]["seed"])).collect();
    let want_seeds: Vec<i128> =
        vec![format!("202610296{b}3").parse().unwrap(), format!("202610296{b}4").parse().unwrap()];
    let tseeds: BTreeSet<i128> = records.iter().map(|r| int(&r["workload"]["target_seed"])).collect();
    let meas = &frozen["measurement"];
    let mut sb = Obj::new();
    sb.set("frozen_spec", spec_name.clone())
        .set("frozen_spec_sha256", v::sha256_file(&v::backlog().join(&spec_name)))
        .set("input_spec_json_equal_frozen", py_eq(&inspec, &frozen))
        .set("session_spec_json_equal_frozen", py_eq(&sspec, &frozen))
        .set("build_info_revision", bi["source_revision"].as_str() == Some(REV))
        .set("build_info_ecbench_sha256", bi["ecbench_sha256"].as_str() == Some(ECB))
        .set("binary_sha256_file", binsha == ECB)
        .set("session_binary_sha256", sess["binary_sha256"].as_str() == Some(ECB))
        .set("records_binary_sha256_all", records.iter().all(|r| r["binary_sha256"].as_str() == Some(ECB)))
        .set("records_session_id_all", records.iter().all(|r| py_eq(&r["session_id"], &sess["session_id"])))
        .set("session_status", sess["status"].clone())
        .set("grading_version", sess.get("grading_version").cloned().unwrap_or(Value::Null))
        .set("executions_planned", sess["executions_planned"].clone())
        .set("records_written", sess["records_written"].clone())
        .set("records_in_file", records.len() as u64)
        .set(
            "session_records_sha256_matches_file",
            sess["records_sha256"].as_str() == Some(&v::sha256_file(&eks.join("session").join("records.jsonl"))),
        )
        .set(
            "session_plan_sha256_matches_file",
            sess["plan_sha256"].as_str() == Some(&v::sha256_file(&eks.join("session").join("plan.json"))),
        )
        .set("curves_are_section3", slugs == curves(b))
        .set("curve_seeds", cseeds.into_iter().collect::<Vec<_>>() == want_seeds)
        .set("target_seed", tseeds == [format!("202610296{b}6").parse::<i128>().unwrap()].into())
        .set("order_seed_in_spec", Num::of(&meas["seed"]).eq(Num::Int(format!("202610296{b}8").parse().unwrap())))
        .set(
            "spec_rounds_warmup_order_level",
            Value::Array(
                ["rounds", "warmup", "order", "isolation_required", "timeout_seconds"]
                    .iter()
                    .map(|k| meas[*k].clone())
                    .collect(),
            ),
        );
    // algorithm seed equal across arms of each (curve, target, round)
    let mut seeds: OMap<v::Key, Vec<Value>> = OMap::new();
    for r in records {
        let e = seeds.entry_or(v::key(r), Vec::new);
        if !e.iter().any(|x| py_eq(x, &r["algorithm_seed"])) {
            e.push(r["algorithm_seed"].clone());
        }
    }
    sb.set("algorithm_seed_shared_by_all_arms_per_key", seeds.values().all(|s| s.len() == 1));
    out.insert("spec_build".into(), sb.val());
    let ap = eks.join("audit.json");
    if ap.exists() {
        let a = v::jload(&ap);
        let sums = sha_line_map(&eks);
        let measured = records.iter().filter(|r| !truthy(&r["warmup"])).count();
        let replays = a["replays"].as_array().unwrap();
        let mut rs: Vec<i128> = replays.iter().map(|x| int(&x["seq"])).collect();
        rs.sort();
        let mut ms: Vec<i128> = records.iter().filter(|r| !truthy(&r["warmup"])).map(|r| int(&r["seq"])).collect();
        ms.sort();
        let mut ao = Obj::new();
        ao.set("ok", a["ok"].clone())
            .set("session_status", a["session_status"].clone())
            .set("problems", a["problems"].as_array().unwrap().len() as u64)
            .set("records", a["records"].clone())
            .set("verified_records", a["verified_records"].clone())
            .set("replay_selection", a["replay_selection"].clone())
            .set("replay_candidates", a["replay_candidates"].clone())
            .set("measured_records", measured as u64)
            .set("replays", replays.len() as u64)
            .set(
                "replays_identical",
                replays.iter().filter(|x| truthy(&x["reproduced"]) && x["detail"].as_str() == Some("identical")).count()
                    as u64,
            )
            .set("replay_seqs_are_all_measured", rs == ms)
            .set("regraded", a.get("regraded").cloned().unwrap_or(Value::Null))
            .set("auditor_binary", a["auditor_binary_sha256"].as_str() == Some(ECB))
            .set(
                "audit_file_hashes_match_sums",
                a["files"]
                    .as_object()
                    .unwrap()
                    .iter()
                    .all(|(k, h)| sums.get(&format!("session/{k}")).map(|x| x.as_str()) == h.as_str().map(Some).unwrap_or(None)),
            )
            .set("audit_stdout_equals_audit_json", sums.get("audit.json") == sums.get("audit.stdout"));
        out.insert("audit".into(), ao.val());
    } else {
        out.insert("audit".into(), Value::Null);
    }
    out
}

fn regrade_counts(records: &[Value]) -> (OMap<String, i128>, Vec<Value>, OMap<String, i128>) {
    let mut lv = OMap::new();
    for r in records {
        bump(&mut lv, s(&r["isolation"]["level"]));
    }
    let mism: Vec<Value> = records
        .iter()
        .filter(|r| level_from_blockers(r["isolation"]["blockers"].as_array().unwrap()) != s(&r["isolation"]["level"]))
        .map(|r| r["record_id"].clone())
        .collect();
    let mut prefixes = OMap::new();
    for r in records {
        for bl in r["isolation"]["blockers"].as_array().unwrap() {
            let bl = s(bl);
            let head = bl.split(": ").next().unwrap();
            let tail = if bl.starts_with("L3") {
                py::prefix_chars(bl.split_once(": ").expect("blocker without ': '").1, 40)
            } else {
                "<L2 or L1 blocker>".to_string()
            };
            bump(&mut prefixes, format!("{head}: {tail}"));
        }
    }
    (lv, mism, prefixes)
}

fn g<'a>(o: &'a Value, k: &str) -> Option<&'a Value> {
    o.get(k).filter(|x| !x.is_null())
}

fn or_num(o: &Value, k: &str, d: i128) -> Num {
    // `o.get(k) or d`
    match o.get(k) {
        Some(x) if truthy(x) => Num::of(x),
        _ => Num::Int(d),
    }
}

/// Partial independent re-grade of the grading-v3 L1/L2 gates from each wall
/// record's raw isolation and timing fields.
fn raw_regrade_wall(b: u32, records: &[Value]) -> Value {
    let sess = v::jload(&v::session_dir(b, "wall").join("session").join("session.json"));
    let hz = Num::of(&sess["ticks_per_second"]);
    let hzf = hz.f();
    let th = &sess["options"]["thresholds"];
    let observed = sess["cpu_plan"]["observed"].as_array().unwrap();
    let mut flagged: OMap<String, i128> = OMap::new();
    let mut bad = Vec::new();
    let empty = Value::Object(Map::new());
    for r in records {
        let (iso, t) = (&r["isolation"], &r["time"]);
        let run = &iso["run_cpu"];
        let o = match t.get("solve") {
            Some(x) if truthy(x) => x,
            _ => &empty,
        };
        let wall_s = Num::of(&t["process_wall_ns"]).f() / 1e9;
        let cpu_s = Num::of(&t["user_ns"]).add(Num::of(&t["sys_ns"])).f() / 1e9;
        let solve_ns = Num::of(&t["solve_wall_ns"]);
        let sec = solve_ns.f() / 1e9;
        let mut f: Vec<&str> = Vec::new();
        let pl = &iso["placement"];
        if !py_eq(&pl["affinity"], &Value::Array(vec![run.clone()])) {
            f.push("L1 affinity");
        }
        if !py_eq(&pl["cpu_at_start"], run) || !py_eq(&pl["cpu_at_end"], run) {
            f.push("L1 cpu moved");
        }
        let thr = |k: &str| or_num(o, k, 1);
        let mx = if thr("threads_before").cmp(thr("threads_after")) == std::cmp::Ordering::Less {
            thr("threads_after")
        } else {
            thr("threads_before")
        };
        if mx.cmp(Num::Int(1)) == std::cmp::Ordering::Greater {
            f.push("L1 threads");
        }
        if wall_s > 0.0 && cpu_s / wall_s > 1.05 {
            f.push("L1 cpu/wall");
        }
        let it: Vec<Num> = [&iso["before"], &iso["after"]]
            .iter()
            .filter(|x| x.get("intruders").is_some_and(truthy))
            .map(|x| Num::of(&x["intruders"]["user_threads"]))
            .collect();
        let max_it = it.iter().copied().reduce(|m, x| if x.cmp(m) == std::cmp::Ordering::Greater { x } else { m });
        if it.is_empty()
            || max_it.unwrap().cmp(Num::of(&sess["eviction"]["failed_user_threads"])) == std::cmp::Ordering::Greater
        {
            f.push("L1 intruders");
        }
        // L2 gates
        if solve_ns.cmp(Num::Int(1_000_000)) == std::cmp::Ordering::Less {
            f.push("L2 too short");
        }
        let runs = run_key(run);
        let a_ = &iso["before"]["reserved_ticks"][&runs];
        let b_ = &iso["after"]["reserved_ticks"][&runs];
        let busy = Num::of(&b_["busy"]).sub(Num::of(&a_["busy"]));
        let foreign = (busy.f() - cpu_s * hzf).round_ties_even();
        if !Num::Float(foreign).eq(Num::of(&iso["foreign_busy_ticks"])) {
            f.push("foreign ticks differ from record");
        }
        if foreign > py_max_f(2.0, 0.01 * wall_s * hzf) {
            f.push("L2 foreign ticks");
        }
        let st = Num::of(&b_["steal"]).sub(Num::of(&a_["steal"]));
        if st.f() > py_max_f(1.0, 0.01 * wall_s * hzf) {
            f.push("L2 steal");
        }
        let mut sbusy = Num::Int(0);
        for c in observed {
            if !py_eq(c, run) {
                let ck = run_key(c);
                let x0 = iso["before"]["reserved_ticks"].get(&ck);
                let x1 = iso["after"]["reserved_ticks"].get(&ck);
                if let (Some(x0), Some(x1)) = (x0, x1) {
                    if truthy(x0) && truthy(x1) {
                        sbusy = sbusy.add(Num::of(&x1["busy"]).sub(Num::of(&x0["busy"])));
                    }
                }
            }
        }
        if sbusy.f() > py_max_f(2.0, 0.01 * wall_s * hzf) {
            f.push("L2 sibling busy");
        }
        let sch = match o.get("schedstat_all") {
            Some(x) if truthy(x) => Some(x),
            _ => g(t, "schedstat_solve"),
        };
        match sch {
            None => f.push("L2 run-queue"),
            Some(sch) => {
                if Num::of(&sch["run_delay_ns"]).f() > py_max_f(100_000.0, 0.005 * solve_ns.f()) {
                    f.push("L2 run-queue")
                }
            }
        }
        if let Some(inv) = g(o, "involuntary_switches") {
            let c = (Num::of(&th["max_preemptions_per_second"]).f() * sec).ceil();
            if Num::of(inv).f() > py_max_f(c, 1.0) {
                f.push("L2 preemption");
            }
        }
        if let Some(irq) = g(o, "irq") {
            let gi = |k: &str| irq.get(k).map(Num::of).unwrap_or(Num::Int(0));
            let nontick = gi("device").add(gi("ipi_other")).add(gi("softirq_work"));
            if nontick.f() > py_max_f((100.0 * sec).ceil(), 2.0) {
                f.push("L2 irq (approx)");
            }
        }
        if let Some(cs) = g(o, "cpu_sched") {
            let own = match o.get("schedstat_all") {
                Some(x) if truthy(x) => x.get("timeslices").map(Num::of).unwrap_or(Num::Int(0)),
                _ => Num::Int(0),
            };
            let fa = Num::of(&cs["pcount"]).sub(own);
            let fa = if fa.cmp(Num::Int(0)) == std::cmp::Ordering::Less { Num::Int(0) } else { fa };
            let sw = |k: &str| match o.get(k) {
                Some(x) if truthy(x) => Num::of(x),
                _ => Num::Int(0),
            };
            let fns = if sw("involuntary_switches").add(sw("voluntary_switches")).eq(Num::Int(0)) {
                Some(Num::of(&cs["rq_cpu_ns"]))
            } else {
                None
            };
            let long = fns.is_some_and(|x| x.f() > py_max_f(100_000.0, 0.01 * sec * 1e9));
            if fa.cmp(Num::Int(1)) == std::cmp::Ordering::Greater || long {
                f.push("L2 foreign slices");
            }
        }
        let pa = Num::of(&iso["before"]["psi_memory_some_total_us"]);
        let pb = Num::of(&iso["after"]["psi_memory_some_total_us"]);
        if pb.sub(pa).f() / 1e6 > py_max_f(0.005 * py_max_f(wall_s, 0.001), 0.001) {
            f.push("L2 memory stall");
        }
        for x in &f {
            bump(&mut flagged, *x);
        }
        let rec_l2: Vec<Value> = iso["blockers"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| s(x).starts_with("L2: ") || s(x).starts_with("L1: "))
            .cloned()
            .collect();
        let mine_l2 = f.iter().any(|x| x.starts_with("L2") || x.starts_with("L1"));
        if !rec_l2.is_empty() != mine_l2 || f.contains(&"foreign ticks differ from record") {
            let mut d = Obj::new();
            d.set("record_id", r["record_id"].clone())
                .set("seq", r["seq"].clone())
                .set("mine", f.iter().map(|x| Value::from(*x)).collect::<Vec<_>>())
                .set("recorded", rec_l2);
            bad.push(d.val());
        }
    }
    let mut o = Obj::new();
    o.set("ticks_per_second", sess["ticks_per_second"].clone())
        .set("thresholds", th.clone())
        .set("records", records.len() as u64)
        .set("flag_counts", counter_val(&flagged))
        .set("disagreements", bad)
        .set(
            "note",
            "partial re-grade: L1/L2 gates that depend only on record fields; preflight, capsule and eviction gates are session-level and read from session.json",
        );
    o.val()
}

/// Python `max(a, b)` on floats (returns a unless b > a).
fn py_max_f(a: f64, b: f64) -> f64 {
    if b > a {
        b
    } else {
        a
    }
}

/// `str(run)` for a JSON int cpu id.
fn run_key(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => py::repr(v),
    }
}

fn positions(records: &[Value]) -> Value {
    let mut blocks: OMap<v::Key, Vec<&Value>> = OMap::new();
    for r in records {
        blocks.entry_or(v::key(r), Vec::new).push(r);
    }
    let mut pos: OMap<String, BTreeMap<usize, i128>> = OMap::new();
    let mut pred: OMap<String, OMap<String, i128>> = OMap::new();
    let mut orders: OMap<String, i128> = OMap::new();
    let mut consecutive = true;
    let mut sizes: BTreeMap<usize, i128> = BTreeMap::new();
    for bl in blocks.values() {
        let mut bl = bl.clone();
        bl.sort_by_key(|r| int(&r["seq"]));
        *sizes.entry(bl.len()).or_insert(0) += 1;
        let seqs: Vec<i128> = bl.iter().map(|r| int(&r["seq"])).collect();
        consecutive &= seqs == (seqs[0]..seqs[0] + seqs.len() as i128).collect::<Vec<_>>();
        if truthy(&bl[0]["warmup"]) {
            continue;
        }
        bump(&mut orders, bl.iter().map(|r| s(&r["arm"])).collect::<Vec<_>>().join(" "));
        for (i, r) in bl.iter().enumerate() {
            *pos.entry_or(v::arm(r), BTreeMap::new).entry(i).or_insert(0) += 1;
            let p = if i > 0 { s(&bl[i - 1]["arm"]).to_string() } else { "(block start)".to_string() };
            bump(pred.entry_or(v::arm(r), OMap::new), p);
        }
    }
    let mut bs = Obj::new();
    for (k, n) in &sizes {
        bs.set(int_key(*k as i64), py::int_val(*n));
    }
    let mut po = Obj::new();
    for (a, c) in pos.iter() {
        let mut o = Obj::new();
        for (i, n) in c {
            o.set(i.to_string(), py::int_val(*n));
        }
        po.set(a.clone(), o.val());
    }
    let mut pr = Obj::new();
    for (a, c) in pred.iter() {
        pr.set(a.clone(), counter_val(c));
    }
    let mut o = Obj::new();
    o.set("blocks", blocks.len() as u64)
        .set("block_sizes", bs.val())
        .set("blocks_consecutive_seq", consecutive)
        .set("measured_orders", counter_val(&orders))
        .set("positions", po.val())
        .set("predecessor_within_block", pr.val());
    o.val()
}

/// ops A/A: per pair, priced GAE equal at each weight; ratio of sums of priced S.
fn counted_aa(records: &[Value], a: &str, b: &str) -> Value {
    let arms = v::by_arm(records, true);
    let (ma, mb) = (&arms[&a.to_string()], &arms[&b.to_string()]);
    let (ka, kb) = (ma.key_set(), mb.key_set());
    let mut o = Obj::new();
    o.set("pairs", ka.intersection(&kb).count() as u64).set("unpaired", ka.symmetric_difference(&kb).count() as u64);
    let mut all1 = true;
    for (w, ws) in WEIGHTS {
        let eq = ma.keys().all(|k| v::priced_gae(ma[k], w) == v::priced_gae(mb[k], w));
        let sa = py::fsum_py(ma.keys().map(|k| v::priced_gae(ma[k], w) / v::curve_r_sqrt(ma[k])));
        let sb = py::fsum_py(ma.keys().map(|k| v::priced_gae(mb[k], w) / v::curve_r_sqrt(mb[k])));
        let ratio = sb / sa;
        all1 &= eq && ratio == 1.0;
        o.set(format!("w={ws}"), serde_json::json!({"every_pair_equal": eq, "ratio_b_over_a": py::fval(ratio)}));
    }
    o.set("exactly_1_at_all_weights", all1);
    o.val()
}

fn grades_map(g: &Value) -> Map<String, Value> {
    let mut m = Map::new();
    for x in g["grades"].as_array().unwrap() {
        m.insert(s(&x["level"]).to_string(), x["count"].clone());
    }
    m
}

fn omap_val_eq(m: &Map<String, Value>, c: &OMap<String, i128>) -> bool {
    m.len() == c.len() && c.iter().all(|(k, n)| m.get(k).is_some_and(|x| Num::of(x).eq(Num::Int(*n))))
}

fn wall_checks(b: u32) -> Value {
    let recs = v::load_wall(b);
    let mut out = common_file_checks(b, "wall", &recs);
    let eks = v::session_dir(b, "wall");
    let g = v::jload(&eks.join("l2-gate.json"));
    let (lv, mism, pref) = regrade_counts(&recs);
    let je = &out["job_exit"];
    let c1 = Num::of(&je["exit_code"]).eq(Num::Int(0)) && truthy(&je["single_pod_uid"]);
    out.insert("cond1_exit".into(), c1.into());
    let a = &out["audit"];
    let c2 = truthy(a)
        && truthy(&a["ok"])
        && a["session_status"].as_str() == Some("complete")
        && int(&a["problems"]) == 0
        && int(&a["replays_identical"]) == int(&a["measured_records"])
        && int(&a["measured_records"]) == int(&a["replays"])
        && truthy(&a["replay_seqs_are_all_measured"]);
    out.insert("cond2_audit".into(), c2.into());
    let mut caa = Obj::new();
    caa.set("dgb->dgb-aa", counted_aa(&recs, "dgb", "dgb-aa")).set("dgbf-edge->dgbf", counted_aa(&recs, "dgbf-edge", "dgbf"));
    let caa = caa.val();
    let c3 = caa.as_object().unwrap().values().all(|x| truthy(&x["exactly_1_at_all_weights"]));
    out.insert("cond3_counted_aa".into(), caa);
    out.insert("cond3".into(), c3.into());
    let sb = &out["spec_build"];
    let c4 = ["input_spec_json_equal_frozen", "build_info_revision", "build_info_ecbench_sha256", "binary_sha256_file"]
        .iter()
        .all(|k| truthy(&sb[*k]));
    out.insert("cond4".into(), c4.into());
    let gm = grades_map(&g);
    let geq = omap_val_eq(&gm, &lv);
    let mut l2 = Obj::new();
    l2.set("accepted", g["accepted"].clone())
        .set("invalid_outcomes", g["invalid_outcomes"].clone())
        .set("below_l2", g["below_l2"].as_array().unwrap().len() as u64)
        .set("grades", g["grades"].clone())
        .set("executions", g["executions"].clone())
        .set("records_levels", counter_val(&lv))
        .set("gate_grades_equal_record_counts", geq)
        .set("level_from_blockers_mismatches", mism.clone())
        .set("blocker_kinds", counter_val(&pref));
    out.insert("l2_gate".into(), l2.val());
    let c5 = g["accepted"] == Value::Bool(true)
        && g["invalid_outcomes"].as_array().is_some_and(|x| x.is_empty())
        && !truthy(&g["below_l2"])
        && lv.keys().all(|k| k == "L2" || k == "L3")
        && geq
        && mism.is_empty()
        && Num::of(&g["executions"]).eq(Num::Int(recs.len() as i128));
    out.insert("cond5".into(), c5.into());
    out.insert("raw_regrade".into(), raw_regrade_wall(b, &recs));
    let cc = v::jload(&eks.join("control-comparison.json"));
    let w = &cc["wall"];
    let mut c6 = Obj::new();
    c6.set("a", cc["a"]["arm"].clone())
        .set("b", cc["b"]["arm"].clone())
        .set("status", w["status"].clone())
        .set("pairs", w["pairs"].clone())
        .set("pairs_below_level", w["pairs_below_level"].clone())
        .set("aa_pairs", w["aa_pairs"].clone())
        .set("aa_ci95", w["aa_ci95"].clone())
        .set("self_aa", w["self_aa"].clone())
        .set("aa_arms", w["aa_arms"].clone())
        .set("aa_selection", w["aa_selection"].clone())
        .set("ratio_b_over_a", w["ratio_b_over_a"].clone())
        .set("ci95", w["ci95"].clone())
        .set("aa_ratio", w["aa_ratio"].clone())
        .set("corrected_ratio", w["corrected_ratio"].clone())
        .set("corrected_ci95", w["corrected_ci95"].clone())
        .set("corrected_dof", w["corrected_dof"].clone())
        .set("ci_method", w["ci_method"].clone())
        .set("resample_seed", cc["resample_seed"].clone())
        .set("ops_ratio", cc["ops"]["ratio_b_over_a"].clone())
        .set("ops_band", cc["ops"]["band"]["ratio_b_over_a"].clone());
    let c6 = c6.val();
    let aa_arms: BTreeSet<&str> = c6["aa_arms"].as_array().unwrap().iter().map(s).collect();
    let cond6 = c6["a"].as_str() == Some("dgbf-edge")
        && c6["b"].as_str() == Some("dgb-aa")
        && c6["status"].as_str() == Some("admitted")
        && Num::of(&c6["pairs"]).eq(Num::Int(288))
        && Num::of(&c6["pairs_below_level"]).eq(Num::Int(0))
        && Num::of(&c6["aa_pairs"]).eq(Num::Int(288))
        && !c6["aa_ci95"].is_null()
        && c6["self_aa"] == Value::Bool(false)
        && aa_arms == ["dgbf", "dgbf-edge"].into()
        && c6["aa_selection"].as_str() == Some("includes arm A");
    out.insert("cond6_control_comparison".into(), c6);
    out.insert("cond6".into(), cond6.into());
    out.insert("positions".into(), positions(&recs));
    Value::Object(out)
}

const RHO: [&str; 2] = ["rho-neg", "rho-neg-aa"];

fn cond5_ok(level: &str, blockers: &[Value]) -> bool {
    if level == "L2" || level == "L3" {
        return true;
    }
    if level != "L1" {
        return false;
    }
    blockers.iter().filter(|x| s(x).starts_with("L2: ")).all(|x| s(x) == v::TOO_SHORT)
        && !blockers.iter().any(|x| s(x).starts_with("L1: "))
}

fn counted_checks(b: u32) -> Value {
    let recs = v::load_counted(b, "counted");
    let mut out = common_file_checks(b, "counted", &recs);
    let eks = v::session_dir(b, "counted");
    let graw = v::jload(&eks.join("l2-gate.json"));
    let below: Vec<Value> = graw["below_l2"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            serde_json::json!({"seq": x["seq"].clone(), "arm": x["arm"].clone(), "level": x["isolation"]["level"].clone(),
             "blockers": v::mask_blockers(x["isolation"]["blockers"].as_array().unwrap())})
        })
        .collect();
    let (invalid, grades, executions, accepted) =
        (graw["invalid_outcomes"].clone(), graw["grades"].clone(), graw["executions"].clone(), graw["accepted"].clone());
    let gm = grades_map(&graw);
    drop(graw);
    let (lv, mism, _) = regrade_counts(&recs);
    let a = out["audit"].clone();
    let ic: Vec<&Value> = recs.iter().filter(|r| r["method"]["id"].as_str() == Some("ic.pipeline")).collect();
    let rh: Vec<&Value> = recs.iter().filter(|r| RHO.contains(&s(&r["arm"]))).collect();
    let mut rec_below: Vec<i128> = recs
        .iter()
        .filter(|r| !matches!(s(&r["isolation"]["level"]), "L2" | "L3"))
        .map(|r| int(&r["seq"]))
        .collect();
    rec_below.sort();
    let mut gate_below: Vec<i128> = below.iter().map(|x| int(&x["seq"])).collect();
    gate_below.sort();
    let c1 = truthy(&a)
        && truthy(&a["ok"])
        && a["session_status"].as_str() == Some("complete")
        && int(&a["problems"]) == 0
        && int(&a["replays_identical"]) == int(&a["measured_records"])
        && int(&a["measured_records"]) == int(&a["replays"])
        && truthy(&a["replay_seqs_are_all_measured"]);
    let c2aa = counted_aa(&recs, "rho-neg", "rho-neg-aa");
    let sb = &out["spec_build"];
    let c3 = ["input_spec_json_equal_frozen", "build_info_revision", "build_info_ecbench_sha256", "binary_sha256_file"]
        .iter()
        .all(|k| truthy(&sb[*k]));
    let c4 = ic.iter().all(|r| matches!(s(&r["isolation"]["level"]), "L2" | "L3"))
        && !below.iter().any(|x| !RHO.contains(&s(&x["arm"])));
    let c5 = rh.iter().all(|r| cond5_ok(s(&r["isolation"]["level"]), r["isolation"]["blockers"].as_array().unwrap()));
    let c6 = invalid.as_array().is_some_and(|x| x.is_empty());
    let mut per_arm: OMap<String, i128> = OMap::new();
    let mut levels: OMap<String, i128> = OMap::new();
    for x in &below {
        bump(&mut per_arm, s(&x["arm"]));
        bump(&mut levels, s(&x["level"]));
    }
    let all_short = below
        .iter()
        .all(|x| cond5_ok(s(&x["level"]), x["blockers"].as_array().unwrap()) && s(&x["level"]) == "L1");
    let mut bg = Obj::new();
    bg.set("count", below.len() as u64)
        .set("per_arm", counter_val(&per_arm))
        .set("levels", counter_val(&levels))
        .set("all_too_short_only", all_short);
    let below_eq = rec_below == gate_below;
    let c2 = truthy(&c2aa["exactly_1_at_all_weights"]);
    let c16 = c1 && c2 && c3 && c4 && c5 && c6;
    let ec = Num::of(&out["job_exit"]["exit_code"]);
    let e42 = ec.eq(Num::Int(42))
        && truthy(&out["job_exit"]["single_pod_uid"])
        && c16
        && !below.is_empty()
        && below.iter().all(|x| RHO.contains(&s(&x["arm"])))
        && all_short
        && below_eq;
    let mut gt = Obj::new();
    gt.set("cond1_audit", c1)
        .set("cond2_rho_aa", c2aa)
        .set("cond3_spec_build", c3)
        .set("cond4_ic_all_L2_L3", c4)
        .set("cond5_rho", c5)
        .set("cond6_invalid_outcomes_empty", c6)
        .set("below_l2_from_gate", bg.val())
        .set("below_l2_from_records_equal_gate", below_eq)
        .set("records_levels", counter_val(&lv))
        .set("gate_grades_equal_record_counts", omap_val_eq(&gm, &lv))
        .set("level_from_blockers_mismatches", mism)
        .set("accepted", accepted)
        .set("executions", executions)
        .set("cond2", c2)
        .set("conditions_1_to_6", c16)
        .set("EXIT42_CLAUSE", e42)
        .set("EXIT43_CLAUSE", if ec.eq(Num::Int(43)) { Value::Null } else { Value::Bool(false) })
        .set("route_satisfied", (ec.eq(Num::Int(0)) && c16) || e42)
        .set("note", "the solve-time range of the waived rho executions is not reported: it is rho wall time (PROTOCOL 10)");
    let _ = grades;
    out.insert("gate".into(), gt.val());
    Value::Object(out)
}

pub fn run(out: &Path) {
    let mut res = Obj::new();
    for b in [32u32, 24] {
        let r = wall_checks(b);
        println!("P{b}-wall conds {} {} {} {} {} {}", r["cond1_exit"], r["cond2_audit"], r["cond3"], r["cond4"], r["cond5"], r["cond6"]);
        res.set(format!("P{b}-wall"), r);
    }
    for b in [32u32, 24, 28] {
        let r = counted_checks(b);
        println!("P{b}-counted route_satisfied {} EXIT42 {}", r["gate"]["route_satisfied"], r["gate"]["EXIT42_CLAUSE"]);
        res.set(format!("P{b}-counted"), r);
    }
    py::dump(&res.val(), out);
}

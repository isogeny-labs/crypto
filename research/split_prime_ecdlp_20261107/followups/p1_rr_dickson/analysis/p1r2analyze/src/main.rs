//! p1r2analyze: P1 round 2 (PROTOCOL-R2.md). Merges parsed.jsonl files from several runs (later runs
//! override a (cell, system) only when they bring a completed job where the earlier had a cap), takes
//! the max over seeds per (cell, system), and applies the frozen comparison with the lower-bound
//! reading of capped jobs (frozen in PROTOCOL-R2 before data): RR below X decided when RR completed
//! and RR < X (or X's lower bound); not below when both completed and RR >= X; else undecided.
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
#[derive(Clone, Debug)] struct A { d: i64, w: i64, q: i64, ok: bool, n: usize }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = a.last().unwrap();
    fs::create_dir_all(out).unwrap();
    let mut runs: Vec<BTreeMap<(String, String), A>> = vec![];
    for f in &a[1..a.len() - 1] {
        let mut m: BTreeMap<(String, String), A> = BTreeMap::new();
        for line in fs::read_to_string(f).unwrap().lines().filter(|l| !l.is_empty()) {
            let v: Value = serde_json::from_str(line).unwrap();
            let file = v["file"].as_str().unwrap();
            let mut it = file.rsplit('/');
            let base = it.next().unwrap().trim_end_matches(".log");
            let cell = it.next().unwrap().to_string();
            let sys = base.split_once("_s").unwrap().0.to_string();
            let ok = v["status"].as_str() == Some("ok");
            let e = m.entry((cell, sys)).or_insert(A { d: 0, w: 0, q: i64::MAX, ok: true, n: 0 });
            e.d = e.d.max(v["D"].as_i64().unwrap());
            e.w = e.w.max(v["widest_cols"].as_i64().unwrap());
            if ok { e.q = e.q.min(v["quotient_dim"].as_i64().unwrap_or(-1)); }
            e.ok &= ok; e.n += 1;
        }
        runs.push(m);
    }
    let mut merged: BTreeMap<(String, String), (A, usize)> = BTreeMap::new();
    for (i, m) in runs.iter().enumerate() {
        for (k, v) in m {
            match merged.get(k) { Some((old, _)) if old.ok && !v.ok => {}, _ => { merged.insert(k.clone(), (v.clone(), i)); } }
        }
    }
    let mut t = String::from("cell\tsystem\tD\twidest\tqdim_min\tcompleted\tsource_run\n");
    for ((c, s), (v, i)) in &merged { t += &format!("{c}\t{s}\t{}\t{}\t{}\t{}\t{i}\n", v.d, v.w, if v.q == i64::MAX { -1 } else { v.q }, v.ok); }
    fs::write(format!("{out}/p1_r2_cells.tsv"), &t).unwrap();
    let get = |c: &str, s: &str| merged.get(&(c.to_string(), s.to_string())).map(|x| x.0.clone());
    let cmp = |r: &A, x: &A, rv: i64, xv: i64| -> &'static str { if !r.ok { "undecided" } else if rv < xv { "below" } else if x.ok { "not_below" } else { "undecided" } };
    let classify = |c: &str, x: &str| -> Option<String> {
        let (r, o) = (get(c, "rr")?, get(c, x)?);
        let (cd, cw) = (cmp(&r, &o, r.d, o.d), cmp(&r, &o, r.w, o.w));
        // "X below RR" for dominance the other way
        let xd = if o.ok && r.ok && o.d < r.d { "x_below" } else { "" };
        let xw = if o.ok && r.ok && o.w < r.w { "x_below" } else { "" };
        let verdict = if cd == "below" && cw == "below" { "RR dominates" }
            else if xd == "x_below" && xw == "x_below" { "X dominates" }
            else if (cd == "below" && (cw == "not_below" || xw == "x_below")) || (cw == "below" && cd == "not_below") { "trade" }
            else if cd == "undecided" || cw == "undecided" { "undecided" } else { "neither" };
        Some(format!("{c} RR vs {x}: D {}{} vs {}{} ({cd}); widest {} vs {}{} ({cw}) -> {verdict}",
            r.d, if r.ok {""} else {"+"}, o.d, if o.ok {""} else {" (lower bound)"}, r.w, o.w, if o.ok {""} else {" (lower bound)"}))
    };
    let mut d = serde_json::Map::new();
    let k0t = get("K0_m3_t5", "t");
    d.insert("rule1_control_K0_m3t5_T".into(), format!("{:?}", k0t.map(|x| (x.d, x.w, x.ok))).into());
    let qmin = merged.values().filter(|x| x.0.ok).map(|x| x.0.q).min().unwrap();
    d.insert("rule1_min_qdim_completed".into(), qmin.into());
    let mut rr_t = vec![]; let mut rr_f = vec![];
    for pr in ["D24", "D31", "K0"] { for tt in ["t3", "t4", "t5"] {
        let c = format!("{pr}_m3_{tt}");
        if let Some(s) = classify(&c, "t") { rr_t.push(s); }
        if let Some(s) = classify(&c, "f") { rr_f.push(s); }
    } }
    d.insert("rule2_m5t3_tower_free".into(), classify("D24_m5_t3", "t").unwrap_or_default().into());
    d.insert("rule3_m5t4".into(), classify("D24_m5_t4", "t").unwrap_or_else(|| "pending (card 2)".into()).into());
    d.insert("rule4_rr_vs_f".into(), serde_json::to_value(&rr_f).unwrap());
    d.insert("rule5_rr_vs_t_m3".into(), serde_json::to_value(&rr_t).unwrap());
    let js = serde_json::to_string_pretty(&Value::Object(d)).unwrap();
    fs::write(format!("{out}/decision_r2.json"), &js).unwrap();
    println!("{js}");
}

//! p1analyze: applies P1's frozen decision rules (PROTOCOL.md rules 1-6, Addendum A)
//! to one run's parsed.jsonl. Per (cell, system) the decided value is the maximum over
//! its seeds (protocol "Metrics"); a cell counts as completed only if every job has
//! status "ok". Writes a TSV table and decision.json; every number is derived from the run.
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

#[derive(Default, Clone, Debug)]
struct Agg { d: i64, widest: i64, qmin: i64, n: usize, all_ok: bool, statuses: Vec<String>, seeds: Vec<String> }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (run, out) = (&a[1], &a[2]);
    fs::create_dir_all(out).unwrap();
    let mut agg: BTreeMap<(String, String), Agg> = BTreeMap::new();
    for line in fs::read_to_string(format!("{run}/parsed.jsonl")).unwrap().lines().filter(|l| !l.is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        let file = v["file"].as_str().unwrap();
        let mut it = file.rsplit('/');
        let base = it.next().unwrap().trim_end_matches(".log"); // rr_s2
        let cell = it.next().unwrap().to_string();               // D24_m3_t3
        let (sys, seed) = base.split_once("_s").unwrap();
        let e = agg.entry((cell, sys.to_string())).or_insert(Agg { qmin: i64::MAX, all_ok: true, ..Default::default() });
        e.d = e.d.max(v["D"].as_i64().unwrap());
        e.widest = e.widest.max(v["widest_cols"].as_i64().unwrap());
        e.qmin = e.qmin.min(v["quotient_dim"].as_i64().unwrap_or(-1));
        e.n += 1;
        let st = v["status"].as_str().unwrap().to_string();
        if st != "ok" { e.all_ok = false; }
        e.statuses.push(st);
        e.seeds.push(seed.to_string());
    }
    let mut tsv = String::from("cell\tsystem\tseeds\tn_jobs\tD_max\twidest_cols_max\tqdim_min\tstatuses\tcompleted\n");
    for ((c, s), g) in &agg {
        tsv += &format!("{c}\t{s}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", g.seeds.join(","), g.n, g.d, g.widest, g.qmin, g.statuses.join(","), g.all_ok);
    }
    fs::write(format!("{out}/p1_cells.tsv"), &tsv).unwrap();
    let get = |c: &str, s: &str| agg.get(&(c.to_string(), s.to_string())).cloned();
    // Bound-aware comparison. A job capped at the memory or time limit reports the highest
    // degree and the widest matrix F4 had reached: lower bounds on its final values. "RR below T"
    // is decided true when RR completed and RR < T (or < T's lower bound); decided false when both
    // completed and RR >= T; undecided otherwise.
    #[derive(Clone, Copy, PartialEq, Debug)] enum Cmp { Below, NotBelow, Undecided }
    let cmp = |r: &Agg, t: &Agg, rv: i64, tv: i64| -> Cmp {
        if !r.all_ok { return Cmp::Undecided; }
        if rv < tv { Cmp::Below } else if t.all_ok { Cmp::NotBelow } else { Cmp::Undecided }
    };
    let mut d = serde_json::Map::new();
    // rule 1: control. qdim is read only from completed jobs (a capped job has no quotient).
    let rrk = get("K0_m3_t4", "rr");
    let ctl_d = rrk.as_ref().map(|g| g.d).unwrap_or(-1);
    let qmin_done = agg.values().filter(|g| g.all_ok).map(|g| g.qmin).min().unwrap();
    let r1 = (ctl_d - 4).abs() > 1 || qmin_done < 1;
    d.insert("rule1_control_RRK_m3t4_D".into(), ctl_d.into());
    d.insert("rule1_min_quotient_dim_completed_jobs".into(), qmin_done.into());
    d.insert("rule1_control_failure".into(), r1.into());
    let cells: std::collections::BTreeSet<String> = agg.keys().map(|k| k.0.clone()).collect();
    let mut rows = Vec::new(); // (cell, rr, t, cmpD, cmpW)
    for c in cells.iter().filter(|c| c.starts_with('D')) {
        if let (Some(r), Some(t)) = (get(c, "rr"), get(c, "t")) {
            let cd = cmp(&r, &t, r.d, t.d);
            let cw = cmp(&r, &t, r.widest, t.widest);
            rows.push((c.clone(), r, t, cd, cw));
        }
    }
    let desc: Vec<String> = rows.iter().map(|(c, r, t, cd, cw)| format!("{c}: D RR {}{} vs T {}{} -> {:?}; widest RR {} vs T {}{} -> {:?}",
        r.d, if r.all_ok {""} else {"(lb)"}, t.d, if t.all_ok {""} else {" (lower bound)"}, cd, r.widest, t.widest, if t.all_ok {""} else {" (lower bound)"}, cw)).collect();
    d.insert("dickson_cells".into(), serde_json::to_value(&desc).unwrap());
    // rule 2: killed if RR is not below T on D in every comparable cell (decided cells only)
    let decided: Vec<_> = rows.iter().filter(|x| x.3 != Cmp::Undecided).collect();
    let r2 = !decided.is_empty() && decided.iter().all(|x| x.3 == Cmp::NotBelow);
    // rule 3: transfer vs Kummer, both RR arms completed at the same (m, t)
    let mut r3 = false; let mut r3_pairs = Vec::new();
    for (c, r, _, _, _) in &rows {
        let kc = format!("K0_{}", &c[4..]);
        if let Some(k) = get(&kc, "rr") { if r.all_ok && k.all_ok {
            r3_pairs.push(format!("{c}: {} vs {kc}: {}", r.d, k.d));
            if (r.d - k.d).abs() >= 2 { r3 = true; } } }
    }
    // rule 4: trade: D lower somewhere, but RR widest larger in every such cell (decided)
    let lower: Vec<_> = rows.iter().filter(|x| x.3 == Cmp::Below).collect();
    let r4 = !lower.is_empty() && lower.iter().all(|x| x.4 == Cmp::NotBelow);
    // rule 5: some m = 3 (m, t) with RR below T on D and widest at both Dickson primes
    let mut r5_mt = Vec::new();
    for t in ["t3", "t4", "t5"] {
        if ["D24", "D31"].iter().all(|pr| rows.iter().any(|x| x.0 == format!("{pr}_m3_{t}") && x.3 == Cmp::Below && x.4 == Cmp::Below)) { r5_mt.push(format!("m3_{t}")); }
    }
    let r5 = !r5_mt.is_empty();
    // rule 6: m5 t3 at D24
    let m5 = rows.iter().find(|x| x.0 == "D24_m5_t3");
    // Frozen text: decide rule 6 only when both m = 5 arms completed; otherwise "undecided".
    // The bound-aware reading is reported separately, labelled post-data.
    let r6_frozen = match m5 {
        Some(x) if x.1.all_ok && x.2.all_ok => if x.3 == Cmp::Below && x.4 == Cmp::Below && r5 { "met" } else { "not met" },
        _ => "undecided (an m = 5 arm did not complete; frozen text)",
    };
    d.insert("rule6_frozen_text".into(), r6_frozen.into());
    let r6_state = match m5 {
        None => "undecided (cell absent)".to_string(),
        Some(x) if !x.1.all_ok && !x.2.all_ok => "undecided (neither arm completed)".to_string(),
        Some(x) if x.3 == Cmp::Below && x.4 == Cmp::Below && r5 => "met".to_string(),
        Some(x) if x.3 == Cmp::Undecided || x.4 == Cmp::Undecided => format!("undecided (D {:?}, widest {:?})", x.3, x.4),
        _ => "not met".to_string(),
    };
    d.insert("rule2_killed".into(), r2.into());
    d.insert("rule3_pairs".into(), serde_json::to_value(&r3_pairs).unwrap());
    d.insert("rule3_transfer_fails".into(), r3.into());
    d.insert("rule4_trade".into(), r4.into());
    d.insert("rule5_cells".into(), serde_json::to_value(&r5_mt).unwrap());
    d.insert("rule5_stage_positive_m3".into(), r5.into());
    d.insert("rule6_post_data_bound_reading".into(), r6_state.clone().into());
    let verdict = if r1 { "1: control failure (P1 moot)".to_string() } else if r2 { "2: killed".into() } else if r3 { "3: transfer fails".into() }
        else if r4 { "4: trade, not a gain".into() } else if r6_frozen == "met" { "6: repository criterion met (algebraic track, stage level)".into() }
        else if r5 { format!("5: stage-level positive at m = 3 (rule 6: {r6_frozen}; post-data bound reading: {r6_state})") } else { "none of 1-6".into() };
    d.insert("verdict".into(), verdict.into());
    let js = serde_json::to_string_pretty(&Value::Object(d)).unwrap();
    fs::write(format!("{out}/decision.json"), &js).unwrap();
    println!("{js}");
}

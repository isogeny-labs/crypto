//! p2r2analyze: applies P2 round 2's frozen rules (PROTOCOL-R2.md) to a round-2 m3.jsonl and compares
//! with round 1's planted m3.jsonl. Rule 1: per (prime, m, t), selected vs random arm mean F4 work, D and
//! widest within 10 %. Every number is derived from the recorded outputs.
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
fn load(p: &str) -> Vec<Value> { fs::read_to_string(p).unwrap().lines().filter(|l| !l.is_empty()).map(|l| serde_json::from_str(l).unwrap()).collect() }
type G = BTreeMap<String, BTreeMap<String, Vec<(f64, f64, f64, f64)>>>;
fn group(rows: &[Value]) -> (G, BTreeMap<String, (usize, usize)>) {
    let mut g: G = BTreeMap::new();
    let mut dec: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for r in rows {
        let id = r["id"].as_str().unwrap();
        let Some((stem, kind)) = id.rsplit_once('.') else { continue };
        let p: Vec<&str> = stem.split('-').collect(); // kind p m t arm ts
        let key = p[..4].join("-");
        let arm = p[4].to_string();
        let o = &r["out"];
        if kind == "parse" {
            g.entry(key).or_default().entry(arm).or_default().push((o["sum_rows_x_cols"].as_f64().unwrap(), o["D_max_deg"].as_f64().unwrap(), o["widest_cols"].as_f64().unwrap(), o["quotient_dim"].as_f64().unwrap_or(-1.0)));
        } else if kind == "emit" {
            let e = dec.entry(key).or_default();
            e.1 += 1;
            if o.get("decomposes").and_then(|x| x.as_bool()) == Some(true) { e.0 += 1; }
        }
    }
    (g, dec)
}
fn mean(v: &[(f64, f64, f64, f64)], i: usize) -> f64 { v.iter().map(|x| [x.0, x.1, x.2, x.3][i]).sum::<f64>() / v.len() as f64 }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (r2, r1, out) = (&a[1], &a[2], &a[3]);
    fs::create_dir_all(out).unwrap();
    let (g2, dec) = group(&load(r2));
    let (g1, _) = group(&load(r1));
    let mut t = String::from("cell\tarm\tmean_work_unplanted\tmean_D\tmean_widest\tqdim_mean\twork_rel_vs_idx0\tD_rel\twidest_rel\tplanted_work_round1\tunplanted_over_planted\n");
    let (mut max_sel, mut max_aa) = (0.0f64, 0.0f64);
    let mut fails = 0;
    for (k, arms) in &g2 {
        let (w0, d0, c0) = (mean(&arms["idx0"], 0), mean(&arms["idx0"], 1), mean(&arms["idx0"], 2));
        for arm in ["idx0", "idx1", "max"] {
            let v = &arms[arm];
            let (w, d, c, q) = (mean(v, 0), mean(v, 1), mean(v, 2), mean(v, 3));
            let (rw, rd, rc) = (w / w0 - 1.0, d / d0 - 1.0, c / c0 - 1.0);
            let worst = rw.abs().max(rd.abs()).max(rc.abs());
            if arm == "max" { max_sel = max_sel.max(worst); if worst > 0.10 { fails += 1; } }
            if arm == "idx1" { max_aa = max_aa.max(worst); }
            let pw = g1.get(k).and_then(|x| x.get(arm)).map(|v| mean(v, 0)).unwrap_or(f64::NAN);
            t += &format!("{k}\t{arm}\t{w:.1}\t{d:.3}\t{c:.1}\t{q:.3}\t{rw:+.4}\t{rd:+.4}\t{rc:+.4}\t{pw:.1}\t{:.4}\n", w / pw);
        }
    }
    fs::write(format!("{out}/r2_unplanted_f4.tsv"), &t).unwrap();
    let decs: Vec<String> = dec.iter().filter(|(_, v)| v.0 > 0).map(|(k, v)| format!("{k}: {}/{}", v.0, v.1)).collect();
    let total: usize = dec.values().map(|v| v.0).sum();
    let mut d = serde_json::Map::new();
    d.insert("groups".into(), g2.len().into());
    d.insert("max_abs_rel_dev_selected_vs_random".into(), max_sel.into());
    d.insert("max_abs_rel_dev_AA".into(), max_aa.into());
    d.insert("groups_beyond_10pct".into(), fails.into());
    d.insert("targets_that_decompose".into(), total.into());
    d.insert("decomposing_by_group".into(), serde_json::to_value(&decs).unwrap());
    d.insert("rule1_coset_independence_unplanted".into(), (if fails == 0 { "holds" } else { "fails" }).into());
    let js = serde_json::to_string_pretty(&Value::Object(d)).unwrap();
    fs::write(format!("{out}/decision_r2.json"), &js).unwrap();
    println!("{js}");
}

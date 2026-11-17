//! p2analyze: applies the frozen decision rules of P2 (PROTOCOL.md, Addendum A,
//! Clarification A.1) to the raw JSONL of one run. Reads m1..m4.jsonl, writes
//! derived tables (TSV) and a decision JSON. Every number it prints is derived
//! from the recorded outputs and labelled so.
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

fn load(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read {path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json line"))
        .collect()
}
fn f(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or_else(|| panic!("missing {k} in {v}"))
}
fn s(v: &Value, k: &str) -> String {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run = &args[1];
    let out = &args[2];
    fs::create_dir_all(out).unwrap();
    let mut decision = serde_json::Map::new();

    // ---- M1: order statistic
    let m1 = load(&format!("{run}/m1.jsonl"));
    let mut t1 = String::from("kind\tp\tcurve\tt\tC\tc_mean\tc_max\tratio_obs\tratio_exact\trel_dev\twithin_10pct\n");
    let (mut m1_n, mut m1_ok) = (0, 0);
    for r in m1.iter().filter(|r| s(r, "id").starts_with("cosets-")) {
        let o = &r["out"];
        let id = s(r, "id");
        let curve = id.rsplit('-').nth(1).unwrap().to_string();
        let obs = f(o, "ratio_sel_over_mean");
        let ex = f(o, "exact_ratio_q_emp");
        let dev = obs / ex - 1.0;
        let ok = dev.abs() <= 0.10;
        m1_n += 1;
        if ok { m1_ok += 1; }
        t1 += &format!("{}\t{}\t{}\t{}\t{}\t{:.4}\t{}\t{:.4}\t{:.4}\t{:+.4}\t{}\n",
            s(o, "kind"), f(o, "p"), curve, f(o, "t"), f(o, "C"), f(o, "c_mean"), f(o, "c_max"), obs, ex, dev, ok);
    }
    fs::write(format!("{out}/m1_order_statistic.tsv"), &t1).unwrap();
    decision.insert("M1_cells".into(), m1_n.into());
    decision.insert("M1_within_10pct".into(), m1_ok.into());

    // ---- M2: coverage law
    let m2 = load(&format!("{run}/m2.jsonl"));
    // key: kind-p-c-t -> arm -> (c_E, cov2, distinct, nominal)
    let mut cov: BTreeMap<String, BTreeMap<String, (f64, f64, f64, f64)>> = BTreeMap::new();
    for r in &m2 {
        let id = s(r, "id"); // cov-<kind>-<p>-c<i>-t<t>-<arm>
        let parts: Vec<&str> = id.split('-').collect();
        let key = parts[1..5].join("-");
        let arm = parts[5].to_string();
        let o = &r["out"];
        cov.entry(key).or_default().insert(arm,
            (f(o, "c_E"), f(o, "m2_coverage"), f(o, "m2_distinct_nonzero_R"), f(o, "m2_nominal_2cE2")));
    }
    let mut t2 = String::from("cell\tcE_idx0\tcE_idx1\tcE_max\tcov_idx0\tcov_max\tratio_obs\tratio_pred\trel_dev\toff_20pct\tAA_ratio_obs\tAA_ratio_pred\tdistinct_eq_nominal_all_arms\n");
    let mut off = 0;
    let mut all_nominal = true;
    for (k, arms) in &cov {
        let a0 = arms["idx0"];
        let a1 = arms["idx1"];
        let am = arms["max"];
        let obs = am.1 / a0.1;
        let pred = (am.0 / a0.0).powi(2);
        let dev = obs / pred - 1.0;
        let o20 = dev.abs() > 0.20;
        if o20 { off += 1; }
        let nominal = [a0, a1, am].iter().all(|x| x.2 == x.3);
        all_nominal &= nominal;
        t2 += &format!("{k}\t{}\t{}\t{}\t{:.6e}\t{:.6e}\t{:.4}\t{:.4}\t{:+.4}\t{}\t{:.4}\t{:.4}\t{}\n",
            a0.0, a1.0, am.0, a0.1, am.1, obs, pred, dev, o20, a1.1 / a0.1, (a1.0 / a0.0).powi(2), nominal);
    }
    fs::write(format!("{out}/m2_coverage_law.tsv"), &t2).unwrap();
    decision.insert("M2_cells".into(), cov.len().into());
    decision.insert("M2_cells_off_20pct".into(), off.into());
    decision.insert("M2_distinct_equals_2cE2_everywhere".into(), all_nominal.into());

    // ---- M3: F4 cost per coset
    let m3 = load(&format!("{run}/m3.jsonl"));
    // group key kind-p-m-t ; arm ; list of (work, D, widest, qdim, cE)
    let mut ce: BTreeMap<String, f64> = BTreeMap::new();
    for r in m3.iter().filter(|r| s(r, "id").ends_with(".emit")) {
        let id = s(r, "id");
        let stem = id.trim_end_matches(".emit");
        let parts: Vec<&str> = stem.split('-').collect(); // kind p m t arm ts
        ce.insert(parts[..5].join("-"), f(&r["out"], "c_E"));
    }
    let mut grp: BTreeMap<String, BTreeMap<String, Vec<(f64, f64, f64, f64)>>> = BTreeMap::new();
    for r in m3.iter().filter(|r| s(r, "id").ends_with(".parse")) {
        let id = s(r, "id");
        let stem = id.trim_end_matches(".parse");
        let parts: Vec<&str> = stem.split('-').collect();
        let key = parts[..4].join("-");
        let arm = parts[4].to_string();
        let o = &r["out"];
        grp.entry(key).or_default().entry(arm).or_default().push(
            (f(o, "sum_rows_x_cols"), f(o, "D_max_deg"), f(o, "widest_cols"), f(o, "quotient_dim")));
    }
    let mean = |v: &Vec<(f64, f64, f64, f64)>, i: usize| -> f64 {
        v.iter().map(|x| [x.0, x.1, x.2, x.3][i]).sum::<f64>() / v.len() as f64
    };
    let mut t3 = String::from("cell\tarm\tn_targets\tc_E\tmean_work\tmean_D\tmean_widest\tmean_qdim\twork_rel_vs_idx0\tD_rel_vs_idx0\twidest_rel_vs_idx0\n");
    let mut max_dev: f64 = 0.0;
    let mut max_dev_aa: f64 = 0.0;
    let mut kill = false;
    for (k, arms) in &grp {
        let w0 = mean(&arms["idx0"], 0);
        let d0 = mean(&arms["idx0"], 1);
        let c0 = mean(&arms["idx0"], 2);
        for arm in ["idx0", "idx1", "max"] {
            let v = &arms[arm];
            let (w, d, c, q) = (mean(v, 0), mean(v, 1), mean(v, 2), mean(v, 3));
            let (rw, rd, rc) = (w / w0 - 1.0, d / d0 - 1.0, c / c0 - 1.0);
            let worst = rw.abs().max(rd.abs()).max(rc.abs());
            if arm == "max" { max_dev = max_dev.max(worst); if worst > 0.10 { kill = true; } }
            if arm == "idx1" { max_dev_aa = max_dev_aa.max(worst); }
            let cek = format!("{k}-{arm}");
            t3 += &format!("{k}\t{arm}\t{}\t{}\t{:.1}\t{:.3}\t{:.1}\t{:.3}\t{:+.4}\t{:+.4}\t{:+.4}\n",
                v.len(), ce.get(&cek).copied().unwrap_or(f64::NAN), w, d, c, q, rw, rd, rc);
        }
    }
    fs::write(format!("{out}/m3_f4_cost.tsv"), &t3).unwrap();
    decision.insert("M3_groups".into(), grp.len().into());
    decision.insert("M3_max_abs_rel_dev_selected_vs_random".into(), max_dev.into());
    decision.insert("M3_max_abs_rel_dev_AA".into(), max_dev_aa.into());

    // ---- M4: purity
    let m4 = load(&format!("{run}/m4.jsonl"));
    let mut t4 = String::from("p\tm\tcurve\tE_target_solutions\tnontrivial_violations\ttrivial_cancelling_pairs\n");
    let mut viol = 0.0;
    for r in &m4 {
        let o = &r["out"];
        let v = f(o, "E_target_with_twist_summand_nontrivial");
        viol += v;
        t4 += &format!("{}\t{}\t{}\t{}\t{}\t{}\n", f(o, "p"), f(o, "m"), f(&o["curve"], "cseed"),
            f(o, "E_target_solutions_total"), v, f(o, "E_target_with_twist_summand_trivial_cancelling_pair"));
    }
    fs::write(format!("{out}/m4_purity.tsv"), &t4).unwrap();
    decision.insert("M4_runs".into(), m4.len().into());
    decision.insert("M4_nontrivial_violations".into(), viol.into());

    // ---- Rule 4 inputs: gain G = (cE_max / cE_idx0)^m at t = 4, curve 0, from M3's emitted c_E; T* derived
    let mut t5 = String::from("cell\tm\tcE_random\tcE_selected\tG=(cE_sel/cE_rand)^m\tC\tt\tlog2p\tselection_cost_fieldmul\tW_F4_mean_random\tcov_random_m2\tcov_selected_m2\tT_star_relations\n");
    let mut gains = Vec::new();
    for (k, _) in &grp {
        let parts: Vec<&str> = k.split('-').collect(); // kind p m t
        let (kind, p, m, t) = (parts[0], parts[1], parts[2], parts[3]);
        if t != "t4" { continue; }
        let m: i32 = m[1..].parse().unwrap();
        let cr = ce[&format!("{k}-idx0")];
        let cs = ce[&format!("{k}-max")];
        let g = (cs / cr).powi(m);
        gains.push(g);
        let pf: f64 = p.parse().unwrap();
        let ckey = format!("{kind}-{p}-c0-t4");
        let c_n = m1.iter().find(|r| s(r, "id") == format!("cosets-{ckey}")).map(|r| f(&r["out"], "C")).unwrap();
        let sel_cost = c_n * 16.0 * pf.log2();
        let w = mean(&grp[k]["idx0"], 0);
        let (cr2, cs2) = (cov[&ckey]["idx0"].1, cov[&ckey]["max"].1);
        // expected F4 work per relation = W / coverage (each F4 solve on a random target succeeds with
        // probability ~ coverage); saving per relation = W (1/cov_r - 1/cov_s); T* = selection cost / saving.
        let saving = w * (1.0 / cr2 - 1.0 / cs2);
        let tstar = if saving > 0.0 { sel_cost / saving } else { f64::INFINITY };
        t5 += &format!("{k}\t{m}\t{cr}\t{cs}\t{g:.4}\t{c_n}\t4\t{:.2}\t{sel_cost:.4e}\t{w:.1}\t{cr2:.4e}\t{cs2:.4e}\t{tstar:.4e}\n", pf.log2());
    }
    fs::write(format!("{out}/rule4_gain_and_tstar.tsv"), &t5).unwrap();
    let gmin = gains.iter().cloned().fold(f64::INFINITY, f64::min);
    decision.insert("rule4_min_gain_t4".into(), gmin.into());

    // ---- decisions in the frozen order
    let r1 = viol > 0.0;
    let r2 = kill;
    let r3 = off > 1;
    let r4 = !r1 && !r2 && !r3 && gmin >= 1.2;
    let verdict = if r1 { "1: lemma application fails" } else if r2 { "2: killed (twist columns not inert)" }
        else if r3 { "3: coverage law fails" } else if r4 { "4: toy-scale gain confirmed" } else { "none of 1-4 (gain below 1.2 at t = 4)" };
    decision.insert("rule1_violation".into(), r1.into());
    decision.insert("rule2_kill".into(), r2.into());
    decision.insert("rule3_coverage_law_fails".into(), r3.into());
    decision.insert("rule4_gain_confirmed".into(), r4.into());
    decision.insert("verdict".into(), verdict.into());
    decision.insert("note".into(), "every value is derived from the run's raw JSONL by p2analyze; see the TSV files".into());
    let js = serde_json::to_string_pretty(&Value::Object(decision)).unwrap();
    fs::write(format!("{out}/decision.json"), &js).unwrap();
    println!("{js}");
}

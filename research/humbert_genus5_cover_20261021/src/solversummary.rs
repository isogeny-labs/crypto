//! `humbert summarize-solver <out-dir> <file.jsonl>...`: tables and verdicts of
//! the linear-algebra campaign (la_solver_20261116/PROTOCOL.md). Records that
//! carry `la_solver` are the campaign (v5); records with `dlp_mode` but no
//! `la_solver` are the dlp_filter_20261115 campaign (v4) that C0 replays against.
//! Duplicate (q, seed, tag) records within one version are an error.
use crate::lpsummary::Rec;
use std::collections::BTreeMap;
use std::fmt::Write as _;

const TAGS: [&str; 8] = ["pre_l1", "lp50_w2", "dlpf50_w2", "dlpf50_w1", "dlpf50_l1", "lp50_l1", "dlpx50_l1", "dlpx50_l1_aa"];
const ARMS: [&str; 7] = ["pre_l1", "lp50_w2", "dlpf50_w2", "dlpf50_w1", "dlpf50_l1", "lp50_l1", "dlpx50_l1"];

fn med(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}
fn lo(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}
fn hi(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

type Inst = BTreeMap<(u64, u64), BTreeMap<String, Rec>>;

pub fn run(out: &str, inputs: &[String]) -> Result<(), String> {
    let mut v5: Inst = BTreeMap::new();
    let mut v4: Inst = BTreeMap::new();
    let mut n5 = 0;
    for path in inputs {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
            let r = Rec::parse(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?;
            let is5 = line.contains("\"la_solver\":");
            if !is5 && !line.contains("\"dlp_mode\":") {
                continue;
            }
            let key = (r.u("p"), r.u("seed"));
            let tag = r.s("tag");
            let slot = if is5 { v5.entry(key).or_default() } else { v4.entry(key).or_default() };
            if slot.contains_key(&tag) {
                return Err(format!("{path}:{}: duplicate record q = {}, seed = {}, tag = {tag}", n + 1, key.0, key.1));
            }
            slot.insert(tag, r);
            n5 += is5 as usize;
        }
    }
    let gates = v5.values().flat_map(|a| a.values()).all(|r| {
        ["solved", "e2e_ok", "pushforward_8P", "T_P_nonzero", "iota_pairs_constant", "x_inf_eq_4c"].iter().all(|k| r.b(k))
    });
    let one_solve: Vec<&Rec> = v5.values().flat_map(|a| a.values()).filter(|r| r.s("la_solver") != "w2").collect();
    let s3_fail = one_solve.iter().filter(|r| !(r.b("e2e_ok") && r.b("solved"))).count();
    let full: Vec<_> = v5.iter().filter(|(_, a)| TAGS.iter().all(|t| a.contains_key(*t))).collect();
    let mut md = format!("v5 runs: {n5}; complete instances: {}; all correctness gates (incl. e2e x P = Q) pass: {gates}\n\n", full.len());

    // ---- C0 -------------------------------------------------------------------
    let (mut rep_ok, mut rep_n) = (0, 0);
    let mut rep_bad = Vec::new();
    let mut same_rel = true;
    let mut aa = true;
    for ((q, s), a) in &full {
        if let Some(old) = v4.get(&(*q, *s)) {
            for (new, oldtag) in [("lp50_w2", "lp50"), ("dlpf50_w2", "dlpf50")] {
                if let Some(o) = old.get(oldtag) {
                    rep_n += 1;
                    if ["rel_candidates", "la_mulmods", "x_recovered"].iter().all(|k| o.s(k) == a[new].s(k)) {
                        rep_ok += 1;
                    } else {
                        rep_bad.push(format!("q {q} seed {s} {new}"));
                    }
                }
            }
        }
        for t in ["dlpf50_w1", "dlpf50_l1"] {
            same_rel &= a[t].s("rel_candidates") == a["dlpf50_w2"].s("rel_candidates");
        }
        same_rel &= a["lp50_l1"].s("rel_candidates") == a["lp50_w2"].s("rel_candidates");
        for k in ["rel_candidates", "la_cols", "la_nnz", "la_mulmods", "filter_merges", "filter_cliques", "x_recovered", "lp_known"] {
            aa &= a["dlpx50_l1"].s(k) == a["dlpx50_l1_aa"].s(k);
        }
    }
    let _ = writeln!(md, "C0: replays of v4 (rel_candidates, la_mulmods, x_recovered) {rep_ok}/{rep_n} exact{}; same candidates across solvers on every instance: {same_rel}; dlpx A/A counted fields identical: {aa}\n",
        if rep_bad.is_empty() { String::new() } else { format!(" (differ: {})", rep_bad.join(", ")) });
    let _ = writeln!(md, "S3: one-solve runs (w1, l1): {}; e2e or solve failures: {s3_fail}{}\n", one_solve.len(),
        if s3_fail == 0 { format!(" (rule of three: failure rate < {:.3} at 95 %)", 3.0 / one_solve.len() as f64) } else { String::new() });

    // ---- per instance ---------------------------------------------------------
    let t = |a: &BTreeMap<String, Rec>, tag: &str, k: &str| a[tag].f(k);
    let rexp = |a: &BTreeMap<String, Rec>| {
        let b = &a["pre_l1"];
        b.f("rho_expected_steps") * b.f("rho_seconds") / b.f("rho_steps")
    };
    // columns of each instance row
    let names = [
        "mm_w1_over_w2", "mm_l1_over_w1", "mm_lpl1_over_lpw2", "tla_l1_over_w2", "mm_x_over_f", "t_f_over_x",
        "t_lpl1_over_fl1", "aa_x", "t_lpw2_over_fl1", "t_fw2_over_fl1",
        "gap_pre_l1", "gap_lp50_w2", "gap_dlpf50_w2", "gap_dlpf50_w1", "gap_dlpf50_l1", "gap_lp50_l1", "gap_dlpx50_l1",
    ];
    md += "| q | seed | mm w1/w2 | mm l1/w1 (dlpf) | mm lp l1/w2 | t_LA dlpf l1/w2 | mm dlpx/dlpf (l1) | t_ic dlpf/dlpx (l1) | t_ic lp/dlpf (l1) | A/A dlpx | IC/rho-exp: dlpf_w2, lp_l1, dlpf_l1, dlpx_l1 |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut inst_csv = format!("q,seed,{}\n", names.join(","));
    let mut perq: BTreeMap<u64, Vec<Vec<f64>>> = BTreeMap::new();
    let (mut s1, mut s2, mut x1) = (true, true, true);
    for ((q, s), a) in &full {
        let re = rexp(a);
        let mm = |tag: &str| t(a, tag, "la_mulmods");
        let ti = |tag: &str| t(a, tag, "t_ic_total");
        let mut row = vec![
            mm("dlpf50_w1") / mm("dlpf50_w2"),
            mm("dlpf50_l1") / mm("dlpf50_w1"),
            mm("lp50_l1") / mm("lp50_w2"),
            t(a, "dlpf50_l1", "t_linear_algebra") / t(a, "dlpf50_w2", "t_linear_algebra"),
            mm("dlpx50_l1") / mm("dlpf50_l1"),
            ti("dlpf50_l1") / ti("dlpx50_l1"),
            ti("lp50_l1") / ti("dlpf50_l1"),
            ti("dlpx50_l1") / ti("dlpx50_l1_aa"),
            ti("lp50_w2") / ti("dlpf50_l1"),
            ti("dlpf50_w2") / ti("dlpf50_l1"),
        ];
        for tag in ARMS {
            row.push(ti(tag) / re);
        }
        s1 &= (0.45..=0.55).contains(&row[0]);
        s2 &= row[1] <= 0.50 && row[2] <= 0.25;
        x1 &= row[4] < 1.0;
        let _ = writeln!(md, "| {q} | {s} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.2} | {:.2} | {:.3} | {:.2}, {:.2}, {:.2}, {:.2} |",
            row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[12], row[15], row[14], row[16]);
        let _ = writeln!(inst_csv, "{q},{s},{}", row.iter().map(|x| format!("{x:.4}")).collect::<Vec<_>>().join(","));
        perq.entry(*q).or_default().push(row);
    }

    // ---- per q ----------------------------------------------------------------
    md += "\n| q | n | t_LA dlpf l1/w2 median | mm dlpx/dlpf median [max] | dlpx faster than dlpf (l1) | dlpf faster than lp (l1) | A/A dlpx median [min, max] | IC/rho-exp median: pre_l1, lp_w2, dlpf_w2, dlpf_w1, dlpf_l1, lp_l1, dlpx_l1 |\n|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut q_csv = format!("q,n,tla_l1_over_w2_med,mm_x_over_f_med,mm_x_over_f_max,x_faster,f_faster_than_lp_l1,aa_med,aa_lo,aa_hi,{}\n",
        ARMS.iter().map(|a| format!("gap_{a}")).collect::<Vec<_>>().join(","));
    let (mut s4, mut x2, mut g1, mut g2) = (true, true, true, true);
    let mut det = BTreeMap::<&str, Vec<String>>::new();
    for (q, v) in &perq {
        let c = |i: usize| -> Vec<f64> { v.iter().map(|r| r[i]).collect() };
        let n = v.len();
        let xf = c(5).iter().filter(|x| **x > 1.0).count();
        let ff = c(6).iter().filter(|x| **x > 1.0).count();
        let gaps: Vec<f64> = (10..17).map(|i| med(&c(i))).collect();
        s4 &= med(&c(3)) <= 0.35;
        det.entry("S4").or_default().push(format!("q {q}: {:.3}", med(&c(3))));
        if *q >= 24001 {
            x2 &= xf >= 4;
            det.entry("X2").or_default().push(format!("q {q}: {xf}/{n}"));
            let best_l1 = [gaps[0], gaps[4], gaps[5], gaps[6]].iter().cloned().fold(f64::INFINITY, f64::min);
            g2 &= best_l1 < 1.0;
            det.entry("G2").or_default().push(format!("q {q}: {best_l1:.2}"));
        }
        g1 &= ff >= 4;
        det.entry("G1").or_default().push(format!("q {q}: {ff}/{n}"));
        let _ = writeln!(md, "| {q} | {n} | {:.3} | {:.3} [{:.3}] | {xf}/{n} | {ff}/{n} | {:.3} [{:.3}, {:.3}] | {} |",
            med(&c(3)), med(&c(4)), hi(&c(4)), med(&c(7)), lo(&c(7)), hi(&c(7)),
            gaps.iter().map(|g| format!("{g:.2}")).collect::<Vec<_>>().join(", "));
        let _ = writeln!(q_csv, "{q},{n},{:.4},{:.4},{:.4},{xf},{ff},{:.4},{:.4},{:.4},{}",
            med(&c(3)), med(&c(4)), hi(&c(4)), med(&c(7)), lo(&c(7)), hi(&c(7)),
            gaps.iter().map(|g| format!("{g:.4}")).collect::<Vec<_>>().join(","));
    }

    // ---- phases ---------------------------------------------------------------
    let mut ph_csv = String::from("q,tag,t_relations,t_filter,t_linear_algebra,t_descent,t_ic,candidates,la_cols,la_nnz,la_mulmods,cliques\n");
    md += "\n| q | arm | t_rel | t_filter | t_LA | t_ic | candidates | LA cols | LA nnz | LA mulmods | cliques |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut slope_pts: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
    for q in perq.keys() {
        for tag in ARMS {
            let rs: Vec<&Rec> = full.iter().filter(|((qq, _), _)| qq == q).map(|(_, a)| &a[tag]).collect();
            let m = |k: &str| med(&rs.iter().map(|r| r.f(k)).collect::<Vec<_>>());
            let cand = med(&rs.iter().map(|r| r.f("rel_candidates") + r.f("quad_candidates")).collect::<Vec<_>>());
            let _ = writeln!(md, "| {q} | {tag} | {:.2} | {:.3} | {:.2} | {:.2} | {cand:.0} | {:.0} | {:.0} | {:.3e} | {:.0} |",
                m("t_relations"), m("t_filter"), m("t_linear_algebra"), m("t_ic_total"), m("la_cols"), m("la_nnz"), m("la_mulmods"), m("filter_cliques"));
            let _ = writeln!(ph_csv, "{q},{tag},{:.4},{:.4},{:.4},{:.4},{:.4},{cand:.0},{:.0},{:.0},{:.0},{:.0}",
                m("t_relations"), m("t_filter"), m("t_linear_algebra"), m("t_descent"), m("t_ic_total"), m("la_cols"), m("la_nnz"), m("la_mulmods"), m("filter_cliques"));
            slope_pts.entry(tag).or_default().push((*q as f64, m("t_ic_total")));
        }
    }
    md += "\nF5 (exploratory): log-log slope of median t_ic, smallest to largest q:";
    for (tag, pts) in &slope_pts {
        if pts.len() >= 2 {
            let (a, b) = (pts[0], pts[pts.len() - 1]);
            let _ = write!(md, " {tag} {:.2};", (b.1 / a.1).ln() / (b.0 / a.0).ln());
        }
    }
    let v = |b: bool| if b { "holds" } else { "FALSIFIED" };
    let d = |k: &str| det.get(k).map(|x| x.join("; ")).unwrap_or_default();
    let _ = writeln!(md, "\n\nVerdicts (PROTOCOL.md): S1 w1/w2 mulmods in [0.45, 0.55] on every instance: {}; S2 l1/w1 <= 0.50 and lp l1/w2 <= 0.25 on every instance: {}; S3 no one-solve failure: {}; S4 median t_LA dlpf l1/w2 <= 0.35 at every q: {} ({}); X1 dlpx < dlpf mulmods (l1) on every instance: {}; X2 dlpx faster than dlpf (l1) on >= 4/5 at q >= 24001: {} ({}); G1 dlpf faster than lp (l1) on >= 4/5 at every q: {} ({}); G2 best l1 median IC/rho-exp < 1 at q >= 24001: {} ({})",
        v(s1), v(s2), v(s3_fail == 0), v(s4), d("S4"), v(x1), v(x2), d("X2"), v(g1), d("G1"), v(g2), d("G2"));

    let dir = std::path::Path::new(out);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v5.md"), &md).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("instances_v5.csv"), &inst_csv).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v5.csv"), &q_csv).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("phases_v5.csv"), &ph_csv).map_err(|e| e.to_string())?;
    print!("{md}");
    Ok(())
}

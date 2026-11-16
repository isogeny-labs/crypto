//! `humbert summarize-filter <out-dir> <file.jsonl>...`: tables and verdicts of
//! the filtered double-large-prime campaign (dlp_filter_20261115/PROTOCOL.md).
//! Files whose records carry `dlp_mode` are the campaign (v4); the others are
//! the dlp_20261113 records (v3) that control C0 replays against.
//! Duplicate (q, seed, tag) records within one version are an error.
use crate::lpsummary::Rec;
use std::collections::BTreeMap;
use std::fmt::Write as _;

const TAGS: [&str; 6] = ["pre", "lp50", "dlp50", "dlpm50", "dlpf50", "dlpf50_aa"];
const ARMS: [&str; 5] = ["pre", "lp50", "dlp50", "dlpm50", "dlpf50"];

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
    let mut v4: Inst = BTreeMap::new();
    let mut v3: Inst = BTreeMap::new();
    let mut n4 = 0;
    for path in inputs {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
            let r = Rec::parse(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?;
            let is4 = line.contains("\"dlp_mode\":");
            let key = (r.u("p"), r.u("seed"));
            let tag = r.s("tag");
            let slot = if is4 { v4.entry(key).or_default() } else { v3.entry(key).or_default() };
            if slot.contains_key(&tag) {
                return Err(format!("{path}:{}: duplicate record q = {}, seed = {}, tag = {tag}", n + 1, key.0, key.1));
            }
            slot.insert(tag, r);
            n4 += is4 as usize;
        }
    }
    let gates = v4.values().flat_map(|a| a.values()).all(|r| {
        ["solved", "pushforward_8P", "T_P_nonzero", "iota_pairs_constant", "x_inf_eq_4c"].iter().all(|k| r.b(k))
    });
    let full: Vec<_> = v4.iter().filter(|(_, a)| TAGS.iter().all(|t| a.contains_key(*t))).collect();
    let mut md = format!("v4 runs: {n4}; complete instances: {}; all correctness gates pass: {gates}\n\n", full.len());

    // ---- C0 controls ----------------------------------------------------------
    let mut c0_replay = (0, 0);
    let mut c0_replay_bad = Vec::new();
    let mut c0_same_rel = true;
    let mut c0_aa = true;
    for ((q, s), a) in &full {
        if let Some(old) = v3.get(&(*q, *s)) {
            for tag in ["pre", "lp50", "dlp50"] {
                if let Some(o) = old.get(tag) {
                    c0_replay.1 += 1;
                    let n = &a[tag];
                    if ["rel_candidates", "la_mulmods", "x_recovered"].iter().all(|k| o.s(k) == n.s(k)) {
                        c0_replay.0 += 1;
                    } else {
                        c0_replay_bad.push(format!("q {q} seed {s} {tag}"));
                    }
                }
            }
        }
        for tag in ["dlpm50", "dlpf50", "dlpf50_aa"] {
            c0_same_rel &= a[tag].s("rel_candidates") == a["dlp50"].s("rel_candidates");
        }
        for k in ["rel_candidates", "la_cols", "la_nnz", "la_mulmods", "filter_merges", "x_recovered", "lp_known"] {
            c0_aa &= a["dlpf50"].s(k) == a["dlpf50_aa"].s(k);
        }
    }
    let _ = writeln!(md, "C0: replays of dlp_20261113 (rel_candidates, la_mulmods, x_recovered) {}/{} exact{}; dlpm/dlpf/dlpf_aa candidates equal dlp50 on every instance: {c0_same_rel}; dlpf A/A counted fields identical: {c0_aa}\n",
        c0_replay.0, c0_replay.1, if c0_replay_bad.is_empty() { String::new() } else { format!(" (differ: {})", c0_replay_bad.join(", ")) });

    // ---- per instance ---------------------------------------------------------
    let t = |a: &BTreeMap<String, Rec>, tag: &str| a[tag].f("t_ic_total");
    let mm = |a: &BTreeMap<String, Rec>, tag: &str| a[tag].f("la_mulmods");
    let rexp = |a: &BTreeMap<String, Rec>| {
        let b = &a["pre"];
        b.f("rho_expected_steps") * b.f("rho_seconds") / b.f("rho_steps")
    };
    md += "| q | seed | mulmods dlpf/dlp | dlpf/dlpm | dlpf/lp | t_ic lp/dlpf | pre/dlpf | dlp/dlpf | A/A dlpf/dlpf_aa | LA cols dlp, dlpm, dlpf | nnz dlp, dlpm, dlpf | IC/rho-exp pre, lp, dlp, dlpf |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut inst_csv = String::from("q,seed,mm_f_over_dlp,mm_f_over_m,mm_f_over_lp,t_lp_over_f,t_pre_over_f,t_dlp_over_f,t_m_over_f,aa,gap_pre,gap_lp50,gap_dlp50,gap_dlpm50,gap_dlpf50\n");
    let mut perq: BTreeMap<u64, Vec<Vec<f64>>> = BTreeMap::new();
    let (mut f1, mut f2) = (true, true);
    for ((q, s), a) in &full {
        let re = rexp(a);
        let row = vec![
            mm(a, "dlpf50") / mm(a, "dlp50"),
            mm(a, "dlpf50") / mm(a, "dlpm50"),
            mm(a, "dlpf50") / mm(a, "lp50"),
            t(a, "lp50") / t(a, "dlpf50"),
            t(a, "pre") / t(a, "dlpf50"),
            t(a, "dlp50") / t(a, "dlpf50"),
            t(a, "dlpm50") / t(a, "dlpf50"),
            t(a, "dlpf50") / t(a, "dlpf50_aa"),
            t(a, "pre") / re,
            t(a, "lp50") / re,
            t(a, "dlp50") / re,
            t(a, "dlpm50") / re,
            t(a, "dlpf50") / re,
        ];
        f1 &= row[0] < 1.0;
        f2 &= row[1] < 1.0;
        let _ = writeln!(md, "| {q} | {s} | {:.3} | {:.3} | {:.3} | {:.2} | {:.2} | {:.2} | {:.3} | {}, {}, {} | {}, {}, {} | {:.2}, {:.2}, {:.2}, {:.2} |",
            row[0], row[1], row[2], row[3], row[4], row[5], row[7],
            a["dlp50"].s("la_cols"), a["dlpm50"].s("la_cols"), a["dlpf50"].s("la_cols"),
            a["dlp50"].s("la_nnz"), a["dlpm50"].s("la_nnz"), a["dlpf50"].s("la_nnz"),
            row[8], row[9], row[10], row[12]);
        let _ = writeln!(inst_csv, "{q},{s},{}", row.iter().map(|x| format!("{x:.4}")).collect::<Vec<_>>().join(","));
        perq.entry(*q).or_default().push(row);
    }

    // ---- per q ----------------------------------------------------------------
    md += "\n| q | n | mulmods dlpf/dlp median [max] | dlpf/dlpm median [max] | dlpf/lp median | t_ic lp/dlpf median [min, max] | dlpf faster than lp | A/A median [min, max] | IC/rho-exp median: pre, lp, dlp, dlpm, dlpf |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut q_csv = String::from("q,n,mm_f_over_dlp_med,mm_f_over_dlp_max,mm_f_over_m_med,mm_f_over_m_max,mm_f_over_lp_med,t_lp_over_f_med,t_lp_over_f_lo,t_lp_over_f_hi,f_faster_than_lp,aa_med,aa_lo,aa_hi,gap_pre,gap_lp50,gap_dlp50,gap_dlpm50,gap_dlpf50\n");
    let (mut f3, mut f4) = (true, true);
    let mut f3_detail = Vec::new();
    let mut f4_detail = Vec::new();
    for (q, v) in &perq {
        let c = |i: usize| -> Vec<f64> { v.iter().map(|r| r[i]).collect() };
        let n = v.len();
        let faster = c(3).iter().filter(|x| **x > 1.0).count();
        let gaps: Vec<f64> = (8..13).map(|i| med(&c(i))).collect();
        if *q >= 12007 {
            f3 &= faster >= 4;
            f3_detail.push(format!("q {q}: {faster}/{n}"));
            let best_other = gaps[0].min(gaps[1]).min(gaps[2]);
            f4 &= gaps[4] < best_other;
            f4_detail.push(format!("q {q}: dlpf {:.2} vs best of pre/lp/dlp {:.2}", gaps[4], best_other));
        }
        let _ = writeln!(md, "| {q} | {n} | {:.3} [{:.3}] | {:.3} [{:.3}] | {:.3} | {:.2} [{:.2}, {:.2}] | {faster}/{n} | {:.3} [{:.3}, {:.3}] | {:.2}, {:.2}, {:.2}, {:.2}, {:.2} |",
            med(&c(0)), hi(&c(0)), med(&c(1)), hi(&c(1)), med(&c(2)), med(&c(3)), lo(&c(3)), hi(&c(3)), med(&c(7)), lo(&c(7)), hi(&c(7)),
            gaps[0], gaps[1], gaps[2], gaps[3], gaps[4]);
        let _ = writeln!(q_csv, "{q},{n},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{faster},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
            med(&c(0)), hi(&c(0)), med(&c(1)), hi(&c(1)), med(&c(2)), med(&c(3)), lo(&c(3)), hi(&c(3)), med(&c(7)), lo(&c(7)), hi(&c(7)),
            gaps[0], gaps[1], gaps[2], gaps[3], gaps[4]);
    }

    // ---- phases ---------------------------------------------------------------
    let mut ph_csv = String::from("q,tag,t_relations,t_reduce,t_filter,t_linear_algebra,t_descent,t_ic,candidates,la_cols,la_nnz,la_mulmods\n");
    md += "\n| q | arm | t_rel | t_reduce | t_filter | t_LA | t_ic | candidates | LA cols | LA nnz | LA mulmods |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut slope_pts: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
    for q in perq.keys() {
        for tag in ARMS {
            let rs: Vec<&Rec> = full.iter().filter(|((qq, _), _)| qq == q).map(|(_, a)| &a[tag]).collect();
            let m = |k: &str| med(&rs.iter().map(|r| r.f(k)).collect::<Vec<_>>());
            let cand = med(&rs.iter().map(|r| r.f("rel_candidates") + r.f("quad_candidates")).collect::<Vec<_>>());
            let _ = writeln!(md, "| {q} | {tag} | {:.2} | {:.2} | {:.3} | {:.2} | {:.2} | {cand:.0} | {:.0} | {:.0} | {:.3e} |",
                m("t_relations"), m("t_reduce"), m("t_filter"), m("t_linear_algebra"), m("t_ic_total"), m("la_cols"), m("la_nnz"), m("la_mulmods"));
            let _ = writeln!(ph_csv, "{q},{tag},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{cand:.0},{:.0},{:.0},{:.0}",
                m("t_relations"), m("t_reduce"), m("t_filter"), m("t_linear_algebra"), m("t_descent"), m("t_ic_total"), m("la_cols"), m("la_nnz"), m("la_mulmods"));
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
    let _ = writeln!(md, "\n\nVerdicts (PROTOCOL.md): F1 dlpf < dlp mulmods on every instance: {}; F2 dlpf < dlpm mulmods on every instance: {}; F3 dlpf faster than lp on >= 4/5 at q >= 12007: {} ({}); F4 dlpf median IC/rho-exp below pre, lp and dlp at q >= 12007: {} ({})",
        if f1 { "holds" } else { "FALSIFIED" }, if f2 { "holds" } else { "FALSIFIED" },
        if f3 { "holds" } else { "FALSIFIED" }, f3_detail.join("; "), if f4 { "holds" } else { "FALSIFIED" }, f4_detail.join("; "));

    let dir = std::path::Path::new(out);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v4.md"), &md).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("instances_v4.csv"), &inst_csv).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v4.csv"), &q_csv).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("phases_v4.csv"), &ph_csv).map_err(|e| e.to_string())?;
    print!("{md}");
    Ok(())
}

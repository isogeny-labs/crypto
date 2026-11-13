//! `humbert summarize-dlp <out-dir> <v3.jsonl>...`: tables of the double-large-
//! prime campaign (dlp_20261113/PROTOCOL.md) from its recorded JSON lines.
//! Paired ratios use the same (q, seed); duplicate (q, seed, tag) is an error.
use crate::lpsummary::Rec;
use std::collections::BTreeMap;
use std::fmt::Write as _;

const TAGS: [&str; 5] = ["pre", "pre_aa", "lp50", "dlp50", "dlp35"];

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

pub fn run(out: &str, inputs: &[String]) -> Result<(), String> {
    let mut inst: BTreeMap<(u64, u64), BTreeMap<String, Rec>> = BTreeMap::new();
    let mut nrec = 0;
    for path in inputs {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
            let r = Rec::parse(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?;
            let key = (r.u("p"), r.u("seed"));
            let tag = r.s("tag");
            let slot = inst.entry(key).or_default();
            if slot.contains_key(&tag) {
                return Err(format!("{path}:{}: duplicate record q = {}, seed = {}, tag = {tag}", n + 1, key.0, key.1));
            }
            slot.insert(tag, r);
            nrec += 1;
        }
    }
    let gates = inst.values().flat_map(|a| a.values()).all(|r| {
        ["solved", "pushforward_8P", "T_P_nonzero", "iota_pairs_constant", "x_inf_eq_4c"].iter().all(|k| r.b(k))
    });
    let full: Vec<_> = inst.iter().filter(|(_, a)| TAGS.iter().all(|t| a.contains_key(*t))).collect();
    let t = |a: &BTreeMap<String, Rec>, tag: &str| a[tag].f("t_ic_total");
    let cand = |a: &BTreeMap<String, Rec>, tag: &str| a[tag].f("rel_candidates") + a[tag].f("quad_candidates");
    let rexp = |a: &BTreeMap<String, Rec>| {
        let b = &a["pre"];
        b.f("rho_expected_steps") * b.f("rho_seconds") / b.f("rho_steps")
    };
    let mut md = format!("runs: {nrec}; complete instances: {}; all correctness gates pass: {gates}\n\n", full.len());
    md += "| q | seed | pre t_ic | A/A pre/pre_aa | lp50 t_ic | dlp50 t_ic | dlp35 t_ic | pre/dlp50 | lp50/dlp50 | cand dlp50/lp50 | IC/rho-exp pre, dlp50 |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut inst_csv = String::from("q,seed,aa,pre_over_lp50,pre_over_dlp50,pre_over_dlp35,lp50_over_dlp50,cand_dlp50_over_lp50,gap_pre,gap_lp50,gap_dlp50,gap_dlp35\n");
    let mut perq: BTreeMap<u64, Vec<[f64; 12]>> = BTreeMap::new();
    for ((q, s), a) in &full {
        let re = rexp(a);
        let row = [
            t(a, "pre") / t(a, "pre_aa"),
            t(a, "pre") / t(a, "lp50"),
            t(a, "pre") / t(a, "dlp50"),
            t(a, "pre") / t(a, "dlp35"),
            t(a, "lp50") / t(a, "dlp50"),
            cand(a, "dlp50") / cand(a, "lp50"),
            t(a, "pre") / re,
            t(a, "lp50") / re,
            t(a, "dlp50") / re,
            t(a, "dlp35") / re,
            t(a, "pre"),
            t(a, "dlp50"),
        ];
        let _ = writeln!(md, "| {q} | {s} | {:.2} | {:.3} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.3} | {:.2}, {:.2} |",
            t(a, "pre"), row[0], t(a, "lp50"), t(a, "dlp50"), t(a, "dlp35"), row[2], row[4], row[5], row[6], row[8]);
        let _ = writeln!(inst_csv, "{q},{s},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
            row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8], row[9]);
        perq.entry(*q).or_default().push(row);
    }
    md += "\n| q | n | A/A median [min, max] | pre/dlp50 median [min, max] | dlp50 faster than pre | lp50/dlp50 median | cand dlp50/lp50 max | IC/rho-exp median: pre, lp50, dlp50, dlp35 |\n|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut q_csv = String::from("q,n,aa_med,aa_lo,aa_hi,m_med,m_lo,m_hi,dlp_faster,lp_over_dlp_med,cand_ratio_max,gap_pre,gap_lp50,gap_dlp50,gap_dlp35,t_pre_med,t_dlp50_med\n");
    for (q, v) in &perq {
        let c = |i: usize| -> Vec<f64> { v.iter().map(|r| r[i]).collect() };
        let n = v.len();
        let faster = c(2).iter().filter(|x| **x > 1.0).count();
        let _ = writeln!(md, "| {q} | {n} | {:.3} [{:.3}, {:.3}] | {:.2} [{:.2}, {:.2}] | {faster}/{n} | {:.2} | {:.3} | {:.2}, {:.2}, {:.2}, {:.2} |",
            med(&c(0)), lo(&c(0)), hi(&c(0)), med(&c(2)), lo(&c(2)), hi(&c(2)), med(&c(4)), hi(&c(5)),
            med(&c(6)), med(&c(7)), med(&c(8)), med(&c(9)));
        let _ = writeln!(q_csv, "{q},{n},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{faster},{:.3},{:.4},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}",
            med(&c(0)), lo(&c(0)), hi(&c(0)), med(&c(2)), lo(&c(2)), hi(&c(2)), med(&c(4)), hi(&c(5)),
            med(&c(6)), med(&c(7)), med(&c(8)), med(&c(9)), med(&c(10)), med(&c(11)));
    }
    // R4: log-log slope of the median t_ic between the smallest and largest q
    if let (Some((q0, v0)), Some((q1, v1))) = (perq.iter().next(), perq.iter().last()) {
        if q0 != q1 {
            let slope = |i: usize| -> f64 {
                let a = med(&v0.iter().map(|r| r[i]).collect::<Vec<_>>());
                let b = med(&v1.iter().map(|r| r[i]).collect::<Vec<_>>());
                (b / a).ln() / (*q1 as f64 / *q0 as f64).ln()
            };
            let _ = writeln!(md, "\nR4 (exploratory): log-log slope of median t_ic, q = {q0} -> {q1}: pre {:.2}, dlp50 {:.2}", slope(10), slope(11));
        }
    }
    let dir = std::path::Path::new(out);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v3.md"), &md).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("instances_v3.csv"), &inst_csv).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("summary_v3.csv"), &q_csv).map_err(|e| e.to_string())?;
    print!("{md}");
    Ok(())
}

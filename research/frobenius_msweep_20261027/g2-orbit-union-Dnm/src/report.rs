//! analyze: read rows_*.tsv, apply the frozen decision rule, write
//! summary.md, summary.tsv and rho_z.svg into the same directory.

use std::fmt::Write as _;
use std::fs;

use crate::combi::PatternTable;
use crate::runner::Row;
use crate::stats::{expected_total_per_target, group, phi_ratio_boot, rho_min, split_stats, SplitStat, MIN_D_FOR_RHO};

const NSPLIT: usize = 40;
const NPERM: usize = 400;
const Z_FLAG: f64 = 4.0; // per-cell flag level (Bonferroni over the grid)
const LEVER_TRANSFER: f64 = 2.0;

struct ArmOut {
    mean_total: f64,
    analytic: f64,
    unamb: f64,
    rep_per_target: f64,
    audit: String,
    aware: Option<SplitStat>,
    naive: Option<SplitStat>,
}

fn z(s: &SplitStat) -> f64 {
    if s.rho_se > 0.0 {
        s.rho / s.rho_se
    } else {
        0.0
    }
}

pub fn analyze_dir(dir: &str) {
    let mut rows: Vec<Row> = vec![];
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with("rows_") && name.ends_with(".tsv") {
            for line in fs::read_to_string(&p).unwrap().lines().skip(1) {
                rows.push(Row::from_tsv(line));
            }
        }
    }
    let cells = group(rows);
    let mut md = String::new();
    let mut tsv = String::from("a\tn\tm\tl\tD\tarm\tmean_total\tanalytic\tunamb\trep_per_target\taudit\tphi_aware\trho_aware\tz_aware\ttransfer_aware\tphi_naive\trho_naive\tz_naive\n");
    let mut zs: Vec<(String, f64, f64, f64)> = vec![];
    writeln!(md, "# G2 orbit-union D(n,m): analysis\n").unwrap();
    writeln!(md, "Decision rule: PROTOCOL.md section 7. Counts of decompositions only; no solver cost is measured.\n").unwrap();
    writeln!(md, "| cell (a,n,m,l) | D | n^(m-1)/D | verdict | rho z frob | rho z rsub | rho z rset | naive z skew | transfer frob | note |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut detail = String::new();
    for ((a, n, m, l), arms) in &cells {
        let pt = PatternTable::new(*n as usize, *m, true);
        let d = pt.n_classes();
        let mut outs: std::collections::BTreeMap<String, ArmOut> = Default::default();
        for (arm, rs) in arms {
            let refs: Vec<&Row> = rs.iter().collect();
            let nrow = rs.len() as f64;
            let nt = rs[0].nt as f64;
            let mean_total = rs.iter().map(|r| r.total as f64).sum::<f64>() / (nrow * nt);
            let analytic = rs.iter().map(|r| expected_total_per_target(&r.xs, r.m, r.order)).sum::<f64>() / nrow;
            let tot: f64 = rs.iter().map(|r| r.total as f64).sum();
            let unamb = rs.iter().map(|r| r.unamb as f64).sum::<f64>() / tot.max(1.0);
            let rep = rs.iter().map(|r| r.yrep.iter().sum::<f64>()).sum::<f64>() / (nrow * nt);
            let audit = {
                let a0: Vec<i32> = rs.iter().map(|r| r.audit).filter(|&v| v >= 0).collect();
                if a0.is_empty() { "-".into() } else { format!("{}", a0[0]) }
            };
            let union = rs[0].nb > 1 && rs.len() >= 4 && d >= 3;
            let (aware, naive) = if union {
                let seed = 0x5EED ^ ((*a as u64) << 40) ^ ((*n as u64) << 32) ^ ((*m as u64) << 24);
                (
                    Some(split_stats(&refs, &pt, true, seed, NSPLIT, NPERM)),
                    Some(split_stats(&refs, &pt, false, seed, NSPLIT, NPERM)),
                )
            } else {
                (None, None)
            };
            outs.insert(arm.clone(), ArmOut { mean_total, analytic, unamb, rep_per_target: rep, audit, aware, naive });
        }
        for (arm, o) in &outs {
            let f = |s: &Option<SplitStat>, g: fn(&SplitStat) -> f64| s.as_ref().map(|s| format!("{:.4}", g(s))).unwrap_or("-".into());
            writeln!(
                tsv,
                "{a}\t{n}\t{m}\t{l}\t{d}\t{arm}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                o.mean_total, o.analytic, o.unamb, o.rep_per_target, o.audit,
                f(&o.aware, |s| s.phi), f(&o.aware, |s| s.rho), f(&o.aware, z), f(&o.aware, |s| s.transfer),
                f(&o.naive, |s| s.phi), f(&o.naive, |s| s.rho), f(&o.naive, z)
            ).unwrap();
        }
        let ratio_ci = {
            let get = |arm: &str| -> Option<Vec<&Row>> { arms.get(arm).map(|v| v.iter().collect()) };
            match (get("frob"), get("rsub"), get("rset")) {
                (Some(f), Some(a2), Some(b2)) if d >= 3 && f.len() >= 4 => {
                    Some(phi_ratio_boot(&f, &[a2, b2], &pt, 0xB007 ^ ((*n as u64) << 8) ^ *m as u64, 400))
                }
                _ => None,
            }
        };
        let zf = |arm: &str| outs.get(arm).and_then(|o| o.aware.as_ref()).map(z);
        let (zfrob, zrsub, zrset) = (zf("frob"), zf("rsub"), zf("rset"));
        let zskew_naive = outs.get("skew").and_then(|o| o.naive.as_ref()).map(z);
        let tr = outs.get("frob").and_then(|o| o.aware.as_ref()).map(|s| s.transfer);
        // ---- frozen decision rule ----
        let mut note = String::new();
        let verdict = (|| {
            let (Some(zfrob), Some(zrsub), Some(zrset)) = (zfrob, zrsub, zrset) else {
                return "NO_DATA";
            };
            for arm in ["frob", "geom"] {
                if let Some(o) = outs.get(arm) {
                    if o.audit == "0" {
                        note.push_str("exact gauge identity FAILED on ");
                        note.push_str(arm);
                        return "INSTRUMENT_FAIL";
                    }
                }
            }
            if zrsub >= Z_FLAG || zrset >= Z_FLAG {
                note.push_str("negative control flagged");
                return "INSTRUMENT_FAIL";
            }
            if d < MIN_D_FOR_RHO {
                // small-D branch: dispersion ratio against the controls, bootstrap over V draws
                let Some((pt_, lo, hi)) = ratio_ci else { return "NO_DATA" };
                note.push_str(&format!("small D={d} (rho_min={:.2}): phi ratio vs controls {pt_:.2} [{lo:.2},{hi:.2}]", rho_min(d)));
                if lo > 1.5 {
                    return "CONCENTRATED_VS_CONTROLS";
                }
                if hi < 1.25 {
                    return "UNIFORM_VS_CONTROLS";
                }
                return "UNRESOLVED";
            }
            match zskew_naive {
                Some(zs) if zs >= Z_FLAG => {}
                _ => {
                    note.push_str("positive control (skew, naive null) not detected");
                    return "INSTRUMENT_FAIL";
                }
            }
            let sf = outs["frob"].aware.as_ref().unwrap();
            let t = tr.unwrap_or(f64::NAN);
            if zfrob >= Z_FLAG && t >= LEVER_TRANSFER {
                return "CONCENTRATED";
            }
            if zfrob >= Z_FLAG {
                note.push_str("replicable but transfer < 2x");
                return "CONCENTRATED_WEAK";
            }
            if sf.rho + 1.96 * sf.rho_se < 0.25 {
                return "UNIFORM";
            }
            note.push_str("rho upper bound >= 0.25");
            "UNRESOLVED"
        })();
        let fmt = |v: Option<f64>| v.map(|x| format!("{x:.2}")).unwrap_or("-".into());
        writeln!(
            md,
            "| {a},{n},{m},{l} | {d} | {:.2} | {verdict} | {} | {} | {} | {} | {} | {note} |",
            crate::combi::baseline_ratio(*n as u64, *m as u64), fmt(zfrob), fmt(zrsub), fmt(zrset), fmt(zskew_naive), fmt(tr)
        ).unwrap();
        zs.push((format!("{a},{n},{m}"), zfrob.unwrap_or(0.0), zrsub.unwrap_or(0.0), zrset.unwrap_or(0.0)));
        // yield comparison block
        let g = |arm: &str| outs.get(arm).map(|o| o.mean_total);
        writeln!(detail, "\n## cell a={a} n={n} m={m} l={l} (D={d})\n").unwrap();
        writeln!(detail, "| arm | dec/target | analytic | obs/analytic | unambiguous | rep-pattern dec/target | audit |").unwrap();
        writeln!(detail, "|---|---|---|---|---|---|---|").unwrap();
        for (arm, o) in &outs {
            writeln!(detail, "| {arm} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {} |", o.mean_total, o.analytic, o.mean_total / o.analytic.max(1e-300), o.unamb, o.rep_per_target, o.audit).unwrap();
        }
        if let (Some(fr), Some(pb), Some(pl)) = (g("frob"), g("plainbig"), g("plain")) {
            writeln!(detail, "\nfrob/plainbig = {:.3} (size-matched; 1 means per-base-size^m cost-neutral in yield), frob/plain = {:.1} (n^m = {:.0}).", fr / pb.max(1e-300), fr / pl.max(1e-300), (*n as f64).powi(*m as i32)).unwrap();
        }
        if let Some(fr) = arms.get("frob") {
            // A/A: the same arm, same code path, disjoint halves of the V draws
            let h = fr.len() / 2;
            if h >= 2 && d >= 3 {
                let (a1, a2): (Vec<&Row>, Vec<&Row>) = (fr[..h].iter().collect(), fr[h..].iter().collect());
                let (p1, p2) = (crate::stats::phi(&crate::stats::pool(&a1, &pt, true, true)), crate::stats::phi(&crate::stats::pool(&a2, &pt, true, true)));
                writeln!(detail, "A/A (frob draws first half vs second half): phi {p1:.3} vs {p2:.3}, ratio {:.3}", p1 / p2).unwrap();
            }
        }
        if let Some(o) = outs.get("frob") {
            writeln!(detail, "rep/total (frob) = {:.4}; 1/n = {:.4} (the rep-pattern systems of the D classes see this fraction of the union's decompositions).", o.rep_per_target / o.mean_total.max(1e-300), 1.0 / *n as f64).unwrap();
        }
    }
    md.push_str(&detail);
    fs::write(format!("{dir}/summary.md"), &md).unwrap();
    fs::write(format!("{dir}/summary.tsv"), &tsv).unwrap();
    fs::write(format!("{dir}/rho_z.svg"), svg(&zs)).unwrap();
    println!("wrote {dir}/summary.md, summary.tsv, rho_z.svg");
}

fn svg(zs: &[(String, f64, f64, f64)]) -> String {
    let w = 60 + zs.len() * 34;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='260' font-family='sans-serif' font-size='10'>");
    s.push_str("<rect width='100%' height='100%' fill='white'/>");
    let y = |v: f64| 130.0 - v.clamp(-12.0, 12.0) * 9.0;
    for (zv, col) in [(4.0, "#b00"), (-4.0, "#b00"), (0.0, "#888")] {
        write!(s, "<line x1='40' x2='{}' y1='{}' y2='{}' stroke='{col}' stroke-dasharray='3'/>", w - 10, y(zv), y(zv)).unwrap();
    }
    for (i, (lab, a, b, c)) in zs.iter().enumerate() {
        let x = 50.0 + i as f64 * 34.0;
        for (j, (v, col)) in [(a, "#1f77b4"), (b, "#999"), (c, "#bbb")].iter().enumerate() {
            write!(s, "<rect x='{}' y='{}' width='8' height='{}' fill='{col}'/>", x + j as f64 * 9.0, y(**v).min(130.0), (y(**v) - 130.0).abs().max(1.0)).unwrap();
        }
        write!(s, "<text x='{}' y='250' transform='rotate(-60 {} 250)'>{lab}</text>", x, x).unwrap();
    }
    s.push_str("<text x='42' y='14'>split-half rho z (blue frob, grey rsub/rset); red = +-4 flag level; cells a,n,m</text></svg>");
    s
}

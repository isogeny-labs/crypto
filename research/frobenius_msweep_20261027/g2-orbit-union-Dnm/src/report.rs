//! analyze: read rows_*.tsv, apply the frozen decision rule, write
//! summary.md, summary.tsv and rho_z.svg into the same directory.

use std::fmt::Write as _;
use std::fs;

use crate::combi::PatternTable;
use crate::runner::Row;
use crate::stats::{
    contrast_z, expected_total_per_target, group, phi_ratio_boot, pool, rel_bias, rho_min,
    split_stats, z_of, NullMode, SplitStat, MIN_D_FOR_RHO,
};

const NSPLIT: usize = 40;
const NPERM: usize = 400;
const Z_FLAG: f64 = 4.0; // per-cell flag level (Bonferroni over the grid)
const LEVER_TRANSFER: f64 = 2.0;

/// Per-class per-half count (integer, floor of total frob yield / (2 D)) from which the shared-nuisance
/// saturation of the split-half correlation can hide a frob-only effect (PROTOCOL section 5, amendment 2026-10-27).
pub const SATURATION_COUNT: u64 = 10_000;

/// True iff the rho-based Frobenius-specificity contrast has resolution at this count level. The comparison is on an
/// integer count, not a float threshold.
pub fn specificity_resolvable(total_yield: f64, d: usize) -> bool {
    if d == 0 {
        return true;
    }
    let per_class_per_half = (total_yield / (2.0 * d as f64)).floor() as u64;
    per_class_per_half < SATURATION_COUNT
}

/// l-ladder status from (l, rel. excess, verdict) triples of one (a, n, m). l = 2 is not used. Evaluable only when
/// l = 3 is frob-specific (CONCENTRATED or CONCENTRATED_WEAK) AND the l = 5 verdict is one the instrument can read
/// (CONCENTRATED, CONCENTRATED_WEAK, UNIFORM, UNRESOLVED); an l = 5 INSTRUMENT_FAIL, NUISANCE_REPLICABLE,
/// REPLICABLE_NOT_SEPARATED, INCOMPLETE or NO_DATA is an instrument or resolution failure, never decay (NOT_EVALUABLE).
pub fn ladder_status(v: &[(usize, Option<f64>, String)]) -> &'static str {
    let at = |l: usize| v.iter().find(|(ll, _, _)| *ll == l);
    let flagged = |vd: &str| vd == "CONCENTRATED" || vd == "CONCENTRATED_WEAK";
    let readable = |vd: &str| flagged(vd) || vd == "UNIFORM" || vd == "UNRESOLVED";
    match (at(3), at(5)) {
        (Some((_, e3, v3)), Some((_, e5, v5))) if flagged(v3) && readable(v5) => match (e3, e5) {
            (Some(e3), Some(e5)) if *e3 > 0.0 && flagged(v5) && *e5 >= 0.5 * *e3 => "PERSISTENT",
            _ => "FALLING_OR_ABSENT",
        },
        _ => "NOT_EVALUABLE (no frob-specific excess at l = 3, no l = 3 / 5 cell, or the l = 5 verdict is an instrument / resolution failure)",
    }
}

struct ArmOut {
    mean_total: f64,
    analytic: f64,
    unamb: f64,
    rep_per_target: f64,
    audit: String,
    aware: Option<SplitStat>,
    size: Option<SplitStat>,
    naive: Option<SplitStat>,
    empty_rows: usize,
    nrows: usize,
    rb: Option<f64>,
    yield_per_subset: f64,
}

fn z(s: &SplitStat) -> f64 {
    z_of(s)
}

pub fn analyze_dir(dir: &str) {
    let mut rows: Vec<Row> = vec![];
    let mut skipped_lines = 0usize;
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with("rows_") && name.ends_with(".tsv") {
            for line in fs::read_to_string(&p).unwrap().lines().skip(1) {
                // a truncated last line (killed process) must not abort the analysis; it is counted
                if line.split('\t').count() != 24 || line.trim().is_empty() {
                    skipped_lines += 1;
                    continue;
                }
                rows.push(Row::from_tsv(line));
            }
        }
    }
    let cells = group(rows);
    let mut md = String::new();
    let mut tsv = String::from("a\tn\tm\tl\tD\tarm\tmean_total\tanalytic\tunamb\trep_per_target\taudit\tphi_aware\trho_aware\tz_aware\ttransfer_aware\tphi_naive\trho_naive\tz_naive\n");
    let mut zs: Vec<(String, f64, f64, f64)> = vec![];
    writeln!(md, "# G2 orbit-union D(n,m): analysis\n").unwrap();
    writeln!(md, "Decision rule: PROTOCOL.md section 7 (revision 3). Counts of decompositions only; no solver cost is measured; no cross-method cost claim, so no AGENTS.md 5a band applies.\n").unwrap();
    writeln!(md, "Reading notes (PROTOCOL.md section 12). In this document n is the FIELD DEGREE and m the DECOMPOSITION SIZE, the reverse of the repository's m = field degree. There is NO m = 83 evidence here (field degrees 7..17 only); ECC2K-130 is over F_(2^131) and nothing here is a statement at 131. D(131,m) is closed-form arithmetic. a = 0 is the ECC2K-130 model at a toy degree, a = 1 its quadratic twist over F_2. ord_n(2) = 3, 10, 12, 8 for n = 7, 11, 13, 17, so n = 7 and 17 have Frobenius-invariant subspaces that 83 and 131 do not.\n").unwrap();
    if skipped_lines > 0 {
        writeln!(md, "WARNING: {skipped_lines} truncated or malformed row line(s) were skipped; the affected cells are marked INCOMPLETE if an arm is short of NV rows.\n").unwrap();
    }
    writeln!(md, "| cell (a,n,m,l) | D | n^(m-1)/D | rho resolved | verdict | z frob | z frobperm | z contrast (frob - frobperm) | z rsub | z rset | naive z skew | transfer frob | phi frob/frobperm | rel. excess frob over frobperm | note |").unwrap();
    writeln!(
        md,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    )
    .unwrap();
    let mut vtsv = String::from("a\tn\tm\tl\tD\trho_resolved\tverdict\tz_frob\tz_frobperm\tz_contrast\tz_frob_sizeonly_null\tspecificity_resolvable\n");
    let mut detail = String::new();
    let mut ladder: std::collections::BTreeMap<
        (u32, u32, usize),
        Vec<(usize, Option<f64>, String)>,
    > = Default::default();
    for ((a, n, m, l), arms) in &cells {
        let pt = PatternTable::new(*n as usize, *m, true);
        let d = pt.n_classes();
        let mut outs: std::collections::BTreeMap<String, ArmOut> = Default::default();
        for (arm, rs) in arms {
            let refs: Vec<&Row> = rs.iter().collect();
            let nrow = rs.len() as f64;
            let nt = rs[0].nt as f64;
            let mean_total = rs.iter().map(|r| r.total as f64).sum::<f64>() / (nrow * nt);
            let analytic = rs
                .iter()
                .map(|r| expected_total_per_target(&r.xs, r.m, r.order))
                .sum::<f64>()
                / nrow;
            let tot: f64 = rs.iter().map(|r| r.total as f64).sum();
            let unamb = rs.iter().map(|r| r.unamb as f64).sum::<f64>() / tot.max(1.0);
            let rep = rs.iter().map(|r| r.yrep.iter().sum::<f64>()).sum::<f64>() / (nrow * nt);
            let audit = {
                let a0: Vec<i32> = rs.iter().map(|r| r.audit).filter(|&v| v >= 0).collect();
                if a0.is_empty() {
                    "-".into()
                } else {
                    format!("{}", a0[0])
                }
            };
            let empty_rows = rs.iter().filter(|r| r.total == 0).count();
            let yield_per_subset = rs
                .iter()
                .map(|r| {
                    let nux = (r.npts / 2) as u64;
                    r.total as f64 / (r.nt as f64 * crate::combi::binom_f(nux, r.m as u64).max(1.0))
                })
                .sum::<f64>()
                / nrow;
            let rb = if rs[0].nb > 1 && d >= 3 {
                Some(rel_bias(&pool(&refs, &pt, NullMode::Parity, true)))
            } else {
                None
            };
            let union = rs[0].nb > 1 && rs.len() >= 4 && d >= 3;
            let (aware, size, naive) = if union {
                let seed = 0x5EED ^ ((*a as u64) << 40) ^ ((*n as u64) << 32) ^ ((*m as u64) << 24);
                (
                    Some(split_stats(
                        &refs,
                        &pt,
                        NullMode::Parity,
                        seed,
                        NSPLIT,
                        NPERM,
                    )),
                    Some(split_stats(&refs, &pt, NullMode::Size, seed, NSPLIT, NPERM)),
                    Some(split_stats(
                        &refs,
                        &pt,
                        NullMode::Naive,
                        seed,
                        NSPLIT,
                        NPERM,
                    )),
                )
            } else {
                (None, None, None)
            };
            outs.insert(
                arm.clone(),
                ArmOut {
                    mean_total,
                    analytic,
                    unamb,
                    rep_per_target: rep,
                    audit,
                    aware,
                    size,
                    naive,
                    empty_rows,
                    nrows: rs.len(),
                    rb,
                    yield_per_subset,
                },
            );
        }
        for (arm, o) in &outs {
            let f = |s: &Option<SplitStat>, g: fn(&SplitStat) -> f64| {
                s.as_ref()
                    .map(|s| format!("{:.4}", g(s)))
                    .unwrap_or("-".into())
            };
            writeln!(
                tsv,
                "{a}\t{n}\t{m}\t{l}\t{d}\t{arm}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                o.mean_total, o.analytic, o.unamb, o.rep_per_target, o.audit,
                f(&o.aware, |s| s.phi), f(&o.aware, |s| s.rho), f(&o.aware, z), f(&o.aware, |s| s.transfer),
                f(&o.naive, |s| s.phi), f(&o.naive, |s| s.rho), f(&o.naive, z)
            ).unwrap();
        }
        let ratio_ci = {
            let get =
                |arm: &str| -> Option<Vec<&Row>> { arms.get(arm).map(|v| v.iter().collect()) };
            match (get("frob"), get("rsub"), get("rset"), get("frobperm")) {
                (Some(f), Some(a2), Some(b2), Some(c2)) if d >= 3 && f.len() >= 4 => {
                    Some(phi_ratio_boot(
                        &f,
                        &[a2, b2, c2],
                        &pt,
                        0xB007 ^ ((*n as u64) << 8) ^ *m as u64,
                        400,
                    ))
                }
                _ => None,
            }
        };
        let zf = |arm: &str| outs.get(arm).and_then(|o| o.aware.as_ref()).map(z);
        let (zfrob, zrsub, zrset, zperm) = (zf("frob"), zf("rsub"), zf("rset"), zf("frobperm"));
        let zsize = outs.get("frob").and_then(|o| o.size.as_ref()).map(z);
        let zskew_naive = outs.get("skew").and_then(|o| o.naive.as_ref()).map(z);
        let tr = outs
            .get("frob")
            .and_then(|o| o.aware.as_ref())
            .map(|s| s.transfer);
        let rho_ok = d >= MIN_D_FOR_RHO;
        let zcon = match (
            outs.get("frob").and_then(|o| o.aware.as_ref()),
            outs.get("frobperm").and_then(|o| o.aware.as_ref()),
        ) {
            (Some(f), Some(p)) => Some(contrast_z(f, p)),
            _ => None,
        };
        let spec_ok = specificity_resolvable(
            arms.get("frob")
                .map(|v| v.iter().map(|r| r.yfrac.iter().sum::<f64>()).sum::<f64>())
                .unwrap_or(0.0),
            d,
        );
        let complete = ["frob", "frobperm", "rsub", "rset"].iter().all(|a| {
            arms.get(*a)
                .map(|v| v.len() == crate::plan::NV)
                .unwrap_or(false)
        });
        // ---- frozen decision rule (PROTOCOL section 7) ----
        let mut note = String::new();
        let verdict = (|| {
            let (Some(zfrob), Some(zrsub), Some(zrset), Some(zperm)) = (zfrob, zrsub, zrset, zperm)
            else {
                return "NO_DATA";
            };
            if !complete {
                note.push_str("an arm is short of NV rows (killed or truncated): NOT RUN");
                return "INCOMPLETE";
            }
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
                note.push_str("negative control flagged (null model wrong for this cell)");
                return "INSTRUMENT_FAIL";
            }
            if !rho_ok {
                // small-D branch: dispersion ratio against ALL controls (rsub, rset, frobperm),
                // bootstrap over V draws. There is no positive control here and no UNIFORM verdict.
                let Some((pt_, lo, hi)) = ratio_ci else {
                    return "NO_DATA";
                };
                note.push_str(&format!("D={d} < {MIN_D_FOR_RHO}, rho_min={:.2}, rho NOT resolved; phi ratio vs max(controls) {pt_:.2} [{lo:.2},{hi:.2}]", rho_min(d)));
                if lo > 1.5 {
                    return "CONCENTRATED_VS_CONTROLS";
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
            if zfrob >= Z_FLAG {
                // Frobenius-specificity is decided by the DIRECT frob - frobperm contrast, never by the
                // power-dependent non-event z(frobperm) < 4.
                let zc = zcon.unwrap_or(f64::NAN);
                if zc >= Z_FLAG {
                    if t >= LEVER_TRANSFER {
                        return "CONCENTRATED";
                    }
                    note.push_str("replicable and frob-specific, but transfer < 2x");
                    return "CONCENTRATED_WEAK";
                }
                if zperm >= Z_FLAG {
                    note.push_str("replicable excess ALSO in frobperm and not separable from it: shared nuisance, not Frobenius-specific");
                    if !spec_ok {
                        note.push_str("; SPECIFICITY NOT RESOLVABLE at this count level (saturation, PROTOCOL 5): not a negative");
                    }
                    return "NUISANCE_REPLICABLE";
                }
                note.push_str("replicable, but frobperm is not significantly lower (contrast z < 4): specificity NOT resolved");
                if !spec_ok {
                    note.push_str("; SPECIFICITY NOT RESOLVABLE at this count level (saturation, PROTOCOL 5): not a negative");
                }
                return "REPLICABLE_NOT_SEPARATED";
            }
            if sf.rho + 1.96 * sf.rho_se < 0.25 {
                return "UNIFORM";
            }
            note.push_str("rho upper bound >= 0.25");
            "UNRESOLVED"
        })();
        let fmt = |v: Option<f64>| v.map(|x| format!("{x:.2}")).unwrap_or("-".into());
        let phi_of = |arm: &str| outs.get(arm).and_then(|o| o.aware.as_ref()).map(|s| s.phi);
        let excess = match (phi_of("frob"), phi_of("frobperm")) {
            (Some(a1), Some(b1)) if b1 > 0.0 => Some(a1 / b1),
            _ => None,
        };
        // relative class-bias excess of frob over frobperm (scale-free; compared across the l ladder)
        let rel_excess = match (
            outs.get("frob").and_then(|o| o.rb),
            outs.get("frobperm").and_then(|o| o.rb),
        ) {
            (Some(x), Some(y)) => Some((x - y).max(0.0).sqrt()),
            _ => None,
        };
        let tiny = arms
            .get("frob")
            .map(|v| {
                (v.iter()
                    .map(|r| r.xs.iter().sum::<u64>() as f64 / r.xs.len().max(1) as f64)
                    .sum::<f64>()
                    / v.len() as f64)
                    < 2.5
            })
            .unwrap_or(false);
        if tiny {
            note.push_str(" [tiny blocks, mean liftable x per block < 2.5: kept in the table, excluded from the campaign tallies]");
        }
        writeln!(
            md,
            "| {a},{n},{m},{l} | {d} | {:.2} | {} | {verdict} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {note} |",
            crate::combi::baseline_ratio(*n as u64, *m as u64), if rho_ok { "yes" } else { "NO" },
            fmt(zfrob), fmt(zperm), fmt(zcon), fmt(zrsub), fmt(zrset), fmt(zskew_naive), fmt(tr), fmt(excess), rel_excess.map(|v| format!("{v:.4}")).unwrap_or("-".into())
        ).unwrap();
        ladder
            .entry((*a, *n, *m))
            .or_default()
            .push((*l, rel_excess, verdict.to_string()));
        writeln!(
            vtsv,
            "{a}\t{n}\t{m}\t{l}\t{d}\t{rho_ok}\t{verdict}\t{}\t{}\t{}\t{}\t{}",
            fmt(zfrob),
            fmt(zperm),
            fmt(zcon),
            fmt(zsize),
            spec_ok
        )
        .unwrap();
        zs.push((
            format!("{a},{n},{m},{l}"),
            zfrob.unwrap_or(0.0),
            zrsub.unwrap_or(0.0),
            zrset.unwrap_or(0.0),
        ));
        // yield comparison block
        let g = |arm: &str| outs.get(arm).map(|o| o.mean_total);
        writeln!(detail, "\n## cell a={a} n={n} m={m} l={l} (D={d})\n").unwrap();
        writeln!(detail, "| arm | dec/target | analytic | obs/analytic | dec/target/C(nux,m) | unambiguous | rep-pattern dec/target | audit (12 targets, vd 0) | empty rows |").unwrap();
        writeln!(detail, "|---|---|---|---|---|---|---|---|---|").unwrap();
        for (arm, o) in &outs {
            writeln!(
                detail,
                "| {arm} | {:.3} | {:.3} | {:.3} | {:.3e} | {:.3} | {:.3} | {} | {}/{} |",
                o.mean_total,
                o.analytic,
                o.mean_total / o.analytic.max(1e-300),
                o.yield_per_subset,
                o.unamb,
                o.rep_per_target,
                o.audit,
                o.empty_rows,
                o.nrows
            )
            .unwrap();
        }
        writeln!(detail, "\naudit: 1 = Proposition T held, 0 = violated, - = not run. It is REPORTED for rsub, rset and frobperm (expected 0 when the blocks are big enough to show a violation) but is not part of the verdict; only a 0 on frob or geom is an INSTRUMENT_FAIL. Empty rows (no decomposition of any target) are listed, not dropped.").unwrap();
        if let (Some(fr), Some(pb), Some(pl)) = (g("frob"), g("plainbig"), g("plain")) {
            writeln!(detail, "\nfrob/plainbig = {:.3} raw (plainbig is the LARGER compact subspace of dimension l+ceil(log2 n), NOT size-matched; expected raw ratio is about (|U|/|plainbig|)^m); frob/plain = {:.1} (n^m = {:.0}).", fr / pb.max(1e-300), fr / pl.max(1e-300), (*n as f64).powi(*m as i32)).unwrap();
            if let (Some(yf), Some(yb)) = (
                outs.get("frob").map(|o| o.yield_per_subset),
                outs.get("plainbig").map(|o| o.yield_per_subset),
            ) {
                writeln!(detail, "Size-normalised (decompositions per target per C(#distinct liftable x, m)): frob/plainbig = {:.3}; 1 means the union has the same yield per m-subset of its x-values as a compact subspace.", yf / yb.max(1e-300)).unwrap();
            }
        }
        if let Some(fr) = arms.get("frob") {
            // A/A: the same arm, same code path, disjoint halves of the V draws
            let h = fr.len() / 2;
            if h >= 2 && d >= 3 {
                let (a1, a2): (Vec<&Row>, Vec<&Row>) =
                    (fr[..h].iter().collect(), fr[h..].iter().collect());
                let (p1, p2) = (
                    crate::stats::phi(&crate::stats::pool(&a1, &pt, true, true)),
                    crate::stats::phi(&crate::stats::pool(&a2, &pt, true, true)),
                );
                writeln!(detail, "A/A (frob draws first half vs second half): phi {p1:.3} vs {p2:.3}, ratio {:.3}", p1 / p2).unwrap();
            }
        }
        if let Some(o) = outs.get("frob") {
            writeln!(detail, "rep/total (frob) = {:.4}; 1/n = {:.4} (the rep-pattern systems of the D classes see this fraction of the union's decompositions).", o.rep_per_target / o.mean_total.max(1e-300), 1.0 / *n as f64).unwrap();
        }
    }
    // l ladder: does a frob-over-frobperm excess persist as l grows (a Frobenius-twisted toy artefact
    // is predicted to fall off like 2^-l)?
    writeln!(
        md,
        "\n## l ladder (relative excess of frob over frobperm; fixed (a,n,m), several l)\n"
    )
    .unwrap();
    for ((a, n, m), v) in &ladder {
        if v.len() > 1 {
            let cells: Vec<String> = v
                .iter()
                .map(|(l, e, vd)| {
                    format!(
                        "l={l}: {} ({vd})",
                        e.map(|x| format!("{x:.4}")).unwrap_or("-".into())
                    )
                })
                .collect();
            // Coded persistence criterion (PROTOCOL section 7, revision 3). l = 2 is a tiny-block cell and is
            // printed but not used. Evaluable only where l = 3 shows a frob-specific replicable excess; it
            // PERSISTS iff l = 5 is also flagged frob-specific and rel. excess(l=5) >= 0.5 * rel. excess(l=3) > 0.
            // A 2^-l artefact predicts a ratio of 1/4.
            let status = ladder_status(v);
            writeln!(
                md,
                "* a={a} n={n} m={m}: {} -> ladder {status}",
                cells.join("; ")
            )
            .unwrap();
        }
    }
    md.push_str(&detail);
    fs::write(format!("{dir}/summary.md"), &md).unwrap();
    fs::write(format!("{dir}/summary.tsv"), &tsv).unwrap();
    fs::write(format!("{dir}/verdicts.tsv"), &vtsv).unwrap();
    fs::write(format!("{dir}/rho_z.svg"), svg(&zs)).unwrap();
    println!("wrote {dir}/summary.md, summary.tsv, rho_z.svg");
}

fn svg(zs: &[(String, f64, f64, f64)]) -> String {
    let w = 60 + zs.len() * 34;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='260' font-family='sans-serif' font-size='10'>");
    s.push_str("<rect width='100%' height='100%' fill='white'/>");
    let y = |v: f64| 130.0 - v.clamp(-12.0, 12.0) * 9.0;
    for (zv, col) in [(4.0, "#b00"), (-4.0, "#b00"), (0.0, "#888")] {
        write!(
            s,
            "<line x1='40' x2='{}' y1='{}' y2='{}' stroke='{col}' stroke-dasharray='3'/>",
            w - 10,
            y(zv),
            y(zv)
        )
        .unwrap();
    }
    for (i, (lab, a, b, c)) in zs.iter().enumerate() {
        let x = 50.0 + i as f64 * 34.0;
        for (j, (v, col)) in [(a, "#1f77b4"), (b, "#999"), (c, "#bbb")]
            .iter()
            .enumerate()
        {
            write!(
                s,
                "<rect x='{}' y='{}' width='8' height='{}' fill='{col}'/>",
                x + j as f64 * 9.0,
                y(**v).min(130.0),
                (y(**v) - 130.0).abs().max(1.0)
            )
            .unwrap();
        }
        write!(
            s,
            "<text x='{}' y='250' transform='rotate(-60 {} 250)'>{lab}</text>",
            x, x
        )
        .unwrap();
    }
    s.push_str("<text x='42' y='14'>split-half rho z (blue frob, grey rsub/rset); red = +-4 flag level; cells a,n,m</text></svg>");
    s
}

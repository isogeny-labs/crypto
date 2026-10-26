//! Fixed analysis program (decision rule frozen in PROTOCOL.md section 7). Reads rows_*.tsv,
//! writes summary.tsv, summary.md and fig_cost_ratio.svg. No analyst choices: roles are hard-coded.

use crate::bases::Rng;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

type Row = HashMap<String, String>;
type CellKey = (u32, u32, usize, u32);

pub fn load(dir: &str) -> Result<BTreeMap<CellKey, Vec<Row>>, String> {
    let mut out: BTreeMap<CellKey, Vec<Row>> = BTreeMap::new();
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if !(name.starts_with("rows_") && name.ends_with(".tsv")) {
            continue;
        }
        let txt = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
        let mut lines = txt.lines();
        let hdr: Vec<&str> = match lines.next() {
            Some(h) => h.split('\t').collect(),
            None => continue,
        };
        for ln in lines {
            let r: Row = hdr.iter().zip(ln.split('\t')).map(|(k, v)| (k.to_string(), v.to_string())).collect();
            let key = (r["a"].parse().unwrap(), r["n"].parse().unwrap(), r["m"].parse().unwrap(), r["l"].parse().unwrap());
            out.entry(key).or_default().push(r);
        }
    }
    Ok(out)
}

fn col(rows: &[Row], arm: &str, half: &str, k: &str) -> Vec<f64> {
    rows.iter()
        .filter(|r| r["arm"] == arm && r["half"] == half)
        .filter_map(|r| r.get(k).and_then(|v| v.parse::<f64>().ok()))
        .filter(|v| v.is_finite())
        .collect()
}
fn count(rows: &[Row], arm: &str, half: &str) -> usize {
    rows.iter().filter(|r| r["arm"] == arm && r["half"] == half).count()
}
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 }
}

#[derive(Clone, Copy, Debug)]
pub struct Ci {
    pub est: f64,
    pub lo: f64,
    pub hi: f64,
}
fn pct(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    (v[(0.025 * n as f64) as usize], v[((0.975 * n as f64) as usize).min(n - 1)])
}
const NB: usize = 2000;
pub fn boot_mean(v: &[f64], seed: u64) -> Ci {
    if v.is_empty() {
        return Ci { est: f64::NAN, lo: f64::NAN, hi: f64::NAN };
    }
    let mut rng = Rng(seed);
    let bs: Vec<f64> = (0..NB).map(|_| mean(&(0..v.len()).map(|_| v[rng.below(v.len() as u64) as usize]).collect::<Vec<_>>())).collect();
    let (lo, hi) = pct(bs);
    Ci { est: mean(v), lo, hi }
}
/// Ratio of means (num/den), both resampled independently.
pub fn boot_ratio(num: &[f64], den: &[f64], seed: u64) -> Ci {
    if num.is_empty() || den.is_empty() {
        return Ci { est: f64::NAN, lo: f64::NAN, hi: f64::NAN };
    }
    let mut rng = Rng(seed);
    let rs = |v: &[f64], rng: &mut Rng| mean(&(0..v.len()).map(|_| v[rng.below(v.len() as u64) as usize]).collect::<Vec<_>>());
    let bs: Vec<f64> = (0..NB).map(|_| rs(num, &mut rng) / rs(den, &mut rng)).collect();
    let (lo, hi) = pct(bs);
    Ci { est: mean(num) / mean(den), lo, hi }
}
fn f3(c: Ci) -> String {
    format!("{:.3} [{:.3}, {:.3}]", c.est, c.lo, c.hi)
}
fn classify(r: Ci, delta: f64) -> &'static str {
    if !r.est.is_finite() { "NO_DATA" }
    else if r.lo > 1.0 + delta { "GAIN" }
    else if r.hi < 1.0 / (1.0 + delta) { "LOSS" }
    else if r.lo >= 1.0 / (1.0 + delta) && r.hi <= 1.0 + delta { "PARITY" }
    else { "UNRESOLVED" }
}

pub fn analyze(dir: &str) -> Result<(), String> {
    let cells = load(dir)?;
    if cells.is_empty() {
        return Err("no rows_*.tsv found".into());
    }
    let mut md = String::new();
    let mut tsv = String::from("a\tn\tm\tl\tarm\thalf\treps\tmetric\tmean\tlo\thi\n");
    let mut verdicts = String::new();
    let mut fig: Vec<(String, usize, Ci, Ci)> = vec![]; // label, m, R_best, R_blind
    let metrics = ["nx", "nitems", "ncls", "lam_pred", "mean_d", "fano_d", "mean_o", "fano_o", "hit_d", "hit_pois_o", "collapse",
        "active", "rank_final", "t90", "cpr_blind", "cpr_best", "deg", "param_deg", "anf_terms", "closure", "enum_ms"];
    writeln!(md, "# G4 iota-stable bases: run summary\n\nDirectory: `{dir}`. Every number below is computed by the fixed analysis program from the raw rows; intervals are 95% percentile bootstrap over bases (2000 resamples, seeded).\nThe decision rule is PROTOCOL.md section 7. Nothing here is a solver result or a rho comparison.\n").unwrap();
    writeln!(md, "## 1. Instrument and control checks (exact)\n\n| cell (a,n,m,l) | stable arms ident_ok / quot_ok / closure=1 | non-stable arms max closure | collapse g4rand | collapse vrand | collapse rset | collapse wrong | status |\n|---|---|---|---|---|---|---|---|").unwrap();
    let mut body = String::new();
    for (k, rows) in &cells {
        let (a, n, m, l) = *k;
        let lab = format!("a{a} n{n} m{m} l{l}");
        for arm in crate::run::ARMS {
            for half in ["main", "aa"] {
                for me in metrics {
                    let v = col(rows, arm, half, me);
                    if v.is_empty() { continue; }
                    let c = boot_mean(&v, 7);
                    writeln!(tsv, "{a}\t{n}\t{m}\t{l}\t{arm}\t{half}\t{}\t{me}\t{}\t{}\t{}", v.len(), c.est, c.lo, c.hi).unwrap();
                }
            }
        }
        // instrument checks
        let stable_arms = ["g4poly", "g4rand", "riota"];
        let mut ident_all = true;
        let mut quot_all = true;
        let mut clos_all = true;
        for arm in stable_arms {
            for half in ["main", "aa"] {
                for r in rows.iter().filter(|r| r["arm"] == arm && r["half"] == half) {
                    ident_all &= r["ident_ok"] == "1";
                    quot_all &= r["quot_ok"] == "1";
                    clos_all &= r["closure"].parse::<f64>().unwrap_or(0.0) == 1.0;
                }
            }
        }
        let max_clos = ["vpoly", "vrand", "rset", "wrong"].iter().flat_map(|a| col(rows, a, "main", "closure")).fold(0.0f64, f64::max);
        let cm = |arm: &str| mean(&col(rows, arm, "main", "collapse"));
        let want = (1u64 << (m - 1)) as f64;
        let ctl_ok = (cm("g4rand") - want).abs() < 1e-9 && cm("vrand") <= 1.02 && cm("rset") <= 1.02 && cm("wrong") <= 1.10;
        let status = if !(ident_all && quot_all && clos_all) { "INSTRUMENT_FAIL" } else if !ctl_ok { "CONTROL_FAIL" } else { "ok" };
        writeln!(md, "| {lab} | {ident_all}/{quot_all}/{clos_all} | {max_clos:.3} | {:.4} (want {want}) | {:.4} | {:.4} | {:.4} | {status} |", cm("g4rand"), cm("vrand"), cm("rset"), cm("wrong")).unwrap();

        // per-arm metrics
        writeln!(body, "### {lab}  (|G| targets = {})\n\n| arm | reps | nx | lam_pred | mean D | Fano D | mean orbits | Fano orbits | hit rate | Poisson hit (orbit mean) | active cols | t90 targets | cpr_blind | cpr_best | ANF deg | censored |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|",
            rows.iter().find(|r| r["arm"] == "vrand").map(|r| r["ntargets_g"].clone()).unwrap_or_default()).unwrap();
        for arm in crate::run::ARMS {
            let nrep = count(rows, arm, "main");
            if nrep == 0 { continue; }
            let g = |me: &str| {
                let v = col(rows, arm, "main", me);
                if v.is_empty() { "NA".to_string() } else { let c = boot_mean(&v, 11); format!("{:.4} [{:.4}, {:.4}]", c.est, c.lo, c.hi) }
            };
            let cens = nrep - col(rows, arm, "main", "cpr_blind").len();
            writeln!(body, "| {arm} | {nrep} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {cens} |",
                g("nx"), g("lam_pred"), g("mean_d"), g("fano_d"), g("mean_o"), g("fano_o"), g("hit_d"), g("hit_pois_o"), g("active"), g("t90"), g("cpr_blind"), g("cpr_best"), g("deg")).unwrap();
        }
        writeln!(body).unwrap();

        // decision rule: relation economics (search-op cost per independent relation)
        let vb = col(rows, "vrand", "main", "cpr_blind");
        let gb = col(rows, "g4rand", "main", "cpr_blind");
        let gbest = col(rows, "g4rand", "main", "cpr_best");
        let r_blind = boot_ratio(&vb, &gb, 21);
        let r_best = boot_ratio(&vb, &gbest, 22);
        // A/A noise floors: same arm, disjoint seed halves, same estimand
        let aa_v = boot_ratio(&vb, &col(rows, "vrand", "aa", "cpr_blind"), 23);
        let aa_g = boot_ratio(&gbest, &col(rows, "g4rand", "aa", "cpr_best"), 24);
        let hw = |c: Ci| if c.est.is_finite() { (c.hi - 1.0).max(1.0 - c.lo) } else { 0.0 };
        let delta = 0.05f64.max(hw(aa_v)).max(hw(aa_g));
        let cens_frac = |arm: &str, me: &str| {
            let n = count(rows, arm, "main");
            if n == 0 { 1.0 } else { 1.0 - col(rows, arm, "main", me).len() as f64 / n as f64 }
        };
        let censored = cens_frac("vrand", "cpr_blind") > 0.2 || cens_frac("g4rand", "cpr_best") > 0.2;
        let primary = n >= 13 && a == 0;
        let size_ratio = mean(&col(rows, "g4rand", "main", "nitems")) / mean(&col(rows, "vrand", "main", "nitems"));
        let col_ratio = mean(&col(rows, "g4rand", "main", "active")) / mean(&col(rows, "vrand", "main", "active"));
        let unmatched = !(0.75..=1.33).contains(&size_ratio);
        let (vc, gc) = if unmatched { ("UNMATCHED".to_string(), "UNMATCHED".to_string()) } else if status != "ok" { ("VOID".to_string(), "VOID".to_string()) } else if censored { ("CENSORED".into(), "CENSORED".into()) } else { (classify(r_best, delta).to_string(), classify(r_blind, delta).to_string()) };
        writeln!(verdicts, "| {lab} | {} | {size_ratio:.2} | {col_ratio:.2} | {} | {} | {} | {} | {delta:.3} | {} | {} | {} |", if primary { "primary" } else { "secondary" },
            f3(r_best), vc, f3(r_blind), gc, f3(boot_mean(&col(rows, "g4rand", "main", "fano_o"), 31)), f3(boot_mean(&col(rows, "vrand", "main", "fano_d"), 32)),
            format!("ANF {:.1} vs {:.1}; param {:.1} vs {:.1}", mean(&col(rows, "g4rand", "main", "deg")), mean(&col(rows, "vrand", "main", "deg")), mean(&col(rows, "g4rand", "main", "param_deg")), mean(&col(rows, "vrand", "main", "param_deg")))).unwrap();
        fig.push((lab, m, r_best, r_blind));
    }
    writeln!(md, "\nstatus ok requires: every stable-arm base satisfies D = 2^(m-1) O at every target, the quotient search equals the orbit count, closure = 1; collapse(g4rand) = 2^(m-1) exactly (positive control), collapse of vrand and rset <= 1.02 and of the wrong-involution control <= 1.10 (negative controls).\n").unwrap();
    writeln!(md, "## 2. Decision table (search-op cost per independent relation to 90% rank; V = subspace, G = iota-stable)\n\nR_best = cpr_blind(V) / cpr_best(G): V searched blind, G searched with the quotient (symmetry-aware). R_blind = cpr_blind(V) / cpr_blind(G): the same blind search code on both. R > 1 means G is cheaper per independent relation. delta = max(0.05, A/A half-width). GAIN / LOSS / PARITY / UNRESOLVED as defined in PROTOCOL.md section 7; CENSORED if more than 20% of bases never reached 90% rank; VOID if the instrument or control checks failed; UNMATCHED if the mean item counts of G and V differ by more than a factor 4/3. Model costs are MITM probe counts, not solver conflicts.\n\n| cell | tier | items G/V | active columns G/V | R_best (95% CI) | class | R_blind (95% CI) | class | delta | Fano of orbits, G | Fano of D, V | membership degree G vs V |\n|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    md.push_str(&verdicts);
    writeln!(md, "\n## 3. Per-cell arm tables (mean over bases, 95% CI)\n").unwrap();
    md.push_str(&body);
    writeln!(md, "## 4. What this does not establish\n\nNo SAT, Groebner or other solver was run: the membership-system price is reported as the exact ANF degree of the indicator only. The cost columns are meet-in-the-middle probe counts of a model search, not measured solver work, and no matched Pollard rho reference or section 5a pricing band is attached, so no row here may enter the IC leaderboard or benchmarks/README.md. A GAIN class means only that the relation-collection economics on the toy field favour the iota-stable base before the membership price; it is not an improvement over the index-calculus baseline.").unwrap();
    std::fs::write(format!("{dir}/summary.md"), md).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/summary.tsv"), tsv).map_err(|e| e.to_string())?;
    std::fs::write(format!("{dir}/fig_cost_ratio.svg"), svg(&fig)).map_err(|e| e.to_string())?;
    Ok(())
}

fn svg(fig: &[(String, usize, Ci, Ci)]) -> String {
    let (w, h, ml, mb) = (900.0, 420.0, 60.0, 90.0);
    let n = fig.len().max(1) as f64;
    let vals: Vec<f64> = fig.iter().flat_map(|f| [f.2.lo, f.2.hi, f.3.lo, f.3.hi]).filter(|v| v.is_finite() && *v > 0.0).map(|v| v.log2()).collect();
    let (lo, hi) = (vals.iter().cloned().fold(-1.0, f64::min), vals.iter().cloned().fold(1.0, f64::max));
    let y = |v: f64| 20.0 + (h - mb - 20.0) * (hi - v.log2()) / (hi - lo);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='10'><rect width='100%' height='100%' fill='white'/>");
    write!(s, "<text x='{ml}' y='12'>log2 of cost ratio cpr(V)/cpr(G); above 0 = iota-stable base cheaper per independent relation. Blue: quotient-aware search on G (R_best). Orange: same blind search (R_blind). Bars: 95% bootstrap CI over bases.</text>").unwrap();
    let y1 = y(1.0);
    write!(s, "<line x1='{ml}' x2='{}' y1='{y1}' y2='{y1}' stroke='#888'/>", w - 10.0).unwrap();
    for (i, (lab, _m, rb, rl)) in fig.iter().enumerate() {
        let x = ml + (w - ml - 10.0) * (i as f64 + 0.5) / n;
        for (c, col, dx) in [(rb, "#1f5fa8", -3.0), (rl, "#d9822b", 3.0)] {
            if c.est.is_finite() && c.est > 0.0 && c.lo > 0.0 && c.hi > 0.0 {
                write!(s, "<line x1='{0}' x2='{0}' y1='{1}' y2='{2}' stroke='{col}'/><circle cx='{0}' cy='{3}' r='2.5' fill='{col}'/>", x + dx, y(c.lo), y(c.hi), y(c.est)).unwrap();
            }
        }
        write!(s, "<text transform='translate({x},{}) rotate(60)' font-size='8'>{lab}</text>", h - mb + 8.0).unwrap();
    }
    s.push_str("</svg>");
    s
}

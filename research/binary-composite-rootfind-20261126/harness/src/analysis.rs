//! Frozen analysis (PROTOCOL.md "Analysis"): paired IC/rho ratios per rung
//! with a hierarchical bootstrap, root-finder effect F/G and F/F0, exponent
//! fits, the §5a band, and the SVG figure.
use crate::seeds::Sm64;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ARMS: [&str; 5] = ["F", "F0", "G", "M3", "M2"];
const KEYS: [&str; 5] = ["meq_s1_u1", "meq_s1_u01", "meq_s1_u0", "meq_s025_u1", "meq_s025_u0"];

#[derive(Default, Clone)]
struct CaseData {
    ell: f64,
    rho: Vec<BTreeMap<String, f64>>, // per seed: key -> cost
    rho_wall: Vec<f64>,
    ic: BTreeMap<String, BTreeMap<String, f64>>, // arm -> key -> cost
    ic_wall: BTreeMap<String, f64>,
}

fn gmean(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn pct(v: &mut Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = ((v.len() - 1) as f64 * p).round() as usize;
    v[k]
}

/// rung -> curve -> target -> case
type Tree = BTreeMap<u32, BTreeMap<u32, BTreeMap<u32, CaseData>>>;

fn ratio_stat(
    tree_r: &BTreeMap<u32, BTreeMap<u32, CaseData>>,
    num: &dyn Fn(&CaseData) -> Option<f64>,
    den: &dyn Fn(&CaseData, Option<&[usize]>) -> Option<f64>,
    rng: &mut Sm64,
    b: usize,
) -> Option<(f64, f64, f64, usize)> {
    let mut pts = vec![];
    for (_, ts) in tree_r {
        for (_, cd) in ts {
            if let (Some(a), Some(d)) = (num(cd), den(cd, None)) {
                pts.push(a / d);
            }
        }
    }
    if pts.is_empty() {
        return None;
    }
    let point = gmean(&pts);
    let curves: Vec<&BTreeMap<u32, CaseData>> = tree_r.values().collect();
    let mut boots = Vec::with_capacity(b);
    for _ in 0..b {
        let mut v = vec![];
        for _ in 0..curves.len() {
            let cv = curves[rng.below(curves.len() as u64) as usize];
            let ts: Vec<&CaseData> = cv.values().collect();
            for _ in 0..ts.len() {
                let cd = ts[rng.below(ts.len() as u64) as usize];
                let ns = cd.rho.len();
                let idx: Vec<usize> = (0..ns).map(|_| rng.below(ns.max(1) as u64) as usize).collect();
                if let (Some(a), Some(d)) = (num(cd), den(cd, Some(&idx))) {
                    v.push(a / d);
                }
            }
        }
        if !v.is_empty() {
            boots.push(gmean(&v));
        }
    }
    Some((point, pct(&mut boots.clone(), 0.025), pct(&mut boots, 0.975), pts.len()))
}

fn rho_mean(cd: &CaseData, key: &str, idx: Option<&[usize]>) -> Option<f64> {
    if cd.rho.is_empty() {
        return None;
    }
    let v: Vec<f64> = match idx {
        None => cd.rho.iter().map(|r| r[key]).collect(),
        Some(ix) => ix.iter().map(|&i| cd.rho[i][key]).collect(),
    };
    Some(v.iter().sum::<f64>() / v.len() as f64)
}

fn ols(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    sxy / sxx
}

pub fn analyze(_protocol: &str, runs: &str, out: &str, svg: &str) {
    let text = std::fs::read_to_string(runs).unwrap();
    let mut tree: Tree = BTreeMap::new();
    let (mut n_rec, mut n_bad, mut mismatches, mut unresolved) = (0u64, 0u64, 0u64, 0u64);
    let mut bad_list = vec![];
    let mut per_arm_counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let v: Value = serde_json::from_str(line).unwrap();
        n_rec += 1;
        let c = &v["case"];
        let (n, ci, ti) = (c["n"].as_u64().unwrap() as u32, c["ci"].as_u64().unwrap() as u32, c["ti"].as_u64().unwrap() as u32);
        let arm = v["arm"].as_str().unwrap().to_string();
        let ok = v["correct"].as_bool() == Some(true) && v["verified"].as_bool() == Some(true);
        let e = per_arm_counts.entry(arm.clone()).or_default();
        e.0 += 1;
        if !ok {
            n_bad += 1;
            e.1 += 1;
            bad_list.push(json!({"n": n, "ci": ci, "ti": ti, "arm": arm, "seed_index": v["seed_index"]}));
        }
        mismatches += v["gcd_set_mismatches"].as_u64().unwrap_or(0);
        unresolved += v["rows_unresolved"].as_u64().unwrap_or(0);
        let cd = tree.entry(n).or_default().entry(ci).or_default().entry(ti).or_default();
        cd.ell = c["curve"]["ell"].as_f64().unwrap();
        let costs: BTreeMap<String, f64> = KEYS.iter().map(|k| (k.to_string(), v["cost"][k].as_f64().unwrap())).collect();
        let wall = v["wall_ns"].as_f64().unwrap_or(0.0);
        if arm == "RHO" {
            // A failed rho run is retained in the record; it is not a reference.
            if ok {
                cd.rho.push(costs);
                cd.rho_wall.push(wall);
            }
        } else if ok {
            cd.ic.insert(arm.clone(), costs);
            cd.ic_wall.insert(arm, wall);
        }
    }
    let mut rng = Sm64::keyed(&["boot", "main"]);
    let bn = 10000;
    let mut rungs = vec![];
    for (&n, tr) in &tree {
        let ell = gmean(&tr.values().flat_map(|t| t.values().map(|c| c.ell)).collect::<Vec<_>>());
        let mut arms = serde_json::Map::new();
        for arm in ARMS {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                let r = ratio_stat(
                    tr,
                    &|cd| cd.ic.get(arm).map(|m| m[key]),
                    &|cd, ix| rho_mean(cd, key, ix),
                    &mut rng,
                    if key == "meq_s1_u1" { bn } else { 2000 },
                );
                if let Some((p, lo, hi, k)) = r {
                    band.insert(key.into(), json!({"ratio": p, "lo": lo, "hi": hi, "cases": k}));
                }
            }
            // wall (indicative), point estimate only
            let mut wr = vec![];
            for t in tr.values() {
                for cd in t.values() {
                    if let (Some(w), false) = (cd.ic_wall.get(arm), cd.rho_wall.is_empty()) {
                        wr.push(w / (cd.rho_wall.iter().sum::<f64>() / cd.rho_wall.len() as f64));
                    }
                }
            }
            let mut s_vals = vec![];
            for t in tr.values() {
                for cd in t.values() {
                    if let Some(m) = cd.ic.get(arm) {
                        s_vals.push(m["meq_s1_u1"] / cd.ell.sqrt());
                    }
                }
            }
            arms.insert(
                arm.into(),
                json!({
                    "vs_rho": band,
                    "wall_ratio_gmean_indicative": if wr.is_empty() { Value::Null } else { json!(gmean(&wr)) },
                    "S_meq_per_sqrt_ell_gmean": if s_vals.is_empty() { Value::Null } else { json!(gmean(&s_vals)) },
                }),
            );
        }
        // rho S
        let mut rs = vec![];
        for t in tr.values() {
            for cd in t.values() {
                if let Some(m) = rho_mean(cd, "meq_s1_u1", None) {
                    rs.push(m / cd.ell.sqrt());
                }
            }
        }
        // root-finder effect
        let mut eff = serde_json::Map::new();
        for (lab, a, bb) in [("F_over_G", "F", "G"), ("F_over_F0", "F", "F0"), ("M3_over_F", "M3", "F"), ("M2_over_F", "M2", "F")] {
            let mut band = serde_json::Map::new();
            for key in KEYS {
                if let Some((p, lo, hi, k)) = ratio_stat(
                    tr,
                    &|cd| cd.ic.get(a).map(|m| m[key]),
                    &|cd, _| cd.ic.get(bb).map(|m| m[key]),
                    &mut rng,
                    2000,
                ) {
                    band.insert(key.into(), json!({"ratio": p, "lo": lo, "hi": hi, "cases": k}));
                }
            }
            eff.insert(lab.into(), Value::Object(band));
        }
        rungs.push(json!({
            "n": n, "np": n / 3, "ell_gmean": ell, "log2_ell": ell.log2(),
            "curves": tr.len(), "cases": tr.values().map(|t| t.len()).sum::<usize>(),
            "rho_S_meq_gmean": if rs.is_empty() { Value::Null } else { json!(gmean(&rs)) },
            "arms": arms, "effects": eff,
        }));
    }
    // exponent fits (primary key)
    let key = "meq_s1_u1";
    let collect = |arm: &str, tree: &Tree, rng: Option<&mut Sm64>| -> (Vec<f64>, Vec<f64>) {
        let mut xs = vec![];
        let mut ys = vec![];
        let mut rng = rng;
        for tr in tree.values() {
            let cases: Vec<&CaseData> = tr.values().flat_map(|t| t.values()).collect();
            let picks: Vec<&CaseData> = match rng.as_mut() {
                None => cases.clone(),
                Some(r) => (0..cases.len()).map(|_| cases[r.below(cases.len() as u64) as usize]).collect(),
            };
            for cd in picks {
                let y = if arm == "RHO" { rho_mean(cd, key, None) } else { cd.ic.get(arm).map(|m| m[key]) };
                if let Some(y) = y {
                    xs.push(cd.ell.log2());
                    ys.push(y.log2());
                }
            }
        }
        (xs, ys)
    };
    let mut fits = serde_json::Map::new();
    for arm in ["RHO", "F", "F0", "G", "M3", "M2"] {
        let (xs, ys) = collect(arm, &tree, None);
        if xs.len() < 3 {
            continue;
        }
        let slope = ols(&xs, &ys);
        let (rx, ry) = collect("RHO", &tree, None);
        let rslope = ols(&rx, &ry);
        let mut bs = vec![];
        let mut bd = vec![];
        let mut brng = Sm64::keyed(&["boot", "slope", arm]);
        for _ in 0..2000 {
            // same resample of cases for both arms
            let st = brng.0;
            let (xa, ya) = collect(arm, &tree, Some(&mut brng));
            let mut r2 = Sm64(st);
            let (xb, yb) = collect("RHO", &tree, Some(&mut r2));
            let sa = ols(&xa, &ya);
            bs.push(sa);
            bd.push(sa - ols(&xb, &yb));
        }
        fits.insert(
            arm.into(),
            json!({"slope": slope, "slope_lo": pct(&mut bs.clone(), 0.025), "slope_hi": pct(&mut bs, 0.975),
                   "minus_rho": slope - rslope, "minus_rho_lo": pct(&mut bd.clone(), 0.025), "minus_rho_hi": pct(&mut bd, 0.975),
                   "points": xs.len()}),
        );
    }
    let summary = json!({
        "schema": "bcr-summary/v1", "records": n_rec, "records_failed": n_bad, "failed": bad_list,
        "per_arm_records_and_failures": per_arm_counts.iter().map(|(k, v)| (k.clone(), json!({"records": v.0, "failed": v.1}))).collect::<serde_json::Map<_,_>>(),
        "gcd_set_mismatches": mismatches, "rows_unresolved": unresolved,
        "bootstrap": {"B_primary": bn, "B_band_and_effects": 2000, "B_slopes": 2000, "stream": "boot|main, boot|slope|<arm>"},
        "rungs": rungs, "exponent_fits_log2cost_vs_log2ell": fits,
    });
    std::fs::write(out, serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    std::fs::write(svg, figure(&summary)).unwrap();
    println!("{}", table(&summary));
}

fn table(s: &Value) -> String {
    let mut o = String::new();
    o += "| n | n' | log2 ell | cases | rho S | F/rho [95% CI] | G/rho | F0/rho | M3/rho | M2/rho | F/G |\n|---|---|---|---|---|---|---|---|---|---|---|\n";
    for r in s["rungs"].as_array().unwrap() {
        let f = |arm: &str| {
            let x = &r["arms"][arm]["vs_rho"]["meq_s1_u1"];
            if x.is_null() {
                "—".to_string()
            } else {
                format!("{:.3e} [{:.3e}, {:.3e}]", x["ratio"].as_f64().unwrap(), x["lo"].as_f64().unwrap(), x["hi"].as_f64().unwrap())
            }
        };
        let e = &r["effects"]["F_over_G"]["meq_s1_u1"];
        let fg = if e.is_null() { "—".into() } else { format!("{:.3} [{:.3}, {:.3}]", e["ratio"].as_f64().unwrap(), e["lo"].as_f64().unwrap(), e["hi"].as_f64().unwrap()) };
        o += &format!(
            "| {} | {} | {:.1} | {} | {:.2} | {} | {} | {} | {} | {} | {} |\n",
            r["n"], r["np"], r["log2_ell"].as_f64().unwrap(), r["cases"], r["rho_S_meq_gmean"].as_f64().unwrap_or(f64::NAN),
            f("F"), f("G"), f("F0"), f("M3"), f("M2"), fg
        );
    }
    o
}

fn figure(s: &Value) -> String {
    // Two panels: (a) paired IC/rho ratio per rung (log scale), (b) F/G and F/F0.
    let (w, h) = (980.0, 420.0);
    let rungs = s["rungs"].as_array().unwrap();
    let ns: Vec<f64> = rungs.iter().map(|r| r["n"].as_f64().unwrap()).collect();
    let (xmin, xmax) = (ns[0] - 1.5, ns[ns.len() - 1] + 1.5);
    let mut svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n"
    );
    let colors = [("F", "#1f6fb4"), ("G", "#c0392b"), ("F0", "#7f8c8d"), ("M3", "#27ae60"), ("M2", "#8e44ad")];
    // panel a
    let (px, py, pw, ph) = (70.0, 40.0, 520.0, 320.0);
    let (ymin, ymax) = (0.0f64, 7.0f64); // log10 ratio
    let sx = |n: f64| px + (n - xmin) / (xmax - xmin) * pw;
    let sy = |l: f64| py + ph - (l - ymin) / (ymax - ymin) * ph;
    svg += &format!("<text x='{}' y='22' font-size='14' font-weight='bold'>(a) IC cost / matched negation-map rho (paired, Meq, 95% CI)</text>\n", px - 40.0);
    for k in 0..=7 {
        let y = sy(k as f64);
        svg += &format!("<line x1='{px}' y1='{y}' x2='{}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>1e{k}</text>\n", px + pw, px - 6.0, y + 4.0);
    }
    svg += &format!("<line x1='{px}' y1='{}' x2='{}' y2='{}' stroke='black' stroke-width='1.5' stroke-dasharray='6 3'/><text x='{}' y='{}'>parity (ratio 1)</text>\n", sy(0.0), px + pw, sy(0.0), px + pw - 95.0, sy(0.0) - 5.0);
    for &n in &ns {
        svg += &format!("<text x='{}' y='{}' text-anchor='middle'>{}</text>\n", sx(n), py + ph + 18.0, n);
    }
    svg += &format!("<text x='{}' y='{}' text-anchor='middle'>n  (field F_2^n, subfield F_2^(n/3))</text>\n", px + pw / 2.0, py + ph + 36.0);
    for (k, (arm, col)) in colors.iter().enumerate() {
        let off = (k as f64 - 2.0) * 0.18;
        let mut path = String::new();
        for r in rungs {
            let x = &r["arms"][*arm]["vs_rho"]["meq_s1_u1"];
            if x.is_null() {
                continue;
            }
            let n = r["n"].as_f64().unwrap() + off;
            let (p, lo, hi) = (x["ratio"].as_f64().unwrap().log10(), x["lo"].as_f64().unwrap().log10(), x["hi"].as_f64().unwrap().log10());
            svg += &format!("<line x1='{0}' y1='{1}' x2='{0}' y2='{2}' stroke='{col}' stroke-width='2'/>\n", sx(n), sy(lo), sy(hi));
            svg += &format!("<circle cx='{}' cy='{}' r='3.5' fill='{col}'/>\n", sx(n), sy(p));
            path += &format!("{}{:.1},{:.1} ", if path.is_empty() { "M" } else { "L" }, sx(n), sy(p));
        }
        svg += &format!("<path d='{path}' fill='none' stroke='{col}' stroke-width='1.2' opacity='0.7'/>\n");
        let ly = py + 12.0 + k as f64 * 16.0;
        let label = match *arm {
            "F" => "F: m=3 pairs-and-solve, factored S4 root finder (tuned)",
            "G" => "G: m=3 pairs-and-solve, subspace gcd (before)",
            "F0" => "F0: factored, no pair table",
            "M3" => "M3: m=3 meet-in-the-middle (u-table)",
            _ => "M2: m=2 meet-in-the-middle (u-table)",
        };
        svg += &format!("<rect x='{}' y='{}' width='10' height='10' fill='{col}'/><text x='{}' y='{}' font-size='11'>{label}</text>\n", px + 8.0, ly - 9.0, px + 22.0, ly);
    }
    svg += &format!("<rect x='{px}' y='{py}' width='{pw}' height='{ph}' fill='none' stroke='black'/>\n");
    // panel b
    let (qx, qy, qw, qh) = (680.0, 40.0, 270.0, 320.0);
    let (bmin, bmax) = (0.0f64, 1.2f64);
    let tx = |n: f64| qx + (n - xmin) / (xmax - xmin) * qw;
    let ty = |v: f64| qy + qh - (v - bmin) / (bmax - bmin) * qh;
    svg += &format!("<text x='{}' y='22' font-size='14' font-weight='bold'>(b) root-finder effect on the whole solve</text>\n", qx - 30.0);
    for k in 0..=6 {
        let v = k as f64 * 0.2;
        svg += &format!("<line x1='{qx}' y1='{0}' x2='{1}' y2='{0}' stroke='#ddd'/><text x='{2}' y='{3}' text-anchor='end'>{v:.1}</text>\n", ty(v), qx + qw, qx - 6.0, ty(v) + 4.0);
    }
    svg += &format!("<line x1='{qx}' y1='{0}' x2='{1}' y2='{0}' stroke='black' stroke-dasharray='6 3'/>\n", ty(1.0), qx + qw);
    for &n in &ns {
        svg += &format!("<text x='{}' y='{}' text-anchor='middle'>{}</text>\n", tx(n), qy + qh + 18.0, n);
    }
    for (k, (lab, col, name)) in [("F_over_G", "#1f6fb4", "F / G"), ("F_over_F0", "#7f8c8d", "F / F0")].iter().enumerate() {
        let off = (k as f64 - 0.5) * 0.3;
        for r in rungs {
            let x = &r["effects"][*lab]["meq_s1_u1"];
            if x.is_null() {
                continue;
            }
            let n = r["n"].as_f64().unwrap() + off;
            svg += &format!(
                "<line x1='{0}' y1='{1}' x2='{0}' y2='{2}' stroke='{col}' stroke-width='2'/><circle cx='{0}' cy='{3}' r='3.5' fill='{col}'/>\n",
                tx(n), ty(x["lo"].as_f64().unwrap()), ty(x["hi"].as_f64().unwrap()), ty(x["ratio"].as_f64().unwrap())
            );
        }
        svg += &format!("<rect x='{}' y='{}' width='10' height='10' fill='{col}'/><text x='{}' y='{}' font-size='11'>{name} (paired, same cases)</text>\n", qx + 8.0, qy + 3.0 + k as f64 * 16.0, qx + 22.0, qy + 12.0 + k as f64 * 16.0);
    }
    svg += &format!("<rect x='{qx}' y='{qy}' width='{qw}' height='{qh}' fill='none' stroke='black'/>\n");
    svg += &format!("<text x='20' y='{}' font-size='10' fill='#555'>bcr-20261126/v1 · random curves y²+xy=x³+ax²+b over F_2^n, b in no proper subfield, cofactor 2 or 4 · subfield factor base F_2^(n/3) · every log verified [d]G=Q · [pcc:unverified], L0, shared host</text>\n", h - 12.0);
    svg += "</svg>\n";
    svg
}

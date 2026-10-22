//! Analysis of `research/cheon_p224_shape_20261022/` (see `PROTOCOL.md`,
//! section 7).  Reads recorded values only: `curves.json`, the Cheon runs
//! (`runs.jsonl`) and the ecbench rho session(s) (`records.jsonl`).  Writes
//! `results.md` (tables and fits), `results.json` and the SVG figures.
//!
//! ```text
//! cheon_p224_shape_analyse --curves curves.json --runs runs.jsonl \
//!     --rho SESSION/records.jsonl [--rho ...] --out DIR
//! ```

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt::Write as _;

const LOG2_N_P224: f64 = 224.000_000_17;
const LOG2_D_MINUS: f64 = 28.991_298_55;
const LOG2_D_PLUS: f64 = 47.278_406_86;
const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

#[derive(Clone, Debug)]
struct Obs {
    t: u64,
    ops: f64,
    unc: f64,
    correct: bool,
    phases: BTreeMap<String, f64>,
    uncharged: BTreeMap<String, f64>,
    leaked: u64,
    note: String,
}

#[derive(Clone, Debug)]
struct Cell {
    fam: String,
    bits: u64,
    n: u64,
    d: u64,
}

fn arg_all(args: &[String], name: &str) -> Vec<String> {
    args.iter()
        .enumerate()
        .filter(|(_, a)| *a == name)
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect()
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len();
    if m == 0 {
        return f64::NAN;
    }
    if m % 2 == 1 {
        v[m / 2]
    } else {
        (v[m / 2 - 1] + v[m / 2]) / 2.0
    }
}

fn t95(dof: usize) -> f64 {
    const T: [f64; 12] = [
        12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179,
    ];
    if dof == 0 {
        f64::NAN
    } else if dof <= 12 {
        T[dof - 1]
    } else {
        1.96
    }
}

#[derive(Clone, Copy, Debug)]
struct Fit {
    a: f64,
    b: f64,
    se_b: f64,
    s: f64,
    k: usize,
    xbar: f64,
    sxx: f64,
}

fn ols(x: &[f64], y: &[f64]) -> Option<Fit> {
    let k = x.len();
    if k < 3 {
        return None;
    }
    let xbar = x.iter().sum::<f64>() / k as f64;
    let ybar = y.iter().sum::<f64>() / k as f64;
    let sxx: f64 = x.iter().map(|v| (v - xbar).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - xbar) * (b - ybar)).sum();
    let b = sxy / sxx;
    let a = ybar - b * xbar;
    let rss: f64 = x
        .iter()
        .zip(y)
        .map(|(xi, yi)| (yi - a - b * xi).powi(2))
        .sum();
    let s = (rss / (k as f64 - 2.0)).sqrt();
    Some(Fit {
        a,
        b,
        se_b: s / sxx.sqrt(),
        s,
        k,
        xbar,
        sxx,
    })
}

impl Fit {
    fn at(&self, x: f64) -> f64 {
        self.a + self.b * x
    }
    fn slope_ci(&self) -> (f64, f64) {
        let t = t95(self.k - 2);
        (self.b - t * self.se_b, self.b + t * self.se_b)
    }
    fn pred_ci(&self, x0: f64) -> (f64, f64) {
        let t = t95(self.k - 2);
        let h = self.s * (1.0 + 1.0 / self.k as f64 + (x0 - self.xbar).powi(2) / self.sxx).sqrt();
        (self.at(x0) - t * h, self.at(x0) + t * h)
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() {
        return f64::NAN;
    }
    v[((v.len() - 1) as f64 * q).round() as usize]
}

type Data = BTreeMap<(String, u64), BTreeMap<String, Vec<Obs>>>; // (fam,bits) -> arm -> obs

fn read_lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or(f64::NAN)
}

fn fmt_n(x: f64) -> String {
    let s = format!("{:.0}", x);
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let curves_path = arg_all(&args, "--curves").pop().expect("--curves");
    let runs_path = arg_all(&args, "--runs").pop().expect("--runs");
    let rho_paths = arg_all(&args, "--rho");
    let out_dir = arg_all(&args, "--out").pop().expect("--out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let cv: Value = serde_json::from_str(&std::fs::read_to_string(&curves_path).unwrap()).unwrap();
    let cells: Vec<Cell> = cv["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| Cell {
            fam: c["family"].as_str().unwrap().into(),
            bits: c["bits"].as_u64().unwrap(),
            n: c["n"].as_u64().unwrap(),
            d: c["d"].as_u64().unwrap(),
        })
        .collect();
    let by_n: BTreeMap<u64, &Cell> = cells.iter().map(|c| (c.n, c)).collect();

    let mut data: Data = BTreeMap::new();
    let mut attempts: BTreeMap<(String, u64, String), (usize, usize, usize)> = BTreeMap::new();
    for r in read_lines(&runs_path) {
        let fam = r["family"].as_str().unwrap().to_string();
        let bits = r["bits"].as_u64().unwrap();
        let arm = r["arm"].as_str().unwrap().to_string();
        let correct = r["correct"].as_bool().unwrap_or(false);
        let timed = r["timed_out"].as_bool().unwrap_or(false);
        let e = attempts
            .entry((fam.clone(), bits, arm.clone()))
            .or_default();
        e.0 += 1;
        e.1 += correct as usize;
        e.2 += timed as usize;
        let phases: BTreeMap<String, f64> = r["phases"]
            .as_object()
            .map(|o| o.iter().map(|(k, v)| (k.clone(), f(v))).collect())
            .unwrap_or_default();
        let uncharged: BTreeMap<String, f64> = r["uncharged"]
            .as_object()
            .map(|o| o.iter().map(|(k, v)| (k.clone(), f(v))).collect())
            .unwrap_or_default();
        let unc = uncharged.values().sum();
        data.entry((fam, bits))
            .or_default()
            .entry(arm)
            .or_default()
            .push(Obs {
                t: r["target"].as_u64().unwrap(),
                ops: f(&r["ops"]),
                unc,
                correct,
                phases,
                uncharged,
                leaked: r["leaked_powers"].as_u64().unwrap_or(0),
                note: r["error"].as_str().unwrap_or("").to_string(),
            });
    }
    for p in &rho_paths {
        for r in read_lines(p) {
            let n = r["workload"]["curve"]["r"].as_u64().unwrap();
            let Some(c) = by_n.get(&n) else { continue };
            let status = r["outcome"]["status"].as_str().unwrap_or("");
            let counters = r["counters"].as_object();
            let uncharged: BTreeMap<String, f64> = counters
                .map(|o| {
                    o.iter()
                        .filter(|(k, _)| k.ends_with("_uncharged"))
                        .map(|(k, v)| (k.clone(), f(v)))
                        .collect()
                })
                .unwrap_or_default();
            let unc = uncharged.values().sum();
            let correct = status == "verified" && r["outcome"]["matches_planted"] == true;
            let e = attempts
                .entry((c.fam.clone(), c.bits, "rho.negation".into()))
                .or_default();
            e.0 += 1;
            e.1 += correct as usize;
            let phases = BTreeMap::new();
            data.entry((c.fam.clone(), c.bits))
                .or_default()
                .entry("rho.negation".into())
                .or_default()
                .push(Obs {
                    t: r["workload"]["target_index"].as_u64().unwrap(),
                    ops: f(&r["cost"]["total_gae"]),
                    unc,
                    correct,
                    phases,
                    uncharged,
                    leaked: 0,
                    note: status.to_string(),
                });
        }
    }

    let mut md = String::new();
    let mut js = json!({});
    let arms_of = |fam: &str| -> Vec<&'static str> {
        if fam == "minus" {
            vec!["rho.negation", "cheon.pminus1_comb", "cheon.pminus1_naive"]
        } else {
            vec!["rho.negation", "cheon.pplus1_comb"]
        }
    };
    let ver = |fam: &str, bits: u64, arm: &str| -> Vec<Obs> {
        data.get(&(fam.to_string(), bits))
            .and_then(|m| m.get(arm))
            .map(|v| v.iter().filter(|o| o.correct).cloned().collect())
            .unwrap_or_default()
    };

    // ── Table 1: observations ────────────────────────────────────────────
    writeln!(md, "### Table 1. Measured operation counts, every run\n").unwrap();
    writeln!(md, "Unit: group operations (one affine addition or doubling = 1), every phase charged. `S = median ops / sqrt(n)`. `F(n,d)` is the formula `sqrt(n/d)+sqrt(d)` (minus) or `sqrt(n/d)+d` (plus). `ratio to rho` is the median over targets of the paired ratio (same curve, same target). `correct` counts recovered logarithms equal to the planted one.\n").unwrap();
    writeln!(md, "| family | bits | n | d | arm | leaked powers | correct | median ops | min | max | S | ops/F | ratio to rho |").unwrap();
    writeln!(md, "|:--|--:|--:|--:|:--|--:|:--|--:|--:|--:|--:|--:|--:|").unwrap();
    let mut rows_js = vec![];
    for c in &cells {
        let rho = ver(&c.fam, c.bits, "rho.negation");
        for arm in arms_of(&c.fam) {
            let all = data
                .get(&(c.fam.clone(), c.bits))
                .and_then(|m| m.get(arm))
                .cloned()
                .unwrap_or_default();
            if all.is_empty() {
                writeln!(
                    md,
                    "| {} | {} | {} | {} | `{}` | | not run | | | | | | |",
                    c.fam, c.bits, c.n, c.d, arm
                )
                .unwrap();
                continue;
            }
            let v: Vec<Obs> = all.iter().filter(|o| o.correct).cloned().collect();
            let mut ops: Vec<f64> = v.iter().map(|o| o.ops).collect();
            let med = median(&mut ops.clone());
            let fnd = if c.fam == "minus" {
                ((c.n as f64) / c.d as f64).sqrt() + (c.d as f64).sqrt()
            } else {
                ((c.n as f64) / c.d as f64).sqrt() + c.d as f64
            };
            let mut ratios = vec![];
            for o in &v {
                if let Some(r) = rho.iter().find(|r| r.t == o.t) {
                    ratios.push(o.ops / r.ops);
                }
            }
            let rr = if arm == "rho.negation" {
                "1 (reference)".to_string()
            } else if ratios.is_empty() {
                "n/a".into()
            } else {
                format!("{:.2}", median(&mut ratios))
            };
            ops.sort_by(|a, b| a.partial_cmp(b).unwrap());
            writeln!(
                md,
                "| {} | {} | {} | {} | `{}` | {} | {}/{} | {} | {} | {} | {:.3} | {:.1} | {} |",
                c.fam,
                c.bits,
                c.n,
                c.d,
                arm,
                all[0].leaked,
                v.len(),
                all.len(),
                fmt_n(med),
                ops.first().map_or("-".into(), |x| fmt_n(*x)),
                ops.last().map_or("-".into(), |x| fmt_n(*x)),
                med / (c.n as f64).sqrt(),
                med / fnd,
                rr
            )
            .unwrap();
            rows_js.push(json!({"family": c.fam, "bits": c.bits, "n": c.n, "d": c.d, "arm": arm,
                "leaked_powers": all[0].leaked, "correct": v.len(), "runs": all.len(),
                "median_ops": med, "S": med/(c.n as f64).sqrt(), "ops_over_F": med/fnd,
                "ratio_to_rho_median": if ratios.is_empty() { Value::Null } else { json!(median(&mut ratios.clone())) }}));
        }
    }
    js["table1"] = json!(rows_js);

    // failures and timeouts, preserved
    writeln!(
        md,
        "\n### Table 1b. Failures, wrong answers and timeouts (preserved)\n"
    )
    .unwrap();
    let mut any = false;
    for ((fam, bits, arm), (runs, ok, to)) in &attempts {
        if ok != runs {
            any = true;
            let notes: Vec<String> = data[&(fam.clone(), *bits)][arm]
                .iter()
                .filter(|o| !o.correct)
                .map(|o| format!("t{}: {}", o.t, o.note))
                .collect();
            writeln!(
                md,
                "- {fam} {bits} bits `{arm}`: {ok}/{runs} correct, {to} timeouts. {}",
                notes.join("; ")
            )
            .unwrap();
        }
    }
    if !any {
        writeln!(
            md,
            "None: every recorded run returned the planted logarithm."
        )
        .unwrap();
    }

    // ── Table 2: unpriced work and the 0 / 0.1 / 1 band ──────────────────
    writeln!(
        md,
        "\n### Table 2. Unpriced work and the pricing band (AGENTS.md 5a)\n"
    )
    .unwrap();
    writeln!(md, "Unpriced counters per arm (median over targets), then the Cheon/rho cost ratio when every unpriced operation is priced at `w` group-addition equivalents, the same `w` for both arms. Order is **resolved** only if the ratio is on the same side of 1 at w = 0, 0.1 and 1.\n").unwrap();
    writeln!(md, "| family | bits | arm | unpriced counters (median) | ratio w=0 | ratio w=0.1 | ratio w=1 | order |").unwrap();
    writeln!(md, "|:--|--:|:--|:--|--:|--:|--:|:--|").unwrap();
    let mut band_js = vec![];
    for c in &cells {
        let rho = ver(&c.fam, c.bits, "rho.negation");
        for arm in arms_of(&c.fam) {
            let v = ver(&c.fam, c.bits, arm);
            if v.is_empty() {
                continue;
            }
            let mut names: BTreeMap<String, Vec<f64>> = BTreeMap::new();
            for o in &v {
                for (k, x) in &o.uncharged {
                    names.entry(k.clone()).or_default().push(*x);
                }
            }
            let desc: Vec<String> = names
                .iter_mut()
                .map(|(k, x)| format!("{k}={}", fmt_n(median(x))))
                .collect();
            let mut cols = vec![];
            let mut sides = vec![];
            for w in WEIGHTS {
                let mut rs = vec![];
                for o in &v {
                    if let Some(r) = rho.iter().find(|r| r.t == o.t) {
                        rs.push((o.ops + w * o.unc) / (r.ops + w * r.unc));
                    }
                }
                let m = median(&mut rs);
                cols.push(m);
                sides.push(m < 1.0);
            }
            let order = if arm == "rho.negation" {
                "reference".to_string()
            } else if sides.iter().all(|s| *s) {
                "cheaper than rho at all weights".into()
            } else if sides.iter().all(|s| !*s) {
                "dearer than rho at all weights".into()
            } else {
                "unresolved".into()
            };
            writeln!(
                md,
                "| {} | {} | `{}` | {} | {:.2} | {:.2} | {:.2} | {} |",
                c.fam,
                c.bits,
                arm,
                desc.join(", "),
                cols[0],
                cols[1],
                cols[2],
                order
            )
            .unwrap();
            band_js.push(json!({"family": c.fam, "bits": c.bits, "arm": arm, "ratios": cols, "order": order}));
        }
    }
    js["band"] = json!(band_js);

    // ── fits ─────────────────────────────────────────────────────────────
    writeln!(
        md,
        "\n### Table 3. Fitted exponents, `log2(ops) = a + b * log2(n)`\n"
    )
    .unwrap();
    writeln!(md, "Ordinary least squares on the per-size mean of `log2(ops)` over verified targets (each size one point). `t 95%` is the Student interval (sizes - 2 degrees of freedom); `boot 95%` is the percentile interval from 2000 resamples of the targets inside every size. Sizes used are listed.\n").unwrap();
    writeln!(md, "| family | arm | sizes | slope b | s.e. | t 95% | boot 95% | prediction (PROTOCOL 5) | inside |").unwrap();
    writeln!(md, "|:--|:--|--:|--:|--:|:--|:--|:--|:--|").unwrap();
    let preds: BTreeMap<(&str, &str), (f64, f64)> = [
        (("minus", "cheon.pminus1_comb"), (0.40, 0.50)),
        (("plus", "cheon.pplus1_comb"), (0.30, 0.46)),
        (("minus", "rho.negation"), (0.45, 0.55)),
        (("plus", "rho.negation"), (0.45, 0.55)),
    ]
    .into_iter()
    .collect();

    struct Series {
        x: Vec<f64>,
        y: Vec<f64>,
        d: Vec<f64>,
        per_target: Vec<BTreeMap<u64, f64>>, // per size: target -> log2 ops
    }
    let series = |fam: &str, arm: &str| -> Series {
        let mut s = Series {
            x: vec![],
            y: vec![],
            d: vec![],
            per_target: vec![],
        };
        for c in cells.iter().filter(|c| c.fam == fam) {
            let v = ver(fam, c.bits, arm);
            let all = data
                .get(&(fam.to_string(), c.bits))
                .and_then(|m| m.get(arm))
                .map_or(0, |v| v.len());
            if v.len() >= 4 && v.len() == all {
                s.x.push((c.n as f64).log2());
                s.y.push(v.iter().map(|o| o.ops.log2()).sum::<f64>() / v.len() as f64);
                s.d.push((c.d as f64).log2());
                s.per_target
                    .push(v.iter().map(|o| (o.t, o.ops.log2())).collect());
            }
        }
        s
    };
    let mut fit_js = json!({});
    let mut fits: BTreeMap<(String, String), (Fit, (f64, f64))> = BTreeMap::new();
    for (fam, arm) in [
        ("minus", "rho.negation"),
        ("minus", "cheon.pminus1_comb"),
        ("minus", "cheon.pminus1_naive"),
        ("plus", "rho.negation"),
        ("plus", "cheon.pplus1_comb"),
    ] {
        let s = series(fam, arm);
        let Some(fit) = ols(&s.x, &s.y) else {
            writeln!(
                md,
                "| {fam} | `{arm}` | {} | no fit (fewer than 3 complete sizes) | | | | | |",
                s.x.len()
            )
            .unwrap();
            continue;
        };
        // bootstrap over targets inside every size
        let mut rng = Rng(0xC4EE_0000 ^ (s.x.len() as u64));
        let mut slopes = vec![];
        let mut ext = vec![];
        for _ in 0..2000 {
            let y: Vec<f64> = s
                .per_target
                .iter()
                .map(|m| {
                    let vals: Vec<f64> = m.values().cloned().collect();
                    let k = vals.len();
                    (0..k)
                        .map(|_| vals[(rng.next() % k as u64) as usize])
                        .sum::<f64>()
                        / k as f64
                })
                .collect();
            if let Some(ft) = ols(&s.x, &y) {
                slopes.push(ft.b);
                ext.push(ft.at(LOG2_N_P224));
            }
        }
        let (tlo, thi) = fit.slope_ci();
        let (blo, bhi) = (pct(&mut slopes.clone(), 0.025), pct(&mut slopes, 0.975));
        let (lo, hi) = preds
            .get(&(fam, arm))
            .copied()
            .unwrap_or((f64::NAN, f64::NAN));
        let inside = if lo.is_nan() {
            "n/a (no prediction)".to_string()
        } else if fit.b >= lo && fit.b <= hi {
            "inside".into()
        } else {
            "OUTSIDE".into()
        };
        writeln!(
            md,
            "| {fam} | `{arm}` | {} | {:.3} | {:.3} | [{:.3}, {:.3}] | [{:.3}, {:.3}] | {} | {} |",
            s.x.len(),
            fit.b,
            fit.se_b,
            tlo,
            thi,
            blo,
            bhi,
            if lo.is_nan() {
                "none".into()
            } else {
                format!("[{lo:.2}, {hi:.2}]")
            },
            inside
        )
        .unwrap();
        let (elo, ehi) = (pct(&mut ext.clone(), 0.025), pct(&mut ext, 0.975));
        fit_js[format!("{fam}/{arm}")] = json!({"a": fit.a, "b": fit.b, "se_b": fit.se_b, "t95": [tlo, thi],
            "boot95": [blo, bhi], "sizes": s.x.len(), "extrap224": fit.at(LOG2_N_P224),
            "extrap224_boot95": [elo, ehi], "extrap224_pred95": fit.pred_ci(LOG2_N_P224)});
        fits.insert((fam.to_string(), arm.to_string()), (fit, (elo, ehi)));
    }

    // ── extrapolation ────────────────────────────────────────────────────
    writeln!(
        md,
        "\n### Table 4. EXTRAPOLATION to log2 n = 224 (not measured)\n"
    )
    .unwrap();
    writeln!(md, "Every figure here is an extrapolation of a fit over the sizes in Table 3, from at most {:.0} bits to 224 bits. Fit A is the free power law of Table 3. Fit B regresses `log2(ops / F(n,d))` on `log2 n` and adds the formula value at the P-224 shape (`F = 2^97.50` minus, `2^88.36` plus). Fit C (plus only) fits the polynomial phase on `log2 d` and the search phases on `log2 sqrt(n/d)` separately and adds them. 'pred 95%' is the OLS prediction interval; 'boot 95%' the bootstrap interval of the extrapolated value. Gap = extrapolated minus formula, in bits.\n", cells.iter().map(|c| c.bits).max().unwrap_or(0) as f64).unwrap();
    writeln!(md, "| family | arm | fit | log2 ops at 224 | pred 95% | boot 95% | formula | gap (bits) | S/S_rho at 224 |").unwrap();
    writeln!(md, "|:--|:--|:--|--:|:--|:--|--:|--:|--:|").unwrap();
    let formula = |fam: &str, arm: &str| -> f64 {
        if arm == "rho.negation" {
            // 0.886 sqrt(n) at 224 bits
            112.0 + (std::f64::consts::PI / 4.0).sqrt().log2()
        } else if fam == "minus" {
            let ld = LOG2_D_MINUS;
            ((2f64).powf((LOG2_N_P224 - ld) / 2.0) + (2f64).powf(ld / 2.0)).log2()
        } else {
            let ld = LOG2_D_PLUS;
            ((2f64).powf((LOG2_N_P224 - ld) / 2.0) + (2f64).powf(ld)).log2()
        }
    };
    let mut ext_js = vec![];
    for fam in ["minus", "plus"] {
        let rho_fit = fits
            .get(&(fam.to_string(), "rho.negation".to_string()))
            .map(|x| x.0);
        for arm in arms_of(fam) {
            let Some((fit, (elo, ehi))) = fits.get(&(fam.to_string(), arm.to_string())) else {
                continue;
            };
            let fo = formula(fam, arm);
            let ext = fit.at(LOG2_N_P224);
            let (plo, phi) = fit.pred_ci(LOG2_N_P224);
            let s_ratio = rho_fit.map_or("n/a".into(), |r| {
                format!("2^{:+.1}", ext - r.at(LOG2_N_P224))
            });
            writeln!(md, "| {fam} | `{arm}` | A | {ext:.2} | [{plo:.1}, {phi:.1}] | [{elo:.1}, {ehi:.1}] | {fo:.2} | {:+.2} | {s_ratio} |", ext - fo).unwrap();
            ext_js.push(json!({"family": fam, "arm": arm, "fit": "A", "log2_ops_224": ext, "pred95": [plo, phi], "boot95": [elo, ehi], "formula": fo, "gap_bits": ext - fo}));
            if arm == "rho.negation" {
                continue;
            }
            // fit B
            let s = series(fam, arm);
            let yb: Vec<f64> =
                s.x.iter()
                    .zip(&s.y)
                    .zip(&s.d)
                    .map(|((x, y), ld)| {
                        let ln = *x;
                        let fval = if fam == "minus" {
                            (2f64).powf((ln - ld) / 2.0) + (2f64).powf(ld / 2.0)
                        } else {
                            (2f64).powf((ln - ld) / 2.0) + (2f64).powf(*ld)
                        };
                        y - fval.log2()
                    })
                    .collect();
            if let Some(fb) = ols(&s.x, &yb) {
                let eb = fo + fb.at(LOG2_N_P224);
                let (plo, phi) = fb.pred_ci(LOG2_N_P224);
                let rf = rho_fit.map_or("n/a".into(), |r| {
                    format!("2^{:+.1}", eb - r.at(LOG2_N_P224))
                });
                writeln!(md, "| {fam} | `{arm}` | B (slope of log2 ops/F on log2 n: {:.3} +- {:.3}) | {eb:.2} | [{:.1}, {:.1}] | | {fo:.2} | {:+.2} | {rf} |", fb.b, fb.se_b, fo + plo, fo + phi, eb - fo).unwrap();
                ext_js.push(json!({"family": fam, "arm": arm, "fit": "B", "slope": fb.b, "se": fb.se_b, "log2_ops_224": eb, "formula": fo, "gap_bits": eb - fo}));
            }
            // fit C (plus)
            if arm == "cheon.pplus1_comb" {
                let mut xs_s = vec![];
                let mut ys_s = vec![];
                let mut xs_p = vec![];
                let mut ys_p = vec![];
                for c in cells.iter().filter(|c| c.fam == fam) {
                    let v = ver(fam, c.bits, arm);
                    if v.len() < 4 {
                        continue;
                    }
                    let poly = v
                        .iter()
                        .map(|o| {
                            o.phases
                                .get("p+1:poly")
                                .copied()
                                .unwrap_or(0.0)
                                .max(1.0)
                                .log2()
                        })
                        .sum::<f64>()
                        / v.len() as f64;
                    let search = v
                        .iter()
                        .map(|o| {
                            (o.ops - o.phases.get("p+1:poly").copied().unwrap_or(0.0))
                                .max(1.0)
                                .log2()
                        })
                        .sum::<f64>()
                        / v.len() as f64;
                    xs_p.push((c.d as f64).log2());
                    ys_p.push(poly);
                    xs_s.push(((c.n as f64) / (c.d as f64)).sqrt().log2());
                    ys_s.push(search);
                }
                if let (Some(fp), Some(fs)) = (ols(&xs_p, &ys_p), ols(&xs_s, &ys_s)) {
                    let poly224 = fp.at(LOG2_D_PLUS);
                    let search224 = fs.at((LOG2_N_P224 - LOG2_D_PLUS) / 2.0);
                    let tot = ((2f64).powf(poly224) + (2f64).powf(search224)).log2();
                    let rf = rho_fit.map_or("n/a".into(), |r| {
                        format!("2^{:+.1}", tot - r.at(LOG2_N_P224))
                    });
                    writeln!(md, "| {fam} | `{arm}` | C (poly: log2 ops = {:.2} + {:.3} log2 d, search: {:.2} + {:.3} log2 sqrt(n/d)) | {tot:.2} | | | {fo:.2} | {:+.2} | {rf} |", fp.a, fp.b, fs.a, fs.b, tot - fo).unwrap();
                    ext_js.push(json!({"family": fam, "arm": arm, "fit": "C", "poly_slope": fp.b, "search_slope": fs.b, "poly224": poly224, "search224": search224, "log2_ops_224": tot, "formula": fo, "gap_bits": tot - fo}));
                }
            }
        }
    }
    js["fits"] = fit_js;
    js["extrapolation"] = json!(ext_js);

    // ── crossover with rho (extrapolation) ───────────────────────────────
    writeln!(md, "\n### Table 5. Distance to the matched rho (largest measured size, and EXTRAPOLATED crossing)\n").unwrap();
    writeln!(md, "`ratio` is the median paired Cheon/rho ratio at the largest size with a complete arm. `crossing` is the `log2 n` at which fit A of the arm meets fit A of rho (extrapolation; `none` if the arm's slope is not below rho's).\n").unwrap();
    writeln!(
        md,
        "| family | arm | largest size | ratio at largest size | crossing log2 n (extrapolated) |"
    )
    .unwrap();
    writeln!(md, "|:--|:--|--:|--:|--:|").unwrap();
    for fam in ["minus", "plus"] {
        for arm in arms_of(fam) {
            if arm == "rho.negation" {
                continue;
            }
            let rho = ver(fam, 0, "x");
            let _ = rho;
            let mut big = None;
            for c in cells.iter().filter(|c| c.fam == fam) {
                if !ver(fam, c.bits, arm).is_empty() && !ver(fam, c.bits, "rho.negation").is_empty()
                {
                    big = Some(c.bits);
                }
            }
            let Some(bits) = big else { continue };
            let r = ver(fam, bits, "rho.negation");
            let mut rs = vec![];
            for o in ver(fam, bits, arm) {
                if let Some(x) = r.iter().find(|x| x.t == o.t) {
                    rs.push(o.ops / x.ops);
                }
            }
            let cross = match (
                fits.get(&(fam.to_string(), arm.to_string())),
                fits.get(&(fam.to_string(), "rho.negation".to_string())),
            ) {
                (Some((fa, _)), Some((fr, _))) if fa.b < fr.b => {
                    format!("{:.0}", (fr.a - fa.a) / (fa.b - fr.b))
                }
                _ => "none".into(),
            };
            writeln!(
                md,
                "| {fam} | `{arm}` | {bits} | {:.2} | {cross} |",
                median(&mut rs)
            )
            .unwrap();
        }
    }

    // ── phases ───────────────────────────────────────────────────────────
    writeln!(
        md,
        "\n### Table 6. Where the operations go (median share of total, per phase)\n"
    )
    .unwrap();
    for (fam, arm) in [
        ("minus", "cheon.pminus1_comb"),
        ("plus", "cheon.pplus1_comb"),
    ] {
        let mut names: Vec<String> = vec![];
        for c in cells.iter().filter(|c| c.fam == fam) {
            for o in ver(fam, c.bits, arm) {
                for k in o.phases.keys() {
                    if !names.contains(k) {
                        names.push(k.clone());
                    }
                }
            }
        }
        names.sort();
        writeln!(md, "`{arm}` ({fam}). Shares in percent.\n").unwrap();
        write!(md, "| bits |").unwrap();
        for n in &names {
            write!(md, " {n} |").unwrap();
        }
        writeln!(md, "\n|--:|{}", "--:|".repeat(names.len())).unwrap();
        for c in cells.iter().filter(|c| c.fam == fam) {
            let v = ver(fam, c.bits, arm);
            if v.is_empty() {
                continue;
            }
            write!(md, "| {} |", c.bits).unwrap();
            for n in &names {
                let mut sh: Vec<f64> = v
                    .iter()
                    .map(|o| 100.0 * o.phases.get(n).copied().unwrap_or(0.0) / o.ops)
                    .collect();
                write!(md, " {:.1} |", median(&mut sh)).unwrap();
            }
            writeln!(md).unwrap();
        }
        writeln!(md).unwrap();
    }

    // ── figures ──────────────────────────────────────────────────────────
    figures(&out_dir, &cells, &fits, &ver, &formula);

    std::fs::write(format!("{out_dir}/results.md"), &md).unwrap();
    std::fs::write(
        format!("{out_dir}/results.json"),
        serde_json::to_string_pretty(&js).unwrap() + "\n",
    )
    .unwrap();
    println!("{md}");
}

// ── SVG ──────────────────────────────────────────────────────────────────

struct Plot {
    w: f64,
    h: f64,
    ml: f64,
    mr: f64,
    mt: f64,
    mb: f64,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    s: String,
}

const COL_RHO: &str = "#6b7280";
const COL_A: &str = "#c2410c";
const COL_B: &str = "#1d4ed8";
const COL_C: &str = "#15803d";

impl Plot {
    fn new(w: f64, h: f64, x: (f64, f64), y: (f64, f64)) -> Self {
        Plot {
            w,
            h,
            ml: 62.0,
            mr: 16.0,
            mt: 34.0,
            mb: 46.0,
            x0: x.0,
            x1: x.1,
            y0: y.0,
            y1: y.1,
            s: String::new(),
        }
    }
    fn px(&self, x: f64) -> f64 {
        self.ml + (x - self.x0) / (self.x1 - self.x0) * (self.w - self.ml - self.mr)
    }
    fn py(&self, y: f64) -> f64 {
        self.h - self.mb - (y - self.y0) / (self.y1 - self.y0) * (self.h - self.mt - self.mb)
    }
    fn axes(&mut self, xt: f64, yt: f64, xl: &str, yl: &str, title: &str) {
        let mut x = (self.x0 / xt).ceil() * xt;
        while x <= self.x1 + 1e-9 {
            let p = self.px(x);
            write!(self.s, "<line x1='{p:.1}' y1='{:.1}' x2='{p:.1}' y2='{:.1}' stroke='var(--grid)'/><text x='{p:.1}' y='{:.1}' text-anchor='middle' class='t'>{x:.0}</text>", self.mt, self.h - self.mb, self.h - self.mb + 15.0).unwrap();
            x += xt;
        }
        let mut y = (self.y0 / yt).ceil() * yt;
        while y <= self.y1 + 1e-9 {
            let p = self.py(y);
            write!(self.s, "<line x1='{:.1}' y1='{p:.1}' x2='{:.1}' y2='{p:.1}' stroke='var(--grid)'/><text x='{:.1}' y='{:.1}' text-anchor='end' class='t'>{y:.0}</text>", self.ml, self.w - self.mr, self.ml - 6.0, p + 4.0).unwrap();
            y += yt;
        }
        write!(self.s, "<rect x='{:.1}' y='{:.1}' width='{:.1}' height='{:.1}' fill='none' stroke='var(--axis)'/>", self.ml, self.mt, self.w - self.ml - self.mr, self.h - self.mt - self.mb).unwrap();
        write!(
            self.s,
            "<text x='{:.1}' y='{:.1}' text-anchor='middle' class='l'>{xl}</text>",
            (self.ml + self.w - self.mr) / 2.0,
            self.h - 8.0
        )
        .unwrap();
        write!(self.s, "<text transform='translate(14,{:.1}) rotate(-90)' text-anchor='middle' class='l'>{yl}</text>", (self.mt + self.h - self.mb) / 2.0).unwrap();
        write!(
            self.s,
            "<text x='{:.1}' y='20' class='h'>{title}</text>",
            self.ml
        )
        .unwrap();
    }
    fn line(&mut self, pts: &[(f64, f64)], col: &str, dash: bool, w: f64) {
        let d: Vec<String> = pts
            .iter()
            .map(|(x, y)| format!("{:.1},{:.1}", self.px(*x), self.py(*y)))
            .collect();
        write!(
            self.s,
            "<polyline points='{}' fill='none' stroke='{col}' stroke-width='{w}'{}/>",
            d.join(" "),
            if dash { " stroke-dasharray='5 4'" } else { "" }
        )
        .unwrap();
    }
    fn dot(&mut self, x: f64, y: f64, col: &str, r: f64) {
        write!(
            self.s,
            "<circle cx='{:.1}' cy='{:.1}' r='{r}' fill='{col}'/>",
            self.px(x),
            self.py(y)
        )
        .unwrap();
    }
    fn diamond(&mut self, x: f64, y: f64, col: &str) {
        let (cx, cy) = (self.px(x), self.py(y));
        write!(self.s, "<path d='M{cx:.1},{:.1} L{:.1},{cy:.1} L{cx:.1},{:.1} L{:.1},{cy:.1} Z' fill='{col}' stroke='var(--bg)'/>", cy - 6.0, cx + 6.0, cy + 6.0, cx - 6.0).unwrap();
    }
    fn vbar(&mut self, x: f64, lo: f64, hi: f64, col: &str) {
        write!(self.s, "<line x1='{p:.1}' y1='{:.1}' x2='{p:.1}' y2='{:.1}' stroke='{col}' stroke-width='1.5'/>", self.py(lo), self.py(hi), p = self.px(x)).unwrap();
    }
    fn text(&mut self, x: f64, y: f64, t: &str, col: &str) {
        write!(
            self.s,
            "<text x='{x:.1}' y='{y:.1}' class='t' fill='{col}'>{t}</text>",
        )
        .unwrap();
    }
    fn legend(&mut self, items: &[(&str, &str, bool)]) {
        let mut y = self.mt + 16.0;
        for (col, label, dash) in items {
            let x = self.ml + 10.0;
            write!(self.s, "<line x1='{x:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{col}' stroke-width='2'{}/><text x='{:.1}' y='{:.1}' class='t'>{label}</text>", y - 4.0, x + 22.0, y - 4.0, if *dash { " stroke-dasharray='5 4'" } else { "" }, x + 28.0, y).unwrap();
            y += 15.0;
        }
    }
}

fn svg_wrap(w: f64, h: f64, body: &str, caption: &str) -> String {
    format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' width='{w}' height='{h}' font-family='system-ui, sans-serif'>\n<style>:root{{--bg:#ffffff;--fg:#111827;--grid:#e5e7eb;--axis:#6b7280}}@media (prefers-color-scheme: dark){{:root{{--bg:#111827;--fg:#e5e7eb;--grid:#374151;--axis:#9ca3af}}}}.t{{font-size:11px;fill:var(--fg)}}.l{{font-size:12px;fill:var(--fg)}}.h{{font-size:13px;font-weight:600;fill:var(--fg)}}.c{{font-size:11px;fill:var(--fg)}}</style>\n<rect width='100%' height='100%' fill='var(--bg)'/>\n{body}\n<text x='10' y='{:.1}' class='c'>{caption}</text>\n</svg>\n",
        h - 6.0
    )
}

#[allow(clippy::type_complexity)]
fn figures(
    out: &str,
    cells: &[Cell],
    fits: &BTreeMap<(String, String), (Fit, (f64, f64))>,
    ver: &dyn Fn(&str, u64, &str) -> Vec<Obs>,
    formula: &dyn Fn(&str, &str) -> f64,
) {
    // Fig 1: scaling and extrapolation, two panels
    let (pw, ph) = (560.0, 400.0);
    let mut body = String::new();
    for (i, fam) in ["minus", "plus"].iter().enumerate() {
        let mut p = Plot::new(pw, ph, (28.0, 232.0), (10.0, 120.0));
        let title = if *fam == "minus" {
            "n-1 shape (d = n^0.129), Cheon vs rho"
        } else {
            "n+1 shape (d = n^0.211), Cheon vs rho"
        };
        p.axes(40.0, 20.0, "log2 n", "log2 group operations", title);
        let arms: Vec<(&str, &str, &str)> = if *fam == "minus" {
            vec![
                ("rho.negation", COL_RHO, "rho.negation"),
                ("cheon.pminus1_comb", COL_A, "Cheon comb"),
                ("cheon.pminus1_naive", COL_B, "Cheon naive"),
            ]
        } else {
            vec![
                ("rho.negation", COL_RHO, "rho.negation"),
                ("cheon.pplus1_comb", COL_A, "Cheon comb (2d powers)"),
            ]
        };
        let mut legend = vec![];
        for (arm, col, label) in &arms {
            for c in cells.iter().filter(|c| c.fam == *fam) {
                let v = ver(fam, c.bits, arm);
                if v.is_empty() {
                    continue;
                }
                let ys: Vec<f64> = v.iter().map(|o| o.ops.log2()).collect();
                let x = (c.n as f64).log2();
                p.vbar(
                    x,
                    ys.iter().cloned().fold(f64::MAX, f64::min),
                    ys.iter().cloned().fold(f64::MIN, f64::max),
                    col,
                );
                p.dot(x, ys.iter().sum::<f64>() / ys.len() as f64, col, 3.5);
            }
            if let Some((ft, _)) = fits.get(&(fam.to_string(), arm.to_string())) {
                p.line(
                    &[(30.0, ft.at(30.0)), (224.0, ft.at(224.0))],
                    col,
                    true,
                    1.5,
                );
                p.diamond(224.0, formula(fam, arm), col);
            }
            legend.push((*col, *label, false));
        }
        legend.push(("var(--fg)", "dashed: fit A, EXTRAPOLATED to 224", true));
        p.legend(&legend);
        p.text(
            p.px(224.0) - 150.0,
            p.py(10.0) - 6.0,
            "diamonds: formula at the P-224 shape",
            "var(--fg)",
        );
        write!(
            body,
            "<g transform='translate({},0)'>{}</g>",
            i as f64 * pw,
            p.s
        )
        .unwrap();
    }
    let cap = "Fig. 1. Measured log2 operations (dots = mean over verified targets, bars = min..max) and fit A extended to 224 bits (dashed, extrapolation). Source: results.json.";
    std::fs::write(
        format!("{out}/fig1_scaling.svg"),
        svg_wrap(pw * 2.0, ph, &body, cap),
    )
    .unwrap();

    // Fig 2: ratio to rho by size
    let mut p = Plot::new(620.0, 360.0, (30.0, 58.0), (-4.0, 12.0));
    p.axes(
        4.0,
        2.0,
        "bits of n",
        "log2 (ops / rho ops)  (0 = parity)",
        "Distance to the matched rho (paired by target)",
    );
    p.line(&[(30.0, 0.0), (58.0, 0.0)], "var(--axis)", false, 1.0);
    let sets = [
        ("minus", "cheon.pminus1_comb", COL_A, "n-1 shape, comb"),
        ("minus", "cheon.pminus1_naive", COL_B, "n-1 shape, naive"),
        ("plus", "cheon.pplus1_comb", COL_C, "n+1 shape, comb"),
    ];
    let mut leg = vec![];
    for (fam, arm, col, label) in sets {
        let mut pts = vec![];
        for c in cells.iter().filter(|c| c.fam == fam) {
            let r = ver(fam, c.bits, "rho.negation");
            let mut rs = vec![];
            for o in ver(fam, c.bits, arm) {
                if let Some(x) = r.iter().find(|x| x.t == o.t) {
                    rs.push((o.ops / x.ops).log2());
                }
            }
            if rs.is_empty() {
                continue;
            }
            let m = rs.iter().sum::<f64>() / rs.len() as f64;
            let b = c.bits as f64 - 0.4;
            p.vbar(
                b,
                rs.iter().cloned().fold(f64::MAX, f64::min),
                rs.iter().cloned().fold(f64::MIN, f64::max),
                col,
            );
            pts.push((b, m));
        }
        if pts.len() >= 2 {
            p.line(&pts, col, false, 1.5);
        }
        for (x, y) in &pts {
            p.dot(*x, *y, col, 3.5);
        }
        leg.push((col, label, false));
    }
    p.legend(&leg);
    let cap = "Fig. 2. log2 of the Cheon/rho operation ratio per size (dots = mean, bars = min..max over paired targets). Above 0 = Cheon dearer. Source: runs.jsonl and the ecbench rho session.";
    std::fs::write(
        format!("{out}/fig2_ratio_to_rho.svg"),
        svg_wrap(620.0, 360.0, &p.s, cap),
    )
    .unwrap();

    // Fig 3: phase shares, plus variant (poly vs search) and minus (precompute vs search)
    let (w3, h3) = (620.0, 340.0);
    let mut body = String::new();
    let mut p = Plot::new(w3, h3, (30.0, 58.0), (0.0, 100.0));
    p.axes(
        4.0,
        20.0,
        "bits of n",
        "share of total operations (%)",
        "Cheon n+1: polynomial phase vs search phases",
    );
    let mut poly = vec![];
    let mut tabs = vec![];
    for c in cells.iter().filter(|c| c.fam == "plus") {
        let v = ver("plus", c.bits, "cheon.pplus1_comb");
        if v.is_empty() {
            continue;
        }
        let sh = |name: &str| {
            let mut s: Vec<f64> = v
                .iter()
                .map(|o| 100.0 * o.phases.get(name).copied().unwrap_or(0.0) / o.ops)
                .collect();
            median(&mut s)
        };
        poly.push((c.bits as f64 - 0.4, sh("p+1:poly")));
        let tb: f64 = ["p+1:step1:precompute", "p+1:step2:precompute"]
            .iter()
            .map(|n| sh(n))
            .sum();
        tabs.push((c.bits as f64 - 0.4, tb));
    }
    p.line(&poly, COL_A, false, 2.0);
    for (x, y) in &poly {
        p.dot(*x, *y, COL_A, 3.5);
    }
    p.line(&tabs, COL_B, false, 2.0);
    for (x, y) in &tabs {
        p.dot(*x, *y, COL_B, 3.5);
    }
    p.legend(&[
        (
            COL_A,
            "polynomial multi-scalar phase (3(2d+1) terms)",
            false,
        ),
        (COL_B, "comb-table construction", false),
    ]);
    write!(body, "{}", p.s).unwrap();
    let cap = "Fig. 3. Median share of the n+1 variant's operations by phase. Source: runs.jsonl phase columns.";
    std::fs::write(
        format!("{out}/fig3_phase_shares.svg"),
        svg_wrap(w3, h3, &body, cap),
    )
    .unwrap();
}

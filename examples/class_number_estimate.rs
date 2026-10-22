//! Class-number estimate for the CM fields of NIST P-224 and P-256.
//!
//! Protocol: `research/p224_class_number_20261022/PROTOCOL.md`.
//!
//! ```bash
//! CARGO_TARGET_DIR=target cargo run --release --example class_number_estimate -- \
//!     --curve both --bound 10000000000 --threads 4 \
//!     --out research/p224_class_number_20261022/run
//! ```
//!
//! Writes `results.json` and `convergence.svg` into `--out`.

use crypto::cryptanalysis::class_number_estimate::*;
use num_bigint::{BigInt, BigUint};
use num_traits::Num;
use serde_json::json;
use std::time::Instant;

fn dec(s: &str) -> BigUint {
    BigUint::from_str_radix(s, 10).expect("decimal")
}
fn hex(s: &str) -> BigUint {
    BigUint::from_str_radix(s, 16).expect("hex")
}

fn inputs() -> Vec<CmInput> {
    let one = BigUint::from(1u32);
    let p224 = (&one << 224u32) - (&one << 96u32) + &one;
    let p256 = (&one << 256u32) - (&one << 224u32) + (&one << 192u32) + (&one << 96u32) - &one;
    let n256 = hex("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551");
    let t256 = &p256 + &one - n256;
    vec![
        CmInput {
            name: "P-224",
            p: p224,
            t: dec("4733100108545601916421827343930821"),
            conductor: 3,
            factors: [
                "3",
                "29",
                "79",
                "7523",
                "40927",
                "11549194661",
                "388425074903852603481408727235725774844022299",
            ]
            .iter()
            .map(|s| dec(s))
            .collect(),
        },
        CmInput {
            name: "P-256",
            p: p256,
            t: t256,
            conductor: 1,
            factors: [
                "3",
                "5",
                "456597257999",
                "1428624589419343516204097",
                "46523541035814968339936406074986559003387",
            ]
            .iter()
            .map(|s| dec(s))
            .collect(),
        },
    ]
}

fn log2_from_ln(x: f64) -> f64 {
    x / std::f64::consts::LN_2
}

fn sci(ln_x: f64) -> String {
    let (m, e) = sci_from_ln(ln_x);
    format!("{m:.4}e{e}")
}

struct Series {
    label: String,
    color: &'static str,
    dash: bool,
    pts: Vec<(f64, f64)>,
}

type Rect = (f64, f64, f64, f64);

fn panel(
    svg: &mut String,
    (x0, y0, w, h): Rect,
    titles: (&str, &str, &str),
    (xmin, xmax): (f64, f64),
    (ymin, ymax): (f64, f64),
    series: &[Series],
) {
    let (title, xlabel, ylabel) = titles;
    let px = |x: f64| x0 + (x - xmin) / (xmax - xmin) * w;
    let py = |y: f64| y0 + h - (y - ymin) / (ymax - ymin) * h;
    svg.push_str(&format!(
        "<text x=\"{x0}\" y=\"{}\" font-size=\"15\" font-weight=\"bold\">{title}</text>\n",
        y0 - 12.0
    ));
    svg.push_str(&format!(
        "<rect x=\"{x0}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\" fill=\"none\" stroke=\"#444\"/>\n"
    ));
    for i in 0..=7 {
        let x = xmin + (xmax - xmin) * i as f64 / 7.0;
        svg.push_str(&format!(
            "<line x1=\"{0}\" y1=\"{y0}\" x2=\"{0}\" y2=\"{1}\" stroke=\"#ddd\"/><text x=\"{0}\" y=\"{2}\" font-size=\"11\" text-anchor=\"middle\">{3:.0}</text>\n",
            px(x),
            y0 + h,
            y0 + h + 14.0,
            x
        ));
    }
    for i in 0..=5 {
        let y = ymin + (ymax - ymin) * i as f64 / 5.0;
        svg.push_str(&format!(
            "<line x1=\"{x0}\" y1=\"{0}\" x2=\"{1}\" y2=\"{0}\" stroke=\"#ddd\"/><text x=\"{2}\" y=\"{3}\" font-size=\"11\" text-anchor=\"end\">{4:.3}</text>\n",
            py(y),
            x0 + w,
            x0 - 5.0,
            py(y) + 4.0,
            y
        ));
    }
    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-size=\"12\" text-anchor=\"middle\">{xlabel}</text>\n",
        x0 + w / 2.0,
        y0 + h + 30.0
    ));
    svg.push_str(&format!(
        "<text transform=\"translate({},{}) rotate(-90)\" font-size=\"12\" text-anchor=\"middle\">{ylabel}</text>\n",
        x0 - 48.0,
        y0 + h / 2.0
    ));
    for (k, s) in series.iter().enumerate() {
        let pts: Vec<String> = s
            .pts
            .iter()
            .map(|&(x, y)| format!("{:.1},{:.1}", px(x), py(y.clamp(ymin, ymax))))
            .collect();
        let dash = if s.dash {
            "stroke-dasharray=\"6 4\""
        } else {
            ""
        };
        svg.push_str(&format!(
            "<polyline fill=\"none\" stroke=\"{}\" stroke-width=\"2\" {dash} points=\"{}\"/>\n",
            s.color,
            pts.join(" ")
        ));
        for &(x, y) in &s.pts {
            svg.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"{}\"/>\n",
                px(x),
                py(y.clamp(ymin, ymax)),
                s.color
            ));
        }
        let ly = y0 + 16.0 + 15.0 * k as f64;
        svg.push_str(&format!(
            "<line x1=\"{0}\" y1=\"{ly}\" x2=\"{1}\" y2=\"{ly}\" stroke=\"{2}\" stroke-width=\"2\" {dash}/><text x=\"{3}\" y=\"{4}\" font-size=\"11\">{5}</text>\n",
            x0 + w - 330.0,
            x0 + w - 305.0,
            s.color,
            x0 + w - 300.0,
            ly + 4.0,
            s.label
        ));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let which = get("--curve", "both");
    let bmax: u64 = get("--bound", "10000000000").parse().expect("bound");
    let threads: usize = get("--threads", "4").parse().expect("threads");
    let ncontrol: usize = get("--controls", "400").parse().expect("controls");
    let out = get("--out", "research/p224_class_number_20261022/run");
    std::fs::create_dir_all(&out).expect("out dir");

    let c_headline = 1.0 / std::f64::consts::PI;
    let c_schoenfeld = 1.0 / (8.0 * std::f64::consts::PI);

    // checkpoints: 1, 2, 5 x 10^k
    let mut bounds: Vec<u64> = Vec::new();
    let mut dec_pow = 1000u64;
    while dec_pow <= bmax {
        for m in [1u64, 2, 5] {
            if m * dec_pow <= bmax {
                bounds.push(m * dec_pow);
            }
        }
        dec_pow = dec_pow.saturating_mul(10);
    }
    if *bounds.last().unwrap() != bmax {
        bounds.push(bmax);
    }

    let mut results = Vec::new();
    let mut plot_curves: Vec<(String, Vec<Checkpoint>, f64)> = Vec::new();
    for cm in inputs() {
        if which != "both" && cm.name.to_lowercase().replace('-', "") != which.to_lowercase() {
            continue;
        }
        let abs = verify_cm_input(&cm).expect("input verification");
        let nd = NegDisc::new(abs.clone()).expect("discriminant shape");
        let genus = genus_data(&(-BigInt::from(abs.clone())), &cm.factors).expect("genus data");
        eprintln!(
            "{}: |D| = 2^{:.3}, t = {}, 2-rank {}, 4-rank {}",
            cm.name,
            nd.ln_abs / std::f64::consts::LN_2,
            genus.t,
            genus.two_rank,
            genus.four_rank
        );
        let t0 = Instant::now();
        let cps = euler_sweep(&nd, &bounds, threads);
        let wall = t0.elapsed().as_secs_f64();
        eprintln!("{}: sweep to {} in {:.1}s", cm.name, bmax, wall);

        let shape = |c: &Checkpoint| {
            let b = c.bound as f64;
            c.s_chi.abs() / (b.sqrt() * (nd.ln_abs + b.ln()).powi(2))
        };
        let mut rows = Vec::new();
        for cp in &cps {
            let b = cp.bound as f64;
            let ln_h = nd.ln_h(cp.ln_l);
            rows.push(json!({
                "bound": cp.bound,
                "ln_L": cp.ln_l,
                "L": cp.ln_l.exp(),
                "ln_h": ln_h,
                "log2_h": log2_from_ln(ln_h),
                "h_sci": sci(ln_h),
                "heuristic_sigma_lnL": heuristic_tail_sigma(b),
                "grh_halfwidth_c_1_over_pi": grh_tail_bound(nd.ln_abs, b, c_headline),
                "grh_halfwidth_c_1_over_8pi": grh_tail_bound(nd.ln_abs, b, c_schoenfeld),
                "sum_chi_ln_p": cp.s_chi,
                "grh_shape_ratio": shape(cp),
                "n_split": cp.n_split, "n_inert": cp.n_inert, "n_ramified": cp.n_ramified,
            }));
        }
        let last = cps.last().unwrap();
        let b = last.bound as f64;
        let ln_h = nd.ln_h(last.ln_l);
        let sigma = heuristic_tail_sigma(b);
        let g1 = grh_tail_bound(nd.ln_abs, b, c_headline);
        let g2 = grh_tail_bound(nd.ln_abs, b, c_schoenfeld);
        let band = |hw: f64| {
            json!({
                "log2_lo": log2_from_ln(ln_h - hw), "log2_hi": log2_from_ln(ln_h + hw),
                "lo": sci(ln_h - hw), "hi": sci(ln_h + hw),
                "ln_halfwidth": hw,
            })
        };
        let max_ratio = cps.iter().map(shape).fold(0.0f64, f64::max);
        results.push(json!({
            "curve": cm.name,
            "p": cm.p.to_string(), "trace": cm.t.to_string(), "conductor": cm.conductor,
            "abs_D": abs.to_string(),
            "log2_abs_D": nd.ln_abs / std::f64::consts::LN_2,
            "factors": cm.factors.iter().map(|f| f.to_string()).collect::<Vec<_>>(),
            "D_mod_8": if nd.chi2 == 1 { 1 } else { 5 },
            "genus": {
                "t_prime_discriminants": genus.t, "two_rank": genus.two_rank,
                "redei_rank": genus.redei_rank, "four_rank": genus.four_rank,
                "two_power_dividing_h": genus.two_rank + genus.four_rank,
                "elementary_two_part": genus.four_rank == 0,
            },
            "euler_bound": last.bound,
            "sweep_wall_seconds": wall, "threads": threads,
            "estimate": {
                "ln_h": ln_h, "log2_h": log2_from_ln(ln_h), "h_sci": sci(ln_h),
                "L1": last.ln_l.exp(),
                "heuristic_1sigma_lnL": sigma,
                "heuristic_3sigma": band(3.0 * sigma),
                "grh_conditional_c_1_over_pi": band(g1),
                "grh_conditional_c_1_over_8pi": band(g2),
                "max_grh_shape_ratio_observed": max_ratio,
                "assumed_c_headline": c_headline,
            },
            "checkpoints": rows,
        }));
        plot_curves.push((cm.name.to_string(), cps, nd.ln_abs));
    }

    // ---- controls ---------------------------------------------------------
    let mut ds: Vec<i64> = Vec::new();
    let mut d = -10_000_000i64;
    while ds.len() < ncontrol {
        d -= 1;
        if is_fundamental_negative(d) {
            ds.push(d);
        }
    }
    let ctrl_bounds = [1_000u64, 10_000, 100_000];
    let mut z: Vec<Vec<f64>> = vec![Vec::new(); 3];
    let mut grh_cover = [0usize; 3];
    let mut genus_bad = 0usize;
    for &d in &ds {
        let nd = NegDisc::from_i64(d);
        let cps = euler_sweep(&nd, &ctrl_bounds, 1);
        let h = exact_class_number(d);
        let hx = (h as f64).ln();
        for (k, cp) in cps.iter().enumerate() {
            let b = cp.bound as f64;
            let err = hx - nd.ln_h(cp.ln_l);
            z[k].push(err / heuristic_tail_sigma(b));
            if err.abs() <= grh_tail_bound(nd.ln_abs, b, c_headline) {
                grh_cover[k] += 1;
            }
        }
        let g = genus_data(&BigInt::from(d), &odd_prime_factors_small(d)).expect("genus");
        let v2 = h.trailing_zeros() as usize;
        if v2 < g.two_rank + g.four_rank || (g.four_rank == 0 && v2 != g.two_rank) {
            genus_bad += 1;
        }
    }
    let mut crows = Vec::new();
    for k in 0..3 {
        let n = z[k].len() as f64;
        let mean = z[k].iter().sum::<f64>() / n;
        let sd = (z[k].iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
        let within3 = z[k].iter().filter(|x| x.abs() <= 3.0).count() as f64 / n;
        let maxabs = z[k].iter().fold(0.0f64, |a, x| a.max(x.abs()));
        crows.push(json!({
            "bound": ctrl_bounds[k], "n": z[k].len(), "z_mean": mean, "z_sd": sd,
            "z_max_abs": maxabs, "fraction_within_3sigma": within3,
            "grh_band_cover": grh_cover[k] as f64 / n,
        }));
    }
    let controls = json!({
        "discriminants": format!("first {} fundamental D below -10^7", ds.len()),
        "d_range": [ds[0], ds[ds.len() - 1]],
        "calibration_by_bound": crows,
        "genus_theory_violations": genus_bad,
    });
    std::fs::write(
        format!("{out}/results.json"),
        serde_json::to_string_pretty(&json!({ "results": results, "controls": controls })).unwrap(),
    )
    .unwrap();

    // ---- figure -----------------------------------------------------------
    let colors = ["#1f5fbf", "#d1601b"];
    let mut svg = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"860\" height=\"1000\" font-family=\"sans-serif\">\n<rect width=\"860\" height=\"1000\" fill=\"#fff\"/>\n",
    );
    svg.push_str("<text x=\"20\" y=\"24\" font-size=\"17\" font-weight=\"bold\">Euler-product convergence for the CM discriminants of P-224 and P-256</text>\n");
    let (mut sa, mut sb, mut sc) = (Vec::new(), Vec::new(), Vec::new());
    for (i, (name, cps, ln_abs)) in plot_curves.iter().enumerate() {
        let col = colors[i % 2];
        sa.push(Series {
            label: format!("{name}: ln L_B - ln L_final"),
            color: col,
            dash: false,
            pts: cps
                .iter()
                .map(|c| ((c.bound as f64).log10(), c.ln_l - cps.last().unwrap().ln_l))
                .collect(),
        });
        sb.push(Series {
            label: format!("{name}: GRH half-width, c=1/pi"),
            color: col,
            dash: false,
            pts: cps
                .iter()
                .map(|c| {
                    let b = c.bound as f64;
                    (b.log10(), grh_tail_bound(*ln_abs, b, c_headline).log10())
                })
                .collect(),
        });
        sb.push(Series {
            label: format!("{name}: heuristic 3 sigma"),
            color: col,
            dash: true,
            pts: cps
                .iter()
                .map(|c| {
                    let b = c.bound as f64;
                    (b.log10(), (3.0 * heuristic_tail_sigma(b)).log10())
                })
                .collect(),
        });
        sc.push(Series {
            label: format!("{name}: |sum chi ln p| / (sqrt(x) ln^2(|D|x))"),
            color: col,
            dash: false,
            pts: cps
                .iter()
                .map(|c| {
                    let b = c.bound as f64;
                    (
                        b.log10(),
                        c.s_chi.abs() / (b.sqrt() * (ln_abs + b.ln()).powi(2)),
                    )
                })
                .collect(),
        });
    }
    let xr = (3.0, (bmax as f64).log10());
    sc.push(Series {
        label: "assumed c = 1/(8 pi) = 0.0398 (headline c = 1/pi is off scale)".to_string(),
        color: "#555555",
        dash: true,
        pts: vec![(3.0, c_schoenfeld), ((bmax as f64).log10(), c_schoenfeld)],
    });
    panel(
        &mut svg,
        (90.0, 70.0, 730.0, 250.0),
        (
            "(a) partial Euler product minus its final value (measured)",
            "log10 B (primes up to B)",
            "ln L_B - ln L_final",
        ),
        xr,
        (-0.02, 0.04),
        &sa,
    );
    panel(
        &mut svg,
        (90.0, 400.0, 730.0, 250.0),
        (
            "(b) stated error half-widths for ln L (derived)",
            "log10 B",
            "log10 half-width",
        ),
        xr,
        (-6.0, 2.0),
        &sb,
    );
    panel(
        &mut svg,
        (90.0, 730.0, 730.0, 220.0),
        (
            "(c) observed ratio against the assumed GRH shape (measured)",
            "log10 B",
            "ratio",
        ),
        xr,
        (0.0, 0.05),
        &sc,
    );
    svg.push_str("<text x=\"20\" y=\"990\" font-size=\"11\">run/results.json; protocol research/p224_class_number_20261022/PROTOCOL.md. GRH bands assume |sum Lambda chi| &lt;= c sqrt(x) ln^2(|D|x); see README.</text>\n</svg>\n");
    std::fs::write(format!("{out}/convergence.svg"), svg).unwrap();
    eprintln!("wrote {out}/results.json and convergence.svg");
}

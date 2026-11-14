//! Formula-level cost of carrying an order-`r ~ p` elliptic discrete log up a
//! tower of covers into `J(C_g)(F_p)`, against Pollard rho on `E`.
//!
//! Formula-level, not measured: no Jacobian arithmetic is done here.  Every
//! cost is `log2` of group-operation equivalents.  The group-operation counts
//! use the published formulas below; the "weighted" columns additionally price
//! one genus-`g` Jacobian operation with the repository's frozen
//! [`CostModel`] (`src/cryptanalysis/jacobian_transfer/cost.rs`).
//!
//! Usage: `tower_formula [--json PATH] [--svg PATH]`; markdown tables go to
//! stdout.

use crypto_lib::cryptanalysis::jacobian_transfer::cost::{log2_add, log2_factorial, CostModel};
use serde_json::{json, Value};
use std::f64::consts::{LN_2, PI};
use std::fs;

const P_BITS: [f64; 5] = [64.0, 128.0, 192.0, 256.0, 521.0];
const G_MAX: u32 = 64;
const THETAS: [f64; 3] = [1.0, 0.5, 0.25];
/// Genus scan limit used to check that the subexponential formula is
/// increasing in `g` past the start of its regime.
const G_SCAN_MAX: u32 = 1 << 15;

/// Pollard rho on E with the negation map as the reference level the task
/// fixes: `sqrt(pi r / 4)` with `r ~ p`, in `log2` group operations.
fn log2_rho(n: f64) -> f64 {
    0.5 * n + 0.5 * (PI / 4.0).log2()
}

/// Gaudry's basic genus-`g` index calculus: relation collection `g! p`
/// trials plus sparse linear algebra `p^2` (Gaudry 2000, J. Symb. Comput.
/// 2000 / Gaudry, "An algorithm for solving the discrete log problem on
/// hyperelliptic curves", EUROCRYPT 2000).
fn log2_gaudry_basic(n: f64, g: u32) -> f64 {
    log2_add(log2_factorial(f64::from(g)) + n, 2.0 * n)
}

/// Reduced-base / double-large-prime exponent only: `p^(2-2/g)`, dropping
/// every g-dependent constant, `g!` and log factor (Thériault 2003;
/// Gaudry-Thomé-Thériault-Diem 2007).
fn log2_dlp_bare(n: f64, g: u32) -> f64 {
    (2.0 - 2.0 / f64::from(g)) * n
}

/// The repository's pricing of the same family (`CostModel::log2_hyperelliptic_ic`
/// without the per-operation weight): `g! p + g^2 p^(2-2/g)`.
fn log2_dlp_costmodel(m: &CostModel, n: f64, g: u32) -> f64 {
    let lg = f64::from(g).log2();
    let la = log2_dlp_bare(n, g) + m.hyperelliptic_ic_genus_polynomial_exponent * lg;
    log2_add(la, log2_factorial(f64::from(g)) + n)
}

/// `L_N[1/2, c]` with `N = p^g` and the `o(1)` dropped, in `log2`:
/// `c sqrt(ln N ln ln N) / ln 2` (Enge-Gaudry 2002; Diem 2006).
fn log2_subexp(c: f64, n: f64, g: u32) -> f64 {
    let ln_n = f64::from(g) * n * LN_2;
    if ln_n <= std::f64::consts::E {
        return 0.0;
    }
    c * (ln_n * ln_n.ln()).sqrt() / LN_2
}

/// First genus of the regime `g >= theta * ln p`.
fn regime_start(theta: f64, n: f64) -> u32 {
    (theta * n * LN_2).ceil().max(1.0) as u32
}

#[derive(Clone)]
struct Row {
    g: u32,
    gaudry: Option<f64>,
    dlp_bare: Option<f64>,
    dlp_cm: Option<f64>,
    subexp: f64,
    subexp_valid: bool,
    w_shift: f64,
}

fn rows(m: &CostModel, n: f64) -> Vec<Row> {
    let theta_gap = regime_start(1.0, n);
    (1..=G_MAX)
        .map(|g| {
            let ic = g >= 2;
            // Weighted shift: Jacobian op minus elliptic op, in bits.
            let w_shift = m.log2_jacobian_op(f64::from(g).log2(), n) - m.log2_elliptic_op(n);
            Row {
                g,
                gaudry: ic.then(|| log2_gaudry_basic(n, g)),
                dlp_bare: ic.then(|| log2_dlp_bare(n, g)),
                dlp_cm: ic.then(|| log2_dlp_costmodel(m, n, g)),
                subexp: log2_subexp(m.enge_gaudry_c, n, g),
                subexp_valid: g >= theta_gap,
                w_shift,
            }
        })
        .collect()
}

/// `(best g, cost, weighted shift)` of a method by the unweighted group-operation count (the task's unit).
fn best_ops<F: Fn(&Row) -> Option<f64>>(rs: &[Row], f: F) -> Option<(u32, f64, f64)> {
    rs.iter()
        .filter_map(|r| f(r).map(|c| (r.g, c, r.w_shift)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

fn fmt_row(name: &str, b: Option<(u32, f64, f64)>, rho: f64, note: &str) -> String {
    match b {
        Some((g, c, w)) => format!(
            "| {name} | {g} | {c:.1} | {rho:.1} | {:+.1} | {:+.1} | {note} |\n",
            c - rho,
            c - rho + w
        ),
        None => format!("| {name} | none | n/a | {rho:.1} | n/a | n/a | {note} |\n"),
    }
}

fn opt(v: Option<f64>) -> Value {
    v.map_or(Value::Null, |x| json!(x))
}

/// Largest value plotted in a panel, rounded up to a multiple of 50.
fn panel_ymax(rs: &[Row], rho: f64) -> f64 {
    let mut m = rho;
    for r in rs {
        for v in [r.gaudry, r.dlp_bare, r.dlp_cm, Some(r.subexp)]
            .into_iter()
            .flatten()
        {
            m = m.max(v);
        }
    }
    (m / 50.0).ceil() * 50.0
}

fn svg_panel(
    out: &mut String,
    x0: f64,
    n: f64,
    rs: &[Row],
    rho: f64,
    colors: &[(&str, &str, &str)],
) {
    let (w, h) = (440.0, 330.0);
    let y0 = 60.0;
    let ymax = panel_ymax(rs, rho);
    let sx = |g: u32| x0 + (f64::from(g) - 1.0) / f64::from(G_MAX - 1) * w;
    let sy = |v: f64| y0 + h - v / ymax * h;
    out.push_str(&format!(
        "<text x=\"{:.0}\" y=\"30\" font-size=\"16\" font-weight=\"bold\">p = 2^{n:.0}: log2 cost vs genus g</text>\n",
        x0
    ));
    out.push_str(&format!(
        "<rect x=\"{x0}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\" fill=\"none\" stroke=\"#444\"/>\n"
    ));
    let step = if ymax > 400.0 { 100.0 } else { 50.0 };
    let mut t = 0.0;
    while t <= ymax + 1e-9 {
        out.push_str(&format!(
            "<line x1=\"{x0}\" y1=\"{y:.1}\" x2=\"{x1}\" y2=\"{y:.1}\" stroke=\"#ddd\"/><text x=\"{tx:.0}\" y=\"{ty:.1}\" font-size=\"11\" text-anchor=\"end\">{t:.0}</text>\n",
            y = sy(t),
            x1 = x0 + w,
            tx = x0 - 4.0,
            ty = sy(t) + 4.0
        ));
        t += step;
    }
    for g in [1u32, 8, 16, 24, 32, 40, 48, 56, 64] {
        out.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.0}\" font-size=\"11\" text-anchor=\"middle\">{g}</text>\n",
            sx(g),
            y0 + h + 16.0
        ));
    }
    out.push_str(&format!(
        "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"12\" text-anchor=\"middle\">genus g</text>\n",
        x0 + w / 2.0,
        y0 + h + 34.0
    ));
    let series: [(usize, Box<dyn Fn(&Row) -> Option<f64>>); 4] = [
        (0, Box::new(|r| r.gaudry)),
        (1, Box::new(|r| r.dlp_bare)),
        (2, Box::new(|r| r.dlp_cm)),
        (3, Box::new(|r| Some(r.subexp))),
    ];
    for (i, f) in &series {
        let pts: Vec<String> = rs
            .iter()
            .filter_map(|r| f(r).map(|v| format!("{:.1},{:.1}", sx(r.g), sy(v))))
            .collect();
        let dash = if *i == 3 && !rs.iter().any(|r| r.subexp_valid) {
            " stroke-dasharray=\"6 4\""
        } else {
            ""
        };
        out.push_str(&format!(
            "<polyline fill=\"none\" stroke=\"{}\" stroke-width=\"2\"{dash} points=\"{}\"/>\n",
            colors[*i].1,
            pts.join(" ")
        ));
    }
    out.push_str(&format!(
        "<line x1=\"{x0}\" y1=\"{y:.1}\" x2=\"{x1}\" y2=\"{y:.1}\" stroke=\"#000\" stroke-width=\"2\" stroke-dasharray=\"2 3\"/>\n",
        y = sy(rho),
        x1 = x0 + w
    ));
    out.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\">rho on E = {rho:.1}</text>\n",
        x0 + 6.0,
        sy(rho) - 5.0
    ));
    if let Some(r) = rs.iter().find(|r| r.subexp_valid) {
        out.push_str(&format!(
            "<line x1=\"{x:.1}\" y1=\"{y0}\" x2=\"{x:.1}\" y2=\"{yb:.1}\" stroke=\"{c}\" stroke-dasharray=\"3 3\"/><text x=\"{x:.1}\" y=\"{ty}\" font-size=\"10\" fill=\"{c}\">L regime starts g={g}</text>\n",
            x = sx(r.g),
            yb = y0 + h,
            c = colors[3].1,
            ty = y0 + 12.0,
            g = r.g
        ));
    } else {
        out.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{:.0}\" font-size=\"10\" fill=\"{}\">L curve dashed: outside g >= ln p = {} (formula only)</text>\n",
            x0 + 6.0,
            y0 + h - 8.0,
            colors[3].1,
            regime_start(1.0, n)
        ));
    }
}

fn render_svg(m: &CostModel) -> String {
    let colors = [
        ("Gaudry basic: g!p + p^2", "#1f77b4", ""),
        ("p^(2-2/g), constants dropped", "#2ca02c", ""),
        ("g!p + g^2 p^(2-2/g) (CostModel)", "#d62728", ""),
        ("L_{p^g}[1/2, sqrt2] (needs g >= ln p)", "#9467bd", ""),
    ];
    let mut s = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"500\" font-family=\"sans-serif\">\n<rect width=\"1000\" height=\"500\" fill=\"#fff\"/>\n",
    );
    for (x0, n) in [(50.0, 128.0), (540.0, 256.0)] {
        let rs = rows(m, n);
        svg_panel(&mut s, x0, n, &rs, log2_rho(n), &colors);
    }
    for (i, (label, col, _)) in colors.iter().enumerate() {
        let x = 50.0 + (i % 2) as f64 * 400.0;
        let y = 462.0 + (i / 2) as f64 * 18.0;
        s.push_str(&format!(
            "<line x1=\"{x}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"{col}\" stroke-width=\"2\"/><text x=\"{tx}\" y=\"{ty}\" font-size=\"12\">{label}</text>\n",
            x2 = x + 24.0,
            tx = x + 30.0,
            ty = y + 4.0
        ));
    }
    s.push_str(
        "<text x=\"810\" y=\"475\" font-size=\"12\">dotted black: rho on E, sqrt(pi p/4)</text>\n",
    );
    s.push_str(
        "<text x=\"810\" y=\"492\" font-size=\"11\">formula-level, not measured</text>\n</svg>\n",
    );
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |flag: &str| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let m = CostModel::default();
    let c = m.enge_gaudry_c;
    let mut md = String::new();
    let mut per_p = Vec::new();

    for n in P_BITS {
        let rs = rows(&m, n);
        let rho = log2_rho(n);
        md.push_str(&format!(
            "### p = 2^{n:.0}  (rho on E = {rho:.2} log2 group ops)\n\n| method | best g | log2 cost | rho | gap (bits) | weighted gap | note |\n|---|---|---|---|---|---|---|\n"
        ));
        let b1 = best_ops(&rs, |r| r.gaudry);
        let b2 = best_ops(&rs, |r| r.dlp_bare);
        let b2c = best_ops(&rs, |r| r.dlp_cm);
        let v3 = best_ops(&rs, |r| r.subexp_valid.then_some(r.subexp));
        let u3 = best_ops(&rs, |r| Some(r.subexp));
        md.push_str(&fmt_row("Gaudry basic g!p+p^2", b1, rho, "g in 2..64"));
        md.push_str(&fmt_row(
            "p^(2-2/g) bare",
            b2,
            rho,
            "constants dropped, lower bound on the family",
        ));
        md.push_str(&fmt_row("g!p+g^2 p^(2-2/g)", b2c, rho, "CostModel form"));
        md.push_str(&fmt_row(
            "L[1/2,sqrt2], g >= ln p",
            v3,
            rho,
            "in-regime, inside g<=64",
        ));
        md.push_str(&fmt_row(
            "L[1/2,sqrt2], unconstrained",
            u3,
            rho,
            "INVALID outside g >= ln p",
        ));
        let overall = [b1, b2, b2c, v3]
            .into_iter()
            .flatten()
            .min_by(|a, b| a.1.total_cmp(&b.1));
        md.push_str(&fmt_row(
            "best valid overall",
            overall,
            rho,
            "min of the rows above except unconstrained",
        ));
        md.push('\n');

        // Regime start, off-grid evaluation, and monotonicity check.
        let mut regimes = Vec::new();
        for theta in THETAS {
            let g0 = regime_start(theta, n);
            let at0 = log2_subexp(c, n, g0);
            let at0_c1 = log2_subexp(1.0, n, g0);
            let min_scan = (g0..=G_SCAN_MAX.max(g0))
                .map(|g| log2_subexp(c, n, g))
                .fold(f64::INFINITY, f64::min);
            md.push_str(&format!(
                "- theta={theta}: regime g0={g0}; L at g0 = {at0:.1} (c=1: {at0_c1:.1}); min over g0..{G_SCAN_MAX} = {min_scan:.1}; gap to rho = {:+.1} bits ({:+.1} with c=1)\n",
                at0 - rho,
                at0_c1 - rho
            ));
            regimes.push(json!({
                "theta": theta, "g0": g0, "log2_cost_at_g0": at0, "log2_cost_at_g0_c1": at0_c1,
                "min_over_scan": min_scan, "gap_bits": at0 - rho, "gap_bits_c1": at0_c1 - rho,
                "weighted_shift_bits": m.log2_jacobian_op(f64::from(g0).log2(), n) - m.log2_elliptic_op(n),
            }));
        }
        md.push('\n');

        let grid: Vec<Value> = rs
            .iter()
            .map(|r| {
                json!({"g": r.g, "gaudry_basic": opt(r.gaudry), "dlp_bare": opt(r.dlp_bare),
                    "dlp_costmodel": opt(r.dlp_cm), "subexp_L": r.subexp,
                    "subexp_in_regime_theta1": r.subexp_valid, "weighted_shift_bits": r.w_shift})
            })
            .collect();
        let pack = |b: Option<(u32, f64, f64)>| {
            b.map_or(Value::Null, |(g, cst, w)| {
                json!({"best_g": g, "log2_cost": cst, "gap_bits": cst - rho, "weighted_gap_bits": cst - rho + w})
            })
        };
        per_p.push(json!({
            "p_bits": n, "log2_rho": rho,
            "best": {"gaudry_basic": pack(b1), "dlp_bare": pack(b2), "dlp_costmodel": pack(b2c),
                "subexp_in_regime": pack(v3), "subexp_unconstrained_INVALID": pack(u3), "overall_valid": pack(overall)},
            "subexp_regimes": regimes, "grid": grid,
        }));
    }
    print!("{md}");

    if let Some(path) = arg("--json") {
        let doc = json!({
            "label": "formula-level, not measured",
            "unit": "log2 group-operation equivalents; weighted_* adds log2(Jacobian op / elliptic op) from CostModel",
            "enge_gaudry_c": c, "regime_thetas": THETAS, "g_scan_max": G_SCAN_MAX,
            "dlp_subgroup_bits": "~log2 p (order r ~ p); Jacobian order ~ g*log2 p bits",
            "per_p": per_p,
        });
        fs::write(&path, serde_json::to_string_pretty(&doc).expect("json")).expect("write json");
    }
    if let Some(path) = arg("--svg") {
        fs::write(&path, render_svg(&m)).expect("write svg");
    }
}

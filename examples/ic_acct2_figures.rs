//! Figures of `research/ic_acct2_setup_20261021/` (README.md, REPORT.typ),
//! drawn as SVG from the study's committed `analysis/` files only: the
//! sealed bounds `analysis/bounds/<arm>-<basis>-<lo>-<hi>.json` and the
//! `ecbench amortize` reports `analysis/amortize/*.json`.  It recomputes no
//! fit and takes no measurement; the one derived series is the non-search
//! share `1 − search/total` of two committed per-size means.
//!
//! ```sh
//! cargo run --release --example ic_acct2_figures -- research/ic_acct2_setup_20261021
//! ```
//!
//! Writes `figures/{alpha_forest,rho_neg_cost_by_basis,ic_rho_ratios}.svg`
//! and renders each to a 150 dpi PNG beside it with `rsvg-convert` (librsvg),
//! so the PNG comes from the same source as the SVG.

use serde_json::Value;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const C_TOTAL: &str = "#4C6EF5";
const C_PER_TARGET: &str = "#868E96";
const C_SEARCH: &str = "#E8590C";
const C_SHARE: &str = "#2B8A3E";
const FONT: &str = "Helvetica,Arial,sans-serif";

fn load(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn f(v: &Value) -> f64 {
    v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .unwrap_or_else(|| panic!("not a number: {v}"))
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// An SVG document with a white background.
struct Svg {
    w: f64,
    h: f64,
    body: String,
}

impl Svg {
    fn new(w: f64, h: f64) -> Self {
        Svg {
            w,
            h,
            body: String::new(),
        }
    }
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, width: f64, dash: &str) {
        let dash = if dash.is_empty() {
            String::new()
        } else {
            format!(r#" stroke-dasharray="{dash}""#)
        };
        writeln!(
            self.body,
            r#"<line x1="{x1:.2}" y1="{y1:.2}" x2="{x2:.2}" y2="{y2:.2}" stroke="{stroke}" stroke-width="{width}"{dash}/>"#
        )
        .unwrap();
    }
    fn text(&mut self, x: f64, y: f64, size: f64, anchor: &str, fill: &str, s: &str) {
        writeln!(
            self.body,
            r#"<text x="{x:.2}" y="{y:.2}" font-size="{size}" text-anchor="{anchor}" fill="{fill}">{}</text>"#,
            esc(s)
        )
        .unwrap();
    }
    fn text_rot(&mut self, x: f64, y: f64, size: f64, s: &str) {
        writeln!(
            self.body,
            r#"<text x="{x:.2}" y="{y:.2}" font-size="{size}" text-anchor="middle" transform="rotate(-90 {x:.2} {y:.2})">{}</text>"#,
            esc(s)
        )
        .unwrap();
    }
    /// A marker: `o` circle, `s` square, `^` triangle.
    fn marker(&mut self, x: f64, y: f64, shape: char, r: f64, fill: &str) {
        match shape {
            's' => writeln!(
                self.body,
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{fill}"/>"#,
                x - r,
                y - r,
                2.0 * r,
                2.0 * r
            ),
            '^' => writeln!(
                self.body,
                r#"<polygon points="{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}" fill="{fill}"/>"#,
                x,
                y - 1.2 * r,
                x - 1.1 * r,
                y + 0.8 * r,
                x + 1.1 * r,
                y + 0.8 * r
            ),
            _ => writeln!(
                self.body,
                r#"<circle cx="{x:.2}" cy="{y:.2}" r="{r:.2}" fill="{fill}"/>"#
            ),
        }
        .unwrap();
    }
    fn polyline(&mut self, pts: &[(f64, f64)], stroke: &str, dash: &str) {
        let p: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.2},{y:.2}")).collect();
        let dash = if dash.is_empty() {
            String::new()
        } else {
            format!(r#" stroke-dasharray="{dash}""#)
        };
        writeln!(
            self.body,
            r#"<polyline points="{}" fill="none" stroke="{stroke}" stroke-width="1.5"{dash}/>"#,
            p.join(" ")
        )
        .unwrap();
    }
    fn write(&self, path: &Path) {
        let doc = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}" font-family="{FONT}" font-size="11">
<rect width="{w}" height="{h}" fill="#ffffff"/>
{}</svg>
"##,
            self.body,
            w = self.w,
            h = self.h
        );
        fs::write(path, doc).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    }
}

/// A plotting frame: a data rectangle mapped onto a pixel rectangle, with
/// a linear x axis and a linear or base-10 log y axis.
struct Frame {
    px: (f64, f64, f64, f64), // left, top, right, bottom
    x: (f64, f64),
    y: (f64, f64),
    log_y: bool,
}

impl Frame {
    fn sx(&self, x: f64) -> f64 {
        self.px.0 + (x - self.x.0) / (self.x.1 - self.x.0) * (self.px.2 - self.px.0)
    }
    fn sy(&self, y: f64) -> f64 {
        let (y, lo, hi) = if self.log_y {
            (y.log10(), self.y.0.log10(), self.y.1.log10())
        } else {
            (y, self.y.0, self.y.1)
        };
        self.px.3 - (y - lo) / (hi - lo) * (self.px.3 - self.px.1)
    }
    /// Box, ticks, tick labels and axis labels.
    fn axes(&self, s: &mut Svg, xticks: &[f64], xlabel: &str, ylabel: &str, title: &str) {
        let (l, t, r, b) = self.px;
        writeln!(
            s.body,
            r##"<rect x="{l:.2}" y="{t:.2}" width="{:.2}" height="{:.2}" fill="none" stroke="#000000" stroke-width="0.8"/>"##,
            r - l,
            b - t
        )
        .unwrap();
        for &x in xticks {
            let px = self.sx(x);
            s.line(px, b, px, b + 4.0, "#000000", 0.8, "");
            s.text(px, b + 15.0, 10.0, "middle", "#000000", &tick(x));
        }
        for y in self.yticks() {
            let py = self.sy(y);
            s.line(l - 4.0, py, l, py, "#000000", 0.8, "");
            let label = if self.log_y {
                format!("1e{}", y.log10().round() as i64)
            } else {
                tick(y)
            };
            s.text(l - 7.0, py + 3.5, 10.0, "end", "#000000", &label);
        }
        s.text((l + r) / 2.0, b + 32.0, 11.0, "middle", "#000000", xlabel);
        if !ylabel.is_empty() {
            s.text_rot(l - 46.0, (t + b) / 2.0, 11.0, ylabel);
        }
        s.text((l + r) / 2.0, t - 8.0, 12.0, "middle", "#000000", title);
    }
    fn yticks(&self) -> Vec<f64> {
        if self.log_y {
            let (a, b) = (
                self.y.0.log10().ceil() as i32,
                self.y.1.log10().floor() as i32,
            );
            (a..=b).map(|e| 10f64.powi(e)).collect()
        } else {
            nice_ticks(self.y.0, self.y.1)
        }
    }
}

fn tick(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        let s = format!("{v:.3}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn nice_ticks(lo: f64, hi: f64) -> Vec<f64> {
    let raw = (hi - lo) / 5.0;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0]
        .iter()
        .map(|m| m * mag)
        .find(|s| *s >= raw)
        .unwrap();
    let mut v = (lo / step).ceil() * step;
    let mut out = Vec::new();
    while v <= hi + 1e-12 {
        out.push((v / step).round() * step);
        v += step;
    }
    out
}

/// The log-axis range covering `vals`, padded to whole decades.
fn log_range(vals: impl Iterator<Item = f64>) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for v in vals {
        lo = lo.min(v);
        hi = hi.max(v);
    }
    (
        10f64.powf((lo.log10() - 0.15).floor()),
        10f64.powf((hi.log10() + 0.15).ceil()),
    )
}

/// A legend box at `(x, y)` (top left): `(marker, colour, dash, label)`.
fn legend(s: &mut Svg, x: f64, y: f64, items: &[(char, &str, &str, &str)]) {
    let w = 8.0 + items.iter().map(|i| i.3.chars().count()).max().unwrap_or(0) as f64 * 5.6 + 34.0;
    let h = 8.0 + 15.0 * items.len() as f64;
    writeln!(
        s.body,
        r##"<rect x="{x:.2}" y="{y:.2}" width="{w:.2}" height="{h:.2}" fill="#ffffff" stroke="#cccccc" stroke-width="0.8"/>"##
    )
    .unwrap();
    for (i, (m, c, dash, label)) in items.iter().enumerate() {
        let yy = y + 12.0 + 15.0 * i as f64;
        s.line(x + 6.0, yy, x + 28.0, yy, c, 1.5, dash);
        s.marker(x + 17.0, yy, *m, 3.0, c);
        s.text(x + 34.0, yy + 3.5, 10.0, "start", "#000000", label);
    }
}

fn sizes(dir: &Path, name: &str) -> Vec<(f64, f64)> {
    load(&dir.join(format!("analysis/bounds/{name}.json")))["sizes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| (f(&s["log2_r"]), f(&s["mean_gae"])))
        .collect()
}

/// 1. Exponent α in additions with its 95% CI, total vs search basis, for
///    the three generic arms on each fitted range.
fn alpha_forest(dir: &Path, out: &Path) {
    let arms = ["rho-neg", "bsgs-neg", "kangaroo"];
    let ranges = ["16-22", "23-32", "16-26", "33-40"];
    let mut rows = Vec::new(); // (label, [(basis, a, lo, hi)])
    for arm in arms {
        for rng in ranges {
            let mut pts = Vec::new();
            for basis in ["total", "search"] {
                let b = load(&dir.join(format!("analysis/bounds/{arm}-{basis}-{rng}.json")));
                let fit = &b["fit"];
                let ci = fit["alpha_in_additions_ci95"].as_array().unwrap();
                pts.push((basis, f(&fit["alpha_in_additions"]), f(&ci[0]), f(&ci[1])));
            }
            rows.push((format!("{arm} {rng}"), pts));
        }
    }
    let (lo, hi) = rows
        .iter()
        .flat_map(|r| r.1.iter())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), p| {
            (l.min(p.2), h.max(p.3))
        });
    let (x0, x1) = ((lo * 20.0).floor() / 20.0, (hi * 20.0).ceil() / 20.0);
    let mut s = Svg::new(750.0, 520.0);
    // Rows top to bottom, with half a row between arms.
    let n = rows.len() as f64 + 0.5 * (arms.len() - 1) as f64;
    let fr = Frame {
        px: (110.0, 34.0, 730.0, 470.0),
        x: (x0, x1),
        y: (0.0, n + 1.0),
        log_y: false,
    };
    let row_y = |i: usize| -> f64 {
        let gap = 0.5 * (i / ranges.len()) as f64;
        fr.sy(n - (i as f64 + gap))
    };
    let (l, t, r, b) = fr.px;
    writeln!(
        s.body,
        r##"<rect x="{l:.2}" y="{t:.2}" width="{:.2}" height="{:.2}" fill="none" stroke="#000000" stroke-width="0.8"/>"##,
        r - l,
        b - t
    )
    .unwrap();
    for x in nice_ticks(x0, x1) {
        let px = fr.sx(x);
        s.line(px, b, px, b + 4.0, "#000000", 0.8, "");
        s.text(px, b + 15.0, 10.0, "middle", "#000000", &tick(x));
    }
    s.line(fr.sx(0.5), t, fr.sx(0.5), b, "#000000", 0.8, "5,3");
    for (i, (label, pts)) in rows.iter().enumerate() {
        let y = row_y(i);
        s.line(l - 4.0, y, l, y, "#000000", 0.8, "");
        s.text(l - 7.0, y + 3.5, 9.0, "end", "#000000", label);
        for &(basis, a, lo, hi) in pts {
            let (c, dy) = if basis == "total" {
                (C_TOTAL, -5.0)
            } else {
                (C_SEARCH, 5.0)
            };
            let yy = y + dy;
            s.line(fr.sx(lo), yy, fr.sx(hi), yy, c, 1.2, "");
            s.line(fr.sx(lo), yy - 2.5, fr.sx(lo), yy + 2.5, c, 1.2, "");
            s.line(fr.sx(hi), yy - 2.5, fr.sx(hi), yy + 2.5, c, 1.2, "");
            s.marker(fr.sx(a), yy, 'o', 3.2, c);
        }
    }
    s.text(
        (l + r) / 2.0,
        b + 32.0,
        11.0,
        "middle",
        "#000000",
        "exponent α in additions (95% CI)",
    );
    s.text(
        (l + r) / 2.0,
        t - 10.0,
        12.0,
        "middle",
        "#000000",
        "Generic arms: total vs search basis (bitop/v1)",
    );
    legend(
        &mut s,
        r - 100.0,
        b - 44.0,
        &[('o', C_TOTAL, "", "total"), ('o', C_SEARCH, "", "search")],
    );
    s.write(out);
}

/// 2. rho-neg per-size mean gates on the total and search bases (16–26
///    bits), and the non-search share `1 − search/total`.
fn rho_neg_cost_by_basis(dir: &Path, out: &Path) {
    let tot = sizes(dir, "rho-neg-total-16-26");
    let sea = sizes(dir, "rho-neg-search-16-26");
    assert_eq!(tot.len(), sea.len(), "the two fits read the same sizes");
    for (a, b) in tot.iter().zip(&sea) {
        assert_eq!(a.0, b.0, "the two fits read the same sizes");
    }
    let share: Vec<(f64, f64)> = tot
        .iter()
        .zip(&sea)
        .map(|(t, s)| (t.0, 1.0 - s.1 / t.1))
        .collect();
    let xr = (15.5, 26.5);
    let xticks = [16.0, 18.0, 20.0, 22.0, 24.0, 26.0];
    let mut s = Svg::new(900.0, 360.0);

    let left = Frame {
        px: (75.0, 30.0, 430.0, 300.0),
        x: xr,
        y: log_range(tot.iter().chain(&sea).map(|p| p.1)),
        log_y: true,
    };
    left.axes(
        &mut s,
        &xticks,
        "log2 r",
        "mean gates per solve",
        "rho-neg, 16–26 bits",
    );
    let (_, t, _, b) = left.px;
    s.line(left.sx(22.5), t, left.sx(22.5), b, "#808080", 0.8, "2,2");
    s.text(
        left.sx(22.6),
        b - 8.0,
        9.0,
        "start",
        "#808080",
        "8 → 16 jumps",
    );
    for (pts, c, m) in [(&tot, C_TOTAL, 'o'), (&sea, C_SEARCH, 's')] {
        let px: Vec<(f64, f64)> = pts.iter().map(|p| (left.sx(p.0), left.sy(p.1))).collect();
        s.polyline(&px, c, "");
        for p in &px {
            s.marker(p.0, p.1, m, 3.2, c);
        }
    }
    legend(
        &mut s,
        left.px.0 + 8.0,
        left.px.1 + 8.0,
        &[('o', C_TOTAL, "", "total"), ('s', C_SEARCH, "", "search")],
    );

    let smax = share.iter().map(|p| p.1).fold(0.0, f64::max);
    let right = Frame {
        px: (525.0, 30.0, 880.0, 300.0),
        x: xr,
        y: (0.0, (smax * 10.0).ceil() / 10.0),
        log_y: false,
    };
    right.axes(
        &mut s,
        &xticks,
        "log2 r",
        "non-search share of total",
        "set-up + verification + natives",
    );
    let (_, t, _, b) = right.px;
    s.line(right.sx(22.5), t, right.sx(22.5), b, "#808080", 0.8, "2,2");
    let px: Vec<(f64, f64)> = share
        .iter()
        .map(|p| (right.sx(p.0), right.sy(p.1)))
        .collect();
    s.polyline(&px, C_SHARE, "");
    for p in &px {
        s.marker(p.0, p.1, 'o', 3.2, C_SHARE);
    }
    s.write(out);
}

/// 3. ic-pair-rule over rho-neg per curve: the cold ratio and the
///    per-target ratio under the stored and the target-aware split.
fn ic_rho_ratios(dir: &Path, out: &Path) {
    let curves = |name: &str| -> Vec<(String, f64, f64, f64)> {
        load(&dir.join(format!("analysis/amortize/{name}.json")))["curves"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| {
                (
                    c["curve"].as_str().unwrap().to_string(),
                    f(&c["log2_r"]),
                    f(&c["cold_ratio"]),
                    f(&c["per_target_ratio"]),
                )
            })
            .collect()
    };
    let stored = curves("ic-pair-rule-stored-split");
    let aware = curves("ic-pair-rule-target-aware");
    assert_eq!(stored.len(), aware.len());
    for (a, b) in stored.iter().zip(&aware) {
        assert_eq!(a.0, b.0, "the two reports read the same curves");
    }
    let cold: Vec<(f64, f64)> = aware.iter().map(|c| (c.1, c.2)).collect();
    let ps: Vec<(f64, f64)> = stored.iter().map(|c| (c.1, c.3)).collect();
    let pa: Vec<(f64, f64)> = aware.iter().map(|c| (c.1, c.3)).collect();
    let (xlo, xhi) = cold
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), p| {
            (l.min(p.0), h.max(p.0))
        });
    let yr = log_range(cold.iter().chain(&ps).chain(&pa).map(|p| p.1).chain([1.0]));
    let mut s = Svg::new(650.0, 360.0);
    let fr = Frame {
        px: (75.0, 30.0, 630.0, 300.0),
        x: (xlo.floor() - 0.5, xhi.ceil() + 0.5),
        y: yr,
        log_y: true,
    };
    let xticks: Vec<f64> = nice_ticks(xlo.floor(), xhi.ceil());
    fr.axes(
        &mut s,
        &xticks,
        "log2 r",
        "ic-pair-rule / rho-neg (gates)",
        "IC over rho: the correction lowers the per-target ratio",
    );
    s.line(fr.px.0, fr.sy(1.0), fr.px.2, fr.sy(1.0), "#808080", 0.8, "");
    let series = [
        (&cold, "#000000", 'o', "", "cold (total/total)"),
        (&ps, C_PER_TARGET, 's', "5,3", "per-target, stored split"),
        (&pa, C_SEARCH, '^', "", "per-target, target-aware"),
    ];
    for (pts, c, m, dash, _) in &series {
        let px: Vec<(f64, f64)> = pts.iter().map(|p| (fr.sx(p.0), fr.sy(p.1))).collect();
        s.polyline(&px, c, dash);
        for p in &px {
            s.marker(p.0, p.1, *m, 2.4, c);
        }
    }
    let items: Vec<(char, &str, &str, &str)> = series
        .iter()
        .map(|(_, c, m, d, l)| (*m, *c, *d, *l))
        .collect();
    legend(&mut s, fr.px.2 - 190.0, fr.px.3 - 58.0, &items);
    s.write(out);
}

fn main() {
    let dir: PathBuf = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "research/ic_acct2_setup_20261021".into())
        .into();
    let figs = dir.join("figures");
    fs::create_dir_all(&figs).unwrap();
    let jobs: [(&str, fn(&Path, &Path)); 3] = [
        ("alpha_forest", alpha_forest),
        ("rho_neg_cost_by_basis", rho_neg_cost_by_basis),
        ("ic_rho_ratios", ic_rho_ratios),
    ];
    for (name, draw) in jobs {
        let svg = figs.join(format!("{name}.svg"));
        let png = figs.join(format!("{name}.png"));
        draw(&dir, &svg);
        // 150 dpi: the SVG is laid out in CSS pixels (96 per inch).
        let status = std::process::Command::new("rsvg-convert")
            .arg("--zoom")
            .arg(format!("{}", 150.0 / 96.0))
            .arg("--background-color=white")
            .arg("-o")
            .arg(&png)
            .arg(&svg)
            .status()
            .unwrap_or_else(|e| panic!("rsvg-convert (librsvg) is needed for the PNG: {e}"));
        assert!(status.success(), "rsvg-convert failed on {}", svg.display());
        println!("{}  {}", svg.display(), png.display());
    }
}

//! Roofline SVG for optlevel records.
//!
//! Log-log axes: x = arithmetic intensity in operations per byte of memory
//! traffic, y = operations per second.  One roof per operation class (solid:
//! one core, dashed: all cores), one marker per record at its single-thread
//! point (filled) and its full-parallel point (ring).  Records whose traffic is
//! unknown or whose working set stays in cache are drawn at the right edge and
//! say so.
//!
//! Colours are the first three categorical slots of the lab's validated chart
//! palette (blue, orange, aqua), which pass the all-pairs colour-vision checks
//! on both surfaces; a fourth class folds into grey "other".  Every element
//! carries a light-mode presentation attribute and a class that the embedded
//! stylesheet re-colours under `prefers-color-scheme: dark`, so the file reads
//! on both backgrounds and in renderers that ignore stylesheets.
//!
//! The output is byte-identical to the tool's first (Python) version for the
//! same inputs (tests/golden.rs).
use crate::score::{class_entry, f};
use crate::util::{html_escape as esc, py_g3, py_repr};
use serde_json::Value;
use std::path::Path;

struct Pal {
    surface: &'static str,
    text: &'static str,
    text2: &'static str,
    muted: &'static str,
    grid: &'static str,
    axis: &'static str,
    s: [&'static str; 4],
}
const LIGHT: Pal = Pal {
    surface: "#fcfcfb",
    text: "#0b0b0b",
    text2: "#52514e",
    muted: "#898781",
    grid: "#e4e3dd",
    axis: "#c3c2b7",
    s: ["#2a78d6", "#eb6834", "#1baf7a", "#898781"],
};
const DARK: Pal = Pal {
    surface: "#1a1a19",
    text: "#f0efec",
    text2: "#c3c2b7",
    muted: "#898781",
    grid: "#2c2c2a",
    axis: "#383835",
    s: ["#3987e5", "#d95926", "#199e70", "#898781"],
};
const W: f64 = 920.0;
const H: f64 = 560.0;
const ML: f64 = 86.0;
const MR: f64 = 250.0;
const MT: f64 = 54.0;
const MB: f64 = 62.0;

fn fmt(v: f64) -> String {
    for (div, suf) in [(1e12, "T"), (1e9, "G"), (1e6, "M"), (1e3, "k")] {
        if v.abs() >= div {
            return format!("{}{suf}", py_g3(v / div));
        }
    }
    py_g3(v)
}

fn css() -> String {
    fn block(p: &Pal, pre: &str) -> String {
        let mut out = format!("{pre}.surface{{fill:{}}}{pre}.t1{{fill:{}}}{pre}.t2{{fill:{}}}{pre}.mut{{fill:{}}}{pre}.grid{{stroke:{}}}{pre}.axis{{stroke:{}}}{pre}.ring{{stroke:{}}}{pre}.halo{{stroke:{}}}",
                              p.surface, p.text, p.text2, p.muted, p.grid, p.axis, p.surface, p.surface);
        for i in 0..4 {
            out += &format!(
                "{pre}.l{i}{{stroke:{}}}{pre}.f{i}{{fill:{}}}",
                p.s[i], p.s[i]
            );
        }
        out
    }
    format!("text{{font-family:system-ui,-apple-system,'Segoe UI',Helvetica,Arial,sans-serif}}{}@media (prefers-color-scheme: dark){{{}}}", block(&LIGHT, ""), block(&DARK, ""))
}

struct Pt {
    label: String,
    cls: String,
    x: Option<f64>,
    y: f64,
    lo: f64,
    hi: f64,
    single: bool,
    att: f64,
    tier: String,
    threads: i64,
    incache: bool,
}

/// An integer-valued coordinate the way Python prints an int.
fn int(v: f64) -> String {
    format!("{}", v as i64)
}

pub fn roofline_svg(calib: &Value, records: &[Value], title: &str) -> Result<String, String> {
    // a record's roof is its class's calibration entry (or its alias's)
    let roof_class = |r: &Value| -> Option<String> {
        class_entry(calib, r["kernel"]["op_class"].as_str()?)
            .ok()
            .map(|(_, n)| n)
    };
    let mut classes: Vec<String> = Vec::new();
    for r in records {
        if let Some(c) = roof_class(r) {
            if !classes.contains(&c) {
                classes.push(c);
            }
        }
    }
    let slot = |c: &str| classes.iter().position(|x| x == c).unwrap_or(0).min(3);
    let bw = &calib["memory"]["bandwidth_bytes_per_s"];
    let mut ks: Vec<i64> = bw
        .as_object()
        .map(|o| o.keys().filter_map(|k| k.parse().ok()).collect())
        .unwrap_or_default();
    ks.sort();
    let (Some(&k1), Some(&kn)) = (ks.first(), ks.last()) else {
        return Err("calibration has no bandwidth points".into());
    };
    let bwmax = |k: i64| f(&bw[k.to_string()], "max").unwrap_or(f64::NAN);
    let peak = |c: &str, k: i64| {
        f(
            &calib["compute"][c]["total_ops_per_s"][k.to_string()],
            "max",
        )
        .unwrap_or(f64::NAN)
    };
    let mut pts: Vec<Pt> = Vec::new();
    for r in records {
        let Some(c) = roof_class(r) else { continue };
        let opb = r["intensity"]["ops_per_byte"].as_f64();
        for name in ["single", "full"] {
            let Some(att) = r["attainment"].get(name) else {
                continue;
            };
            let ach = if name == "single" {
                &r["achieved"]["single"]
            } else {
                &r["achieved"][format!("threads_{}", r["roofline"]["full"]["threads"])]
            };
            let s = &ach["ops_per_s"];
            let label = r["label"]
                .as_str()
                .filter(|l| !l.is_empty())
                .unwrap_or(&c)
                .to_string();
            pts.push(Pt {
                label,
                cls: c.clone(),
                x: opb,
                y: f(s, "median").unwrap_or(f64::NAN),
                lo: f(s, "min").unwrap_or(f64::NAN),
                hi: f(s, "max").unwrap_or(f64::NAN),
                single: name == "single",
                att: f(att, "value").unwrap_or(f64::NAN),
                tier: r["tier"].as_str().unwrap_or("").to_string(),
                threads: r["roofline"][name]["threads"].as_i64().unwrap_or(0),
                incache: r["intensity"]["in_cache"].as_bool().unwrap_or(false),
            });
        }
    }
    if classes.is_empty() {
        return Err("no record has an operation class the calibration knows".into());
    }
    let mut peaks = Vec::new();
    let mut knees = Vec::new();
    for c in &classes {
        for k in [k1, kn] {
            peaks.push(peak(c, k));
            knees.push(peak(c, k) / bwmax(k));
        }
    }
    let xs: Vec<f64> = pts
        .iter()
        .filter_map(|p| p.x.filter(|x| *x != 0.0))
        .chain(knees.iter().copied())
        .collect();
    let fold = |v: &[f64], init: f64, g: fn(f64, f64) -> f64| v.iter().copied().fold(init, g);
    let xmin = 10f64.powf((fold(&xs, f64::INFINITY, f64::min) / 4.0).log10().floor());
    let xmax = 10f64.powf(
        (fold(&xs, f64::NEG_INFINITY, f64::max) * 4.0)
            .log10()
            .ceil(),
    );
    let mut ys: Vec<f64> = pts.iter().map(|p| p.lo).collect();
    ys.push(xmin * bwmax(k1));
    let ymin = 10f64.powf((fold(&ys, f64::INFINITY, f64::min) / 1.5).log10().floor());
    let mut tops = peaks.clone();
    tops.extend(pts.iter().map(|p| p.hi));
    let ymax = 10f64.powf(
        (fold(&tops, f64::NEG_INFINITY, f64::max) * 1.2)
            .log10()
            .ceil(),
    );
    let (pw, ph) = (W - ML - MR, H - MT - MB);
    let x_of = |v: f64| ML + pw * (v.log10() - xmin.log10()) / (xmax.log10() - xmin.log10());
    let y_of = |v: f64| {
        let v = v.max(ymin).min(ymax);
        MT + ph * (1.0 - (v.log10() - ymin.log10()) / (ymax.log10() - ymin.log10()))
    };
    let (w, h, ml, mt) = (int(W), int(H), int(ML), int(MT));
    let mut o: Vec<String> = vec![
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" aria-label=\"{}: roofline, operations per second against operations per byte\">", esc(title)),
        format!("<style>{}</style>", css()),
        format!("<rect class=\"surface\" x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" fill=\"{}\"/>", LIGHT.surface),
        format!("<text class=\"t1\" x=\"{ml}\" y=\"24\" font-size=\"16\" font-weight=\"600\" fill=\"{}\">{}</text>", LIGHT.text, esc(title)),
        format!("<text class=\"t2\" x=\"{ml}\" y=\"42\" font-size=\"11.5\" fill=\"{}\">host {} ({}); attainment = marker / roof at its intensity; not a speed claim</text>",
                LIGHT.text2, esc(calib["host"]["cpu_model"].as_str().unwrap_or("")), calib["host"]["fingerprint"].as_str().unwrap_or("")),
    ];
    let mut e = xmin.log10().floor() as i32;
    while 10f64.powf(e as f64) <= xmax * 1.0001 {
        let x = x_of(10f64.powf(e as f64));
        o.push(format!("<line class=\"grid\" x1=\"{x:.1}\" y1=\"{mt}\" x2=\"{x:.1}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>", int(MT + ph), LIGHT.grid));
        o.push(format!("<text class=\"mut\" x=\"{x:.1}\" y=\"{}\" font-size=\"11\" text-anchor=\"middle\" fill=\"{}\">{}</text>", int(MT + ph + 18.0), LIGHT.muted, fmt(10f64.powf(e as f64))));
        e += 1;
    }
    let mut e = ymin.log10().floor() as i32;
    while 10f64.powf(e as f64) <= ymax * 1.0001 {
        let y = y_of(10f64.powf(e as f64));
        o.push(format!("<line class=\"grid\" x1=\"{ml}\" y1=\"{y:.1}\" x2=\"{}\" y2=\"{y:.1}\" stroke=\"{}\" stroke-width=\"1\"/>", int(ML + pw), LIGHT.grid));
        o.push(format!("<text class=\"mut\" x=\"{}\" y=\"{:.1}\" font-size=\"11\" text-anchor=\"end\" fill=\"{}\">{}</text>", int(ML - 8.0), y + 4.0, LIGHT.muted, fmt(10f64.powf(e as f64))));
        e += 1;
    }
    o.push(format!("<line class=\"axis\" x1=\"{ml}\" y1=\"{0}\" x2=\"{1}\" y2=\"{0}\" stroke=\"{2}\" stroke-width=\"1\"/>", int(MT + ph), int(ML + pw), LIGHT.axis));
    o.push(format!("<text class=\"t2\" x=\"{}\" y=\"{}\" font-size=\"12\" text-anchor=\"middle\" fill=\"{}\">arithmetic intensity (operations per byte of memory traffic)</text>",
                   py_repr(ML + pw / 2.0), int(H - 14.0), LIGHT.text2));
    o.push(format!("<text class=\"t2\" transform=\"translate(18 {}) rotate(-90)\" font-size=\"12\" text-anchor=\"middle\" fill=\"{}\">operations per second</text>",
                   py_repr(MT + ph / 2.0), LIGHT.text2));
    // roofs
    for c in &classes {
        let i = slot(c);
        let mut lines = vec![(k1, "")];
        if kn != k1 {
            lines.push((kn, " stroke-dasharray=\"6 4\""));
        }
        for (k, dash) in lines {
            let (pk, b) = (peak(c, k), bwmax(k));
            let knee = (pk / b).max(xmin).min(xmax);
            let d = format!(
                "M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1}",
                x_of(xmin),
                y_of(xmin * b),
                x_of(knee),
                y_of(pk.min(knee * b)),
                x_of(xmax),
                y_of(pk)
            );
            o.push(format!("<path class=\"l{i}\" d=\"{d}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"{dash}><title>{} roof, {k} core(s): peak {} ops/s, bandwidth {} B/s</title></path>",
                           LIGHT.s[i], esc(c), fmt(pk), fmt(b)));
        }
    }
    // markers, with direct labels stacked top to bottom in marker order
    let mut geo: Vec<(f64, f64, bool, &Pt)> = pts
        .iter()
        .map(|p| {
            (
                y_of(p.y),
                p.x.map(x_of).unwrap_or(ML + pw - 10.0),
                p.x.is_none(),
                p,
            )
        })
        .collect();
    geo.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let (mut last_right, mut last_left) = (-1e9f64, -1e9f64);
    for (y, x, unknown, p) in geo {
        let i = slot(&p.cls);
        let col = LIGHT.s[i];
        let which = if p.single { "single" } else { "full" };
        let tip = format!(
            "{} ({which}, {} thread(s), {}): {} ops/s [{}, {}], attainment {:.3}{}",
            p.label,
            p.threads,
            p.tier,
            fmt(p.y),
            fmt(p.lo),
            fmt(p.hi),
            p.att,
            if p.incache {
                "; working set in cache, no DRAM traffic, drawn at the right edge"
            } else if unknown {
                "; memory traffic unknown, drawn at the right edge"
            } else {
                ""
            }
        );
        o.push(format!("<line class=\"l{i}\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{col}\" stroke-width=\"2\"/>", y_of(p.lo), y_of(p.hi)));
        if p.single {
            o.push(format!("<circle class=\"f{i} ring\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"5.5\" fill=\"{col}\" stroke=\"{}\" stroke-width=\"2\"><title>{}</title></circle>", LIGHT.surface, esc(&tip)));
        } else {
            o.push(format!("<circle class=\"l{i} surface\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"5\" fill=\"{}\" stroke=\"{col}\" stroke-width=\"2\"><title>{}</title></circle>", LIGHT.surface, esc(&tip)));
        }
        let right = x > ML + pw - 170.0;
        let ly = (y + 4.0).max(if right { last_right } else { last_left } + 14.0);
        if right {
            last_right = ly;
        } else {
            last_left = ly;
        }
        let tag = if p.single {
            "1t".to_string()
        } else {
            format!("{}t", p.threads)
        };
        let (anchor, lx) = if right {
            ("end", x - 22.0)
        } else {
            ("start", x + 22.0)
        };
        let ex = if right { x - 7.0 } else { x + 7.0 };
        o.push(format!("<path class=\"axis\" d=\"M{ex:.1},{y:.1} L{:.1},{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1\"/>", lx + if right { 4.0 } else { -4.0 }, ly - 4.0, LIGHT.axis));
        o.push(format!("<text class=\"t1 halo\" x=\"{lx:.1}\" y=\"{ly:.1}\" font-size=\"11\" text-anchor=\"{anchor}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"3\" paint-order=\"stroke\" stroke-linejoin=\"round\">{} {tag}: {:.2}{}</text>",
                       LIGHT.text, LIGHT.surface, esc(&p.label), p.att, if !unknown { "" } else if p.incache { " (in cache)" } else { "*" }));
    }
    // legend
    let (lx, mut ly) = ((ML + pw + 18.0) as i64, (MT + 6.0) as i64);
    o.push(format!("<text class=\"t2\" x=\"{lx}\" y=\"{ly}\" font-size=\"11.5\" font-weight=\"600\" fill=\"{}\">roof per operation class</text>", LIGHT.text2));
    for c in &classes {
        let i = slot(c);
        ly += 20;
        o.push(format!("<line class=\"l{i}\" x1=\"{lx}\" y1=\"{0}\" x2=\"{1}\" y2=\"{0}\" stroke=\"{2}\" stroke-width=\"2\"/>", ly - 4, lx + 26, LIGHT.s[i]));
        o.push(format!(
            "<text class=\"t1\" x=\"{}\" y=\"{ly}\" font-size=\"11.5\" fill=\"{}\">{}</text>",
            lx + 32,
            LIGHT.text,
            esc(c)
        ));
    }
    let notes = [
        format!("solid: {k1} core"),
        if kn != k1 {
            format!("dashed: all {kn} cores")
        } else {
            String::new()
        },
        "filled marker: single thread".into(),
        "ring: full parallelism".into(),
        "bar: min to max over runs".into(),
        "label: attainment".into(),
        "in cache: no DRAM traffic (right edge)".into(),
        "*: traffic unknown (right edge)".into(),
    ];
    ly += 14;
    for n in notes.iter().filter(|n| !n.is_empty()) {
        ly += 16;
        o.push(format!(
            "<text class=\"t2\" x=\"{lx}\" y=\"{ly}\" font-size=\"11\" fill=\"{}\">{}</text>",
            LIGHT.text2,
            esc(n)
        ));
    }
    o.push("</svg>".into());
    Ok(o.join("\n") + "\n")
}

pub fn write_roofline(
    calib: &Value,
    records: &[Value],
    out: &Path,
    title: &str,
) -> Result<(), String> {
    let svg = roofline_svg(calib, records, title)?;
    if let Some(p) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(p).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    std::fs::write(out, svg).map_err(|e| format!("{}: {e}", out.display()))
}

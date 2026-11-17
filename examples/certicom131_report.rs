//! Aggregates the recorded runs of `research/certicom131_ic_program_20261020/`
//! into the tables and SVG figures of the program report.  Reads recorded
//! values only; computes no new measurement.
//!
//! ```sh
//! cargo run --release --example certicom131_report -- research/certicom131_ic_program_20261020
//! ```
//!
//! Inputs: `runs/c*.jsonl` (Falsifier A, `certicom131_fppr_dreg`),
//! `runs/fb_*.jsonl` (Falsifier B, `certicom131_prime_combined_count`),
//! `gates.json` (`certicom131_ic_gates`).  Outputs: `tables/*.md`,
//! `figures/*.svg` (the PNG and PDF are rendered from these).

use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

fn lines(dir: &Path, prefix: &str) -> Vec<Value> {
    let mut out = Vec::new();
    let mut names: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "jsonl")
                && p.file_name().unwrap().to_string_lossy().starts_with(prefix)
        })
        .collect();
    names.sort();
    for p in names {
        for l in fs::read_to_string(&p).unwrap().lines() {
            if let Ok(v) = serde_json::from_str::<Value>(l) {
                out.push(v);
            }
        }
    }
    out
}

/// A cell's reading of one outcome object: (degree, is_exact).
fn reading(o: &Value) -> Option<(u32, bool)> {
    match o["kind"].as_str()? {
        "resolved" => Some((o["degree"].as_u64()? as u32, true)),
        "at_least" => Some((o["degree"].as_u64()? as u32, false)),
        _ => None,
    }
}

fn show(r: Option<(u32, bool)>) -> String {
    r.map_or("caps".into(), |(d, e)| {
        if e {
            d.to_string()
        } else {
            format!("≥{d}")
        }
    })
}

struct Svg(String);

impl Svg {
    fn new(w: f64, h: f64) -> Self {
        let mut s = String::new();
        write!(s, r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}" font-family="Helvetica,Arial,sans-serif" font-size="12"><rect width="{w}" height="{h}" fill="#ffffff"/>"##).unwrap();
        Svg(s)
    }
    fn raw(&mut self, t: &str) {
        self.0.push_str(t);
    }
    fn text(&mut self, x: f64, y: f64, anchor: &str, size: f64, fill: &str, t: &str) {
        let t = t.replace('&', "&amp;").replace('<', "&lt;");
        write!(self.0, r#"<text x="{x:.1}" y="{y:.1}" text-anchor="{anchor}" font-size="{size}" fill="{fill}">{t}</text>"#).unwrap();
    }
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, c: &str, w: f64, dash: &str) {
        write!(self.0, r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="{c}" stroke-width="{w}" stroke-dasharray="{dash}"/>"#).unwrap();
    }
    fn circle(&mut self, x: f64, y: f64, r: f64, fill: &str, stroke: &str) {
        write!(self.0, r#"<circle cx="{x:.1}" cy="{y:.1}" r="{r}" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>"#).unwrap();
    }
    fn finish(mut self) -> String {
        self.0.push_str("</svg>");
        self.0
    }
}

const INK: &str = "#222";
const GRID: &str = "#ddd";
const BLUE: &str = "#1f6fb2";
const ORANGE: &str = "#d9730d";
const GREEN: &str = "#2a8a4a";
const RED: &str = "#b3261e";

fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("study directory (research/certicom131_ic_program_20261020)");
    let root = Path::new(&root);
    let runs = root.join("runs");
    fs::create_dir_all(root.join("tables")).unwrap();
    fs::create_dir_all(root.join("figures")).unwrap();

    // ---- Falsifier A ------------------------------------------------------
    let mut a = lines(&runs, "c");
    // A draw measured by both a sequential and an `--unsat-index` process is
    // one draw: keep one record, preferring the one with the paired b = 1 reading.
    a.sort_by_key(|v| v["koblitz_b1"].is_null());
    let mut seen = std::collections::HashSet::new();
    a.retain(|v| seen.insert((v["n"].as_u64(), v["ell"].as_u64(), v["draw"].as_u64())));
    a.sort_by_key(|v| (v["n"].as_u64(), v["ell"].as_u64(), v["draw"].as_u64()));
    // (n, ell) -> draws
    let mut cells: BTreeMap<(u64, u64), Vec<&Value>> = BTreeMap::new();
    for v in &a {
        cells
            .entry((v["n"].as_u64().unwrap(), v["ell"].as_u64().unwrap()))
            .or_default()
            .push(v);
    }
    let mut md = String::new();
    writeln!(md, "| n | ℓ | S = n−3ℓ | unknowns | unsat draws measured | random-b D | b=1 D (same V, x_R) | FFD | semi-regular | median secs |").unwrap();
    writeln!(md, "|--:|--:|--:|--:|--:|:--|:--|:--|--:|--:|").unwrap();
    let mut pts: Vec<(u64, u64, Option<(u32, bool)>, Vec<Option<(u32, bool)>>)> = Vec::new();
    for ((n, ell), ds) in &cells {
        let r: Vec<Option<(u32, bool)>> = ds.iter().map(|d| reading(&d["outcome"])).collect();
        let k: Vec<String> = ds
            .iter()
            .map(|d| {
                let kb = &d["koblitz_b1"];
                if kb.is_null() {
                    "-".into()
                } else {
                    match kb["outcome"]["kind"].as_str().unwrap() {
                        "satisfiable" => "sat".into(),
                        _ => show(reading(&kb["outcome"])),
                    }
                }
            })
            .collect();
        let f: Vec<String> = ds
            .iter()
            .map(|d| d["ffd"].as_u64().map_or(">".into(), |x| x.to_string()))
            .collect();
        let mut secs: Vec<f64> = ds.iter().map(|d| d["secs"].as_f64().unwrap()).collect();
        secs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let semi = ds[0]["semireg"]
            .as_u64()
            .map_or("-".into(), |x| x.to_string());
        // Counting: group identical readings.
        let compress = |v: &[String]| {
            let mut m: BTreeMap<&str, usize> = BTreeMap::new();
            for s in v {
                *m.entry(s.as_str()).or_default() += 1;
            }
            m.iter()
                .map(|(k, c)| format!("{k}×{c}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let rs: Vec<String> = r.iter().map(|x| show(*x)).collect();
        writeln!(
            md,
            "| {n} | {ell} | {} | {} | {} | {} | {} | {} | {semi} | {:.1} |",
            *n as i64 - 3 * *ell as i64,
            ds[0]["n_vars"],
            ds.len(),
            compress(&rs),
            compress(&k),
            compress(&f),
            secs[secs.len() / 2]
        )
        .unwrap();
        // Single representative reading for the figure per draw.
        pts.push((*n, *ell, None, r));
    }
    // Paired comparison: random-b vs b=1 where both resolve.
    let (mut same, mut diff, mut skip) = (0, 0, 0);
    for v in &a {
        let kb = &v["koblitz_b1"];
        if kb.is_null() {
            continue;
        }
        match (reading(&v["outcome"]), reading(&kb["outcome"])) {
            (Some(x), Some(y)) if x == y => same += 1,
            (Some(_), Some(_)) => diff += 1,
            _ => skip += 1,
        }
    }
    writeln!(
        md,
        "\nPaired draws (random-b vs b = 1, same V and x_R): identical reading {same}, differing {diff}, not comparable {skip} (one side satisfiable or capped)."
    )
    .unwrap();
    fs::write(root.join("tables/dreg_summary.md"), &md).unwrap();

    // Figure: D against ℓ.
    let (w, h) = (720.0, 420.0);
    let (ml, mr, mt, mb) = (70.0, 30.0, 50.0, 60.0);
    let mut s = Svg::new(w, h);
    let max_ell = pts.iter().map(|p| p.1).max().unwrap_or(6) as f64 + 1.0;
    let max_d = 10.0;
    let xs = |l: f64| ml + (l - 0.5) / (max_ell - 0.5) * (w - ml - mr);
    let ys = |d: f64| h - mb - d / max_d * (h - mt - mb);
    for d in 0..=10 {
        s.line(ml, ys(d as f64), w - mr, ys(d as f64), GRID, 1.0, "");
        s.text(
            ml - 8.0,
            ys(d as f64) + 4.0,
            "end",
            11.0,
            INK,
            &d.to_string(),
        );
    }
    for l in 1..=(max_ell as u64) {
        s.text(
            xs(l as f64),
            h - mb + 18.0,
            "middle",
            11.0,
            INK,
            &l.to_string(),
        );
    }
    s.text(
        w / 2.0,
        h - 14.0,
        "middle",
        12.0,
        INK,
        "ℓ = dim V (the n of each cell is printed above its points)",
    );
    s.raw(&format!(r#"<text transform="translate(18 {}) rotate(-90)" text-anchor="middle" font-size="12" fill="{INK}">resolving degree D</text>"#, h / 2.0));
    s.text(
        w / 2.0,
        22.0,
        "middle",
        14.0,
        INK,
        "Falsifier A: FPPR m = 3 resolving degree on random binary curves, by ℓ",
    );
    s.text(w / 2.0, 38.0, "middle", 11.0, "#555", "filled = measured exactly; open = lower bound ≥ D; one dot per unsatisfiable draw (jittered); orange line = gate D* ≤ 3 (m = 4–6, chained)");
    s.line(ml, ys(3.0), w - mr, ys(3.0), ORANGE, 1.5, "6 4");
    let mut label_slot: BTreeMap<u64, usize> = BTreeMap::new();
    for (n, ell, _, r) in &pts {
        let cnt = r.len() as f64;
        for (i, rd) in r.iter().enumerate() {
            if let Some((d, exact)) = rd {
                let x = xs(*ell as f64) + (i as f64 - (cnt - 1.0) / 2.0) * (60.0 / cnt.max(6.0));
                let col = if *exact { BLUE } else { RED };
                s.circle(
                    x,
                    ys(*d as f64),
                    4.0,
                    if *exact { col } else { "#fff" },
                    col,
                );
            }
        }
        let slot = label_slot.entry(*ell).or_insert(0usize);
        s.text(
            xs(*ell as f64),
            ys(0.0) - 6.0 - 12.0 * *slot as f64,
            "middle",
            10.0,
            "#555",
            &format!("n={n}"),
        );
        *slot += 1;
    }
    fs::write(root.join("figures/fig_dreg_vs_ell.svg"), s.finish()).unwrap();

    // ---- Falsifier B ------------------------------------------------------
    let b = lines(&runs, "fb_");
    // key: (bits, base-kind, on_curve, B) for m=3 -> (sum count, targets)
    #[derive(Default)]
    struct Acc {
        n: f64,
        t: f64,
        bsz: f64,
        r: f64,
    }
    let mut acc: BTreeMap<(String, bool, u64, u64), Acc> = BTreeMap::new();
    let curves: Vec<Value> = fs::read_to_string("research/pkm_tower_ecbench_20261013/curves.jsonl")
        .unwrap()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let order_of = |p: u64| {
        curves
            .iter()
            .find(|c| c["p"].as_u64() == Some(p))
            .map(|c| c["group_order"].as_f64().unwrap())
            .unwrap()
    };
    for v in &b {
        if v["m"].as_u64() != Some(3) {
            continue;
        }
        let name = v["base"].as_str().unwrap();
        let on = name.ends_with("|on_curve");
        let base = name.trim_end_matches("|on_curve");
        let kind = if base.starts_with("kummer_mu") {
            "Kummer mu_(2^t)"
        } else if base.starts_with("kummer_coset") {
            "coset g*mu_(2^t)"
        } else if base.starts_with("random") {
            "random subset"
        } else if base.starts_with("interval") {
            "interval [1,B]"
        } else {
            "subgroup (order not a power of 2)"
        };
        let p = v["p"].as_u64().unwrap();
        let a = acc
            .entry((kind.to_string(), on, p, v["B"].as_u64().unwrap()))
            .or_default();
        a.n += v["count"].as_f64().unwrap();
        a.t += 1.0;
        a.bsz = v["B"].as_f64().unwrap();
        a.r = order_of(p);
    }
    // Pool per (kind, on_curve), over cells with expected count ≥ 20.
    let mut pooled: BTreeMap<(String, bool), (f64, f64, usize)> = BTreeMap::new();
    let mut mdb = String::new();
    writeln!(
        mdb,
        "| p (bits) | base | on-curve only | B | targets | Σ N₃ | Σ E₃ (random oracle) | N/E |"
    )
    .unwrap();
    writeln!(mdb, "|--:|:--|:--|--:|--:|--:|--:|--:|").unwrap();
    for ((kind, on, p, bsz), a) in &acc {
        // N counts (x1<x2, x3) with multiplicity 3; E per target as in the driver.
        let e = 8.0 * a.bsz.powi(3) / (6.0 * a.r) * 3.0 * a.t;
        if *on {
            writeln!(
                mdb,
                "| {} | {kind} | yes | {bsz} | {} | {:.0} | {:.1} | {:.3} |",
                64 - p.leading_zeros(),
                a.t,
                a.n,
                e,
                a.n / e
            )
            .unwrap();
            if e >= 20.0 * a.t {
                let ent = pooled.entry((kind.clone(), *on)).or_default();
                ent.0 += a.n;
                ent.1 += e;
                ent.2 += 1;
            }
        }
    }
    fs::write(root.join("tables/fb_cells.md"), &mdb).unwrap();
    let mut mdp = String::new();
    writeln!(mdp, "| base (on-curve abscissae) | cells pooled | Σ N₃ | Σ E₃ | N/E | 95% interval (Poisson on triples, normal approx.) |").unwrap();
    writeln!(mdp, "|:--|--:|--:|--:|--:|:--|").unwrap();
    let mut fig_rows = Vec::new();
    for ((kind, _), (n, e, c)) in &pooled {
        let r = n / e;
        let hw = 1.96 * (3.0 * n).sqrt() / e; // each unordered triple is counted 3 times
        writeln!(
            mdp,
            "| {kind} | {c} | {n:.0} | {e:.1} | {r:.3} | [{:.3}, {:.3}] |",
            r - hw,
            r + hw
        )
        .unwrap();
        fig_rows.push((kind.clone(), r, hw));
    }
    fs::write(root.join("tables/fb_pooled.md"), &mdp).unwrap();
    let (w, h) = (720.0, 80.0 + 46.0 * fig_rows.len().max(1) as f64);
    let mut s = Svg::new(w, h);
    let (ml, mr) = (190.0, 40.0);
    let xs = |v: f64| ml + (v - 0.5) / 1.0 * (w - ml - mr);
    s.text(
        w / 2.0,
        22.0,
        "middle",
        14.0,
        INK,
        "Falsifier B: decompositions found over a structured base ÷ random-oracle expectation",
    );
    s.text(w / 2.0, 38.0, "middle", 11.0, "#555", "m = 3 exact counts on the on-curve abscissae of V, pooled over cells with E ≥ 20 per target; bars: 95% Poisson interval");
    for t in [0.5, 0.75, 1.0, 1.25, 1.5] {
        s.line(
            xs(t),
            52.0,
            xs(t),
            h - 34.0,
            if t == 1.0 { INK } else { GRID },
            if t == 1.0 { 1.5 } else { 1.0 },
            "",
        );
        s.text(xs(t), h - 18.0, "middle", 11.0, INK, &format!("{t}"));
    }
    for (i, (kind, r, hw)) in fig_rows.iter().enumerate() {
        let y = 70.0 + 46.0 * i as f64;
        s.text(ml - 10.0, y + 4.0, "end", 12.0, INK, kind);
        s.line(xs(r - hw), y, xs(r + hw), y, GREEN, 2.0, "");
        s.circle(xs(*r), y, 5.0, GREEN, GREEN);
    }
    fs::write(root.join("figures/fig_combined_count.svg"), s.finish()).unwrap();

    // ---- Gates figure -------------------------------------------------------
    let g: Value =
        serde_json::from_str(&fs::read_to_string(root.join("gates.json")).unwrap()).unwrap();
    let mut rows: Vec<(String, f64, f64)> = Vec::new(); // label, plain gate, generous gate
    for chained in [false, true] {
        for m in 4..=6u64 {
            let pick = |price: &str| {
                g["ecc2_131"]["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["m"] == m && r["price"] == price && r["chained"] == chained)
                    .and_then(|r| r["gate_d"].as_u64())
                    .map_or(0.0, |x| x as f64)
            };
            rows.push((
                format!("binary m={m}{}", if chained { " chained" } else { "" }),
                pick("plain"),
                pick("generous"),
            ));
        }
    }
    for m in 4..=6u64 {
        let pick = |price: &str| {
            g["eccp_131"]["pkm"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["m"] == m && r["price"] == price)
                .and_then(|r| r["gate_d"].as_u64())
                .map_or(0.0, |x| x as f64)
        };
        rows.push((format!("prime PKM m={m}"), pick("plain"), pick("generous")));
    }
    let (w, h) = (720.0, 90.0 + 30.0 * rows.len() as f64);
    let mut s = Svg::new(w, h);
    let (ml, mr) = (170.0, 30.0);
    let xs = |d: f64| ml + d / 9.0 * (w - ml - mr);
    s.text(
        w / 2.0,
        22.0,
        "middle",
        14.0,
        INK,
        "Gates: largest solving degree D at which the family's optimised cost reaches rho",
    );
    s.text(w / 2.0, 38.0, "middle", 11.0, "#555", "filled = plain price, open = generous price (see doc §2). m = 2, 3 have no gate. Chained = unknowns (m−2)n+mℓ.");
    for d in 0..=9 {
        s.line(xs(d as f64), 52.0, xs(d as f64), h - 30.0, GRID, 1.0, "");
        s.text(xs(d as f64), h - 14.0, "middle", 11.0, INK, &d.to_string());
    }
    for (i, (label, plain, gen)) in rows.iter().enumerate() {
        let y = 70.0 + 30.0 * i as f64;
        s.text(ml - 10.0, y + 4.0, "end", 12.0, INK, label);
        s.line(xs(*plain), y, xs(*gen), y, BLUE, 3.0, "");
        s.circle(xs(*plain), y, 4.0, BLUE, BLUE);
        s.circle(xs(*gen), y, 4.0, "#fff", BLUE);
    }
    s.line(xs(5.0), 52.0, xs(5.0), h - 30.0, RED, 2.0, "");
    s.text(
        xs(5.0) + 4.0,
        62.0,
        "start",
        10.0,
        RED,
        "smallest measured D: 5 (ℓ=2), 6 (ℓ≥3)",
    );
    fs::write(root.join("figures/fig_gates.svg"), s.finish()).unwrap();
    println!("{md}\n{mdp}");
}

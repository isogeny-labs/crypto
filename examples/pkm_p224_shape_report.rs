//! Tables, figures and the derived F4 price for
//! `research/pkm_p224_kummer_shape_20261022/` (see its `PROTOCOL.md`).
//!
//! Nothing here measures an ECDLP method. It reads the rows the F4 runs wrote
//! (`runs/*.jsonl`, the `pkm_tower_pilot` row format) and the sealed ecbench
//! sessions, tabulates them, applies the protocol's frozen decision rules and
//! pricing formula (PROTOCOL §3 and §4.4), and lays out SVG figures. The one
//! measurement it makes is the conversion factor `ns / group addition` of
//! PROTOCOL §4.4, a microbenchmark of the harness's own counted `PrimeCurve`
//! addition.
//!
//! ```text
//! pkm_p224_shape_report f4 RUNS_DIR OUT_DIR
//! pkm_p224_shape_report price RUNS_DIR OUT_DIR CURVES.jsonl SESSION_DIR...
//! ```
//!
//! `price` needs, for each `SESSION_DIR`, the sealed ecbench session
//! (`records.jsonl`).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use serde_json::Value;

/// (id, p as decimal) for the protocol's primes (PROTOCOL §2.1).
const PRIMES: [(&str, &str); 12] = [
    ("SL20", "1032193"),
    ("SL24", "16760833"),
    ("SL26", "67104769"),
    ("SL28", "268369921"),
    ("SL31", "2147352577"),
    ("SL32", "4293918721"),
    ("SL48", "281474975662081"),
    ("SL61", "2305843009146585089"),
    (
        "P224",
        "26959946667150639794667015087019630673557916260026308143510066298881",
    ),
    ("p0", "786433"),
    ("p1", "2013265921"),
    ("p2", "3221225473"),
];

/// Recorded Kummer `D` (PROTOCOL §2.4): (m, N) -> D.
fn recorded(m: u64, n: u64) -> Option<u64> {
    match m {
        2 => match n {
            2 => Some(3),
            4..=8 if n % 2 == 0 => Some(4),
            10..=18 if n % 2 == 0 => Some(5),
            20 | 22 => Some(6),
            _ => None,
        },
        3 => match n {
            6 => Some(5),
            9 | 12 => Some(6),
            15 | 18 => Some(7),
            _ => None,
        },
        4 => match n {
            8 => Some(6),
            12 | 16 => Some(7),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Clone, Debug)]
struct Row {
    run: String,
    prime: String,
    kind: String,
    control: String,
    m: u64,
    n: u64,
    target: u64,
    d: u64,
    finished: bool,
    cols: u64,
    steps: u64,
    muladds: u64,
    ms: f64,
    pairs: u64,
    reducers: u64,
    max_rows: u64,
    max_nnz: u64,
    verify_ok: Option<bool>,
    backend: String,
}

fn get_u(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(Value::as_u64).unwrap_or(0)
}

fn prime_id(p: &str) -> String {
    PRIMES
        .iter()
        .find(|(_, q)| *q == p)
        .map(|(i, _)| (*i).to_string())
        .unwrap_or_else(|| format!("p={p}"))
}

fn read_rows(dir: &Path) -> Vec<Row> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .expect("runs dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jsonl"))
        .collect();
    files.sort();
    let mut rows = Vec::new();
    for f in files {
        let run = f.file_stem().unwrap().to_string_lossy().to_string();
        for line in fs::read_to_string(&f).unwrap().lines() {
            let Ok(v) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            if v.get("N").is_none() {
                continue;
            }
            let p = match v.get("p") {
                Some(Value::String(s)) => s.clone(),
                Some(x) => x.to_string(),
                None => String::new(),
            };
            let timed = v.get("timed_out").and_then(Value::as_bool).unwrap_or(false);
            let stopped = !v.get("stopped_step").unwrap_or(&Value::Null).is_null();
            let above = get_u(&v, "pairs_above_bound");
            rows.push(Row {
                run: run.clone(),
                prime: prime_id(&p),
                kind: v["kind"].as_str().unwrap_or("").to_string(),
                control: v["control"].as_str().unwrap_or("").to_string(),
                m: get_u(&v, "m"),
                n: get_u(&v, "N"),
                target: get_u(&v, "target_index"),
                d: get_u(&v, "solving_degree_max"),
                finished: !timed && !stopped && above == 0,
                cols: get_u(&v, "max_cols"),
                steps: get_u(&v, "steps"),
                muladds: get_u(&v, "muladds"),
                ms: v.get("ms").and_then(Value::as_f64).unwrap_or(0.0),
                pairs: get_u(&v, "critical_pairs_reduced") + get_u(&v, "tower_pairs_reduced"),
                reducers: get_u(&v, "reducer_rows"),
                max_rows: get_u(&v, "max_rows"),
                max_nnz: get_u(&v, "max_nnz"),
                verify_ok: v
                    .get("verify")
                    .and_then(|x| x.get("ok"))
                    .and_then(Value::as_bool),
                backend: v
                    .get("backend")
                    .and_then(Value::as_str)
                    .unwrap_or("fp32")
                    .to_string(),
            });
        }
    }
    rows
}

type CellKey = (String, String, u64, String, u64); // prime, kind, m, control, N

fn cells(rows: &[Row]) -> BTreeMap<CellKey, Vec<&Row>> {
    let mut out: BTreeMap<CellKey, Vec<&Row>> = BTreeMap::new();
    for r in rows {
        out.entry((r.prime.clone(), r.kind.clone(), r.m, r.control.clone(), r.n))
            .or_default()
            .push(r);
    }
    out
}

/// `Some(D)` when every target finished with the same `D`; `Err` text else.
fn cell_value(rs: &[&Row]) -> Result<u64, String> {
    if rs.iter().any(|r| !r.finished) {
        let lb = rs.iter().map(|r| r.d).max().unwrap_or(0);
        return Err(format!("stopped (D>={lb})"));
    }
    let d0 = rs[0].d;
    if rs.iter().all(|r| r.d == d0) {
        Ok(d0)
    } else {
        Err(format!(
            "mixed {:?}",
            rs.iter().map(|r| r.d).collect::<Vec<_>>()
        ))
    }
}

// ---------------------------------------------------------------- SVG

struct Series {
    name: String,
    pts: Vec<(f64, f64)>,
    color: &'static str,
    dashed: bool,
}

const PALETTE: [&str; 8] = [
    "#1f77b4", "#d62728", "#2ca02c", "#9467bd", "#8c564b", "#e377c2", "#17becf", "#7f7f7f",
];

fn svg_plot(
    path: &Path,
    title: &str,
    caption: &str,
    xl: &str,
    yl: &str,
    logy: bool,
    series: &[Series],
) {
    let (w, h) = (860.0, 520.0);
    let (ml, mr, mt, mb) = (70.0, 200.0, 40.0, 80.0);
    let tx = |x: f64, lo: f64, hi: f64| ml + (x - lo) / (hi - lo).max(1e-9) * (w - ml - mr);
    let ty = |y: f64, lo: f64, hi: f64| {
        let (y, lo, hi) = if logy {
            (y.log10(), lo.log10(), hi.log10())
        } else {
            (y, lo, hi)
        };
        h - mb - (y - lo) / (hi - lo).max(1e-9) * (h - mt - mb)
    };
    let all: Vec<(f64, f64)> = series.iter().flat_map(|s| s.pts.iter().copied()).collect();
    let xlo = all.iter().map(|p| p.0).fold(f64::MAX, f64::min);
    let xhi = all.iter().map(|p| p.0).fold(f64::MIN, f64::max);
    let mut ylo = all.iter().map(|p| p.1).fold(f64::MAX, f64::min);
    let mut yhi = all.iter().map(|p| p.1).fold(f64::MIN, f64::max);
    if logy {
        ylo = 10f64.powf(ylo.log10().floor());
        yhi = 10f64.powf(yhi.log10().ceil());
    } else {
        ylo = (ylo - 0.5).floor().min(ylo);
        yhi = (yhi + 0.5).ceil();
    }
    let mut s = String::new();
    let _ = write!(
        s,
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' width='{w}' height='{h}' font-family='sans-serif' font-size='12'>\
<rect width='{w}' height='{h}' fill='white'/>\
<text x='{ml}' y='22' font-size='15' font-weight='bold'>{}</text>",
        esc(title)
    );
    // axes and ticks
    let _ = write!(
        s,
        "<rect x='{ml}' y='{mt}' width='{}' height='{}' fill='none' stroke='#444'/>",
        w - ml - mr,
        h - mt - mb
    );
    let nx = 8;
    for i in 0..=nx {
        let x = xlo + (xhi - xlo) * i as f64 / nx as f64;
        let px = tx(x, xlo, xhi);
        let _ = write!(
            s,
            "<line x1='{px}' y1='{}' x2='{px}' y2='{}' stroke='#ccc'/><text x='{px}' y='{}' text-anchor='middle'>{:.0}</text>",
            mt,
            h - mb,
            h - mb + 16.0,
            x
        );
    }
    if logy {
        let mut v = ylo;
        while v <= yhi * 1.01 {
            let py = ty(v, ylo, yhi);
            let _ = write!(
                s,
                "<line x1='{ml}' y1='{py}' x2='{}' y2='{py}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>1e{}</text>",
                w - mr,
                ml - 6.0,
                py + 4.0,
                v.log10().round()
            );
            v *= 10.0;
        }
    } else {
        let mut v = ylo;
        while v <= yhi + 1e-9 {
            let py = ty(v, ylo, yhi);
            let _ = write!(
                s,
                "<line x1='{ml}' y1='{py}' x2='{}' y2='{py}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{v:.0}</text>",
                w - mr,
                ml - 6.0,
                py + 4.0
            );
            v += 1.0;
        }
    }
    let _ = write!(
        s,
        "<text x='{}' y='{}' text-anchor='middle'>{}</text><text transform='translate(18,{}) rotate(-90)' text-anchor='middle'>{}</text>",
        ml + (w - ml - mr) / 2.0,
        h - mb + 36.0,
        esc(xl),
        mt + (h - mt - mb) / 2.0,
        esc(yl)
    );
    for (i, se) in series.iter().enumerate() {
        let col = se.color;
        let pts: Vec<String> = se
            .pts
            .iter()
            .map(|&(x, y)| format!("{:.1},{:.1}", tx(x, xlo, xhi), ty(y, ylo, yhi)))
            .collect();
        let dash = if se.dashed {
            " stroke-dasharray='6 4'"
        } else {
            ""
        };
        let _ = write!(
            s,
            "<polyline points='{}' fill='none' stroke='{col}' stroke-width='1.8'{dash}/>",
            pts.join(" ")
        );
        for &(x, y) in &se.pts {
            let _ = write!(
                s,
                "<circle cx='{:.1}' cy='{:.1}' r='3' fill='{col}'/>",
                tx(x, xlo, xhi),
                ty(y, ylo, yhi)
            );
        }
        let ly = mt + 14.0 + 16.0 * i as f64;
        let _ = write!(
            s,
            "<line x1='{}' y1='{ly}' x2='{}' y2='{ly}' stroke='{col}' stroke-width='2'{dash}/><text x='{}' y='{}'>{}</text>",
            w - mr + 10.0,
            w - mr + 34.0,
            w - mr + 40.0,
            ly + 4.0,
            esc(&se.name)
        );
    }
    let _ = write!(
        s,
        "<text x='{ml}' y='{}' font-size='11' fill='#333'>{}</text></svg>",
        h - 10.0,
        esc(caption)
    );
    fs::write(path, s).unwrap();
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ---------------------------------------------------------------- f4

fn cmd_f4(runs: &Path, out: &Path) {
    let rows = read_rows(runs);
    fs::create_dir_all(out.join("tables")).unwrap();
    fs::create_dir_all(out.join("figures")).unwrap();
    let cs = cells(&rows);
    let mut tsv = String::from(
        "prime\tkind\tm\tcontrol\tN\ttargets\tD\tcell_value\trecorded_D\tmatch\twidth\tsteps\tmuladds\tms\tbackend\n",
    );
    let mut same = Vec::new();
    let mut diff = Vec::new();
    let mut other = Vec::new();
    let mut md = String::from(
        "| prime | kind | m | control | N | targets | D per target | cell | recorded D | width | steps | multiply-adds | ms (mean) |\n|:--|:--|--:|:--|--:|--:|:--|:--|--:|--:|--:|--:|--:|\n",
    );
    for ((prime, kind, m, control, n), rs) in &cs {
        let val = cell_value(rs);
        let rec = if kind == "kummer" {
            recorded(*m, *n)
        } else {
            None
        };
        let ds: Vec<String> = rs.iter().map(|r| r.d.to_string()).collect();
        let cv = match &val {
            Ok(d) => d.to_string(),
            Err(e) => e.clone(),
        };
        let mt = match (&val, rec) {
            (Ok(d), Some(r)) => {
                if control == "tower" {
                    let tag = format!("{prime} m={m} N={n}: {d} vs recorded {r}");
                    if *d == r {
                        same.push(tag);
                    } else {
                        diff.push(tag);
                    }
                }
                if *d == r {
                    "same"
                } else {
                    "DIFF"
                }
            }
            (Err(e), _) => {
                other.push(format!("{prime} {control} m={m} N={n}: {e}"));
                "-"
            }
            _ => "-",
        };
        let mean = |f: &dyn Fn(&Row) -> f64| rs.iter().map(|r| f(r)).sum::<f64>() / rs.len() as f64;
        let _ = writeln!(
            tsv,
            "{prime}\t{kind}\t{m}\t{control}\t{n}\t{}\t{}\t{cv}\t{}\t{mt}\t{}\t{}\t{:.0}\t{:.1}\t{}",
            rs.len(),
            ds.join(","),
            rec.map(|r| r.to_string()).unwrap_or_default(),
            rs[0].cols,
            rs[0].steps,
            mean(&|r| r.muladds as f64),
            mean(&|r| r.ms),
            rs[0].backend
        );
        let _ = writeln!(
            md,
            "| {prime} | {kind} | {m} | {control} | {n} | {} | {} | {cv} | {} | {} | {} | {:.3e} | {:.1} |",
            rs.len(),
            ds.join(", "),
            rec.map(|r| r.to_string()).unwrap_or_else(|| "-".into()),
            rs[0].cols,
            rs[0].steps,
            mean(&|r| r.muladds as f64),
            mean(&|r| r.ms)
        );
    }
    fs::write(out.join("tables/f4_cells.tsv"), tsv).unwrap();
    fs::write(out.join("tables/f4_cells.md"), md).unwrap();

    // tower vs null and verification
    let mut tn = String::new();
    let mut tn_diff = 0;
    let mut tn_n = 0;
    for ((prime, kind, m, control, n), rs) in &cs {
        if control != "tower" {
            continue;
        }
        let nk = (prime.clone(), kind.clone(), *m, "null".to_string(), *n);
        if let (Ok(a), Some(Ok(b))) = (cell_value(rs), cs.get(&nk).map(|x| cell_value(x))) {
            tn_n += 1;
            if a != b {
                tn_diff += 1;
                let _ = writeln!(tn, "{prime} m={m} N={n}: tower {a} null {b}");
            }
        }
    }
    let nver_ok = rows.iter().filter(|r| r.verify_ok == Some(true)).count();
    let nver_bad = rows.iter().filter(|r| r.verify_ok == Some(false)).count();
    // Q4: same width/steps across primes at one (m, N) for kummer tower
    let mut q4 = String::new();
    let mut by_mn: BTreeMap<(u64, u64), Vec<(String, u64, u64, u64)>> = BTreeMap::new();
    for ((prime, kind, m, control, n), rs) in &cs {
        if kind == "kummer" && control == "tower" && rs.iter().all(|r| r.finished) {
            by_mn.entry((*m, *n)).or_default().push((
                prime.clone(),
                rs[0].cols,
                rs[0].steps,
                rs[0].d,
            ));
        }
    }
    let mut q4_bad = 0;
    for ((m, n), v) in &by_mn {
        let w0 = v[0].1;
        let s0 = v[0].2;
        let ok = v.iter().all(|x| x.1 == w0 && x.2 == s0);
        if !ok {
            q4_bad += 1;
        }
        let _ = writeln!(
            q4,
            "m={m} N={n}: {} -> {}",
            v.iter()
                .map(|x| format!("{}(w={},s={},D={})", x.0, x.1, x.2, x.3))
                .collect::<Vec<_>>()
                .join(" "),
            if ok {
                "width and steps equal"
            } else {
                "DIFFER"
            }
        );
    }
    let mut summary = String::new();
    let _ = writeln!(summary, "H_same cells equal to recorded: {}", same.len());
    let _ = writeln!(
        summary,
        "H_diff cells (tower D != recorded): {}",
        diff.len()
    );
    for d in &diff {
        let _ = writeln!(summary, "  DIFF {d}");
    }
    let _ = writeln!(summary, "Not valued (stopped/mixed): {}", other.len());
    for o in &other {
        let _ = writeln!(summary, "  {o}");
    }
    let _ = writeln!(
        summary,
        "tower vs null: {tn_n} paired cells, {tn_diff} differ\n{tn}"
    );
    let _ = writeln!(
        summary,
        "wide-builder verify: ok {nver_ok}, FAIL {nver_bad}"
    );
    let _ = writeln!(
        summary,
        "Q4 (width/steps across primes, same m,N): {q4_bad} (m,N) differ\n{q4}"
    );
    fs::write(out.join("tables/f4_summary.txt"), &summary).unwrap();
    print!("{summary}");

    // figure: D vs N, m = 2, tower per prime and null, recorded
    let mut series = Vec::new();
    let mut ci = 0;
    let mut by_prime: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    let mut nullp: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for ((prime, kind, m, control, n), rs) in &cs {
        if kind != "kummer" || *m != 2 {
            continue;
        }
        if let Ok(d) = cell_value(rs) {
            let t = if control == "tower" {
                &mut by_prime
            } else {
                &mut nullp
            };
            t.entry(prime.clone())
                .or_default()
                .push((*n as f64, d as f64));
        }
    }
    let rec: Vec<(f64, f64)> = (1..=12)
        .filter_map(|t| recorded(2, 2 * t).map(|d| (2.0 * t as f64, d as f64)))
        .collect();
    series.push(Series {
        name: "recorded (p0,p1,p2)".into(),
        pts: rec,
        color: "#000000",
        dashed: true,
    });
    for (prime, pts) in by_prime {
        series.push(Series {
            name: format!("{prime} tower"),
            pts,
            color: PALETTE[ci % 8],
            dashed: false,
        });
        ci += 1;
    }
    for (prime, pts) in nullp {
        if prime == "SL31" || prime == "P224" {
            series.push(Series {
                name: format!("{prime} null"),
                pts,
                color: PALETTE[ci % 8],
                dashed: true,
            });
            ci += 1;
        }
    }
    svg_plot(
        &out.join("figures/d_vs_n.svg"),
        "F4 solving degree D against N = 2t, m = 2, Kummer tower",
        "Measured rows of runs/*.jsonl (pkm_p224_kummer_shape_20261022); recorded = PKM note 11-12, 16. Stage diagnostic, no S.",
        "N = m*t (tower unknowns)",
        "solving degree D",
        false,
        &series,
    );
    // figure: multiply-adds vs N at m = 2
    let mut ms_series = Vec::new();
    let mut ci = 0;
    let mut mp: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for ((prime, kind, m, control, n), rs) in &cs {
        if kind == "kummer" && *m == 2 && control == "tower" && rs.iter().all(|r| r.finished) {
            let mean = rs.iter().map(|r| r.muladds as f64).sum::<f64>() / rs.len() as f64;
            if mean > 0.0 {
                mp.entry(prime.clone()).or_default().push((*n as f64, mean));
            }
        }
    }
    for (prime, pts) in mp {
        ms_series.push(Series {
            name: prime,
            pts,
            color: PALETTE[ci % 8],
            dashed: false,
        });
        ci += 1;
    }
    svg_plot(
        &out.join("figures/madds_vs_n.svg"),
        "F4 multiply-adds per system against N, m = 2",
        "Measured; mean over the cell's finished targets. Counts depend on the thread count (chunking).",
        "N = 2t",
        "field multiply-adds",
        true,
        &ms_series,
    );
}

// -------------------------------------------------------------- price

/// Median ns of one counted affine addition on `curve`, 5 x 10^6 additions.
fn ns_per_gae(curve: &PrimeCurve) -> f64 {
    let g = (1..)
        .find_map(|x| curve.sqrt(curve.rhs(x)).map(|y| PrimePoint::affine(x, y)))
        .unwrap();
    let h = curve.double(&mut GroupOps::default(), g);
    let mut samples = Vec::new();
    for _ in 0..5 {
        let mut ops = GroupOps::default();
        let mut q = g;
        let t = Instant::now();
        for _ in 0..1_000_000 {
            q = curve.add(&mut ops, q, h);
        }
        let ns = t.elapsed().as_nanos() as f64 / 1e6;
        std::hint::black_box(q);
        samples.push(ns);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples[2]
}

struct Rec {
    arm: String,
    slug: String,
    log2r: f64,
    bits: u64,
    s: f64,
    gae: f64,
    unpriced: Vec<String>,
    counters: BTreeMap<String, u64>,
    columns: Option<u64>,
    family: String,
}

fn read_records(session: &Path) -> Vec<Rec> {
    let text = fs::read_to_string(session.join("records.jsonl")).expect("records.jsonl");
    let mut out = Vec::new();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if v["warmup"].as_bool().unwrap_or(true) {
            continue;
        }
        if v["outcome"]["status"].as_str() != Some("verified") {
            continue;
        }
        let r = v["workload"]["curve"]["r"].as_f64().unwrap_or(0.0);
        let mut counters = BTreeMap::new();
        if let Some(c) = v["counters"].as_object() {
            for (k, x) in c {
                if let Some(u) = x.as_u64() {
                    counters.insert(k.clone(), u);
                }
            }
        }
        // unpriced counts that live in phase native maps
        if let Some(ph) = v["phases"].as_array() {
            for p in ph {
                if let Some(nat) = p["native"].as_object() {
                    for (k, x) in nat {
                        if let Some(u) = x.as_u64() {
                            *counters.entry(k.clone()).or_default() += u;
                        }
                    }
                }
            }
        }
        out.push(Rec {
            arm: v["arm"].as_str().unwrap_or("").to_string(),
            slug: v["workload"]["curve"]["slug"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            log2r: r.log2(),
            bits: v["workload"]["curve"]["field_bits"].as_u64().unwrap_or(0),
            s: v["cost"]["s"].as_f64().unwrap_or(f64::NAN),
            gae: v["cost"]["total_gae"].as_f64().unwrap_or(f64::NAN),
            unpriced: v["cost"]["unpriced"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            counters,
            columns: v["factor_base"]["columns"].as_u64(),
            family: v["workload"]["curve_spec"]["p"].to_string(),
        });
    }
    out
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

/// Least-squares slope and its 95% half width (1.96 s.e.).
fn slope(pts: &[(f64, f64)]) -> (f64, f64) {
    let n = pts.len() as f64;
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    let sxy: f64 = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = pts.iter().map(|p| (p.1 - a - b * p.0).powi(2)).sum();
    let se = (rss / (n - 2.0).max(1.0) / sxx).sqrt();
    (b, 1.96 * se)
}

fn cmd_price(runs: &Path, out: &Path, curves: &Path, sessions: &[PathBuf]) {
    fs::create_dir_all(out.join("tables")).unwrap();
    fs::create_dir_all(out.join("figures")).unwrap();
    let weights = [0.0, 0.1, 1.0];
    // --- ecbench arms: S, ratio to matched rho, band
    let mut md = String::from(
        "| bits | curve family | log2 r | arm | mean S | S / S_rho(neg) | unpriced counters (mean count) | S at w=0 / 0.1 / 1 |\n|--:|:--|--:|:--|--:|--:|:--|:--|\n",
    );
    // (family, bits) -> arm -> Vec<(s, gae, unpriced total, missing)>
    let mut table: BTreeMap<(String, u64), BTreeMap<String, Vec<(f64, f64, f64, bool)>>> =
        BTreeMap::new();
    let mut log2r: BTreeMap<(String, u64), f64> = BTreeMap::new();
    let mut columns: BTreeMap<(String, u64), BTreeMap<String, u64>> = BTreeMap::new();
    let mut ctr_names: BTreeMap<(String, u64, String), BTreeMap<String, Vec<u64>>> =
        BTreeMap::new();
    for sd in sessions {
        for r in read_records(sd) {
            let solinas = fam_is_solinas(&r.family);
            let fam = if solinas { "Solinas" } else { "Proth" }.to_string();
            let key = (fam, r.bits);
            log2r.insert(key.clone(), r.log2r);
            let (mut u, mut missing) = (0.0, false);
            for name in &r.unpriced {
                match r
                    .counters
                    .get(name)
                    .or_else(|| r.counters.get(name.trim_end_matches("_uncharged")))
                {
                    Some(c) => u += *c as f64,
                    None => missing = true,
                }
            }
            for name in &r.unpriced {
                let c = r
                    .counters
                    .get(name)
                    .or_else(|| r.counters.get(name.trim_end_matches("_uncharged")))
                    .copied()
                    .unwrap_or(0);
                ctr_names
                    .entry((key.0.clone(), key.1, r.arm.clone()))
                    .or_default()
                    .entry(name.clone())
                    .or_default()
                    .push(c);
            }
            if let Some(c) = r.columns {
                columns
                    .entry(key.clone())
                    .or_default()
                    .insert(r.arm.clone(), c);
            }
            table
                .entry(key)
                .or_default()
                .entry(r.arm.clone())
                .or_default()
                .push((r.s, r.gae, u, missing));
        }
    }
    let mut ratio_pts: BTreeMap<(String, String), Vec<(f64, f64)>> = BTreeMap::new();
    for ((fam, bits), arms) in &table {
        let rho = &arms["rho-neg"];
        let sr = mean(&rho.iter().map(|x| x.0).collect::<Vec<_>>());
        let lr = log2r[&(fam.clone(), *bits)];
        let sqrt_r = 2f64.powf(lr / 2.0);
        for (arm, v) in arms {
            let s = mean(&v.iter().map(|x| x.0).collect::<Vec<_>>());
            let gae = mean(&v.iter().map(|x| x.1).collect::<Vec<_>>());
            let u = mean(&v.iter().map(|x| x.2).collect::<Vec<_>>());
            let miss = v.iter().any(|x| x.3);
            let band: Vec<String> = weights
                .iter()
                .map(|w| {
                    if *w > 0.0 && miss {
                        "lower bound".into()
                    } else {
                        format!("{:.3}", (gae + w * u) / sqrt_r)
                    }
                })
                .collect();
            let cn = ctr_names
                .get(&(fam.clone(), *bits, arm.clone()))
                .map(|m| {
                    m.iter()
                        .map(|(k, c)| {
                            format!(
                                "{}={:.0}",
                                k,
                                mean(&c.iter().map(|x| *x as f64).collect::<Vec<_>>())
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            let _ = writeln!(
                md,
                "| {bits} | {fam} | {lr:.2} | {arm} | {s:.3} | {:.2} | {cn} | {} |",
                s / sr,
                band.join(" / ")
            );
            if arm.starts_with("ic-") {
                ratio_pts
                    .entry((fam.clone(), arm.clone()))
                    .or_default()
                    .push((lr, s / sr));
            }
        }
    }
    fs::write(out.join("tables/ecbench_arms.md"), &md).unwrap();
    println!("{md}");

    // growth fits: log2(S_ic/S_rho) against log2 r
    let mut fits = String::new();
    for ((fam, arm), pts) in &ratio_pts {
        let lp: Vec<(f64, f64)> = pts.iter().map(|p| (p.0, p.1.log2())).collect();
        if lp.len() >= 3 {
            let (b, h) = slope(&lp);
            let _ = writeln!(
                fits,
                "{fam} {arm}: {} sizes, d log2(S_ic/S_rho) / d log2 r = {b:.3} +/- {h:.3} (95%, least squares; prediction 0.17)",
                lp.len()
            );
        }
    }
    println!("{fits}");

    // --- F4 priced rows (PROTOCOL 4.4)
    let rows = read_rows(runs);
    let curves_txt = fs::read_to_string(curves).unwrap();
    let mut f4 = String::from(
        "| curve (bits, p) | F4 cell | R = columns+10 | madds / system | ns / madd | ns / GAE | M (madds per GAE) | c_F4 GAE (w=0) | S_F4 w=0 | w=0.1 | w=1 | S_F4 / S_rho(neg) w=0 | w=1 | generous M=4: S_F4 w=0 |\n|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n",
    );
    let wanted: [(&str, u64, u64); 2] = [("SL20", 20, 20), ("SL24", 24, 24)];
    let mut f4_pts: Vec<(f64, f64)> = Vec::new();
    for (pid, bits, nn) in wanted {
        let p = PRIMES.iter().find(|x| x.0 == pid).unwrap().1;
        let rs: Vec<&Row> = rows
            .iter()
            .filter(|r| {
                r.prime == pid
                    && r.m == 2
                    && r.n == nn
                    && r.control == "tower"
                    && r.kind == "kummer"
                    && r.finished
            })
            .collect();
        if rs.is_empty() {
            let _ = writeln!(f4, "| {bits}-bit p={p} | m=2 N={nn} | not measured (no finished row) | | | | | | | | | | | |");
            continue;
        }
        let curve_line = curves_txt
            .lines()
            .find(|l| l.contains(&format!("\"p\": {p},")))
            .unwrap_or_else(|| panic!("no curve for p={p} in curves.jsonl"));
        let cj: Value = serde_json::from_str(curve_line).unwrap();
        let curve = PrimeCurve {
            p: cj["p"].as_u64().unwrap(),
            a: cj["a"].as_u64().unwrap(),
            b: cj["b"].as_u64().unwrap(),
        };
        let r = cj["r"].as_f64().unwrap();
        let ns_gae = ns_per_gae(&curve);
        let madds = mean(&rs.iter().map(|x| x.muladds as f64).collect::<Vec<_>>());
        let ms = mean(&rs.iter().map(|x| x.ms).collect::<Vec<_>>());
        let ns_madd = ms * 1e6 / madds;
        let m_conv = ns_gae / ns_madd;
        let u = mean(
            &rs.iter()
                .map(|x| (x.pairs + x.reducers + x.max_rows) as f64)
                .collect::<Vec<_>>(),
        );
        // columns: the Solinas session's factor-base record for the F4 height t = N/2
        let t_f4 = nn / 2;
        let col = columns
            .get(&("Solinas".to_string(), bits))
            .and_then(|m| {
                m.get(&format!("ic-mitm-tf4-t{t_f4}"))
                    .or_else(|| m.get("ic-mitm-tf4"))
            })
            .copied();
        let col_txt = col
            .map(|c| c.to_string())
            .unwrap_or_else(|| "missing".into());
        let big_r = col.map(|c| (c + 10) as f64).unwrap_or(f64::NAN);
        let c0 = madds / m_conv;
        let s_of = |w: f64, m: f64| big_r * (madds / m + w * u) / r.sqrt();
        let rho_s = table
            .get(&("Solinas".to_string(), bits))
            .and_then(|a| a.get("rho-neg"))
            .map(|v| mean(&v.iter().map(|x| x.0).collect::<Vec<_>>()))
            .unwrap_or(f64::NAN);
        let _ = writeln!(
            f4,
            "| {bits}-bit p={p} (r={r:.0}) | m=2 N={nn} t={t_f4} ({} target(s)) | {col_txt} | {madds:.3e} | {ns_madd:.3} | {ns_gae:.1} | {m_conv:.1} | {c0:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} | {:.3e} |",
            rs.len(),
            s_of(0.0, m_conv),
            s_of(0.1, m_conv),
            s_of(1.0, m_conv),
            s_of(0.0, m_conv) / rho_s,
            s_of(1.0, m_conv) / rho_s,
            s_of(0.0, 4.0)
        );
        f4_pts.push((r.log2(), s_of(0.0, m_conv) / rho_s));
    }
    fs::write(out.join("tables/f4_priced.md"), &f4).unwrap();
    println!("{f4}");
    fs::write(out.join("tables/growth_fits.txt"), &fits).unwrap();

    // figure: S/S_rho by log2 r
    let mut series = Vec::new();
    let mut ci = 0;
    for ((fam, arm), pts) in &ratio_pts {
        series.push(Series {
            name: format!("{fam} {arm}"),
            pts: pts.clone(),
            color: PALETTE[ci % 8],
            dashed: fam == "Proth",
        });
        ci += 1;
    }
    if !f4_pts.is_empty() {
        series.push(Series {
            name: "F4 arm, lower bound (w=0)".into(),
            pts: f4_pts,
            color: "#000000",
            dashed: false,
        });
    }
    series.push(Series {
        name: "rho (matched) = 1".into(),
        pts: vec![(18.0, 1.0), (27.0, 1.0)],
        color: "#7f7f7f",
        dashed: true,
    });
    svg_plot(
        &out.join("figures/s_ratio_by_size.svg"),
        "Whole-method S against matched rho (negation), by curve size",
        "ecbench sessions (measured, mean S per arm over verified measured runs) and the derived F4 lower bound (PROTOCOL 4.4). Solid = Solinas curves, dashed = Proth controls.",
        "log2 r",
        "S_arm / S_rho (log scale)",
        true,
        &series,
    );
}

/// A prime_explicit curve spec's `p` is Solinas when `p - 1 = 2^k (2^(n-k) - 1)`.
fn fam_is_solinas(p_json: &str) -> bool {
    let Ok(p) = p_json.trim().parse::<u64>() else {
        return false;
    };
    let k = (p - 1).trailing_zeros();
    let odd = (p - 1) >> k;
    // odd part of the form 2^j - 1 means all ones in binary
    odd & (odd + 1) == 0 && odd > 1
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("f4") => cmd_f4(Path::new(&a[2]), Path::new(&a[3])),
        Some("price") => cmd_price(
            Path::new(&a[2]),
            Path::new(&a[3]),
            Path::new(&a[4]),
            &a[5..].iter().map(PathBuf::from).collect::<Vec<_>>(),
        ),
        _ => {
            eprintln!("usage: pkm_p224_shape_report f4 RUNS OUT | price RUNS OUT CURVES SESSION...")
        }
    }
}

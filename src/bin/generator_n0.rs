//! generator-n0: experiment E3a of `research/generator_effect_ic_20261019`.
//!
//! Does the base's cofactor-class count `N0` (`exact_decomposition_ceiling`)
//! explain the volcano study's IC relation-yield differences between
//! models, curves and levels? Two subcommands, both native:
//!
//!   generator-n0 census  [--root DIR] --out n0.json [--csv-out n0.csv]
//!       For every ordinary workload curve, build the `prime-abscissa:size=32`
//!       base exactly as `prime_factor_base` does, check its point hash
//!       against the measured records' `factor_base.points_sha256`, and
//!       compute `N0/r` for two and three summands. Reads no yield.
//!
//!   generator-n0 regress --n0 n0.json [--root DIR] --out regression.json
//!                        [--csv-out workloads.csv] [--boot 2000] [--perm 10000]
//!       G1 (slope of ln yield on ln N0/r, with fixed-effect variants and a
//!       cluster bootstrap) and G4 (the floor/crater contrast with and
//!       without the N0 adjustment). G2 and G3 are `volcano-stats --n0`.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Read;

use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ic_boundary::{
    exact_decomposition_ceiling, prime_factor_base, CountedGroup, PrimeCurve, PrimeInstance,
};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const DEFAULT_ROOT: &str = "research/isogeny-volcano-cost_20261017";
/// Class index order of `analysis-v1.1.json`; the session whose records the
/// analysis used for each (`l3-split/spec-r2` replaces the interrupted run).
const CLASSES: [(&str, &str); 6] = [
    ("l2-ramified", "spec"),
    ("l2-split", "spec"),
    ("l2-inert", "spec"),
    ("l3-ramified", "spec"),
    ("l3-split", "spec-r2"),
    ("l3-inert", "spec"),
];

fn die(msg: &str) -> ! {
    eprintln!("generator-n0: {msg}");
    std::process::exit(2);
}

fn read_gz(path: &str) -> String {
    let bytes = fs::read(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut out = String::new();
    flate2::read::GzDecoder::new(&bytes[..])
        .read_to_string(&mut out)
        .unwrap_or_else(|e| die(&format!("{path}: {e}")));
    out
}

/// A session's records, checked against the SHA-256 manifest of the
/// uncompressed file.
fn records(root: &str, class: &str, spec: &str, manifest: &HashMap<String, String>) -> String {
    let text = read_gz(&format!("{root}/sessions/{class}/{spec}/records.jsonl.gz"));
    let key = format!("sessions/{class}/{spec}/records.jsonl");
    let want = manifest
        .get(&key)
        .unwrap_or_else(|| die(&format!("{key} not in the manifest")));
    let got = sha256_hex(text.as_bytes());
    if &got != want {
        die(&format!("{key}: sha256 {got} differs from manifest {want}"));
    }
    text
}

fn manifest(root: &str) -> HashMap<String, String> {
    let text = fs::read_to_string(format!("{root}/sessions/MANIFEST.sha256"))
        .unwrap_or_else(|e| die(&format!("manifest: {e}")));
    text.lines()
        .filter_map(|l| l.split_once("  "))
        .map(|(h, p)| (p.trim().to_string(), h.to_string()))
        .collect()
}

type Key = (u64, u64, u64, u64, u64);
fn curve_key(cs: &Value) -> Key {
    let u = |k: &str| cs[k].as_u64().unwrap_or_else(|| die(&format!("curve_spec.{k}")));
    (u("p"), u("a"), u("b"), u("gx"), u("gy"))
}
fn key_str(k: &Key) -> String {
    format!("{};{};{};{};{}", k.0, k.1, k.2, k.3, k.4)
}

// ---------------------------------------------------------------- census

fn census(argv: &[String]) {
    let get = |k: &str| argv.iter().position(|a| a == k).and_then(|i| argv.get(i + 1));
    let root = get("--root").map_or(DEFAULT_ROOT, |s| s.as_str());
    let out = get("--out").unwrap_or_else(|| die("census needs --out"));
    let man = manifest(root);
    let mut rows = Vec::new();
    let mut csv = String::from("class,key,h,r,classes,w2,w3\n");
    let (mut checked, mut capped) = (0usize, 0usize);
    for (class, spec) in CLASSES {
        let text = records(root, class, spec, &man);
        let mut seen: HashMap<Key, ()> = HashMap::new();
        for line in text.lines() {
            // Cheap filter before parsing: only the IC arm carries a base.
            if !line.contains("\"arm\":\"ic-mitm\"") {
                continue;
            }
            let rec: Value = serde_json::from_str(line).unwrap();
            let cs = &rec["workload"]["curve_spec"];
            let key = curve_key(cs);
            if seen.insert(key, ()).is_some() {
                continue;
            }
            let fb_json = &rec["factor_base"];
            if fb_json["family"] != "prime-abscissa" || fb_json["params"]["size"] != "32" {
                die(&format!("{class}: base is not prime-abscissa:size=32"));
            }
            let u = |k: &str| cs[k].as_u64().unwrap();
            let (go, r) = (u("group_order"), u("r"));
            let inst = PrimeInstance {
                name: key_str(&key),
                curve: PrimeCurve {
                    p: key.0,
                    a: key.1,
                    b: key.2,
                },
                group_order: go,
                r,
                cofactor: go / r,
                generator: (key.3, key.4),
            };
            let fb = prime_factor_base(&inst, 32);
            // The hash ecbench records: sorted point keys, big-endian.
            let mut keys: Vec<u64> = fb.points.iter().map(|p| inst.curve.key(p)).collect();
            keys.sort_unstable();
            let bytes: Vec<u8> = keys.iter().flat_map(|k| k.to_be_bytes()).collect();
            let sha = sha256_hex(&bytes);
            let recorded = fb_json["points_sha256"].as_str().unwrap();
            if sha != recorded {
                die(&format!(
                    "{class} {}: base hash {sha} differs from the measured {recorded}",
                    key_str(&key)
                ));
            }
            checked += 1;
            let m2 = exact_decomposition_ceiling(&inst.curve, &fb.points, r, 2)
                .unwrap_or_else(|| die("N0 (m = 2) not computable"));
            let m3 = exact_decomposition_ceiling(&inst.curve, &fb.points, r, 3);
            if m2.1 >= 1.0 {
                capped += 1;
            }
            rows.push(json!({
                "class": class, "key": key_str(&key), "p": key.0, "a": key.1, "b": key.2,
                "gx": key.3, "gy": key.4, "group_order": go, "r": r, "h": go / r,
                "signed_points": fb.points.len(), "classes": m2.0,
                "n0_over_r_m2": m2.1, "n0_over_r_m3": m3.map(|x| x.1),
                "points_sha256": sha, "hash_matches_measured": true,
            }));
            csv.push_str(&format!(
                "{class},{},{},{r},{},{:.9e},{}\n",
                key_str(&key),
                go / r,
                m2.0,
                m2.1,
                m3.map_or("NA".into(), |x| format!("{:.9e}", x.1))
            ));
        }
    }
    eprintln!(
        "census: {} workloads, {checked} base hashes match the measured records, {capped} capped at 1",
        rows.len()
    );
    let doc = json!({"schema": "generator-n0.census/v1", "root": root, "m": 2,
        "workloads": rows.len(), "hash_checked": checked, "capped_at_one": capped,
        "rows": rows, "argv": std::env::args().collect::<Vec<_>>()});
    fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    if let Some(p) = get("--csv-out") {
        fs::write(p, csv).unwrap();
    }
}

// --------------------------------------------------------------- regress

#[derive(Clone)]
struct W {
    class: usize,
    vertex: u64,
    model: u64,
    level: u64,
    w: f64,
    rel: f64,
    tries: f64,
    h: u64,
}

fn load(root: &str, n0: &str) -> Vec<W> {
    let doc: Value = serde_json::from_str(&fs::read_to_string(n0).unwrap()).unwrap();
    let mut wmap: HashMap<String, (f64, u64)> = HashMap::new();
    for r in doc["rows"].as_array().unwrap() {
        wmap.insert(
            r["key"].as_str().unwrap().into(),
            (r["n0_over_r_m2"].as_f64().unwrap(), r["h"].as_u64().unwrap()),
        );
    }
    let man = manifest(root);
    let mut out = Vec::new();
    for (ci, (class, spec)) in CLASSES.iter().enumerate() {
        // Sidecar: key → (vertex, model, level).
        let mut side: HashMap<String, (u64, u64, u64)> = HashMap::new();
        for line in fs::read_to_string(format!("{root}/classes/{class}/curves.jsonl"))
            .unwrap()
            .lines()
        {
            let c: Value = serde_json::from_str(line).unwrap();
            let u = |k: &str| c[k].as_u64().unwrap();
            let k = (u("p"), u("a"), u("b"), u("gx"), u("gy"));
            side.insert(key_str(&k), (u("vertex"), u("model"), u("level")));
        }
        let mut acc: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        for line in records(root, class, spec, &man).lines() {
            if !line.contains("\"arm\":\"ic-mitm\"") {
                continue;
            }
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool() == Some(true) {
                continue;
            }
            if r["outcome"]["status"] != "verified" {
                die("unverified run in the IC arm");
            }
            let k = key_str(&curve_key(&r["workload"]["curve_spec"]));
            let e = acc.entry(k).or_insert((0.0, 0.0));
            e.0 += r["counters"]["relations_found"].as_f64().unwrap();
            e.1 += r["counters"]["targets_tried"].as_f64().unwrap();
        }
        for (k, (rel, tries)) in acc {
            let (vertex, model, level) = *side
                .get(&k)
                .unwrap_or_else(|| die(&format!("{class}: {k} not in the sidecar")));
            let (w, h) = *wmap
                .get(&k)
                .unwrap_or_else(|| die(&format!("{class}: {k} not in the census")));
            out.push(W {
                class: ci,
                vertex,
                model,
                level,
                w,
                rel,
                tries,
                h,
            });
        }
    }
    out
}

/// Weighted least squares slope of `y` on `x`; `group` demeans within groups
/// first (fixed effects). Weights are the relation counts.
fn slope(d: &[(&W, usize)], fe: bool) -> f64 {
    let pts: Vec<(f64, f64, f64, usize)> = d
        .iter()
        .filter(|(w, _)| w.rel > 0.0)
        .map(|(w, g)| (w.w.ln(), (w.rel / w.tries).ln(), w.rel, *g))
        .collect();
    let mut gm: HashMap<usize, (f64, f64, f64)> = HashMap::new();
    if fe {
        for (x, y, wt, g) in &pts {
            let e = gm.entry(*g).or_default();
            e.0 += wt * x;
            e.1 += wt * y;
            e.2 += wt;
        }
    } else {
        let e = gm.entry(0).or_default();
        for (x, y, wt, _) in &pts {
            e.0 += wt * x;
            e.1 += wt * y;
            e.2 += wt;
        }
    }
    let (mut sxy, mut sxx) = (0.0, 0.0);
    for (x, y, wt, g) in &pts {
        let m = &gm[&if fe { *g } else { 0 }];
        let (dx, dy) = (x - m.0 / m.2, y - m.1 / m.2);
        sxy += wt * dx * dy;
        sxx += wt * dx * dx;
    }
    sxy / sxx
}

fn ols(d: &[(&W, usize)]) -> f64 {
    let pts: Vec<(f64, f64)> = d
        .iter()
        .filter(|(w, _)| w.rel > 0.0)
        .map(|(w, _)| (w.w.ln(), (w.rel / w.tries).ln()))
        .collect();
    let n = pts.len() as f64;
    let (mx, my) = (
        pts.iter().map(|p| p.0).sum::<f64>() / n,
        pts.iter().map(|p| p.1).sum::<f64>() / n,
    );
    pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum::<f64>()
        / pts.iter().map(|p| (p.0 - mx).powi(2)).sum::<f64>()
}

fn quantile(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((q * (v.len() - 1) as f64).round() as usize).min(v.len() - 1)]
}

/// v1.1 stratified floor/crater contrast: mean over the classes in `set` of
/// ln(floor/crater yield), the ratio of sums; `adj` divides each workload's
/// relations by its `N0/r`. `level_of` gives a curve's (possibly permuted) level.
fn contrast(
    ws: &[&W],
    set: &[usize],
    adj: bool,
    level_of: &dyn Fn(&W) -> u64,
    floor: &[u64],
) -> Option<f64> {
    let (mut acc, mut k) = (0.0, 0);
    for &c in set {
        let mut cr = (0.0, 0.0);
        let mut fl = (0.0, 0.0);
        for w in ws.iter().filter(|w| w.class == c) {
            let n = if adj { w.rel / w.w } else { w.rel };
            let l = level_of(w);
            if l == 0 {
                cr.0 += n;
                cr.1 += w.tries;
            } else if l == floor[c] {
                fl.0 += n;
                fl.1 += w.tries;
            }
        }
        if cr.1 > 0.0 && fl.1 > 0.0 && cr.0 > 0.0 && fl.0 > 0.0 {
            acc += ((fl.0 / fl.1) / (cr.0 / cr.1)).ln();
            k += 1;
        }
    }
    (k > 0).then(|| acc / k as f64)
}

fn regress(argv: &[String]) {
    let get = |k: &str| argv.iter().position(|a| a == k).and_then(|i| argv.get(i + 1));
    let root = get("--root").map_or(DEFAULT_ROOT, |s| s.as_str());
    let n0 = get("--n0").unwrap_or_else(|| die("regress needs --n0"));
    let out = get("--out").unwrap_or_else(|| die("regress needs --out"));
    let boot: usize = get("--boot").map_or(2000, |v| v.parse().unwrap());
    let perms: usize = get("--perm").map_or(10000, |v| v.parse().unwrap());
    let mut rng = SmallRng::seed_from_u64(20261017);
    let ws = load(root, n0);
    let excluded = ws.iter().filter(|w| w.rel == 0.0).count();

    // Clusters: the 200 curves (class, vertex) with their models.
    let mut clusters: BTreeMap<(usize, u64), Vec<usize>> = BTreeMap::new();
    for (i, w) in ws.iter().enumerate() {
        clusters.entry((w.class, w.vertex)).or_default().push(i);
    }
    let cl: Vec<&Vec<usize>> = clusters.values().collect();
    let floor: Vec<u64> = (0..6)
        .map(|c| ws.iter().filter(|w| w.class == c).map(|w| w.level).max().unwrap())
        .collect();

    // G1: the point estimates and the cluster bootstrap. Groups: 0 for
    // none, class for class fixed effects, the draw index for curve ones.
    let view = |sel: &[usize], g: &dyn Fn(usize, &W) -> usize| -> Vec<(usize, usize)> {
        sel.iter().enumerate().map(|(pos, &i)| (i, g(pos, &ws[i]))).collect()
    };
    let est = |sel: &[usize], draws: &[usize]| -> [f64; 4] {
        let mk = |g: &dyn Fn(usize, &W) -> usize| -> Vec<(&W, usize)> {
            view(sel, g).into_iter().map(|(i, gg)| (&ws[i], gg)).collect()
        };
        let none = mk(&|_, _| 0);
        let byclass = mk(&|_, w| w.class);
        let bycurve = mk(&|pos, _| draws[pos]);
        [ols(&none), slope(&none, false), slope(&byclass, true), slope(&bycurve, true)]
    };
    let all: Vec<usize> = (0..ws.len()).collect();
    let draws: Vec<usize> = all
        .iter()
        .map(|&i| cl.iter().position(|c| c.contains(&i)).unwrap())
        .collect();
    let point = est(&all, &draws);
    let mut bs: [Vec<f64>; 4] = Default::default();
    for _ in 0..boot {
        let (mut sel, mut dr) = (Vec::new(), Vec::new());
        for d in 0..cl.len() {
            let c = cl[rng.gen_range(0..cl.len())];
            for &i in c {
                sel.push(i);
                dr.push(d);
            }
        }
        let e = est(&sel, &dr);
        for k in 0..4 {
            bs[k].push(e[k]);
        }
    }
    let names = ["ols", "wls_pooled", "wls_class_fe", "wls_curve_fe"];
    let g1: Vec<Value> = (0..4)
        .map(|k| {
            json!({"name": names[k], "slope": point[k],
                   "ci95": [quantile(&mut bs[k].clone(), 0.025), quantile(&mut bs[k].clone(), 0.975)]})
        })
        .collect();
    let spread = {
        let xs: Vec<f64> = ws.iter().map(|w| w.w.ln()).collect();
        let m = xs.iter().sum::<f64>() / xs.len() as f64;
        (xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / xs.len() as f64).sqrt()
    };
    let g1_pass = (0.9..=1.1).contains(&point[1]);

    // G4: the contrast, raw and adjusted, and the explained fraction.
    let refs: Vec<&W> = ws.iter().collect();
    let identity = |w: &W| w.level;
    let sets: [(&str, Vec<usize>); 4] = [
        ("l2_all (primary)", vec![0, 1, 2]),
        ("l2_ramified_split", vec![0, 1]),
        ("l3_all (control)", vec![3, 4, 5]),
        ("all_six", vec![0, 1, 2, 3, 4, 5]),
    ];
    let mut g4 = Vec::new();
    for (name, set) in &sets {
        let raw = contrast(&refs, set, false, &identity, &floor);
        let adj = contrast(&refs, set, true, &identity, &floor);
        let (mut br, mut ba, mut be) = (Vec::new(), Vec::new(), Vec::new());
        for _ in 0..boot {
            // Curves within class × level.
            let mut sample: Vec<&W> = Vec::new();
            let mut strata: BTreeMap<(usize, u64), Vec<&Vec<usize>>> = BTreeMap::new();
            for c in &cl {
                let w = &ws[c[0]];
                strata.entry((w.class, w.level)).or_default().push(c);
            }
            for cs in strata.values() {
                for _ in 0..cs.len() {
                    for &i in cs[rng.gen_range(0..cs.len())] {
                        sample.push(&ws[i]);
                    }
                }
            }
            if let (Some(r), Some(a)) = (
                contrast(&sample, set, false, &identity, &floor),
                contrast(&sample, set, true, &identity, &floor),
            ) {
                br.push(r);
                ba.push(a);
                be.push(1.0 - a / r);
            }
        }
        // Permutation of level labels among a class's curves; p for C_adj.
        let mut ge = 0usize;
        if let Some(a0) = adj {
            for _ in 0..perms {
                let mut lv: HashMap<(usize, u64), u64> = HashMap::new();
                for &c in set {
                    let keys: Vec<(usize, u64)> =
                        clusters.keys().copied().filter(|k| k.0 == c).collect();
                    let mut labels: Vec<u64> =
                        keys.iter().map(|k| ws[clusters[k][0]].level).collect();
                    labels.shuffle(&mut rng);
                    for (k, l) in keys.into_iter().zip(labels) {
                        lv.insert(k, l);
                    }
                }
                let pl = |w: &W| lv[&(w.class, w.vertex)];
                if let Some(p) = contrast(&refs, set, true, &pl, &floor) {
                    if p.abs() >= a0.abs() - 1e-12 {
                        ge += 1;
                    }
                }
            }
        }
        let q = |v: &Vec<f64>, x: f64| if v.is_empty() { f64::NAN } else { quantile(&mut v.clone(), x) };
        let explained = raw.zip(adj).map(|(r, a)| 1.0 - a / r);
        let testable = br.len() > 0 && !(q(&br, 0.025) <= 0.0 && q(&br, 0.975) >= 0.0);
        g4.push(json!({
            "set": name, "classes": set.iter().map(|&c| CLASSES[c].0).collect::<Vec<_>>(),
            "c_raw": raw, "c_raw_ci95": [q(&br, 0.025), q(&br, 0.975)],
            "ratio_raw": raw.map(f64::exp),
            "c_adj": adj, "c_adj_ci90": [q(&ba, 0.05), q(&ba, 0.95)],
            "ratio_adj": adj.map(f64::exp),
            "c_adj_perm_p": (ge + 1) as f64 / (perms + 1) as f64,
            "explained": explained, "explained_ci95": [q(&be, 0.025), q(&be, 0.975)],
            "testable": testable,
            "pass": if testable { json!(explained.is_some_and(|e| e >= 0.8)) } else { Value::Null },
        }));
    }

    // Context for the report: yield by cofactor and the spread of N0/r.
    let mut by_h: BTreeMap<u64, (f64, f64, f64, usize)> = BTreeMap::new();
    for w in &ws {
        let e = by_h.entry(w.h).or_default();
        e.0 += w.rel;
        e.1 += w.tries;
        e.2 += w.w.ln();
        e.3 += 1;
    }
    let by_h: Vec<Value> = by_h
        .iter()
        .map(|(h, e)| json!({"h": h, "workloads": e.3, "yield": e.0 / e.1, "mean_ln_w": e.2 / e.3 as f64}))
        .collect();

    let doc = json!({
        "schema": "generator-n0.regression/v1", "seed": 20261017, "boot": boot, "perm": perms,
        "workloads": ws.len(), "curves": cl.len(), "excluded_zero_relations": excluded,
        "ln_w_sd": spread, "g1": {"pass_point": g1_pass, "band": [0.9, 1.1], "estimates": g1},
        "g4": g4, "by_cofactor": by_h,
        "argv": std::env::args().collect::<Vec<_>>(),
    });
    fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    if let Some(p) = get("--csv-out") {
        let mut s = String::from("class,vertex,model,level,h,w,relations,tries,yield\n");
        for w in &ws {
            s.push_str(&format!(
                "{},{},{},{},{},{:.9e},{},{},{:.9e}\n",
                CLASSES[w.class].0, w.vertex, w.model, w.level, w.h, w.w, w.rel, w.tries,
                w.rel / w.tries
            ));
        }
        fs::write(p, s).unwrap();
    }
    eprintln!(
        "G1 slope (WLS) {:.4} [{:.4}, {:.4}] pass={g1_pass}; OLS {:.4}; class-FE {:.4}; curve-FE {:.4}; excluded {excluded}",
        point[1],
        quantile(&mut bs[1].clone(), 0.025),
        quantile(&mut bs[1].clone(), 0.975),
        point[0],
        point[2],
        point[3]
    );
    for g in &g4 {
        eprintln!(
            "G4 {:>20}: raw {:?} adj {:?} explained {:?} testable {} pass {}",
            g["set"].as_str().unwrap(),
            g["ratio_raw"],
            g["ratio_adj"],
            g["explained"],
            g["testable"],
            g["pass"]
        );
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match argv.first().map(|s| s.as_str()) {
        Some("census") => census(&argv[1..]),
        Some("regress") => regress(&argv[1..]),
        _ => die("usage: generator-n0 census|regress ..."),
    }
}

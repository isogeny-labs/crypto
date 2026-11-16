//! Independent recompute of the headline numbers of
//! research/isogeny-volcano-cost-v2_20261017 (validator, 2026-10-22).
//!
//! Written from PROTOCOL.md section 4 and the statistic definitions only; shares no
//! code with volcano-stats or ecbench. Reads the committed records.jsonl.gz
//! and curves.jsonl directly.
//!
//! usage: vrecompute STUDY_DIR NONCE_HEX OUT.json

use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

// ---------------------------------------------------------------- arithmetic
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, p);
        }
        b = mulm(b, b, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    // extended Euclid (independent of Fermat; p is checked prime separately)
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (p as i128, (a % p) as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert!(r == 1, "not invertible");
    (((t % p as i128) + p as i128) % p as i128) as u64
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Pt {
    Inf,
    A(u64, u64),
}
fn on_curve(pt: Pt, a: u64, b: u64, p: u64) -> bool {
    match pt {
        Pt::Inf => true,
        Pt::A(x, y) => mulm(y, y, p) == addm(addm(mulm(mulm(x, x, p), x, p), mulm(a, x, p), p), b, p),
    }
}
fn ec_add(u: Pt, v: Pt, a: u64, p: u64) -> Pt {
    match (u, v) {
        (Pt::Inf, q) | (q, Pt::Inf) => q,
        (Pt::A(x1, y1), Pt::A(x2, y2)) => {
            let lam = if x1 == x2 {
                if addm(y1, y2, p) == 0 {
                    return Pt::Inf;
                }
                // tangent
                mulm(addm(mulm(3, mulm(x1, x1, p), p), a, p), invm(mulm(2, y1, p), p), p)
            } else {
                mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
            };
            let x3 = subm(subm(mulm(lam, lam, p), x1, p), x2, p);
            let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
            Pt::A(x3, y3)
        }
    }
}
fn ec_mul(mut k: u128, pt: Pt, a: u64, p: u64) -> Pt {
    let mut acc = Pt::Inf;
    let mut base = pt;
    while k > 0 {
        if k & 1 == 1 {
            acc = ec_add(acc, base, a, p);
        }
        base = ec_add(base, base, a, p);
        k >>= 1;
    }
    acc
}
fn jinv(a: u64, b: u64, p: u64) -> u64 {
    let a3 = mulm(4, mulm(mulm(a, a, p), a, p), p);
    let den = addm(a3, mulm(27, mulm(b, b, p), p), p);
    mulm(mulm(1728 % p, a3, p), invm(den, p), p)
}
fn hex(v: &Value) -> u64 {
    u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
}

// ------------------------------------------------------------------ records
#[derive(Clone, Copy)]
struct Side {
    vertex: u64,
    model: u64,
    level: u64,
    j: u64,
}

#[derive(Default, Clone)]
struct CurveAgg {
    level: u64,
    s_sum: f64,
    n: f64,
    rel: f64,
    tried: f64,
    ic: bool,
}

#[derive(Default)]
struct BandAgg {
    class: usize,
    vertex: u64,
    model: u64,
    n: u64,
    sums: [f64; 3],
    missing: bool,
}

const W: [f64; 3] = [0.0, 0.1, 1.0];

/// Count behind each unpriced counter (AGENTS.md section 5a definition as documented in the
/// band's module comment): top-level counter by name, else the phases' native
/// maps by name or by the name without `_uncharged`, else solver ops for a
/// `solver_*` counter. Returns (total, any_missing).
fn unpriced(r: &Value) -> (f64, bool) {
    let mut tot = 0.0;
    let mut missing = false;
    for name in r["cost"]["unpriced"].as_array().into_iter().flatten() {
        let name = name.as_str().unwrap();
        if let Some(v) = r["counters"].get(name).and_then(|v| v.as_u64()) {
            tot += v as f64;
            continue;
        }
        let base = name.strip_suffix("_uncharged").unwrap_or(name);
        let mut found = false;
        for ph in r["phases"].as_array().into_iter().flatten() {
            let nat = &ph["native"];
            if let Some(v) = nat.get(name).or_else(|| nat.get(base)).and_then(|v| v.as_u64()) {
                tot += v as f64;
                found = true;
            }
        }
        if !found && base.starts_with("solver_") {
            if let Some(ops) = r["solver"].get("ops").and_then(|v| v.as_u64()) {
                tot += ops as f64;
                continue;
            }
        }
        if !found {
            missing = true;
        }
    }
    (tot, missing)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn median(v: &mut Vec<f64>) -> (f64, f64, f64) {
    // (lower middle, upper middle, average of the two)
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    let lo = v[(n - 1) / 2];
    let hi = v[n / 2];
    (lo, hi, (lo + hi) / 2.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = &args[1];
    let nonce = u64::from_str_radix(&args[2][..16], 16).unwrap();
    let out_path = &args[3];

    let mut classes: Vec<String> = Vec::new();
    for l in ["l2", "l3"] {
        for s in 1001..=1014 {
            classes.push(format!("{l}-s{s}"));
        }
    }

    // per-class sidecar
    let mut sides: Vec<HashMap<(u64, u64, u64, u64, u64), Side>> = Vec::new();
    let mut floor_level: Vec<u64> = Vec::new();
    let mut n_curve_models = 0usize;
    let mut curve_checks: BTreeMap<String, u64> = BTreeMap::new();
    let mut bump = |m: &mut BTreeMap<String, u64>, k: &str| *m.entry(k.to_string()).or_default() += 1;
    for c in &classes {
        let f = BufReader::new(File::open(format!("{dir}/classes/{c}/curves.jsonl")).unwrap());
        let mut m = HashMap::new();
        let mut fl = 0;
        for line in f.lines() {
            let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let u = |k: &str| v[k].as_u64().unwrap();
            let key = (u("p"), u("a"), u("b"), u("gx"), u("gy"));
            let s = Side { vertex: u("vertex"), model: u("model"), level: u("level"), j: u("j") };
            assert!(m.insert(key, s).is_none(), "duplicate sidecar key");
            fl = fl.max(s.level);
            n_curve_models += 1;
        }
        sides.push(m);
        floor_level.push(fl);
    }

    // aggregates
    let mut session_counts: BTreeMap<String, Value> = BTreeMap::new();
    let mut total_records = 0u64;
    let mut total_warmup = 0u64;
    let mut measured_verified = 0u64;
    let mut measured_unverified = 0u64;
    let mut unmatched = 0u64;
    let mut scalar_ok = 0u64;
    let mut scalar_bad: Vec<String> = Vec::new();
    let mut status_vs_check_disagree = 0u64;
    let mut nonverified: Vec<String> = Vec::new();
    let mut binaries: BTreeSet<String> = BTreeSet::new();
    let mut pinned_calibration = 0u64;
    let mut s_consistency_max_rel = 0f64;
    let mut autos: BTreeSet<u64> = BTreeSet::new();
    let mut arms_seen: BTreeMap<String, u64> = BTreeMap::new();
    let mut curve_cache: HashMap<(u64, u64, u64, u64, u64), bool> = HashMap::new();

    // (arm, class, vertex) -> CurveAgg
    let mut cur: BTreeMap<(String, usize, u64), CurveAgg> = BTreeMap::new();
    // pair: arm -> (class, vertex, model, target) -> (rel, tried)
    let mut pair: HashMap<String, HashMap<(usize, u64, u64, u64), (f64, f64)>> = HashMap::new();
    let mut pair_dupes = 0u64;
    // bsgs exactness: (class, target) -> distinct s bit patterns
    let mut bsgs: BTreeMap<(usize, u64), BTreeSet<u64>> = BTreeMap::new();
    // band: (slug, arm) -> agg (main session only)
    let mut band: BTreeMap<(String, String), BandAgg> = BTreeMap::new();

    for (ci, c) in classes.iter().enumerate() {
        let main = if std::path::Path::new(&format!("{dir}/sessions/{c}/spec-r2")).is_dir() { "spec-r2" } else { "spec" };
        let mut sess: Vec<(String, bool)> = vec![(main.to_string(), true)];
        for s in ["spec-aa", "spec-bsgs", "spec-icctl"] {
            sess.push((s.to_string(), true));
        }
        if main == "spec-r2" {
            sess.push(("spec".to_string(), false)); // interrupted: counted and scalar-checked, not analysed
        }
        for (s, analysed) in &sess {
            let path = format!("{dir}/sessions/{c}/{s}/records.jsonl.gz");
            let rd = BufReader::with_capacity(1 << 20, GzDecoder::new(File::open(&path).unwrap()));
            let (mut n, mut nw, mut nmv, mut nother) = (0u64, 0u64, 0u64, 0u64);
            for line in rd.lines() {
                let line = line.unwrap();
                let r: Value = serde_json::from_str(&line).unwrap();
                n += 1;
                total_records += 1;
                let warm = r["warmup"].as_bool().unwrap();
                if warm {
                    nw += 1;
                    total_warmup += 1;
                }
                let arm = r["arm"].as_str().unwrap().to_string();
                let status = r["outcome"]["status"].as_str().unwrap_or("").to_string();
                binaries.insert(r["binary_sha256"].as_str().unwrap_or("?").to_string());
                if let Some(a) = r["workload"]["curve"]["automorphisms_available"].as_u64() {
                    autos.insert(a);
                }
                let cs = &r["workload"]["curve_spec"];
                let u = |k: &str| cs[k].as_u64().unwrap();
                let (p, a, b, gx, gy) = (u("p"), u("a"), u("b"), u("gx"), u("gy"));
                let key = (p, a, b, gx, gy);
                // curve-level checks, once per curve-model
                let side = sides[ci].get(&key).copied();
                let good_curve = *curve_cache.entry(key).or_insert_with(|| {
                    let rr = u("r");
                    let order = u("group_order");
                    let g = Pt::A(gx, gy);
                    let mut ok = true;
                    let mut chk = |name: &str, cond: bool| {
                        if !cond {
                            ok = false;
                            eprintln!("curve check {name} failed for {key:?}");
                        }
                    };
                    chk("p_prime", is_prime(p));
                    chk("r_prime", is_prime(rr));
                    chk("r_divides_order", order % rr == 0);
                    chk("nonsingular", addm(mulm(4, powm(a, 3, p), p), mulm(27, mulm(b, b, p), p), p) != 0);
                    chk("j_not_0", a % p != 0);
                    chk("j_not_1728", b % p != 0);
                    chk("G_on_curve", on_curve(g, a, b, p));
                    chk("rG_is_O", ec_mul(rr as u128, g, a, p) == Pt::Inf);
                    chk("order_kills_G", ec_mul(order as u128, g, a, p) == Pt::Inf);
                    // Hasse bound for the recorded group order
                    let d = (order as f64 - (p as f64 + 1.0)).abs();
                    chk("hasse", d <= 2.0 * (p as f64).sqrt() + 1e-9);
                    if let Some(sd) = side {
                        chk("j_matches_sidecar", jinv(a, b, p) == sd.j);
                    }
                    ok
                });
                bump(&mut curve_checks, if good_curve { "records_on_good_curves" } else { "records_on_bad_curves" });

                // independent scalar check: [k]G == Q, Q on curve, k == planted mod r
                let q = Pt::A(hex(&r["workload"]["target"][0]), hex(&r["workload"]["target"][1]));
                let rr = u("r") as u128;
                let mut check_ok = false;
                if let Some(ks) = r["outcome"]["recovered"].as_str() {
                    let k: u128 = ks.parse().unwrap();
                    let planted = r["workload"]["planted"].as_u64().unwrap() as u128;
                    check_ok = on_curve(q, a, b, p)
                        && ec_mul(k, Pt::A(gx, gy), a, p) == q
                        && k % rr == planted % rr;
                }
                let verified = status == "verified";
                if verified && check_ok {
                    scalar_ok += 1;
                } else if verified && !check_ok {
                    scalar_bad.push(format!("{c}/{s} seq {}", r["seq"]));
                }
                if verified != check_ok {
                    status_vs_check_disagree += 1;
                }
                if !verified {
                    nonverified.push(format!("{c}/{s} seq {} arm {arm} warmup {warm} status {status}", r["seq"]));
                }
                if !warm && verified {
                    nmv += 1;
                } else if !warm {
                    nother += 1;
                }
                if !analysed {
                    continue;
                }
                if !warm {
                    if verified {
                        measured_verified += 1;
                    } else {
                        measured_unverified += 1;
                    }
                }
                if warm || !verified {
                    continue;
                }
                *arms_seen.entry(arm.clone()).or_default() += 1;
                let Some(sd) = side else {
                    unmatched += 1;
                    continue;
                };
                let s_val = r["cost"]["s"].as_f64().unwrap();
                let gae = r["cost"]["total_gae"].as_f64().unwrap();
                let sq = (u("r") as f64).sqrt();
                s_consistency_max_rel = s_consistency_max_rel.max(((gae / sq) - s_val).abs() / s_val.abs().max(1e-300));
                if r["detail"]["calibration_pinned_units"].as_array().map_or(false, |x| !x.is_empty()) {
                    pinned_calibration += 1;
                }
                let target = r["workload"]["target_index"].as_u64().unwrap();
                let rel = r["counters"]["relations_found"].as_f64();
                let tried = r["counters"]["targets_tried"].as_f64();
                let e = cur.entry((arm.clone(), ci, sd.vertex)).or_insert_with(|| CurveAgg { level: sd.level, ..Default::default() });
                assert_eq!(e.level, sd.level);
                e.s_sum += s_val;
                e.n += 1.0;
                if let (Some(rl), Some(tr)) = (rel, tried) {
                    e.ic = true;
                    e.rel += rl;
                    e.tried += tr;
                    if pair
                        .entry(arm.clone())
                        .or_default()
                        .insert((ci, sd.vertex, sd.model, target), (rl, tr))
                        .is_some()
                    {
                        pair_dupes += 1;
                    }
                }
                if r["method"]["family"].as_str() == Some("bsgs") {
                    bsgs.entry((ci, target)).or_default().insert(s_val.to_bits());
                }
                if s == main {
                    let (un, miss) = unpriced(&r);
                    let slug = r["workload"]["curve"]["slug"].as_str().unwrap().to_string();
                    let b = band.entry((slug, arm.clone())).or_default();
                    b.class = ci;
                    b.vertex = sd.vertex;
                    b.model = sd.model;
                    b.n += 1;
                    for (i, w) in W.iter().enumerate() {
                        b.sums[i] += (gae + w * un) / sq;
                    }
                    if miss {
                        b.missing = true;
                    }
                }
            }
            session_counts.insert(
                format!("{c}/{s}"),
                json!({"records": n, "warmup": nw, "measured_verified": nmv, "measured_other": nother, "analysed": analysed}),
            );
            eprintln!("{c}/{s}: {n} records");
        }
    }

    // ------------------------------------------------------------ statistics
    let l2: Vec<usize> = (0..14).collect();
    let l3: Vec<usize> = (14..28).collect();
    let all: Vec<usize> = (0..28).collect();
    let v2l2: Vec<usize> = (0..4).collect();
    let v2l3: Vec<usize> = (14..18).collect();
    let v2all: Vec<usize> = v2l2.iter().chain(v2l3.iter()).copied().collect();

    // per class, per curve: (level, num, den) for (arm, metric)
    let curves_of = |arm: &str, metric: &str, ci: usize| -> Vec<(u64, f64, f64)> {
        cur.range((arm.to_string(), ci, 0)..=(arm.to_string(), ci, u64::MAX))
            .map(|(_, a)| if metric == "yield" { (a.level, a.rel, a.tried) } else { (a.level, a.s_sum, a.n) })
            .collect()
    };
    let class_ratio = |cv: &[(u64, f64, f64)], fl: u64| -> f64 {
        let (mut nf, mut df, mut nc, mut dc) = (0.0, 0.0, 0.0, 0.0);
        for &(l, n, d) in cv {
            if l == fl && l > 0 {
                nf += n;
                df += d;
            } else if l == 0 {
                nc += n;
                dc += d;
            }
        }
        (nf / df) / (nc / dc)
    };
    let strat = |arm: &str, metric: &str, cls: &[usize]| -> (f64, Vec<(String, f64)>) {
        let mut acc = 0.0;
        let mut per = Vec::new();
        for &ci in cls {
            let x = class_ratio(&curves_of(arm, metric, ci), floor_level[ci]);
            acc += x.ln();
            per.push((classes[ci].clone(), x));
        }
        ((acc / cls.len() as f64).exp(), per)
    };
    // permutation p: level labels shuffled among each class's curves
    let mut rng = Rng(nonce);
    let mut perm_p = |arm: &str, metric: &str, cls: &[usize], alt: &str, perms: usize| -> (f64, usize) {
        let data: Vec<Vec<(u64, f64, f64)>> = cls.iter().map(|&ci| curves_of(arm, metric, ci)).collect();
        let obs = {
            let mut acc = 0.0;
            for (k, &ci) in cls.iter().enumerate() {
                acc += class_ratio(&data[k], floor_level[ci]).ln();
            }
            acc / cls.len() as f64
        };
        let mut ge = 0usize;
        for _ in 0..perms {
            let mut acc = 0.0;
            for (k, &ci) in cls.iter().enumerate() {
                let mut labels: Vec<u64> = data[k].iter().map(|x| x.0).collect();
                for i in (1..labels.len()).rev() {
                    let j = rng.below(i + 1);
                    labels.swap(i, j);
                }
                let sh: Vec<(u64, f64, f64)> = data[k].iter().zip(labels).map(|(x, l)| (l, x.1, x.2)).collect();
                acc += class_ratio(&sh, floor_level[ci]).ln();
            }
            let m = acc / cls.len() as f64;
            let extreme = match alt {
                "less" => m <= obs + 1e-12,
                _ => m.abs() >= obs.abs() - 1e-12,
            };
            if extreme {
                ge += 1;
            }
        }
        ((ge as f64 + 1.0) / (perms as f64 + 1.0), ge)
    };

    let (h1, h1_per) = strat("ic-mitm", "yield", &l2);
    let (h2, h2_per) = strat("ic-mitm", "yield", &l3);
    let (h3, _) = strat("rho-neg", "S", &all);
    let (h5_l2, _) = strat("ic-mitm", "S", &l2);
    let (h5_l3, _) = strat("ic-mitm", "S", &l3);
    let (rho_l2, _) = strat("rho-neg", "S", &l2);
    let (rho_l3, _) = strat("rho-neg", "S", &l3);
    let (v2_h1, _) = strat("ic-mitm", "yield", &v2l2);
    let (v2_h2, _) = strat("ic-mitm", "yield", &v2l3);
    let (v2_h3, _) = strat("rho-neg", "S", &v2all);
    let (v2_h5_l2, _) = strat("ic-mitm", "S", &v2l2);
    let (v2_h5_l3, _) = strat("ic-mitm", "S", &v2l3);
    let (_, all_yield_per) = strat("ic-mitm", "yield", &all);

    let (p_h1, ge_h1) = perm_p("ic-mitm", "yield", &l2, "less", 10000);
    let (p_h2, ge_h2) = perm_p("ic-mitm", "yield", &l3, "two-sided", 10000);
    let (p_h3, ge_h3) = perm_p("rho-neg", "S", &all, "two-sided", 10000);

    // planted pair ic-mitm-plus1 : ic-mitm, matched by (class, vertex, model, target)
    let pa = &pair["ic-mitm-plus1"];
    let pb = &pair["ic-mitm"];
    let mut per_class_sums: BTreeMap<usize, (f64, f64, f64, f64, u64)> = BTreeMap::new();
    let mut matched = 0u64;
    for (k, &(ra, ta)) in pa {
        if let Some(&(rb, tb)) = pb.get(k) {
            let e = per_class_sums.entry(k.0).or_default();
            e.0 += ra;
            e.1 += ta;
            e.2 += rb;
            e.3 += tb;
            e.4 += 1;
            matched += 1;
        }
    }
    let pair_stat = |cls: &[usize]| -> f64 {
        let mut acc = 0.0;
        let mut k = 0;
        for ci in cls {
            if let Some(e) = per_class_sums.get(ci) {
                acc += ((e.0 / e.1) / (e.2 / e.3)).ln();
                k += 1;
            }
        }
        (acc / k as f64).exp()
    };
    let planted_all = pair_stat(&all);
    let planted_l2 = pair_stat(&l2);
    let planted_l3 = pair_stat(&l3);

    // BSGS exactness
    let bsgs_targets = bsgs.len();
    let bsgs_max_distinct = bsgs.values().map(|s| s.len()).max().unwrap_or(0);

    // band
    let mut slugs: BTreeMap<String, (Option<&BandAgg>, Option<&BandAgg>)> = BTreeMap::new();
    for ((slug, arm), b) in &band {
        let e = slugs.entry(slug.clone()).or_default();
        match arm.as_str() {
            "ic-mitm" => e.0 = Some(b),
            "rho-neg" => e.1 = Some(b),
            _ => {}
        }
    }
    let mut ratios: [Vec<f64>; 3] = [vec![], vec![], vec![]];
    let mut ic_means: [Vec<f64>; 3] = [vec![], vec![], vec![]];
    let mut rho_means: [Vec<f64>; 3] = [vec![], vec![], vec![]];
    let mut rho_below_everywhere = 0u64;
    let mut band_incomplete = 0u64;
    let mut band_missing = 0u64;
    let mut slug_keys: BTreeSet<(usize, u64, u64)> = BTreeSet::new();
    for (_, (ic, rho)) in &slugs {
        let (Some(ic), Some(rho)) = (ic, rho) else {
            band_incomplete += 1;
            continue;
        };
        if ic.missing || rho.missing {
            band_missing += 1;
        }
        slug_keys.insert((ic.class, ic.vertex, ic.model));
        let mut below = true;
        for i in 0..3 {
            let mi = ic.sums[i] / ic.n as f64;
            let mr = rho.sums[i] / rho.n as f64;
            ratios[i].push(mi / mr);
            ic_means[i].push(mi);
            rho_means[i].push(mr);
            if !(mr < mi) {
                below = false;
            }
        }
        if below {
            rho_below_everywhere += 1;
        }
    }
    let mut band_out = Vec::new();
    let mut gae_band = serde_json::Map::new();
    for i in 0..3 {
        let mut r = ratios[i].clone();
        let (lo, hi, avg) = median(&mut r);
        let mn = r.first().copied().unwrap();
        let mx = r.last().copied().unwrap();
        let (_, _, ic_med) = median(&mut ic_means[i].clone());
        let (_, _, rho_med) = median(&mut rho_means[i].clone());
        let ic_mean = ic_means[i].iter().sum::<f64>() / ic_means[i].len() as f64;
        let rho_mean = rho_means[i].iter().sum::<f64>() / rho_means[i].len() as f64;
        band_out.push(json!({"weight": W[i], "curve_models": r.len(), "ratio_min": mn, "ratio_median_lower": lo, "ratio_median_upper": hi, "ratio_median": avg, "ratio_max": mx,
            "ic_S_median_over_curve_models": ic_med, "rho_S_median_over_curve_models": rho_med, "ic_S_mean_over_curve_models": ic_mean, "rho_S_mean_over_curve_models": rho_mean}));
        gae_band.insert(format!("{}", W[i]), json!({"ic": ic_med, "rho": rho_med}));
    }

    let out = json!({
        "tool": "vrecompute (validator scratch program; independent of volcano-stats and ecbench)",
        "nonce_for_permutations": args[2],
        "counts": {
            "sidecar_curve_models": n_curve_models,
            "total_records_including_interrupted": total_records,
            "warmup_records": total_warmup,
            "measured_verified_in_analysed_sessions": measured_verified,
            "measured_unverified_in_analysed_sessions": measured_unverified,
            "unmatched_measured_records": unmatched,
            "arms_measured_verified": arms_seen,
            "pinned_calibration_records": pinned_calibration,
            "binary_sha256_values": binaries,
            "automorphisms_available_values": autos,
            "max_rel_dev_total_gae_over_sqrt_r_vs_s": s_consistency_max_rel,
            "sessions": session_counts,
        },
        "scalar_check": {
            "verified_records_scalar_reproduced": scalar_ok,
            "verified_records_scalar_failed": scalar_bad,
            "status_vs_independent_check_disagreements": status_vs_check_disagree,
            "nonverified_records": nonverified,
            "curve_checks": curve_checks,
            "distinct_curve_models_checked": curve_cache.len(),
            "distinct_curve_models_failed": curve_cache.values().filter(|v| !**v).count(),
        },
        "floor_level_per_class": floor_level,
        "v2.1": {
            "H1_l2_ic_yield_floor_over_crater": h1,
            "H1_per_class": h1_per,
            "H1_perm_p_less": p_h1, "H1_perm_extreme": ge_h1,
            "H2_l3_ic_yield_floor_over_crater": h2,
            "H2_per_class": h2_per,
            "H2_perm_p_two_sided": p_h2, "H2_perm_extreme": ge_h2,
            "H3_all_rho_S_floor_over_crater": h3,
            "H3_perm_p_two_sided": p_h3, "H3_perm_extreme": ge_h3,
            "H5_ic_S_l2": h5_l2, "H5_ic_S_l3": h5_l3,
            "rho_S_l2": rho_l2, "rho_S_l3": rho_l3,
            "H1_power_plant_0.95_point": h2 * 0.95,
            "H3_power_plant_1.05_point": h3 * 1.05,
            "per_class_ic_yield": all_yield_per,
            "planted_ic_pair_all": planted_all, "planted_ic_pair_l2": planted_l2, "planted_ic_pair_l3": planted_l3,
            "planted_pair_matched_workloads": matched, "planted_pair_classes": per_class_sums.len(), "pair_duplicate_keys": pair_dupes,
        },
        "v2": {
            "H1": v2_h1, "H2": v2_h2, "H3": v2_h3, "H5_ic_S_l2": v2_h5_l2, "H5_ic_S_l3": v2_h5_l3,
        },
        "bsgs_exactness": {"paired_targets": bsgs_targets, "max_distinct_cost_per_target": bsgs_max_distinct},
        "band": {
            "curve_models_with_both_arms": slug_keys.len(),
            "slugs_missing_an_arm": band_incomplete,
            "slugs_with_uncounted_unpriced": band_missing,
            "curve_models_rho_below_ic_at_every_weight": rho_below_everywhere,
            "per_weight": band_out,
            "gae_band_median_of_per_curve_model_mean_S": gae_band,
        },
    });
    std::fs::write(out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
}

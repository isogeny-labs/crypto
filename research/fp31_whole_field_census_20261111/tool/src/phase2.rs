//! Phase 2 (rho across isogeny classes): `roots`, `select`, `analyze`.
//!
//! `roots`   draws curves y^2 = x^3 + a x + b over F_p (p = 3 mod 4) from a
//!           seeded SplitMix64 stream and finds #E by baby-step giant-step on a
//!           random point; it keeps a curve only when exactly one multiple of the
//!           point order lies in the Hasse interval (then that multiple is #E).
//! `select`  joins roots with the census rows by trace and applies the frozen
//!           selection rule of PROTOCOL.md section 3 (first-come in draw order).
//! `analyze` reads every class's isoclass-rho trials.csv and computes the
//!           frozen tests of PROTOCOL.md section 5. Every statistic is computed here.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::nt::{factor, is_prime, isqrt};

// ---------------------------------------------------------------- arithmetic
fn mulm(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 { r = mulm(r, a, p); }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 { powm(a, p - 2, p) }

type Pt = Option<(u64, u64)>;

fn add(p: u64, a: u64, s: Pt, t: Pt) -> Pt {
    let (x1, y1) = match s { None => return t, Some(v) => v };
    let (x2, y2) = match t { None => return s, Some(v) => v };
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 { return None; }
        mulm((3 * mulm(x1, x1, p) + a) % p, inv(2 * y1 % p, p), p)
    } else {
        mulm((y2 + p - y1) % p, inv((x2 + p - x1) % p, p), p)
    };
    let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
    let y3 = (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p;
    Some((x3, y3))
}
fn neg(p: u64, s: Pt) -> Pt { s.map(|(x, y)| (x, (p - y) % p)) }
fn mul(p: u64, a: u64, mut k: u64, s: Pt) -> Pt {
    let (mut r, mut q) = (None, s);
    while k > 0 {
        if k & 1 == 1 { r = add(p, a, r, q); }
        q = add(p, a, q, q);
        k >>= 1;
    }
    r
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

/// All m in [lo, hi] with [m]P = O.
fn bsgs_multiples(p: u64, a: u64, pt: Pt, lo: u64, hi: u64) -> Vec<u64> {
    let w = hi - lo;
    let s = isqrt(w as u128) as u64 + 1;
    let mut table: HashMap<u64, u64> = HashMap::with_capacity(s as usize);
    let mut cur: Pt = None;
    for j in 1..=s {
        cur = add(p, a, cur, pt);
        if let Some((x, _)) = cur { table.entry(x).or_insert(j); }
    }
    let step = neg(p, mul(p, a, s, pt)); // giant step adds -sP? we walk G_i = (lo + i s)P
    let step = neg(p, step); // = sP
    let mut g = mul(p, a, lo, pt);
    let mut out = Vec::new();
    let mut i = 0u64;
    loop {
        let base = lo + i * s;
        if base > hi + s { break; }
        match g {
            None => out.push(base),
            Some((x, _)) => {
                if let Some(&j) = table.get(&x) {
                    let jp = mul(p, a, j, pt);
                    if jp == neg(p, g) { out.push(base + j); }
                    if jp == g && base >= j { out.push(base - j); }
                }
            }
        }
        g = add(p, a, g, step);
        i += 1;
    }
    out.retain(|&m| m >= lo && m <= hi && m > 0 && mul(p, a, m, pt).is_none());
    out.sort_unstable();
    out.dedup();
    out
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

pub fn cmd_roots(args: &[String]) {
    let p: u64 = arg(args, "--p").map(|s| s.parse().unwrap()).unwrap_or((1 << 31) - 1);
    let seed: u64 = arg(args, "--seed").expect("--seed").parse().unwrap();
    let draws: u64 = arg(args, "--draws").expect("--draws").parse().unwrap();
    let j0 = args.iter().any(|a| a == "--j0");
    let out = arg(args, "--out").expect("--out");
    assert!(p % 4 == 3 && is_prime(p as u128), "roots needs a prime p = 3 mod 4");
    let b2 = 2 * isqrt(p as u128) as u64;
    let (lo, hi) = (p + 1 - b2, p + 1 + b2);
    let mut rng = SplitMix(seed);
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    let (mut kept, mut ambiguous) = (0u64, 0u64);
    for i in 0..draws {
        let a = if j0 { 0 } else { 1 + rng.below(p - 1) };
        let b = 1 + rng.below(p - 1);
        if (4 * mulm(mulm(a, a, p), a, p) + 27 * mulm(b, b, p)) % p == 0 { continue; }
        // a random point: x from the stream until x^3 + a x + b is a nonzero square
        let pt = loop {
            let x = rng.below(p);
            let rhs = (mulm(mulm(x, x, p), x, p) + mulm(a, x, p) + b) % p;
            if rhs != 0 && powm(rhs, (p - 1) / 2, p) == 1 {
                break Some((x, powm(rhs, (p + 1) / 4, p)));
            }
        };
        let ms = bsgs_multiples(p, a, pt, lo, hi);
        if ms.len() != 1 { ambiguous += 1; continue; }
        let n = ms[0];
        let t = p as i64 + 1 - n as i64;
        let r = factor(n as u128).into_iter().map(|x| x.0 as u64).max().unwrap();
        kept += 1;
        writeln!(f, "{{\"draw\":{i},\"p\":{p},\"a\":{a},\"b\":{b},\"N\":{n},\"t\":{t},\"r\":{r},\"cofactor\":{}}}", n / r).unwrap();
    }
    eprintln!("{{\"draws\":{draws},\"kept\":{kept},\"ambiguous_or_dropped\":{ambiguous},\"seed\":{seed},\"j0\":{j0}}}");
}

// ------------------------------------------------------------------ select
fn jget(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find(|c: char| c == ',' || c == '}').unwrap();
    Some(rest[..end].trim_matches('"').to_string())
}

/// Frozen rule (PROTOCOL.md section 3): in draw order, prime-order roots fill
/// strata S1 (one endomorphism ring), S2 (two), S3 (three or more), `per` each,
/// one root per trace; the first `nscale` roots with 4 <= cofactor <= 64 form
/// the r-scaling control; `--j0` roots supply the automorphism-fold control.
pub fn cmd_select(args: &[String]) {
    let roots = arg(args, "--roots").expect("--roots");
    let rows = arg(args, "--rows").expect("--rows");
    let j0 = arg(args, "--j0-roots").expect("--j0-roots");
    let per: usize = arg(args, "--per").expect("--per").parse().unwrap();
    let nscale: usize = arg(args, "--scale").expect("--scale").parse().unwrap();
    let base_seed: u64 = arg(args, "--seed").expect("--seed").parse().unwrap();
    let out = arg(args, "--out").expect("--out");
    let mut census: HashMap<i64, String> = HashMap::new();
    for line in fs::read_to_string(&rows).unwrap().lines() {
        census.insert(jget(line, "t").unwrap().parse().unwrap(), line.to_string());
    }
    let mut seen: HashSet<i64> = HashSet::new();
    let mut fill: BTreeMap<&str, usize> = BTreeMap::new();
    let mut picked: Vec<(String, String, String)> = Vec::new(); // (stratum, root line, census line)
    for line in fs::read_to_string(&roots).unwrap().lines() {
        let t: i64 = jget(line, "t").unwrap().parse().unwrap();
        let cof: u64 = jget(line, "cofactor").unwrap().parse().unwrap();
        if seen.contains(&t) { continue; }
        let row = census.get(&t).expect("trace missing from census");
        let no: u64 = jget(row, "n_orders").unwrap().parse().unwrap();
        let stratum = if cof == 1 {
            match no { 1 => "S1", 2 => "S2", _ => "S3" }
        } else if (4..=64).contains(&cof) {
            "R"
        } else {
            continue;
        };
        let cap = if stratum == "R" { nscale } else { per };
        let e = fill.entry(stratum).or_insert(0);
        if *e >= cap { continue; }
        *e += 1;
        seen.insert(t);
        picked.push((stratum.to_string(), line.to_string(), row.clone()));
    }
    if let Some(line) = fs::read_to_string(&j0).unwrap().lines().find(|l| jget(l, "cofactor").map_or(false, |c| c.parse::<u64>().unwrap() <= 64)) {
        let t: i64 = jget(line, "t").unwrap().parse().unwrap();
        picked.push(("J0".into(), line.to_string(), census[&t].clone()));
    }
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    let mut s = String::from("[\n");
    for (i, (st, root, row)) in picked.iter().enumerate() {
        let g = |k: &str| jget(row, k).unwrap();
        s += &format!(
            "{{\"id\":\"c{i:03}\",\"stratum\":\"{st}\",\"seed\":{},\"p\":{},\"a\":{},\"b\":{},\"N\":{},\"t\":{},\"r\":{},\"H_num\":{},\"H_den\":{},\"f\":{},\"n_orders\":{},\"D_K\":{},\"draw\":{}}}{}\n",
            base_seed * 1000 + i as u64, jget(root, "p").unwrap(), jget(root, "a").unwrap(), jget(root, "b").unwrap(),
            jget(root, "N").unwrap(), g("t"), g("r"), g("H_num"), g("H_den"), g("f"), g("n_orders"), g("D_K"),
            jget(root, "draw").unwrap(), if i + 1 < picked.len() { "," } else { "" }
        );
    }
    s += "]\n";
    f.write_all(s.as_bytes()).unwrap();
    eprintln!("selected {:?} + J0: {}", fill, picked.iter().any(|x| x.0 == "J0"));
}

// ----------------------------------------------------------------- analyze
struct Solve { arm: String, kind: String, c: f64 }

fn read_trials(path: &str, r: f64) -> Vec<Solve> {
    let text = match fs::read_to_string(path) { Ok(t) => t, Err(_) => return vec![] };
    let norm = (std::f64::consts::PI * r / 2.0).sqrt();
    let mut v = Vec::new();
    for line in text.lines().skip(1) {
        let c: Vec<&str> = line.split(',').collect();
        let iters: f64 = c[6].parse().unwrap();
        v.push(Solve { arm: c[0].to_string(), kind: c[1].to_string(), c: iters / norm });
    }
    v
}

fn mean(x: &[f64]) -> f64 { x.iter().sum::<f64>() / x.len() as f64 }
fn var(x: &[f64]) -> f64 { let m = mean(x); x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() as f64 - 1.0) }

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] { j += 1; }
        for k in i..=j { r[idx[k]] = (i + j) as f64 / 2.0 + 1.0; }
        i = j + 1;
    }
    r
}

/// ln Gamma (Lanczos), for the chi-square tail.
fn ln_gamma(x: f64) -> f64 {
    let g = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for c in g { y += 1.0; ser += c / y; }
    -tmp + (2.5066282746310005 * ser / x).ln()
}
/// Upper regularized incomplete gamma Q(a, x).
fn gamma_q(a: f64, x: f64) -> f64 {
    if x <= 0.0 { return 1.0; }
    if x < a + 1.0 {
        let (mut sum, mut del, mut ap) = (1.0 / a, 1.0 / a, a);
        for _ in 0..10000 { ap += 1.0; del *= x / ap; sum += del; if del.abs() < sum.abs() * 1e-15 { break; } }
        1.0 - sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        let (mut b, mut c, mut d) = (x + 1.0 - a, 1e300, 1.0 / (x + 1.0 - a));
        let mut h = d;
        for i in 1..10000 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b; if d.abs() < 1e-300 { d = 1e-300; }
            c = b + an / c; if c.abs() < 1e-300 { c = 1e-300; }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 { break; }
        }
        (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

/// Kruskal-Wallis H with tie correction; (H, df, p) by the chi-square tail.
fn kruskal(groups: &[Vec<f64>]) -> (f64, usize, f64) {
    let all: Vec<f64> = groups.iter().flatten().cloned().collect();
    let n = all.len() as f64;
    let r = ranks(&all);
    let mut h = 0.0;
    let mut off = 0;
    for g in groups {
        let s: f64 = r[off..off + g.len()].iter().sum();
        h += s * s / g.len() as f64;
        off += g.len();
    }
    h = 12.0 / (n * (n + 1.0)) * h - 3.0 * (n + 1.0);
    let mut sorted = all.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut tie = 0.0;
    let mut i = 0;
    while i < sorted.len() {
        let mut j = i;
        while j + 1 < sorted.len() && sorted[j + 1] == sorted[i] { j += 1; }
        let t = (j - i + 1) as f64;
        tie += t * t * t - t;
        i = j + 1;
    }
    h /= 1.0 - tie / (n * n * n - n);
    let df = groups.len() - 1;
    (h, df, gamma_q(df as f64 / 2.0, h / 2.0))
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let (ma, mb) = (mean(a), mean(b));
    let (mut s, mut sa, mut sb) = (0.0, 0.0, 0.0);
    for i in 0..a.len() { s += (a[i] - ma) * (b[i] - mb); sa += (a[i] - ma).powi(2); sb += (b[i] - mb).powi(2); }
    s / (sa * sb).sqrt()
}

/// Spearman rho and a two-sided permutation p (seeded shuffles of y).
fn spearman_perm(x: &[f64], y: &[f64], perms: usize, seed: u64) -> (f64, f64) {
    let rx = ranks(x);
    let mut ry = ranks(y);
    let rho = pearson(&rx, &ry);
    let mut rng = SplitMix(seed);
    let mut ge = 0usize;
    for _ in 0..perms {
        for i in (1..ry.len()).rev() { let j = rng.below(i as u64 + 1) as usize; ry.swap(i, j); }
        if pearson(&rx, &ry).abs() >= rho.abs() - 1e-12 { ge += 1; }
    }
    (rho, (ge + 1) as f64 / (perms + 1) as f64)
}

/// One-sample KS of p-values against U(0,1); (D, asymptotic p).
fn ks_uniform(ps: &[f64]) -> (f64, f64) {
    let mut v = ps.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len() as f64;
    let mut d: f64 = 0.0;
    for (i, &x) in v.iter().enumerate() {
        d = d.max((i as f64 + 1.0) / n - x).max(x - i as f64 / n);
    }
    let lam = (n.sqrt() + 0.12 + 0.11 / n.sqrt()) * d;
    let mut p = 0.0;
    for k in 1..200 { let k = k as f64; p += 2.0 * (-1f64).powf(k - 1.0) * (-2.0 * k * k * lam * lam).exp(); }
    (d, p.clamp(0.0, 1.0))
}

fn ratio(a: &[f64], b: &[f64]) -> (f64, f64) {
    let (ma, mb) = (mean(a), mean(b));
    let r = ma / mb;
    let se = r * (var(a) / (a.len() as f64 * ma * ma) + var(b) / (b.len() as f64 * mb * mb)).sqrt();
    (r, se)
}

pub fn cmd_analyze(args: &[String]) {
    let manifest = arg(args, "--manifest").expect("--manifest");
    let runs = arg(args, "--runs").expect("--runs");
    let out = arg(args, "--out").expect("--out");
    let perms: usize = arg(args, "--perms").map(|s| s.parse().unwrap()).unwrap_or(20000);
    let text = fs::read_to_string(&manifest).unwrap();
    let lines: Vec<&str> = text.lines().filter(|l| l.starts_with('{')).collect();
    let mut o = String::from("{\n");
    let mut class_rows = Vec::new();
    let (mut main_groups, mut cm, mut lh, mut lf, mut no, mut ldk) = (vec![], vec![], vec![], vec![], vec![], vec![]);
    let (mut within_p, mut gen_p) = (vec![], vec![]);
    let mut ctl: BTreeMap<String, (Vec<f64>, Vec<f64>)> = BTreeMap::new(); // arm kind -> (arm solves, root v0 solves)
    let (mut scale_c, mut main_c_all) = (vec![], vec![]);
    let mut missing = Vec::new();
    for l in &lines {
        let id = jget(l, "id").unwrap();
        let st = jget(l, "stratum").unwrap();
        let r: f64 = jget(l, "r").unwrap().parse().unwrap();
        let h: f64 = jget(l, "H_num").unwrap().parse::<f64>().unwrap() / jget(l, "H_den").unwrap().parse::<f64>().unwrap();
        let cls = read_trials(&format!("{runs}/{id}/class/trials.csv"), r);
        let gen = read_trials(&format!("{runs}/{id}/gen/trials.csv"), r);
        if cls.is_empty() { missing.push(id.clone()); continue; }
        let iso: Vec<&Solve> = cls.iter().filter(|s| s.kind == "isogenous").collect();
        let v0: Vec<f64> = cls.iter().filter(|s| s.arm == "v0").map(|s| s.c).collect();
        let mut by_v: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for s in &iso { by_v.entry(s.arm.clone()).or_default().push(s.c); }
        let wp = if by_v.len() >= 2 { Some(kruskal(&by_v.values().cloned().collect::<Vec<_>>()).2) } else { None };
        let mut by_g: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for s in gen.iter().filter(|s| s.kind == "generator") { by_g.entry(s.arm.clone()).or_default().push(s.c); }
        let gp = if by_g.len() >= 2 { Some(kruskal(&by_g.values().cloned().collect::<Vec<_>>()).2) } else { None };
        for s in cls.iter().filter(|s| matches!(s.kind.as_str(), "plant" | "fold" | "aa-replica" | "iso-control")) {
            let key = if s.kind == "iso-control" { "iso-control".to_string() } else { s.arm.clone() };
            let key = if st == "J0" { format!("J0:{key}") } else { key };
            ctl.entry(key).or_default().0.push(s.c);
        }
        for (k, e) in ctl.iter_mut() {
            if (st == "J0") == k.starts_with("J0:") && cls.iter().any(|s| (s.arm == *k.trim_start_matches("J0:") || (k.ends_with("iso-control") && s.kind == "iso-control"))) {
                e.1.extend(v0.iter().cloned());
            }
        }
        let isoc: Vec<f64> = iso.iter().map(|s| s.c).collect();
        let m = mean(&isoc);
        if st.starts_with('S') {
            main_groups.push(isoc.clone());
            main_c_all.extend(isoc.iter().cloned());
            cm.push(m);
            lh.push(h.log2());
            lf.push((jget(l, "f").unwrap().parse::<f64>().unwrap()).log2());
            no.push(jget(l, "n_orders").unwrap().parse::<f64>().unwrap());
            ldk.push((jget(l, "D_K").unwrap().parse::<f64>().unwrap()).abs().log2());
            if let Some(p) = wp { within_p.push(p); }
            if let Some(p) = gp { gen_p.push(p); }
        } else if st == "R" {
            scale_c.extend(isoc.iter().cloned());
        }
        class_rows.push(format!(
            "{{\"id\":\"{id}\",\"stratum\":\"{st}\",\"t\":{},\"log2_r\":{:.4},\"H\":{h},\"n_orders\":{},\"vertices\":{},\"solves_isogenous\":{},\"mean_c\":{m:.5},\"sd_c\":{:.5},\"mean_c_v0\":{:.5},\"within_kw_p\":{},\"generators\":{},\"generator_kw_p\":{}}}",
            jget(l, "t").unwrap(), r.log2(), jget(l, "n_orders").unwrap(), by_v.len(), isoc.len(), var(&isoc).sqrt(),
            if v0.is_empty() { f64::NAN } else { mean(&v0) },
            wp.map(|p| format!("{p:.6}")).unwrap_or("null".into()), by_g.len(), gp.map(|p| format!("{p:.6}")).unwrap_or("null".into())
        ));
    }
    o += &format!("\"classes\":[\n{}\n],\n\"missing\":{:?},\n", class_rows.join(",\n"), missing);
    // primary family (PROTOCOL.md section 5): 7 tests, Bonferroni alpha 0.05/7
    let (hk, dfk, pk) = kruskal(&main_groups);
    let sp = [("log2_H", &lh), ("log2_f", &lf), ("n_orders", &no), ("log2_abs_DK", &ldk)];
    let mut tests = vec![format!("{{\"test\":\"T1 cross-class Kruskal-Wallis on c\",\"H\":{hk:.3},\"df\":{dfk},\"p\":{pk:.6}}}")];
    for (i, (name, x)) in sp.iter().enumerate() {
        let (rho, p) = spearman_perm(x, &cm, perms, 0x5EED + i as u64);
        tests.push(format!("{{\"test\":\"T{} Spearman class mean c vs {name}\",\"rho\":{rho:.4},\"p_perm\":{p:.6},\"n\":{}}}", i + 2, cm.len()));
    }
    let (dw, pw) = ks_uniform(&within_p);
    tests.push(format!("{{\"test\":\"T6 within-class KW p-values ~ U(0,1)\",\"classes\":{},\"D\":{dw:.4},\"p\":{pw:.6},\"min_p\":{:.6}}}", within_p.len(), within_p.iter().cloned().fold(1.0, f64::min)));
    let (dg, pg) = ks_uniform(&gen_p);
    tests.push(format!("{{\"test\":\"T7 generator KW p-values ~ U(0,1)\",\"classes\":{},\"D\":{dg:.4},\"p\":{pg:.6},\"min_p\":{:.6}}}", gen_p.len(), gen_p.iter().cloned().fold(1.0, f64::min)));
    o += &format!("\"alpha_bonferroni\":{:.6},\n\"primary_tests\":[\n{}\n],\n", 0.05 / 7.0, tests.join(",\n"));
    let sd_between = var(&cm).sqrt();
    let se_within = mean(&main_groups.iter().map(|g| var(g) / g.len() as f64).collect::<Vec<_>>()).sqrt();
    o += &format!("\"class_means\":{{\"n\":{},\"mean\":{:.5},\"sd_between\":{sd_between:.5},\"expected_sd_from_sampling\":{se_within:.5},\"min\":{:.5},\"max\":{:.5}}},\n",
        cm.len(), mean(&cm), cm.iter().cloned().fold(f64::INFINITY, f64::min), cm.iter().cloned().fold(0.0, f64::max));
    let mut cl = Vec::new();
    for (k, (a, b)) in &ctl {
        if a.len() > 1 && b.len() > 1 {
            let (r, se) = ratio(a, b);
            cl.push(format!("{{\"arm\":\"{k}\",\"ratio_to_root_v0\":{r:.4},\"se\":{se:.4},\"n_arm\":{},\"n_root\":{}}}", a.len(), b.len()));
        }
    }
    if scale_c.len() > 1 && main_c_all.len() > 1 {
        let (r, se) = ratio(&scale_c, &main_c_all);
        cl.push(format!("{{\"arm\":\"R r-scaling classes vs main classes (normalised c)\",\"ratio\":{r:.4},\"se\":{se:.4},\"n_arm\":{},\"n_root\":{}}}", scale_c.len(), main_c_all.len()));
    }
    o += &format!("\"controls\":[\n{}\n]\n}}\n", cl.join(",\n"));
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists");
    f.write_all(o.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bsgs_finds_order_small_field() {
        // brute-force #E for a few curves over p = 1019 (3 mod 4) and compare
        let p = 1019u64;
        for (a, b) in [(1u64, 1u64), (2, 3), (5, 7), (100, 200)] {
            let mut n = 1u64;
            for x in 0..p {
                let rhs = (x * x % p * x + a * x + b) % p;
                n += if rhs == 0 { 1 } else if powm(rhs, (p - 1) / 2, p) == 1 { 2 } else { 0 };
            }
            let b2 = 2 * isqrt(p as u128) as u64;
            let mut found = false;
            for x in 0..p {
                let rhs = (x * x % p * x + a * x + b) % p;
                if rhs != 0 && powm(rhs, (p - 1) / 2, p) == 1 {
                    let ms = bsgs_multiples(p, a, Some((x, powm(rhs, (p + 1) / 4, p))), p + 1 - b2, p + 1 + b2);
                    assert!(ms.contains(&n), "a={a} b={b} n={n} ms={ms:?}");
                    found = true;
                    break;
                }
            }
            assert!(found);
        }
    }
    #[test]
    fn kruskal_and_gamma() {
        assert!((gamma_q(1.0, 2.0) - (-2f64).exp()).abs() < 1e-12);
        let g = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let (h, df, _) = kruskal(&g);
        assert_eq!(df, 1);
        assert!((h - 3.857142857).abs() < 1e-6);
    }
}

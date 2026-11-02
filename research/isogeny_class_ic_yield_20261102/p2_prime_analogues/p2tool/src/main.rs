//! p2tool: build / spec / analyze for P2 (scaled ECCp-131 analogues).
//! Rust only (AGENTS.md "Implementation language: no Python"). libpari is
//! used through dlopen by `build` alone, on a machine that has it.
mod analyze;
mod ec;
mod pari;
mod spec;

use ec::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn parse_ints(s: &str) -> Vec<u64> {
    s.trim_matches(|c| c == '[' || c == ']' || c == ' ' || c == '~')
        .split(',')
        .filter_map(|t| t.trim().parse::<u64>().ok())
        .collect()
}

/// Deterministic stream: sha256(seed || counter).
pub struct Prg {
    seed: String,
    ctr: u64,
}
impl Prg {
    pub fn new(seed: &str) -> Self {
        Prg { seed: seed.to_string(), ctr: 0 }
    }
    pub fn next(&mut self) -> u64 {
        let h = Sha256::digest(format!("{}|{}", self.seed, self.ctr).as_bytes());
        self.ctr += 1;
        u64::from_le_bytes(h[..8].try_into().unwrap())
    }
}


/// p = (t^2 - D)/4 just below 2^bits with p and n = p+1-t both prime (t > 0).
fn find_pt(dd: i64, bits: u32) -> Option<(u64, i128, u64)> {
    let pmax = (1u128 << bits) - 1;
    let mut t = ((4 * pmax as i128 + dd as i128) as f64).sqrt() as i128;
    if t.rem_euclid(2) != dd.rem_euclid(2) as i128 {
        t -= 1;
    }
    while t > 2 {
        let p4 = t * t - dd as i128;
        if p4 % 4 == 0 && p4 / 4 <= pmax as i128 {
            let p = (p4 / 4) as u64;
            let n = (p as i128 + 1 - t) as u64;
            if is_prime(p) && is_prime(n) && n != p {
                return Some((p, t, n));
            }
        }
        t -= 2;
        if t < 3 * (pmax as f64).sqrt() as i128 / 2 {
            return None; // p stays within [0.56, 1] * 2^bits
        }
    }
    None
}

struct ClassSpec {
    label: &'static str,
    h_lo: u64,
    h_hi: u64,
}
const CLASSES: [ClassSpec; 2] = [
    ClassSpec { label: "A", h_lo: 120, h_hi: 200 },
    ClassSpec { label: "B", h_lo: 300, h_hi: 400 },
];

fn build(args: &[String]) {
    let lib = arg(args, "--libpari").unwrap_or_else(|| std::env::var("P2_LIBPARI").expect("set --libpari or P2_LIBPARI"));
    let out = arg(args, "--out").unwrap_or("classes.json".into());
    let sizes: Vec<u32> = arg(args, "--sizes")
        .unwrap_or("24,28,32,36,40".into())
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let which: Vec<String> = arg(args, "--classes").unwrap_or("A,B".into()).split(',').map(String::from).collect();
    let n_iso: usize = arg(args, "--iso-models").map(|s| s.parse().unwrap()).unwrap_or(8);
    let pari = pari::Pari::open(&lib).expect("libpari");
    eprintln!("PARI {}", pari.version);

    // 1. Discriminants: first fundamental D (by |D|) with h in the class window.
    let mut chosen: BTreeMap<&str, (i64, u64)> = BTreeMap::new();
    let mut d: i64 = -3;
    while chosen.len() < CLASSES.len() {
        d -= 1;
        // D = 1 mod 8 splits 2, which forces #E = p+1-t even: no prime order.
        if d.rem_euclid(8) == 1 || pari.eval(&format!("isfundamental({d})")).unwrap() != "1" {
            continue;
        }
        let h: u64 = pari.eval(&format!("qfbclassno({d})")).unwrap().parse().unwrap();
        if !sizes.iter().all(|&b| find_pt(d, b).is_some()) {
            continue;
        }
        for c in CLASSES.iter() {
            if !chosen.contains_key(c.label) && h >= c.h_lo && h <= c.h_hi && d != -4 && d != -3 {
                chosen.insert(c.label, (d, h));
            }
        }
    }
    eprintln!("discriminants: {:?}", chosen);

    let mut classes = vec![];
    for &bits in &sizes {
        for label in &which {
            let (dd, h) = chosen[label.as_str()];
            // 2. (p, t): p = (t^2 - D)/4 just below 2^bits, p and n = p+1-t prime.
            let (p, t, n) = find_pt(dd, bits).expect("no prime-order (p,t)");
            assert!(2 * n as u128 > p as u128 + 1 + 2 * (p as f64).sqrt() as u128 + 2);
            eprintln!("  p={p} t={t} n={n}");
            // 3. All h roots of H_D mod p.
            let roots = parse_ints(&pari.eval(&format!("lift(polrootsmod(polclass({dd}),{p}))")).unwrap());
            assert_eq!(roots.len() as u64, h, "D={dd} p={p}: {} roots of H_D mod p, expected h={h}", roots.len());
            let rset: BTreeSet<u64> = roots.iter().cloned().collect();
            assert_eq!(rset.len(), roots.len());
            eprintln!("  roots done");
            // 4. A curve of exact order n for every j; verified without PARI.
            let mut curves = vec![];
            for &j in &roots {
                let v = parse_ints(&pari.eval(&format!("lift(ellfromj(Mod({j},{p})))")).unwrap());
                assert!(v.len() == 5 && v[0] == 0 && v[1] == 0 && v[2] == 0, "ellfromj shape for j={j}: {v:?}");
                let mut c = Curve { p, a: v[3], b: v[4] };
                if !order_is(&c, n) {
                    let mut dn = 2;
                    while legendre(dn, p) != -1 {
                        dn += 1;
                    }
                    let d2 = mulm(dn, dn, p);
                    let d3 = mulm(d2, dn, p);
                    c = Curve { p, a: mulm(c.a, d2, p), b: mulm(c.b, d3, p) };
                    assert!(order_is(&c, n), "neither twist has order {n} for j={j}");
                }
                assert_eq!(c.j(), j, "j mismatch");
                let (gx, gy) = c.first_point();
                curves.push(json!({"j": j, "a": c.a, "b": c.b, "gx": gx, "gy": gy, "group_order": n, "r": n,
                                   "order_verified": "n*G=O in Rust, n prime, Hasse-unique"}));
            }
            eprintln!("  curves done");
            // 5. Isogeny edges: F_p-rational l-isogenies from Phi_l(j, Y) (PARI polmodular).
            let idx: BTreeMap<u64, usize> = roots.iter().enumerate().map(|(i, &j)| (j, i)).collect();
            let mut edges: BTreeSet<(usize, usize, u64)> = BTreeSet::new();
            let mut off_class = 0u64;
            let mut deg: BTreeMap<u64, BTreeSet<usize>> = BTreeMap::new();
            let ells = [2u64, 3, 5, 7, 11, 13];
            for &l in &ells {
                pari.eval(&format!("Phi{l}=polmodular({l})")).unwrap();
                for (&j, &i) in &idx {
                    let rts = parse_ints(&pari.eval(&format!("lift(polrootsmod(subst(Phi{l},x,{j}),{p}))")).unwrap());
                    for r in rts {
                        match idx.get(&r) {
                            Some(&k) => {
                                edges.insert((i.min(k), i.max(k), l));
                                deg.entry(l).or_default().insert(i);
                            }
                            None => off_class += 1,
                        }
                    }
                }
            }
            // connectivity under l <= 13
            let mut comp: Vec<usize> = (0..roots.len()).collect();
            fn find(c: &mut Vec<usize>, x: usize) -> usize {
                let mut r = x;
                while c[r] != r {
                    r = c[r];
                }
                c[x] = r;
                r
            }
            for &(a, b, _) in &edges {
                let (ra, rb) = (find(&mut comp, a), find(&mut comp, b));
                comp[ra] = rb;
            }
            let ncomp = (0..roots.len()).filter(|&i| find(&mut comp, i) == i).count();
            // 6. Isomorphic-model controls: u-scaling of n_iso curves chosen by seeded PRG.
            let mut prg = Prg::new(&format!("p2-iso-{bits}-{label}-{dd}-{p}"));
            let mut iso = vec![];
            let mut taken = BTreeSet::new();
            while iso.len() < n_iso.min(roots.len()) {
                let ci = (prg.next() % roots.len() as u64) as usize;
                if !taken.insert(ci) {
                    continue;
                }
                let u = 2 + prg.next() % (p - 3);
                let (u2, u3) = (mulm(u, u, p), mulm(mulm(u, u, p), u, p));
                let (u4, u6) = (mulm(u2, u2, p), mulm(u3, u3, p));
                let base = &curves[ci];
                let (a, b) = (base["a"].as_u64().unwrap(), base["b"].as_u64().unwrap());
                let c2 = Curve { p, a: mulm(a, u4, p), b: mulm(b, u6, p) };
                let (gx, gy) = (mulm(base["gx"].as_u64().unwrap(), u2, p), mulm(base["gy"].as_u64().unwrap(), u3, p));
                assert!(c2.on(Some((gx, gy))) && c2.mul(n, Some((gx, gy))).is_none() && c2.j() == base["j"].as_u64().unwrap());
                iso.push(json!({"of": ci, "u": u, "a": c2.a, "b": c2.b, "gx": gx, "gy": gy, "group_order": n, "r": n, "j": c2.j()}));
            }
            let h_est = (dd.unsigned_abs() as f64).sqrt() / (dd.unsigned_abs() as f64).ln();
            eprintln!("bits={bits} class={label} D={dd} h={h} p={p} t={t} n={n} edges={} components={ncomp} off_class_rational_neighbours={off_class}", edges.len());
            classes.push(json!({
                "id": format!("c{bits}{label}"), "bits": bits, "label": label,
                "D": dd, "h": h, "p": p, "t": t.to_string(), "n": n,
                "frobenius_disc": (t * t - 4 * p as i128).to_string(),
                "conductor_of_Z_pi": 1,
                "fundamental_discriminant": true,
                "n_prime": true, "log2_p": (p as f64).log2(),
                "brauer_siegel_scale_sqrtD_over_lnD": h_est,
                "curves": curves, "iso_models": iso,
                "edges": edges.iter().map(|&(a, b, l)| json!([a, b, l])).collect::<Vec<_>>(),
                "edge_ells": ells,
                "connected_components_under_ells_le_13": ncomp,
                "off_class_rational_neighbours": off_class,
                "vertices_with_rational_neighbour_per_ell": deg.iter().map(|(l, s)| (l.to_string(), s.len())).collect::<BTreeMap<_, _>>(),
                "source": format!("PARI {} polclass/polrootsmod/ellfromj/polmodular via dlopen; #E re-verified in Rust", pari.version),
            }));
        }
    }
    let doc = json!({"schema": "p2.classes/v1", "pari_version": pari.version, "classes": classes});
    std::fs::write(&out, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    eprintln!("wrote {out}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("build") => build(&args),
        Some("spec") => spec::run(&args),
        Some("analyze") => analyze::run(&args),
        _ => eprintln!("usage: p2tool build|spec|analyze ..."),
    }
}

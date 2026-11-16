#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
#[path = "../hec_lazy_forest_20261115/lazy.rs"]
mod lazy;
mod sieve;
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, thread_curve},
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
    solve::Row,
};
use std::{collections::HashSet, fs, path::Path};

fn brute(c: &Curve, d: &Div, fb: &[(u64, u64)], h: usize) -> Vec<sieve::Relation> {
    if d.u.deg() != c.g as isize {
        return Vec::new();
    }
    let mut out = Vec::new();
    for lambda in 0..c.fp.p {
        let (q, t) = sieve::polynomial(c, d, lambda);
        let hits = fb[..h]
            .iter()
            .filter(|&&(x, _)| d.u.eval(x, &c.fp) != 0 && q.eval(x, &c.fp) == 0)
            .count();
        if hits < c.g - 1 {
            continue;
        }
        let mut rem = q;
        let mut roots = Vec::new();
        for x in 0..c.fp.p {
            let mut m = 0;
            while rem.deg() > 0 && rem.eval(x, &c.fp) == 0 {
                rem = rem.div_exact(&Poly::x_minus(x, &c.fp), &c.fp);
                m += 1;
            }
            if m > 0 {
                roots.push((x, m));
            }
        }
        if rem.deg() == 0 {
            out.push(sieve::Relation {
                lambda,
                terms: sieve::orient(c, &t, fb, &roots),
            });
        }
    }
    out
}

fn small_gate() {
    let mut cases = 0;
    let mut accepted = 0;
    let mut coverage = sieve::Counts::default();
    for g in [2, 3, 4] {
        for p in [11, 13] {
            for cc in [1, 2] {
                let Some(c) = thread_curve(p, cc, g) else {
                    continue;
                };
                let fb = factor_base(&c);
                let mut rng = Rng::new(9901);
                for index in 0..18 {
                    let d = if index == 0 {
                        c.zero()
                    } else if index == 1 {
                        c.place(fb[0].0, fb[0].1)
                    } else {
                        (0..g).fold(c.zero(), |d, _| {
                            let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                            c.add(&d, &c.place(x, y))
                        })
                    };
                    for h in [fb.len() / 2, fb.len()] {
                        let got = sieve::batch(&c, &d, &fb, h, &mut coverage);
                        assert_eq!(
                            got,
                            brute(&c, &d, &fb, h),
                            "g={g} p={p} c={cc} d={index} h={h}"
                        );
                        for r in &got {
                            assert_eq!(c.add(&d, &sieve::sum(&c, &fb, &r.terms)), c.zero());
                        }
                        accepted += got.len();
                        cases += 1;
                    }
                }
            }
        }
    }
    assert!(
        coverage.low_degree > 0
            && coverage.skipped_u > 0
            && coverage.repeated > 0
            && coverage.ramified > 0
    );
    println!("SMALL cases={cases} accepted={accepted} {coverage:?}");
}

fn fixture(path: &Path) -> (Curve, Div, u64) {
    let text = fs::read_to_string(path).unwrap();
    let line = text.lines().find(|s| s.starts_with("FIXTURE ")).unwrap();
    let v: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let (g, p, ell) = (v[0] as usize, v[1], v[4]);
    let mut cursor = 5;
    let mut poly = || {
        let n = v[cursor] as usize;
        cursor += 1;
        let p = Poly::from_coeffs(&v[cursor..cursor + n]);
        cursor += n;
        p
    };
    let c = Curve::new(Fp::new(p), poly());
    let d = Div {
        u: poly(),
        v: poly(),
    };
    assert_eq!(c.g, g);
    assert!(c.is_valid(&d));
    assert!(!c.is_zero(&d));
    assert!(c.is_zero(&c.mul(&d, ell)));
    (c, d, ell)
}

fn cofactor(c: &Curve, fb: &[(u64, u64)], ell: u64) -> u64 {
    let s = (c.fp.p as f64).sqrt() as u128;
    let lo = s.saturating_sub(1).pow((2 * c.g) as u32);
    let hi = (s + 2).pow((2 * c.g) as u32);
    assert!((ell as u128).pow(2) > hi);
    for h in lo.div_ceil(ell as u128).max(1)..=hi / (ell as u128) {
        let n = u64::try_from(h * (ell as u128)).unwrap();
        if fb
            .iter()
            .all(|&(x, y)| c.is_zero(&c.mul(&c.place(x, y), n)))
        {
            let h = h as u64;
            assert_ne!(h % ell, 0);
            return h;
        }
    }
    panic!("no annihilating cofactor certificate")
}

fn target(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    fb: &[(u64, u64)],
    h: usize,
    projected: &[Div],
    seed: u64,
) -> Option<u64> {
    let f = Fp::new(ell);
    let mut rng = Rng::new(20261115 + seed);
    let mut dense = dense::DenseForest::new(fb.len(), h + 2, ell);
    let mut lazy = lazy::LazyForest::new(fb.len(), h + 2, ell);
    let mut echelon = dense::Echelon::new(h, ell);
    let mut seen = HashSet::new();
    let mut stats = sieve::Counts::default();
    let mut admitted = 0;
    let mut duplicates = 0;
    let mut pure = 0;
    let mut found = None;
    let mut jumps = Vec::new();
    for _ in 0..16 {
        let a = rng.below(ell);
        let b = rng.below(ell);
        jumps.push((a, b, c.add(&c.mul(d1, a), &c.mul(d2, b))));
    }
    let (mut a, mut b, mut d) = (0, 0, c.zero());
    for _ in 0..128 {
        let (da, db, jump) = jumps[rng.below(16) as usize];
        a = f.add(a, da);
        b = f.add(b, db);
        d = c.add(&d, &jump);
        assert_eq!(
            d,
            c.add(&c.mul(d1, a), &c.mul(d2, b)),
            "public challenge labels"
        );
        for relation in sieve::batch(c, &d, fb, h, &mut stats) {
            assert_eq!(
                c.add(&d, &sieve::sum(c, fb, &relation.terms)),
                c.zero(),
                "raw principal"
            );
            let mut row: Row = relation
                .terms
                .iter()
                .map(|&(j, m)| (j as u32, f.from_i64(m)))
                .filter(|e| e.1 != 0)
                .collect();
            if a != 0 {
                row.push((fb.len() as u32, a));
            }
            if b != 0 {
                row.push((fb.len() as u32 + 1, b));
            }
            if row.is_empty() {
                continue;
            }
            let inverse = f.inv(row[0].1);
            for (_, v) in &mut row {
                *v = f.mul(*v, inverse);
            }
            if !seen.insert(row.clone()) {
                duplicates += 1;
                continue;
            }
            admitted += 1;
            let (mut off, mut on) = (Vec::new(), Row::new());
            for (j, v) in row {
                let j = j as usize;
                if j < h {
                    on.push((j as u32, v));
                } else if j < fb.len() {
                    off.push((j + 1, v));
                } else {
                    on.push(((h + j - fb.len()) as u32, v));
                }
            }
            assert!(off.len() <= 2);
            let expected = dense.insert(&off, &on);
            let got = lazy.insert(&off, &on);
            assert_eq!(got, expected, "real stream prefix");
            assert_eq!((dense.unions, dense.pins), (lazy.unions, lazy.pins));
            if let Some(row) = got {
                let sum = row.iter().fold(c.zero(), |s, &(j, v)| {
                    c.add(&s, &c.mul(&projected[j as usize], v))
                });
                assert_eq!(sum, c.zero(), "projected emitted relation");
                pure += 1;
                let mut rhs = 0;
                let mut equation = Row::new();
                for (j, v) in row {
                    if j as usize == h {
                        rhs = f.neg(v);
                    } else {
                        equation.push((if j as usize == h + 1 { h as u32 } else { j }, v));
                    }
                }
                if let Some(k) = echelon.insert(&equation, rhs) {
                    assert_eq!(c.mul(d1, k), *d2);
                    found = Some(k);
                }
            }
        }
    }
    println!("TARGET g={} p={} seed={seed} h={h} admitted={admitted} duplicates={duplicates} pure={pure} rank={} k={found:?} unions={} pins={} dense_bytes={} lazy_bytes={} stats={stats:?} lazy={:?}",c.g,c.fp.p,echelon.rank,lazy.unions,lazy.pins,dense.bytes(),lazy.retained_bytes(),lazy.counts);
    found
}

fn main() {
    small_gate();
    let root = std::env::args().nth(1).expect("fixture root");
    let mut solved = 0;
    for (g, p) in [
        (2, 1009),
        (2, 16381),
        (3, 127),
        (3, 1009),
        (4, 31),
        (4, 127),
    ] {
        let (c, d1, ell) = fixture(&Path::new(&root).join(format!("select-g{g}-p{p}-f0.out")));
        let fb = factor_base(&c);
        let h = ((p as f64).powf(1.0 - 1.0 / g as f64).ceil() as usize).min(fb.len());
        let co = cofactor(&c, &fb, ell);
        for seed in [8, 9, 10] {
            // Expected scalar remains outside target(), which receives public inputs only.
            let mut rng = Rng::new(202611150000 + seed);
            let k = 1 + rng.below(ell - 1);
            let d2 = c.mul(&d1, k);
            let mut projected: Vec<_> = fb[..h]
                .iter()
                .map(|&(x, y)| c.mul(&c.place(x, y), co))
                .collect();
            projected.push(c.mul(&d1, co));
            projected.push(c.mul(&d2, co));
            for d in &projected {
                assert!(c.is_zero(&c.mul(d, ell)));
            }
            let got = target(&c, &d1, &d2, ell, &fb, h, &projected, seed);
            if let Some(got) = got {
                assert_eq!(got, k);
                solved += 1;
            }
        }
    }
    println!(
        "TERMINAL targets=18 recovered={solved} capped={}",
        18 - solved
    );
}

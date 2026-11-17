use crate::core::Core;
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split, OracleStats},
    rng::Rng,
    solve::Row,
};
use std::time::Instant;
pub struct Tracker {
    pub h: usize,
    pub core: Core,
}
impl Tracker {
    pub fn new(n: usize, h: usize, ell: u64) -> Self {
        Self {
            h,
            core: Core::new(n, h, ell),
        }
    }
    pub fn feed(&mut self, places: &Row, constant: u64, target: u64, t: u64) {
        let (mut on, mut off) = (Row::new(), Vec::new());
        for &(j, v) in places {
            if (j as usize) < self.h {
                on.push((j, v));
            } else {
                off.push((j as usize + 1, v));
            }
        }
        if constant != 0 {
            on.push((self.h as u32, constant));
        }
        if target != 0 {
            on.push((self.h as u32 + 1, target));
        }
        assert!(off.len() <= 2);
        self.core.feed(&off, &on, t);
    }
}
#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub trials: u64,
    pub smooth: u64,
    pub raw_checks: u64,
    pub hash: u64,
    pub setup: u128,
    pub walk: u128,
    pub oracle: u128,
    pub rows: u128,
    pub total: u128,
}
pub fn run(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    seed: u64,
    hs: &[usize],
    gate: bool,
    stop: bool,
    limit: u64,
) -> (Report, Vec<Tracker>) {
    assert_eq!(c.g, 2);
    let all = Instant::now();
    let mut r = Report::default();
    let f = Fp::new(ell);
    let fb = factor_base(c);
    let n = fb.len();
    let mut tracks: Vec<_> = hs
        .iter()
        .map(|&h| {
            assert!(h <= n);
            Tracker::new(n, h, ell)
        })
        .collect();
    assert!(!tracks.is_empty());
    let mut rng = Rng::new(202611150000 + seed);
    let mut jumps = Vec::new();
    for _ in 0..16 {
        let a = rng.below(ell);
        let b = rng.below(ell);
        jumps.push((a, b, c.add(&c.mul(d1, a), &c.mul(d2, b))));
    }
    let (mut a, mut b, mut d) = (0, 0, c.zero());
    r.setup = all.elapsed().as_nanos();
    for t in 1..=limit {
        r.trials = t;
        let clock = Instant::now();
        let (da, db, jump) = jumps[rng.below(16) as usize];
        a = f.add(a, da);
        b = f.add(b, db);
        d = c.add(&d, &jump);
        r.walk += clock.elapsed().as_nanos();
        let clock = Instant::now();
        let roots = split(&d.u, &c.fp, &mut OracleStats::default());
        r.oracle += clock.elapsed().as_nanos();
        if let Some(roots) = roots {
            r.smooth += 1;
            let clock = Instant::now();
            let mut places = Row::new();
            let mut sum = c.zero();
            for &(x, m) in roots.as_slice() {
                let j = fb.binary_search_by_key(&x, |&(x, _)| x).unwrap();
                let y = d.v.eval(x, &c.fp);
                assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
                let v = if y == fb[j].1 {
                    m as u64
                } else {
                    assert_eq!(y, c.fp.neg(fb[j].1));
                    f.neg(m as u64)
                };
                places.push((j as u32, v));
                if gate {
                    sum = c.add(&sum, &c.mul(&c.place(x, y), m as u64));
                }
            }
            if gate {
                assert_eq!(sum, d);
                r.raw_checks += 1;
            }
            for &(j, v) in &places {
                r.hash = (r.hash ^ j as u64).wrapping_mul(0x100000001b3);
                r.hash = (r.hash ^ v).wrapping_mul(0x100000001b3);
            }
            r.hash = (r.hash ^ a).wrapping_mul(0x100000001b3);
            r.hash = (r.hash ^ b).wrapping_mul(0x100000001b3);
            for track in &mut tracks {
                let old = track.core.k;
                track.feed(&places, f.neg(a), f.neg(b), t);
                if old.is_none() {
                    if let Some(k) = track.core.k {
                        assert_eq!(c.mul(d1, k), *d2);
                    }
                }
            }
            for track in &tracks {
                assert_eq!(track.core.k, tracks[0].core.k, "partition prefix trial {t}");
            }
            r.rows += clock.elapsed().as_nanos();
            r.k = tracks[0].core.k;
            if stop && r.k.is_some() {
                break;
            }
        }
        if gate && [512, 2048, 8192, 32768].contains(&t) {
            assert_eq!(d, c.add(&c.mul(d1, a), &c.mul(d2, b)));
        }
    }
    if let Some(k) = r.k {
        assert_eq!(c.mul(d1, k), *d2);
    }
    r.total = all.elapsed().as_nanos();
    (r, tracks)
}

#[path = "../hec_dlp_graph_20261115/core.rs"]
mod core;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
mod engine;
use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::{collections::BTreeMap, fs, path::Path};
fn prime(n: u64) -> bool {
    n >= 2 && !(2..).take_while(|d| d * d <= n).any(|d| n % d == 0)
}
fn upper(p: u64, g: usize) -> u128 {
    let s = (p as f64).sqrt() as u128 + 2;
    s.pow((2 * g) as u32)
}
struct Fixture {
    c: Curve,
    d1: Div,
    ell: u64,
    family: u64,
    attempt: u64,
    selection_ns: u64,
}
impl Fixture {
    fn encode(&self) -> String {
        let mut v = vec![
            "FIXTURE".to_string(),
            self.c.g.to_string(),
            self.c.fp.p.to_string(),
            self.family.to_string(),
            self.attempt.to_string(),
            self.ell.to_string(),
        ];
        for p in [&self.c.f, &self.d1.u, &self.d1.v] {
            v.push(p.n.to_string());
            v.extend(p.c[..p.n].iter().map(u64::to_string));
        }
        v.push(self.selection_ns.to_string());
        v.join(" ")
    }
    fn parse(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next(), Some("FIXTURE"));
        let mut take = || it.next().unwrap().parse::<u64>().unwrap();
        let g = take() as usize;
        let p = take();
        let family = take();
        let attempt = take();
        let ell = take();
        assert!(prime(p) && p > 3 && p <= 1_000_000);
        assert!(prime(ell));
        let mut polys = Vec::new();
        for _ in 0..3 {
            let n = take() as usize;
            assert!(n <= 24);
            let cs: Vec<_> = (0..n).map(|_| take()).collect();
            assert!(cs.iter().all(|&v| v < p));
            polys.push(Poly::from_coeffs(&cs));
        }
        let selection_ns = take();
        assert!(it.next().is_none());
        let c = Curve::new(Fp::new(p), polys[0]);
        let d1 = Div {
            u: polys[1],
            v: polys[2],
        };
        assert_eq!(c.g, g);
        assert_eq!(c.f.lc(), 1);
        assert!(c.is_valid(&d1) && !c.is_zero(&d1));
        assert!(c.is_zero(&c.mul(&d1, ell)));
        assert!((ell as u128).pow(2) > upper(p, g));
        Self {
            c,
            d1,
            ell,
            family,
            attempt,
            selection_ns,
        }
    }
    fn target(&self, s: u64) -> (u64, Div) {
        let seed =
            9000001 ^ ((self.c.g as u64) << 48) ^ (self.c.fp.p << 16) ^ (self.family << 8) ^ s;
        let k = 1 + Rng::new(seed).below(self.ell - 1);
        (k, self.c.mul(&self.d1, k))
    }
}
fn synthetic() {
    let (mut prefixes, mut comparisons) = (0, 0);
    for ell in [5, 7, 101] {
        let f = Fp::new(ell);
        for n in [1, 3, 8, 20] {
            for seed in 0..32 {
                let mut rng = Rng::new(2026111500 + seed);
                let logs: Vec<_> = (0..n).map(|_| rng.below(ell)).collect();
                let target = rng.below(ell);
                let mut hs = vec![0, 1, n / 2, n];
                hs.sort_unstable();
                hs.dedup();
                let mut tracks: Vec<_> = hs
                    .into_iter()
                    .map(|h| engine::Tracker::new(n, h, ell))
                    .collect();
                let mut rows = Vec::new();
                for prefix in 0..=2 * n + 4 {
                    if prefix > 0 {
                        let mut map = BTreeMap::new();
                        for _ in 0..rng.below(3) {
                            let j = rng.below(n as u64) as u32;
                            let value = f.add(map.get(&j).copied().unwrap_or(0), rng.below(ell));
                            map.insert(j, value);
                        }
                        let places: Row = map.into_iter().filter(|&(_, v)| v != 0).collect();
                        let b = rng.below(ell);
                        let rhs = f.add(
                            places
                                .iter()
                                .fold(0, |s, &(j, c)| f.add(s, f.mul(c, logs[j as usize]))),
                            f.mul(b, target),
                        );
                        let mut row = places.clone();
                        if b != 0 {
                            row.push((n as u32, b));
                        }
                        rows.push((row, rhs));
                        for track in &mut tracks {
                            track.feed(&places, f.neg(rhs), b, prefix as u64);
                        }
                    }
                    let expected = solve_fast(&rows, n, ell, &mut SolveStats::default());
                    for track in &tracks {
                        assert_eq!(track.core.k, expected);
                        if let Some(k) = track.core.k {
                            assert_eq!(k, target);
                        }
                        comparisons += 1;
                    }
                    prefixes += 1;
                }
            }
        }
    }
    let mut track = engine::Tracker::new(2, 0, 101);
    track.feed(&vec![(0, 1), (1, 1)], 0, 1, 1);
    assert_eq!(track.core.k, None);
    track.feed(&vec![(0, 1), (1, 2)], 0, 0, 2);
    assert_eq!(track.core.k, None);
    assert!(track.core.forest.pins > 0);
    track.feed(&vec![(0, 1), (1, 1)], 0, 2, 3);
    assert_eq!(track.core.k, Some(0));
    println!("SYNTHETIC prefixes={prefixes} partition_comparisons={comparisons} unbalanced_pin_then_target=true");
}
fn main() {
    synthetic();
    if std::env::args().nth(1).as_deref() == Some("--synthetic") {
        return;
    }
    let root = std::env::args().nth(1).expect("fixture root");
    let mut checked = 0;
    let mut recovered = 0;
    for p in [1009u64, 16381] {
        let fix = Fixture::parse(
            &fs::read_to_string(Path::new(&root).join(format!("select-g2-p{p}-f0.out"))).unwrap(),
        );
        let h = (p as f64).sqrt().ceil() as usize;
        for seed in [14, 15, 16] {
            let (expected, target) = fix.target(seed);
            let (r, tracks) = engine::run(
                &fix.c,
                &fix.d1,
                &target,
                fix.ell,
                seed,
                &[0, 1, h],
                true,
                false,
                32768,
            );
            checked += r.raw_checks;
            for track in &tracks {
                assert_eq!(track.core.k, Some(expected));
                assert_eq!(track.core.solved_at, tracks[0].core.solved_at);
            }
            recovered += 1;
            println!("TARGET {{\"p\":{p},\"seed\":{seed},\"trials\":{},\"smooth\":{},\"raw_checks\":{},\"hash\":\"{}\",\"first_target\":{},\"k\":{expected},\"h\":{h},\"zero_forest_bytes\":{},\"positive_forest_bytes\":{},\"positive_rank\":{},\"zero_rank\":{},\"correct\":true}}",r.trials,r.smooth,r.raw_checks,r.hash,tracks[0].core.solved_at,tracks[0].core.forest.bytes(),tracks[2].core.forest.bytes(),tracks[2].core.echelon.rank,tracks[0].core.echelon.rank);
        }
    }
    println!("TERMINAL targets=6 recovered={recovered} raw_prefixes={checked} partition_comparisons={} exact=true",checked*3);
}

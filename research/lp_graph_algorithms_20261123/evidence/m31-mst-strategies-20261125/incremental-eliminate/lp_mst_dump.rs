//! Differential dump: full MST outputs for seeded streams and given files.
use crypto_lib::cryptanalysis::large_prime_filter::{
    filter_large_primes_mst_with, LargePrimeFilterOptions, MstLimits, MstStrategy,
    PartialRelation,
};
use sha2::{Digest, Sha256};
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
fn main() {
    let strategies = [MstStrategy::Exhaustive, MstStrategy::OverlapSensitive, MstStrategy::SharedKey];
    let mut streams: Vec<(String, u64, Vec<PartialRelation>, LargePrimeFilterOptions, MstLimits)> = Vec::new();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for t in 0..4000 {
        let p = [3u64, 7, 101, 1_000_003, 2_147_446_831, (1u64 << 61) - 1][t % 6];
        let rows = 2 + rng.below(120) as usize;
        let lp = 1 + rng.below(40);
        let sc = [4u64, 16, 256, 1 << 20][(t / 6) % 4];
        let hot = t % 5 == 0;
        let src: Vec<PartialRelation> = (0..rows)
            .map(|i| {
                let mut small: Vec<(u32, u64)> = (0..1 + rng.below(5))
                    .map(|_| (rng.below(sc) as u32, 1 + rng.below(p - 1)))
                    .collect();
                if hot {
                    small.push((u32::MAX, 1 + rng.below(3)));
                }
                PartialRelation {
                    id: i as u64,
                    small,
                    large: (0..rng.below(4)).map(|_| (rng.below(lp), 1 + rng.below(p - 1))).collect(),
                    rhs: rng.below(p),
                }
            })
            .collect();
        let tight = t % 4 == 3;
        let o = LargePrimeFilterOptions {
            modulus: p,
            max_small_weight: if tight { 3 + rng.below(12) as usize } else { 4096 },
            max_sources: if tight { 2 + rng.below(10) as usize } else { 4096 },
        };
        let l = MstLimits {
            max_rows: if t % 17 == 0 { rng.below(100) as usize } else { 4096 },
            max_column_degree: if t % 13 == 0 { 2 + rng.below(10) as usize } else { 64 },
            max_edge_evaluations: if t % 11 == 0 { rng.below(2000) as usize } else { 1 << 22 },
            max_active_coefficients: if t % 7 == 0 { 50 + rng.below(800) as usize } else { 1 << 22 },
        };
        streams.push((format!("seed-{t}"), p, src, o, l));
    }
    for path in std::env::args().skip(1) {
        let src: Vec<PartialRelation> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let p = 2_147_446_831;
        let o = LargePrimeFilterOptions { modulus: p, ..Default::default() };
        streams.push((path, p, src, o, MstLimits::default()));
    }
    let mut all = Sha256::new();
    let mut refused = 0;
    for (name, _p, src, o, l) in &streams {
        for s in strategies {
            let out = match filter_large_primes_mst_with(src.clone(), o, l, s) {
                Ok(a) => {
                    refused += usize::from(a.relations.is_none());
                    serde_json::json!({"rel": a.relations, "report": a.report, "stats": a.stats})
                }
                Err(e) => serde_json::json!({"err": format!("{e:?}")}),
            };
            let line = format!("{name} {s:?} {}\n", serde_json::to_string(&out).unwrap());
            all.update(line.as_bytes());
            print!("{line}");
        }
    }
    eprintln!("streams {} refused-arms {refused} digest {:x}", streams.len(), all.finalize());
}

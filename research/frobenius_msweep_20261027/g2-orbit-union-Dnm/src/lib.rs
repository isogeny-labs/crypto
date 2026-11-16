//! Cell G2: per-slot-shift orbit-union bases priced against the exact class
//! count D(n,m) = C(n+m-1,m)/n. Std-only; exact enumeration on toy Koblitz
//! curves E_a: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n}, n an odd prime.
//! Design: ../PROTOCOL.md. Status: prepared, not run.

pub mod combi;
pub mod curve;
pub mod enumr;
pub mod family;
pub mod field;
pub mod plan;
pub mod report;
pub mod runner;
pub mod stats;

/// splitmix64-based generator (same constants as research/twist_obstruction_20261026).
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
    /// Poisson(lam): Knuth for lam <= 30; above, the normal approximation WITHOUT rounding
    /// (the class counts here are real-valued fractional sums, and the variance stays lam).
    pub fn poisson(&mut self, lam: f64) -> f64 {
        if lam <= 0.0 {
            return 0.0;
        }
        if lam > 30.0 {
            let (u1, u2) = (self.unit().max(1e-12), self.unit());
            let g = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            return lam + lam.sqrt() * g;
        }
        let (l, mut k, mut p) = ((-lam).exp(), 0.0, 1.0);
        loop {
            p *= self.unit();
            if p <= l {
                return k;
            }
            k += 1.0;
        }
    }
    pub fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Derive an independent seed from a base seed and a list of labels.
pub fn derive_seed(base: u64, labels: &[u64]) -> u64 {
    let mut r = Rng(base ^ 0xD1B54A32D192ED03);
    let mut s = r.next();
    for &l in labels {
        r = Rng(s ^ l.wrapping_mul(0x9E3779B97F4A7C15));
        s = r.next();
    }
    s
}

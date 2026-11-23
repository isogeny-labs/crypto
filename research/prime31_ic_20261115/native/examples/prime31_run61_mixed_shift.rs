//! Prime31 run61: *public shift only* mixed-Jacobian/affine ablation against
//! the ORIGINAL Rust `Arithmetic::add`. Not an index-calculus extractor or DLP.
//! Reproduce: cargo run --release --example prime31_run61_mixed_shift
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use std::hint::black_box;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const ITER: u64 = 250_000;
const REPS: usize = 5;
const OLD_BSGS_MS: f64 = 170.113;
const MODEL_QUERIES: f64 = 4_699_949.0;
#[derive(Copy, Clone, Debug)]
struct Jac { x: u64, y: u64, z: u64 }

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 { a * b % P }
#[inline(always)]
fn sub(a: u64, b: u64) -> u64 { (a + P - b) % P }
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 { r = mul(r, a); }
        a = mul(a, a);
        n >>= 1;
    }
    r
}
impl Jac {
    fn new(q: (u64, u64)) -> Self { Self { x: q.0, y: q.1, z: 1 } }
    fn affine(&self) -> (u64, u64) {
        assert_ne!(self.z, 0, "infinity requires separate handling");
        let iz = pow(self.z, P - 2);
        let iz2 = mul(iz, iz);
        (mul(self.x, iz2), mul(self.y, mul(iz2, iz)))
    }
    /// Existing standard mixed Jacobian/affine group-law formula, specialized
    /// to a fixed affine G. Caller excludes exceptional H=0; fail closed.
    #[inline(always)]
    fn add_fixed(&mut self, g: (u64,u64)) {
        let zz = mul(self.z, self.z);
        let u = mul(g.0, zz);
        let s = mul(g.1, mul(self.z, zz));
        let h = sub(u, self.x);
        let r = sub(s, self.y);
        assert_ne!(h, 0, "doubling/infinity exceptional branch not covered");
        let hh = mul(h, h);
        let hhh = mul(h, hh);
        let v = mul(self.x, hh);
        let nx = sub(sub(mul(r, r), hhh), (2 * v) % P);
        let ny = sub(mul(r, sub(v, nx)), mul(self.y, hhh));
        self.z = mul(self.z, h);
        self.x = nx;
        self.y = ny;
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(PRIME31.p, P);
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let g = Some(PRIME31.g);
    let mut a = ar.scalar(PRIME31, 2, g).expect("2G was infinity");
    let mut j = Jac::new(a);
    // Exact boundary against the unchanged original affine backend, outside
    // both timed loops; includes all 1024 early powers and conversion.
    for _ in 0..1024 {
        j.add_fixed(PRIME31.g);
        a = ar.add(PRIME31, Some(a), g).expect("affine prefix infinity");
        assert_eq!(j.affine(), a, "Jacobian prefix mismatch");
    }
    assert_eq!(Some(a), ar.scalar(PRIME31, 1026, g));
    let mut projective = Vec::with_capacity(REPS);
    let mut affine = Vec::with_capacity(REPS);
    let mut digest = 0u64;
    let mut cumulative = 1026_u64;
    for rep in 0..REPS {
        // Same start point for each arm. Keeping both public point chains
        // independent avoids a secret scalar or a planted target as input.
        let start = Instant::now();
        for _ in 0..ITER {
            j.add_fixed(PRIME31.g);
            digest ^= j.x ^ j.y ^ j.z;
            black_box(j);
        }
        let proj_ns = start.elapsed().as_nanos() as f64 / ITER as f64;

        let before = ar.counts.group_additions;
        let start = Instant::now();
        for _ in 0..ITER {
            a = ar.add(PRIME31, Some(a), g).expect("unexpected infinity");
            digest ^= a.0 ^ a.1;
            black_box(a);
        }
        let affine_ns = start.elapsed().as_nanos() as f64 / ITER as f64;
        assert_eq!(ar.counts.group_additions - before, ITER);
        assert_eq!(j.affine(), a, "Jacobian and original affine outputs differ");
        cumulative += ITER;
        assert_eq!(Some(a), ar.scalar(PRIME31, cumulative, g),
                   "original independent scalar cross-check failed");
        println!("run61 rep={} mixed_jacobian_ns={:.3} original_affine_ns={:.3}",
                 rep, proj_ns, affine_ns);
        projective.push(proj_ns);
        affine.push(affine_ns);
    }
    black_box(digest);
    projective.sort_by(|a,b| a.total_cmp(b));
    affine.sort_by(|a,b| a.total_cmp(b));
    let m_proj = projective[REPS/2];
    let m_affine = affine[REPS/2];
    let threshold = OLD_BSGS_MS * 1e6 / MODEL_QUERIES;
    println!("run61 Prime31 p={} n={} mixed_jacobian_ns={:.3} original_affine_ns={:.3} \
              speedup_affine_over_mixed={:.3} cross_run_sensitivity_ns={:.3} \
              mixed_1024_shift_only_ms={:.3} digest={} \
              classification=public_shift_primitive_only_no_blind_algebraic_extraction",
             PRIME31.p, PRIME31.n, m_proj, m_affine, m_affine/m_proj,
             threshold, m_proj*MODEL_QUERIES/1e6, digest);
    Ok(())
}

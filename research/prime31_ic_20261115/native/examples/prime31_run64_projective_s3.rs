//! Prime31 run64: projective Semaev S3 zero tests versus batched affine-X S3.
//! Reuses the original Prime31 arithmetic, public depth-10 factor base, and
//! the fixed-generator Jacobian/batch-normalization design from run63.
//! This only times K fixed, public S3 polynomials; NOT blind root extraction.
//! cargo run --locked --release --example prime31_run64_projective_s3
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use std::hint::black_box;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const G: (u64, u64) = (625_037_715, 2_100_627_251);
const ITER: usize = 250_000;
const REPS: usize = 5;
const BATCH: usize = 256;
const K_VALUES: [usize; 3] = [1, 4, 16];

#[inline(always)] fn add(a: u64, b: u64) -> u64 {
    let t = a + b; if t >= P { t - P } else { t }
}
#[inline(always)] fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
#[inline(always)] fn mul(a: u64, b: u64) -> u64 {
    debug_assert!(a < P && b < P);
    let t = a * b;
    let s = (t & P) + (t >> 31);
    if s >= P { s - P } else { s }
}
fn pow(mut x: u64, mut e: u64) -> u64 {
    let mut v = 1;
    while e > 0 {
        if e & 1 != 0 { v = mul(v, x); }
        x = mul(x, x);
        e >>= 1;
    }
    v
}

#[derive(Clone, Copy, Debug)]
struct J { x: u64, y: u64, z: u64 }
impl J {
    fn affine(p: (u64, u64)) -> Self { Self { x: p.0, y: p.1, z: 1 } }
    fn double(self) -> Option<Self> {
        if self.y == 0 { return None; }
        let xx = mul(self.x, self.x);
        let yy = mul(self.y, self.y);
        let yyyy = mul(yy, yy);
        let z2 = mul(self.z, self.z);
        let v = mul(add(self.x, yy), add(self.x, yy));
        let d = add(sub(sub(v, xx), yyyy), sub(sub(v, xx), yyyy));
        let m = add(add(xx, add(xx, xx)), mul(A, mul(z2, z2)));
        let nx = sub(mul(m, m), add(d, d));
        let ny = sub(mul(m, sub(d, nx)), mul(8, yyyy));
        let nz = mul(add(self.y, self.y), self.z);
        Some(Self { x: nx, y: ny, z: nz })
    }
    fn mixed(self, g: (u64, u64)) -> Option<Self> {
        let zz = mul(self.z, self.z);
        let u = mul(g.0, zz);
        let s = mul(g.1, mul(self.z, zz));
        let h = sub(u, self.x);
        let r = sub(s, self.y);
        if h == 0 { return if r == 0 { self.double() } else { None }; }
        let hh = mul(h, h);
        let hhh = mul(h, hh);
        let v = mul(self.x, hh);
        let nx = sub(sub(mul(r, r), hhh), add(v, v));
        let ny = sub(mul(r, sub(v, nx)), mul(self.y, hhh));
        Some(Self { x: nx, y: ny, z: mul(self.z, h) })
    }
    fn xy(self) -> (u64, u64) {
        assert_ne!(self.z, 0);
        let iz = pow(self.z, P - 2);
        let iz2 = mul(iz, iz);
        (mul(self.x, iz2), mul(self.y, mul(iz2, iz)))
    }
}
#[inline(always)] fn shift(q: Option<J>) -> Option<J> {
    match q { None => Some(J::affine(G)), Some(p) => p.mixed(G) }
}

#[derive(Clone, Copy)]
struct S3 { a: u64, b: u64, c: u64 }
impl S3 {
    fn from_x(x: u64, y: u64) -> Self {
        let xy = mul(x, y);
        let s = add(x, y);
        let c1 = add(mul(s, add(xy, A)), mul(2, B));
        let d = sub(xy, A);
        Self {
            a: mul(sub(x, y), sub(x, y)),
            b: sub(0, mul(2, c1)),
            c: sub(mul(d, d), mul(mul(4, B), s)),
        }
    }
    #[inline(always)] fn affine(self, x: u64) -> u64 {
        add(mul(add(mul(self.a, x), self.b), x), self.c)
    }
    // x = X/Z^2 => Z^4 S3(x) = a X^2 + b X Z^2 + c Z^4.
    #[inline(always)] fn projective_precomputed(self, x2: u64, xz2: u64, z4: u64) -> u64 {
        add(add(mul(self.a, x2), mul(self.b, xz2)), mul(self.c, z4))
    }
}
#[inline(always)] fn eval_projective(j: J, coeffs: &[S3]) -> u64 {
    let z2 = mul(j.z, j.z);
    let z4 = mul(z2, z2);
    let x2 = mul(j.x, j.x);
    let xz2 = mul(j.x, z2);
    let mut digest = 0u64;
    for &c in coeffs {
        digest ^= c.projective_precomputed(x2, xz2, z4);
    }
    digest
}
// Original run63 Montgomery inverse path, one inversion per 256 points.
// Affine x only, since an x-coordinate summation polynomial does not need y.
fn normalize_x(chunk: &[Option<J>]) -> Vec<Option<u64>> {
    let mut prefix = Vec::with_capacity(chunk.len());
    let mut product = 1u64;
    for &q in chunk {
        prefix.push(product);
        if let Some(j) = q {
            assert_ne!(j.z, 0);
            product = mul(product, j.z);
        }
    }
    let mut inv = pow(product, P - 2);
    let mut out = vec![None; chunk.len()];
    for i in (0..chunk.len()).rev() {
        if let Some(j) = chunk[i] {
            let zi = mul(prefix[i], inv);
            inv = mul(inv, j.z);
            out[i] = Some(mul(j.x, mul(zi, zi)));
        }
    }
    assert_eq!(inv, 1);
    out
}
fn verify(ar: &mut Arithmetic, coeffs: &[S3], points: &[(u64, u64)]) {
    assert_eq!(coeffs.len(), 16);
    let mut seed = 0x64_2026_1122_4242u64;
    let boundaries = [0, 1, 2, 3, PRIME31.n - 1, PRIME31.n - 2, PRIME31.n / 2, 1000];
    let mut checked = 0usize;
    for case in 0..40 {
        let k = if case < boundaries.len() { boundaries[case] }
        else {
            seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17;
            seed % PRIME31.n
        };
        let mut q = ar.scalar(PRIME31, k, Some(G));
        let mut j = q.map(J::affine);
        for step in 0..192 {
            q = ar.add(PRIME31, q, Some(G));
            j = shift(j);
            assert_eq!(j.map(J::xy), q, "group-law case={case} step={step}");
            if let Some(pj) = j {
                let x = q.unwrap().0;
                let z2 = mul(pj.z, pj.z);
                let z4 = mul(z2, z2);
                let x2 = mul(pj.x, pj.x);
                let xz2 = mul(pj.x, z2);
                for &c in coeffs {
                    assert_eq!(
                        c.projective_precomputed(x2, xz2, z4),
                        mul(z4, c.affine(x)),
                        "homogeneous chart case={case} step={step}"
                    );
                    checked += 1;
                }
            }
        }
    }
    let mut positive = 0;
    for i in 0..16 {
        let p = points[2 * i];
        let q = points[2 * i + 1];
        let r = ar.add(PRIME31, Some(p), Some(q));
        if let Some(r) = r {
            assert_eq!(coeffs[i].affine(r.0), 0, "sum S3 i={i}");
            let negative = ar.neg(PRIME31, Some(r));
            assert_eq!(coeffs[i].affine(negative.unwrap().0), 0, "negated sum i={i}");
            positive += 1;
        }
        let doubled = ar.add(PRIME31, Some(p), Some(p));
        if let Some(d) = doubled {
            assert_eq!(S3::from_x(p.0, p.0).affine(d.0), 0, "doubling i={i}");
        }
    }
    assert_eq!(positive, 16);
    println!("run64 group_law=PASS 7680_steps homogeneous_identities=PASS {checked} positive_sums=PASS {positive} doubling=PASS 16");
}
fn median(values: &[f64]) -> f64 {
    let mut xs = values.to_vec();
    xs.sort_by(|a, b| a.total_cmp(b));
    xs[xs.len() / 2]
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!((PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.g), (P, A, B, G));
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let base = Base::build(PRIME31, 10, &mut ar)?;
    assert_eq!(base.points.len(), 509);
    let points = &base.points[..32];
    let coeffs: Vec<S3> = (0..16).map(|i| S3::from_x(points[2*i].0, points[2*i+1].0)).collect();
    verify(&mut ar, &coeffs, points);
    let mut times: [Vec<f64>; 7] = std::array::from_fn(|_| Vec::new());
    let mut digests = [0u64; 7];
    for rep in 0..REPS {
        let k = 1000 + rep as u64 * 400_001;
        let initial = ar.scalar(PRIME31, k, Some(G)).unwrap();
        let expected = ar.scalar(PRIME31, k + ITER as u64, Some(G));
        for offset in 0..7 {
            let arm = (rep * 3 + offset) % 7;
            let mut j = Some(J::affine(initial));
            let mut digest = 0u64;
            let start = Instant::now();
            if arm == 0 {
                for _ in 0..ITER {
                    j = shift(j);
                    if let Some(q) = j { digest ^= q.x ^ q.y ^ q.z; }
                    black_box(j);
                }
            } else if arm <= 3 {
                let count = K_VALUES[arm - 1];
                for _ in 0..ITER {
                    j = shift(j);
                    if let Some(q) = j {
                        digest ^= eval_projective(q, &coeffs[..count]);
                    }
                    black_box(j);
                }
            } else {
                let count = K_VALUES[arm - 4];
                let mut done = 0usize;
                while done < ITER {
                    let size = BATCH.min(ITER - done);
                    let mut chunk = Vec::with_capacity(size);
                    for _ in 0..size {
                        j = shift(j);
                        chunk.push(j);
                        black_box(j);
                    }
                    for x in normalize_x(&chunk).into_iter().flatten() {
                        for &c in &coeffs[..count] {
                            digest ^= c.affine(x);
                        }
                        black_box(x);
                    }
                    done += size;
                }
            }
            let ns = start.elapsed().as_nanos() as f64 / ITER as f64;
            assert_eq!(j.map(J::xy), expected, "benchmark endpoint arm={arm} rep={rep}");
            times[arm].push(ns);
            digests[arm] ^= digest;
            println!("run64 rep={rep} arm={arm} ns_per_probe={ns:.4} endpoint=PASS");
        }
    }
    for arm in 0..7 {
        println!("run64 summary arm={arm} median_ns={:.4} raw_ns={:?} digest={}",
                 median(&times[arm]), times[arm], digests[arm]);
    }
    for (idx, &k) in K_VALUES.iter().enumerate() {
        let hp = median(&times[idx+1]);
        let ax = median(&times[idx+4]);
        println!("run64 comparison pairs={k} homogeneous_ns={hp:.4} affine_batch_ns={ax:.4} hom_over_affine={:.5}", hp/ax);
    }
    println!("run64 classification=fixed_pair_s3_component_not_blind_extraction_not_complete_ECDLP");
    black_box(digests);
    Ok(())
}

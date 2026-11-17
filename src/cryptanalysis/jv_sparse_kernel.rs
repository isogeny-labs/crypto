//! Exact sparse homogeneous kernels, with batch accounting for deterministic loops.
//! Lanczos: Eberly--Kaltofen (1997), Figures 1--4, with full randomization.
use crate::cryptanalysis::{
    gaudry_cubic::{wiedemann_u64, SparseRel},
    jv_bitops as work,
    jv_sieve::two_large_prime::CountedRelationSystem,
    residual_walk::{inv_mod, is_prime_u64},
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Matrix {
    pub modulus: u64,
    pub columns: usize,
    pub rows: Vec<Vec<(usize, u64)>>,
}
impl Matrix {
    pub fn validate(&self) -> Result<(), String> {
        if !(3..1u64 << 63).contains(&self.modulus)
            || !is_prime_u64(self.modulus)
            || !(2..=4096).contains(&self.columns)
            || self.rows.len() < self.columns - 1
            || self.rows.len() > 8192
            || self.rows.iter().map(Vec::len).sum::<usize>() > 131072
        {
            return Err("matrix resource/prime domain".into());
        }
        if self.rows.iter().any(|r| {
            r.windows(2).any(|w| w[0].0 >= w[1].0)
                || r.iter()
                    .any(|&(j, a)| j >= self.columns || a == 0 || a >= self.modulus)
        }) {
            return Err("canonical sparse rows".into());
        }
        Ok(())
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Counts {
    pub products: u64,
    pub additions: u64,
    pub inversions: u64,
    pub words: u64,
    pub fixed_setups: u64,
    pub draws: u64,
}
pub struct Arithmetic {
    pub l: u64,
    pub counts: Counts,
}
impl Arithmetic {
    fn batch(&mut self, products: usize, additions: usize, words: usize) {
        self.counts.products += products as u64;
        self.counts.additions += additions as u64;
        work::hook_muls(self.l, products as u64);
        work::hook_adds(self.l, additions as u64);
        self.words(words);
    }
    pub fn new(l: u64) -> Self {
        Self {
            l,
            counts: Counts::default(),
        }
    }
    pub fn words(&mut self, n: usize) {
        self.counts.words += n as u64;
        work::words(n as u64);
    }
    pub fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.counts.additions += 1;
        work::hook_add(self.l);
        if a >= b {
            a - b
        } else {
            a + (self.l - b)
        }
    }
    pub fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.counts.products += 1;
        work::hook_mul(self.l);
        (a as u128 * b as u128 % self.l as u128) as u64
    }
    pub fn inv(&mut self, a: u64) -> Result<u64, String> {
        if a == 0 {
            return Err("zero inverse refused".into());
        }
        self.counts.inversions += 1;
        Ok(inv_mod(a, self.l))
    }
    pub fn draw(&mut self, r: &mut StdRng, nonzero: bool) -> u64 {
        self.counts.draws += 1;
        work::uniform_mod_u64(r, u64::from(nonzero), self.l)
    }
    pub fn dot(&mut self, a: &[u64], b: &[u64]) -> u64 {
        let mut out = 0;
        for (&x, &y) in a.iter().zip(b) {
            let term = (x as u128 * y as u128 % self.l as u128) as u64;
            out = add_raw(out, term, self.l);
        }
        let terms = a.len().min(b.len());
        self.batch(terms, terms, 2 * a.len());
        out
    }
}
/// Same fixed-factor reduction as the existing counted IC RREF backend.
#[derive(Clone, Copy)]
struct Fixed {
    w: u64,
    q: u64,
}
impl Fixed {
    fn new(w: u64, c: &mut Arithmetic) -> Self {
        c.counts.fixed_setups += 1;
        c.words(2);
        Self {
            w,
            q: (((w as u128) << 64) / c.l as u128) as u64,
        }
    }
    fn mul(self, x: u64, c: &mut Arithmetic) -> u64 {
        c.counts.products += 1;
        work::hook_mul(c.l);
        c.words(4);
        self.raw(x, c.l)
    }
    #[inline]
    fn raw(self, x: u64, l: u64) -> u64 {
        let q = ((self.q as u128 * x as u128) >> 64) as u64;
        let r = self.w.wrapping_mul(x).wrapping_sub(q.wrapping_mul(l));
        if r >= l {
            r - l
        } else {
            r
        }
    }
}
#[inline]
fn add_raw(a: u64, b: u64, l: u64) -> u64 {
    let s = a + b;
    if s >= l {
        s - l
    } else {
        s
    }
}
#[inline]
fn sub_raw(a: u64, b: u64, l: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + (l - b)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    pub diagonal: u64,
    pub alpha: u64,
    pub beta: u64,
    pub projection: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attempt {
    pub index: usize,
    pub iterations: usize,
    pub refusal: Option<String>,
    pub steps: Vec<Step>,
    pub row_weights: Vec<u64>,
    pub column_weights: Vec<u64>,
    pub rhs_shift: Vec<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResultRow {
    pub method: String,
    pub kernel: Option<Vec<u64>>,
    pub rank_certificate: Option<usize>,
    pub counts: Counts,
    pub legacy_products: u64,
    pub attempts: Vec<Attempt>,
    pub refusal: Option<String>,
}
fn result(method: &str) -> ResultRow {
    ResultRow {
        method: method.into(),
        kernel: None,
        rank_certificate: None,
        counts: Counts::default(),
        legacy_products: 0,
        attempts: vec![],
        refusal: None,
    }
}
fn verify(matrix: &Matrix, h: &[u64], c: &mut Arithmetic) -> bool {
    if h.len() != matrix.columns || h.last() != Some(&1) || h.iter().any(|&v| v >= matrix.modulus) {
        return false;
    }
    for row in &matrix.rows {
        let mut sum = 0;
        for &(i, a) in row {
            let v = (a as u128 * h[i] as u128 % matrix.modulus as u128) as u64;
            sum = add_raw(sum, v, matrix.modulus);
        }
        c.batch(row.len(), row.len(), 2 * row.len());
        if sum != 0 {
            return false;
        }
    }
    true
}
pub fn counted_rref(matrix: &Matrix) -> Result<ResultRow, String> {
    matrix.validate()?;
    let mut out = result("counted-rref");
    let mut c = Arithmetic::new(matrix.modulus);
    let mut system = CountedRelationSystem::new(matrix.modulus, matrix.columns);
    for row in &matrix.rows {
        let mut dense = vec![0; matrix.columns];
        c.words(matrix.columns + 2 * row.len());
        for &(i, a) in row {
            dense[i] = a;
        }
        system.insert(dense, 0);
    }
    let rank = system.rank();
    let mut normal = vec![0; matrix.columns];
    normal[matrix.columns - 1] = 1;
    c.words(matrix.columns);
    system.insert(normal, 1);
    out.legacy_products = system.ops();
    let h = (0..matrix.columns)
        .map(|j| system.solved(j))
        .collect::<Option<Vec<_>>>();
    if rank + 1 == matrix.columns {
        if let Some(h) = h {
            if verify(matrix, &h, &mut c) {
                out.kernel = Some(h);
                out.rank_certificate = Some(rank);
            }
        }
    }
    if out.kernel.is_none() {
        out.refusal = Some("normalized unique kernel not established".into());
    }
    out.counts = c.counts;
    Ok(out)
}
/// Existing production Wiedemann, square-bordered with random dummy columns.
/// Dummy values must be zero and every original row is checked afterward.
pub fn library_wiedemann(
    matrix: &Matrix,
    seed: u64,
    max_attempts: usize,
) -> Result<ResultRow, String> {
    matrix.validate()?;
    let mut out = result("library-wiedemann");
    let mut c = Arithmetic::new(matrix.modulus);
    let n = matrix.columns - 1;
    let m = matrix.rows.len();
    for attempt in 0..max_attempts {
        let mut rng =
            StdRng::seed_from_u64(seed ^ (attempt as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let mut rows = Vec::with_capacity(m);
        c.words(m);
        for row in &matrix.rows {
            let mut cols = Vec::new();
            let mut rhs = 0;
            for &(j, a) in row {
                c.words(2);
                if j == n {
                    rhs = c.sub(0, a);
                } else {
                    cols.push((j, a));
                }
            }
            for j in n..m {
                cols.push((j, c.draw(&mut rng, false)));
                c.words(2);
            }
            rows.push(SparseRel { cols, rhs });
        }
        let mut ops = 0;
        let answer = wiedemann_u64(&rows, m, matrix.modulus, &mut rng, &mut ops);
        out.legacy_products += ops;
        let mut record = Attempt {
            index: attempt,
            iterations: 0,
            refusal: None,
            steps: vec![],
            row_weights: vec![],
            column_weights: vec![],
            rhs_shift: vec![],
        };
        if let Some(x) = answer {
            c.words(x.len());
            if x[n..].iter().all(|&a| a == 0) {
                let mut h = x[..n].to_vec();
                h.push(1);
                if verify(matrix, &h, &mut c) {
                    out.kernel = Some(h);
                    out.attempts.push(record);
                    break;
                }
            }
            record.refusal = Some("dummy/original-row verification refused".into());
        } else {
            record.refusal = Some("library bounded projections refused".into());
        }
        out.attempts.push(record);
    }
    if out.kernel.is_none() {
        out.refusal = Some("border/projection attempt cap".into());
    }
    out.counts = c.counts;
    Ok(out)
}
struct Normal {
    rows: Vec<Vec<(usize, Fixed)>>,
    weights: Vec<Fixed>,
    columns: usize,
    nonzeros: usize,
}
impl Normal {
    fn transpose(&self, v: &[u64], c: &mut Arithmetic) -> Vec<u64> {
        let mut out = vec![0; self.columns];
        for (row, &v) in self.rows.iter().zip(v) {
            for &(j, a) in row {
                let z = a.raw(v, c.l);
                out[j] = add_raw(out[j], z, c.l);
            }
        }
        c.batch(
            self.nonzeros,
            self.nonzeros,
            self.columns + 4 * self.nonzeros,
        );
        out
    }
    fn apply(&self, v: &[u64], c: &mut Arithmetic) -> Vec<u64> {
        let mut tmp = vec![0; self.rows.len()];
        for (i, row) in self.rows.iter().enumerate() {
            for &(j, a) in row {
                let z = a.raw(v[j], c.l);
                tmp[i] = add_raw(tmp[i], z, c.l);
            }
            tmp[i] = self.weights[i].raw(tmp[i], c.l);
        }
        c.batch(
            self.nonzeros + self.rows.len(),
            self.nonzeros,
            5 * self.rows.len() + 4 * self.nonzeros,
        );
        self.transpose(&tmp, c)
    }
}
pub fn randomized_lanczos(
    matrix: &Matrix,
    seed: u64,
    max_attempts: usize,
) -> Result<ResultRow, String> {
    matrix.validate()?;
    let mut out = result("randomized-lanczos");
    let mut c = Arithmetic::new(matrix.modulus);
    let n = matrix.columns - 1;
    for attempt in 0..max_attempts {
        let mut rng =
            StdRng::seed_from_u64(seed ^ (attempt as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let rw: Vec<_> = (0..matrix.rows.len())
            .map(|_| c.draw(&mut rng, true))
            .collect();
        let cw: Vec<_> = (0..n).map(|_| c.draw(&mut rng, true)).collect();
        let shift: Vec<_> = (0..n).map(|_| c.draw(&mut rng, false)).collect();
        let weights = rw
            .iter()
            .map(|&w| Fixed::new(w, &mut c))
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        let mut weighted_rhs = Vec::new();
        for (i, row) in matrix.rows.iter().enumerate() {
            let mut r = Vec::new();
            let mut rhs = 0;
            for &(j, a) in row {
                c.words(2);
                if j == n {
                    rhs = c.sub(0, a);
                } else {
                    let a = c.mul(a, cw[j]);
                    r.push((j, Fixed::new(a, &mut c)));
                }
            }
            weighted_rhs.push(weights[i].mul(rhs, &mut c));
            rows.push(r);
        }
        let nonzeros = rows.iter().map(Vec::len).sum();
        let op = Normal {
            rows,
            weights,
            columns: n,
            nonzeros,
        };
        let mut b = op.transpose(&weighted_rhs, &mut c);
        let shifted = op.apply(&shift, &mut c);
        for (i, v) in b.iter_mut().enumerate() {
            *v = add_raw(*v, shifted[i], c.l);
        }
        c.batch(0, n, 0);
        let mut w = b.clone();
        let mut prev = vec![0; n];
        let mut prev_image = vec![0; n];
        let mut prev_inverse = 1;
        let mut x = vec![0; n];
        c.words(5 * n);
        let mut record = Attempt {
            index: attempt,
            iterations: 0,
            refusal: None,
            steps: vec![],
            row_weights: rw,
            column_weights: cw.clone(),
            rhs_shift: shift.clone(),
        };
        for i in 0..n {
            c.words(n);
            if w.iter().all(|&a| a == 0) {
                record.refusal = Some("early Krylov stop lacks full-rank certificate".into());
                break;
            }
            let image = op.apply(&w, &mut c);
            let d = c.dot(&w, &image);
            if d == 0 {
                record.refusal = Some("self-orthogonal Lanczos direction".into());
                break;
            }
            let inverse = c.inv(d)?;
            let projection0 = c.dot(&b, &w);
            let projection = c.mul(projection0, inverse);
            let projection_fixed = Fixed::new(projection, &mut c);
            for j in 0..n {
                let z = projection_fixed.raw(w[j], c.l);
                x[j] = add_raw(x[j], z, c.l);
            }
            c.batch(n, n, 4 * n);
            let alpha0 = c.dot(&image, &image);
            let alpha = c.mul(alpha0, inverse);
            let beta = if i == 0 {
                0
            } else {
                let beta0 = c.dot(&image, &prev_image);
                c.mul(beta0, prev_inverse)
            };
            let af = Fixed::new(alpha, &mut c);
            let bf = Fixed::new(beta, &mut c);
            let mut next = Vec::with_capacity(n);
            for j in 0..n {
                let a = af.raw(w[j], c.l);
                let z = sub_raw(image[j], a, c.l);
                let b = bf.raw(prev[j], c.l);
                next.push(sub_raw(z, b, c.l));
            }
            c.batch(2 * n, 2 * n, 8 * n);
            record.steps.push(Step {
                diagonal: d,
                alpha,
                beta,
                projection,
            });
            record.iterations = i + 1;
            if i + 1 == n {
                c.words(n);
                if next.iter().any(|&a| a != 0) {
                    record.refusal = Some("Krylov basis did not close at dimension".into());
                    break;
                }
                let mut h = Vec::with_capacity(n + 1);
                for j in 0..n {
                    let z = sub_raw(x[j], shift[j], c.l);
                    h.push((cw[j] as u128 * z as u128 % c.l as u128) as u64);
                }
                c.batch(n, n, 0);
                h.push(1);
                if verify(matrix, &h, &mut c) {
                    out.kernel = Some(h);
                    out.rank_certificate = Some(n);
                } else {
                    record.refusal = Some("original-row residual check refused".into());
                }
                break;
            }
            prev = w;
            prev_image = image;
            prev_inverse = inverse;
            w = next;
        }
        out.attempts.push(record);
        if out.kernel.is_some() {
            break;
        }
    }
    if out.kernel.is_none() {
        out.refusal = Some("registered preconditioner cap".into());
    }
    out.counts = c.counts;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(l: u64, n: usize, extra: usize) -> (Matrix, Vec<u64>) {
        let h = (0..n)
            .map(|i| ((3 * i + 2) as u64) % l)
            .chain([1])
            .collect::<Vec<_>>();
        let mut rows = (0..n)
            .map(|i| vec![(i, 1), (n, (l - h[i]) % l)])
            .collect::<Vec<_>>();
        for k in 0..extra {
            let a = k % n;
            let b = (k + 1) % n;
            let rhs = (l - ((h[a] + h[b]) % l)) % l;
            let mut row = vec![(a, 1), (b, 1), (n, rhs)];
            row.sort_unstable();
            row.retain(|&(_, x)| x != 0);
            rows.push(row);
        }
        (
            Matrix {
                modulus: l,
                columns: n + 1,
                rows,
            },
            h,
        )
    }
    #[test]
    fn all_three_solvers_return_the_exact_normalized_kernel() {
        for (l, n, e) in [(101, 4, 2), (1009, 8, 3), (263807419614139553, 12, 2)] {
            let (m, h) = fixture(l, n, e);
            assert_eq!(counted_rref(&m).unwrap().kernel, Some(h.clone()));
            assert_eq!(
                library_wiedemann(&m, 433, 8).unwrap().kernel,
                Some(h.clone())
            );
            let r = randomized_lanczos(&m, 433, 8).unwrap();
            assert_eq!(r.kernel, Some(h));
            assert_eq!(r.rank_certificate, Some(n));
        }
    }
    #[test]
    fn fixed_coefficients_match_u128_for_large_and_small_moduli() {
        for l in [101, 263807419614139553, (1u64 << 63) - 25] {
            let mut c = Arithmetic::new(l);
            for w in [0, 1, 2, l / 2, l - 1] {
                let f = Fixed::new(w, &mut c);
                for x in [0, 1, 2, l - 1, u64::MAX] {
                    assert_eq!(f.mul(x, &mut c), (w as u128 * x as u128 % l as u128) as u64);
                }
            }
        }
    }
    #[test]
    fn deficient_system_does_not_get_a_false_full_rank_certificate() {
        let m = Matrix {
            modulus: 101,
            columns: 4,
            rows: vec![vec![(0, 1), (1, 1), (2, 1), (3, 100)]; 3],
        };
        let r = randomized_lanczos(&m, 433, 3).unwrap();
        assert!(r.kernel.is_none());
        assert!(r.rank_certificate.is_none());
        assert_eq!(r.attempts.len(), 3);
    }
    #[test]
    fn malformed_sparse_input_and_nonunits_are_refused() {
        let (mut m, _) = fixture(101, 4, 0);
        m.rows[0].insert(0, (0, 2));
        assert!(m.validate().is_err());
        m.modulus = 15;
        assert!(m.validate().is_err());
        let mut c = Arithmetic::new(101);
        assert!(c.inv(0).is_err());
    }
}

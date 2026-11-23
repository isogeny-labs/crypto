//! Block Wiedemann over `F_p` (catalog #4917 §3, §8 item 6).
//!
//! Finds a kernel vector of a square sparse matrix `A` (and solves `A x = b`
//! through a bordered matrix) with `m × n` blocks:
//!
//! 1. random dense projections `X` (`m × N`) and `Y` (`N × n`), `Z = A Y`;
//! 2. the block sequence `a_i = X Aⁱ Z`, `i < L = ⌈N/m⌉ + ⌈N/n⌉ + extra`
//!    (`n · L` sparse products, `m n N L` projection multiplications);
//! 3. a right matrix generator from a σ-basis (order basis) of
//!    `G = [S(x) | −I_m]`, `S = Σ aᵢ xⁱ`, computed by the iterative
//!    Beckermann–Labahn / Giorgi–Jeannerod–Villard algorithm with shifted
//!    degrees (`g`-columns start at shift 1), so every basis column
//!    `[f̃; g]` satisfies `deg f̃ ≤ δ`, `deg g < δ` and therefore
//!    `Σ_k a_{i+k} f_k = 0` for `i ≤ L − 1 − δ`;
//! 4. `w = Σ_k A^k Y f_k` by Horner from the true degree of `f̃` (factors
//!    of `λ` stripped), then `A w`, `A² w`, … until a product vanishes; the returned vector is **verified**
//!    `A w = 0, w ≠ 0` exactly before it is returned.
//!
//! Costs (field operations, an inversion counted as one): products
//! `O((N/m + N/n) · n · Z)`; dense projection `O(N² (1 + n/m) · m)`, i.e. the
//! catalog's `O(N² b)` for `m = n = b`; σ-basis `O((m + n) · m · L²)` with
//! this quadratic iterative algorithm (the catalog's `Õ(N b^{ω−1})` needs a
//! fast polynomial-matrix generator, not implemented). The stats record
//! every count so block-size schedules can be compared on equal terms.
//!
//! Probabilistic: over small fields, or with unlucky projections, no
//! candidate may verify; the caller retries with another seed. A returned
//! vector is always a true kernel vector.

use serde::{Deserialize, Serialize};

#[inline]
fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

#[inline]
fn add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

#[inline]
fn sub(a: u64, b: u64, p: u64) -> u64 {
    add(a, p - b % p, p)
}

fn inv(a: u64, p: u64) -> Option<u64> {
    let (mut r0, mut r1) = (a as i128, p as i128);
    let (mut s0, mut s1) = (1i128, 0i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    (r0 == 1).then(|| s0.rem_euclid(p as i128) as u64)
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, p: u64) -> u64 {
        self.next() % p
    }
}

/// A square sparse matrix over `F_p` in row form.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SparseMatrix {
    pub n: usize,
    /// Per row, `(column, nonzero coefficient < p)`.
    pub rows: Vec<Vec<(usize, u64)>>,
}

impl SparseMatrix {
    /// Reduce coefficients, combine repeated columns, drop zeros.
    pub fn new(n: usize, rows: Vec<Vec<(usize, u64)>>, p: u64) -> Result<Self, BwError> {
        if !valid_modulus(p) {
            return Err(BwError::InvalidModulus);
        }
        if rows.len() != n {
            return Err(BwError::NotSquare {
                rows: rows.len(),
                n,
            });
        }
        let rows = rows
            .into_iter()
            .map(|r| {
                let mut m = std::collections::BTreeMap::<usize, u64>::new();
                for (c, a) in r {
                    let slot = m.entry(c).or_default();
                    *slot = add(*slot, a % p, p);
                }
                m.into_iter().filter(|&(_, a)| a != 0).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if let Some(&(c, _)) = rows.iter().flatten().find(|&&(c, _)| c >= n) {
            return Err(BwError::ColumnOutOfRange { column: c });
        }
        Ok(Self { n, rows })
    }

    pub fn nnz(&self) -> usize {
        self.rows.iter().map(Vec::len).sum()
    }

    pub fn mul_vec(&self, v: &[u64], p: u64) -> Vec<u64> {
        self.rows
            .iter()
            .map(|r| {
                r.iter()
                    .fold(0u64, |acc, &(c, a)| add(acc, mul(a, v[c], p), p))
            })
            .collect()
    }
}

fn valid_modulus(p: u64) -> bool {
    (3..(1u64 << 63)).contains(&p) && p % 2 == 1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BwParams {
    /// Left block size (rows of `X`).
    pub m: usize,
    /// Right block size (columns of `Y`).
    pub n: usize,
    pub seed: u64,
    /// Extra sequence terms beyond `⌈N/m⌉ + ⌈N/n⌉`.
    pub extra: usize,
}

impl Default for BwParams {
    fn default() -> Self {
        Self {
            m: 4,
            n: 4,
            seed: 1,
            extra: 8,
        }
    }
}

/// Operation counts of one run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BwStats {
    pub dimension: usize,
    pub nonzeros: usize,
    pub m: usize,
    pub n: usize,
    /// Sparse matrix–vector products (sequence + Horner + kernel search).
    pub spmv: u64,
    /// Products spent on the sequence alone (`n · L`).
    pub sequence_spmv: u64,
    /// Dense projection multiplications `X · (Aⁱ Z)`.
    pub projection_mults: u64,
    /// Field multiplications in the σ-basis computation.
    pub generator_mults: u64,
    pub sequence_length: usize,
    /// Smallest generator degree among basis columns with `f̃ ≠ 0`.
    pub generator_degree: usize,
    pub candidates_tried: usize,
    /// `j` such that the returned `w = A^j (Σ A^k Y f_k)`.
    pub kernel_power: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BwError {
    InvalidModulus,
    NotSquare {
        rows: usize,
        n: usize,
    },
    ColumnOutOfRange {
        column: usize,
    },
    BadParams,
    DimensionMismatch,
    /// No candidate verified; retry with another seed.
    NoKernelVector,
    /// The bordered system gave only kernel vectors of `A` (`t = 0`).
    NoSolution,
}

/// A nonzero `w` with `A w = 0`, verified exactly before it is returned.
// Column operations index two columns of the same matrix; iterators would alias.
#[allow(clippy::needless_range_loop)]
pub fn kernel_vector(
    a: &SparseMatrix,
    p: u64,
    params: BwParams,
) -> Result<(Vec<u64>, BwStats), BwError> {
    if !valid_modulus(p) {
        return Err(BwError::InvalidModulus);
    }
    let big_n = a.n;
    let (m, n) = (params.m, params.n);
    if m == 0 || n == 0 || big_n == 0 {
        return Err(BwError::BadParams);
    }
    let mut stats = BwStats {
        dimension: big_n,
        nonzeros: a.nnz(),
        m,
        n,
        ..Default::default()
    };
    let mut rng = SplitMix(params.seed ^ 0x5bd1_e995);
    let y: Vec<Vec<u64>> = (0..n)
        .map(|_| (0..big_n).map(|_| rng.below(p)).collect())
        .collect();
    let x: Vec<Vec<u64>> = (0..m)
        .map(|_| (0..big_n).map(|_| rng.below(p)).collect())
        .collect();
    let big_l = big_n.div_ceil(m) + big_n.div_ceil(n) + params.extra;
    stats.sequence_length = big_l;

    // Block sequence a_i[r][c] = x_r · A^i (A y_c).
    let mut v: Vec<Vec<u64>> = y.iter().map(|c| a.mul_vec(c, p)).collect();
    stats.spmv += n as u64;
    let mut seq = vec![vec![vec![0u64; big_l]; n]; m]; // seq[r][c][i]
    for i in 0..big_l {
        for (r, xr) in x.iter().enumerate() {
            for (c, vc) in v.iter().enumerate() {
                seq[r][c][i] = xr
                    .iter()
                    .zip(vc)
                    .fold(0u64, |acc, (&s, &t)| add(acc, mul(s, t, p), p));
            }
        }
        stats.projection_mults += (m * n * big_n) as u64;
        if i + 1 < big_l {
            v = v.iter().map(|c| a.mul_vec(c, p)).collect();
            stats.spmv += n as u64;
        }
    }
    stats.sequence_spmv = stats.spmv;

    // σ-basis of G = [S | -I_m] at order L.
    let s = n + m;
    // poly[j][row] coefficients of column j of the basis (rows 0..n: f̃, n..s: g).
    let mut poly: Vec<Vec<Vec<u64>>> = (0..s)
        .map(|j| (0..s).map(|row| vec![u64::from(row == j)]).collect())
        .collect();
    // res[j][r][i]: coefficient i of row r of G·(column j).
    let mut res: Vec<Vec<Vec<u64>>> = (0..s)
        .map(|j| {
            (0..m)
                .map(|r| {
                    if j < n {
                        seq[r][j].clone()
                    } else {
                        let mut e = vec![0u64; big_l];
                        if r == j - n {
                            e[0] = p - 1;
                        }
                        e
                    }
                })
                .collect()
        })
        .collect();
    let mut delta: Vec<usize> = (0..s).map(|j| usize::from(j >= n)).collect();
    for k in 0..big_l {
        let mut used = vec![false; s];
        for r in 0..m {
            let Some(piv) = (0..s)
                .filter(|&j| !used[j] && res[j][r][k] != 0)
                .min_by_key(|&j| (delta[j], j))
            else {
                continue;
            };
            used[piv] = true;
            let pinv = inv(res[piv][r][k], p).ok_or(BwError::InvalidModulus)?;
            for j in 0..s {
                if used[j] || res[j][r][k] == 0 {
                    continue;
                }
                debug_assert!(delta[piv] <= delta[j]);
                let c = mul(res[j][r][k], pinv, p);
                // column j -= c · column piv (residual from k on; basis polynomials).
                for rr in 0..m {
                    for i in k..big_l {
                        let t = res[piv][rr][i];
                        if t != 0 {
                            res[j][rr][i] = sub(res[j][rr][i], mul(c, t, p), p);
                        }
                    }
                }
                for row in 0..s {
                    let src = poly[piv][row].clone();
                    let dst = &mut poly[j][row];
                    if dst.len() < src.len() {
                        dst.resize(src.len(), 0);
                    }
                    for (d, &t) in dst.iter_mut().zip(&src) {
                        if t != 0 {
                            *d = sub(*d, mul(c, t, p), p);
                        }
                    }
                }
                stats.generator_mults += (m * (big_l - k) + s * (delta[piv] + 1)) as u64;
            }
        }
        // Multiply pivots by x.
        for j in 0..s {
            if used[j] {
                for rr in 0..m {
                    let col = &mut res[j][rr];
                    col.rotate_right(1);
                    col[0] = 0;
                }
                for row in 0..s {
                    poly[j][row].insert(0, 0);
                }
                delta[j] += 1;
            }
        }
    }

    // Candidates: columns with f̃ ≠ 0, smallest degree first.
    let mut order: Vec<usize> = (0..s)
        .filter(|&j| poly[j][..n].iter().any(|q| q.iter().any(|&t| t != 0)))
        .collect();
    order.sort_by_key(|&j| (delta[j], j));
    stats.generator_degree = order.first().map_or(0, |&j| delta[j]);
    for &j in &order {
        stats.candidates_tried += 1;
        // Strip factors of λ: start Horner at the true degree of f̃ (≤ δ), so
        // `w` is not forced into the range of `A`; then step up by powers.
        let d = poly[j][..n]
            .iter()
            .filter_map(|q| q.iter().rposition(|&t| t != 0))
            .max()
            .expect("f̃ ≠ 0");
        // f_k[c] = coefficient x^{d-k} of f̃_c.
        let coef = |c: usize, k: usize| -> u64 { poly[j][c].get(d - k).copied().unwrap_or(0) };
        let y_f = |k: usize| -> Vec<u64> {
            let mut out = vec![0u64; big_n];
            for (c, yc) in y.iter().enumerate() {
                let f = coef(c, k);
                if f != 0 {
                    for (o, &t) in out.iter_mut().zip(yc) {
                        *o = add(*o, mul(f, t, p), p);
                    }
                }
            }
            out
        };
        let mut w = y_f(d);
        for k in (0..d).rev() {
            let aw = a.mul_vec(&w, p);
            stats.spmv += 1;
            let yk = y_f(k);
            w = aw.iter().zip(&yk).map(|(&s, &t)| add(s, t, p)).collect();
        }
        for power in 0..=d + 2 {
            if w.iter().all(|&t| t == 0) {
                break;
            }
            let aw = a.mul_vec(&w, p);
            stats.spmv += 1;
            if aw.iter().all(|&t| t == 0) {
                stats.kernel_power = power;
                return Ok((w, stats));
            }
            w = aw;
        }
    }
    Err(BwError::NoKernelVector)
}

/// Solve `A x = b` through the bordered matrix `[A | −b; 0 | 0]`, whose kernel
/// holds `(x, 1)` exactly when `A x = b`.
///
/// A kernel vector reached only through powers of the matrix lies in its
/// range and has last coordinate 0; that happens when `ker A ∩ range A ≠ 0`
/// (a nilpotent Jordan block, e.g. an empty column inside the range). After
/// `tries` plain attempts the solver therefore preconditions: it solves
/// `(A P) y = b` with a random sparse `P = I + R` (two entries per row) and
/// returns `x = P y`. Every returned `x` is verified `A x = b`, so `P` need not
/// be proven invertible.
pub fn solve(
    a: &SparseMatrix,
    b: &[u64],
    p: u64,
    params: BwParams,
    tries: usize,
) -> Result<(Vec<u64>, BwStats), BwError> {
    if b.len() != a.n {
        return Err(BwError::DimensionMismatch);
    }
    if !valid_modulus(p) {
        return Err(BwError::InvalidModulus);
    }
    let bb: Vec<u64> = b.iter().map(|&t| t % p).collect();
    let mut last_err = BwError::NoKernelVector;
    for t in 0..2 * tries.max(1) {
        let pr = BwParams {
            seed: params.seed.wrapping_add(t as u64 * 0x9e37),
            ..params
        };
        let precondition = t >= tries.max(1);
        let (op, pre) = if precondition {
            let pre = random_preconditioner(a.n, p, pr.seed);
            (product(a, &pre, p), Some(pre))
        } else {
            (a.clone(), None)
        };
        match bordered_solution(&op, &bb, p, pr) {
            Ok((y, mut stats)) => {
                let x = match &pre {
                    Some(m) => {
                        stats.spmv += 1;
                        m.mul_vec(&y, p)
                    }
                    None => y,
                };
                if a.mul_vec(&x, p) == bb {
                    return Ok((x, stats));
                }
                last_err = BwError::NoSolution;
            }
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

fn bordered_solution(
    a: &SparseMatrix,
    b: &[u64],
    p: u64,
    params: BwParams,
) -> Result<(Vec<u64>, BwStats), BwError> {
    let n = a.n;
    let mut rows: Vec<Vec<(usize, u64)>> = a.rows.clone();
    for (i, r) in rows.iter_mut().enumerate() {
        if b[i] != 0 {
            r.push((n, p - b[i]));
        }
    }
    rows.push(Vec::new());
    let bordered = SparseMatrix::new(n + 1, rows, p)?;
    let (w, stats) = kernel_vector(&bordered, p, params)?;
    if w[n] == 0 {
        return Err(BwError::NoSolution);
    }
    let s = inv(w[n], p).ok_or(BwError::InvalidModulus)?;
    Ok((w[..n].iter().map(|&t| mul(t, s, p)).collect(), stats))
}

/// `P = I + R`, `R` with two random entries per row.
fn random_preconditioner(n: usize, p: u64, seed: u64) -> SparseMatrix {
    let mut rng = SplitMix(seed ^ 0x2545_f491_4f6c_dd1d);
    let rows = (0..n)
        .map(|i| {
            let mut r = vec![(i, 1u64)];
            for _ in 0..2 {
                r.push(((rng.next() % n as u64) as usize, 1 + rng.below(p - 1)));
            }
            r
        })
        .collect();
    SparseMatrix::new(n, rows, p).expect("valid preconditioner")
}

/// Sparse product `A P`.
fn product(a: &SparseMatrix, pm: &SparseMatrix, p: u64) -> SparseMatrix {
    let rows = a
        .rows
        .iter()
        .map(|r| {
            r.iter()
                .flat_map(|&(k, x)| pm.rows[k].iter().map(move |&(c, y)| (c, mul(x, y, p))))
                .collect()
        })
        .collect();
    SparseMatrix::new(a.n, rows, p).expect("square product")
}

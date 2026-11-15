//! Exact multivariate arithmetic helpers over `F_p` on top of `algebra::Pol`:
//! the iterated Sylvester resultant `S_{m+1}`, the elementary-symmetric rewrite and the
//! remainder of `X^(2^t)` modulo the monic polynomial with roots `x_0..x_{m-1}`.
//!
//! `[pcc:unverified]` instrument construction, no claim.
use crate::algebra::{s3, Pol};
use crate::util::{addm, mulm, subm};
use std::collections::HashMap;

pub fn add_in(f: &mut Pol, g: &Pol, p: u64) {
    for (e, &c) in &g.t {
        let v = f.t.entry(e.clone()).or_insert(0);
        *v = addm(*v, c, p);
    }
    f.t.retain(|_, c| *c != 0);
}

pub fn neg(f: &Pol, p: u64) -> Pol {
    f.scale(p - 1, p)
}

/// Coefficients of `f` as a polynomial in variable `var` (index = power), trailing zeros trimmed.
pub fn coeffs_in(f: &Pol, var: usize) -> Vec<Pol> {
    let maxk = f.t.keys().map(|e| e[var]).max().unwrap_or(0) as usize;
    let mut out = vec![Pol::zero(f.n); maxk + 1];
    for (e, &c) in &f.t {
        let mut e2 = e.clone();
        let k = e2[var] as usize;
        e2[var] = 0;
        out[k].t.insert(e2, c);
    }
    out
}

/// Exact determinant of a square matrix of optional polynomials (`None` = 0) by the
/// column-subset dynamic programme, no division.
fn det_poly(mat: &[Vec<Option<Pol>>], n_ring: usize, p: u64) -> Pol {
    let n = mat.len();
    let mut dp: HashMap<u32, Pol> = HashMap::new();
    dp.insert(0, Pol::c(n_ring, 1, p));
    for row in mat.iter() {
        let mut next: HashMap<u32, Pol> = HashMap::new();
        for (&mask, val) in &dp {
            for (c, ent) in row.iter().enumerate().take(n) {
                if mask >> c & 1 == 1 {
                    continue;
                }
                let Some(e) = ent else { continue };
                let mut term = val.mul(e, p);
                if (mask >> (c + 1)).count_ones() % 2 == 1 {
                    term = neg(&term, p);
                }
                add_in(next.entry(mask | 1 << c).or_insert_with(|| Pol::zero(n_ring)), &term, p);
            }
        }
        dp = next;
    }
    dp.remove(&((1u32 << n) - 1)).unwrap_or_else(|| Pol::zero(n_ring))
}

/// `Res_var(f, g)` as the Sylvester determinant with the formal degrees `deg_var f`, `deg_var g`
/// (the largest exponent present), rows laid out as `algebra::sylvester` (`dg` rows of `f`
/// coefficients, then `df` rows of `g`). No content or scalar is removed.
pub fn resultant(f: &Pol, g: &Pol, var: usize, p: u64) -> Pol {
    let cf = coeffs_in(f, var);
    let cg = coeffs_in(g, var);
    let (df, dg) = (cf.len() - 1, cg.len() - 1);
    let n = df + dg;
    if n == 0 {
        return Pol::c(f.n, 1, p);
    }
    let mut mat: Vec<Vec<Option<Pol>>> = vec![vec![None; n]; n];
    let nz = |q: &Pol| if q.t.is_empty() { None } else { Some(q.clone()) };
    for i in 0..dg {
        for k in 0..=df {
            mat[i][i + k] = nz(&cf[df - k]);
        }
    }
    for i in 0..df {
        for k in 0..=dg {
            mat[dg + i][i + k] = nz(&cg[dg - k]);
        }
    }
    det_poly(&mat, f.n, p)
}

/// Last argument of a summation polynomial: a constant (`x_R`) or a ring variable.
#[derive(Clone, Copy)]
pub enum Last {
    Const(u64),
    Var(usize),
}

/// Ring of the unreduced summation polynomial: `x_0..x_{m-1}`, wires `m..2m-3`, optional last var `2m`.
pub fn s_ring(m: usize) -> usize {
    2 * m + 1
}

fn s_rec(k: usize, z: &Pol, m: usize, n: usize, a: u64, b: u64, p: u64) -> Pol {
    if k == 3 {
        return s3(&Pol::var(n, 0), &Pol::var(n, 1), z, a, b, p);
    }
    let wire = m + (k - 4);
    let u = Pol::var(n, wire);
    let inner = s_rec(k - 1, &u, m, n, a, b, p);
    let outer = s3(&u, &Pol::var(n, k - 2), z, a, b, p);
    resultant(&inner, &outer, wire, p)
}

/// `S_{m+1}(x_0..x_{m-1}, last)` by eliminating every internal wire with a Sylvester resultant,
/// in the ring of `s_ring(m)` variables.
pub fn summation_full(m: usize, last: Last, a: u64, b: u64, p: u64) -> Pol {
    assert!(m >= 2);
    let n = s_ring(m);
    let z = match last {
        Last::Const(c) => Pol::c(n, c, p),
        Last::Var(i) => Pol::var(n, i),
    };
    if m == 2 {
        return s3(&Pol::var(n, 0), &Pol::var(n, 1), &z, a, b, p);
    }
    s_rec(m + 1, &z, m, n, a, b, p)
}

/// `S_{m+1}(x_0..x_{m-1}, x_R)` in an `m`-variable ring.
pub fn summation(m: usize, x_r: u64, a: u64, b: u64, p: u64) -> Pol {
    let f = summation_full(m, Last::Const(x_r), a, b, p);
    let mut out = Pol::zero(m);
    for (e, &c) in &f.t {
        assert!(e[m..].iter().all(|&k| k == 0), "wire left in S_(m+1)");
        out.t.insert(e[..m].to_vec(), c);
    }
    out
}

fn elem_x(m: usize, k: usize) -> Pol {
    let mut s = Pol::zero(m);
    for mask in 0u32..(1 << m) {
        if mask.count_ones() as usize == k {
            s.t.insert((0..m).map(|i| mask >> i & 1).collect(), 1);
        }
    }
    s
}

/// Rewrite a symmetric `f` in `m` variables as a polynomial in `e_1..e_m` (variable `k-1` is
/// `e_k`): repeatedly subtract the leading (lex) term's `e_1^(a_1-a_2) ... e_m^(a_m)` product.
/// `Err` if a leading exponent vector is not non-increasing (f not symmetric).
pub fn to_elementary(f: &Pol, m: usize, p: u64) -> Result<Pol, String> {
    let ek: Vec<Pol> = (1..=m).map(|k| elem_x(m, k)).collect();
    let mut memo: HashMap<Vec<u32>, Pol> = HashMap::new();
    memo.insert(vec![0; m], Pol::c(m, 1, p));
    fn prod(a: &[u32], ek: &[Pol], memo: &mut HashMap<Vec<u32>, Pol>, m: usize, p: u64) -> Pol {
        if let Some(v) = memo.get(a) {
            return v.clone();
        }
        let k = a.iter().position(|&x| x > 0).expect("nonzero");
        let mut sub = a.to_vec();
        sub[k] -= 1;
        let r = prod(&sub, ek, memo, m, p).mul(&ek[k], p);
        let _ = m;
        memo.insert(a.to_vec(), r.clone());
        r
    }
    let mut rest = f.clone();
    let mut out = Pol::zero(m);
    while !rest.t.is_empty() {
        let (lead, &c) = rest.t.iter().max_by(|x, y| x.0.cmp(y.0)).expect("nonempty");
        let lead = lead.clone();
        if (1..m).any(|i| lead[i - 1] < lead[i]) {
            return Err(format!("not symmetric: leading exponent {lead:?}"));
        }
        let pw: Vec<u32> = (0..m).map(|k| lead[k] - if k + 1 < m { lead[k + 1] } else { 0 }).collect();
        let term = prod(&pw, &ek, &mut memo, m, p).scale(c, p);
        add_in(&mut rest, &neg(&term, p), p);
        let mut mono = Pol::zero(m);
        mono.t.insert(pw, c);
        add_in(&mut out, &mono, p);
    }
    Ok(out)
}

/// Remainder of `X^(2^t)` modulo `P(X) = X^m - e_1 X^(m-1) + e_2 X^(m-2) - ...` as `m` polynomials
/// in `e_1..e_m` (constant term first), by `t` squarings in `F_p[e][X]/(P)`.
pub fn x_pow_2t_mod(m: usize, t: usize, p: u64) -> Vec<Pol> {
    let e = |k: usize| Pol::var(m, k - 1);
    let reduce = |coef: Vec<Pol>| -> Vec<Pol> {
        let mut c = coef;
        while c.len() > m {
            let top = c.pop().expect("nonempty");
            let d = c.len();
            for k in 1..=m {
                let term = top.mul(&e(k), p);
                let idx = d - k;
                if k % 2 == 1 {
                    add_in(&mut c[idx], &term, p);
                } else {
                    add_in(&mut c[idx], &neg(&term, p), p);
                }
            }
        }
        while c.len() < m {
            c.push(Pol::zero(m));
        }
        c
    };
    let mut r: Vec<Pol> = reduce(vec![Pol::zero(m), Pol::c(m, 1, p)]);
    for _ in 0..t {
        let mut c = vec![Pol::zero(m); 2 * r.len() - 1];
        for i in 0..r.len() {
            for j in 0..r.len() {
                let pr = r[i].mul(&r[j], p);
                add_in(&mut c[i + j], &pr, p);
            }
        }
        r = reduce(c);
    }
    r
}

/// `e_1..e_m` of the given numbers.
pub fn elem_values(xs: &[u64], p: u64) -> Vec<u64> {
    let m = xs.len();
    let mut e = vec![0u64; m + 1];
    e[0] = 1;
    for &x in xs {
        for k in (1..=m).rev() {
            e[k] = addm(e[k], mulm(e[k - 1], x, p), p);
        }
    }
    e[1..].to_vec()
}

/// Coefficients (constant first) of `P(X) = prod (X - x_i)` from `e_1..e_m`.
pub fn p_poly_from_e(e: &[u64], p: u64) -> Vec<u64> {
    let m = e.len();
    let mut c = vec![0u64; m + 1];
    c[m] = 1;
    for k in 1..=m {
        let v = e[k - 1] % p;
        c[m - k] = if k % 2 == 0 { v } else { subm(0, v, p) };
    }
    c
}

/// `S_{m+1}(x_0..x_{m-1}, z)` split by the power of `z` (the last argument): entry `k` is the
/// coefficient of `z^k` as an `m`-variable polynomial. Independent of the target.
pub fn summation_by_last(m: usize, a: u64, b: u64, p: u64) -> Vec<Pol> {
    let f = summation_full(m, Last::Var(2 * m), a, b, p);
    let zi = 2 * m;
    let kmax = f.t.keys().map(|e| e[zi]).max().unwrap_or(0) as usize;
    let mut out = vec![Pol::zero(m); kmax + 1];
    for (e, &c) in &f.t {
        assert!(e[m..2 * m].iter().all(|&k| k == 0), "wire left in S_(m+1)");
        out[e[zi] as usize].t.insert(e[..m].to_vec(), c);
    }
    out
}

/// `sum_k z^k * parts[k]` for a constant `z` (counted multiplications).
pub fn eval_last(parts: &[Pol], z: u64, p: u64) -> Pol {
    let n = parts.first().map_or(0, |q| q.n);
    let mut out = Pol::zero(n);
    let mut pw = 1u64;
    for q in parts {
        add_in(&mut out, &q.scale(pw, p), p);
        pw = crate::algebra::mulm(pw, z, p);
    }
    out
}

/// The `R_j` tower of the symmetric presentation in a ring of `m*t` variables: `e_1..e_m` at
/// `0..m`, the coefficients of `R_j = X^(2^j) mod P(X)` (constant first) at `m*j..m*j+m`,
/// `j = 1..t-1`. Equations `R_j - R_{j-1}^2 mod P` and `R_{t-1}^2 - 1 mod P` (same order as
/// `algebra::symmetric_system`, without its leading `S_{m+1}` equation).
pub fn sym_tower_equations(m: usize, t: usize, p: u64) -> Vec<Pol> {
    let n = m * t;
    let e = |k: usize| Pol::var(n, k - 1);
    let reduce = |coef: Vec<Pol>| -> Vec<Pol> {
        let mut c = coef;
        while c.len() > m {
            let top = c.pop().expect("nonempty");
            let d = c.len();
            for k in 1..=m {
                let term = top.mul(&e(k), p);
                if k % 2 == 1 {
                    add_in(&mut c[d - k], &term, p);
                } else {
                    add_in(&mut c[d - k], &neg(&term, p), p);
                }
            }
        }
        while c.len() < m {
            c.push(Pol::zero(n));
        }
        c
    };
    let square = |r: &[Pol]| -> Vec<Pol> {
        let mut c = vec![Pol::zero(n); 2 * r.len() - 1];
        for i in 0..r.len() {
            for j in 0..r.len() {
                let pr = r[i].mul(&r[j], p);
                add_in(&mut c[i + j], &pr, p);
            }
        }
        reduce(c)
    };
    let mut eqs = Vec::new();
    let mut r: Vec<Pol> = (0..m).map(|k| if k == 1 { Pol::c(n, 1, p) } else { Pol::zero(n) }).collect();
    if m == 1 {
        r = reduce(vec![Pol::zero(n), Pol::c(n, 1, p)]);
    }
    for j in 1..t {
        let next = square(&r);
        let vars: Vec<Pol> = (0..m).map(|k| Pol::var(n, m * j + k)).collect();
        for k in 0..m {
            eqs.push(vars[k].sub(&next[k], p));
        }
        r = vars;
    }
    let last = square(&r);
    for (k, c) in last.iter().enumerate() {
        let target = if k == 0 { Pol::c(n, 1, p) } else { Pol::zero(n) };
        eqs.push(c.sub(&target, p));
    }
    eqs
}

/// Numeric `e_1..e_m` of the roots `w` followed by `R_j` coefficients, the planted point of the
/// symmetric tower.
pub fn sym_tower_point(w: &[u64], t: usize, p: u64) -> Vec<u64> {
    let m = w.len();
    let ek = {
        let mut e = vec![0u64; m + 1];
        e[0] = 1;
        for &wi in w {
            for k in (1..=m).rev() {
                e[k] = addm(e[k], mulm(e[k - 1], wi, p), p);
            }
        }
        e
    };
    let mut sol = vec![0u64; m * t];
    sol[..m].copy_from_slice(&ek[1..=m]);
    let sq = |a: &[u64]| -> Vec<u64> {
        let mut c = vec![0u64; 2 * m - 1];
        for i in 0..m {
            for j in 0..m {
                c[i + j] = addm(c[i + j], mulm(a[i], a[j], p), p);
            }
        }
        while c.len() > m {
            let top = c.pop().expect("nonempty");
            let d = c.len();
            for k in 1..=m {
                let term = mulm(top, ek[k], p);
                c[d - k] = if k % 2 == 1 { addm(c[d - k], term, p) } else { subm(c[d - k], term, p) };
            }
        }
        c
    };
    let mut r: Vec<u64> = (0..m).map(|k| u64::from(k == 1)).collect();
    for j in 1..t {
        r = sq(&r);
        sol[m * j..m * j + m].copy_from_slice(&r);
    }
    sol
}

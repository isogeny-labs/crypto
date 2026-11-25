//! Coefficient lattice: integer solutions u in Z^7 of A u = r (two rows),
//! LLL reduction of the kernel, exact closest vector, neighbourhoods.
//! Integer data are i128 with checked arithmetic; f64 only guides LLL and
//! enumeration, and every final comparison is exact.

pub type V7 = [i128; 7];

#[derive(Clone, Debug)]
pub struct CoeffLattice {
    pub rows: [V7; 2],
    pub rhs: [i128; 2],
    pub particular: V7,
    pub basis: Vec<V7>, // 5 LLL-reduced kernel vectors
}

#[derive(Debug)]
pub enum LatErr {
    RankDeficient,
    NoIntegerSolution,
    Overflow,
}

fn ck(v: Option<i128>) -> Result<i128, LatErr> {
    v.ok_or(LatErr::Overflow)
}

#[allow(dead_code)]
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    // returns (g, s, t) with s a + t b = g >= 0
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1i128, 0i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0.div_euclid(r1);
        let r2 = r0 - q * r1;
        r0 = r1;
        r1 = r2;
        let s2 = s0 - q * s1;
        s0 = s1;
        s1 = s2;
        let t2 = t0 - q * t1;
        t0 = t1;
        t1 = t2;
    }
    if r0 < 0 {
        (-r0, -s0, -t0)
    } else {
        (r0, s0, t0)
    }
}

pub fn constraint_row(pt: [i64; 3]) -> V7 {
    let (x, y, z) = (pt[0] as i128, pt[1] as i128, pt[2] as i128);
    [y * y * z, x * y * z, y * z * z, -x * x * x, -x * x * z, -x * z * z, -z * z * z]
}

pub fn constraint_rhs(pt: [i64; 3], p: i128, a: i128, b: i128) -> i128 {
    let (x, y, z) = (pt[0] as i128, pt[1] as i128, pt[2] as i128);
    let base = y * y * z - x * x * x - a * x * z * z - b * z * z * z;
    assert!(base % p == 0, "lift does not satisfy the finite equation");
    -base / p
}

fn dot(a: &V7, b: &V7) -> Result<i128, LatErr> {
    let mut s: i128 = 0;
    for i in 0..7 {
        s = ck(s.checked_add(ck(a[i].checked_mul(b[i]))?))?;
    }
    Ok(s)
}

fn axpy(y: &V7, q: i128, x: &V7) -> Result<V7, LatErr> {
    // y + q x
    let mut r = *y;
    for i in 0..7 {
        r[i] = ck(r[i].checked_add(ck(q.checked_mul(x[i]))?))?;
    }
    Ok(r)
}

pub fn solve(lifts: &[[i64; 3]; 2], p: i128, a: i128, b: i128) -> Result<CoeffLattice, LatErr> {
    let rows = [constraint_row(lifts[0]), constraint_row(lifts[1])];
    let rhs = [constraint_rhs(lifts[0], p, a, b), constraint_rhs(lifts[1], p, a, b)];
    // columns: cols[j] = (A[0][j], A[1][j]); V columns vcols[j] in Z^7
    let mut acol: Vec<[i128; 2]> = (0..7).map(|j| [rows[0][j], rows[1][j]]).collect();
    let mut vcol: Vec<V7> = (0..7)
        .map(|j| {
            let mut e = [0i128; 7];
            e[j] = 1;
            e
        })
        .collect();
    // eliminate row r over columns r..7 by a column Euclid (smallest pivot,
    // rounded quotients), which keeps the unimodular transform small
    for r in 0..2 {
        loop {
            let nz: Vec<usize> = (r..7).filter(|&c| acol[c][r] != 0).collect();
            if nz.len() <= 1 {
                if let Some(&c) = nz.first() {
                    acol.swap(r, c);
                    vcol.swap(r, c);
                }
                break;
            }
            let piv = *nz.iter().min_by_key(|&&c| acol[c][r].abs()).unwrap();
            for &c in &nz {
                if c == piv {
                    continue;
                }
                let a = acol[c][r];
                let b = acol[piv][r];
                // nearest-integer quotient: remainder in (-|b|/2, |b|/2]
                let mut q = a.div_euclid(b);
                let rr = a - q * b; // 0 <= rr < |b|
                if 2 * rr > b.abs() {
                    q += b.signum();
                }
                if q == 0 {
                    continue;
                }
                let pa = acol[piv];
                for k in 0..2 {
                    acol[c][k] = ck(acol[c][k].checked_sub(ck(q.checked_mul(pa[k]))?))?;
                }
                let pv = vcol[piv];
                vcol[c] = axpy(&vcol[c], -q, &pv)?;
            }
        }
    }
    let h11 = acol[0][0];
    let h21 = acol[0][1];
    let h22 = acol[1][1];
    if h11 == 0 || h22 == 0 {
        return Err(LatErr::RankDeficient);
    }
    if rhs[0] % h11 != 0 {
        return Err(LatErr::NoIntegerSolution);
    }
    let w0 = rhs[0] / h11;
    let t = ck(rhs[1].checked_sub(ck(h21.checked_mul(w0))?))?;
    if t % h22 != 0 {
        return Err(LatErr::NoIntegerSolution);
    }
    let w1 = t / h22;
    let mut particular = [0i128; 7];
    for k in 0..7 {
        particular[k] =
            ck(ck(w0.checked_mul(vcol[0][k]))?.checked_add(ck(w1.checked_mul(vcol[1][k]))?))?;
    }
    let kernel: Vec<V7> = vcol[2..7].to_vec();
    // exact checks
    for i in 0..2 {
        if dot(&rows[i], &particular)? != rhs[i] {
            panic!("particular solution check failed");
        }
        for v in &kernel {
            if dot(&rows[i], v)? != 0 {
                panic!("kernel check failed");
            }
        }
    }
    let basis = lll(kernel, 0.99)?;
    let mut lat = CoeffLattice { rows, rhs, particular, basis };
    // reduce the particular solution towards 0 (exact integer steps)
    for _ in 0..8 {
        let (v, _) = babai(&lat.basis, &neg7(&lat.particular))?;
        let np = add7(&lat.particular, &v)?;
        if np == lat.particular {
            break;
        }
        lat.particular = np;
    }
    for i in 0..2 {
        assert_eq!(dot(&lat.rows[i], &lat.particular)?, lat.rhs[i]);
    }
    Ok(lat)
}

fn neg7(a: &V7) -> V7 {
    let mut r = *a;
    for x in r.iter_mut() {
        *x = -*x;
    }
    r
}
pub fn add7(a: &V7, b: &V7) -> Result<V7, LatErr> {
    let mut r = [0i128; 7];
    for i in 0..7 {
        r[i] = ck(a[i].checked_add(b[i]))?;
    }
    Ok(r)
}

struct Gso {
    bstar: Vec<[f64; 7]>,
    bn: Vec<f64>,
    mu: Vec<Vec<f64>>,
}

fn gso(b: &[V7]) -> Gso {
    let n = b.len();
    let mut bstar: Vec<[f64; 7]> = Vec::with_capacity(n);
    let mut bn = vec![0.0; n];
    let mut mu = vec![vec![0.0; n]; n];
    for i in 0..n {
        let mut v = [0f64; 7];
        for k in 0..7 {
            v[k] = b[i][k] as f64;
        }
        for j in 0..i {
            let mut d = 0.0;
            for k in 0..7 {
                d += (b[i][k] as f64) * bstar[j][k];
            }
            mu[i][j] = d / bn[j];
            for k in 0..7 {
                v[k] -= mu[i][j] * bstar[j][k];
            }
        }
        bn[i] = v.iter().map(|x| x * x).sum();
        bstar.push(v);
    }
    Gso { bstar, bn, mu }
}

pub fn lll(mut b: Vec<V7>, delta: f64) -> Result<Vec<V7>, LatErr> {
    let n = b.len();
    let mut k = 1;
    let mut iters = 0u64;
    while k < n {
        iters += 1;
        if iters > 100_000 {
            panic!("LLL did not terminate");
        }
        // size reduce b_k
        for j in (0..k).rev() {
            let g = gso(&b);
            let m = g.mu[k][j];
            if m.abs() > 0.51 {
                let q = m.round() as i128;
                b[k] = axpy(&b[k], -q, &b[j])?;
            }
        }
        let g = gso(&b);
        if g.bn[k] >= (delta - g.mu[k][k - 1] * g.mu[k][k - 1]) * g.bn[k - 1] {
            k += 1;
        } else {
            b.swap(k, k - 1);
            k = if k > 1 { k - 1 } else { 1 };
        }
    }
    Ok(b)
}

/// Babai nearest plane: lattice vector close to target t. Returns (v, coeffs).
pub fn babai(b: &[V7], t: &V7) -> Result<(V7, Vec<i128>), LatErr> {
    let n = b.len();
    let g = gso(b);
    let mut r = [0f64; 7];
    for k in 0..7 {
        r[k] = t[k] as f64;
    }
    let mut coeffs = vec![0i128; n];
    let mut v = [0i128; 7];
    for i in (0..n).rev() {
        let mut d = 0.0;
        for k in 0..7 {
            d += r[k] * g.bstar[i][k];
        }
        let c = (d / g.bn[i]).round();
        coeffs[i] = c as i128;
        for k in 0..7 {
            r[k] -= c * (b[i][k] as f64);
        }
        v = axpy(&v, c as i128, &b[i])?;
    }
    Ok((v, coeffs))
}

pub fn norm2(v: &V7) -> Result<i128, LatErr> {
    dot(v, v)
}

/// Exact closest vector of particular + L to 0 (Euclidean). Ties: lexicographically smallest u.
pub fn closest_to_zero(lat: &CoeffLattice) -> Result<V7, LatErr> {
    let b = &lat.basis;
    let n = b.len();
    let t = neg7(&lat.particular); // want v in L closest to t; u = particular + v
    let (vb, _) = babai(b, &t)?;
    let ub = add7(&lat.particular, &vb)?;
    let rad = norm2(&ub)? as f64;
    let g = gso(b);
    // coordinates of t in GSO basis and its orthogonal part
    let tf: Vec<f64> = t.iter().map(|&x| x as f64).collect();
    let mut tau = vec![0.0; n];
    let mut proj2 = 0.0;
    for i in 0..n {
        let mut d = 0.0;
        for k in 0..7 {
            d += tf[k] * g.bstar[i][k];
        }
        tau[i] = d / g.bn[i];
        proj2 += tau[i] * tau[i] * g.bn[i];
    }
    let t2: f64 = tf.iter().map(|x| x * x).sum();
    let perp = (t2 - proj2).max(0.0);
    let bound = (rad - perp) + 1e-6 * rad + 2.0;
    let mut best: Option<(i128, V7)> = None;
    let mut x = vec![0i128; n];
    let mut count = 0u64;
    enumerate(n, n, &g, &tau, bound, 0.0, &mut x, &mut |xs: &[i128]| {
        count += 1;
        let mut v = [0i128; 7];
        for i in 0..n {
            v = axpy(&v, xs[i], &b[i]).expect("overflow in enumeration");
        }
        let u = add7(&lat.particular, &v).expect("overflow");
        let nn = norm2(&u).expect("overflow");
        match &best {
            None => best = Some((nn, u)),
            Some((bn, bu)) => {
                if nn < *bn || (nn == *bn && u < *bu) {
                    best = Some((nn, u));
                }
            }
        }
    });
    let (bn, bu) = best.expect("enumeration found nothing (Babai point must be inside)");
    assert!(bn <= norm2(&ub)?);
    Ok(bu)
}

fn enumerate(
    n: usize,
    level: usize,
    g: &Gso,
    tau: &[f64],
    bound: f64,
    partial: f64,
    x: &mut Vec<i128>,
    f: &mut dyn FnMut(&[i128]),
) {
    if level == 0 {
        f(x);
        return;
    }
    let i = level - 1;
    let mut c = tau[i];
    for j in (i + 1)..n {
        c -= g.mu[j][i] * (x[j] as f64);
    }
    let rem = bound - partial;
    if rem < 0.0 {
        return;
    }
    let w = (rem / g.bn[i]).sqrt();
    let lo = (c - w).ceil() as i128 - 1;
    let hi = (c + w).floor() as i128 + 1;
    for xi in lo..=hi {
        let d = xi as f64 - c;
        let np = partial + d * d * g.bn[i];
        if np > bound {
            continue;
        }
        x[i] = xi;
        enumerate(n, level - 1, g, tau, bound, np, x, f);
    }
    x[i] = 0;
}

/// Neighbourhood u* + sum d_i b_i, d in {-r..r}^5, in lexicographic order of d.
pub fn neighbourhood(center: &V7, basis: &[V7], r: i128) -> Result<Vec<V7>, LatErr> {
    let n = basis.len();
    let side = (2 * r + 1) as usize;
    let total = side.pow(n as u32);
    let mut out = Vec::with_capacity(total);
    for idx in 0..total {
        let mut rem = idx;
        let mut d = vec![0i128; n];
        for i in (0..n).rev() {
            d[i] = (rem % side) as i128 - r;
            rem /= side;
        }
        let mut u = *center;
        for i in 0..n {
            u = axpy(&u, d[i], &basis[i])?;
        }
        out.push(u);
    }
    Ok(out)
}

//! COPY (round 2) of research/isogeny-walk-quartic_20261115/src/planefit.rs with F_p multiplication
//! counting hooks (`st::count_m`, `st::count_i`) and the `super::st` field type.
//! Plane-curve fitting over F_p for points with F_{p^2} coordinates.
//! Used by `smith_walk_dlp modeltest` to ask whether the projected points of
//! Recillas' curve X lie on a plane curve of degree d.

use super::st::{count_i, count_m, Fp2, F2};

fn inv(a: u64, p: u64) -> u64 {
    count_i(1);
    let (mut r, mut b, mut e) = (1u64, a % p, p - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

/// Rank of a matrix over F_p (rows destroyed).
pub fn rank(rows: &mut [Vec<u64>], cols: usize, p: u64) -> usize {
    let mut r = 0;
    for c in 0..cols {
        let Some(piv) = (r..rows.len()).find(|&i| rows[i][c] != 0) else {
            continue;
        };
        rows.swap(r, piv);
        let iv = inv(rows[r][c], p);
        count_m(cols as u64);
        for x in rows[r].iter_mut() {
            *x = *x * iv % p;
        }
        for i in 0..rows.len() {
            if i != r && rows[i][c] != 0 {
                let f = rows[i][c];
                count_m(cols as u64);
                for j in 0..cols {
                    rows[i][j] = (rows[i][j] + p - f * rows[r][j] % p) % p;
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    r
}

/// Exponents (i, j) with i + j <= d.
pub fn monomials(d: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for i in 0..=d {
        for j in 0..=(d - i) {
            v.push((i, j));
        }
    }
    v
}

fn pw(k: &F2, x: Fp2, e: usize) -> Fp2 {
    let mut r = k.c(1);
    for _ in 0..e {
        r = k.mul(r, x);
    }
    r
}

/// Dimension of the space of polynomials of degree <= d in (u, v) over F_p
/// vanishing at every given point (F_{p^2} points contribute two F_p rows each).
pub fn kernel_dim(pts: &[(Fp2, Fp2)], d: usize, p: u64) -> usize {
    let k = F2::new(p);
    let mons = monomials(d);
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for &(u, v) in pts {
        let vals: Vec<Fp2> = mons
            .iter()
            .map(|&(i, j)| k.mul(pw(&k, u, i), pw(&k, v, j)))
            .collect();
        rows.push(vals.iter().map(|z| z.a).collect());
        if vals.iter().any(|z| z.b != 0) {
            rows.push(vals.iter().map(|z| z.b).collect());
        }
    }
    let n = mons.len();
    let r = rank(&mut rows, n, p);
    n - r
}

/// Basis of the space of plane curves of degree <= d (monomial order `monomials(d)`)
/// through the points (F_{p^2} points contribute two F_p rows each).
pub fn kernel_basis(pts: &[(Fp2, Fp2)], d: usize, p: u64) -> Vec<Vec<u64>> {
    let k = F2::new(p);
    let mons = monomials(d);
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for &(u, v) in pts {
        let vals: Vec<Fp2> = mons
            .iter()
            .map(|&(i, j)| k.mul(pw(&k, u, i), pw(&k, v, j)))
            .collect();
        rows.push(vals.iter().map(|z| z.a).collect());
        if vals.iter().any(|z| z.b != 0) {
            rows.push(vals.iter().map(|z| z.b).collect());
        }
    }
    nullspace(&mut rows, mons.len(), p)
}

/// Nullspace of a matrix over F_p (rows destroyed): basis vectors `z` with `rows * z = 0`.
pub fn nullspace(rows: &mut [Vec<u64>], cols: usize, p: u64) -> Vec<Vec<u64>> {
    let mut pivcols = Vec::new();
    let mut r = 0;
    for c in 0..cols {
        let Some(piv) = (r..rows.len()).find(|&i| rows[i][c] != 0) else {
            continue;
        };
        rows.swap(r, piv);
        let iv = inv(rows[r][c], p);
        count_m(cols as u64);
        for x in rows[r].iter_mut() {
            *x = *x * iv % p;
        }
        for i in 0..rows.len() {
            if i != r && rows[i][c] != 0 {
                let f = rows[i][c];
                count_m(cols as u64);
                for j in 0..cols {
                    rows[i][j] = (rows[i][j] + p - f * rows[r][j] % p) % p;
                }
            }
        }
        pivcols.push(c);
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    let mut out = Vec::new();
    for fc in (0..cols).filter(|c| !pivcols.contains(c)) {
        let mut z = vec![0u64; cols];
        z[fc] = 1;
        for (ri, &pc) in pivcols.iter().enumerate() {
            z[pc] = (p - rows[ri][fc]) % p;
        }
        out.push(z);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conic_through_points_of_circle() {
        // x^2 + y^2 = 1 over F_13: points have kernel dim 1 at d = 2, 0 at d = 1.
        let p = 13u64;
        let k = F2::new(p);
        let mut pts = Vec::new();
        for x in 0..p {
            for y in 0..p {
                if (x * x + y * y) % p == 1 {
                    pts.push((k.c(x), k.c(y)));
                }
            }
        }
        assert!(pts.len() > 8);
        assert_eq!(kernel_dim(&pts, 1, p), 0);
        assert_eq!(kernel_dim(&pts, 2, p), 1);
        assert_eq!(kernel_dim(&pts, 3, p), 3); // conic * linear forms (1, u, v)
        let b = kernel_basis(&pts, 2, p);
        assert_eq!(b.len(), 1);
        let mons = monomials(2);
        for &(x, y) in &[(1u64, 0u64), (0, 1)] {
            let v: u64 = mons
                .iter()
                .zip(&b[0])
                .map(|(&(i, j), &c)| c * x.pow(i as u32) % p * y.pow(j as u32) % p)
                .sum::<u64>()
                % p;
            assert_eq!(v, 0);
        }
    }

    #[test]
    fn fp2_points_split_into_two_rows() {
        // The line u = w (w^2 = nr) is not defined over F_p; its conjugate pair
        // (w, v) and (-w, v) forces u^2 - nr = 0 : kernel dim at d = 2 with
        // points (w, v), (-w, v) for v in 0..7 is exactly 1 + (vanishing of u^2 - nr) .
        let p = 13u64;
        let k = F2::new(p);
        let w = Fp2 { a: 0, b: 1 };
        let mut pts = Vec::new();
        for v in 0..7 {
            pts.push((w, k.c(v)));
            pts.push((k.neg(w), k.c(v)));
        }
        assert_eq!(kernel_dim(&pts, 2, p), 1);
    }
}

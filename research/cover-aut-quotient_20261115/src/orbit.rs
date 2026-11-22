//! Orbit quotient of a signed factor base by a group of automorphisms acting through scalars.
//!
//! Points of the factor base are signed ids `s`.  A generator `g` is a permutation `pi_g` of the ids
//! with a scalar `lambda_g` modulo `l` such that `log P_{pi_g(s)} = lambda_g * log P_s`.  The unknowns
//! of the quotient system are the orbits; a point `s` contributes the coefficient `coef[s]` on the
//! column `orbit_of[s]` (`log P_s = coef[s] * log rep`, `coef[rep] = 1`).

pub struct OrbitMap {
    pub l: u64,
    pub orbit_of: Vec<usize>,
    pub coef: Vec<u64>,
    pub n_orbits: usize,
}

#[inline]
fn mulmod(a: u64, b: u64, l: u64) -> u64 {
    ((a as u128 * b as u128) % l as u128) as u64
}

/// `Err` when a closed cycle forces `log P = 0` or two paths disagree on a coefficient (the
/// scalars are then not those of a group acting on the unknowns).
pub fn build_orbits(n: usize, l: u64, gens: &[(Vec<usize>, u64)]) -> Result<OrbitMap, String> {
    for (perm, _) in gens {
        if perm.len() != n {
            return Err("permutation length".into());
        }
    }
    let mut orbit_of = vec![usize::MAX; n];
    let mut coef = vec![0u64; n];
    let mut n_orbits = 0;
    for s0 in 0..n {
        if orbit_of[s0] != usize::MAX {
            continue;
        }
        orbit_of[s0] = n_orbits;
        coef[s0] = 1;
        let mut stack = vec![s0];
        while let Some(s) = stack.pop() {
            for (perm, lam) in gens {
                let t = perm[s];
                let want = mulmod(*lam, coef[s], l);
                if orbit_of[t] == usize::MAX {
                    orbit_of[t] = n_orbits;
                    coef[t] = want;
                    stack.push(t);
                } else if coef[t] != want {
                    return Err(format!(
                        "inconsistent scalar on orbit {n_orbits}: id {t} has {} but {want} forced",
                        coef[t]
                    ));
                }
            }
        }
        n_orbits += 1;
    }
    Ok(OrbitMap {
        l,
        orbit_of,
        coef,
        n_orbits,
    })
}

impl OrbitMap {
    /// Row columns for `sum_k e_k log P_{s_k}` over `(s_k, e_k)`: merged per orbit, zero entries
    /// dropped, sorted by column.
    pub fn row(&self, terms: &[(usize, u64)]) -> Vec<(usize, u64)> {
        let mut v: Vec<(usize, u64)> = Vec::new();
        for &(s, e) in terms {
            let c = mulmod(self.coef[s], e, self.l);
            let o = self.orbit_of[s];
            match v.iter_mut().find(|(oo, _)| *oo == o) {
                Some(slot) => slot.1 = (slot.1 + c) % self.l,
                None => v.push((o, c)),
            }
        }
        v.retain(|&(_, c)| c != 0);
        v.sort_unstable();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const L: u64 = 1_000_000_007;

    fn pow(b: u64, mut e: u64, l: u64) -> u64 {
        let (mut b, mut r) = (b % l, 1u64);
        while e > 0 {
            if e & 1 == 1 {
                r = mulmod(r, b, l);
            }
            b = mulmod(b, b, l);
            e >>= 1;
        }
        r
    }

    /// A prime with 3 | l - 1 and a primitive cube root of unity.
    fn prime_with_cube_root() -> (u64, u64) {
        let mut l = 1_000_000_021u64;
        loop {
            if (l - 1) % 3 == 0 && (2..).take_while(|d| d * d <= l).all(|d| l % d != 0) {
                break;
            }
            l += 1;
        }
        let g = (2..).find(|&g| pow(g, (l - 1) / 3, l) != 1).unwrap();
        (l, pow(g, (l - 1) / 3, l))
    }

    #[test]
    fn iota_only_pairs_up() {
        let b = 10;
        let perm: Vec<usize> = (0..2 * b).map(|s| s ^ 1).collect();
        let m = build_orbits(2 * b, L, &[(perm, L - 1)]).unwrap();
        assert_eq!(m.n_orbits, b);
        for i in 0..b {
            assert_eq!(m.orbit_of[2 * i], i);
            assert_eq!(m.coef[2 * i], 1);
            assert_eq!(m.coef[2 * i + 1], L - 1);
        }
    }

    #[test]
    fn inconsistent_scalar_is_rejected() {
        // a 2-cycle with lambda = 2 forces log P = 4 log P
        assert!(build_orbits(2, L, &[(vec![1, 0], 2)]).is_err());
    }

    #[test]
    fn order_six_group_divides_unknowns_by_six_and_recovers_logs() {
        let (l, w) = prime_with_cube_root();
        // layout within an orbit o: s = 6 o + 2 j + e, j in 0..3, e = sign
        let orbits = 40;
        let n = 6 * orbits;
        let rho: Vec<usize> = (0..n)
            .map(|s| {
                let (o, r) = (s / 6, s % 6);
                6 * o + 2 * ((r / 2 + 1) % 3) + r % 2
            })
            .collect();
        let iota: Vec<usize> = (0..n).map(|s| s ^ 1).collect();
        let m = build_orbits(n, l, &[(rho, w), (iota, l - 1)]).unwrap();
        assert_eq!(m.n_orbits, orbits);
        assert_eq!(n / m.n_orbits, 6);
        let rep_log: Vec<u64> = (0..orbits as u64).map(|o| (o * 7919 + 13) % l).collect();
        let log_of = |s: usize| mulmod(m.coef[s], rep_log[m.orbit_of[s]], l);
        for s in 0..n {
            let (o, r) = (s / 6, s % 6);
            let t = 6 * o + 2 * ((r / 2 + 1) % 3) + r % 2;
            assert_eq!(log_of(t), mulmod(w, log_of(s), l));
            assert_eq!(log_of(s ^ 1), mulmod(l - 1, log_of(s), l));
        }
        let mut state = 12345u64;
        let mut next = || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as usize
        };
        let mut rows: Vec<Vec<u64>> = Vec::new();
        while rows.len() < orbits + 5 {
            let terms: Vec<(usize, u64)> = (0..6).map(|_| (next() % n, 1u64)).collect();
            let cols = m.row(&terms);
            let rhs = terms.iter().fold(0u64, |a, &(s, _)| (a + log_of(s)) % l);
            let mut dense = vec![0u64; orbits + 1];
            for (c, v) in cols {
                dense[c] = v;
            }
            dense[orbits] = rhs;
            rows.push(dense);
        }
        let mut r = 0;
        for c in 0..orbits {
            let Some(pv) = (r..rows.len()).find(|&i| rows[i][c] != 0) else {
                continue;
            };
            rows.swap(r, pv);
            let inv = pow(rows[r][c], l - 2, l);
            for x in rows[r].iter_mut() {
                *x = mulmod(*x, inv, l);
            }
            for i in 0..rows.len() {
                if i != r && rows[i][c] != 0 {
                    let f = rows[i][c];
                    for k in 0..=orbits {
                        let sub = mulmod(f, rows[r][k], l);
                        rows[i][k] = (rows[i][k] + l - sub) % l;
                    }
                }
            }
            r += 1;
        }
        assert_eq!(r, orbits, "full rank on the quotient unknowns");
        for i in 0..orbits {
            assert_eq!(rows[i][orbits], rep_log[i]);
        }
    }

    #[test]
    fn same_orbit_terms_merge() {
        let perm: Vec<usize> = (0..4).map(|s| s ^ 1).collect();
        let m = build_orbits(4, L, &[(perm, L - 1)]).unwrap();
        assert!(m.row(&[(0, 1), (1, 1)]).is_empty());
        assert_eq!(m.row(&[(0, 1), (0, 1)]), vec![(0, 2)]);
    }
}

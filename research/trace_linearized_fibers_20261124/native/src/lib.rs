//! Conditional scalar fibers of binary S3, not a solver for unknown leaves.
//! Uses the preceding study's field implementation without modifying its pins.
use trace_kernel_basis::Field;

fn divide(f: Field, a: u16, b: u16) -> u16 {
    assert_ne!(b, 0);
    f.multiply(a, f.power(b, f.size() - 2))
}

#[derive(Debug, Clone, Copy)]
pub struct Linearized {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

impl Linearized {
    pub fn from_s3(f: Field, x: u16, y: u16, curve_c: u16) -> Self {
        let product = f.multiply(x, y);
        Self {
            a: f.square(x ^ y),
            b: product,
            c: f.square(product) ^ curve_c,
        }
    }

    pub fn linear(self, f: Field, u: u16) -> u16 {
        f.multiply(self.a, f.square(u)) ^ f.multiply(self.b, u)
    }

    pub fn evaluate(self, f: Field, u: u16) -> u16 {
        self.linear(f, u) ^ self.c
    }

    /// Full GF(2) row reduction, retaining every free coordinate, even when
    /// both leading coefficients vanish. No trace condition is implicit.
    pub fn roots_linear(self, f: Field) -> Vec<u16> {
        let n = f.degree as usize;
        let columns: Vec<_> = (0..n).map(|i| self.linear(f, 1 << i)).collect();
        let mut rows: Vec<u32> = (0..n)
            .map(|bit| {
                let lhs = columns
                    .iter()
                    .enumerate()
                    .fold(0, |row, (i, v)| row | (u32::from((v >> bit) & 1) << i));
                lhs | (u32::from((self.c >> bit) & 1) << n)
            })
            .collect();
        let mut pivots = vec![];
        for column in 0..n {
            let Some(found) = (pivots.len()..n).find(|&i| rows[i] & (1 << column) != 0) else {
                continue;
            };
            let pivot = pivots.len();
            rows.swap(pivot, found);
            for i in 0..n {
                if i != pivot && rows[i] & (1 << column) != 0 {
                    rows[i] ^= rows[pivot];
                }
            }
            pivots.push(column);
        }
        let mask = (1 << n) - 1;
        if rows.iter().any(|r| r & mask == 0 && r & (1 << n) != 0) {
            return vec![];
        }
        let mut particular = 0u16;
        for (row, &column) in pivots.iter().enumerate() {
            if rows[row] & (1 << n) != 0 {
                particular |= 1 << column;
            }
        }
        let mut roots = vec![particular];
        for free in (0..n).filter(|c| !pivots.contains(c)) {
            let mut vector = 1 << free;
            for (row, &column) in pivots.iter().enumerate() {
                if rows[row] & (1 << free) != 0 {
                    vector |= 1 << column;
                }
            }
            let more: Vec<_> = roots.iter().map(|v| v ^ vector).collect();
            roots.extend(more);
        }
        roots.sort_unstable();
        roots
    }

    /// Independent odd-degree half-trace route. Even degree is explicit.
    pub fn roots_half_trace(self, f: Field) -> Result<Vec<u16>, &'static str> {
        if f.degree.is_multiple_of(2) {
            return Err("half-trace route requires odd extension degree");
        }
        if self.a == 0 {
            return Ok(if self.b != 0 {
                vec![divide(f, self.c, self.b)]
            } else if self.c == 0 {
                (0..f.size()).collect()
            } else {
                vec![]
            });
        }
        if self.b == 0 {
            return Ok(vec![f.power(divide(f, self.c, self.a), 1 << (f.degree - 1))]);
        }
        let h = divide(f, f.multiply(self.a, self.c), f.square(self.b));
        if f.trace(h) != 0 {
            return Ok(vec![]);
        }
        let delta = divide(f, self.b, self.a);
        let mut term = h;
        let mut half_trace = 0;
        for _ in 0..f.degree.div_ceil(2) {
            half_trace ^= term;
            term = f.square(f.square(term));
        }
        let root = f.multiply(delta, half_trace);
        let mut out = vec![root, root ^ delta];
        out.sort_unstable();
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    UniqueCandidate,
    InconsistentCombination,
    DegenerateFallback,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fiber {
    pub roots: Vec<u16>,
    pub route: Route,
}

/// Eliminate u^2 from TWO blocks, testing original equations before admission.
/// Includes D=0, zero coordinates and linear/constant polynomial branches.
pub fn chain_fiber(f: Field, x: u16, y: u16, z: u16, t: u16, curve_c: u16) -> Fiber {
    let left = Linearized::from_s3(f, x, y, curve_c);
    let right = Linearized::from_s3(f, z, t, curve_c);
    let d = f.multiply(left.a, right.b) ^ f.multiply(right.a, left.b);
    let numerator = f.multiply(left.a, right.c) ^ f.multiply(right.a, left.c);
    let (candidates, route) = if d != 0 {
        (vec![divide(f, numerator, d)], Route::UniqueCandidate)
    } else if numerator != 0 {
        (vec![], Route::InconsistentCombination)
    } else {
        (left.roots_linear(f), Route::DegenerateFallback)
    };
    Fiber {
        roots: candidates
            .into_iter()
            .filter(|&u| left.evaluate(f, u) == 0 && right.evaluate(f, u) == 0)
            .collect(),
        route,
    }
}

pub fn trace_filter(f: Field, fiber: Fiber, tau: u16) -> Result<Fiber, &'static str> {
    if tau > 1 {
        return Err("trace class must be a bit");
    }
    Ok(Fiber {
        roots: fiber
            .roots
            .into_iter()
            .filter(|&u| f.trace(u) == tau)
            .collect(),
        route: fiber.route,
    })
}

/// Dividing the first S3 equation by y^2 fixes its linear operator when r=x/y.
pub fn ratio_operator(f: Field, r: u16) -> (u16, u16) {
    (f.square(r ^ 1), r)
}

/// Put the conditional null direction first in a full trace-kernel basis.
/// This exposes a zero column without changing the operator's intrinsic rank.
pub fn aligned_trace_basis(f: Field, x: u16, y: u16) -> Result<Vec<u16>, &'static str> {
    use trace_kernel_basis::{rank, Chart};
    if x == 0 || y == 0 || x == y {
        return Err("kernel-aligned chart requires nonzero distinct coordinates");
    }
    let delta = divide(f, f.multiply(x, y), f.square(x ^ y));
    let mut basis = vec![delta];
    for value in Chart::standard(f, 0).kernel {
        let mut next = basis.clone();
        next.push(value);
        if rank(&next) > basis.len() {
            basis.push(value);
        }
    }
    Ok(basis)
}

#[cfg(test)]
mod tests {
    use super::*;
    use trace_kernel_basis::{rank, Chart};

    fn brute(f: Field, p: Linearized) -> Vec<u16> {
        (0..f.size()).filter(|&u| p.evaluate(f, u) == 0).collect()
    }

    #[test]
    fn every_small_linearized_quadratic_matches_exhaustive_roots() {
        let f = Field::new(3).unwrap();
        for a in 0..f.size() {
            for b in 0..f.size() {
                for c in 0..f.size() {
                    let p = Linearized { a, b, c };
                    let expected = brute(f, p);
                    assert_eq!(p.roots_linear(f), expected);
                    assert_eq!(p.roots_half_trace(f).unwrap(), expected);
                }
            }
        }
    }

    #[test]
    fn odd_degree_half_trace_matches_linear_algebra_and_even_is_explicit() {
        let f = Field::new(7).unwrap();
        for a in [0, 1, 3, 11] {
            for b in [0, 1, 5] {
                for c in 0..f.size() {
                    let p = Linearized { a, b, c };
                    assert_eq!(p.roots_half_trace(f).unwrap(), p.roots_linear(f));
                }
            }
        }
        assert!(Linearized { a: 1, b: 1, c: 0 }
            .roots_half_trace(Field::new(4).unwrap())
            .is_err());
    }

    #[test]
    fn kernel_delta_is_trace_zero_and_restricted_rank_is_n_minus_two() {
        for n in [3, 4] {
            let f = Field::new(n).unwrap();
            let kernel = Chart::standard(f, 0).kernel;
            for x in 1..f.size() {
                for y in 1..f.size() {
                    if x == y {
                        continue;
                    }
                    let p = Linearized::from_s3(f, x, y, 3);
                    let delta = divide(f, p.b, p.a);
                    let s = divide(f, x, x ^ y);
                    assert_eq!(delta, f.square(s) ^ s);
                    assert_ne!(delta, 0);
                    assert_eq!(f.trace(delta), 0);
                    let columns: Vec<_> = kernel.iter().map(|&v| p.linear(f, v)).collect();
                    assert_eq!(rank(&columns), n as usize - 2);
                    let aligned = aligned_trace_basis(f, x, y).unwrap();
                    assert_eq!(aligned[0], delta);
                    assert_eq!(
                        trace_kernel_basis::span(&aligned),
                        trace_kernel_basis::span(&kernel)
                    );
                    assert_eq!(p.linear(f, aligned[0]), 0);
                    let remaining: Vec<_> = aligned[1..].iter().map(|&v| p.linear(f, v)).collect();
                    assert_eq!(rank(&remaining), n as usize - 2);
                    for tau in [0, 1] {
                        let offset = Chart::standard(f, tau).offset;
                        let controlled = Linearized {
                            c: p.linear(f, offset),
                            ..p
                        };
                        let mut expected = vec![offset, offset ^ delta];
                        expected.sort_unstable();
                        assert_eq!(controlled.roots_linear(f), expected);
                        assert!(expected.iter().all(|&u| f.trace(u) == tau));
                    }
                    let roots = p.roots_linear(f);
                    if roots.len() == 2 {
                        assert_eq!(roots[0] ^ roots[1], delta);
                        assert_eq!(f.trace(roots[0]), f.trace(roots[1]));
                    }
                }
            }
        }
    }

    #[test]
    fn complete_chain_roots_and_determinant_factorization() {
        let guard_field = Field::new(3).unwrap();
        assert!(aligned_trace_basis(guard_field, 0, 1).is_err());
        assert!(aligned_trace_basis(guard_field, 1, 1).is_err());
        let f = Field::new(3).unwrap();
        let curve_c = 3;
        let sqrt_c = f.power(curve_c, 1 << (f.degree - 1));
        let mut seen = [false; 3];
        for x in 0..f.size() {
            for y in 0..f.size() {
                for z in 0..f.size() {
                    for t in 0..f.size() {
                        let a = Linearized::from_s3(f, x, y, curve_c);
                        let b = Linearized::from_s3(f, z, t, curve_c);
                        let d = f.multiply(a.a, b.b) ^ f.multiply(b.a, a.b);
                        let factored = f.multiply(
                            f.multiply(x, z) ^ f.multiply(y, t),
                            f.multiply(x, t) ^ f.multiply(y, z),
                        );
                        assert_eq!(d, factored);
                        let e3 = f.multiply(f.multiply(x, y), z)
                            ^ f.multiply(f.multiply(x, y), t)
                            ^ f.multiply(f.multiply(x, z), t)
                            ^ f.multiply(f.multiply(y, z), t);
                        let numerator = f.multiply(a.a, b.c) ^ f.multiply(b.a, a.c);
                        assert_eq!(numerator, f.square(e3 ^ f.multiply(sqrt_c, x ^ y ^ z ^ t)));
                        let got = chain_fiber(f, x, y, z, t, curve_c);
                        seen[match got.route {
                            Route::UniqueCandidate => 0,
                            Route::InconsistentCombination => 1,
                            Route::DegenerateFallback => 2,
                        }] = true;
                        let expected: Vec<_> = (0..f.size())
                            .filter(|&u| a.evaluate(f, u) == 0 && b.evaluate(f, u) == 0)
                            .collect();
                        assert_eq!(got.roots, expected);
                        for tau in [0, 1] {
                            let filtered =
                                trace_filter(f, chain_fiber(f, x, y, z, t, curve_c), tau).unwrap();
                            assert_eq!(
                                filtered.roots,
                                expected
                                    .iter()
                                    .copied()
                                    .filter(|&u| f.trace(u) == tau)
                                    .collect::<Vec<_>>()
                            );
                        }
                    }
                }
            }
        }
        assert!(seen.into_iter().all(|v| v));
    }

    #[test]
    fn coupled_operator_restricted_rank_tracks_kernel_alignment() {
        let f = Field::new(3).unwrap();
        let kernel = Chart::standard(f, 0).kernel;
        for x in 1..f.size() {
            for y in 1..f.size() {
                for z in 1..f.size() {
                    for t in 1..f.size() {
                        if x == y || z == t {
                            continue;
                        }
                        let a = Linearized::from_s3(f, x, y, 3);
                        let b = Linearized::from_s3(f, z, t, 3);
                        let d = f.multiply(a.a, b.b) ^ f.multiply(b.a, a.b);
                        let columns: Vec<_> = kernel
                            .iter()
                            .map(|&v| a.linear(f, v) | (b.linear(f, v) << f.degree))
                            .collect();
                        assert_eq!(rank(&columns), if d == 0 { 1 } else { 2 });
                    }
                }
            }
        }
    }

    #[test]
    fn equal_ratios_share_an_operator_after_exact_scaling() {
        for n in [3, 4] {
            let f = Field::new(n).unwrap();
            for r in 1..f.size() {
                for y in 1..f.size() {
                    let x = f.multiply(r, y);
                    let p = Linearized::from_s3(f, x, y, 3);
                    let (a, b) = ratio_operator(f, r);
                    assert_eq!(divide(f, p.a, f.square(y)), a);
                    assert_eq!(divide(f, p.b, f.square(y)), b);
                    let expected_c =
                        f.multiply(f.square(r), f.square(y)) ^ divide(f, 3, f.square(y));
                    assert_eq!(divide(f, p.c, f.square(y)), expected_c);
                }
            }
        }
    }
}

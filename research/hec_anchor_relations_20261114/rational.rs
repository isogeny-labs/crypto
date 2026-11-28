//! Correctness gate only. See RATIONAL-PROTOCOL.md for scope and prior art.
use hec_native_ic::{fp::Fp, jac::Curve, poly::Poly};

pub struct Primitive {
    pub a: Poly,
    pub b: Poly,
    pub q: Poly,
    pub keep_mask: usize,
    pub common_degree: usize,
    pub removed: usize,
}

/// Remove vertical factors and the anchor conditions they invalidated.
/// Does not assume interpolation survives at a canceled anchor.
pub fn primitive(c: &Curve, points: &[(u64, u64)], a: Poly, b: Poly) -> Option<Primitive> {
    if a.is_zero() && b.is_zero() {
        return None;
    }
    let f = &c.fp;
    let h = Poly::gcd(&a, &b, f);
    let a = a.div_exact(&h, f);
    let b = b.div_exact(&h, f);
    assert_eq!(Poly::gcd(&a, &b, f).deg(), 0);
    let mut keep_mask = 0;
    let mut removed = 0;
    let mut anchors = Poly::one();
    for (i, &(x, _)) in points.iter().enumerate() {
        if h.eval(x, f) == 0 {
            removed += 1;
        } else {
            keep_mask |= 1 << i;
            anchors = anchors.mul(&Poly::x_minus(x, f), f);
        }
    }
    assert!(removed <= h.deg() as usize);
    let norm = a.mul(&a, f).sub(&c.f.mul(&b.mul(&b, f), f), f);
    assert!(!norm.is_zero());
    let (q, r) = norm.divrem(&anchors, f);
    assert!(r.is_zero());
    assert!(q.deg() <= c.g as isize);
    Some(Primitive {
        a,
        b,
        q,
        keep_mask,
        common_degree: h.deg() as usize,
        removed,
    })
}

pub struct SignedFamily {
    pub m: usize,
    pub d: usize,
    terms: Vec<(usize, Poly, Poly)>,
}

fn product(xs: &[u64], mask: usize, f: &Fp) -> Poly {
    xs.iter()
        .enumerate()
        .filter(|(i, _)| mask & (1 << i) != 0)
        .fold(Poly::one(), |a, (_, &x)| a.mul(&Poly::x_minus(x, f), f))
}

impl SignedFamily {
    pub fn new(g: usize, d: usize, points: &[(u64, u64)], f: &Fp) -> Self {
        let m = g + 2 * d;
        assert!((2..=4).contains(&g) && (1..=3).contains(&d));
        assert_eq!(points.len(), m);
        let xs: Vec<_> = points.iter().map(|p| p.0).collect();
        let mut weights = Vec::new();
        for (i, &(x, y)) in points.iter().enumerate() {
            let mut der = 1;
            for (j, &(z, _)) in points.iter().enumerate() {
                if i != j {
                    assert_ne!(x, z, "duplicate abscissa");
                    der = f.mul(der, f.sub(x, z));
                }
            }
            weights.push(f.mul(y, f.inv(der)));
        }
        let full = (1 << m) - 1;
        let mut terms = Vec::new();
        for mask in 0usize..=full {
            let n = mask.count_ones() as usize;
            if n != d && n != d - 1 {
                continue;
            }
            let mut w = 1;
            for i in 0..m {
                if mask & (1 << i) == 0 {
                    continue;
                }
                w = f.mul(w, weights[i]);
                for j in 0..i {
                    if mask & (1 << j) != 0 {
                        let delta = f.sub(xs[i], xs[j]);
                        w = f.mul(w, f.mul(delta, delta));
                    }
                }
            }
            let a = if n == d {
                product(&xs, full ^ mask, f).scale(w, f)
            } else {
                Poly::zero()
            };
            let b = if n == d - 1 {
                product(&xs, mask, f).scale(w, f)
            } else {
                Poly::zero()
            };
            terms.push((mask >> 1, a, b)); // sigma0 = +1
        }
        Self { m, d, terms }
    }

    pub fn direct(&self, signs: usize, f: &Fp) -> (Poly, Poly) {
        let mut a = Poly::zero();
        let mut b = Poly::zero();
        for &(mask, x, y) in &self.terms {
            if (mask & signs).count_ones() % 2 == 0 {
                a = a.add(&x, f);
                b = b.add(&y, f);
            } else {
                a = a.sub(&x, f);
                b = b.sub(&y, f);
            }
        }
        (a, b)
    }

    pub fn walsh(&self, f: &Fp) -> Vec<(Poly, Poly)> {
        let n = 1 << (self.m - 1);
        let mut out = vec![(Poly::zero(), Poly::zero()); n];
        for &(mask, a, b) in &self.terms {
            out[mask].0 = out[mask].0.add(&a, f);
            out[mask].1 = out[mask].1.add(&b, f);
        }
        let mut h = 1;
        while h < n {
            for base in (0..n).step_by(2 * h) {
                for j in base..base + h {
                    let (a, b) = out[j];
                    let (c, d) = out[j + h];
                    out[j] = (a.add(&c, f), b.add(&d, f));
                    out[j + h] = (a.sub(&c, f), b.sub(&d, f));
                }
            }
            h *= 2;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hec_native_ic::{
        instance::{factor_base, thread_curve},
        jac::Curve,
    };

    // Independent homogeneous elimination; no subset weights or norm used.
    fn gaussian(g: usize, d: usize, pts: &[(u64, u64)], signs: usize, f: &Fp) -> (usize, Vec<u64>) {
        let na = g + d + 1;
        let cols = na + d;
        let mut rows = vec![vec![0; cols]; pts.len()];
        for (i, &(x, y)) in pts.iter().enumerate() {
            let y = if i > 0 && signs & (1 << (i - 1)) != 0 {
                f.neg(y)
            } else {
                y
            };
            let mut power = 1;
            for j in 0..na {
                rows[i][j] = power;
                power = f.mul(power, x);
            }
            power = 1;
            for j in 0..d {
                rows[i][na + j] = f.neg(f.mul(y, power));
                power = f.mul(power, x);
            }
        }
        let mut pivots = Vec::new();
        for col in 0..cols {
            let r = pivots.len();
            let Some(p) = (r..rows.len()).find(|&i| rows[i][col] != 0) else {
                continue;
            };
            rows.swap(r, p);
            let inv = f.inv(rows[r][col]);
            for j in col..cols {
                rows[r][j] = f.mul(rows[r][j], inv);
            }
            for i in 0..rows.len() {
                if i == r {
                    continue;
                }
                let q = rows[i][col];
                for j in col..cols {
                    rows[i][j] = f.sub(rows[i][j], f.mul(q, rows[r][j]));
                }
            }
            pivots.push(col);
            if pivots.len() == rows.len() {
                break;
            }
        }
        let free = (0..cols).find(|j| !pivots.contains(j)).unwrap();
        let mut v = vec![0; cols];
        v[free] = 1;
        for (r, &col) in pivots.iter().enumerate() {
            v[col] = f.neg(rows[r][free]);
        }
        (cols - pivots.len(), v)
    }

    fn verify_linear(
        g: usize,
        d: usize,
        pts: &[(u64, u64)],
        s: usize,
        a: Poly,
        b: Poly,
        f: &Fp,
    ) -> usize {
        let (nullity, v) = gaussian(g, d, pts, s, f);
        let na = g + d + 1;
        for (i, &(x, y)) in pts.iter().enumerate() {
            let y = if i > 0 && s & (1 << (i - 1)) != 0 {
                f.neg(y)
            } else {
                y
            };
            assert_eq!(a.eval(x, f), f.mul(y, b.eval(x, f)), "subset interpolation");
            let ga = Poly::from_coeffs(&v[..na]);
            let gb = Poly::from_coeffs(&v[na..]);
            assert_eq!(
                ga.eval(x, f),
                f.mul(y, gb.eval(x, f)),
                "Gaussian interpolation"
            );
        }
        if nullity == 1 {
            let u: Vec<_> = (0..na)
                .map(|i| a.c[i])
                .chain((0..d).map(|i| b.c[i]))
                .collect();
            // A cofactor formula can vanish at a rank-deficient specialization;
            // full row rank must produce a nonzero projective kernel vector.
            let first = u
                .iter()
                .position(|&x| x != 0)
                .expect("zero formula at full row rank");
            let scale = f.mul(u[first], f.inv(v[first]));
            for i in 0..u.len() {
                assert_eq!(u[i], f.mul(scale, v[i]));
            }
        }
        nullity
    }

    #[test]
    fn exhaustive_arbitrary_interpolation() {
        let f = Fp::new(7);
        for code in 0..7u64.pow(4) {
            let mut k = code;
            let pts: Vec<_> = (0..4)
                .map(|x| {
                    let y = k % 7;
                    k /= 7;
                    (x, y)
                })
                .collect();
            let family = SignedFamily::new(2, 1, &pts, &f);
            let (a, b) = family.direct(0, &f);
            verify_linear(2, 1, &pts, 0, a, b, &f);
        }
        println!("arbitrary_interpolation_cases=2401");
    }

    #[test]
    fn curve_norms_and_jacobian_relations() {
        for g in 2..=4 {
            for p in [101, 127] {
                for d in 1..=3 {
                    let c = (1..60).find_map(|k| thread_curve(p, k, g)).unwrap();
                    let f = &c.fp;
                    let fb = factor_base(&c);
                    let m = g + 2 * d;
                    assert!(fb.len() >= m + 9);
                    let mut counts = [0usize; 8]; // cases,multi-nullity,zero,nonprimitive,degree-drop,nonsplit,group,branch roots
                    for offset in [0, 3, 6, 9] {
                        let pts = &fb[offset..offset + m];
                        let family = SignedFamily::new(g, d, pts, f);
                        let batch = family.walsh(f);
                        let xs: Vec<_> = pts.iter().map(|p| p.0).collect();
                        let anchor = product(&xs, (1 << m) - 1, f);
                        for (s, &(a, b)) in batch.iter().enumerate() {
                            counts[0] += 1;
                            assert_eq!((a, b), family.direct(s, f));
                            let nullity = verify_linear(g, d, pts, s, a, b, f);
                            if nullity > 1 {
                                counts[1] += 1;
                            }
                            let norm = a.mul(&a, f).sub(&c.f.mul(&b.mul(&b, f), f), f);
                            let (q, r) = norm.divrem(&anchor, f);
                            assert!(r.is_zero());
                            assert!(q.deg() <= g as isize);
                            if a.is_zero() && b.is_zero() {
                                counts[2] += 1;
                                continue;
                            }
                            assert!(!norm.is_zero());
                            if Poly::gcd(&a, &b, f).deg() > 0 {
                                counts[3] += 1;
                                continue;
                            }
                            assert!(!b.is_zero());
                            if q.deg() < g as isize {
                                counts[4] += 1;
                            }
                            let mut rest = q;
                            let mut roots = Vec::new();
                            for x in 0..p {
                                while rest.deg() > 0 && rest.eval(x, f) == 0 {
                                    let (quot, rem) = rest.divrem(&Poly::x_minus(x, f), f);
                                    assert!(rem.is_zero());
                                    roots.push(x);
                                    rest = quot;
                                }
                            }
                            if rest.deg() > 0 {
                                counts[5] += 1;
                                continue;
                            }
                            let mut sum = c.zero();
                            for (i, &(x, y)) in pts.iter().enumerate() {
                                assert_ne!(b.eval(x, f), 0);
                                let y = if i > 0 && s & (1 << (i - 1)) != 0 {
                                    f.neg(y)
                                } else {
                                    y
                                };
                                sum = c.add(&sum, &c.place(x, y));
                            }
                            for x in roots {
                                let bx = b.eval(x, f);
                                assert_ne!(bx, 0);
                                let y = f.mul(a.eval(x, f), f.inv(bx));
                                assert_eq!(f.mul(y, y), c.f.eval(x, f));
                                if y == 0 {
                                    counts[7] += 1;
                                }
                                sum = c.add(&sum, &c.place(x, y));
                            }
                            assert!(
                                c.is_zero(&sum),
                                "g={g} p={p} d={d} offset={offset} signs={s}"
                            );
                            counts[6] += 1;
                        }
                    }
                    assert!(counts[6] > 0, "vacuous group gate");
                    println!("g={g} p={p} d={d} cases={} multi_nullity={} zero={} nonprimitive={} degree_drop={} nonsplit={} group={} branch_roots={}",counts[0],counts[1],counts[2],counts[3],counts[4],counts[5],counts[6],counts[7]);
                }
            }
        }
    }

    #[test]
    fn all_zero_degenerate_control() {
        let f = Fp::new(101);
        for d in [2, 3] {
            let pts: Vec<_> = (0..2 + 2 * d).map(|x| (x as u64, 0)).collect();
            let family = SignedFamily::new(2, d, &pts, &f);
            for (s, (a, b)) in family.walsh(&f).into_iter().enumerate() {
                assert!(a.is_zero() && b.is_zero());
                assert!(verify_linear(2, d, &pts, s, a, b, &f) > 1);
            }
        }
    }

    #[test]
    #[should_panic(expected = "duplicate abscissa")]
    fn duplicate_x_rejected() {
        SignedFamily::new(2, 1, &[(0, 1), (1, 2), (1, 3), (3, 4)], &Fp::new(101));
    }

    #[test]
    fn branch_anchor_control() {
        let f = Fp::new(101);
        let c = Curve::new(f, Poly::from_coeffs(&[0, 1, 0, 0, 0, 1]));
        let fb = factor_base(&c);
        assert_eq!(fb[0], (0, 0));
        for d in 1..=3 {
            let pts = &fb[..2 + 2 * d];
            let family = SignedFamily::new(2, d, pts, &f);
            for (s, (a, b)) in family.walsh(&f).into_iter().enumerate() {
                verify_linear(2, d, pts, s, a, b, &f);
            }
        }
    }

    fn check_primitive(
        c: &Curve,
        pts: &[(u64, u64)],
        signs: usize,
        a: Poly,
        b: Poly,
    ) -> (bool, bool, usize) {
        let f = &c.fp;
        let Some(p) = primitive(c, pts, a, b) else {
            return (false, false, 0);
        };
        for (i, &(x, y)) in pts.iter().enumerate() {
            if p.keep_mask & (1 << i) == 0 {
                continue;
            }
            let y = if i > 0 && signs & (1 << (i - 1)) != 0 {
                f.neg(y)
            } else {
                y
            };
            assert_ne!(p.b.eval(x, f), 0);
            assert_eq!(p.a.eval(x, f), f.mul(y, p.b.eval(x, f)));
        }
        let mut q = p.q;
        let mut roots = Vec::new();
        for x in 0..f.p {
            while q.deg() > 0 && q.eval(x, f) == 0 {
                roots.push(x);
                q = q.div_exact(&Poly::x_minus(x, f), f);
            }
        }
        if q.deg() > 0 {
            return (p.common_degree > 0, false, p.removed);
        }
        let mut sum = c.zero();
        for (i, &(x, y)) in pts.iter().enumerate() {
            if p.keep_mask & (1 << i) != 0 {
                let y = if i > 0 && signs & (1 << (i - 1)) != 0 {
                    f.neg(y)
                } else {
                    y
                };
                sum = c.add(&sum, &c.place(x, y));
            }
        }
        for x in roots {
            let bx = p.b.eval(x, f);
            assert_ne!(bx, 0);
            let y = f.mul(p.a.eval(x, f), f.inv(bx));
            assert_eq!(f.mul(y, y), c.f.eval(x, f));
            sum = c.add(&sum, &c.place(x, y));
        }
        assert!(c.is_zero(&sum), "normalized principal identity");
        (p.common_degree > 0, true, p.removed)
    }

    #[test]
    fn normalized_curve_relations() {
        for g in 2..=4 {
            let (mut n, mut common, mut split, mut recovered, mut removed) = (0, 0, 0, 0, 0);
            for p in [101, 127] {
                for d in 1..=3 {
                    let c = (1..60).find_map(|k| thread_curve(p, k, g)).unwrap();
                    let fb = factor_base(&c);
                    let m = g + 2 * d;
                    for offset in [0, 3, 6, 9] {
                        let pts = &fb[offset..offset + m];
                        let family = SignedFamily::new(g, d, pts, &c.fp);
                        for (s, (a, b)) in family.walsh(&c.fp).into_iter().enumerate() {
                            let norm = a
                                .mul(&a, &c.fp)
                                .sub(&c.f.mul(&b.mul(&b, &c.fp), &c.fp), &c.fp);
                            let (is_common, is_split, k) = check_primitive(&c, pts, s, a, b);
                            if let Some(z) = primitive(&c, pts, a, b) {
                                assert!(
                                    z.q.deg()
                                        <= norm.deg()
                                            - 2 * z.common_degree as isize
                                            - (m - z.removed) as isize
                                );
                            }
                            n += 1;
                            common += is_common as usize;
                            split += is_split as usize;
                            recovered += (is_common && is_split) as usize;
                            removed += k;
                        }
                    }
                }
            }
            assert!(recovered > 0, "vacuous recovered-common-factor gate");
            println!("primitive g={g} cases={n} common={common} split_group_checks={split} recovered_common_split={recovered} removed_anchor_occurrences={removed}");
        }
    }

    #[test]
    fn vertical_and_repeated_factor_controls() {
        let f = Fp::new(101);
        let c = Curve::new(f, Poly::from_coeffs(&[0, 100, 0, 0, 0, 1]));
        let at = Poly::x_minus(0, &f);
        let away = Poly::x_minus(2, &f);
        for h in [at, at.mul(&at, &f)] {
            let pts = [(0, 0), (1, 0), (100, 0), (10, 0)];
            let z = primitive(&c, &pts, Poly::zero(), h).unwrap();
            assert_eq!(z.keep_mask, 0b1110);
            assert_eq!(z.removed, 1);
            assert!(check_primitive(&c, &pts, 0, Poly::zero(), h).1);
        }
        for h in [away, away.mul(&away, &f)] {
            let pts = [(0, 0), (1, 0), (100, 0)];
            let z = primitive(&c, &pts, Poly::zero(), h).unwrap();
            assert_eq!(z.keep_mask, 0b111);
            assert_eq!(z.removed, 0);
            assert!(check_primitive(&c, &pts, 0, Poly::zero(), h).1);
        }
        let z = primitive(&c, &[(0, 0)], at, Poly::zero()).unwrap();
        assert_eq!(z.keep_mask, 0);
        assert_eq!(z.q, Poly::one());
        assert!(primitive(&c, &[], Poly::zero(), Poly::zero()).is_none());
    }
}

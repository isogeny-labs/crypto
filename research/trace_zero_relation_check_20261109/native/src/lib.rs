#![allow(dead_code)]
// Reuse production arithmetic without building the entire root library.
#[path = "../../../../src/binary_ecc/f2m.rs"]
mod field;
#[path = "../../../../src/binary_ecc/operation_tally.rs"]
pub mod operation_tally;
#[cfg(test)]
mod binary_ecc {
    pub use crate::operation_tally;
}

use field::{F2mElement, IrreduciblePoly};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tau(BigInt, BigInt); // a+b*tau, tau^2+tau+2=0
impl Tau {
    fn add(&self, b: &Self) -> Self {
        Self(&self.0 + &b.0, &self.1 + &b.1)
    }
    fn sub(&self, b: &Self) -> Self {
        Self(&self.0 - &b.0, &self.1 - &b.1)
    }
    fn mul(&self, b: &Self) -> Self {
        Self(
            &self.0 * &b.0 - 2 * &self.1 * &b.1,
            &self.0 * &b.1 + &self.1 * &b.0 - &self.1 * &b.1,
        )
    }
    fn norm(&self) -> BigInt {
        &self.0 * &self.0 - &self.0 * &self.1 + 2 * &self.1 * &self.1
    }
}
fn cardinality(n: u32) -> BigInt {
    let (mut t0, mut t1) = (BigInt::from(2), BigInt::from(-1));
    for _ in 1..n {
        let t = -&t1 - 2 * &t0;
        t0 = t1;
        t1 = t;
    }
    (BigInt::one() << n) + 1 - t1
}
fn order_two_mod(n: u32) -> u32 {
    let mut v = 2 % n;
    let mut k = 1;
    while v != 1 {
        v = v * 2 % n;
        k += 1;
        assert!(k < n);
    }
    k
}
fn modulus(n: u32) -> IrreduciblePoly {
    IrreduciblePoly {
        degree: n,
        low_terms: match n {
            31 => vec![0, 3],
            53 => vec![0, 1, 2, 6],
            83 => vec![0, 1, 2, 45],
            131 => vec![0, 1, 2, 13],
            _ => panic!("unfrozen field"),
        },
    }
}
fn trace(x: &F2mElement, f: &IrreduciblePoly) -> bool {
    let mut s = F2mElement::zero(f.degree);
    let mut p = x.clone();
    for _ in 0..f.degree {
        s = s.add(&p);
        p = p.square(f);
    }
    assert!(s.is_zero() || s == F2mElement::one(f.degree));
    !s.is_zero()
}
fn poly_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        while !a.is_zero() && a.bits() >= b.bits() {
            let shift = (a.bits() - b.bits()) as usize;
            a ^= &b << shift;
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}
fn certify_prime_degree_field(f: &IrreduciblePoly) {
    let n = f.degree;
    assert!((2..n).all(|d| n % d != 0));
    let mut polynomial = BigUint::one() << n;
    for &i in &f.low_terms {
        polynomial ^= BigUint::one() << i;
    }
    let z = F2mElement::from_bit_positions(&[1], n);
    let mut power = z.clone();
    for _ in 0..n {
        power = power.square(f);
    }
    assert_eq!(power, z);
    let z2_minus_z = z.square(f).add(&z).to_biguint();
    assert_eq!(poly_gcd(z2_minus_z, polynomial), BigUint::one());
}
fn trace_kernel_basis(n: u32, ell: u32) -> Vec<F2mElement> {
    let f = modulus(n);
    let basis: Vec<_> = (0..ell)
        .map(|j| F2mElement::from_bit_positions(&[j], n))
        .collect();
    let Some(pivot) = basis.iter().position(|b| trace(b, &f)) else {
        return basis;
    };
    basis
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if i == pivot {
                None
            } else if trace(b, &f) {
                Some(b.add(&basis[pivot]))
            } else {
                Some(b.clone())
            }
        })
        .collect()
}
fn choose(n: u32, k: u32) -> BigUint {
    let mut v = BigUint::one();
    for i in 1..=k {
        v = v * BigUint::from(n - k + i) / BigUint::from(i);
    }
    v
}
#[derive(Clone, Copy)]
struct Toy {
    n: u32,
    polynomial: u32,
}
impl Toy {
    fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut c = 0;
        while b != 0 {
            if b & 1 != 0 {
                c ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & (1 << self.n) != 0 {
                a ^= self.polynomial;
            }
        }
        c
    }
    fn sq(self, a: u32) -> u32 {
        self.mul(a, a)
    }
    fn inv(self, a: u32) -> u32 {
        assert!(a != 0);
        let mut r = 1;
        let mut b = a;
        let mut e = (1 << self.n) - 2;
        while e != 0 {
            if e & 1 != 0 {
                r = self.mul(r, b);
            }
            b = self.sq(b);
            e >>= 1;
        }
        r
    }
    fn tr(self, a: u32) -> u32 {
        let (mut s, mut p) = (0, a);
        for _ in 0..self.n {
            s ^= p;
            p = self.sq(p);
        }
        assert!(s <= 1);
        s
    }
    fn add(self, p: Point, q: Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return p.or(q);
        };
        if x == u && (y != v || x == 0) {
            return None;
        }
        let l = if x == u {
            x ^ self.mul(y, self.inv(x))
        } else {
            self.mul(y ^ v, self.inv(x ^ u))
        };
        let xr = self.sq(l) ^ l ^ x ^ u;
        let yr = if x == u {
            self.sq(x) ^ self.mul(l ^ 1, xr)
        } else {
            self.mul(l, x ^ xr) ^ xr ^ y
        };
        Some((xr, yr))
    }
    fn point_trace(self, p: Point) -> Point {
        let (mut out, mut power) = (None, p);
        for _ in 0..self.n {
            out = self.add(out, power);
            power = power.map(|(x, y)| (self.sq(x), self.sq(y)));
        }
        out
    }
    fn points(self) -> Vec<Point> {
        let mut out = vec![None];
        for x in 0..1 << self.n {
            for y in 0..1 << self.n {
                if self.sq(y) ^ self.mul(x, y) == self.mul(self.sq(x), x) ^ 1 {
                    out.push(Some((x, y)));
                }
            }
        }
        out
    }
}
type Point = Option<(u32, u32)>;

#[cfg(test)]
mod checks {
    use super::*;
    #[test]
    fn tau_norm_and_cofactor_projection_are_exact() {
        let one = Tau(1.into(), 0.into());
        let tau = Tau(0.into(), 1.into());
        let bar = Tau((-1).into(), (-1).into());
        assert_eq!(tau.sub(&one), bar.mul(&bar));
        assert_eq!(tau.sub(&one).norm(), 4.into());
        for n in [31, 53, 83, 131] {
            let (mut power, mut sum) = (one.clone(), Tau(0.into(), 0.into()));
            for _ in 0..n {
                sum = sum.add(&power);
                power = power.mul(&tau);
            }
            assert_eq!(tau.sub(&one).mul(&sum), power.sub(&one));
            assert_eq!(power.sub(&one).norm(), cardinality(n));
            assert_eq!(sum.norm() * 4, cardinality(n));
            if n == 83 {
                assert_eq!(sum.norm().to_string(), "2417851639230796216685689");
            }
            if n == 131 {
                assert_eq!(
                    sum.norm().to_string(),
                    "680564733841876926932320129493409985129"
                );
            }
            println!(
                "RING n={n} ord_n(2)={} dim_Res={} dim_trace_zero={} N_S={}",
                order_two_mod(n),
                n,
                n - 1,
                sum.norm()
            );
        }
    }
    #[test]
    fn exact_trace_coordinate_elimination_and_chained_layouts() {
        for n in [31, 53, 83, 131] {
            let f = modulus(n);
            certify_prime_degree_field(&f);
            let support: Vec<_> = (0..n)
                .filter(|&j| trace(&F2mElement::from_bit_positions(&[j], n), &f))
                .collect();
            let full_kernel = trace_kernel_basis(n, n);
            assert_eq!(full_kernel.len(), (n - 1) as usize);
            assert!(full_kernel.iter().all(|b| !trace(b, &f)));
            let ell = 17;
            let restricted = trace_kernel_basis(n, ell);
            assert_eq!(restricted.len(), (ell - 1) as usize);
            for m in [2u32, 3, 5] {
                let naive = m * ell + (m - 2) * n;
                let summands_only = m * (restricted.len() as u32) + (m - 2) * n;
                let affine_trace = m * (restricted.len() as u32) + (m - 2) * (n - 1);
                assert_eq!(naive - affine_trace, 2 * m - 2);
                println!("LAYOUT n={n} m={m} ell={ell} naive_bits={naive} summand_trace_bits={summands_only} also_affine_intermediates_bits={affine_trace} S3_component_rows={} degree_bound={} field_trace_support={support:?}",
                    (m-1)*n,if m==2 {2}else{3});
            }
        }
    }
    #[test]
    fn point_trace_and_field_trace_are_different_and_same_base_is_retained() {
        for f in [
            Toy {
                n: 3,
                polynomial: 0b1011,
            },
            Toy {
                n: 5,
                polynomial: 0b100101,
            },
            Toy {
                n: 7,
                polynomial: 0b10000011,
            },
            Toy {
                n: 9,
                polynomial: 0b1000010001,
            },
        ] {
            let points = f.points();
            assert_eq!(BigInt::from(points.len()), cardinality(f.n));
            let kernel: Vec<_> = points
                .iter()
                .copied()
                .filter(|&p| f.point_trace(p).is_none())
                .collect();
            let field_kernel: Vec<_> = points
                .iter()
                .copied()
                .filter(|p| p.map(|(x, _)| f.tr(x) == 0).unwrap_or(true))
                .collect();
            assert_eq!(kernel.len() * 4, points.len());
            assert_eq!(field_kernel.len() * 2, points.len());
            let image: std::collections::BTreeSet<_> = points
                .iter()
                .map(|&p| {
                    let fp = p.map(|(x, y)| (f.sq(x), f.sq(y)));
                    f.add(fp, p.map(|(x, y)| (x, y ^ x)))
                })
                .collect();
            assert_eq!(image, kernel.iter().copied().collect());
            assert_eq!(
                points
                    .iter()
                    .filter(|&&p| p.map(|(x, y)| x <= 1 && y <= 1).unwrap_or(true))
                    .count(),
                4
            );
            for &p in &points {
                assert!(f
                    .point_trace(p)
                    .map(|(x, y)| x <= 1 && y <= 1)
                    .unwrap_or(true));
                // Compare the independent word field against production arithmetic.
                if let Some((x, _)) = p {
                    let irr = IrreduciblePoly {
                        degree: f.n,
                        low_terms: (0..f.n).filter(|j| f.polynomial & (1 << j) != 0).collect(),
                    };
                    assert_eq!(
                        trace(&F2mElement::from_biguint(&BigUint::from(x), f.n), &irr),
                        f.tr(x) != 0
                    );
                }
            }
            let base: Vec<_> = kernel
                .iter()
                .copied()
                .filter(|p| p.map(|(x, _)| x < 8).unwrap_or(false))
                .collect();
            let compressed: Vec<_> = kernel
                .iter()
                .copied()
                .filter(|p| p.map(|(x, _)| x < 8 && f.tr(x) == 0).unwrap_or(false))
                .collect();
            assert_eq!(base, compressed);
            println!("TOY n={} group_points={} point_trace_kernel={} field_trace_kernel={} same_prime_trace_base_points={}",
                f.n,points.len(),kernel.len(),field_kernel.len(),base.len());
        }
    }
    #[test]
    fn fixed_two_bit_factor_base_cannot_be_assumed_large_at_q_two() {
        // Two free F2 coordinates give <=4 abscissae and <=8 affine points.
        let b = 8;
        let m = 130;
        let max_multisets = choose(m + b - 1, b - 1);
        let subgroup =
            BigUint::parse_bytes(b"680564733841876926932320129493409985129", 10).unwrap();
        assert!((&max_multisets << 90usize) < subgroup);
        println!("GM q=2 n=131 summands=130 variables=260 equations=261 max_affine_base=8 max_unordered_multisets={} subgroup_order={} uniform_success_upper_bound_below_2^-90=true",
            max_multisets,subgroup);
        // This is a group counting upper bound, not a measured solver cost.
    }
}

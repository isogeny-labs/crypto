//! Experimental NTT adapter over original native field and evaluator code.
use num_bigint::BigUint;
use std::sync::Arc;
use vieta_coset_lab::vieta_coset::PrimeField;
use vieta_shared_fp::fp_backend::PrimeField as Backend;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoded<E>(E);
#[derive(Clone, Debug)]
pub struct SharedField<F: Backend>(
    Arc<F>,
    bool,
    std::rc::Rc<std::cell::RefCell<ntt::Tables<F::Elem>>>,
);
mod ntt;
impl<F: Backend> SharedField<F> {
    pub fn new(field: F) -> Self {
        Self(Arc::new(field), false, Default::default())
    }
    pub fn with_ntt(mut self) -> Self {
        self.1 = true;
        self
    }
    pub fn transform_products(&self) -> u64 {
        self.2.borrow().products
    }
    pub fn from_biguint(&self, x: &BigUint) -> Encoded<F::Elem> {
        Encoded(self.0.from_biguint(x))
    }
}
impl<F: Backend> PrimeField for SharedField<F>
where
    F::Elem: Ord,
{
    type E = Encoded<F::Elem>;
    fn reset_polynomial_cache(&self) {
        *self.2.borrow_mut() = Default::default();
    }
    fn convolution(
        &self,
        a: &[Self::E],
        b: &[Self::E],
        limits: &vieta_coset_lab::vieta_coset::Limits,
    ) -> vieta_coset_lab::vieta_coset::Result<
        Option<(Vec<Self::E>, vieta_coset_lab::vieta_coset::Work)>,
    > {
        if !self.1 || a.len().min(b.len()) < 128 {
            return Ok(None);
        }
        ntt::convolution(&*self.0, &mut self.2.borrow_mut(), a, b, limits).map(Some)
    }
    fn modulus(&self) -> BigUint {
        self.0.modulus().clone()
    }
    fn element(&self, n: u64) -> Self::E {
        Encoded(self.0.from_u64(n))
    }
    #[inline(always)]
    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Encoded(self.0.add(a.0, b.0))
    }
    #[inline(always)]
    fn sub(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Encoded(self.0.sub(a.0, b.0))
    }
    #[inline(always)]
    fn mul(&self, a: &Self::E, b: &Self::E) -> Self::E {
        Encoded(self.0.mul(a.0, b.0))
    }
    fn hex(&self, a: &Self::E) -> String {
        self.0.to_biguint(a.0).to_str_radix(16)
    }
    fn canonical(&self, a: &Self::E) -> bool {
        self.0.to_biguint(a.0) < *self.0.modulus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vieta_coset_lab::vieta_coset::*;
    use vieta_shared_fp::fp_backend::{Fp224, Fp64};
    fn physical<F: PrimeField>(f: &F, rs: Vec<Relation<F::E>>) -> Vec<[u64; 4]> {
        let mut v: Vec<_> = rs
            .into_iter()
            .map(|(a, b)| {
                [a.0, a.1, b.0, b.1].map(|x| u64::from_str_radix(&f.hex(&x), 16).unwrap())
            })
            .collect();
        v.sort();
        v
    }
    #[test]
    fn shared_representation_roundtrips_and_edge_products_match_reference() {
        for p in [17, 65537, 4179340454199820289, 9223372036854775783] {
            let r = WordField(p);
            let f = SharedField::new(Fp64::new(p));
            let edge = [0, 1, 2, p - 2, p - 1, u64::MAX];
            for a in edge {
                for b in edge {
                    let (a, b) = (a % p, b % p);
                    let (x, y) = (f.element(a), f.element(b));
                    assert_eq!(f.hex(&x), r.hex(&a));
                    assert_eq!(f.hex(&f.add(&x, &y)), r.hex(&r.add(&a, &b)));
                    assert_eq!(f.hex(&f.sub(&x, &y)), r.hex(&r.sub(&a, &b)));
                    assert_eq!(f.hex(&f.mul(&x, &y)), r.hex(&r.mul(&a, &b)));
                }
            }
            let mut state = 0x53504545445550u64;
            for _ in 0..10000 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let a = state % p;
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let b = state % p;
                assert_eq!(
                    f.hex(&f.mul(&f.element(a), &f.element(b))),
                    r.hex(&r.mul(&a, &b))
                );
            }
        }
    }
    fn complete<F: PrimeField + Clone>(f: F) {
        let r = Arithmetic::new(WordField(17), Limits::default()).unwrap();
        let e = Curve { a: 2, b: 2 };
        let c = r.coset(4, 1).unwrap();
        let base = r.factor_base(&e, &c).unwrap();
        let mut targets = vec![None];
        for x in 0..17 {
            for y in 0..17 {
                if r.on_curve(&e, &Some((x, y))) {
                    targets.push(Some((x, y)));
                }
            }
        }
        for mode in [
            PolynomialBackend::Schoolbook,
            PolynomialBackend::KaratsubaNewton,
            PolynomialBackend::HalfGcd,
        ] {
            let a = Arithmetic::new(f.clone(), Limits::default())
                .unwrap()
                .with_polynomials(mode);
            let ec = Curve {
                a: f.element(2),
                b: f.element(2),
            };
            let co = a.coset(4, f.element(1)).unwrap();
            let map = RationalMap {
                numerator: vec![f.element(1), f.element(0), f.element(1)],
                denominator: vec![f.element(0), f.element(2)],
            };
            let tower = a
                .certify_tower(&[map.clone(), map], f.element(4), 42)
                .unwrap();
            let tb = a.tower_factor_base(&ec, &tower).unwrap();
            for target in &targets {
                let encoded = target.map(|(x, y)| (f.element(x), f.element(y)));
                assert_eq!(
                    physical(&f, a.solve(&ec, &co, &encoded, 19).unwrap()),
                    physical(&WordField(17), r.lookup_scalar(&e, &base, target).unwrap())
                );
                assert_eq!(
                    physical(&f, a.solve_tower(&ec, &tower, &encoded, 19).unwrap()),
                    physical(&f, a.lookup_scalar(&ec, &tb, &encoded).unwrap())
                );
            }
        }
    }
    #[test]
    fn all_toy_prime_curve_targets_and_tower_relations_survive_montgomery_ordering() {
        complete(SharedField::new(Fp64::new(17)));
    }
    #[test]
    fn same_adapter_supports_existing_fixed_limb_wide_field() {
        complete(SharedField::new(Fp224::new(&BigUint::from(17u8))));
    }
}

#[cfg(test)]
mod wide_edges {
    use super::*;
    use num_traits::One;
    use vieta_coset_lab::vieta_coset::WideField;
    use vieta_shared_fp::fp_backend::Fp224;
    #[test]
    fn actual_p224_carry_edges_and_random_products_match_biguint() {
        let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
        let f = SharedField::new(Fp224::new(&p));
        let r = WideField(p.clone());
        let edge = [
            BigUint::from(0u8),
            BigUint::one(),
            &p - 1u8,
            &p - 2u8,
            BigUint::one() << 192usize,
            BigUint::one() << 223usize,
        ];
        for a in &edge {
            for b in &edge {
                let (x, y) = (f.from_biguint(a), f.from_biguint(b));
                assert_eq!(f.hex(&f.mul(&x, &y)), r.hex(&r.mul(a, b)));
                assert_eq!(f.hex(&f.add(&x, &y)), r.hex(&r.add(a, b)));
                assert_eq!(f.hex(&f.sub(&x, &y)), r.hex(&r.sub(a, b)));
            }
        }
        let mut state = 0x503232345f454447u64;
        for _ in 0..1000 {
            let mut next = || {
                let mut bytes = vec![];
                for _ in 0..4 {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    bytes.extend_from_slice(&state.to_le_bytes());
                }
                BigUint::from_bytes_le(&bytes) % &p
            };
            let (a, b) = (next(), next());
            assert_eq!(
                f.hex(&f.mul(&f.from_biguint(&a), &f.from_biguint(&b))),
                r.hex(&r.mul(&a, &b))
            );
        }
    }
}

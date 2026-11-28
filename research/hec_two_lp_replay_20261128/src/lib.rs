#[path = "../../../src/utils/mod.rs"]
pub mod utils;

pub mod cryptanalysis {
    #[path = "../../../../src/cryptanalysis/bitops.rs"]
    pub mod bitops;
    pub mod curve_traits {
        pub mod arith {
            use num_bigint::BigUint;
            pub fn is_prime(p: &BigUint) -> bool {
                let digits = p.to_u64_digits();
                digits.len() == 1 && crate::prime_hyperelliptic::fp2::is_prime_u64(digits[0])
            }
        }
    }
    pub mod ec_index_calculus {
        use num_bigint::BigUint;
        pub struct ModNSolution {
            pub values: Vec<BigUint>,
            pub determined: Vec<bool>,
        }
        pub fn sqrt_mod_p(n: &BigUint, p: &BigUint) -> Option<BigUint> {
            let prime = p.to_u64_digits().first().copied()?;
            let value = (n % p).to_u64_digits().first().copied().unwrap_or(0);
            (0..prime)
                .find(|&y| y * y % prime == value)
                .map(BigUint::from)
        }
        pub fn gaussian_eliminate_mod_n_particular(
            _matrix: &mut [Vec<BigUint>],
            _rhs: &mut [BigUint],
            _n: &BigUint,
        ) -> Option<ModNSolution> {
            None
        }
    }
    #[path = "../../../../src/cryptanalysis/hec_two_large_prime_graph.rs"]
    mod hec_two_large_prime_graph;
    #[path = "../../../../src/cryptanalysis/hyperelliptic_index_calculus.rs"]
    pub mod hyperelliptic_index_calculus;
}

pub mod prime_hyperelliptic {
    #[path = "../../../../src/prime_hyperelliptic/curve.rs"]
    pub mod curve;
    #[path = "../../../../src/prime_hyperelliptic/fp2.rs"]
    pub mod fp2;
    #[path = "../../../../src/prime_hyperelliptic/fp_poly.rs"]
    pub mod fp_poly;
    #[path = "../../../../src/prime_hyperelliptic/ops.rs"]
    pub mod ops;
    pub use curve::{brute_force_jac_order_via_lpoly, HyperellipticCurveP, MumfordDivisorP};
    pub use fp_poly::FpPoly;
}

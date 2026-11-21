//! Established cofactor inversion for one validated original cubic context.
//! The frozen reference is compiled unchanged. Counters are its declared units,
//! not calibrated runtime, instruction, or complete ECDLP costs.

use original_kf223::{bump_mul, Kf, K, KZERO};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    WrongFieldParameters,
    NoncanonicalInput,
    ZeroInput,
    ZeroNorm,
}

/// Validation is separate from inversion and is not a measured setup result.
pub struct Cofactor223<'a> {
    original: &'a Kf,
}

impl<'a> Cofactor223<'a> {
    pub fn new(original: &'a Kf) -> Result<Self, Rejection> {
        if original.q() != 223
            || original.c != 3
            || original.w != 183
            || original.w2 != 39
            || original.size != 11_089_567
        {
            return Err(Rejection::WrongFieldParameters);
        }
        Ok(Self { original })
    }

    fn validate_input(a: &K) -> Result<(), Rejection> {
        if a.iter().any(|coefficient| *coefficient >= 223) {
            Err(Rejection::NoncanonicalInput)
        } else {
            Ok(())
        }
    }

    /// P=(a²-3bc,3c²-ab,b²-ac), with every product/subtraction reduced by
    /// the unchanged original prime-field routines. Zero has zero cofactor.
    pub fn conjugate_cofactor(&self, a: &K) -> Result<K, Rejection> {
        Self::validate_input(a)?;
        Ok(self.cofactor_unchecked(a))
    }

    fn cofactor_unchecked(&self, a: &K) -> K {
        let f = &self.original.fp;
        let aa = f.mul(a[0], a[0]);
        let bc = f.mul(a[1], a[2]);
        let cc = f.mul(a[2], a[2]);
        let ab = f.mul(a[0], a[1]);
        let bb = f.mul(a[1], a[1]);
        let ac = f.mul(a[0], a[2]);
        let d_bc = f.mul(self.original.c, bc);
        let d_cc = f.mul(self.original.c, cc);
        [f.sub(aa, d_bc), f.sub(d_cc, ab), f.sub(bb, ac)]
    }

    pub fn inverse(&self, a: &K) -> Result<K, Rejection> {
        Self::validate_input(a)?;
        if *a == KZERO {
            return Err(Rejection::ZeroInput);
        }
        let p = self.cofactor_unchecked(a);
        // Preserve the original literal norm block and declared charge. It
        // charges five despite four explicit multiplication expressions and
        // does not charge its additions; do not silently recalibrate it.
        let n = {
            bump_mul(5);
            let q = self.original.fp.q;
            let t0 = a[0] * p[0] % q;
            let t3 = (a[1] * p[2] + a[2] * p[1]) % q;
            (t0 + self.original.c * t3) % q
        };
        if n == 0 {
            return Err(Rejection::ZeroNorm);
        }
        let ni = self.original.fp.inv(n);
        Ok(self.original.mul_fq(&p, ni))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use original_kf223::{snap, Cnt, KONE};

    fn counters(delta: Cnt) -> (u64, u64, u64) {
        (delta.mul, delta.inv, delta.add)
    }

    fn all_frozen_fixtures() -> Vec<K> {
        let mut inputs = Vec::new();
        for line in include_str!("../fixtures-06.tsv").lines() {
            let values: Vec<u64> = line
                .split_whitespace()
                .map(|value| value.parse().expect("canonical public fixture coefficient"))
                .collect();
            assert_eq!(values.len(), 3);
            inputs.push([values[0], values[1], values[2]]);
        }
        assert_eq!(inputs.len(), 15, "retain every preregistered fixture");
        inputs
    }

    #[test]
    fn all_frozen_inputs_match_original_and_both_inverse_identities() {
        let k = Kf::new(223);
        let candidate = Cofactor223::new(&k).expect("exact original context");
        for a in all_frozen_fixtures() {
            let before = snap();
            let conjugate1 = k.frob(&a);
            let conjugate2 = k.frob(&conjugate1);
            let reference_p = k.mul(&conjugate1, &conjugate2);
            let reference_p_counts = counters(snap().since(&before));
            let before = snap();
            let candidate_p = candidate.conjugate_cofactor(&a).unwrap();
            let candidate_p_counts = counters(snap().since(&before));
            assert_eq!(candidate_p, reference_p, "cofactor for {a:?}");
            assert_eq!(reference_p_counts, (15, 0, 8));
            assert_eq!(candidate_p_counts, (8, 0, 3));

            let before = snap();
            let reference_inverse = k.inv(&a);
            let reference_counts = counters(snap().since(&before));
            let before = snap();
            let candidate_inverse = candidate.inverse(&a).unwrap();
            let candidate_counts = counters(snap().since(&before));
            assert_eq!(candidate_inverse, reference_inverse, "inverse for {a:?}");
            assert_eq!(reference_counts, (23, 1, 8));
            assert_eq!(candidate_counts, (16, 1, 3));
            // Verification work lies outside both captured counter intervals.
            assert_eq!(k.mul(&a, &candidate_inverse), KONE);
            assert_eq!(k.mul(&candidate_inverse, &a), KONE);
            let norm_product = k.mul(&a, &candidate_p);
            assert!(k.is_fq(&norm_product));
            assert_ne!(norm_product[0], 0);
            println!(
                "development_inverse\tinput={a:?}\toutput={candidate_inverse:?}\treference={reference_counts:?}\tcandidate={candidate_counts:?}\tnorm={norm_product:?}"
            );
        }
    }

    #[test]
    fn rejects_invalid_domain_without_replacing_original_behavior() {
        let k = Kf::new(223);
        let candidate = Cofactor223::new(&k).unwrap();
        assert_eq!(candidate.inverse(&KZERO), Err(Rejection::ZeroInput));
        assert_eq!(candidate.conjugate_cofactor(&KZERO), Ok(KZERO));
        for a in [[223, 0, 0], [0, 223, 0], [0, 0, 223], [u64::MAX, 1, 0]] {
            assert_eq!(candidate.inverse(&a), Err(Rejection::NoncanonicalInput));
            assert_eq!(
                candidate.conjugate_cofactor(&a),
                Err(Rejection::NoncanonicalInput)
            );
        }
        let changes: [fn(&mut Kf); 5] = [
            |context| context.fp.q = 222,
            |context| context.c = 2,
            |context| context.w = 1,
            |context| context.w2 = 1,
            |context| context.size = 1,
        ];
        for change in changes {
            let mut altered = k.clone();
            change(&mut altered);
            assert_eq!(
                Cofactor223::new(&altered).err(),
                Some(Rejection::WrongFieldParameters)
            );
        }
        println!("expected_original_zero_inverse_panic follows");
        assert!(std::panic::catch_unwind(|| k.inv(&KZERO)).is_err());
    }

    #[test]
    fn wrong_cofactor_sign_control_is_detected() {
        let k = Kf::new(223);
        let candidate = Cofactor223::new(&k).unwrap();
        let a = [1, 1, 1];
        let correct = candidate.conjugate_cofactor(&a).unwrap();
        let wrong_last = k.fp.add(k.fp.mul(a[1], a[1]), k.fp.mul(a[0], a[2]));
        let mutant = [correct[0], correct[1], wrong_last];
        assert_ne!(mutant, correct);
        assert!(!k.is_fq(&k.mul(&a, &mutant)));
    }
}

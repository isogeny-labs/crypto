//! Exact small algebra controls. No ECDLP worker or performance experiment.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_certificate_check.rs"]
pub mod independent_check;
#[allow(dead_code, clippy::too_many_arguments, clippy::type_complexity)] // Unchanged legacy fixture/signature.
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
mod principal_relation;
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
pub mod quadratic_pair_graph;
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
pub mod quadratic_pair_relation;

/// The controls are unit tests: `cargo test --example <this example>`.
#[allow(dead_code)]
fn main() {}

#[cfg(test)]
pub(crate) mod tests {
    use super::{principal_relation as single, quadratic_pair_relation as pair};
    use crypto_lib::cryptanalysis::{
        jv_cover::{BaseEl, Div, Fld, Fq, Hyp, E2},
        jv_sieve::FpRing,
    };
    use rand::{rngs::StdRng, SeedableRng};
    fn smooth(f: &Fq, h: &[E2]) -> bool {
        let mut a = h.to_vec();
        let mut b: Vec<_> = h
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| f.scale(x, i as u64))
            .collect();
        while !b.is_empty() {
            if b.len() == 1 {
                return b[0] != E2::ZERO;
            }
            let inverse = f.inv(b.last().unwrap());
            let monic: Vec<_> = b.iter().map(|x| f.mul(x, &inverse)).collect();
            let r = single::modulo(f, &a, &monic).unwrap();
            a = b;
            b = r;
        }
        a.len() == 1
    }
    pub(crate) type Fixture = (
        Fq,
        Vec<E2>,
        Vec<BaseEl>,
        Vec<E2>,
        Vec<E2>,
        Vec<u64>,
        Vec<(usize, i8)>,
        Vec<Vec<u64>>,
    );
    pub(crate) fn hand() -> Fixture {
        let f = Fq::new(101);
        let ring = FpRing::new(101);
        let factors = vec![vec![98, 0, 1], vec![99, 0, 1]];
        assert!(factors
            .iter()
            .all(|u| pair::is_irreducible_quadratic(&ring, u)));
        let mut polynomial = ring.mul(&factors[0], &factors[1]);
        for x in 1..=5 {
            polynomial = ring.mul(&polynomial, &[101 - x, 1]);
        }
        let norm = ring.scale(&polynomial, 100);
        let a0 = f.from_fp(35);
        assert_eq!(f.sq(&a0), f.from_fp(norm[0]));
        let a1 = f.mul(&f.from_fp(norm[1]), &f.inv(&f.scale(&a0, 2)));
        let a = vec![a0, a1, E2([1, 3])];
        let c = vec![E2::ZERO, E2::ONE];
        let numerator = single::sub(&f, &single::mul(&f, &a, &a), &single::embed(&norm));
        assert_eq!(&numerator[..2], &[E2::ZERO, E2::ZERO]);
        let h = numerator[2..].to_vec();
        assert_eq!(h.len(), 8);
        assert_eq!(h[7], E2::ONE);
        assert!(smooth(&f, &h));
        let base = (1..=5)
            .map(|x| {
                let y = f.neg(&f.mul(&single::eval(&f, &a, x), &f.inv(&f.from_fp(x))));
                BaseEl {
                    x,
                    y,
                    d: Div {
                        u: vec![f.neg(&f.from_fp(x)), E2::ONE],
                        v: vec![y],
                    },
                }
            })
            .collect();
        let terms = (0..5).map(|i| (i, 1)).collect();
        (f, h, base, a, c, norm, terms, factors)
    }
    #[test]
    fn five_small_two_irreducible_principal_classes_pass_direct_cantor() {
        let (f, h, base, a, c, norm, terms, factors) = hand();
        let jac = Hyp {
            f: &f,
            h: h.clone(),
            g: 3,
        };
        let v = pair::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &factors).unwrap();
        assert_eq!(v.certificate().remainders.len(), 2);
        assert!(pair::replay_group(&jac, &base, v.certificate()));
        assert_eq!(v.large_terms().len(), 2);
        let domain = super::independent_check::Domain::new(
            f.p,
            f.w,
            single::encode(&h),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        assert_eq!(
            domain
                .check(&serde_json::to_value(v.certificate()).unwrap())
                .unwrap()["quadratic_classes"],
            2
        );
    }
    #[test]
    fn old_single_remainder_admission_is_reused_exactly() {
        let (f, h, base, a, c, norm, terms, tail) = single::tests::hand();
        let ring = FpRing::new(f.p);
        let factors = vec![ring.monic(&tail)];
        let old = single::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail)
            .unwrap()
            .into_certificate();
        let new = pair::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &factors)
            .unwrap()
            .into_certificate();
        assert_eq!(new.terms, old.terms);
        assert_eq!(
            new.remainders,
            old.remainder.into_iter().collect::<Vec<_>>()
        );
        assert_eq!(new.a, old.a);
        assert_eq!(new.c, old.c);
        assert_eq!(new.norm, old.norm);
        let domain = super::independent_check::Domain::new(
            f.p,
            f.w,
            single::encode(&h),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        domain.check(&serde_json::to_value(new).unwrap()).unwrap();
    }
    #[test]
    fn splitter_admits_only_exact_distinct_irreducible_quadratics() {
        for p in [5u64, 7, 101, 1009] {
            let ring = FpRing::new(p);
            let quads: Vec<_> = (0..p)
                .flat_map(|a| (0..p.min(7)).map(move |b| vec![b, a, 1]))
                .filter(|u| pair::is_irreducible_quadratic(&ring, u))
                .take(7)
                .collect();
            assert!(quads.len() >= 2);
            let mut successes = 0;
            for i in 0..quads.len() {
                for j in i + 1..quads.len() {
                    let tail = ring.scale(&ring.mul(&quads[i], &quads[j]), p - 1);
                    let mut rng = StdRng::seed_from_u64(389 + i as u64 * 17 + j as u64);
                    let r = pair::split_quartic(
                        &ring,
                        &tail,
                        pair::SplitOptions { max_attempts: 16 },
                        &mut rng,
                    )
                    .unwrap();
                    super::independent_check::check_split(
                        &serde_json::to_value(&r).unwrap(),
                        &tail,
                        16,
                    )
                    .unwrap();
                    if let Some(mut factors) = r.factors {
                        successes += 1;
                        factors.sort();
                        let mut expected = [quads[i].clone(), quads[j].clone()];
                        expected.sort();
                        assert_eq!(factors, expected);
                    } else {
                        assert_eq!(r.attempts.len(), 16);
                    }
                }
            }
            assert!(successes > 0, "nonempty positive split control over Fp{p}");
        }
    }
    #[test]
    fn changed_norm_sign_class_or_factor_is_rejected() {
        let (f, h, base, a, c, norm, terms, factors) = hand();
        let mut bad = norm.clone();
        bad[0] = (bad[0] + 1) % f.p;
        assert!(pair::check_equation(&f, &h, &base, &a, &c, &bad, &terms, &factors).is_err());
        let mut bad = terms.clone();
        bad[0].1 = -1;
        assert!(pair::check_equation(&f, &h, &base, &a, &c, &norm, &bad, &factors).is_err());
        let bad = vec![factors[0].clone(), factors[0].clone()];
        assert!(pair::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &bad).is_err());
        let mut row = pair::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &factors)
            .unwrap()
            .into_certificate();
        let jac = Hyp {
            f: &f,
            h: h.clone(),
            g: 3,
        };
        row.remainders[0].coefficient *= -1;
        assert!(!pair::replay_group(&jac, &base, &row));
        let domain = super::independent_check::Domain::new(
            f.p,
            f.w,
            single::encode(&h),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        assert!(domain.check(&serde_json::to_value(row).unwrap()).is_err());
    }
    #[test]
    fn independent_checker_refuses_corrupted_cofactor_and_split_traces() {
        let (f, h, base, a, c, norm, terms, factors) = hand();
        let certificate = pair::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &factors)
            .unwrap()
            .into_certificate();
        let domain = super::independent_check::Domain::new(
            f.p,
            f.w,
            single::encode(&h),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        let raw = serde_json::to_value(certificate).unwrap();
        domain.check(&raw).unwrap();
        for pointer in [
            "/norm/0",
            "/a/0/0",
            "/terms/0/1",
            "/remainders/0/key/2",
            "/remainders/0/coefficient",
        ] {
            let mut bad = raw.clone();
            *bad.pointer_mut(pointer).unwrap() = serde_json::json!(999);
            assert!(domain.check(&bad).is_err(), "{pointer}");
        }
        let ring = FpRing::new(f.p);
        let tail = ring.mul(&factors[0], &factors[1]);
        let mut rng = StdRng::seed_from_u64(389);
        let report = pair::split_quartic(
            &ring,
            &tail,
            pair::SplitOptions { max_attempts: 16 },
            &mut rng,
        )
        .unwrap();
        assert!(report.factors.is_some());
        let raw = serde_json::to_value(report).unwrap();
        super::independent_check::check_split(&raw, &tail, 16).unwrap();
        for pointer in [
            "/normalized/0",
            "/squarefree_gcd/0",
            "/frobenius_p/0",
            "/attempts/0/gcd/0",
            "/factors/0/0",
            "/max_attempts",
        ] {
            let mut bad = raw.clone();
            *bad.pointer_mut(pointer).unwrap() = serde_json::json!(999);
            assert!(
                super::independent_check::check_split(&bad, &tail, 16).is_err(),
                "{pointer}"
            );
        }
        // Construct a mathematically valid refused trace to exercise the
        // independent refusal path, without claiming it was sampled by a worker.
        let mut refused = raw.clone();
        refused["max_attempts"] = serde_json::json!(8);
        refused["attempts"] = serde_json::json!(vec![
            serde_json::json!({"polynomial":[0,0,0,0],"gcd":[1]});
            8
        ]);
        refused["factors"] = serde_json::Value::Null;
        refused["refusal"] = serde_json::json!("quartic split attempt cap");
        assert_eq!(
            super::independent_check::check_split(&refused, &tail, 8).unwrap()
                ["attempt_cap_refusal"],
            true
        );
        refused["attempts"][7]["gcd"] = serde_json::json!([1, 1]);
        assert!(super::independent_check::check_split(&refused, &tail, 8).is_err());
    }
    #[test]
    fn independent_structural_refusal_prefixes_are_replayed() {
        let ring = FpRing::new(101);
        let mut rng = StdRng::seed_from_u64(389);
        let repeated = ring.mul(&[99, 0, 1], &[99, 0, 1]);
        let linear = ring.mul(&[99, 0, 1], &ring.mul(&[100, 1], &[99, 1]));
        for tail in [repeated, linear, vec![2, 0, 0, 0, 1]] {
            let report =
                pair::split_quartic(&ring, &tail, pair::SplitOptions::default(), &mut rng).unwrap();
            assert!(report.attempts.is_empty());
            assert!(report.refusal.is_some());
            super::independent_check::check_split(&serde_json::to_value(report).unwrap(), &tail, 8)
                .unwrap();
        }
    }
}

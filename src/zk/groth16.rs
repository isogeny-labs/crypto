//! **Groth16 zk-SNARK** (Groth 2016).
//!
//! The most-deployed pairing-based SNARK: 3 group elements proof size
//! (about 200 bytes), 1 pairing equation to verify, succinct
//! independent of circuit size.  Used by Zcash Sapling, Filecoin,
//! Tornado Cash, many other production zk systems.
//!
//! ## The pipeline
//!
//! 1. **R1CS** (Rank-1 Constraint System): encode the computation
//!    as a list of constraints `(<a_i, z>) · (<b_i, z>) = (<c_i, z>)`
//!    where `z = (1, x_pub, w_priv)` is the assignment vector.
//! 2. **QAP** (Quadratic Arithmetic Program): convert R1CS into
//!    polynomials `A_i(X), B_i(X), C_i(X)` via Lagrange
//!    interpolation over distinct evaluation points.  The witness
//!    is valid iff a single polynomial identity holds.
//! 3. **Trusted setup**: sample secrets `α, β, γ, δ, τ` and publish
//!    structured reference string (SRS).  Discard the toxic waste.
//! 4. **Prover**: produce `(A, B, C) ∈ G₁ × G₂ × G₁` from the witness
//!    and SRS.
//! 5. **Verifier**: check the Groth16 pairing equation:
//!
//!    ```text
//!    e(A, B) == e(α·G₁, β·G₂) · e(IC(x_pub), γ·G₂) · e(C, δ·G₂)
//!    ```
//!
//! ## What this module ships
//!
//! - [`R1cs`] — a minimal R1CS representation.
//! - [`Qap`] — the QAP derived from an R1CS.
//! - [`Groth16Srs`] — trusted-setup output (insecurely generated for
//!   testing).
//! - [`Groth16Proof`] — the 3-element SNARK proof.
//! - [`prove`] / [`verify`] — the prover and verifier.
//!
//! ## Worked example
//!
//! ```text
//! Statement: "I know x such that x² = 25"  (public output = 25)
//! R1CS:      x · x = 25  ⟹  one constraint
//! Witness:   x = 5 (or x = -5)
//! ```
//!
//! ## Honest scope
//!
//! - **Insecure setup**: we generate `α, β, γ, δ, τ` in-process for
//!   testing.  Production code MUST consume an MPC-generated SRS.
//! - **Educational implementation**: BigUint arithmetic, ~minutes
//!   to prove and verify.  Real Groth16 (with libsnark / arkworks)
//!   is milliseconds.
//! - **Toy constraint counts**: tests use circuits with 1–3
//!   constraints.  The algorithm scales to thousands, but the slow
//!   pairing makes large tests impractical here.
//!
//! ## Soundness & zero-knowledge notes
//!
//! - **SRS wire partition**: `g1_public_coeffs` covers only the public
//!   statement wires `0..=n_public` (divided by `γ`), and
//!   `g1_witness_coeffs` covers strictly the private witness wires
//!   `(n_public + 1)..n_vars` (divided by `δ`).  Emitting `/δ` terms
//!   for public wires would collapse Groth16's `γ`/`δ` separation and
//!   allow forging any public input from any valid proof.
//! - **Zero-knowledge blinding**: [`prove`] masks `A` and `B` with
//!   fresh scalars `r, s ←$ F_r` using `δ·G₁` and `δ·G₂` and folds the
//!   cross-terms into `C`.
//! - **Subgroup & canonical-scalar checks**: [`verify`] validates that
//!   all three proof points lie on their curve and in the prime-order
//!   `r`-subgroup and that every public input is canonical (`< r`).

use super::polynomial::{fr_add, fr_inv, fr_mul, fr_neg, fr_reduce, Poly};
use crate::bls12_381::fq::scalar_modulus;
use crate::bls12_381::g1::G1Point;
use crate::bls12_381::g2::G2Point;
use crate::bls12_381::pairing::pairing;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::rngs::OsRng;

/// A single R1CS constraint: `(a · z) · (b · z) = (c · z)` where
/// `a, b, c` are length-`n` coefficient vectors and `z` is the
/// assignment.
#[derive(Clone, Debug)]
pub struct Constraint {
    pub a: Vec<BigUint>,
    pub b: Vec<BigUint>,
    pub c: Vec<BigUint>,
}

/// A complete R1CS: list of constraints, plus the number of public
/// inputs (index range in `z`).
#[derive(Clone, Debug)]
pub struct R1cs {
    /// Vector length (= 1 + num_public + num_witness).
    pub n_vars: usize,
    /// Number of public inputs (excluding the constant `1` at index 0).
    pub n_public: usize,
    pub constraints: Vec<Constraint>,
}

impl R1cs {
    /// Check that `z` satisfies all constraints (`a·z`)(`b·z`) = `c·z`.
    pub fn is_satisfied(&self, z: &[BigUint]) -> bool {
        if z.len() != self.n_vars {
            return false;
        }
        for con in &self.constraints {
            let az = inner(&con.a, z);
            let bz = inner(&con.b, z);
            let cz = inner(&con.c, z);
            if fr_mul(&az, &bz) != cz {
                return false;
            }
        }
        true
    }
}

fn inner(coeffs: &[BigUint], z: &[BigUint]) -> BigUint {
    coeffs.iter().zip(z).fold(BigUint::zero(), |acc, (ci, zi)| {
        fr_add(&acc, &fr_mul(ci, zi))
    })
}

/// A QAP: polynomials `A_i(X), B_i(X), C_i(X)` for each variable
/// index `i ∈ [0, n_vars)`, plus the vanishing polynomial `Z(X)
/// = ∏ⱼ (X − ωⱼ)` over the chosen evaluation points.
#[derive(Clone, Debug)]
pub struct Qap {
    pub n_vars: usize,
    pub n_public: usize,
    pub a_polys: Vec<Poly>,
    pub b_polys: Vec<Poly>,
    pub c_polys: Vec<Poly>,
    pub z_poly: Poly,
    /// Evaluation points (one per constraint).
    pub points: Vec<BigUint>,
}

/// Convert R1CS to QAP using small distinct evaluation points
/// `1, 2, 3, …, m` (where `m = #constraints`).  Each `A_i(X)` is
/// the Lagrange interpolation through `(j, a_{j,i})` for constraint
/// `j` and variable `i`.
pub fn r1cs_to_qap(r1cs: &R1cs) -> Qap {
    let m = r1cs.constraints.len();
    let points: Vec<BigUint> = (1..=m as u64).map(BigUint::from).collect();

    let mut a_polys = Vec::with_capacity(r1cs.n_vars);
    let mut b_polys = Vec::with_capacity(r1cs.n_vars);
    let mut c_polys = Vec::with_capacity(r1cs.n_vars);
    for i in 0..r1cs.n_vars {
        let a_pts: Vec<(BigUint, BigUint)> = r1cs
            .constraints
            .iter()
            .enumerate()
            .map(|(j, con)| (points[j].clone(), con.a[i].clone()))
            .collect();
        let b_pts: Vec<(BigUint, BigUint)> = r1cs
            .constraints
            .iter()
            .enumerate()
            .map(|(j, con)| (points[j].clone(), con.b[i].clone()))
            .collect();
        let c_pts: Vec<(BigUint, BigUint)> = r1cs
            .constraints
            .iter()
            .enumerate()
            .map(|(j, con)| (points[j].clone(), con.c[i].clone()))
            .collect();
        a_polys.push(Poly::interpolate(&a_pts));
        b_polys.push(Poly::interpolate(&b_pts));
        c_polys.push(Poly::interpolate(&c_pts));
    }
    // Z(X) = ∏ (X − pt_j)
    let mut z = Poly::from_coeffs(vec![BigUint::one()]);
    for pt in &points {
        let factor = Poly::from_coeffs(vec![fr_neg(pt), BigUint::one()]);
        z = z.mul(&factor);
    }
    Qap {
        n_vars: r1cs.n_vars,
        n_public: r1cs.n_public,
        a_polys,
        b_polys,
        c_polys,
        z_poly: z,
        points,
    }
}

/// Insecurely-generated Groth16 SRS.  In production, this is the
/// output of a multi-party computation ceremony.
#[derive(Clone, Debug)]
pub struct Groth16Srs {
    /// Proving & verifying key components.
    pub g1_alpha: G1Point,
    pub g1_beta: G1Point,
    pub g1_delta: G1Point,
    pub g2_beta: G2Point,
    pub g2_delta: G2Point,
    pub g2_gamma: G2Point,
    /// `[τⁱ·G₁]` for i in 0..=max_deg.
    pub g1_tau_pow: Vec<G1Point>,
    /// `[τⁱ·G₂]` for i in 0..=max_deg.
    pub g2_tau_pow: Vec<G2Point>,
    /// Private-witness SRS elements strictly for `i ∈ (n_public + 1)..n_vars`:
    ///   `g1_witness_coeffs[i - (n_public + 1)] = (β·A_i(τ) + α·B_i(τ) + C_i(τ)) / δ · G₁`.
    pub g1_witness_coeffs: Vec<G1Point>,
    /// Public-input SRS elements strictly for `i ∈ 0..=n_public`:
    ///   `g1_public_coeffs[i] = (β·A_i(τ) + α·B_i(τ) + C_i(τ)) / γ · G₁`.
    pub g1_public_coeffs: Vec<G1Point>,
    /// `[τⁱ · Z(τ) / δ · G₁]` for H polynomial.
    pub g1_h_coeffs: Vec<G1Point>,
}

/// **INSECURE**: generate Groth16 SRS in-process.  For tests only.
pub fn groth16_insecure_setup(qap: &Qap) -> Groth16Srs {
    assert!(
        qap.n_public < qap.n_vars,
        "QAP must include the constant wire 1 plus n_public public wires"
    );
    let mut rng = OsRng;
    let r = scalar_modulus();
    let one = BigUint::one();
    let alpha = rng.gen_biguint_range(&one, &r);
    let beta = rng.gen_biguint_range(&one, &r);
    let gamma = rng.gen_biguint_range(&one, &r);
    let delta = rng.gen_biguint_range(&one, &r);
    let min_tau = BigUint::from((qap.points.len() as u64) + 1);
    let tau = rng.gen_biguint_range(&min_tau, &r);

    let g1 = G1Point::generator();
    let g2 = G2Point::generator();
    let max_poly_deg = qap
        .a_polys
        .iter()
        .chain(qap.b_polys.iter())
        .chain(qap.c_polys.iter())
        .filter_map(Poly::degree)
        .max()
        .unwrap_or(0);
    let z_deg = qap.z_poly.degree().unwrap_or(0);
    let max_deg = max_poly_deg.max(z_deg);

    let mut g1_tau_pow = Vec::with_capacity(max_deg + 1);
    let mut g2_tau_pow = Vec::with_capacity(max_deg + 1);
    let mut tau_pow = BigUint::one();
    for _ in 0..=max_deg {
        g1_tau_pow.push(g1.scalar_mul(&tau_pow));
        g2_tau_pow.push(g2.scalar_mul(&tau_pow));
        tau_pow = fr_mul(&tau_pow, &tau);
    }

    let g1_alpha = g1.scalar_mul(&alpha);
    let g1_beta = g1.scalar_mul(&beta);
    let g1_delta = g1.scalar_mul(&delta);
    let g2_beta = g2.scalar_mul(&beta);
    let g2_gamma = g2.scalar_mul(&gamma);
    let g2_delta = g2.scalar_mul(&delta);

    // Partition wires strictly:
    //   0..=n_public        -> divided by γ (verifying key IC)
    //   (n_public+1)..n_vars -> divided by δ (proving key witness)
    let gamma_inv = fr_inv(&gamma).unwrap();
    let delta_inv = fr_inv(&delta).unwrap();
    let n_witness = qap.n_vars - (qap.n_public + 1);
    let mut g1_public_coeffs = Vec::with_capacity(qap.n_public + 1);
    let mut g1_witness_coeffs = Vec::with_capacity(n_witness);
    for i in 0..qap.n_vars {
        let ai_tau = qap.a_polys[i].evaluate(&tau);
        let bi_tau = qap.b_polys[i].evaluate(&tau);
        let ci_tau = qap.c_polys[i].evaluate(&tau);
        let combined = fr_add(
            &fr_mul(&beta, &ai_tau),
            &fr_add(&fr_mul(&alpha, &bi_tau), &ci_tau),
        );
        if i <= qap.n_public {
            let public_factor = fr_mul(&combined, &gamma_inv);
            g1_public_coeffs.push(g1.scalar_mul(&public_factor));
        } else {
            let witness_factor = fr_mul(&combined, &delta_inv);
            g1_witness_coeffs.push(g1.scalar_mul(&witness_factor));
        }
    }

    let h_max = (2 * max_poly_deg).saturating_sub(z_deg);
    let z_tau = qap.z_poly.evaluate(&tau);
    let z_over_delta = fr_mul(&z_tau, &delta_inv);
    let mut g1_h_coeffs = Vec::with_capacity(h_max + 1);
    let mut tau_pow_h = BigUint::one();
    for _ in 0..=h_max {
        let scalar = fr_mul(&tau_pow_h, &z_over_delta);
        g1_h_coeffs.push(g1.scalar_mul(&scalar));
        tau_pow_h = fr_mul(&tau_pow_h, &tau);
    }

    Groth16Srs {
        g1_alpha,
        g1_beta,
        g1_delta,
        g2_beta,
        g2_delta,
        g2_gamma,
        g1_tau_pow,
        g2_tau_pow,
        g1_witness_coeffs,
        g1_public_coeffs,
        g1_h_coeffs,
    }
}

/// A Groth16 proof: three group elements.
#[derive(Clone, Debug, PartialEq)]
pub struct Groth16Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

/// **Prove** that the QAP is satisfied by the witness `z[0..n_vars]`.
/// `n_public` is split off via the QAP / SRS structure.
pub fn prove(qap: &Qap, srs: &Groth16Srs, z: &[BigUint]) -> Groth16Proof {
    assert_eq!(z.len(), qap.n_vars);
    assert!(
        qap.n_public < qap.n_vars,
        "QAP must have at least the constant-1 wire"
    );
    assert_eq!(
        fr_reduce(&z[0]),
        BigUint::one(),
        "z[0] must be the constant wire 1"
    );
    assert_eq!(
        srs.g1_witness_coeffs.len(),
        qap.n_vars - (qap.n_public + 1),
        "SRS witness coefficient count mismatch"
    );
    let mut rng = OsRng;
    let r = scalar_modulus();
    let r_blind = rng.gen_biguint_below(&r);
    let s_blind = rng.gen_biguint_below(&r);

    // Compute A(X), B(X), C(X) as Σ_i z_i · A_i(X) etc.
    let mut a_poly = Poly::zero();
    let mut b_poly = Poly::zero();
    let mut c_poly = Poly::zero();
    for i in 0..qap.n_vars {
        let zi = fr_reduce(&z[i]);
        a_poly = a_poly.add(&qap.a_polys[i].scale(&zi));
        b_poly = b_poly.add(&qap.b_polys[i].scale(&zi));
        c_poly = c_poly.add(&qap.c_polys[i].scale(&zi));
    }

    // QAP identity: A(X) · B(X) − C(X) = H(X) · Z(X).
    let lhs = a_poly.mul(&b_poly).sub(&c_poly);
    let (h_poly, rem) = lhs.divmod(&qap.z_poly);
    assert!(
        rem.coeffs.iter().all(|c| c.is_zero()),
        "QAP not satisfied — witness invalid"
    );

    let a_g1 = commit_poly_g1(&a_poly, &srs.g1_tau_pow)
        .add(&srs.g1_alpha)
        .add(&srs.g1_delta.scalar_mul(&r_blind));
    let b_g2 = commit_poly_g2(&b_poly, &srs.g2_tau_pow)
        .add(&srs.g2_beta)
        .add(&srs.g2_delta.scalar_mul(&s_blind));
    let b_g1 = commit_poly_g1(&b_poly, &srs.g1_tau_pow)
        .add(&srs.g1_beta)
        .add(&srs.g1_delta.scalar_mul(&s_blind));

    let mut c_proof = G1Point::Infinity;
    for (offset, i) in ((1 + qap.n_public)..qap.n_vars).enumerate() {
        let zi = fr_reduce(&z[i]);
        if !zi.is_zero() {
            c_proof = c_proof.add(&srs.g1_witness_coeffs[offset].scalar_mul(&zi));
        }
    }
    for (i, hc) in h_poly.coeffs.iter().enumerate() {
        if hc.is_zero() {
            continue;
        }
        assert!(
            i < srs.g1_h_coeffs.len(),
            "H(X) degree exceeds SRS g1_h_coeffs capacity"
        );
        c_proof = c_proof.add(&srs.g1_h_coeffs[i].scalar_mul(hc));
    }
    let neg_rs = fr_neg(&fr_mul(&r_blind, &s_blind));
    c_proof = c_proof
        .add(&a_g1.scalar_mul(&s_blind))
        .add(&b_g1.scalar_mul(&r_blind))
        .add(&srs.g1_delta.scalar_mul(&neg_rs));

    Groth16Proof {
        a: a_g1,
        b: b_g2,
        c: c_proof,
    }
}

fn commit_poly_g1(p: &Poly, g1_tau_pow: &[G1Point]) -> G1Point {
    let mut acc = G1Point::Infinity;
    for (i, c) in p.coeffs.iter().enumerate() {
        if c.is_zero() {
            continue;
        }
        assert!(
            i < g1_tau_pow.len(),
            "polynomial degree exceeds SRS g1_tau_pow capacity"
        );
        acc = acc.add(&g1_tau_pow[i].scalar_mul(c));
    }
    acc
}
fn commit_poly_g2(p: &Poly, g2_tau_pow: &[G2Point]) -> G2Point {
    let mut acc = G2Point::Infinity;
    for (i, c) in p.coeffs.iter().enumerate() {
        if c.is_zero() {
            continue;
        }
        assert!(
            i < g2_tau_pow.len(),
            "polynomial degree exceeds SRS g2_tau_pow capacity"
        );
        acc = acc.add(&g2_tau_pow[i].scalar_mul(c));
    }
    acc
}

/// **Verify** a Groth16 proof.
///
/// Equation: `e(A, B) == e(α·G₁, β·G₂) · e(IC(x_pub), γ·G₂) · e(C, δ·G₂)`
/// where `IC(x_pub) = Σ_{i ≤ n_public} x_pub_i · g1_public_coeffs[i]`
/// (with `x_pub[0] = 1` for the constant term).
pub fn verify(
    proof: &Groth16Proof,
    public_inputs: &[BigUint],
    srs: &Groth16Srs,
    qap: &Qap,
) -> bool {
    let r = scalar_modulus();
    if qap.n_public >= qap.n_vars
        || public_inputs.len() != qap.n_public
        || srs.g1_public_coeffs.len() != qap.n_public + 1
        || srs.g1_witness_coeffs.len() != qap.n_vars - (qap.n_public + 1)
        || public_inputs.iter().any(|v| *v >= r)
        || !proof.a.is_on_curve()
        || !proof.a.is_in_correct_subgroup()
        || !proof.b.is_on_curve()
        || !proof.b.is_in_correct_subgroup()
        || !proof.c.is_on_curve()
        || !proof.c.is_in_correct_subgroup()
    {
        return false;
    }

    let mut ic = srs.g1_public_coeffs[0].clone();
    for (j, xj) in public_inputs.iter().enumerate() {
        if !xj.is_zero() {
            ic = ic.add(&srs.g1_public_coeffs[j + 1].scalar_mul(xj));
        }
    }

    let lhs = pairing(&proof.a, &proof.b);
    let term_alpha_beta = pairing(&srs.g1_alpha, &srs.g2_beta);
    let term_ic = pairing(&ic, &srs.g2_gamma);
    let term_c = pairing(&proof.c, &srs.g2_delta);
    let rhs = term_alpha_beta.mul(&term_ic).mul(&term_c);

    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bls12_381::fq::Fq;

    /// R1CS satisfaction check on the toy x² = 25 problem.
    #[test]
    fn r1cs_x_squared_25_satisfied() {
        let constraint = Constraint {
            a: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            b: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            c: vec![BigUint::zero(), BigUint::one(), BigUint::zero()],
        };
        let r1cs = R1cs {
            n_vars: 3,
            n_public: 1,
            constraints: vec![constraint],
        };
        let z = vec![BigUint::one(), BigUint::from(25u32), BigUint::from(5u32)];
        assert!(r1cs.is_satisfied(&z));
        let z_bad = vec![BigUint::one(), BigUint::from(25u32), BigUint::from(4u32)];
        assert!(!r1cs.is_satisfied(&z_bad));
    }

    /// QAP conversion preserves satisfaction at each evaluation point.
    #[test]
    fn qap_preserves_satisfaction() {
        let constraint = Constraint {
            a: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            b: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            c: vec![BigUint::zero(), BigUint::one(), BigUint::zero()],
        };
        let r1cs = R1cs {
            n_vars: 3,
            n_public: 1,
            constraints: vec![constraint.clone(), constraint.clone(), constraint],
        };
        let qap = r1cs_to_qap(&r1cs);
        let z = [BigUint::one(), BigUint::from(25u32), BigUint::from(5u32)];

        for pt in &qap.points {
            let a_eval = qap
                .a_polys
                .iter()
                .enumerate()
                .fold(BigUint::zero(), |acc, (i, p)| {
                    fr_add(&acc, &fr_mul(&z[i], &p.evaluate(pt)))
                });
            let b_eval = qap
                .b_polys
                .iter()
                .enumerate()
                .fold(BigUint::zero(), |acc, (i, p)| {
                    fr_add(&acc, &fr_mul(&z[i], &p.evaluate(pt)))
                });
            let c_eval = qap
                .c_polys
                .iter()
                .enumerate()
                .fold(BigUint::zero(), |acc, (i, p)| {
                    fr_add(&acc, &fr_mul(&z[i], &p.evaluate(pt)))
                });
            assert_eq!(fr_mul(&a_eval, &b_eval), c_eval);
        }
    }

    /// SRS partition check (VULN-1A regression): `g1_public_coeffs` covers only
    /// wires `0..=n_public` and `g1_witness_coeffs` covers strictly private wires
    /// `(n_public + 1)..n_vars`.
    #[test]
    fn srs_strictly_partitions_public_and_witness_wires() {
        let constraint = Constraint {
            a: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            b: vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
            c: vec![BigUint::zero(), BigUint::one(), BigUint::zero()],
        };
        let r1cs = R1cs {
            n_vars: 3,
            n_public: 1,
            constraints: vec![constraint],
        };
        let qap = r1cs_to_qap(&r1cs);
        let srs = groth16_insecure_setup(&qap);
        assert_eq!(srs.g1_public_coeffs.len(), 2, "wires 0..=n_public only");
        assert_eq!(
            srs.g1_witness_coeffs.len(),
            1,
            "strictly private witness wires (n_public+1)..n_vars"
        );
    }

    /// **End-to-end Groth16 completeness & soundness** (SHF-20261028-0f3f64, S2N-9):
    /// positive control on `x² = 25` plus rejection of wrong public inputs,
    /// tampered `(A, B, C)` elements, non-canonical scalars, and off-curve points.
    #[test]
    fn groth16_x_squared_25_prove_verify() {
        let c1 = Constraint {
            a: vec![
                BigUint::zero(),
                BigUint::zero(),
                BigUint::one(),
                BigUint::zero(),
            ],
            b: vec![
                BigUint::zero(),
                BigUint::zero(),
                BigUint::one(),
                BigUint::zero(),
            ],
            c: vec![
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                BigUint::one(),
            ],
        };
        // Second constraint: y · 1 = out  (multi-constraint QAP so deg(H) >= 1 is exercised).
        let c2 = Constraint {
            a: vec![
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                BigUint::one(),
            ],
            b: vec![
                BigUint::one(),
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
            ],
            c: vec![
                BigUint::zero(),
                BigUint::one(),
                BigUint::zero(),
                BigUint::zero(),
            ],
        };
        let r1cs = R1cs {
            n_vars: 4,
            n_public: 1,
            constraints: vec![c1, c2],
        };
        let qap = r1cs_to_qap(&r1cs);
        let srs = groth16_insecure_setup(&qap);

        // z = [1, out=25, x=5, y=25].
        let z = vec![
            BigUint::one(),
            BigUint::from(25u32),
            BigUint::from(5u32),
            BigUint::from(25u32),
        ];
        let proof1 = prove(&qap, &srs, &z);
        let proof2 = prove(&qap, &srs, &z);
        // Zero-knowledge blinding rerandomizes the proof.
        assert_ne!(proof1, proof2);

        let public_inputs = vec![BigUint::from(25u32)];
        assert!(verify(&proof1, &public_inputs, &srs, &qap));
        assert!(verify(&proof2, &public_inputs, &srs, &qap));

        // Wrong public statement (out = 26) must fail.
        assert!(!verify(&proof1, &[BigUint::from(26u32)], &srs, &qap));

        // Non-canonical public input 25 + r must fail.
        let non_canonical = BigUint::from(25u32) + scalar_modulus();
        assert!(!verify(&proof1, &[non_canonical], &srs, &qap));

        // Tampering with each proof element individually must fail.
        let mut bad_a = proof1.clone();
        bad_a.a = bad_a.a.add(&G1Point::generator());
        assert!(!verify(&bad_a, &public_inputs, &srs, &qap));

        let mut bad_b = proof1.clone();
        bad_b.b = bad_b.b.add(&G2Point::generator());
        assert!(!verify(&bad_b, &public_inputs, &srs, &qap));

        let mut bad_c = proof1.clone();
        bad_c.c = bad_c.c.add(&G1Point::generator());
        assert!(!verify(&bad_c, &public_inputs, &srs, &qap));

        // Off-curve point must return false without panicking.
        let mut off_curve = proof1;
        off_curve.a = G1Point::Affine {
            x: Fq::zero(),
            y: Fq::one(),
        };
        assert!(!verify(&off_curve, &public_inputs, &srs, &qap));
    }
}

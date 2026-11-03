//! **PLONK** (Gabizon, Williamson, Ciobotaru 2019) — universal-setup,
//! polynomial-IOP-based zk-SNARK.
//!
//! PLONK encodes a computation as an **arithmetic circuit** of gates
//! with five selector polynomials (`q_L, q_R, q_O, q_M, q_C`) and
//! **copy constraints** wiring outputs of some gates to inputs of
//! others.  The witness `(a, b, c)` (length-`n` left/right/output
//! wire vectors) satisfies the circuit iff:
//!
//! 1. **Gate equation** (at each row `i`):
//!    `q_L[i]·a[i] + q_R[i]·b[i] + q_O[i]·c[i] + q_M[i]·a[i]·b[i] + q_C[i] = 0`.
//! 2. **Copy constraint**: the wires form a multiset matching the
//!    intended wiring permutation `σ`, expressed as a "grand-product"
//!    polynomial identity.
//!
//! ## Polynomial IOP
//!
//! Both constraints reduce to **polynomial identities on a
//! multiplicative subgroup `H ⊂ F_r`**.  The prover commits to the
//! wire polynomials `a(X), b(X), c(X)` and the permutation
//! accumulator `Z(X)` via KZG.  The verifier picks a random
//! challenge `ζ ∈ F_r` (Fiat-Shamir), the prover opens the relevant
//! polynomials at `ζ`, and the verifier checks the combined
//! identity reduces to zero at `ζ`.
//!
//! ## What this module ships
//!
//! - [`Circuit`] — gate selectors + permutation indices.
//! - [`Witness`] — left/right/output wire assignments.
//! - [`Plonkish`] — circuit + witness checker (sanity check that
//!   the gate equation and copy constraints hold).
//! - [`prove`] / [`verify`] — full polynomial-IOP prover and
//!   verifier with Fiat-Shamir.
//!
//! ## Honest scope
//!
//! - **Polynomial-IOP level only; the polynomials stand in for their
//!   commitments.**  The proof carries `a, b, c, Z, t` in the clear (so
//!   it is neither succinct nor zero-knowledge) and no KZG opening
//!   proofs: the KZG layer is in [`super::kzg`], and combining the two
//!   needs a working pairing, which `bls12_381::pairing` does not yet
//!   provide (see the Groth16 notes).  [`verify`] absorbs every
//!   polynomial, checks each sent evaluation `a(ζ), b(ζ), c(ζ), Z(ζω)`
//!   against the polynomial it was sent (the step KZG openings would
//!   do), and checks
//!   `gate(ζ) + α·perm(ζ) + α²·L₁(ζ)·(Z(ζ) − 1) = t(ζ)·Z_H(ζ)` from
//!   those evaluations: the gate equation, the copy constraints (`σ`
//!   polynomials, coset constants 1, 2, 3) and `Z(1) = 1`.  Before
//!   2026-10-28 the sent evaluations were never checked and the
//!   quotient left out the permutation, so a forger could solve for
//!   `Z(ζω)` and pass any circuit (sheriff case SHF-20261028-0f3f64,
//!   S2N-10).  Soundness is the usual Schwartz–Zippel bound over a
//!   random-oracle Fiat–Shamir transcript; this is an educational,
//!   unaudited verifier with no stated security level and must not back
//!   a research claim.
//!
//! - **Single multiplicative-subgroup domain.**  We use a small
//!   power-of-2 domain `H` with a primitive root of unity `ω` over
//!   `F_r` (the BLS12-381 scalar field has a 2^32-th root of unity,
//!   so any domain up to size `2^32` is supported).

use super::polynomial::{fr_add, fr_inv, fr_mul, fr_neg, fr_pow, fr_sub, Poly};
use crate::bls12_381::fq::scalar_modulus;
use crate::hash::sha256::sha256;
use num_bigint::BigUint;
use num_traits::{One, Zero};

const FS_TAG: &str = "ZK-PLONK/v1";

// ── Multiplicative subgroup helpers ────────────────────────────────

/// `ω` = a primitive `n`-th root of unity in `F_r`, where `n` is a
/// power of 2 dividing `2^32`.
///
/// Derivation: `F_r^*` has order `r − 1 = 2^32 · t` for odd `t`.
/// For any generator `g` of `F_r^*` (a multiplicative non-residue
/// works), `g^((r−1)/n)` is a primitive `n`-th root of unity.
///
/// We use `g = 7` (verified non-residue in `F_r` for BLS12-381) and
/// compute `g^((r−1)/n)` at runtime.  Cached on first call would
/// be a natural optimisation; we recompute for simplicity.
pub fn primitive_nth_root(n: usize) -> BigUint {
    assert!(n.is_power_of_two(), "n must be a power of 2");
    let log_n = n.trailing_zeros() as u64;
    assert!(log_n <= 32, "n must divide 2^32");
    let r = scalar_modulus();
    // For BLS12-381, `g = 7` is a multiplicative generator of F_r*.
    let g = BigUint::from(7u32);
    let exp = (&r - BigUint::one()) / BigUint::from(n as u64);
    fr_pow(&g, &exp)
}

/// `[1, ω, ω², …, ω^{n−1}]`.
fn domain(n: usize) -> Vec<BigUint> {
    let omega = primitive_nth_root(n);
    let mut out = Vec::with_capacity(n);
    let mut acc = BigUint::one();
    for _ in 0..n {
        out.push(acc.clone());
        acc = fr_mul(&acc, &omega);
    }
    out
}

// ── Circuit / witness ─────────────────────────────────────────────

/// A PLONK arithmetic circuit, encoded as selector polynomials
/// over a multiplicative-subgroup domain of size `n`.  All vectors
/// have length `n` (the gate count, padded to a power of 2).
#[derive(Clone, Debug)]
pub struct Circuit {
    pub n: usize,
    /// Selector for the `a` input (left-wire) term.
    pub q_l: Vec<BigUint>,
    pub q_r: Vec<BigUint>,
    pub q_o: Vec<BigUint>,
    pub q_m: Vec<BigUint>,
    pub q_c: Vec<BigUint>,
    /// Permutation: for each of the 3n wires, the *index* it's
    /// wired to.  Wire indices follow the convention used in the
    /// PLONK paper: wires 0..n are `a`-column, n..2n are `b`,
    /// 2n..3n are `c`.
    pub permutation: Vec<usize>,
}

/// Witness vectors: the prover's secret left/right/output wire
/// assignments for each gate row.
#[derive(Clone, Debug)]
pub struct Witness {
    pub a: Vec<BigUint>,
    pub b: Vec<BigUint>,
    pub c: Vec<BigUint>,
}

impl Circuit {
    pub fn is_well_formed(&self) -> bool {
        self.n.is_power_of_two()
            && self.q_l.len() == self.n
            && self.q_r.len() == self.n
            && self.q_o.len() == self.n
            && self.q_m.len() == self.n
            && self.q_c.len() == self.n
            && self.permutation.len() == 3 * self.n
            && self.permutation.iter().all(|&i| i < 3 * self.n)
    }

    /// **Direct check** of the circuit constraints against a witness.
    /// Used for sanity testing the prover; the real verifier would
    /// instead check via the polynomial identity at challenge ζ.
    pub fn is_satisfied(&self, w: &Witness) -> bool {
        if !self.is_well_formed() {
            return false;
        }
        if w.a.len() != self.n || w.b.len() != self.n || w.c.len() != self.n {
            return false;
        }
        // Gate equation.
        for i in 0..self.n {
            let term = fr_add(
                &fr_add(
                    &fr_mul(&self.q_l[i], &w.a[i]),
                    &fr_add(
                        &fr_mul(&self.q_r[i], &w.b[i]),
                        &fr_mul(&self.q_o[i], &w.c[i]),
                    ),
                ),
                &fr_add(
                    &fr_mul(&self.q_m[i], &fr_mul(&w.a[i], &w.b[i])),
                    &self.q_c[i],
                ),
            );
            if !term.is_zero() {
                return false;
            }
        }
        // Permutation (copy constraints).
        let all_wires: Vec<&BigUint> = w.a.iter().chain(w.b.iter()).chain(w.c.iter()).collect();
        for (i, &target) in self.permutation.iter().enumerate() {
            if all_wires[i] != all_wires[target] {
                return false;
            }
        }
        true
    }
}

// ── Fiat-Shamir transcript ─────────────────────────────────────────

#[derive(Clone)]
struct Transcript {
    state: Vec<u8>,
}

impl Transcript {
    fn new() -> Self {
        Self {
            state: FS_TAG.as_bytes().to_vec(),
        }
    }
    fn append(&mut self, label: &str, bytes: &[u8]) {
        self.state.extend_from_slice(label.as_bytes());
        self.state.extend_from_slice(bytes);
    }
    fn append_scalar(&mut self, label: &str, s: &BigUint) {
        let mut buf = [0u8; 32];
        let bytes = s.to_bytes_be();
        buf[32 - bytes.len()..].copy_from_slice(&bytes);
        self.append(label, &buf);
    }
    fn append_poly(&mut self, label: &str, p: &Poly) {
        let n = p.coeffs.len();
        self.append(label, &(n as u64).to_be_bytes());
        for c in &p.coeffs {
            self.append_scalar("c", c);
        }
    }
    fn challenge(&mut self, label: &str) -> BigUint {
        self.state.extend_from_slice(label.as_bytes());
        let h = sha256(&self.state);
        self.state.extend_from_slice(&h);
        BigUint::from_bytes_be(&h) % scalar_modulus()
    }
}

// ── Prover / Verifier ──────────────────────────────────────────────

/// A PLONK proof at the polynomial-IOP level: polynomial commitments
/// represented as the polynomials themselves (no KZG binding yet),
/// plus the evaluations at the verifier's challenge `ζ` and the
/// quotient polynomial `t`.
#[derive(Clone, Debug)]
pub struct PlonkProof {
    /// Wire polynomials interpolated at the domain.
    pub a_poly: Poly,
    pub b_poly: Poly,
    pub c_poly: Poly,
    /// Permutation grand-product accumulator polynomial `Z(X)`.
    pub z_poly: Poly,
    /// Quotient polynomial `t(X)` such that the combined identity
    /// equals `t(X) · Z_H(X)` (where `Z_H` is the vanishing
    /// polynomial of the domain `H`).
    pub t_poly: Poly,
    /// Wire evaluations at `ζ`.
    pub a_zeta: BigUint,
    pub b_zeta: BigUint,
    pub c_zeta: BigUint,
    pub z_zeta_omega: BigUint,
}

/// Prove that `witness` satisfies `circuit`.  Returns a [`PlonkProof`].
pub fn prove(circuit: &Circuit, witness: &Witness) -> PlonkProof {
    assert!(
        circuit.is_satisfied(witness),
        "witness does not satisfy circuit (prover would fail anyway; flagged early)"
    );
    let (proof, rem) = prove_unchecked(circuit, witness);
    assert!(rem.coeffs.is_empty(), "Z_H divides the combined identity");
    proof
}

/// The prover without the witness check: also returns the remainder of
/// the combined identity modulo `Z_H`, which is zero iff the witness
/// satisfies the circuit (the tests build cheating proofs with it).
fn prove_unchecked(circuit: &Circuit, witness: &Witness) -> (PlonkProof, Poly) {
    let n = circuit.n;
    let h_domain = domain(n);

    // Interpolate witness polynomials over H.
    let a_poly = lagrange_from_evaluations(&h_domain, &witness.a);
    let b_poly = lagrange_from_evaluations(&h_domain, &witness.b);
    let c_poly = lagrange_from_evaluations(&h_domain, &witness.c);

    // Build the permutation accumulator Z(X) using PLONK's
    // beta/gamma challenges.
    let mut transcript = Transcript::new();
    transcript.append_poly("a", &a_poly);
    transcript.append_poly("b", &b_poly);
    transcript.append_poly("c", &c_poly);
    let beta = transcript.challenge("beta");
    let gamma = transcript.challenge("gamma");

    let z_values = compute_z(circuit, witness, &h_domain, &beta, &gamma);
    let z_poly = lagrange_from_evaluations(&h_domain, &z_values);

    // Construct the quotient polynomial t(X) such that
    //   t(X) · Z_H(X) = combined_identity(X).
    transcript.append_poly("z", &z_poly);
    let alpha = transcript.challenge("alpha");

    let (t_poly, rem) = compute_quotient(
        circuit, &h_domain, &a_poly, &b_poly, &c_poly, &z_poly, &beta, &gamma, &alpha,
    );

    transcript.append_poly("t", &t_poly);
    let zeta = transcript.challenge("zeta");

    let omega = primitive_nth_root(n);
    let zeta_omega = fr_mul(&zeta, &omega);

    let proof = PlonkProof {
        a_zeta: a_poly.evaluate(&zeta),
        b_zeta: b_poly.evaluate(&zeta),
        c_zeta: c_poly.evaluate(&zeta),
        z_zeta_omega: z_poly.evaluate(&zeta_omega),
        a_poly,
        b_poly,
        c_poly,
        z_poly,
        t_poly,
    };
    (proof, rem)
}

/// The Fiat-Shamir challenges `(β, γ, α, ζ)` of `proof`.
fn challenges(proof: &PlonkProof) -> (BigUint, BigUint, BigUint, BigUint) {
    let mut transcript = Transcript::new();
    transcript.append_poly("a", &proof.a_poly);
    transcript.append_poly("b", &proof.b_poly);
    transcript.append_poly("c", &proof.c_poly);
    let beta = transcript.challenge("beta");
    let gamma = transcript.challenge("gamma");
    transcript.append_poly("z", &proof.z_poly);
    let alpha = transcript.challenge("alpha");
    transcript.append_poly("t", &proof.t_poly);
    let zeta = transcript.challenge("zeta");
    (beta, gamma, alpha, zeta)
}

/// Verify a [`PlonkProof`] against the circuit.  Rejects non-canonical
/// values, replays the transcript to recover `(β, γ, α, ζ)`, checks
/// every evaluation the proof sends against the polynomial it was sent
/// (`a(ζ), b(ζ), c(ζ), Z(ζω)`), then checks the combined identity
/// (gate, copy constraints, `Z(1) = 1`) at `ζ` from those evaluations.
/// Returns `false`, never panics, on malformed input.
pub fn verify(circuit: &Circuit, proof: &PlonkProof) -> bool {
    if !circuit.is_well_formed() {
        return false;
    }
    let r = scalar_modulus();
    let polys = [
        &proof.a_poly,
        &proof.b_poly,
        &proof.c_poly,
        &proof.z_poly,
        &proof.t_poly,
    ];
    let evals = [
        &proof.a_zeta,
        &proof.b_zeta,
        &proof.c_zeta,
        &proof.z_zeta_omega,
    ];
    if polys.iter().any(|p| p.coeffs.iter().any(|c| *c >= r)) || evals.iter().any(|v| **v >= r) {
        return false;
    }
    let (_, _, _, zeta) = challenges(proof);
    let zeta_omega = fr_mul(&zeta, &primitive_nth_root(circuit.n));
    // Each sent evaluation is the sent polynomial's (the polynomials
    // stand in for their commitments; see "Honest scope").
    if proof.a_zeta != proof.a_poly.evaluate(&zeta)
        || proof.b_zeta != proof.b_poly.evaluate(&zeta)
        || proof.c_zeta != proof.c_poly.evaluate(&zeta)
        || proof.z_zeta_omega != proof.z_poly.evaluate(&zeta_omega)
    {
        return false;
    }
    identity_residual(circuit, proof).is_zero()
}

/// `combined(ζ) − t(ζ)·Z_H(ζ)` computed from the proof's sent
/// evaluations, where
///   `combined = gate + α·perm + α²·L₁·(Z − 1)`.
/// Zero is the identity check of [`verify`]; on its own it is the
/// pre-2026-10-28 verifier, which never tied the sent evaluations to
/// the polynomials (SHF-20261028-0f3f64, S2N-10).
fn identity_residual(circuit: &Circuit, proof: &PlonkProof) -> BigUint {
    let n = circuit.n;
    let h_domain = domain(n);
    let (beta, gamma, alpha, zeta) = challenges(proof);

    // Recompute the selector and permutation values at ζ (the verifier
    // knows the circuit, so this is "preprocessing").
    let [q_l, q_r, q_o, q_m, q_c] = selector_polys(circuit, &h_domain);
    let a_z = &proof.a_zeta;
    let b_z = &proof.b_zeta;
    let c_z = &proof.c_zeta;

    // Gate identity at ζ:
    //   gate(ζ) = q_M·a·b + q_L·a + q_R·b + q_O·c + q_C.
    let gate_zeta = fr_add(
        &fr_add(
            &fr_add(
                &fr_mul(&q_m.evaluate(&zeta), &fr_mul(a_z, b_z)),
                &fr_mul(&q_l.evaluate(&zeta), a_z),
            ),
            &fr_add(
                &fr_mul(&q_r.evaluate(&zeta), b_z),
                &fr_mul(&q_o.evaluate(&zeta), c_z),
            ),
        ),
        &q_c.evaluate(&zeta),
    );

    // Permutation identity at ζ:
    //   perm(ζ) = Z(ζ)·∏_i (wire_i(ζ) + β·k_i·ζ + γ)
    //           − Z(ζω)·∏_i (wire_i(ζ) + β·σ_i(ζ) + γ).
    let z_z = proof.z_poly.evaluate(&zeta);
    let perm_check = permutation_identity_at_zeta(
        circuit,
        &h_domain,
        &zeta,
        &beta,
        &gamma,
        a_z,
        b_z,
        c_z,
        &z_z,
        &proof.z_zeta_omega,
    );
    // Z(1) = 1: L₁(ζ)·(Z(ζ) − 1).
    let boundary = fr_mul(
        &lagrange_first(&h_domain).evaluate(&zeta),
        &fr_sub(&z_z, &BigUint::one()),
    );

    let combined = fr_add(
        &gate_zeta,
        &fr_add(
            &fr_mul(&alpha, &perm_check),
            &fr_mul(&fr_mul(&alpha, &alpha), &boundary),
        ),
    );

    // Vanishing polynomial Z_H(X) = X^n − 1 at ζ.
    let zeta_n = fr_pow(&zeta, &BigUint::from(n as u64));
    let z_h_zeta = fr_sub(&zeta_n, &BigUint::one());
    let rhs = fr_mul(&proof.t_poly.evaluate(&zeta), &z_h_zeta);
    fr_sub(&combined, &rhs)
}

// ── Helpers ─────────────────────────────────────────────────────────

/// The coset constants `k_a, k_b, k_c` of the three wire columns.
fn wire_ks() -> [BigUint; 3] {
    [BigUint::one(), BigUint::from(2u32), BigUint::from(3u32)]
}

/// Selector polynomials `[q_L, q_R, q_O, q_M, q_C]`.
fn selector_polys(circuit: &Circuit, h_domain: &[BigUint]) -> [Poly; 5] {
    [
        &circuit.q_l,
        &circuit.q_r,
        &circuit.q_o,
        &circuit.q_m,
        &circuit.q_c,
    ]
    .map(|q| lagrange_from_evaluations(h_domain, q))
}

/// Permutation polynomials `[σ_a, σ_b, σ_c]`: column `j` takes
/// `k_col · ω^row` of the wire that wire `j·n + i` is wired to.
fn sigma_polys(circuit: &Circuit, h_domain: &[BigUint]) -> [Poly; 3] {
    let n = circuit.n;
    let [k_a, k_b, k_c] = wire_ks();
    [0, 1, 2].map(|col| {
        let evals: Vec<BigUint> = (0..n)
            .map(|i| {
                wire_id_to_omega(
                    circuit.permutation[col * n + i],
                    n,
                    h_domain,
                    &k_a,
                    &k_b,
                    &k_c,
                )
            })
            .collect();
        lagrange_from_evaluations(h_domain, &evals)
    })
}

/// `L₁(X)`: 1 at `ω⁰ = 1`, 0 elsewhere on `H`.
fn lagrange_first(h_domain: &[BigUint]) -> Poly {
    let mut e = vec![BigUint::zero(); h_domain.len()];
    e[0] = BigUint::one();
    lagrange_from_evaluations(h_domain, &e)
}

/// Build the polynomial of degree `< n` that takes `evals[i]` at
/// `domain[i]`.  Uses naive Lagrange interpolation; for small `n`
/// this is fine, but a production PLONK would use an inverse-FFT
/// over the multiplicative subgroup.
fn lagrange_from_evaluations(domain: &[BigUint], evals: &[BigUint]) -> Poly {
    assert_eq!(domain.len(), evals.len());
    let pts: Vec<(BigUint, BigUint)> = domain
        .iter()
        .zip(evals)
        .map(|(d, e)| (d.clone(), e.clone()))
        .collect();
    Poly::interpolate(&pts)
}

/// Compute the values of the permutation grand-product polynomial
/// `Z(X)` on the domain `H`.
///
/// Recursion:
///   `Z(ω^0) = 1`
///   `Z(ω^{i+1}) = Z(ω^i) · numerator_i / denominator_i`
/// where:
///   `numerator_i = ∏_{wires}(wire_i + β·k_wire·ω^i + γ)`
///   `denominator_i = ∏_{wires}(wire_i + β·σ(wire_position_i) + γ)`
///
/// Here `k_a, k_b, k_c` are coset-distinguishing constants (1, k1, k2);
/// for the educational implementation, we use `k_a = 1`, `k_b = 2`,
/// `k_c = 3`.
fn compute_z(
    circuit: &Circuit,
    witness: &Witness,
    h_domain: &[BigUint],
    beta: &BigUint,
    gamma: &BigUint,
) -> Vec<BigUint> {
    let n = circuit.n;
    let [k_a, k_b, k_c] = wire_ks();
    let mut z = Vec::with_capacity(n);
    z.push(BigUint::one()); // Z(ω^0) = 1
    let mut acc = BigUint::one();
    for i in 0..(n - 1) {
        // Numerator at row i.
        let num_a = fr_add(
            &witness.a[i],
            &fr_add(&fr_mul(beta, &fr_mul(&k_a, &h_domain[i])), gamma),
        );
        let num_b = fr_add(
            &witness.b[i],
            &fr_add(&fr_mul(beta, &fr_mul(&k_b, &h_domain[i])), gamma),
        );
        let num_c = fr_add(
            &witness.c[i],
            &fr_add(&fr_mul(beta, &fr_mul(&k_c, &h_domain[i])), gamma),
        );
        let num = fr_mul(&fr_mul(&num_a, &num_b), &num_c);
        // Denominator at row i: uses the permutation σ.
        let sigma_a = wire_id_to_omega(circuit.permutation[i], n, h_domain, &k_a, &k_b, &k_c);
        let sigma_b = wire_id_to_omega(circuit.permutation[n + i], n, h_domain, &k_a, &k_b, &k_c);
        let sigma_c = wire_id_to_omega(
            circuit.permutation[2 * n + i],
            n,
            h_domain,
            &k_a,
            &k_b,
            &k_c,
        );
        let den_a = fr_add(&witness.a[i], &fr_add(&fr_mul(beta, &sigma_a), gamma));
        let den_b = fr_add(&witness.b[i], &fr_add(&fr_mul(beta, &sigma_b), gamma));
        let den_c = fr_add(&witness.c[i], &fr_add(&fr_mul(beta, &sigma_c), gamma));
        let den = fr_mul(&fr_mul(&den_a, &den_b), &den_c);
        let den_inv = fr_inv(&den).expect("non-zero permutation denominator");
        acc = fr_mul(&acc, &fr_mul(&num, &den_inv));
        z.push(acc.clone());
    }
    z
}

fn wire_id_to_omega(
    wire_id: usize,
    n: usize,
    h_domain: &[BigUint],
    k_a: &BigUint,
    k_b: &BigUint,
    k_c: &BigUint,
) -> BigUint {
    let row = wire_id % n;
    let column = wire_id / n;
    let k = match column {
        0 => k_a,
        1 => k_b,
        2 => k_c,
        _ => panic!("wire_id out of range"),
    };
    fr_mul(k, &h_domain[row])
}

/// Compute the quotient polynomial `t(X)` and remainder of
///   `gate(X) + α·perm(X) + α²·L₁(X)·(Z(X) − 1)` divided by
/// `Z_H(X) = X^n − 1`, where
///   `perm(X) = Z(X)·∏_i (w_i(X) + β·k_i·X + γ)
///            − Z(ωX)·∏_i (w_i(X) + β·σ_i(X) + γ)`.
/// The remainder is zero iff the witness satisfies the gates, the copy
/// constraints and `Z(1) = 1` on `H`.
fn compute_quotient(
    circuit: &Circuit,
    h_domain: &[BigUint],
    a_poly: &Poly,
    b_poly: &Poly,
    c_poly: &Poly,
    z_poly: &Poly,
    beta: &BigUint,
    gamma: &BigUint,
    alpha: &BigUint,
) -> (Poly, Poly) {
    let n = circuit.n;
    let [q_l, q_r, q_o, q_m, q_c] = selector_polys(circuit, h_domain);

    // Gate(X) = q_M·a·b + q_L·a + q_R·b + q_O·c + q_C.
    let gate = q_m
        .mul(&a_poly.mul(b_poly))
        .add(&q_l.mul(a_poly))
        .add(&q_r.mul(b_poly))
        .add(&q_o.mul(c_poly))
        .add(&q_c);

    // perm(X), with w + β·s + γ for s = k·X (identity) or σ (wiring).
    let x = Poly::from_coeffs(vec![BigUint::zero(), BigUint::one()]);
    let gamma_c = Poly::constant(gamma.clone());
    let term = |w: &Poly, s: &Poly| w.add(&s.scale(beta)).add(&gamma_c);
    let wires = [a_poly, b_poly, c_poly];
    let ks = wire_ks();
    let sigmas = sigma_polys(circuit, h_domain);
    let mut num = Poly::constant(BigUint::one());
    let mut den = Poly::constant(BigUint::one());
    for i in 0..3 {
        num = num.mul(&term(wires[i], &x.scale(&ks[i])));
        den = den.mul(&term(wires[i], &sigmas[i]));
    }
    // Z(ωX): scale coefficient j by ω^j.
    let omega = primitive_nth_root(n);
    let mut w_j = BigUint::one();
    let mut z_shift = Vec::with_capacity(z_poly.coeffs.len());
    for c in &z_poly.coeffs {
        z_shift.push(fr_mul(c, &w_j));
        w_j = fr_mul(&w_j, &omega);
    }
    let z_shift = Poly::from_coeffs(z_shift);
    let perm = z_poly.mul(&num).sub(&z_shift.mul(&den));
    let boundary = lagrange_first(h_domain).mul(&z_poly.sub(&Poly::constant(BigUint::one())));
    let combined = gate
        .add(&perm.scale(alpha))
        .add(&boundary.scale(&fr_mul(alpha, alpha)));

    // Z_H(X) = X^n − 1.
    let mut z_h_coeffs = vec![BigUint::zero(); n + 1];
    z_h_coeffs[0] = fr_neg(&BigUint::one());
    z_h_coeffs[n] = BigUint::one();
    combined.divmod(&Poly::from_coeffs(z_h_coeffs))
}

fn permutation_identity_at_zeta(
    circuit: &Circuit,
    h_domain: &[BigUint],
    zeta: &BigUint,
    beta: &BigUint,
    gamma: &BigUint,
    a_z: &BigUint,
    b_z: &BigUint,
    c_z: &BigUint,
    z_z: &BigUint,
    z_zw: &BigUint,
) -> BigUint {
    let ks = wire_ks();
    let sigmas = sigma_polys(circuit, h_domain);
    let wires = [a_z, b_z, c_z];
    let mut num = BigUint::one();
    let mut den = BigUint::one();
    for i in 0..3 {
        let w_gamma = fr_add(wires[i], gamma);
        num = fr_mul(
            &num,
            &fr_add(&w_gamma, &fr_mul(beta, &fr_mul(&ks[i], zeta))),
        );
        den = fr_mul(
            &den,
            &fr_add(&w_gamma, &fr_mul(beta, &sigmas[i].evaluate(zeta))),
        );
    }

    // Identity: Z(ζ)·num − Z(ζω)·den = 0  (rearranged from
    // Z(ζω)/Z(ζ) = num/den).
    fr_sub(&fr_mul(z_z, &num), &fr_mul(z_zw, &den))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a tiny 4-row circuit that proves `x · y = z + 1`.
    /// Concretely: one multiplication gate `a₀ · b₀ = c₀` and one
    /// addition gate `a₁ + b₁ = c₁` with a fixed constant.  The
    /// other rows are zero gates (always satisfied).
    fn toy_circuit() -> (Circuit, Witness) {
        let n = 4;
        // Gate 0: a·b - c = 0  ⇒  q_M=1, q_O=-1, others 0.
        // Gate 1: a + b - c = 0 ⇒ q_L=1, q_R=1, q_O=-1, others 0.
        // Gates 2, 3: 0 = 0 (no-op).
        let neg_one = fr_neg(&BigUint::one());
        let q_l = vec![
            BigUint::zero(),
            BigUint::one(),
            BigUint::zero(),
            BigUint::zero(),
        ];
        let q_r = vec![
            BigUint::zero(),
            BigUint::one(),
            BigUint::zero(),
            BigUint::zero(),
        ];
        let q_o = vec![
            neg_one.clone(),
            neg_one.clone(),
            BigUint::zero(),
            BigUint::zero(),
        ];
        let q_m = vec![
            BigUint::one(),
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
        ];
        let q_c = vec![BigUint::zero(); n];
        // Identity permutation (each wire maps to itself; no copy constraints).
        let permutation: Vec<usize> = (0..(3 * n)).collect();
        let circuit = Circuit {
            n,
            q_l,
            q_r,
            q_o,
            q_m,
            q_c,
            permutation,
        };

        // Witness: a = (3, 5, 0, 0), b = (4, 7, 0, 0),
        //          c = (12, 12, 0, 0).
        // 3·4 = 12 ✓; 5 + 7 = 12 ✓.
        let witness = Witness {
            a: vec![
                BigUint::from(3u32),
                BigUint::from(5u32),
                BigUint::zero(),
                BigUint::zero(),
            ],
            b: vec![
                BigUint::from(4u32),
                BigUint::from(7u32),
                BigUint::zero(),
                BigUint::zero(),
            ],
            c: vec![
                BigUint::from(12u32),
                BigUint::from(12u32),
                BigUint::zero(),
                BigUint::zero(),
            ],
        };
        (circuit, witness)
    }

    /// `x · y = c₀` then `c₀ + 5 = c₁`, with the copy constraint
    /// `c₀ = a₁` (wires 8 and 1 swapped in the permutation).
    fn copy_circuit() -> (Circuit, Witness) {
        let (mut circuit, _) = toy_circuit();
        circuit.permutation.swap(1, 8);
        let v = |x: [u32; 4]| x.map(BigUint::from).to_vec();
        let witness = Witness {
            a: v([3, 12, 0, 0]),
            b: v([4, 5, 0, 0]),
            c: v([12, 17, 0, 0]),
        };
        (circuit, witness)
    }

    /// The four evaluations a proof sends.
    fn evals(p: &mut PlonkProof) -> [&mut BigUint; 4] {
        [
            &mut p.a_zeta,
            &mut p.b_zeta,
            &mut p.c_zeta,
            &mut p.z_zeta_omega,
        ]
    }

    /// Set evaluation `j` so the identity at ζ holds (the residual is
    /// affine in each sent evaluation); `None` if its slope is zero.
    fn solve_eval(circuit: &Circuit, proof: &mut PlonkProof, j: usize) -> Option<()> {
        *evals(proof)[j] = BigUint::zero();
        let r0 = identity_residual(circuit, proof);
        *evals(proof)[j] = BigUint::one();
        let slope = fr_sub(&identity_residual(circuit, proof), &r0);
        *evals(proof)[j] = fr_mul(&fr_neg(&r0), &fr_inv(&slope)?);
        Some(())
    }

    /// **Direct R1CS-style check** that the witness satisfies the
    /// circuit's gate equations.
    #[test]
    fn circuit_is_satisfied_by_correct_witness() {
        let (circuit, witness) = toy_circuit();
        assert!(circuit.is_satisfied(&witness));
    }

    /// **Bad witness rejected** by the direct check.
    #[test]
    fn circuit_rejects_bad_witness() {
        let (circuit, mut witness) = toy_circuit();
        // Tamper: change c[0] so 3·4 ≠ c[0].
        witness.c[0] = BigUint::from(99u32);
        assert!(!circuit.is_satisfied(&witness));
    }

    /// **Sanity: primitive nth root** ω satisfies `ω^n = 1` but
    /// `ω^{n/2} ≠ 1`.
    #[test]
    fn primitive_root_is_primitive() {
        let n = 8;
        let omega = primitive_nth_root(n);
        let omega_n = fr_pow(&omega, &BigUint::from(n as u64));
        assert_eq!(omega_n, BigUint::one(), "ω^n must equal 1");
        let omega_half = fr_pow(&omega, &BigUint::from((n / 2) as u64));
        assert_ne!(omega_half, BigUint::one(), "ω^(n/2) must NOT equal 1");
    }

    /// Lagrange interpolation through the domain points reproduces
    /// the evaluations.
    #[test]
    fn lagrange_passes_through_evaluations() {
        let n = 4;
        let dom = domain(n);
        let evals: Vec<BigUint> = (0..n as u32).map(BigUint::from).collect();
        let p = lagrange_from_evaluations(&dom, &evals);
        for (d, e) in dom.iter().zip(&evals) {
            assert_eq!(&p.evaluate(d), e);
        }
    }

    /// Copy constraints are now part of the quotient: an honest proof
    /// for a circuit with a non-identity permutation verifies.
    #[test]
    fn plonk_prove_verify_with_copy_constraint() {
        let (circuit, witness) = copy_circuit();
        assert!(verify(&circuit, &prove(&circuit, &witness)));
    }

    /// S2N-10 (SHF-20261028-0f3f64): each sent evaluation is checked
    /// against its polynomial.  Shifting one by 1 is rejected; so is the
    /// finding's forgery, which shifts one and solves another (`Z(ζω)`,
    /// or `c(ζ)` when `Z(ζω)` is the one shifted) so the identity at ζ
    /// still holds.  Control: the forged identity residual is zero, so
    /// the identity check alone (the old verifier) accepts it.
    #[test]
    fn rejects_tampered_evaluations() {
        for (circuit, witness) in [toy_circuit(), copy_circuit()] {
            let proof = prove(&circuit, &witness);
            assert!(identity_residual(&circuit, &proof).is_zero());
            for i in 0..4 {
                let mut p = proof.clone();
                let shifted = fr_add(evals(&mut p)[i], &BigUint::one());
                *evals(&mut p)[i] = shifted.clone();
                assert!(!verify(&circuit, &p), "eval {i} + 1");
                let j = if i == 3 { 2 } else { 3 };
                solve_eval(&circuit, &mut p, j).expect("non-zero slope");
                assert_eq!(*evals(&mut p)[i], shifted);
                assert!(identity_residual(&circuit, &p).is_zero(), "control {i}");
                assert!(!verify(&circuit, &p), "forged eval {i}");
            }
        }
    }

    /// Witnesses that break a gate, a copy constraint, or `Z(1) = 1`
    /// (an all-zero accumulator satisfies the grand-product recursion
    /// trivially) leave a remainder mod `Z_H`; their proofs, with
    /// honest evaluations, are rejected.
    #[test]
    fn rejects_unsatisfying_witnesses() {
        let (circuit, mut witness) = toy_circuit();
        witness.c[0] = BigUint::from(99u32);
        let (p, rem) = prove_unchecked(&circuit, &witness);
        assert!(!rem.coeffs.is_empty());
        assert!(!verify(&circuit, &p));
        // Gates hold, the copy c₀ = a₁ does not.
        let (circuit, mut witness) = copy_circuit();
        witness.a[1] = BigUint::from(11u32);
        witness.c[1] = BigUint::from(16u32);
        let (p, rem) = prove_unchecked(&circuit, &witness);
        assert!(!rem.coeffs.is_empty());
        assert!(!verify(&circuit, &p));
        // Z ≡ 0 with the true witness, t the floor quotient.
        let (circuit, witness) = copy_circuit();
        let mut p = prove(&circuit, &witness);
        let h = domain(circuit.n);
        let z_poly = Poly::zero();
        let mut t = Transcript::new();
        for (l, q) in [("a", &p.a_poly), ("b", &p.b_poly), ("c", &p.c_poly)] {
            t.append_poly(l, q);
        }
        let (beta, gamma) = (t.challenge("beta"), t.challenge("gamma"));
        t.append_poly("z", &z_poly);
        let alpha = t.challenge("alpha");
        let (t_poly, rem) = compute_quotient(
            &circuit, &h, &p.a_poly, &p.b_poly, &p.c_poly, &z_poly, &beta, &gamma, &alpha,
        );
        assert!(!rem.coeffs.is_empty(), "L₁·(Z − 1) leaves a remainder");
        p.z_poly = z_poly;
        p.t_poly = t_poly;
        let (_, _, _, zeta) = challenges(&p);
        p.a_zeta = p.a_poly.evaluate(&zeta);
        p.b_zeta = p.b_poly.evaluate(&zeta);
        p.c_zeta = p.c_poly.evaluate(&zeta);
        p.z_zeta_omega = BigUint::zero();
        assert!(!verify(&circuit, &p));
    }

    /// Non-canonical evaluations or coefficients (`v + r`, `2^300`)
    /// return false instead of panicking in the transcript.
    #[test]
    fn rejects_non_canonical_values_without_panic() {
        let (circuit, witness) = toy_circuit();
        let proof = prove(&circuit, &witness);
        let r = scalar_modulus();
        for i in 0..4 {
            let mut p = proof.clone();
            *evals(&mut p)[i] += &r;
            assert!(!verify(&circuit, &p));
        }
        let mut p = proof.clone();
        p.a_poly.coeffs[0] = BigUint::one() << 300;
        assert!(!verify(&circuit, &p));
        let mut p = proof.clone();
        p.t_poly.coeffs.push(r.clone());
        assert!(!verify(&circuit, &p));
    }

    /// **Polynomial-IOP prover and verifier** on the toy circuit.
    /// This is the headline test: a complete prove → verify cycle
    /// at the IOP level (no KZG pairings).
    #[test]
    fn plonk_polynomial_iop_prove_verify() {
        let (circuit, witness) = toy_circuit();
        let proof = prove(&circuit, &witness);
        assert!(verify(&circuit, &proof));
    }
}

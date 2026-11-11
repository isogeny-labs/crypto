# Protocol: does the k = 20 Curve25519 miniature's factor base share one 2-descent class? (frozen before data)

2026-11-11. Follow-up to the note in `research/tracezero_toy_20261106/README.md` ("What this says
about cofactor-8 curves"), which claimed a possible constant-factor filter on the 8 cofactor
lifts of `pkm-semaev:lifts=1` IF the base points share one descent class. That premise is a
hypothesis. This protocol tests it. It is an independent re-construction in Python (stdlib only),
not the Rust instrument.

## Setting

The miniature follows `build_miniature_at` in `src/bin/curve25519_family.rs`: p the largest prime
≡ 5 (mod 8) below 2^20; A the smallest value ≡ 2 (mod 4), A ≠ ±2, stepping by 4, with
A²−4 a nonsquare, n = #E divisible by 8, n/8 prime, (2p+2−n) divisible by 4 with a prime quotient,
and n/8 > 2^17. Curve y² = x³ + Ax² + x, with rational 2-torsion point (0,0) only.
The descent map is delta(P) = x_M(P) mod squares (x_M the Montgomery abscissa), a homomorphism
E(F_p) → {±1}. The Weierstrass abscissa is x_W = x_M + A/3, so x_M = x_W − shift with shift = A/3.

## Checks and questions (all exact enumeration, no sampling)

1. **Construction check.** The built curve satisfies the conditions above and has
   E(F_p) ≅ Z/8 × Z/ℓ (ℓ = n/8): exactly one rational point of order 2.
2. **Homomorphism check.** delta(P + Q) = delta(P)·delta(Q) on 2,000 random pairs.
3. **Base classes.** For the "bits" base of every Weierstrass abscissa x_W ∈ [0, 2^t) that lies
   on the curve, t ∈ {5, 6} (the cells of the oracle study): the number of base abscissae with
   delta = +1 and with delta = −1.
4. **Hits by class.** For every target R = [a]G, a in a fixed set of 200 random values, list every
   (K, ±P_i, ±P_j) with P_i, P_j in the base and R + K = ±P_i ± P_j (K ∈ E[8], pair sums by
   exact addition). Record, per hit, whether delta(R+K) = delta(P_i)·delta(P_j). Count hits per
   delta(R+K) class.

## Decision rule (premise test)

- **Premise holds** (a fixed-class filter exists) if all base abscissae have the same delta in
  both cells. Then half of the 8 lifts cannot decompose.
- **Premise fails** if both classes occur with fraction at least 10% each. Then no lift is excluded; only the
  pair candidates are class-pruned, by a factor of about 1 − (p₊² + p₋²) (for delta(R+K) = −1)
  or p₊² + p₋² (for +1), where p± is the class fraction.
- Check 4 must report zero hits violating the delta identity; one violation falsifies the theory.

## Out of scope

No F4 cost, no matched rho, no change to the oracle. Any cost saving from pair pruning would
need a measurement in the Rust instrument and a frozen spec under §5a.

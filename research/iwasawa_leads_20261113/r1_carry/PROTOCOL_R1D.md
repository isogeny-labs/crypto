# R1d protocol: weight-3 masks and larger p for ψ (RunPod)

Frozen 2026-11-15, before any data was taken. This extends R1c (`PROTOCOL_R1C.md`) to the two
scopes R1c left untested. The object ψ, the lift L and the half range H = {1, …, (n−1)/2} are the
same as in R1c.

- **Compute:** one RunPod CPU pod (cpu5c, 32 vCPU), running static binaries built from the frozen
  commit and checked by SHA-256 inside the pod. Results are uploaded to S3 and the pod is deleted.
- **Budget:** the owner's $20 ceiling, shared with R1c (about $0.01 spent so far).
- **Timing:** this is an exact computation, so no timing claims are made.

## Instrument changes from R1c (fixing R1c's defects before data)

- **No tied masks.** x-masks range over the bitlen(|H|) bits of x, i.e. the bits that vary on H.
  R1c's masks containing an always-zero top bit duplicated other masks and made the argmax ambiguous.
- **Deterministic argmax.** On equal χ², the lowest (x-mask index, ψ-mask index) wins.
- **Correct positive controls.** The plant sets are x ≡ 0 modulo an odd prime, so the planted
  parities vary. χ² is reported directly at each planted mask pair.

## Part 1: weight ≤ 3 masks (`r1d 20261116 6 2097152 3`)

- **Curves:** six non-anomalous prime-order curves with p a prime in [2²¹, 2²²].
- **S6w3:** max 2×2 independence χ² over all pairs (a, b), where a is an x-mask of weight 1–3 and
  b is a ψ-mask of weight 1–3. T is about 6.8×10⁶. Masks whose parity is constant on H are
  skipped. **Null bound: max χ² ≤ 42.0**, which is Bonferroni at family-wise 0.001 for
  T ≤ 7×10⁶ (per-test α ≈ 1.4×10⁻¹⁰).
- **S7 and S3:** exactly as in R1c.

## Part 2: larger p, weight ≤ 2 (`r1d 20261116 2 16777216 2`)

- **Curves:** two non-anomalous prime-order curves with p a prime in [2²⁴, 2²⁵].
- **Statistics:** S6 (weight ≤ 2, null bound ≤ 33.0), S7 (≤ 30 and within 2× shuffled) and
  S3 (≤ 10), as in R1c.

## Controls (both parts, same curves and instrument)

- **Negative:** the ψ table under a seeded permutation of x.
- **P1 (affine plant):** ψ′ = αx + β mod p. S7 must be at least |H|/2.
- **P2 (weight-2 plant):** set ψ bit 0 to bit₀(x) ⊕ bit₁(x) on x ≡ 0 (mod 101). χ² at the pair
  (x-mask 3, ψ-mask 1) must exceed the part's null bound.
- **P3 (weight-3 plant):** set ψ bit 1 to bit₀(x) ⊕ bit₁(x) ⊕ bit₂(x) on x ≡ 0 (mod 103).
  χ² at the pair (x-mask 7, ψ-mask 2) must exceed the part's null bound. This is checked in Part 1
  only, because Part 2 has no weight-3 masks; P3 is still computed and reported there.
- **Run void:** if P1, P2 or (Part 1) P3 fails, the run is void [pcc:unverified].

## Decision rule

- **The lead stays closed at this scope** if every test curve meets every null bound in its part.
- **The lead survives** if any test curve breaks a bound [pcc:unverified]. Then rerun with fresh
  seeds before saying anything.
- **Report regardless of outcome:** all values, curve parameters, binary hashes, pod id and cost.

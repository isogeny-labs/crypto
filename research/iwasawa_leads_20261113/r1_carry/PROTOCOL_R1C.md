# R1c protocol: scale-up and multi-bit leakage of ψ (Iso Lab)

Frozen 2026-11-15, before any data was taken. This extends `PROTOCOL.md` (R1) and
`PROTOCOL_R1B.md` (R1b). The object ψ and the lift L are the same:
ψ(Q) = log(n·L(Q))/(n·p) mod p, where L keeps x(Q) and Hensel-lifts y.

The run is an exact computation with no timing claims. It runs as one Iso Lab command job on EKS
Spot (cpu-spot, 8 vCPU, 16 GiB, deadline 3600 s) with a $20 ceiling. The owner set that ceiling;
it covers every attempt.

R1/R1b established that ψ(−Q) = −ψ(Q), so x and n − x are paired. **Every R1c statistic therefore
uses only the half range H = {1, …, (n−1)/2}**, one point from each ± pair.

## Arm A: replication on a second host

Run the frozen R1 and R1b binaries unchanged with the fresh seed 20261115. Apply their own frozen
null bounds. This only replicates them on another host.

## Arm B: scale-up (`src/bin/r1c.rs`, seed 20261115)

**Curves.**
- Six non-anomalous prime-order curves with p a prime in [2²¹, 2²²], so n is about 2–4 million.
- Each curve's ψ table covers all x = 1..n−1.

**S6: multi-bit (linear-mask) leakage.**
- x-masks a: every mask of Hamming weight 1 or 2 over the bitlen(n−1) bits of x.
- ψ-masks b: every mask of weight 1 or 2 over the bitlen(p−1) bits of ψ.
- For each pair (a, b), form the 2×2 table of (⟨a, x⟩ mod 2, ⟨b, ψ_x⟩ mod 2) over x ∈ H and take
  its independence χ² with the observed marginals (1 degree of freedom). Masks whose parity is
  constant on H are skipped.
- Report T (the number of pairs, about 6×10⁴), max χ² and the argmax.
- Null bound: max χ² ≤ 33.0. This is Bonferroni at family-wise 0.001 for T ≤ 7×10⁴.

**S7: lag-difference concentration.** This catches affine and locally affine structure at sizes
where the exhaustive affine fit (S2) is infeasible.
- For k = 1..16, take the multiset {ψ_{x+k} − ψ_x mod p : x, x + k ∈ H}.
- Report the largest bin count over all k.
- Null bound: ≤ 30 and within a factor 2 of the shuffled control.

**S3 at scale.** Over 10⁶ seeded random pairs (i, j) with i, j, i + j mod n all nonzero, count
the zero cocycles c = ψ(Q_{i+j}) − ψ(Q_i) − ψ(Q_j) mod p. Null bound: ≤ 10, against an expected
value of about 10⁶/p ≈ 0.3.

## Controls (Arm B)

All controls go through the same instrument on the same curves.

- **Negative:** the ψ table under a seeded permutation of x, run through S6 and S7.
- **Positive P1 (affine plant):** ψ′_x = (αx + β) mod p with seeded α and β. S7 must report at
  least |H|/2.
- **Positive P2 (weak pairwise plant):** ψ″ = ψ with bit 0 replaced by bit₀(x) ⊕ bit₁(x) on the
  x ≡ 0 (mod 100) points (1%). That plants a bias ε ≈ 0.005 at one weight-2 mask (expected
  χ² ≈ 10⁻⁴·|H|, about 100–200 here), so S6 must exceed 33.0 at that mask pair. The plant was
  raised from 0.5% after a toy-size development smoke test showed 0.5% too weak at |H| ≈ 10⁶. That
  test did not use the frozen seed or size.
- **Run void:** if P1 or P2 fails, the run is void [pcc:unverified].

## Decision rule

- **The lead stays closed** if every test curve meets every null bound in S3, S6 and S7, and Arm A
  meets its frozen bounds.
- **The lead survives** if any test curve breaks a bound [pcc:unverified]. Then rerun on six fresh seeds before
  saying anything.
- **Report regardless of outcome:** numbers, curve parameters, image digest, source SHA, Job/Pod
  UID and S3 SHA-256.

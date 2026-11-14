# R1 protocol: does the non-homomorphic p-adic invariant ψ leak the discrete log?

Frozen 2026-11-13, before any data was taken. Exploratory exact computation at toy size, run
locally (seconds, no timing measured), not an isolab benchmark.

## Object

E: y² = x³ + ax + b over F_p, n = #E(F_p) prime. Lift a, b to integers in [0, p). For Q = (x, y)
in E(F_p), L(Q) is the point of Ẽ(Z/p²) with x-coordinate exactly x and y Hensel-lifted. Then
n·L(Q) reduces to O, and t = −X/Y of n·L(Q) lies in pZ/p². Since log(t) ≡ t mod p² for short
Weierstrass curves:

- non-anomalous (n ≠ p): ψ(Q) = (t/p)·n⁻¹ mod p;
- anomalous (n = p): ψ(Q) = t/p mod p (Smart's map, a homomorphism).

## Arms

- **Test arm:** 12 curves with n prime and n ≠ p, p a prime in [2¹³, 2¹⁴]. Curves and the base
  point come from seeded random search (seed list in the code).
- **Positive control:** 4 anomalous curves (n = p) from the same range of p, same instrument.
- **Negative control:** each test curve's ψ table under a seeded random permutation of x.

## Statistics, computed exhaustively over x = 1..n−1 with Q_x = [x]P

- **S1:** #{x : ψ(Q_x) ≡ x·ψ(P) mod p}.
- **S2:** max over (α, β) ∈ F_p² of #{x : ψ(Q_x) ≡ αx + β mod p}. This tests affine leakage in
  the integer x.
- **S3:** over 20000 seeded random pairs (i, j), the cocycle c = ψ(Q_{i+j mod n}) − ψ(Q_i) − ψ(Q_j)
  mod p. Report the fraction with c = 0 and a χ² statistic over 16 equal bins (15 degrees of
  freedom).

## Predictions

- **Positive control:** S1 = n − 1, S2 = n − 1, S3 zero fraction = 1. If any of these fails, the
  instrument is broken and the run is void [pcc:unverified].
- **Test arm, if null (lead closes):** S1 ≤ 10. S2 within a factor 2 of the largest
  negative-control S2 over all curves. S3 zero fraction below 5/p, and S3 χ² below 37.7
  (p = 0.001 at 15 degrees of freedom) on at least 11 of 12 curves.
- **Lead survives:** any test curve breaks one of the null bounds. Then reproduce on 12 fresh
  seeds before saying anything.

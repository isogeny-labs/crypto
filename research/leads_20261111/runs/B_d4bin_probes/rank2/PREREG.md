# PREREG: rank-2 elimination variant of the char-2 D4 (k = 4) Frobenius-form solve

Frozen 2026-11-12 20:46 UTC, before any code for this variant was written or run.
Every statement is [pcc:unverified]; no pcc claim filed.

## Premise (from B_d4bin_probes step 1, [pcc:unverified])

After eliminating the (S2^2, S3^2) part (rank 2 on 300/300 targets, S2*S3 absent), two of the four
`system_frobenius` components are linear in (S2, S3):

    L_i = S2*A_i(s4,P) + S3*B_i(s4,P) + C_i(s4,P),   i = 1, 2
    A_i = s4*(a0 + a1 P + a2 s4),  B_i = b0 + b1 P^2 + b2 s4 + b3 s4 P,  C_i: 12 monomials, total deg <= 4

and two (Q_1, Q_2) carry the squares. Cramer: D = A1B2 + A2B1 (divisible by s4),
S2 = N2/D with N2 = B1C2 + B2C1, S3 = N3/D with N3 = A1C2 + A2C1.
F_j = D^2 * Q_j(N2/D, N3/D, s4, P) is a bivariate polynomial of total degree 12 (deg_P 12, deg_s4 12)
by the free degree count. Bezout bound 144 against the 64-dimensional quotient of the current solve.

## Variant to test (opt-in probe, `examples/d4bin_rank2_elim_probe.rs`)

1. Same `system_frobenius` components (same Weil cost).
2. Row-reduce the 4 x 2 (S2^2, S3^2) coefficient matrix -> L_1, L_2, Q_1, Q_2 (counted).
3. Build D, N2, N3, F_1, F_2 as bivariate polynomials (counted).
4. R(s4) = Res_P(F_1, F_2) by evaluation at deg+1 points of F_q and Newton interpolation
   (univariate resultants by the Euclidean algorithm, counted); strip the factor s4^k.
5. Rational roots of R with the library `roots_with` (baseline squaring, or SqMod with `frob`).
6. For each root s4 (plus s4 = 0 handled by the same back-substitution if needed): P from
   gcd(F_1(s4,.), F_2(s4,.)) rational roots; S2, S3 by Cramer when D != 0; keep a candidate only if all
   four original components vanish (counted verification).
7. Inversions are counted as one multiplication each, as `Gf2q::inv` does.

Comparison: the same 300 targets as `d4_char2_profile --m 12 --seed 1 --trials 300` (curve
`find_bin_curve(12,1,16)`, precompute RNG `1^0x9e`, target RNG 77). Paired per target against
`solve_d4_bin` with the default options: counted F_q multiplications of the system solve
(weil + triang + echelon + nf + charpoly + roots + eigvec + verify; the split phase is identical given
identical solutions and is excluded from both arms, reported separately). Both root-finding arms are
reported: baseline roots vs baseline, frob_roots vs frob_roots.

## Correctness requirement

The set of rational (S2, S3, s4, P) solutions must equal the current path's set on every target.
Any target with a solution where D = 0 (lost by Cramer) or any extra/missing solution counts as a
disagreement and is reported; the variant is not "valid" with any disagreement.

## Predictions (central, 80% interval)

- deg R after stripping s4^k: central 110, interval [64, 144].
- Cost components per trial (frob arm): resultant evaluation ~70k, interpolation ~20k,
  roots of R ~(deg/64)^2 * 23k ~ 70-120k, symbolic build < 10k, back-substitution < 5k.
- Paired ratio variant / current, frob_roots arm: central 1.15, 80% interval [0.8, 1.7].
- Paired ratio, baseline-roots arm: central 1.5, 80% interval [1.0, 2.2].
- P(ratio < 1.0 in the frob arm) = 0.25. P(any solution disagreement) = 0.15 (solutions on D = 0).

## Verdict rule

- Success (lever): identical solutions on 300/300 and paired mean ratio < 1.0 in the frob arm with the
  paired 95% bootstrap interval of the ratio of means wholly below 1.0.
- Falsified: ratio interval wholly above 1.0, or any disagreement not removable by an exact,
  counted fallback.
- Otherwise: inconclusive.

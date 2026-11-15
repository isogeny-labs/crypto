# Smith's conditions on near-prime genus-3 Jacobians (frozen protocol, 2026-11-14)

Follow-up to the lead in `research/genus_ladder_20261114/RESULTS.md`, finding 3: only
1 of 28 admitted near-prime genus-3 curves passed Smith's Propositions 2 and 4,
against 0.169 in the unconditioned census. Every figure `[pcc:unverified]`.

## 1. Mechanism under test

For `H: y² = f(x)`, `deg f = 7`, the rational 2-torsion `J_H[2](F_p)` has `F_2`-rank
`k − 1`, `k` the number of irreducible factors of `f` over `F_p`; so `2^{k−1} | #J` and
a cofactor bound `h <= 8` forces `k <= 4`, `h <= 2` forces `k <= 2`. Smith's tractable
subgroups need a Frobenius-stable pairing of the 8 Weierstrass points (the 7 roots and
`∞`, which is fixed): a pairing exists iff every odd cycle length occurs an even number
of times (`smith_kernel_coverage::has_stable_matching`). For `k = 1` the cycle type is
`(7, 1)`: no pairing, so no tractable subgroup and no pass. Larger `k` gives more
pairings. Hypothesis: near-prime admission lowers the Smith pass rate only through `k`.

## 2. Run

`smith_near_prime run --primes 67,131,257,521,1031 --admitted 150 --max-h 8
--max-seeds 20000 --threads 4 --out near_prime.jsonl` on the binary built from the
commit that contains this file. Per prime, seeds `1, 2, ...` in blocks of 64 until at
least 150 curves with `#J = h r`, `r` prime, `r > h`, `h <= 8` are recorded; every curve
of a started block is recorded (squarefree or not; order determined or not), so the
record is a prefix of the seed order. Per curve: `k`, the cycle type on the 8
Weierstrass points, the combinatorial pairing criterion, Smith's analysis
(`smith_prop24::analyze_curve`: tractable subgroups, Proposition 2 and 4 passes),
exact `#J` (`JCurve::jacobian_order`), `h`, `r`, 2-adic valuation of `#J`.

## 3. Predictions (fixed before data)

Rates are shares of `full_success` (untwisted); intervals are Wilson 95%.

| id | prediction | falsified if |
|---|---|---|
| S1 | `k = 1` curves: 0 passes (theorem check, positive control of the mechanism) | any pass |
| S2 | pooled over primes, the admitted (`h <= 8`) pass rate is below the unconditioned rate of the same seed stream, intervals disjoint | overlap or reversal |
| S3 | the by-`k` mixture predicts the admitted rate: `Σ_k w_k rate_k`, with `w_k` the admitted `k`-distribution and `rate_k` the pass rate at `k` over all curves, lies inside the admitted rate's Wilson interval, pooled | outside |
| S4 | within each `k` with at least 40 admitted and 40 non-admitted curves pooled, the two pass rates have overlapping intervals (admission acts only through `k`) | a disjoint pair |
| S5 | `v_2(#J) >= k − 1` on every curve with a determined order (2-rank identity) | a violation |
| S6 | the first run's 1/28 is consistent with the admitted rate measured here: 1 lies inside the central 95% binomial range for n = 28 at that rate | outside |

## 4. Limits

Odd-degree models only (`∞` rational); Smith's own 18.57% is for the full genus-3 locus.
`full_success` is Smith's condition for his explicit construction, not the existence
of any isogeny to a quartic. No map is built here.

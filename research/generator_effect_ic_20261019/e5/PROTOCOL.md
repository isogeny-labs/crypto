# E5a protocol: x-coordinate statistics of the first multiples of G on P-192 and P-224

**Frozen 2026-10-19, before any E5a run.** Committed alone, before the
example that computes the statistics was run. Any later change is an
amendment with its own date, listed below the original text.

**No ECDLP run. No security claim.** E5a asks only whether the standard
generator's first `2^24` multiples look different, in their abscissae, from
random points of the same curve.

## Curves

`CurveParams::p192()` and `CurveParams::p224()` in `src/ecc/curve_zoo.rs`
(FIPS 186-4 §D.2.1 and §D.2.2). Both have cofactor 1. `n` is the group
order, `p` the field prime, `ℓ = bitlen(p)` (192 and 224).

## Sets

For each curve, three streams of `B_max = 2^24` abscissae `x ∈ [0, p)`,
canonical (not Montgomery) representatives:

| set | element `i` (1-based) | randomness |
|:--|:--|:--|
| `gmul` | `x([i]G)` | none |
| `rscalar` | `x([s_i]G)`, `s_i` uniform in `[1, n − 1]` | seeded |
| `rx` | `x_i` uniform in `[0, p)`, kept when `x³ − 3x + b` is a square in `F_p` (zero included), redrawn otherwise | seeded |

`B = 2^20` is the prefix `i ≤ 2^20` of the same streams. Randomness: the
streams are cut into 256 chunks of `2^16` elements; chunk `c` of set `t`
(`rscalar` = 1, `rx` = 2) on curve `k` (P-192 = 1, P-224 = 2) uses
`rand::rngs::StdRng::seed_from_u64((20261019 << 20) | (k << 12) | (t << 8) | c)`.
Scalars and abscissae are drawn by rejection from `bitlen(n)` and
`bitlen(p)` uniform bits.

## Recorded per (curve, set, B)

- `lz(x) = ℓ − bitlen(x)` histogram (bit length is `ℓ − lz`, so it is the
  same statistic and is not tested separately);
- Hamming weight `hw(x)` histogram and mean;
- the minimum `x_min`, its index `i_min`, and `u = B·x_min / p`;
- the record lows (every `i` whose `x` is below all earlier ones), for the
  running-minimum graph.

## Tests (the family)

Null hypothesis for every test: the `gmul` abscissae have the same law as
the control's.

1. **T-LZ** (8 tests: 2 curves × 2 B × 2 controls). Two-sample chi-square
   homogeneity, `gmul` against the control, on `lz` bins
   `0, 1, …, K − 1` and `≥ K`, `K = log2(B) − 6` (14 at `2^20`, 18 at
   `2^24`; the pooled bin then expects 64 per sample). `df = K`.
2. **T-HW** (8 tests, same cells). Two-sample chi-square homogeneity on the
   Hamming-weight histogram, one bin per integer weight in
   `[⌊ℓ/2 − 3σ⌋, ⌈ℓ/2 + 3σ⌉]`, `σ = √ℓ / 2`, both tails pooled into one bin
   each. `df = bins − 1`.
3. **T-MIN** (4 tests: 2 curves × 2 B, `gmul` only). Under the null the
   minimum of `B` abscissae spread like uniform draws on `[0, p)` has
   `P(u ≤ t) = 1 − e^{−t}` (order-statistic limit; expectation of `x_min` is
   `p/(B + 1)`). Two-sided `p = 2·min(F(u), 1 − F(u))`, `F(u) = 1 − e^{−u}`.
   The record reports `log2(x_min / (p/(B + 1)))` as the distance from the
   expectation.

**Multiple comparisons.** Holm–Bonferroni over these 20 tests, family-wise
α = 0.05. The `2^20` cells are prefixes of the `2^24` cells, so the tests
are dependent; Holm is valid under any dependence.

**Outside the family (calibration, reported, no decision):** the same
T-MIN for `rscalar` and `rx`, and each set's `lz` and `hw` histogram
against its analytic law (`lz` geometric with `P(lz ≥ k) ≈ 2^{−k}`, `hw` of
a uniform `ℓ`-bit integer), chi-square on the same bins.

## Prediction

**No test in the family rejects** after Holm at α = 0.05: nothing in the
abscissae of `[i]G`, `i ≤ 2^24`, separates the standard generator's small
multiples from random points.

A rejection on T-MIN would mean some `[i]G`, `i ≤ 2^24`, has an abscissa far
smaller than chance (for example the lexicographically first point,
`x = 0` on P-192 or `x = 3` on P-224, which would make
`G = [i⁻¹ mod n]·P_first`). A rejection on T-LZ or T-HW would be a
distributional bias; both are reported as findings to explain, not as
weaknesses.

## Checks that must pass before the statistics are read

1. `G` on the curve; `[n]G = O` (BigUint arithmetic of `src/ecc`).
2. The fast arithmetic of the example agrees with `Point::scalar_mul_vartime`
   on `x([i]G)` for `i ∈ {1, 2, 3, 1000, 2^16, 2^16 + 1, 2^20, 2^24}` and on 64
   `rscalar` elements, and the `rx` right-hand side is a square for 64
   sampled elements (Euler's criterion in BigUint).
3. **Positive control (power).** For each curve and `B`, the `rx` stream
   with element `B/2` replaced by the lexicographically first valid
   abscissa must make T-MIN reject at the Holm threshold of the first
   step (`0.05/20`).

## Cross-reference

PLAN §4 E5a; trapdoor program `nist_trapdoor_slot_program_20261011/PROTOCOL.md`
§10 (slot S4, experiment E4a, rule "`G = [h]·(first point)`, `h = 1`").

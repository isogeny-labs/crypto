# M31 single-target curve structure: frozen protocol (2026-11-26)

Frozen before any panel data. Amendments are appended below with reasons.

## Question
Does the B677 M31 curve (or the lab's second M31 curve) carry non-generic structure that lowers
single-target ECDLP cost below an equal-engineering negation-map rho, and how large is the
structural lever on the best structured M31 class (j = 0)?

## Instruments
- `tools/structure.py`: exact structure facts, two methods each where a second exists.
- `native/src/main.rs`: one rho engine, three class policies (`plain` {P}, `neg` {±P}, `aut6` {±ζ^i P});
  identical engineering otherwise (W walks, batch inversion, Mersenne reduction, 32-jump walk, DP, start schedule,
  restart rule). `neg`/`aut6` add Wiener–Zuccherato look-ahead and fruitless-cycle escape (ring of 12, doubling of the
  cycle minimum). Every field multiplication/squaring/inversion counted.
- `tools/run_panel.py`: interleaved arms, frozen seeds (`evidence/seed_manifest.json`), independent Python verifier.

## Design (T = 300 trials per cell)
| curve | arms | W, DP | targets |
|---|---|---|---|
| B677 | plain, neg, neg_aa | 64, 5 | the frozen public Q (log 987654321), 300 independent walk seeds |
| M31 y²=x³−3x+21 | plain, neg, neg_aa | 64, 5 | 300 seeded targets |
| j=0, y²=x³+5, subgroup r=165188041 (cofactor 13) | plain, neg, neg_aa, aut6 | 64, 5 | 300 seeded targets |
| same | neg, aut6 | 16, 3 | 300 seeded targets (engineering sensitivity) |

## Endpoints
Primary: group operations (`steps`, look-ahead rejects included). Secondary: walk field multiplications
(walk + restart/escape, inversions expanded at their counted multiplications); total incl. setup; wall (indicative).
Ratio of means, paired by trial index, percentile bootstrap 10,000 resamples, seed 20261126.
§5a band: canonicalisations, ring compares and DP-table operations priced at 0, 0.1, 1 GAE on every arm.

## Predictions
- neg/plain ≈ 1/√2 = 0.707 (generic, present on every curve).
- aut6/neg ≈ 1/√3 = 0.577 on the j = 0 curve (constant factor; exponent unchanged).
- A/A neg_aa/neg: CI includes 1.
- B677, M31(−3,21): no automorphism beyond ±1 and no isogeny path to j ∈ {0, 1728}; no structural arm exists
  beyond `neg`.
Falsification: A/A CI excludes 1 (instrument problem), or any unverified recovered log.

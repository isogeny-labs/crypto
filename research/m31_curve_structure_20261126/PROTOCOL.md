# M31 single-target curve structure: frozen protocol (2026-11-26)

Frozen before any panel data. Amendments are appended below with reasons.

## Question
Does the B677 M31 curve (or the lab's second M31 curve) carry non-generic structure that lowers
single-target ECDLP cost below an equal-engineering negation-map rho, and how large is the
structural lever on the best structured M31 class (j = 0)?

## Instruments
- `native/src/bin/structure.rs` (+ `src/structure.rs`): exact structure facts, two methods each where a second exists.
- `native/src/main.rs`: one rho engine, three class policies (`plain` {P}, `neg` {±P}, `aut6` {±ζ^i P});
  identical engineering otherwise (W walks, batch inversion, Mersenne reduction, 32-jump walk, DP, start schedule,
  restart rule). `neg`/`aut6` add Wiener–Zuccherato look-ahead and fruitless-cycle escape (ring of 12, doubling of the
  cycle minimum). Every field multiplication/squaring/inversion counted.
- `native/src/bin/panel.rs`: interleaved arms, frozen seeds (`evidence/seed_manifest.json`), independent Jacobian-coordinate verifier plus comparison with the evaluator-held log.

## Design (T = 300 trials per cell; T = 2000 by Amendment 1)
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

## Amendment 1 (2026-11-26, before any panel data was analysed)
- The repository rule "Implementation language: no Python" (crypto AGENTS.md) forbids Python drivers, verifiers and
  result generation. The structure tool and the panel driver were rewritten in Rust (`native/src/bin/structure.rs`,
  `native/src/bin/panel.rs`), with the same seed derivation (SHA-256 KDF, checked to reproduce the same walk seeds).
  One Python-orchestrated run of the planned design was started and its output deleted unread; no number from it is
  used anywhere.
- T raised from 300 to 2000 trials per cell: the native driver solves a B677 instance in about 2 ms, so the larger
  panel costs about a minute on one core and tightens the intervals. All other design elements unchanged.

## Amendment 2 (2026-11-26, after the first T = 2000 run; engineering only)
The first native run (kept unedited in `evidence/v1_engine/`, all 24,000 solves verified) showed the negation-map
arm 1.5x *slower* than plain rho in wall time despite 0.715x the group operations. Diagnosis by per-step walk timing:
branch mispredictions in the sign canonicalisation (a fair coin per step) and a 12-entry full-state cycle ring
scanned every step, plus u64 division in coefficient updates. That is an unequal-engineering defect against the
comparator the task names as fair, so the engine was fixed before any conclusion was drawn:
branch-free sign/orbit selection, division-free coefficient arithmetic (Barrett for the ζ scaling), an x-only cycle
ring checked at distances 2 and 4 every step and fully every 16 accepted steps, with replay-confirmed escape, and a
separate walk-phase wall clock. The walk function, partition, jump table, start schedule, DP rule, seeds and design
are unchanged. The full panel is rerun (same manifest, same T); the v2 run is primary and v1 is reported alongside.

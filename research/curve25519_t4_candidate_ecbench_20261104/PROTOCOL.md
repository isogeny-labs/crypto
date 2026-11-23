# Protocol: T13, the t = 4 candidate end to end, plain and with trace replay (frozen 2026-11-04)

Frozen in git before any data. Parents: wave-4 L10 (optimal t = 4 at every
k; `curve25519_leads_wave4_20261104/l10_scaling_exponent`, validated) and
L12 (trace replay 0.4717× per target at the T12 winner; engine stage
validated). T12 (`curve25519_t10_winner_ecbench_20261104`) is the incumbent.

## Question

T10 scanned t ∈ {5, 6, 7} and found t = 5, G = 4, seed 1307. L10 added
t = 4 and found it optimal at every k. At k = 20 the J/columns seed at
t = 4 is **1193**: 8 columns, 52 covered, mpt(4, 3) = 17,592 mul-adds per
target. The cost model E = columns × r × mpt / covered gives
E(4, 3, 1193) / E(5, 4, 1307) = (8 × 17,592 / 52) / (22 × 52,816 / 210)
= 2,706.5 / 5,533.1 = **0.489**. With trace replay (L12 engine ratio
0.4717 at (5, 4); (4, 3) not yet measured) the modelled ratio is about
0.23. At w = 0 only group operations count, ∝ columns / covered:
8/52 = 0.154 against 22/210 = 0.105, so the t = 4 arm should be **1.47×
dearer** than the winner at w = 0 (the same order flip T12 saw).

## Design

- `ecbench/spec.json`: k = 20 miniature; arms `rho-neg`, `proj-a`,
  `pkm-iso-t5-g4-s1307` (T12 winner), `pkm-iso-t4-g3-s1193`
  (`prime-isogeny:t=4,seed=1193` + `pkm-semaev:m=2,mode=full,guess=3`),
  `pkm-iso-t4-g3-s1193-trace` (same, `trace=1`).
- 16 public targets, `target_seed` base 2026110441, measurement seed base
  2026110442, 2 measured rounds, `alternate_measured`, `--allow-busy`.
- Restart-safe execution exactly as T12's Addendum C: 16 one-target chunk
  sessions by `scripts/chunked_run.py` (copied from T12), pooled band by
  `ecbench band --dir <chunks>`, paired ratios by `scripts/pooled_compare.py`
  (copied from T12; reproduces `ecbench compare` to 6 decimals on T9),
  `ecbench verify --replay 1` per chunk. Binary
  `/home/user/ctarget/release/ecbench` (hash recorded at launch). A killed
  chunk is moved to `interrupted/` and re-run; nothing killed is cited.
- If ecbench rejects `trace=1` for this oracle, the trace arm is dropped
  and R4–R5 are reported as not run.

## Predictions (decided on the pooled chunks)

- **R1.** Every arm verifies 32/32; every chunk's replay audit reports
  0 problems.
- **R2.** S(t4)/S(winner) ≤ 0.6 at w = 0.1 and at w = 1, with the 95%
  upper bound < 1. Point: 0.49.
- **R3.** S(t4)/S(winner) ≥ 1.2 at w = 0 (point 1.47): the order flips.
- **R4.** S(t4-trace)/S(t4) ≤ 0.6 at w = 0.1 and at w = 1, with the 95%
  upper bound < 1 (point ≈ 0.47–0.57 from L12's engine ratios).
- **R5.** For every paired (workload, round), total_gae and the recovered
  scalar are identical between the two t = 4 arms (same base, same
  trials, F4 verdicts identical by L12 P1), so the w = 0 ratio is 1.000
  exactly. If the plan does not pair algorithm seeds across arms, R5
  is replaced, before looking at the ratio, by: w = 0 ratio in [0.9, 1.1]
  with the interval covering 1.
- **R6.** Every PKM arm stays behind `proj-a` and `rho-neg` at w = 0.1 and
  at w = 1 (band ranks).

## Costs and scope

- Seed search at t = 4 (2,000 seeds): 4.00·10⁶ group operations
  (`curve25519_isogeny_seed_20261103/analysis/summary.json`), 1.10·10⁴ in
  S; reported charged to one target and amortised. The trace learning
  cost is one plain target per branch system, charged from the engine
  measurement.
- Algebraic-track component result at k = 20. Rho remains ahead; L10
  gives the exponent (1.03 against 0.5), which no constant factor here
  changes. No leaderboard row; no 255-bit statement.
- A failed prediction is reported as failed and not retuned.

# Protocol: jump count of the F = 1 target-in-walk arm, single-target ecbench (frozen 2026-11-08)

Frozen before any measured data of this study. Nothing below changes after the first session
runs; failed predictions stay failed and are reported as failed. Before freezing, the only
executions were `ecbench plan` (no run) on a throwaway spec to confirm that `jumps=2`, `4` and
`64` are accepted by the `tw=1,cj=1` oracle, and the pre-data model `scripts/model.py`, which
uses the earlier study's measured phase table and no data of this study.

## 1. Question

The current best single-target arm on this family at w = 0 is `twcj-x15`
(`wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,seed=run` on a one-abscissa base;
`research/curve25519_f1_targetwalk_20261108`, 1.40x rho-neg at k = 20). Its set-up (jump
construction) costs 0.368 sqrt(r) at k = 20, 17 % of its total. In `TargetWalk` the jump index
of every step is drawn from a seeded stream independently of the point, so the walk is a
random sum of jump points, not an r-adding walk whose trajectories merge; the Brent-Pollard
jump-count correction does not apply. With m jumps and n steps the pairs of positions have
about n^((m+1)/2) distinct difference vectors (derived; the count vectors of two positions
differ in a sum-(t - s) vector whose spread is about sqrt(t - s) per coordinate), so for
m >= 4 that exceeds the n^2/2 pairs and the collision count stays n^2/r as for a uniform
walk, while for m = 2 it falls to about n^(3/2)/r. Does cutting the jump count to 8 or 4
lower the end-to-end cost by the saved set-up with no walk penalty, and does m = 2 fail?

## 2. Implementation under test

No code change. The existing `wwalk` oracle option `jumps` (2 to 64, default 16) on the
`tw=1,cj=1` arm; binary `/home/user/ctarget-f1/release/ecbench` as built for the earlier
study (SHA-256 recorded per size in `data/binary-k{20,24}.sha256` when the sessions start).
That binary has `ecbench band --price-table-work` (PR #3700's opt-in insert pricing), so both
band readings are reported.

## 3. Design (frozen)

- Curves: k = 20 (`p = 1,048,573`, `r = 131,203`) primary; k = 24 (`p = 16,777,213`,
  `r = 2,097,349`) replicate. The same explicit curves as the earlier studies.
- Sessions: 8 per k, each with `targets_per_curve = 1` and its own `target_seed` and
  measurement `seed` (`scripts/make_specs.py`), disjoint from the earlier studies' seeds.
  Each arm runs 64 measured rounds plus 1 warm-up, `order = alternate`. Warm-up records are
  excluded from every pooled figure.
- Arms (all pipeline arms use `prime-projected-abscissa:size=1,scan_cap=32`,
  `targets=stride-generator`, and `wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,seed=run` with the jump
  count varied):
  - `rho-neg` (reference): `rho.negation`.
  - `twcj-j16` (baseline, the current arm): `jumps=16`.
  - `twcj-j2`, `twcj-j4` (**primary**), `twcj-j8`, `twcj-j32` (candidates).
- Counted unit: ecbench GAE; `S = cost/sqrt(r)`; every comparison at w = 0, 0.1, 1
  (AGENTS.md section 5a), under the committed band and under `--price-table-work`.
- Audit: `ecbench verify --replay 1` per session.
- Pooling: `scripts/pooled_compare.py`, copied unchanged from the earlier study (warm-up
  records skipped). Per session, sum S over the measured rounds per arm; ratio of sums;
  linearised t interval with df = 7. One session is one target and one cluster.
- Analysis: `scripts/analyse.py` -> `data/pooled_measured.json` (means, ratios, phase
  breakdown from the `tw_*` counters, walk-step diagnostics).
- Runtime cap: 30 minutes in total. The earlier study took about 50 s per size with 4 arms.

## 4. Model (derived, before data; `scripts/model.py` -> `data/model.json`)

Set-up GAE = doublings (3 chains) + K_0, K_1 evaluations + per-jump cost, read from
`TargetWalk::cheap_jumps`: per jump one chained addition, popcount(alpha) - 1 additions for
the P1 part and one addition to combine, about 3.5 GAE at these sizes. The model's set-up for
16 jumps is 128 GAE at k = 20 and 153 at k = 24, against the measured 133 and 159. The walk
(1.390 and 1.394 sqrt(r)), verification and fixed phases are taken as measured for `twcj-x15`.

| k | arm | d(set-up)/sqrt(r) | model total S | ratio to j16 | ratio to rho-neg |
|--:|:--|--:|--:|--:|--:|
| 20 | j2 | -0.135 | 5.23 (cap + probes) | 2.44 | 3.41 |
| 20 | j4 | -0.116 | 2.022 | 0.946 | 1.321 |
| 20 | j8 | -0.077 | 2.061 | 0.964 | 1.346 |
| 20 | j32 | +0.155 | 2.293 | 1.072 | 1.497 |
| 24 | j4 | -0.029 | 1.594 | 0.980 | 1.382 |
| 24 | j8 | -0.019 | 1.604 | 0.986 | 1.392 |
| 24 | j32 | +0.039 | 1.662 | 1.027 | 1.450 |

The j2 row is a direction, not a point: the walk is expected to reach the cap 3T = 4.5 sqrt(r)
below rank 2 and then to fall back to probes.

## 5. Predictions and thresholds (frozen)

"Ratio" is the pooled ratio of sums with its 95 % interval over 8 sessions, at w = 0 unless
stated. The jump-count change moves only counted group operations (set-up additions) and the
walk length; it adds no native work, so the w = 0.1 and w = 1 verdicts follow the w = 0 ones.

- **P1 (primary).** k = 20: S(twcj-j4) / S(twcj-j16) has its upper end < 1.00 at w = 0,
  0.1 and 1 (committed band). Point estimate expected in [0.92, 0.98] (P1b, scored separately).
- **P2.** k = 20: S(twcj-j8) / S(twcj-j16) point estimate in [0.93, 0.99].
- **P3 (no walk penalty).** For j4 and j8 at both sizes, the mean `tw_steps` / sqrt(r) is
  within 3 % of the j16 arm's mean in the same sessions (ratio in [0.97, 1.03]).
- **P4.** k = 20: S(twcj-j32) / S(twcj-j16) has its lower end > 1.00.
- **P5 (two jumps fail).** Both sizes: S(twcj-j2) / S(twcj-j16) has its lower end > 1.50,
  and the j2 arm reaches the cap below rank 2 (`tw_rank < 2` at the end of the walk) in at
  least 90 % of its records.
- **P6 (replicate).** k = 24: S(twcj-j4) / S(twcj-j16) point estimate in [0.96, 1.00].
- **P7 (no parity).** S(twcj-j4) / S(rho-neg) has its lower end > 1.00 at w = 0, and under
  `--price-table-work` at w = 0, 0.1 and 1, at both sizes. The w = 0 point estimate is
  expected in [1.25, 1.40] at k = 20 (P7b, scored separately).
- Every record must be `verified`, and every session's `ecbench verify --replay 1` must
  reproduce. A session that fails the audit is excluded and reported; it is not rerun.

## 6. Reporting

README.md tabulates measured S per arm and weight (with source file), the pooled ratios, the
status of P1-P7, the phase breakdown, the walk-step diagnostics, the distance to rho-neg at
w = 0, 0.1 and 1 under both band readings, and the scope statement of the earlier study: an
F = 1 target-in-walk arm is one unknown away from rho; with P1 := G it is rho with a stored
walk; it is not decomposition-based index calculus and proposes no leaderboard row. Every
result sentence ends with [pcc:unverified].

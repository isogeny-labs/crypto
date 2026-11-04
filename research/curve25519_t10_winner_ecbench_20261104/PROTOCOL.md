# Protocol: T12, end-to-end ecbench test of the T10 winner (frozen 2026-11-04)

Frozen in git before any data. Parent leads:
- wave-3 T10 (`research/curve25519_leads_wave3_20261103/t10_seed_and_guess`,
  validated pass_with_errata);
- wave-3 T9 (`.../t9_seed_per_column`).

## Question

T10 measured the per-target F4 cost across (t, G, seed) on the k = 20
miniature. It combined that with exact census coverage and columns into a
modelled F4 work per solve, E ∝ columns × mpt / covered. The argmin over 36
cells is **t = 5, G = 4, seed 1307**: 22 columns, 210 covered, 52,816 mul-adds
per target. That is 18.65× below the incumbent (t = 6, G = 4, seed 0: 25
columns, 172 covered, 710,094 per target) and 9.28× below the T9 best
(t = 6, G = 4, seed 1308: 36 columns, 498 covered, 710,095 per target).

Does that modelled advantage hold end to end, with full discrete-log solves
priced across the §5a band?

## Design

- `ecbench/spec.json`. Same k = 20 curve, oracle family and reference arms as
  T5 and T9. The arms are:
  - `rho-neg`;
  - `proj-a`;
  - `pkm-iso-t6-g4` (seed 0, incumbent);
  - `pkm-iso-t6-g4-s1308` (T9 best);
  - `pkm-iso-t5-g4-s1307` (T10 winner).
- The PKM arms all use `pkm-semaev:m=2,mode=full,guess=4`.
- 16 public targets, `target_seed` 2026110411, 2 measured rounds
  (`alternate_measured`), measurement seed 2026110412.
- `scripts/run.sh`:
  - binary `/home/user/ctarget/release/ecbench`, with its hash in
    `ecbench/binary.sha256`;
  - `run --allow-busy`, because the host runs other jobs. S is a counted unit,
    and wall time is not claimed;
  - `band` (w = 0, 0.1, 1);
  - paired `compare` of the winner against each t = 6 arm;
  - `verify --replay 16`.

## Model predictions behind the rules (ratios, winner ÷ comparator)

- At w ≥ 0.1, F4 work dominates. The model gives 0.054 against seed 0 and
  0.108 against seed 1308.
- At w = 0 only group operations count, ∝ columns / covered. The model gives
  0.72 against seed 0 and **1.45 against seed 1308**: the winner should be
  dearer than seed 1308 on group operations alone.

## Predictions (decided from `ecbench band` / `compare` on this session)

- **Q1.** The winner verifies every run (32/32), and the replay audit reports
  0 problems.
- **Q2.** S(winner)/S(seed 0) ≤ 0.15 at w = 0.1 and at w = 1, with the 95%
  upper bound < 1.
- **Q3.** S(winner)/S(seed 1308) ≤ 0.25 at w = 0.1 and at w = 1, with the 95%
  upper bound < 1.
- **Q4 (order flip at w = 0).** S(winner)/S(seed 1308) ≥ 1.2 at w = 0 (point
  estimate). This is a risky prediction from the columns/covered model.
- **Q5.** Every PKM arm stays behind `proj-a` and `rho-neg` at w = 0.1 and 1
  (band ranks).

## Failure handling and scope

- A failed prediction is reported as failed and not retuned.
- The seed search (2,000 seeds at t = 5: 6.94·10⁶ group operations, from
  `curve25519_isogeny_seed_20261103/analysis/summary.json`) and T10's G sweep
  are per-curve precomputation. They are reported both charged to one target
  and amortised.
- This is an algebraic-track component result. Rho remains ahead, and the
  distance remaining is stated, not gated.
- No leaderboard row.
- Conductor and the AWS CLI are absent, so the run is local with no S3
  checkpoint. A container restart kills in-flight runs; a killed session is
  re-run from scratch, never resumed or cited as partial.

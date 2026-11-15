# Protocol: T4 seed-selection replication across k (engine check), frozen 2026-11-03

Status: preregistered design. No engine measurement has been taken for this
protocol. The only data read to write it are the existing census files listed
under "Inputs". Anything reported later that is not listed here is
**exploratory**.

## Question

At k = 20, t = 6, G = 4, the log-free selector J picked isogeny seed 1093
(`research/curve25519_isogeny_seed_20261103`, R4). Against seed 0, the F4
engine showed:

- coverage 528 against 172, matching the census (3.07x);
- the same solving degree (5) and terms per specialisation (16);
- mul-adds per target of 0.991x.

The derived F4 mul-adds per relation therefore fell by about the coverage
ratio (3.10x).

Does that engine behaviour replicate on the other family miniatures,
k in {16, 18, 22, 24}, at the same (t, G)? Concretely: is the seed a free
parameter for the F4 system there too, so that the census coverage gain turns
one-for-one into a per-relation F4 saving? A secondary cell, k = 24, t = 5,
G = 3, asks the same question at a smaller tower.

## Inputs (read-only, hashed now)

- Binary: `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be`. This
  is the same hash as `curve25519_isogeny_seed_20261103/engine/binary.sha256`.
  It is not rebuilt. `run.sh` refuses to start if the hash differs.
- Census files `research/curve25519_isogeny_seed_20261103/data/k<k>.jsonl`
  (seeds 0..1999, 2000 rows per (k, t), no build failures):

  | file | sha256 |
  |---|---|
  | k16.jsonl | 3935c968587dd631c7cebfaaa4f530b03a5ed6da4045ab9cf9ba14504699d498 |
  | k18.jsonl | 4e670176d073d90c7f7dd7a6cd951f25072330128794e981eeb48181696d84cf |
  | k20.jsonl | bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc |
  | k22.jsonl | 11903b11c6d1f2eccfadd27b058e95fb2410999b21e22afc3a9854220c770e9e |
  | k24.jsonl | 39c7543607717def3ff68ef852151f482ea6ff0a13b33a9bd00bc34a2329eba0 |

## Curves

These are the family-rule Curve25519 miniatures k = 16 (A = 1058),
k = 18 (A = 78), k = 20 (A = 926), k = 22 (A = 43186) and k = 24 (A = 318).
The subgroup order r below is read from the census as
covered_points / coverage_exact.

## Seed selection (computed now from the census files, fixed)

The rule is the T4 rule, unchanged: S* = argmax J, then most columns on a
tie, then the lowest seed. The baseline is seed 0, the `isogeny` form.

| cell | r | S* | J(S*) | cols S* / 0 | census covered S* / 0 | census coverage_exact S* / 0 | seed0_cov / sel_cov |
|---|---|---|---|---|---|---|---|
| k16 t6 G4 | 8209 | 1381 | 632 | 40 / 28 | 614 / 198 | 0.07479595565842369 / 0.024119868437081252 | 0.32248 |
| k18 t6 G4 | 32887 | 1277 | 558 | 45 / 30 | 554 / 230 | 0.016845562076200322 / 0.006993644905281722 | 0.41516 |
| k22 t6 G4 | 524731 | 1070 | 568 | 46 / 32 | 568 / 270 | 0.0010824593934797067 / 0.0005145493595766212 | 0.47535 |
| k24 t6 G4 | 2097349 | 1490 | 514 | 41 / 39 | 514 / 364 | 0.0002450712780753227 / 0.00017355242260587057 | 0.70817 |
| k24 t5 G3 (secondary) | 2097349 | 797 | 190 | 24 / 17 | 190 / 76 | 9.059055026130606e-05 / 3.623622010452242e-05 | 0.40000 |
| k20 t6 G4 (control C0) | 131203 | 1093 | 528 | 46 / 25 | 528 / 172 | 0.004024298224888151 / 0.0013109456338650793 | 0.32576 |

Notes on the choices:

- The only J tie in these cells is k24 t5: seeds 797 (24 columns) and 978
  (20 columns) both have J = 190. The column rule picks 797.
- At k16 and k18 the J maximum is above the covered points (632 against 614,
  558 against 554). The selected seed is still the oracle best in each cell.
- At k24 t6 the expected gain is the smallest (1.41x). This is the cell most
  likely to show that a fixed per-target overhead hides the gain.

## Design (exact commands)

Common settings:

- binary B = `/home/user/ctarget/release/curve25519_family`;
- 100 targets, target_seed 2026111506 (the same seed as the k = 20 R4 run);
- cap_s 3600, so the wall cap never truncates;
- m = 2, mode full, lifts 0, max_nnz 30000000.

There is one process at a time and no parallelism. Each run's stdout (one
JSON line) goes to `data/<label>.json` and its stderr to `data/<label>.stderr`.
The order runs seed 0, then S*, alternating within each cell. Primary cells
run before the secondary cell.

| # | label | command |
|---|---|---|
| 0a | k20_t6_g4_s0 (control) | `B f4cost 20 isogeny 6 4 100 2026111506 3600 2 full 0 30000000` |
| 0b | k20_t6_g4_s1093 (control) | `B f4cost 20 isogeny@1093 6 4 100 2026111506 3600 2 full 0 30000000` |
| 1a | k16_t6_g4_s0 | `B f4cost 16 isogeny 6 4 100 2026111506 3600 2 full 0 30000000` |
| 1b | k16_t6_g4_s1381 | `B f4cost 16 isogeny@1381 6 4 100 2026111506 3600 2 full 0 30000000` |
| 2a | k18_t6_g4_s0 | `B f4cost 18 isogeny 6 4 100 2026111506 3600 2 full 0 30000000` |
| 2b | k18_t6_g4_s1277 | `B f4cost 18 isogeny@1277 6 4 100 2026111506 3600 2 full 0 30000000` |
| 3a | k22_t6_g4_s0 | `B f4cost 22 isogeny 6 4 100 2026111506 3600 2 full 0 30000000` |
| 3b | k22_t6_g4_s1070 | `B f4cost 22 isogeny@1070 6 4 100 2026111506 3600 2 full 0 30000000` |
| 4a | k24_t6_g4_s0 | `B f4cost 24 isogeny 6 4 100 2026111506 3600 2 full 0 30000000` |
| 4b | k24_t6_g4_s1490 | `B f4cost 24 isogeny@1490 6 4 100 2026111506 3600 2 full 0 30000000` |
| 5a | k24_t5_g3_s0 (secondary) | `B f4cost 24 isogeny 5 3 100 2026111506 3600 2 full 0 30000000` |
| 5b | k24_t5_g3_s797 (secondary) | `B f4cost 24 isogeny@797 5 3 100 2026111506 3600 2 full 0 30000000` |

Control C0 re-runs the published k = 20 pair with cap 3600 instead of 900.
The cap did not bind there, so the counted fields should reproduce exactly.
C0 checks that the instrument and binary still give the same counts. It is
not a new result.

Quantities, all counted (no timing claims; `wall_s` is logged only):

- `covered_points` and `coverage_exact`;
- `solving_degree_max` and `poly_terms_per_specialisation`;
- `muladds_per_target` (MPT) and `muladds_per_relation_at_exact_coverage`
  (MPR = MPT / coverage_exact);
- `hits` and `hits_verified`;
- `capped`, `targets_done` and `undecided`.

Derived ratios per cell:

- rho_T = MPT(S*) / MPT(0);
- rho_R = MPR(S*) / MPR(0);
- c = seed0_cov / sel_cov, from the census table above.

Identity: rho_R = rho_T x c holds exactly when P1 holds.

## Validity gate (checked before any prediction is scored)

- **V.** Every run exits 0, prints one JSON line, has `capped = false`,
  `targets_done = 100`, `undecided = 0` and `hits_verified = hits`.
- A run that fails V is not scored. If the failure is a crash or empty output
  (not `capped` and not `undecided`), the identical command is re-run once and
  the event is logged. A second failure is reported as "not measured". A
  `capped` or `undecided` run is reported as such. It is not re-run with a
  different cap or nnz limit.

## Predictions

Scored per cell. "Primary cells" means k16, k18, k22 and k24 at t6 G4.

- **P0 (control, k20).** In runs 0a and 0b, each of `f4_muladds`,
  `covered_points`, `solving_degree_max`, `poly_terms_per_specialisation` and
  `hits` equals the value in
  `research/curve25519_isogeny_seed_20261103/engine/isogeny_t6_g4.json` and
  `isogeny_1093_t6_g4.json` (seed 0: 71009400, 172, 5, 16.0, 0; seed 1093:
  70384751, 528, 5, 16.0, 1). Rule: exact equality on all five fields in both
  runs, or P0 fails. If P0 fails, the instrument is not reproducible, and
  P1–P4 are reported as exploratory only.
- **P1 (coverage equals census).** For both runs of a cell, the engine
  `covered_points` equals the census covered points in the table above, and
  `coverage_exact` equals the census value within an absolute 1e-12. Rule:
  exact integer equality, or P1 fails for that cell.
- **P2 (same algebra).** Within a cell, S* and seed 0 have equal
  `solving_degree_max` and equal `poly_terms_per_specialisation`. Rule: exact
  equality of both fields, or P2 fails for that cell.
- **P3 (same per-target cost).** 0.90 <= rho_T <= 1.10 in each cell. Rule:
  rho_T outside [0.90, 1.10] fails P3 for that cell. The value is reported to
  4 significant figures either way.
- **P4 (per-relation saving tracks coverage).** 0.90·c <= rho_R <= 1.10·c in
  each cell. The predicted rho_R values are:

  | cell | predicted rho_R | predicted reduction |
  |---|---|---|
  | k16 | 0.3225 | 3.10x lower |
  | k18 | 0.4152 | 2.41x lower |
  | k22 | 0.4754 | 2.10x lower |
  | k24 t6 | 0.7082 | 1.41x lower |
  | k24 t5 (secondary) | 0.4000 | 2.50x lower |

  Rule: rho_R outside the band fails P4 for that cell. If P1 holds, P4 is
  equivalent to P3.
- **P5 (replication verdict).** The T4 engine result replicates if P1–P4 all
  hold in all 4 primary cells. It replicates partially if they hold in 2 or 3
  primary cells: the failing cells are named and the claim is restricted to
  the passing k. It fails if they hold in 0 or 1 primary cells. The secondary
  cell is scored and reported separately. It does not enter P5.

Hits are not predicted. With 100 targets and coverage at or below 7.5%, hit
counts are too small to score. They are reported as exploratory counts only.

## Failure handling

- A failed prediction is reported as failed, with the measured value next to
  the predicted band.
- No re-selection of S* (no oracle seed, no second-best seed), no change of
  G, t, target seed or target count, and no extra targets to rescue a band.
- If P2 or P3 fails in a cell, the report states which field moved:
  - degree, terms, `calls_per_target`, `solver_extra.max_cols_max` or
    `max_rows_max`;
  - whether the larger column count (for example 40 against 28 at k16) is the
    mechanism.

  The report labels that diagnosis **exploratory**.
- If any run's binary hash, census hash or command differs from this
  protocol, the run is void and is reported as void.
- If the secondary cell (5a/5b) is not run for lack of time, it is reported
  as "not run". It is never partially run or extrapolated.

## Scope

- This is a component check of the F4 phase only. It asks whether a coverage
  gain bought by seed selection turns into a per-relation F4 saving at fixed
  per-target cost.
- It does not price the seed search. At k = 20, t = 6, the 2000-seed search
  cost 1.35·10⁷ group operations, which dominates one solve. It must be
  charged, or amortised and labelled as such, in any end-to-end (ecbench)
  figure.
- It is not an end-to-end solve, a speed claim or a leaderboard row.
- It is not a relabelling: the same instrument, the same counted unit and the
  same targets are used on both arms.
- A passing result says the T4 improvement over the seed-0 incumbent is not a
  k = 20 accident. It says nothing about larger k beyond the cells measured.

## Distance remaining

The algebraic (PKM-isogeny) track is far behind Pollard rho at every
measured k, and rho remains ahead. Even a full replication cuts only the F4
per-relation cost, by 1.4–3.1x. That is a constant factor inside a phase
whose per-relation cost is already ~10⁸ mul-adds at k = 20, against a rho
walk of about √r ≈ 362 group operations. The report will state each cell's
MPR next to the matched rho reference for that k, as the distance remaining,
and will not present the ratio as closing the gap.

## Outputs

- `data/<label>.json` and `.stderr` for each run;
- `data/run.log` (the commands, start and end UTC, exit codes);
- `data/SHA256SUMS`.

The analysis and report (with graph and PDF, research-visuals) follow after
the runs and score P0–P5 exactly as written here.

---
title: "Protocol: prefix inserts (L1) and a joint set-up (L2) for the k-sum -psr arm on Table B"
date: 2026-11-18
status: registered before any confirmatory run (hashes in REGISTRATION.sha256)
---

# Protocol: two set-up levers for the k-sum `-psr` arm, a 2x2 on Table B's eight curves `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Nothing here moves the algebraic track.

## 1. Incumbent, profile, levers

**Incumbent.** The `-psr` arm of `research/ic_ksum_split_20261118` (validator VAL-20261118-1e81cb): `ic.pipeline`,
`oracle = ksum:m=<k>,prune=1,split=1`, `targets = stride-renew`, the producer's factor-base sizes, and per row the k that study
found cheaper at w = 0: k = 5 on P20, P22, P24, B21, B24, B27; k = 4 on B15, B18. The (k, N) retune
(`research/ic_ksum_retune_20261118`) was a null, so the sizes stay.

**Profile (prior data, the retune README):** set-up 28-57 % of S at w = 0 (table build plus the walk's fixed set-up), trials
43-67 %; a trial hits with probability about 2T/r, `T = 2^(k-1) C(N, k)`; relations about N + 0.6. The walk's fixed set-up is
three `[a]G + [b]Q` (the stride S, the renewal step T, the start R_0), each two double-and-add scalar multiplications and one
addition: about `3 (2 log2 r doublings + log2 r additions)`.

**L1, prefix inserts (`prefix=1`, new; `KSumTable::build_negation_folded_prefix`).** The pruned, split build forms partial
sums of `j < k` summands on its way to the k-sums: the base points (depth 0), each lower level's partials, and the pair-sum
table's sums. With `prefix=1` each of those (one per negation orbit, non-identity) is also keyed, after the build and only under
a key no k-sum took, so the k-sum entries, their keys and collisions are those of the build without it. No group operation is
added; each prefix insert is one more counted `table_insert_attempts` (priced as a hash insert in `bitop/v1`; not in GAE, as
for every table insert). A target equal to `+-` such a sum decomposes into `j` summands; the row then has fewer non-zero base
columns, which the elimination accepts as it accepts any row (`Relation::Summands` of any length), and the probe's
confirmation costs `j - 1` additions instead of `k - 1`. Packed entries: unused slots hold index `2^bits - 1`, which no base
point has (`fb.points.len() < 2^bits` is enforced). Expected: prefix entries about 4-9 % of T (B27 4 %, P20/B21/B24/B15/B18
8-9 %), so the trial count falls by about that fraction and S by about the trial share of it: **R_0 about 0.95-0.98**.

**L2, joint set-up (`targets=stride-renew-joint`, new; `TargetSource::StrideRenewJoint`).** The same three `[a]G + [b]Q`, with
coefficients drawn from the same stream in the same order, each formed by one joint double-and-add over `G + Q` (Shamir's
trick, the existing `joint_mul`), `G + Q` added once. The points are equal, so the targets, trials, relations, rows and answer
are those of `stride-renew`, run for run; only the relation phase's set-up additions and doublings fall (about
`log2 r` doublings and a quarter of the additions per pair: `3.75 (log2 r - 1) + 2` GAE in expectation), every one still
counted, and the recorded `scalar_mults` count is 3 instead of 6. Expected ratio `1 - dS/S`: B15 0.93, B18 0.94, P20/B21
0.986, B24 0.990, P22 0.991, P24 0.995, B27 0.996 at w = 0 (smaller still at w > 0, where S is larger).

Both are constant factors: L2 removes a fixed cost against sqrt(r); L1 raises the hit rate by a few per cent. No exponent
changes.

**Unit tests** (`cargo test --release --locked --lib ksum`): `prefix_build_keeps_the_ksum_table_and_adds_shorter_sums`
(ic_boundary.rs: k = 3..6 prime, 3..5 binary, split on and off; every k-sum entry kept, same build ops and collisions, every
prefix entry a correct decomposition found by a probe of `+-` its sum); `prefix_off_is_identical_and_on_adds_no_setup_operation`
and `stride_renew_joint_is_stride_renew_with_a_cheaper_setup` (plugins.rs: pipeline runs, k = 4/5, three seeds; joint = same
trials, relations, rank, answer, every relation counter, fewer GAE, half the scalar multiplications).

*Sources checked* (the research-kb MCP server and the lead atlas are unavailable in this container):
`research/ic_ksum_split_20261118/` (README, PROTOCOL, specs), `research/ic_ksum_retune_20261118/` (README, PROTOCOL, analysis),
`src/cryptanalysis/ic_boundary.rs` (`KSumTable`, `collect_and_solve_relations_*`, `draw_jumps`, `joint_mul`),
`src/cryptanalysis/ic_framework/plugins.rs` (`KSumOracle`), main's `sib` option (ad2a014c00: one inversion per last-level
signed pair, prime curves; a different lever, not duplicated here).

## 2. Pilot (not data)

8 sessions (one per row), fresh seeds (`target_seed = 20261118400 + i`, `seed = 20261118450 + i`, i = row index), 64 public
targets, 1 round, no warm-up; arms `rho-neg`, `inc`, `prefix`, `joint`, `prefix-joint` (`pilot/specs/`, `run_pilot.sh`, the binary
below). 2,560 runs, all verified; 8/8 `verify --replay-all --exit-code` rc 0. Summary `pilot/pilot_table.md`,
`pilot/pilot_summary.json` (`pilot/analyse.py`), `pilot/correctness.json` (`analysis/correctness.py`). Record
`benchmarks/records/20261118T1059Z-ksum-prefix-joint-pilot.json`. Raw sessions deleted after summarising (digests in
`pilot/sessions.sha256`).

- **L2 (`joint / inc`)**: 512/512 pairs identical in outcome, log, trials, relations, rank, every relation counter and the
  oracle set-up; the relation phase saves 32-95 GAE a run (mean 43 on B15 to 82 on B27), scalar multiplications 6 -> 3
  (8 -> 4 after a stride restart). Ratios at w = 0: B15 0.940, B18 0.952, P20 0.986, B21 0.987, B24 0.991, P22 0.992, P24 0.995,
  B27 0.997, intervals +/- 0.001-0.003 (w = 1: 0.973-0.999). As predicted.
- **L1 (`prefix / inc`)** at w = 0: B24 0.913 [0.868, 0.959], B18 0.936, B21 0.937, P20 0.956, B15 0.961, P22 0.964, B27 0.974,
  P24 1.006 [0.983, 1.029]; about flat across w (insert attempts are not GAE). Prefix entries 37 (B15, T = 280) to 199 (B24,
  T = 2,016): 10-13 % of T, more than the 4-9 % first estimated (the pair sums and the unpruned low levels add to it).
  Interval half-width about 0.025-0.045 at 64 pairs.
- **Both**: 0.888 (B18) to 1.001 (P24) at w = 0.
- **Sizing.** With 128 targets x 3 rounds x 3 replicates (1,152 pairs a row, 384 a replicate) the per-replicate half-width is
  about 0.012-0.018 for L1: enough for effects of 4 % or more, not for P24/B27's 0-3 %. The confirmatory design takes
  `targets_per_curve = 128` (changed from the drafted 96 before any confirmatory run).

## 3. Confirmatory design and registered tests

**Rows.** All eight Table B rows (P20, P22, P24, B15, B18, B21, B24, B27), the incumbent's (k, N) per row.

**Sessions.** 3 replicates per row, 24 sessions (`specs/<row>-r<rep>.json`, `make_specs.py confirm`). Curves, max_trials,
timeout, L0 and order `alternate` copied from the split study's spec of the same name; **targets_per_curve 128**, 3 measured
rounds, 1 warm-up. **Fresh seeds:** `target_seed = 20261118800 + idx`, `seed = 20261118900 + idx`,
`idx = 10 * (row index) + rep` (never used by the split study, 000/500 + idx, the retune, 200/300 + i and 600/700 + idx, or this
pilot, 400/450 + i). Arms: `rho-neg` (reference), `rho-neg-aa` (control), `inc`, `prefix` (L1), `joint` (L2), `prefix-joint`
(both); same targets and algorithm seed per round.

**Binary.** `ecbench` built with `cargo build --release --locked --bin ecbench` in a clean worktree of the commit that adds
both levers, its own target directory; sha256 in `BINARY.sha256`, checked by `run_all.sh`; `strings` shows
`stride-renew-joint` and the `prefix` parameter text.

**Route.** Local, `--cpus none`, L0, counted cost only, one session at a time under `flock` on the scratchpad CPU lock,
inside `tools/benchrec.py run`. `ecbench verify --replay-all --exit-code` on every session. Budget at most 3 CPU-hours in total
(pilot included).

**Tests T1 (L1) and T2 (L2), per row.** `R_w = cost(b) / cost(a)` with (a, b) = (inc, prefix) for T1 and (inc, joint) for T2,
paired by target and round: `ecbench compare --dir <the 3 sessions> --a <a> --b <b> --weight w --resamples 4000 --seed
20261118` (ops ratio of totals), and the same per replicate. Upper end = the larger of the linearised-t and bootstrap 95 %
upper ends. A row **holds** if the upper end is below 1 at w = 0, 0.1 and 1, pooled and in each replicate, and
`1 - max upper > max_w |R_aa(w) - 1|` (`R_aa` = `rho-neg-aa` / `rho-neg`, expected exactly 1). A row is **significantly
worse** if the pooled lower end (smaller of the two) is above 1 at any weight.
**Lever verdict:** a lever is an improvement if at least one row holds and no row is significantly worse; the rows that hold
are the rows where it is claimed. `analysis/evaluate.py` computes all of it.

**T2 correctness check (registered).** For every (session, target, round) the `joint` and `inc` records agree in outcome,
recovered log, trials and relations (`analysis/correctness.py`); any disagreement makes T2 invalid on that row.

**Secondary (reported, not gated):** `prefix-joint / inc` (the new arm against the incumbent), `prefix-joint / joint` and
`prefix-joint / prefix` (each lever given the other: the interaction), and the distance remaining `S(arm) / S(rho-neg)` per
weight for the incumbent and for the cheapest arm by pooled S.

**Multiplicity.** Sixteen row tests (8 rows x 2 levers); Holm across the rows of each lever is reported beside the 95 % verdict
when they differ, from the linearised-t interval rescaled to the adjusted level.

**Predictions** (fixed now, from the model and the pilot). T2: holds on all 8 rows (a near-constant set-up saving),
R_0 within +/- 0.01 of the pilot (B15 0.94, B18 0.95, P20/B21 0.986, B24 0.991, P22 0.992, P24 0.995, B27 0.997). T1: R_0
0.91-0.97 on P20, P22, B15, B18, B21, B24 (holds on most of them), 0.96-1.00 on P24 and B27 (likely no detectable change).
L1 and L2 are additive in GAE (prefix-joint / inc about the product).

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, any ranking against tables A and C. All
numbers `[pcc:unverified]`; no leaderboard or `benchmarks/README.md` row (the artifact store is down, so the validation gate
cannot be met).

## 4. Deviations (declared before data)

- Route: local L0 instead of isolab-eks; no S3 checkpoint or raw-data upload (no AWS in this container); `.resume/RESUME.md`
  belongs to another lane and was not rewritten. Conductor and research-kb unavailable.
- The code lives on a side branch (`ksum-prefix-wip`, scratchpad worktree) while another agent merges `main` into the lane
  branch in the shared checkout; the commits are cherry-picked onto the lane branch afterwards (same tree for the source files).
- Raw sessions stay in the scratchpad as gzip archives pinned by sha256 (disk limit); verify reports are summarised in
  `data/`.

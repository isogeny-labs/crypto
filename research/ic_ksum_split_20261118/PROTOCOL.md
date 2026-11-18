---
title: "Protocol: ksum split=1 and stride-renew targets against the pruned k-sum incumbent on Table B's eight curves"
date: 2026-11-18
status: registered before any evidence run (hashes in REGISTRATION.sha256, registered-at.txt)
---

# Protocol: k-sum `split=1` (pair-sum build) and `targets=stride-renew` against the pruned incumbent `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Nothing here moves the algebraic track.

## 1. Baseline, phase breakdown and levers

**Baseline (incumbent).** The pruned k-sum pipeline of `research/ic_ksum_prune_20261115` (validated, VAL-20261115-276156):
`ic.pipeline` with `oracle = ksum:m=<k>,prune=1`, the factor bases of that study, default target source `walk`, k = 4 and 5.

**Phase breakdown (step 1; prior data only).** `analysis/phases.mjs` reads the prune study's committed sessions (measured rounds,
432 runs per row and arm) and splits the counted cost S (w = 0) of `ksum5-p` / `ksum4-p` (`analysis/phases.json`):

| phase | share of S, ksum5-p (8 rows) | share of S, ksum4-p |
|:--|--:|--:|
| table build (`oracle_setup` adds) | 29-55 % | 22-53 % |
| walk steps (one add a trial) | 13-55 % | 16-49 % |
| walk set-up and restarts (16 jumps, start, pooled offsets: about 48 scalar multiplications a run) | 5-47 % | 5-52 % |
| probe confirmations, linear algebra, factor base, verify | at most 4 % | at most 4 % |

The set-up share is largest on the small rows (B15 47 %, B18 45 %), where 48 scalar multiplications are large against sqrt(r).

**Lever A (new code, commit fc0babc73e): `split=1`.** `KSumTable::build_negation_folded_split` forms every k-sum as its
(k-2)-summand prefix plus one precomputed pair sum `P_j + P_l` (two additions per abscissa pair; the other sign patterns are
negatives). The `2^{k-2} C(a-1, k-1)` partial sums of level k-1 are never formed; the pair table costs `2 C(a-k+2, 2)` adds.
Same visiting order and identity skips: identical table, keys, inserts and collisions (unit tests
`split_build_is_the_same_table_with_fewer_adds`, `split_runs_are_identical_but_for_fewer_setup_adds`). Default off.
Real group additions removed from the build, not an accounting change; the pair table is a vector indexed by point pair (memory
reads, as the base's own point reads, not counted by either arm).

**Lever B (existing option): `targets=stride-renew`.** The pipeline's existing target source `R <- R + S`, `S <- S + T` after each
relation (`TargetSource::StrideRenew`): three draws of `[a]G + [b]Q` (S, T, R0; 6 scalar multiplications) instead of the walk's
16 jumps, start and pooled restarts, and no self-collision restarts (`R0 + kS` repeats only after r steps). One addition a trial,
as the walk. A different target stream: outputs are not identical, so this lever is tested on verified answers and paired cost.

**Questions.** (Q1) Is `split=1` cheaper than the incumbent, row by row, at every weight of the section 5a band, with identical
outputs? (Q2) Is the combined pipeline (`split=1` + `stride-renew`) cheaper than the incumbent, row by row, at every weight?

*Sources checked (research-kb MCP server and lead atlas unavailable in this container):* `research/ic_ksum_prune_20261115/`
(PROTOCOL, README, summary.json, sessions, validator/VALIDATION.json), `research/ksum_m67_pilot_20261115/` (m = 6/7 null, named
by the caller), `src/cryptanalysis/ic_boundary.rs` (`KSumTable`, `collect_relations` target sources), `src/cryptanalysis/
ic_framework/stages.rs` (`Targets`), `src/cryptanalysis/ic_framework/mod.rs` test `the_strides_recover_the_logarithm_without_a_jump_table`.

## 2. Design

**Rows.** The eight curves of the prune study (`rows.mjs`): P20, P22, P24 (`icv1-fp20-t1293-3c2999ba`, `icv1-fp22-tm1267-a5acaf6d`,
`icv1-fp24-t1577-77336155`), B15, B18, B21, B24, B27 (`icv1-f2m15-t131-ed7256f0`, `icv1-f2m18-t393-d2c04152`,
`icv1-f2m21-t2591-de54d72e`, `icv1-f2m24-tm3047-2d0c0e8c`, `icv1-f2m27-t12543-569dca8b`). 3 replicate sessions per row, 24 sessions,
`specs/<row>-r<1..3>.json` written by `make_specs.mjs`: curve, `targets_per_curve` = 48, 3 measured rounds, 1 warm-up, order
`alternate`, isolation L0, timeout and factor-base sizes copied from the prune study's spec of the same name (not re-tuned).
**Fresh seeds:** `target_seed = 20261118000 + idx`, measurement `seed = 20261118500 + idx`, `idx = 10 * (row index in rows.mjs) + rep`
(public targets never used before).

**Arms** (same curve, targets and algorithm seed per round for every arm): `rho-neg` (reference), `rho-neg-aa` (control),
and for k in {4, 5}: `ksum<k>-p` (incumbent: `ksum:m=<k>,prune=1`, walk), `ksum<k>-ps` (`ksum:m=<k>,prune=1,split=1`, walk),
`ksum<k>-pr` (`ksum:m=<k>,prune=1`, `targets=stride-renew`), `ksum<k>-psr` (`ksum:m=<k>,prune=1,split=1`, `targets=stride-renew`).
`max_trials` 13528399621 as in the prune study.

**Binary.** `ecbench` built with `cargo build --release --locked --bin ecbench` in a clean worktree of commit fc0babc73e plus the
working-tree fix of the broken merge 833a35d21e in `src/cryptanalysis/f4_fp.rs` (duplicate `VisitReport` fields; HEAD does not
compile without it; patch `f4_fp_mergefix.diff`, sha256 `c73f69e6c2c8823308475d137c3032762082d6a217e1b04ebe98e20f1f17d5f0`, touches no code path of these arms). Binary
sha256 `5fea35b044291caf8c001706bdec3f48404418b70af78aa59f210c8ecfdaf149` (`BINARY.sha256`); `run_all.sh` refuses any other binary.
**Route.** Local, `--cpus none`, isolation L0, counted cost only, one session at a time under `/tmp/crypto-bench.lock`. Deviation
from the default isolab-eks route: no EKS or S3 access in this container (AWS token rejected, checkpoint token expired); counted cost
does not depend on the host; wall time is not claimed. **Verification.** `ecbench verify --dir <session> --replay-all --exit-code`
on every session, same binary, same host. Budget: at most 3 CPU-hours in total.

**Pilot.** None. No value of any run of this study enters this protocol; predictions come from the prune study's committed data.

## 3. Registered tests (AGENTS.md section 5a band: w = 0, 0.1, 1 GAE per unpriced native operation)

For a row, k and candidate X, `R_w(X) = cost(X) / cost(ksum<k>-p)`, paired by target and round:
`ecbench compare --dir <the 3 sessions> --a ksum<k>-p --b X --weight w --resamples 4000 --seed 20261118` (the printed
`ratio_b_over_a`), and the same per replicate session. Upper end = the larger of the linearised-t and bootstrap 95 % upper ends.

**T1 (primary, Q1): X = `ksum5-ps`; secondary `ksum4-ps`.** Holds on a row if the upper end of `R_w` is below 1 at w = 0, 0.1, 1,
pooled and in each of the 3 replicate sessions, and the paired A/A gate holds (`1 - max_w upper > max_w |R_aa(w) - 1|`, `R_aa` =
`rho-neg-aa` over `rho-neg`, expected exactly 1). **Correctness (every target, every round incl. warm-up, every session;
`analysis/correctness.mjs`):** `ksum<k>-ps` equals `ksum<k>-p` in outcome status and recovered log, `counters`, `factor_base`,
every phase's `native` and `work_native`, every phase except `oracle_setup` in adds, doubles, scalar multiplications and GAE,
the `oracle_setup` doubles, scalar multiplications, `native` and `work_native`; and the `oracle_setup` adds of `-ps` are below those
of `-p` by at most `2^{k-2} C(a-1, k-1) - 2 C(a-k+2, 2)` (equal when no partial sum is the identity). Any other difference is a failure.

**T2 (primary, Q2): X = `ksum5-psr`; secondary `ksum4-psr`.** Same rule (pooled and per replicate, all three weights, A/A gate).
Correctness: every run of every arm verified (`outcome.status = verified`, `matches_target`).

**Attribution (reported, not a gate):** `R_w(ksum<k>-pr)` and `cost(ksum<k>-psr) / cost(ksum<k>-pr)`.

**Best against best (reported, the leaderboard reading):** per row and weight, the incumbent is the cheaper (mean S, this study) of
`ksum4-p` and `ksum5-p`, the candidate the cheaper of `ksum4-psr` and `ksum5-psr`; their paired ratio by `ecbench compare`
(the selection of the cheaper arm by point estimate is stated next to it). **Distance remaining (not a gate):**
`S(arm) / S(rho-neg)` per weight for every k-sum arm, and `ecbench band --ranks` per row.

**Multiplicity.** Two primary questions (T1, T2 at k = 5); a row counts toward an "all rows" verdict only when it holds; the
Holm-adjusted reading (each primary test at 97.5 %) is reported beside the 95 % verdict when the two differ.

**Predictions** (director's, from `analysis/predictions.json`, fixed before data): `R_0(ksum5-ps)` = 0.91-0.94 by row (the closed-form
saved adds over the incumbent's S), `R_0(ksum4-ps)` = 0.94-0.97; `R_0(ksum5-psr)` = 0.52 (B15, B18 ~0.52-0.56), 0.77-0.87 elsewhere;
`R_1` closer to 1 for split (native work unchanged), not necessarily for stride-renew (it also removes the walk's step hashes).
A prediction holds if the observed `R_0` lies within +/- 0.03 (split) or +/- 0.06 (stride-renew combined) of the row's value.

**Verdict booleans (summary.json):** `split5_holds_all_rows`, `split4_holds_all_rows`, `combined5_holds_all_rows`,
`combined4_holds_all_rows`, the row lists, `correctness_ok` (T1), `all_verified` (T2), `all_sessions_verified`.

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, a security statement, any ranking against tables
A and C. All numbers `[pcc:unverified]` until an independent validator's report and `VALIDATION.json` exist; a leaderboard component
section is added only if T1 or T2 holds and the validation gate is met.

## 4. Landing

Study directory with README.md, summary.json, results.md, graph (SVG + PNG), report.pdf; `benchmarks/<date>-ksum-split-stride-tableb/`
and a `benchmarks/README.md` row; `benchmarks/records/` record (`tools/benchrec.py`) and regenerated `benchmarks/progress/`; on a win
a validator report, `VALIDATION.json` and a component section in `scripts/build_ic_leaderboard.py`. Session directories committed as
in the prune study (the validator replays them by path). No push, no PR (the caller pushes).

## Amendment A1 (2026-11-18, before any session ran)

The first launch stopped at `run_all.sh`'s binary check, before any session: the check compared both lines of `BINARY.sha256`
against the binary's hash. The check now reads the first line only (`head -1`). No session, spec, arm, test or prediction changed;
`REGISTRATION.sha256` is re-written for `run_all.sh` and this file (`REGISTRATION.A1.sha256`).

## Amendment A2 (2026-11-18, before any session produced data)

The second launch held `/tmp/crypto-bench.lock` with an outer `flock`, and `ecbench run` takes the same lock itself, so every
session refused to start (exit 2, no records; the partial `data/` and `sessions/` were deleted). `run_all.sh` now runs without the
outer `flock` and passes `--wait` to `ecbench run`, which queues on the lock. Nothing else changed (`REGISTRATION.A2.sha256`).

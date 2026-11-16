# Table B random-binary: `descent-algebraic` m = 3 against ksum5 and the tuned pair table (n21, n24) `[pcc:unverified]`

Pilot only (lever 2 of `research/noneric_lane_survey_20261115/README.md`). Every number is an unverified assertion.
Run with `BENCHREC_SKIP="pilot, not data"` (no `bench.record/v1`, no S3 upload, no replay certificate, no validator, no
VALIDATION.json): not a leaderboard row, not a claim. No code change, no push. research-kb unavailable in this session; no novelty claim.

## Method

- Curves: `icv1-f2m21-t2591-de54d72e` (log2 r 19.998) and `icv1-f2m24-tm3047-2d0c0e8c` (log2 r 21.000), from
  `research/ic_tableb_binary_ksum_20261115/specs/B21-r1.json`, `B24-r1.json` (`make_specs.py` derives the specs here, `specs/`).
- Arms: `rho-neg`, `rho-neg-aa`, incumbent `inc-n62` (n21) / `inc-n67` (n24) (`binary-multiplier:size=N,mult=1` + `mitm:m=2,negation_folded=1`),
  `ksum5` (`binary-multiplier:size=9,mult=1` + `ksum:m=5`, frozen size of that study), and
  `binary-subspace:dimension=D` + `descent-algebraic:m=3`, solver `f4-f2` (also `buchberger-f2` once).
- Main sessions `sessions/main21`, `sessions/main24`: 16 public targets, 2 rounds, no warm-up, L0, `--cpus none`, under the shared CPU
  `flock`; 32 verified runs per arm. Binary sha256 `e8abdeb8d178...1355` (same for every session here). Not replayed (`verify --replay-all` not run).
- Descent probes (1 target each, 1 round, `probe21.json` spec, `pilot-probes/`, `sessions/desc24`): D = 4..6 first (killed at 110-900 s), D = 5, 6 (400 s watchdog),
  D = 7, 8 at n21 (unconstrained: kernel OOM kill at 13.9 GB anon RSS), D = 7 (9 GB `ulimit -v`, then `buchberger-f2`, 240 s watchdog), D = 7, 8 at n24 (9 GB cap, 300 s).
  Base size is 2^D points (so D = 7 is 128 points, about 3D = 21 Weil-descent unknowns against n = 21 equations).

## Table (S = GAE / sqrt r, mean over 32 runs; verified/runs; `ecbench band`, w = GAE per unpriced native operation) `[pcc:unverified]`

| arm | n21 S (w=0 / 0.1 / 1) | n21 ok | n24 S (w=0 / 0.1 / 1) | n24 ok | slope n21 to n24 of log2 S on log2 r (w=0 / 0.1 / 1) |
|:--|--:|--:|--:|--:|:--|
| rho-neg (matched; A/A identical) | 1.377 / 1.498 / 2.589 | 32/32 | 1.069 / 1.164 / 2.020 | 32/32 | -0.36 (all w) |
| incumbent inc-n62 / inc-n67 | 10.449 / 11.863 / 24.587 | 32/32 | 11.243 / 12.918 / 27.987 | 32/32 | +0.11 / +0.12 / +0.19 |
| ksum5 (N = 9) | 7.318 / 8.099 / 15.126 | 32/32 | 6.682 / 7.512 / 14.990 | 32/32 | -0.13 / -0.11 / -0.01 |
| descent-algebraic m = 3, D = 5, 6 (n21) | no relation set within 400 s (1 target) | 0/1 each | not run | | none |
| descent-algebraic m = 3, D = 7, 8 (n21) | out of memory (killed at 13.9 GB; D = 7 also at the 9 GB cap) | 0/1 each | out of memory at the 9 GB cap, D = 7 and 8 | 0/1 each | none |
| descent-algebraic m = 3, D = 7, buchberger-f2 (n21) | no result within 240 s | 0/1 | | | none |

Gap to matched rho (ksum5 S / rho S, w = 0): n21 5.3x, n24 6.3x (same order as the earlier study, 5.6x and 5.8x). Incumbent over rho: 7.6x and 10.5x. Band ranks
(`band21.json`, `band24.json`): rho-neg < ksum5 < incumbent at every weight on both curves.
Unpriced native work: the three non-algebraic arms list none beyond table inserts (priced at w by default; `uncounted` empty). The descent arms produced no
verified record, hence no solver price or w reading: nothing to band. For scale, `research/fes_then_batched_20261112/RESULTS.md` (an m = 2 `descent-algebraic` row on
binary n21, D = 11, 529 relations) prices solver work by measured wall (8 ms per call, 100% of GAE) and reports S about 3700 (3000x rho),
a lower bound; a measured-wall price is not a w-band reading.

Slopes are from two points and 16 targets x 2 rounds each; the rho slope (-0.36) shows the noise (rho's expected slope is 0, but the targets and the curve differ).
Read only the sign: ksum5 flat-to-slightly-falling, the pair-table incumbent rising.

## Interpretation

1. **The algebraic route has no point to price.** With 3D unknowns to meet n equations, a decomposition exists only for 3D >= n, so D >= 7 at n21 and D >= 8 at n24.
   Below that (D = 5, 6) the hit rate is about 2^(3D-n)/3! (about 2e-2 at D = 6 and 3e-3 at D = 5 per trial, before the subspace-membership factor), so a rank of order 2^D needs hundreds of solver calls per
   relation at D = 5, 6: no verified run in 400 s (the per-call solver time was not measured). At and above it (D = 7, 8) the implementation exhausts 9-14 GB before one target finishes, so the memory is the wall at these sizes
   on this host (the cause was not localised; set-up versus growth during the run was not separated). On the
   m = 2 comparison (D = 11, n21) the algebraic solver already cost 3000x rho (1.9x the m = 2 pair table at the same D), so m = 3 cannot be cheaper per relation
   unless it fixes both the hit rate and the memory.
2. **Cost per relation or slope:** not measurable from this pilot: there is no completed descent run. The only slopes are the comparators' (ksum5 about -0.1, incumbent about +0.1).
   The leaderboard's descent rows (F4/F5, m = 2 on subspaces) and the survey's L4a/L7 priors (300-1500x the pair-table oracle, drift away from rho) point the same way. Nothing here suggests
   the algebraic arm could undercut ksum or rho at n21-24; the data cannot exclude it at larger n either, because it never reached a number.
3. **Table B non-generic cell:** closed as "not runnable at these sizes with `descent-algebraic:m=3` on `binary-subspace`", not as "slower".

## Verdict

A frozen study is not warranted on the current oracle. Before one is, the oracle would need (a) a memory bound (the run grows past 14 GB at D = 7, n = 21), and (b) a cost
per relation within about 10x of the pair table at some D with 3D >= n; both are engineering, not a research hypothesis. The cheaper next step is a profile of where the memory grows at D = 7
(one target, n21), not another Table B session. ksum5 stays the generic comparator: 0.70x and 0.59x of the tuned incumbent here (n21, n24; w = 0), consistent with the validated study.

## Caveats

Pilot sizing only: 16 targets, 2 rounds; no replay; the ksum5 and incumbent sizes are the frozen ones of the earlier study (not retuned); other agents shared the host (CPU lock held,
but memory shared: one first probe batch hit an OOM kill that may have been partly caused by concurrent jobs, which is why the 9 GB cap and the n24 repeat were used). `solver_budget_seconds` was not set
(nonzero would make runs nondeterministic); the session watchdog bounded the arms instead. Files: `make_specs.py`, `specs/`, `run_all.sh`, `sessions/` (records, logs), `pilot-probes/`, `band21.json`, `band24.json`.

# A sibling trial walk (`walk_sib=m`) for the k-sum Table B arm `[pcc:unverified]`

**Standing: counted cost at L0, `[pcc:unverified]`, no pcc claim, no validator report or `VALIDATION.json`; no leaderboard or
`benchmarks/README.md` row (validation gate not met; artifact store down).** Track: generic decomposition (bounded by
sqrt(r); cannot beat rho or BSGS).

## Outcome

- **Incumbent best per row** (`research/ic_ksum_prefix_joint_20261118`, VAL-20261118-facd3e): `ksum:m=k,prune=1,split=1`,
  `targets=stride-renew-joint`, with `prefix=1` on P20, B15, B18, B21, B24.
- **Lever (new, default off, commit 39d3d20d72).** `walk_sib=m` forms the incumbent's own stride-renew progression in blocks
  of `2m + 1` around a centre `C = A + [m+1]S`: `C + [i]S` in full and the abscissa of `C - [i]S` from ONE shared inversion,
  charged one group addition plus one `sibling_second_points` (priced at w, main's `sib` convention); the abscissa is probed
  by one counted table lookup (key `x + 1`) and, on a hit, the point is formed by one more counted addition and decomposed as
  usual. Multiples `[2]S..[m+1]S` (m counted operations) each time a drawn or renewed stride starts a block, one addition a
  centre. The targets, relations, rows and answer are the incumbent's, run for run (4,608/4,608 triples identical; below).
- **Registered test T1 holds on all 8 rows at w = 0, 0.1 and 1, pooled and in every replicate** (rule: upper 95 % end < 1;
  432 pairs a row). `cand / inc`:

  | row (m) | w = 0 | w = 0.1 | w = 1 |
  |:--|:--|:--|:--|
  | P20 (6) | 0.789 [0.785, 0.793] | 0.818 [0.814, 0.821] | 0.918 [0.916, 0.919] |
  | P22 (8) | 0.779 [0.775, 0.782] | 0.807 [0.804, 0.811] | 0.912 [0.910, 0.913] |
  | P24 (12) | 0.763 [0.760, 0.766] | 0.794 [0.792, 0.797] | 0.906 [0.905, 0.907] |
  | B15 (2) | 0.916 [0.912, 0.920] | 0.927 [0.924, 0.930] | 0.969 [0.968, 0.971] |
  | B18 (3) | 0.848 [0.843, 0.852] | 0.868 [0.864, 0.872] | 0.942 [0.941, 0.944] |
  | B21 (6) | 0.792 [0.789, 0.796] | 0.820 [0.817, 0.823] | 0.918 [0.917, 0.919] |
  | B24 (10) | 0.705 [0.702, 0.709] | 0.748 [0.745, 0.751] | 0.890 [0.889, 0.891] |
  | B27 (12) | 0.789 [0.786, 0.792] | 0.815 [0.813, 0.818] | 0.914 [0.913, 0.915] |

  The upper ends used (worst of pooled and replicates, lin-t or bootstrap) range from 0.715 (B24, w = 0) to 0.974 (B15, w = 1). No row is
  worse. The lever wins at w = 1 too: a pair saves one addition and adds one priced sibling point, but the missed abscissa
  also skips its guard probe (calibration-priced as a lookup), so the native work is unchanged but for two lookups a lifted hit.
- **Prediction (registered, count model fed only by incumbent records): met.** Every row within +-0.02 (largest miss B27
  0.779 predicted, 0.789 measured at w = 0; others within 0.009). On the pilot the model matched 24 (row, m) cells within
  0.001-0.009 at every weight.
- **Distance remaining to matched rho-neg** (this study's targets, incumbent best -> walk_sib): **w = 0 2.49-5.09 ->
  2.11-4.01** (B18 2.49 -> 2.11, B15 2.52 -> 2.31, P20 3.76 -> 2.97, B21 3.76 -> 2.98, B24 4.23 -> 2.98, P22 4.27 -> 3.33,
  P24 4.57 -> 3.48, B27 5.09 -> 4.01); w = 0.1 2.65-5.33 -> 2.30-4.35; w = 1 3.46-6.46 -> 3.26-5.90. Rho is ahead on every row at
  every weight. (The prefix-joint study measured 2.48-4.99 at w = 0 for the same arms on its own targets.)
- **T3 (secondary, fresh registered test of `prefix=1` on P22, P24, B27): fails on all three** (as predicted, low power):
  pooled 0.963 / 0.977 / 0.975 at w = 0 with pooled intervals below 1 at every weight, but a replicate's upper end reaches
  1.001-1.019 at w = 1 (P24 also at w = 0). `prefix=1` stays off on these rows; no row is worse.
- **Controls and correctness.** A/A `rho-neg-aa / rho-neg` = 1.0000 and `off / inc` = 1.000000 on every row and weight;
  24/24 sessions `verify --replay-all --exit-code` OK (24,768 records, 18,576 replays reproduced, 0 problems). Registered
  correctness check over 4,608 (session, round, target) triples: `off` = `inc` in every deterministic field (0 mismatches);
  `cand` = `inc` in outcome, recovered log, trials, relations, rank, oracle set-up, scalar multiplications and stride
  counters (0 mismatches); relation-phase adds + doubles = `inc - stride_steps + pairs + 2 full_pairs + centres + lifts +
  multiple_ops` exactly (0 mismatches).
- **Not an accounting change.** The saving is additions not performed: a sibling pair forms two targets for one inversion,
  and it is charged one addition plus one priced sibling point, the repository's existing convention for sibling pairs
  (main's `sib`, the Koblitz `walk_sib`, `wwalk sib`). Every other operation, including the wasted plus points of a block
  a relation cuts short and the multiples per renewed stride, is counted. Constant factor only: the trial walk's cost per
  target falls from about 1 addition to about (m+1)/(2m+1) additions plus m/(2m+1) priced sibling points; no exponent
  changes. Under a pricing that charged the second sibling point as a full addition the lever would not exist; the band's
  w = 1 column is the conservative end of the convention.

## Deviations

Local L0 instead of isolab-eks; no S3 checkpoint or raw upload (artifact store unavailable); `.resume/RESUME.md` not
written; Conductor and research-kb unavailable in this container. `analysis/correctness.py` was corrected after its pilot
dry run and before registration (compared wall-time fields at first). `results.json` (82 KB) is committed gzipped. The
pilot's benchmark record was closed at the end with this report (benchrec requires a report PDF to close). Raw sessions outside Git:
`sib-confirm-sessions.tar.gz` (sha256 `56caf4b3…5410`) and `sib-pilot-sessions.tar.gz` (`21adfcce…9cb5`) in the producer's
scratchpad; per-file digests in `data/sessions.sha256`, `pilot/sessions.sha256`.

## Open threads

- Independent validation (validator report, `VALIDATION.json`), then the IC leaderboard and `benchmarks/README.md` rows.
- m was chosen on a grid at w = 0.1; the model says the optimum is flat (+-0.01 for m within a factor 2 on the larger rows).
- A sibling *triple* or a batched (Montgomery) inversion across a block's pairs would cut the per-target addition share
  further; the same block structure applies to the binary rows' `x_sibling_pair`.
- `prefix=1` on P22/P24/B27: a powered test (about 3x the pairs) would settle the 2-3 % effect.

## Files

`PROTOCOL.md` (registered 1189d9f96a), `REGISTRATION.sha256`, `registered-at.txt`, `BINARY.sha256`, `M.json` (m per row and
predictions), `make_specs.py`, `specs/`, `pilot/` (specs, `analyse.py`, `pilot_table.txt`, `pilot_summary.json`,
`model_pilot.json`, `correctness_sib{4,8,16}.json`, run log, verify, digests), `run_pilot.sh`, `run_all.sh`, `analysis/`
(model, evaluate, correctness, verify_summary, summary, plot, report), `results.json.gz`, `results.md`, `summary.json`,
`data/` (run log, verify summaries, correctness, evaluation output, digests), `ratios.svg`, `gap.svg`, `model.svg`,
`REPORT.pdf`.

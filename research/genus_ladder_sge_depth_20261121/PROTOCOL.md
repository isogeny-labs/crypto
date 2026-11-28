# SGE merge depth: w_max 8 -> 32, fill_max 3 -> 10, genus 3-5 (frozen protocol, 2026-11-21)

Lead C1 of `research/hecdlp_campaign_20261121/CAMPAIGN.md`. Follow-up to
`research/genus_ladder_sge_20261115/` (SGE(w_max, fill_max); "the fill bound never bound") and
`research/genus_ladder_sge_scale_20261115/` (genus 3, p = 8209..32771: the SGE(8,3) arm's `la`
ratio to the unmerged arm drifts 0.32 -> 0.39 as p grows). Every figure `[pcc:unverified]`.
Synthetic random curves only; nothing here is a leaderboard or `benchmarks/README.md` row (no
validation-gate evidence). Measurement with an existing binary; no source change.

## 1. Question
Does a deeper merge cut the linear algebra further on identical relations, and does it stop the
drift of the `la` ratio with p?

## 2. Instrument
`genus_ladder` and `genus_ladder_analyze` built from the clean committed tree (hashes in
`binary.sha256`; copies in the session scratchpad `c1/` are the only binaries used). Settings on
identical relations, one invocation per cell group, the driver running the unmerged arm beside
the merged ones:
`--sge 8,3 --sge 16,3 --sge 32,3 --sge 16,10 --sge 32,10` (arm names `C_double_lp+pre+sge(W,F)`).

## 3. Grid (arm c, cc 2.0, max-h 8, threads 4)
| id | cell | invocation (other flags: `--arms c --cc 2.0 --max-h 8 --threads 4` + the five `--sge`) |
|---|---|---|
| G3a | genus 3, p = 8209, 16411 | `--primes 8209,16411 --genera 3 --curves 3 --seeds 2 --max-trials 2e8 --seed-search 600` |
| G4 | genus 4, p = 1031, 2053; genus 5, p = 1031 | `--primes 1031,2053 --genera 4 --curves 3 --seeds 3` and `--primes 1031 --genera 5 --curves 3 --seeds 3` |
| G3b | genus 3, p = 32771 (last, nohup, own invocation; instance search about 45 min) | `--primes 32771 --genera 3 --curves 3 --seeds 2 --max-trials 2e8 --seed-search 600` |
| Rho | genus 4 p = 1031, 2053 and genus 5 p = 1031 | same cells with `--arms R` (no `--sge`) |

Genus 3 rho rows: reuse `scratchpad/scale/scale_g3.jsonl` (rho arm `R_rho_neg`) after verifying
that the `instance` lines of the G3a and G3b outputs equal its instance lines field for field;
the unmerged arm's counted fields are also compared with the scaling study's rows. Genus 4 and 5
use the SGE study's instances (first `--curves 3` of the deterministic search); their instance
lines are compared with `research/genus_ladder_sge_20261115/analysis.json` where it lists them.
Host: shared container; counted units are the measurement, wall times are informational only.

## 4. Analysis (fixed before data)
`paired.py` (adapted from the SGE study's, same statistic and bootstrap: per-pair ratios to the
unmerged arm of the same (p, g, curve_seed, seed); hierarchical bootstrap, 2000 resamples, seed
20261121) gives per (setting, genus, p) the `la_mul` (merge work included) and `FPM_mid`
(`cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul`) ratios, structure means (core rows/columns/
nnz before and after, merge work). `genus_ladder_analyze` on the join gives each arm over rho with
intervals. The **best setting** is the one with the lowest mean `la_mul` ratio at genus 3,
p = 32771.

## 5. Predictions
| id | prediction | falsified if |
|---|---|---|
| D1 | relation-phase fields (`trials, candidates, prefilter_rejects, splits, relations, rels_by_lp, rel_mul, rel_add, group_ops, setup_ops, walk_ops, verify_ops, cantor_mul, cantor_inv, attempts, core_rows, core_cols, fb_cols`) identical across the six arms of every (instance, seed); every run certifies with the planted t | any difference or uncertified run |
| D2 | mean `la_mul` falls monotonically with w_max (8 > 16 > 32) at fixed fill_max in every cell (both fill_max 3; fill_max 10 for 16 > 32 only); the best setting's `la` ratio to the unmerged arm is <= 0.25 at genus 3, p = 32771 | any cell non-monotone, or best ratio > 0.25 |
| D3 | with the best setting the `la` ratio differs by <= 0.03 between p = 8209 and p = 32771 | difference > 0.03 |
| D4 | FPM_mid of the best setting over rho <= 0.22 at genus 3, p = 32771 (SGE(8,3): 0.283); best / unmerged FPM_mid <= 0.45 at genus 4, p = 2053 (SGE(8,3): 0.52) | either bound exceeded |
| D5 | fill_max 10 vs 3: no change in `la_mul` at w_max 8 (the bound never binds there: `sge_refused` = 0 and equal merges); a change at w_max 32, expected direction a lower `la_mul` (more merges allowed; the merge's own work may eat part of the gain) | a change at w_max 8, or none at w_max 32 |

Also reported (no prediction): rows, columns, non-zeros and generic entries before and after,
merges, refused merges, merge multiplications and additions, Wiedemann share, per-cell FPM_mid.

## 6. Limits stated in advance
Few curves and seeds per cell, shared host, no independent replay, FPM prices are conventions;
rho ratios are on the primary FPM reading only, the section 5a GAE band is not re-resolved here.

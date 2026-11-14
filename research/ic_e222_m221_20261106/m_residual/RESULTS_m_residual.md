# Montgomery H1 residual: real or noise? (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host; operation counts only (`trials_total`); no replay certificate, validator report or matched-rho band. Not a leaderboard row.

**Question.** `../h1_shortfall/RESULTS_h1_shortfall.md` found that an exact pair-table model of the H1 on/off trial ratio matches Edwards (model 0.272, observed 0.2774) but leaves a +7% residual on Montgomery (model 0.136, observed 0.1525; CI +2.4% to +11.8%; 24 Montgomery instances). Is the residual real?

**Verdict: noise.** On 600 fresh Montgomery instances the residual is +0.60% (bootstrap CI -0.34% to +1.53%); the +7% is excluded. The 24 original instances do not keep it on further fresh solver seeds (below).

## Method

Model code is imported from `../h1_shortfall/h1_shortfall.py` (`model_all.py`). Per instance: `P_off = |T_off ∩ <G>| / n` (exhaustive test `[n]pt = O` of every distinct point `±F_i ± F_j` of the repository's 32-entry small-x table), `P_on = |T_on| / n` (every projected entry lies in <G>, checked), `pred = P_off / P_on`. Observed per instance: the ratio of summed trials over K = (seeds × 34) relations per arm. Residual `log(obs) - log(pred) - (P_on - P_off)/(2K)`; the last term is the second-order bias of a log ratio of geometric sums (at most 0.2%). Model standard error from the delta method, `var = ((1-P_on) + (1-P_off)) / K`. The model assumes R uniform on <G> and independent trials; it does not assume uniform torsion classes (the exact table replaces that).

## Commands

```
python3 -I research/ic_e222_m221_20261106/gen_instances.py --out <dir>/M --families M --bits 12,14,16,18,20 --per 120 --seed e222m221-mresid-v1
python3 -I research/ic_e222_m221_20261106/gen_instances.py --out <dir>/E --families E --bits 12,14,16,18,20 --per 40  --seed e222m221-mresid-v1
TMPW=<tmpdir> python3 -I run_pairs.py <dir>/M,<dir>/E rows.jsonl          # 6,400 runs, seeds 1-4, arms off / --project-cofactor
python3 -I model_all.py <dir>/M,<dir>/E model.json
python3 -I analyze_residual.py rows.jsonl model.json results_residual.json
python3 -I subset_and_split.py results_subset_split.json
python3 -I seed_cells.py results_seed_cells.json
```
The 24 original instances are `../h1_shortfall/instances/M*.json` (generator seed `e222m221-h1default-v1`); extra seeds with `RP_SEEDS=5,...,12`.

## Result 1: large fresh sample (800 instances, 6,400 runs, all recovered the planted log, every run collected exactly fb + 2 relations)

| family | n | mean residual | bootstrap 95% CI | model-based z | median observed / model |
|---|---|---|---|---|---|
| Montgomery (h = 8) | 600 | +0.60% | -0.34% to +1.53% | +1.24 | 0.1385 / 0.1368 |
| Edwards (h = 4), control | 200 | -0.04% | -1.77% to +1.59% | -0.05 | 0.2701 / 0.2646 |

The empirical spread of the per-instance residual (M 0.115, E 0.121) matches the model's own (0.117 both), so the model's uncertainty is the right size. By rung (Montgomery): +0.4% (12), +0.8% (14), +2.5% (16, z +2.2), -0.1% (18), -0.5% (20); the one nominal 2-sigma rung is one of ten rung tests (ten tests: |z| > 2.8 is the 5% two-sided level with a Bonferroni correction). The mean residual does not grow with the sample: -1.3%, -0.3%, +1.3%, +0.6% at 10%, 25%, 50%, 100% of the instances.

Shared-seed noise: instances of one rung and one solver seed draw near-identical uniform numbers. Cell means (seed × rung, 20 cells): chi2 = 17.6 on 19 dof (p = 0.55) for Montgomery, 9.5 on 19 (p = 0.96) for Edwards, so no sign of noise shared between instances; the model's standard errors apply.

Split-half over solver seeds, per-instance residuals (1,2) against (3,4): correlation -0.02 (permutation p = 0.72); half means +0.97% and +0.17%. No instance-level structure.

## Result 2: the 24 original Montgomery instances on fresh solver seeds (24 instances, 8 per rung at 12, 16, 20)

| solver seeds | mean residual | bootstrap 95% CI (percent) | note |
|---|---|---|---|
| 1, 2 | +7.14% | +2.2% to +12.3% | reproduces the earlier note (median 0.1525); this is the set that raised the question |
| 3, 4 | +7.46% | +0.2% to +15.2% | first fresh seeds |
| 5 to 8 | -4.37% | -7.2% to -1.3% | |
| 9 to 12 | +0.28% | -5.4% to +5.9% | |

Pooling all twelve seeds with weights 68, 68, 136, 136 relations: about +0.9%. Excluding the first set (seeds 1, 2, which selected the instances): about -0.2% ± 1.5%, which excludes +7% (4.8 standard errors). Drawing 20,000 random stratified 24-subsets (8 each at 12, 16, 20) from the new 600 gives subset-mean spread 0.022; the old 24 at +7.1% (seeds 1-4) lie in the top 0.21%, i.e. these instances with these seeds are an unusual draw, and the further seeds regress to the model. The four seed-set means scatter more than their standard errors (chi2 = 11.9 on 3 dof, p = 0.008, including the selected set), so a small unexplained excess of scatter remains in this one 24-instance set; it does not appear in the 800-instance sample.

## Reading

- The Montgomery H1 trial ratio agrees with the exact pair-table model within +0.6% ± 0.9% on a sample 25 times larger; the earlier +7% was a selected high draw (24 instances, noticed at p about 0.02) that two later seed sets partly repeated and 16 further seeds did not.
- Step 4 of the brief (testing equal hit probability across table-point classes) was conditional on the residual persisting and was not run. The model computes the usable set exactly, so equal class probabilities are not an assumption of it.

## Limits

Generator rungs 12 to 20 (subgroup orders about 2^12 to 2^21), local host, one binary, Python model on small tables; the model is a model of the pair-table collector with the default fb 32 only. The 24-instance scatter above is a descriptive finding, with no cause identified. Native E-222 / M-221 sizes are not covered (the model's diagonal term is the only size-dependent correction found earlier).

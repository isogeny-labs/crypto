# Relation excess with lightest-row selection, genus 3 at p = 32771, 65537, 131101 (frozen protocol, 2026-11-21)

Lead: the linear-algebra share of FPM_mid rises 0.48 -> 0.68 from p = 8209 to 262147
(`genus_ladder_fast_order_20261121`, 2LP + prefilter + SGE(8,3)); `la_mul` scales as p^1.52, the relation
phase as p^1.25 and `core_cols` as p^0.69, so the average row weight of the core grows with p. Wiedemann
costs `n * (generic entries + n)` multiplications (`genus_ladder_sge_depth_20261121/model_check.md`). Idea:
collect more relations than the stopping rule needs, keep only the lightest after the peel, and hand the
lighter matrix to the same SGE(8,3) and Wiedemann. Every figure `[pcc:unverified]`.

## Code (additive; defaults byte-identical)
`genus_ladder::ExcessParams { extra, keep }` and `run_ic_ext_excess(inst, prm, ext, excess)`; `run_ic_ext`
is `run_ic_ext_excess(.., None)`. (Deviation from the task text, declared before data: the option is an
argument of the new entry point and not a field of `IcParams`, because `IcParams` is built by struct literal
in `genus_ladder_sieve.rs`, `genus_ladder_harvest.rs` and `bin/smith_transfer_cost.rs`, files this task does
not own; adding a field would break their build. `None` runs the code of `run_ic_ext` unchanged.)
* Collection: the first time the existing rule would stop (cycles >= columns + margin), with `N0` relations,
  collection continues to `ceil((1 + extra) * N0)` relations from the same random stream (the first `N0`
  trials equal the baseline's). The extra trials are counted in the relation phase like all others. The stop
  point is the first stop of the rule, not the first stop whose peeled core is large enough (the baseline
  retries when the core is short); the choice avoids a peel that would be charged and thrown away.
* Selection (once per attempt, after the peel, only if the peeled core has at least `cols + margin` rows):
  sort the core rows by weight (ties by index), walk from the heaviest, drop a row while more than
  `kept_target = max(ceil(keep * rows), cols + margin(cols))` rows are alive, unless it is the only live row
  of a column of the core (the target column included); re-peel the survivors with the ordinary `peel`; if
  the re-peeled core is shorter than `cols' + margin(cols')`, fall back to the unselected core (recorded as
  `ex_fallback`). Then SGE and Wiedemann as now, including the `skip_wiedemann_draws` stream rule.
* Counting: weights and dropped-row bookkeeping one addition per entry touched, re-peel as `peel` counts;
  all in `la_sel` (a field of `la`); comparisons of the sort are free (convention of `peel`, `sge_reduce`).
* Driver: `--excess E,K` (repeatable); arm name suffix `+ex(E,K)`; fields `excess_rows_collected`,
  `rows_kept`, `mean_weight_before`, `mean_weight_after`, `core_nnz_before_sge`, `core_nnz_after_sge`,
  `la_sel_mul`, `la_sel_add`, `ex_fallback`, `core_rows_pre`, `core_cols_pre`. Without the flag the layout is
  the frozen one.

## Run
```sh
genus_ladder run --primes 32771,65537,131101 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 \
  --excess 0.2,0.85 --excess 0.5,0.7 --excess 1.0,0.5 --cc 2.0 --max-h 8 --threads 4 \
  --max-trials 4e8 --seed-search 600 --order-method bsgs1 --out grid.jsonl
```
Arms per (prime, curve, seed): `C_double_lp+pre` (unmerged, reused as the SGE-free reference), `+sge(8,3)`
(the plain arm), and `+sge(8,3)+ex(E,K)` for the three settings: 6 runs per arm per prime. Instances are
re-found by the search (not copied); their lines must equal those of `genus_ladder_sge_scale_20261115/sge_g3.jsonl`
(p = 32771) and `genus_ladder_fast_order_20261121/ext_g3.jsonl` (65537, 131101). Rho rows are the `R_rho_neg`
runs of those two files on the same instances (not re-run).
Statistic: FPM_mid = `cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul` (g = 3), ratio of means over the 6
runs of a setting to the mean rho FPM_mid on the same prime (rho FPM_mid from the same fields); paired ratios
of an excess arm to the plain arm per (curve, seed) with the hierarchical bootstrap of
`genus_ladder_sge_depth_20261121/paired.py` (2000 resamples, seed 20261121).

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| X1 | the plain `+sge(8,3)` arm's rows equal those of the earlier studies field for field (relation and LA counts); every run of every arm certifies with the planted t | any field differs or any run fails to certify |
| X2 | `mean_weight_after` falls monotonically with the amount dropped (0.85 > 0.7 > 0.5 kept: after(0.2,0.85) > after(0.5,0.7) > after(1.0,0.5), mean over the six runs at each p, and below the plain arm's `mean_weight_before`); `la_mul` of (0.5,0.7) <= 0.7 of the plain arm at 131101 | any of these |
| X3 | FPM_mid of the best setting (lowest ratio at 131101) <= 0.85 of the plain arm at 131101, and the gain grows with p (the ratio best/plain is smaller at 131101 than at 32771), because the LA share rises with p | either part |
| X4 | the relation phase (`rel_mul + cantor_mul + 10 cantor_inv`) of (1.0,0.5) <= 2.0x the plain one, and the net FPM_mid of (1.0,0.5) is worse than that of (0.5,0.7) at 32771 and better at 131101 | any part (if not, say so) |

Author's expectation, written before data, so a falsification is read against it: with LA share about 0.6 at
131101 and la_mul proportional to nnz, (0.5,0.7) gives roughly 0.4 * 1.5 + 0.6 * 0.6 = 0.96, so I expect X3's
0.85 to fail and X4's cross-over to fail, with the gain direction (smaller ratio at larger p) right. The
numbers decide.

## Limits
Shared 4-vCPU host; 3 curves x 2 seeds per prime (wide intervals); no independent replay; the stop point is
the first stop of the rule (above); rho rows reused, not re-measured; genus 3 only.

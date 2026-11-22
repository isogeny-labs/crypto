# Lead C2: factor-base constant `--cc` for 2LP + prefilter + SGE(8,3), genus 3 and 4 (frozen protocol, 2026-11-21)

Every figure this study produces is `[pcc:unverified]` (one session, shared 4-vCPU host, no independent replay).
No row goes to the IC leaderboard or `benchmarks/README.md` (validation gate). Measurement only: an existing
binary (copy hashed in `binary.sha256`), existing flags, no code change. Question: is `cc = 2.0` (base
`nb = ceil(cc * p^(1-1/g))`, `genus_ladder_analyze` and `PROTOCOL.md` of the first ladder study, sections 2-5)
near the optimum for the best arm at genus 3 and 4? Context: the quartic base-constant sweep
(`research/quartic_dl1_cd_sqrt_20261113/`) found that smaller bases trade relation work for heavier algebra.

## Run
One invocation per `cc in {1.0, 1.5, 2.0, 3.0, 4.0}` and family; the arm name does not carry `cc`, so each
invocation has its own output file, tagged by file name (`g3_cc<X>.jsonl`, `g4_cc<X>.jsonl`).

```sh
genus_ladder run --primes 8209,16411 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 --cc X \
    --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --out g3_ccX.jsonl
genus_ladder run --primes 2053,4099 --genera 4 --curves 3 --seeds 2 --arms c --sge 8,3 --cc X \
    --max-h 2 --threads 4 --max-trials 5e7 --seed-search 4000 --out g4_ccX.jsonl
```
Genus-4 `--max-trials 5e7` and `--seed-search 4000` are the bound-r study's settings (its config line);
the task gave only `--max-h 2 --curves 3 --seeds 2`. The driver runs the unmerged arm
`C_double_lp+pre` beside `C_double_lp+pre+sge(8,3)`. The SGE arm is the primary arm. Rho is not rerun:
matched rho rows (`R_rho_neg`) come from the earlier studies' files on the same instances and seeds
(genus 3: `scale_g3.jsonl`; genus 4: `bound_r.jsonl`), appended to each cc file for the analysis program.
Wall time is never used. Run order: genus 3 (all cc), then genus 4 at 2053 and 4099 (4099's instance
search took 21 min in the earlier study; the genus-4 invocations are the last, under nohup).
Concurrency of invocations on the host is allowed (counted units only).

## Cost readings (as in the first ladder study)
`FPM = cantor_mul + I cantor_inv + rel_mul + kappa la_mul`; `la_mul` includes the merge work. Low: `I=1,
kappa=1`; mid (primary): `I=10, kappa=g^2`; high: `I=30, kappa=g^2`. Ratios to rho and intervals by
`genus_ladder_analyze` (hierarchical bootstrap over curves then runs). Gain of cc over 2.0: paired per-run
ratio `FPM_mid(cc)/FPM_mid(2.0)` on the same (p, curve, seed), hierarchical bootstrap, 2000 resamples,
seed 20261121, computed by `analyze_cc.py`. Best cc of a cell = argmin of the mean FPM_mid over the five
values (ties impossible in practice); a cell where 2.0 is best or the best gain is under 10% reports that.

## Failure criteria (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| F1 | every run certifies with the planted t; the instance lines equal the earlier studies' (genus 3: `scale_g3.jsonl`, 6 lines; genus 4: `bound_r.jsonl` at p in {2053, 4099}, 6 lines), and at cc = 2.0 the unmerged arm's rows equal the earlier study's field for field except wall (genus 3: `sge_g3.jsonl` is the earlier SGE study; here `scale_g3.jsonl` C rows; genus 4: `bound_r.jsonl` C rows) | any non-certified run or mismatch |
| F2 | relation trials fall with cc and `la_mul` (SGE arm) rises with cc, monotonically (non-strict at ties in `ceil`) over the five values in every cell (4 cells) | any cell non-monotone in either |
| F3 | the FPM_mid optimum (mean over the cell) is at cc in [1.5, 3.0] at both genera, and the best cc lowers FPM_mid by >= 10% vs cc = 2.0 at the largest prime of each genus (16411 and 4099) | optimum outside [1.5, 3.0] at a genus, or gain < 10% at either largest prime (then "2.0 is optimal within noise" is the result) |
| F4 | ratio to rho (FPM_mid, SGE arm) at the best cc: genus 3, p = 16411 <= 0.33 (SGE(8,3), cc 2.0 earlier: 0.383); genus 4, p = 4099 <= 0.030 (bound-r without SGE: 0.0375). The SGE effect at cc = 2.0 at genus 4, p = 4099 is reported separately (paired `la_mul` and FPM_mid ratio of merged over unmerged) | ratio above the bound |
| F5 | the cc ranking (order of the five means) is the same under FPM_low and FPM_high as under FPM_mid, per cell; the best cc is the same | any flip, reported with the cell |

A failed prediction is reported as failed; none is revised after the freeze. Limits stated in advance:
three curves and two seeds per cell, two primes per genus, shared host, no replay, selection of near-prime
instances (first study, section 2), FPM prices are conventions.

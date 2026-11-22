# Genus 3 at p = 65537, 131101, 262147 with nine curves per prime: results (2026-11-21/22)

Status: `[pcc:unverified]` (shared 4-vCPU host, no independent replay). Protocol [`PROTOCOL.md`](PROTOCOL.md) (frozen before
data: `frozen.sha256`); binary `binary.sha256`; data `run_65537.jsonl`, `run_131101.jsonl`, `run_262147.jsonl`
(`joined.jsonl`), `analysis.json` (genus_ladder_analyze), `paired.json` (`analyze.py`); `tables.md`; `fig_ratio.svg`;
`data.sha256`. Measurement with an existing binary: HEAD 98005699ee1a, clean `src/`, built by the main session;
`genus_ladder` sha256 6f40afb3...19f9, `genus_ladder_analyze` 943d7f65...b23 (copies in the scratchpad `r2/`, hashes re-checked equal).

## Question and design
With 9 curves per prime instead of 3, where do the SGE arm's ratio to rho and the exponents of both arms land? One
invocation per prime: `--curves 9 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --seed-search 600
--order-method bsgs1 --rho-cap 3e9`. 54 runs per prime (9 curves x 2 seeds x {2LP+pre, 2LP+pre+SGE(8,3), rho}); no arm `a`.
Seed search: 283 / 101 / 261 curves tried for 9 admitted at 65537 / 131101 / 262147 (cell search wall 360 / 303 / 1533 s).
Admitted curve seeds: 65537: 64, 86, 157, 158, 217, 250, 254, 255, 283; 131101: 2, 14, 21, 41, 58, 61, 79, 84, 101;
262147: 13, 19, 23, 69, 113, 206, 207, 241, 261. The first three at each prime are the earlier study's.

## Ratio of the SGE arm to rho (FPM_mid, paired per instance, hierarchical bootstrap)
| p | curves | runs/arm | SGE / measured rho | SGE / expected rho | 3-curve study (measured) | unmerged / measured rho | LA share (SGE arm) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 8209 | 3 | 6 | 0.290 [0.214, 0.423] | 0.322 [0.227, 0.595] | 0.290 | 0.591 [0.437, 0.870] | 0.480 |
| 16411 | 3 | 6 | 0.383 [0.266, 0.726] | 0.528 [0.437, 0.597] | 0.383 | 0.741 [0.499, 1.422] | 0.527 |
| 32771 | 3 | 6 | 0.283 [0.214, 0.361] | 0.375 [0.274, 0.554] | 0.283 | 0.537 [0.407, 0.686] | 0.566 |
| 65537 | 9 | 18 | 0.300 [0.223, 0.420] | 0.330 [0.267, 0.414] | 0.416 [0.336, 0.678] | 0.535 [0.399, 0.760] | 0.604 |
| 131101 | 9 | 18 | 0.399 [0.288, 0.534] | 0.393 [0.336, 0.456] | 0.507 [0.354, 0.823] | 0.668 [0.474, 0.887] | 0.639 |
| 262147 | 9 | 18 | 0.377 [0.263, 0.565] | 0.324 [0.284, 0.374] | 0.293 [0.167, 0.623] | 0.593 [0.415, 0.911] | 0.674 |

Rows 8209 to 32771 are the earlier studies' 3-curve rows, recomputed with this protocol's bootstrap (expected-rho ratios from their
rho instances). Full per-prime tables (cantor_mul, rel_mul, la_mul, FPM_mid, both arms and rho) are in `tables.md`. The analyzer's own
cell ratios on this run's rows agree (0.300 [0.222, 0.424], 0.399 [0.290, 0.531], 0.377 [0.266, 0.565]). Under the GAE w = 1 band the SGE arm is
still 10-13 times rho (analyzer, `tables.md`); the verdict there stays unresolved, as before.

Measured rho against its expectation 0.886 sqrt(r) steps: mean measured/expected FPM 1.10, 0.99, 0.86 at 65537, 131101, 262147
(18 runs each; rho steps have a relative spread of about 50% per run, so the mean of 18 carries about 12%). The 262147 measured rho is 14% under
expectation, which is why its ratio to measured rho (0.377) is above its ratio to expected rho (0.324).

## Exponents of FPM_mid in p (six primes 8209 ... 262147; 95% bootstrap over curves)
| arm | six primes | five primes 8209 ... 131101 | earlier study (five primes, 3 curves) |
|---|--:|--:|--:|
| rho, measured | 1.360 [1.254, 1.464] | 1.361 [1.240, 1.493] | 1.243 [1.069, 1.421] |
| 2LP+pre+SGE(8,3) | 1.422 [1.418, 1.425] | 1.418 [1.414, 1.423] | 1.418 [1.411, 1.425] |
| 2LP+pre (unmerged) | 1.347 [1.345, 1.350] | 1.349 [1.345, 1.352] | 1.349 [1.345, 1.353] |
| rho, expected (0.886 sqrt r x measured cost per step; descriptive) | 1.462 | - | - |

The SGE and unmerged intervals are narrow because the index-calculus cost varies little across curves and seeds; they do not include
the choice of primes or model error. The rho interval is the wide one: 9 curves (18 runs) at each upper prime
shift the point estimate from 1.243 to 1.360 and the upper bound from 1.421 to 1.464.

## Scorecard (predictions of `PROTOCOL.md`)
| id | prediction | outcome |
|---|---|---|
| P1 | all runs certify; first three curves reproduce the earlier lines field for field | **held on every old field.** 162/162 runs certified (0 not certified); the 9 instance lines (3 per prime) equal the earlier ones field for field (curve seed, f, order, h, r); all 54 earlier run rows (18 per prime, except `wall_ns`) are equal on every field the earlier rows had. Deviation: the rebuilt binary (HEAD 98005699) adds fields to the rows: `target_seed`, `la_fallbacks`, `ferrari_undecided` on every run row, and `coef_adds_uncharged`, `compares_uncharged`, `hash_uncharged` on the rho rows; `la_fallbacks` and `ferrari_undecided` are 0 throughout, and the config line adds `target_from`, `targets`. A "field for field" reading that counts added fields as differences would call P1 falsified. |
| P2 | SGE / measured rho in [0.25, 0.45] at all three primes | **held**: 0.300, 0.399, 0.377 |
| P3 | rho exponent: interval contains 1.5, point in [1.35, 1.55]; SGE exponent in [1.38, 1.46] | **falsified** (one condition): rho 1.360 [1.254, 1.464] has the point in range but its interval does not contain 1.5 (upper bound 1.464); SGE 1.4216 in [1.38, 1.46] holds |
| P4 | measured vs expected-rho ratios differ by less than the measured interval width, and fall monotonically from 32771 to 262147 | **falsified** (second clause). First clause held: differences 0.030 / 0.006 / 0.053 against widths 0.197 / 0.246 / 0.302 at 65537 / 131101 / 262147. The expected-rho ratio is 0.375 (32771), 0.330, 0.393, 0.324: not monotone (a rise at 131101), and not flat within 10% (max/min 1.21). Descriptively it is a noisy line near 0.32-0.39 with no clear fall |

## Reading
* With 9 curves the measured SGE / rho ratio is 0.30, 0.40, 0.38 at 65537, 131101, 262147; the earlier 3-curve rows (0.416, 0.507, 0.293) sit inside
  the new intervals, and the 131101 bump shrinks from 0.507 to 0.399. The three upper primes are within a factor 1.33 of each other and the
  intervals overlap: no fall and no rise is resolved from 65537 to 262147. The ratio at 32771 and below (0.28-0.38, 3 curves) overlaps these too.
* The SGE arm's exponent does not move (1.42), and neither does the unmerged arm's (1.35). The rho exponent moves to 1.36 and its interval
  now ends at 1.46: 1.42 for the SGE arm lies inside it, so the data do not separate the two exponents (no resolved drift of the ratio with p), and
  the earlier "SGE exponent above the rho exponent" is a point-estimate difference (0.06) within noise. The expected-rho exponent is 1.46
  (cost per step is flat in p, 0.886 sqrt(r) steps grows with p^1.5): against that reference the SGE arm's exponent is lower by 0.04,
  a slow fall that the 6 points do not resolve above curve-to-curve noise.
* The linear-algebra share of the SGE arm's FPM rises with p: 0.48, 0.53, 0.57, 0.60, 0.64, 0.67 (la_mul grows with an exponent above
  the Cantor work's); this is the component that keeps the arm's exponent at 1.42.
* The ratio is a distance to rho, not a pass/fail gate: SGE arm 0.38 [0.26, 0.57] of measured rho (0.32 [0.28, 0.37] of expected rho) at
  262147 in the FPM reading, about 12x under GAE w = 1.

## Limits
Shared 4-vCPU host (walls not isolated, not used for any claim here); no independent replay; 9 curves per prime out of an admitted population filtered
by h <= 8 and near-prime order (the seed search is deterministic, so the nine are the first nine admitted, not a random sample of curves);
the lower three primes keep their 3 curves, so exponent points mix 3- and 9-curve cell means; the expected-rho reference uses the heuristic
0.886 sqrt(r) steps (an average over targets) and the pooled per-step cost of this prime's rho rows; bootstrap intervals are percentile,
4000 draws; rebuilt binary differs from the earlier one by added output fields (above), with every earlier field equal. The order-search
walls per cell are in the cell lines of the run files.

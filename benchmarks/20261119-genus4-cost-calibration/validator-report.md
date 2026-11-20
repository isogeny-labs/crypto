# Validator report: genus-4 cost calibration (`research/genus4_cost_calibration_20261119`)

Validator agent (Opus 5.5), 2026-11-19, 09:31–09:45 UTC. Independent of the producer's summary. Every
study number stays `[pcc:unverified]` in pcc terms; this report is the validator evidence for it.

**Disposition: validated-with-errata.** In the gate's vocabulary that is `validated`, with the errata
below required before the study is cited. Every point estimate in `analysis.json` and `RESULTS.md`
reproduces exactly from the raw rows with independent code. Gates G1–G3 pass. Counts are
deterministic: 24 replayed rows match in all 54 non-wall fields. The frozen decision
(**conservative**) holds, and it is robust to the model.

The errata are:
- the post hoc cross-check's "within 10%" is false at p = 32771, and the check is circular (F2);
- the stated precision of w_rel and w_la is conditional on one linear model (F3, F4);
- several post hoc statements are not labelled post hoc (F5);
- the verdict is extended beyond the measured primes and curves (F6).

## Artifact set and bindings

| item | value | status |
|---|---|---|
| Validated commit (results) | `b76bc83fbeb8a27cb6271d5b56c350584eac7075` (09:28:34Z); HEAD at start `8959b13533` | bound |
| Protocol commit | `6d70fc12b34f93702d710c94e3f17e8ce90541ae` (2026-11-19 06:32:03Z) | predates data: IC1 started 06:35:02Z (bench record), first data file 06:35 |
| `frozen.sha256` | PROTOCOL.md `6f6387b6…`, run.sh `2c3eaafb…`, analyze.py `29c31142…` | **pass**: working tree, protocol commit and HEAD all equal |
| `binary.sha256` | `493ee294…6b921` | **pass**: hashed at dispatch 09:31:39Z and copied before any replay. This is the prime-order study's binary. Its validator matched `.text/.rodata/.data` against a rebuild of `ca72267899` (`../genus4_prime_order_20261118/validation/out/binary_diff.txt`) |
| Raw data | 16 IC files and 2 rho files, `.jsonl.gz` committed | **pass**: each gitignored `.jsonl` is byte-equal to its committed `.gz`. All 45 tracked study files were byte-equal at validation start (`validation/artifacts_at_validation.sha256`) |
| Later commits | `ad84808c57`…`83a3a0a986` (main-session snapshots) | touch only `validation/` |
| Tools | Python 3.13.18, numpy 2.5.3; host as producer (4-vCPU Xeon @ 2.10 GHz VM) | — |

## Requirements: pass/fail with evidence

| # | requirement | result | evidence |
|---|---|---|---|
| R-1 | Frozen and binary hashes; protocol before data | pass | table above |
| R-2 | G1: every IC run certified and correct, every rho run certified | pass: 128/128 IC, 4/4 rho. Binary self-check only, see F8 | `recompute.json` `gates.G1` |
| R-3 | G2: cc 1.4 and 2.0 at SGE (4, 1.5) equal the prime-order rows | pass: 32 rows (16 keys × 2 passes), 0 mismatches on the 7 protocol fields and on all 54 shared non-wall fields | `gates.G2` |
| R-4 | G3: pass 2 counts equal pass 1 | pass: 64/64, checked both directions on 55 non-wall fields | `gates.G3` |
| R-5 | Count replay (taskset -c 3, exact run.sh argv) | pass: 3 whole configuration files (p16411 cc 2.0, p16411 cc 2.8, p32771 cc 1.4), 24 run rows, 48 comparisons against pass 1 and pass 2, 54 fields each, **0 differences**. Config and instance rows identical; `ok` and `certified` reproduced | `replay.json`, `replay/*.jsonl`, `replay.sh`, `compare_replay.py` |
| R-6 | Rho rows internally consistent | pass (details below) | `recompute.json` `rho_checks` |
| R-7 | IC rows internally consistent | pass: 128/128 rows pass every check (details below) | `ic_row_consistency` |
| R-8 | Instances | pass: f and r equal the prime-order rows; order = r, h = 1. The prime-order validator's independent certificates (`certs.json`) are ok for 373, 455, 355 and 882 | `instances` |
| R-9 | Recompute every headline number (> 1% discrepancy check) | pass: all 0.0% (exact) | `vs_analysis_json`, `vs_results_md_printed` |
| R-10 | Decision rule and P1–P5 | pass: conservative; P1–P5 hold | `decision`, `predictions` |
| R-11 | Post hoc cross-check, per-cc table, sensitivity | numbers reproduce; claim wording fails in places (F2, F7) | `R1.*.crosscheck_*`, `per_cc_mean_wall_s`, `prose` |
| R-12 | Wording, frozen vs post hoc, caveats | errata needed (F2–F7, F10, F11) | below |

The IC consistency checks in R-7 are:
- relations equal Σ rels_by_lp;
- group_ops = walk_ops + setup_ops + verify_ops;
- la_mul = la_sge_mul + la_wied_mul;
- target = k0;
- r equals the instance's r;
- not exhausted, with no LA fallbacks;
- the relation phase is identical across the two SGE settings in all 64 (pass, p, cc, curve, seed) groups.

la_add exceeds la_sge_add + la_wied_add by 159,789–351,282 per row (0.1–0.2%), an un-itemised remainder. It is informational and inside la_ops.

The rho checks in R-6 found the following:
- arm `R_rho_plain_nowin`, so there are no window compares;
- steps = native = walk_ops = hash_uncharged, and coef_adds_uncharged = 2·steps;
- group_ops − steps ∈ {28815, 13344, 17936, 80211};
- cantor_mul per group op is 392.98 in every run;
- Σ run wall + search wall equals the `done` row within 3.8 ms and 1.5 ms;
- steps / sqrt(πr/2) = 1.38, 0.63 and 0.86 at 16411, and 1.96 at 32771. The last has a Rayleigh tail probability of 0.049: unusual but plausible, and it does not affect a per-operation time;
- `rho_cap` 2e9 is the skip threshold on expected steps (`genus_ladder.rs:518`), not a step cap, so the 2.63e9-step run is not a violation.

## Recomputed numbers (validator vs reported)

My code is `validation/recompute.py`, written without analyze.py's code. It differs from analyze.py in four ways:
- rows are keyed by the records' own fields;
- the OLS uses SVD lstsq on scaled columns;
- the cluster bootstrap uses numpy PCG64, seed 4242, 4000 resamples;
- a separately labelled replica of the frozen `random.Random(20261125)` stream checks that the reported intervals are what the frozen procedure yields.

| quantity | reported | validator | diff |
|---|---|---|---|
| t_G(16411), ns per group op (runs) | 1996.3 (1998.9, 1972.0, 2010.1) | 1996.345 (1998.859, 1972.009, 2010.100) | 0 |
| t_G(32771) | 2027.5 | 2027.522 | 0 |
| t_rel / t_la, 16411 (ns) | 5.14 / 8.41 | 5.1418 / 8.4054 | 0 |
| t_rel / t_la, 32771 (ns) | 4.99 / 9.11 | 4.9943 / 9.1130 | 0 |
| intercept 16411 / 32771 (s) | −0.154 / −0.681 (analysis.json) | −0.1542 / −0.6814 | 0 |
| w_rel 16411 [95%] | 0.0026 [0.0019, 0.0032] | 0.0025756; replica [0.0019184, 0.0032212]; independent [0.0018646, 0.0032522] | 0 / ≤2.8% (CI) |
| w_la 16411 | 0.0042 [0.0038, 0.0046] | 0.0042104; replica [0.0037986, 0.0046367]; indep. [0.0037608, 0.0046630] | 0 |
| w_rel 32771 | 0.0025 [0.0018, 0.0031] | 0.0024632; replica [0.0018290, 0.0030548]; indep. [0.0018537, 0.0030610] | 0 |
| w_la 32771 | 0.0045 [0.0041, 0.0048] | 0.0044946; replica [0.0041497, 0.0048309]; indep. [0.0041629, 0.0048236] | 0 |
| ideal negation rho time 16411 / 32771 | 477 s / 1926 s | 477.10 / 1925.61 s | 0 |
| R1 x, 16411 | 0.0053 [0.0052, 0.0054] | 0.0052749 [0.0051924, 0.0053575] (both bootstraps) | 0 |
| R1 x\*, 16411 | 0.0056 [0.0056, 0.0057] | 0.0056110 [0.0055689, 0.0056531] | 0 |
| R1 x, 32771 | 0.0039 [0.0038, 0.0039] | 0.0038645 [0.0038222, 0.0039009] | 0 |
| R1 x\*, 32771 | 0.0039 [0.0038, 0.0039] | 0.0038602 [0.0038206, 0.0038997] | 0 |
| A/A median / max \|ln\|, 64 pairs | 0.023 / 0.105 | 0.023186 (true median); 0.023226 (analyze.py upper median) / 0.104746 | 0 |
| A/A mean signed ln(pass2/pass1) | — | +0.0007 (16411 −0.0030, 32771 +0.0045): no drift | — |
| t_G(32771) / t_G(16411) (P5) | 1.016 | 1.01562 | 0 |
| cross-check x / x\* 16411 (post hoc) | 0.0056 / 0.0060 | 0.0055885 / 0.0060177 | 0 |
| cross-check x / x\* 32771 (post hoc) | 0.0043 / 0.0043 | 0.0042938 / 0.0042751 | 0 |
| per-cc mean wall 16411, cc 1.0/1.4/2.0/2.8 (post hoc) | 3.69 / 2.68 / 2.52 / 2.73 s | 3.690 / 2.677 / 2.517 / 2.733 | 0 |
| per-cc mean wall 32771 | 9.79 / 7.43 / 7.44 / 8.87 s | 9.791 / 7.433 / 7.442 / 8.866 | 0 |
| x, 16411: group / rel / LA time | 0.85 / 0.28 / 1.5 s | 0.851 / 0.285 / 1.530 s | 0 |
| overcharge at w = 1 | ≈220–400× | 222.5–406.0× | 0 |
| one mod-r op vs group op; LA ops per group op | ≈240×; ≈400× | 237.5×; 426.8× | rounding |
| x\* vs x wall, 16411 / 32771 | +6% / tied | +6.37% (paired: all 8 pairs in [+2.2%, +9.4%]) / −0.11% (paired [−2.1%, +4.0%]) | rounding |
| w = 1 ratios on these rows, x/x\* | "64–100%" (prime-order study) | 0.995 / 0.878 (16411), 0.771 / 0.639 (32771). Prime-order analysis.json: 0.999 / 0.876 / 0.769 / 0.640 | consistent |
| sensitivity (t_G ÷ 10) | w_la ≈ 0.04; ≈1.9 s vs 47.7 s, ≈0.04 | w_la 0.0421; 1.900 s / 47.71 s = 0.0398 (0.0367 from measured wall) | 0 |
| decision | conservative | conservative. Max upper bound 0.00483 (independent) / 0.00483 (replica) | — |
| P1–P5 | all hold | all hold (P1 2.00 µs; P2 0.00246–0.00449; P3 0.00561, 0.00386; P4 0.0232; P5 1.0156) | — |

No point estimate differs by more than 1%. The independent bootstrap intervals differ from the frozen
stream's by at most 2.8%, which is expected from a different generator. The replica reproduces them exactly.

## Findings

**F1 (low, process).** At dispatch the data and results were uncommitted. They were committed in
`b76bc83fbe` at the start of validation, and I bound to it (manifest above). The bench record
`benchmarks/records/20261119T0631Z-genus4-cost-calibration.json` is still `status: open`, with:
- `raw: []`: no raw SHA-256, no S3;
- `metrics: []`;
- `isolation: unstated`;
- `commit: f3549ffdb0` with `commit_dirty: true`, which predates the protocol commit.

No `benchmarks/20261119-genus4-cost-calibration/` directory exists yet. Under the validation gate,
nothing from this study reaches `benchmarks/README.md` or the leaderboard until the record is closed
and a `VALIDATION.json` names this report and `replay.json`.

**F2 (medium, unsupported claim).** The post hoc cross-check says the GAE-priced counts "agree with R1
to within 10%, so the counted operations account for the wall time". There are two problems:
1. **"Within 10%" is false at p = 32771.** The cross-check is +11.1% for x and +10.7% for x\*. The
   printed values themselves give 0.0043 / 0.0039 = 1.10. At 16411 the figures are +5.9% and +7.3%.
2. **It is not an independent check.** It prices the same rows with w_rel and w_la fitted on those
   rows. So cross/R1 − 1 equals (−intercept − the arm's mean residual) / mean wall, identically.
   For x at 16411 the fitted wall without the intercept is 2.666 s against a measured 2.517 s. The
   gap measures the negative intercept. It does not show that the counts account for the time.

The supportable statement is that the model y = wall − t_G·group_ops ~ 1 + rel_ops + la_ops has
R² = 0.942 (16411) and 0.986 (32771). The intercept takes −5.3% and −8.1% of the mean wall.

**F3 (medium, precision overstated).** Relation and LA counts are strongly anti-correlated within
each prime: corr(rel_ops, la_ops) is −0.937 at 16411 and −0.929 at 32771. The fitted prices move
outside their bootstrap intervals under small changes to the design. The table below gives the
refits.

| refit | w_rel 16411 | w_la 16411 | w_rel 32771 | w_la 32771 | intercept (s) 16411 / 32771 |
|---|---|---|---|---|---|
| frozen (all rows) | 0.00258 [0.0019, 0.0032] | 0.00421 [0.0038, 0.0046] | 0.00246 [0.0018, 0.0031] | 0.00449 [0.0041, 0.0048] | −0.15 / −0.68 |
| drop one cc level (range over 4) | −0.00004 … 0.00333 | 0.00318 … 0.00501 | 0.00093 … 0.00277 | 0.00404 … 0.00473 | −0.50…+0.53 / −1.04…+0.35 |
| pass 1 only / pass 2 only | 0.00326 / 0.00189 | 0.00441 / 0.00401 | 0.00255 / 0.00238 | 0.00462 / 0.00437 | −0.32, +0.01 / −0.87, −0.49 |
| no intercept | 0.00218 | 0.00393 | 0.00179 | 0.00413 | 0 |

The table shows three things:
- **w_rel is weakly identified.** Leaving out cc 1.0 at 16411 drives it to zero.
- **w_la moves by about ±20%.** It is the better-identified price.
- **t_G cannot be estimated from IC data.** With t_G left free the IC data give 40.9 µs at 16411
  and 5.4 µs at 32771, with negative w_rel. So w_rel rests on transferring rho's per-step time to
  IC group operations. That transfer is reasonable, since both walks do a hash and two mod-r
  coefficient additions per step, but it is an untested identifying assumption.

"The interval for w_la is tight" and the two-significant-figure w_rel overstate the precision: the
intervals hold only under one linear model.

**The decision is not affected.** Model-free upper bounds charge all non-group IC time to one
class. They give w_la ≤ 0.0097 / 0.0078 and w_rel ≤ 0.032 / 0.042 at 16411 / 32771, all ≪ 1. So
"conservative" holds with a margin of more than 20× whatever the model.

**F4 (low, missing or incorrect caveat).**
- The caveat reports the negative intercept only at p = 16411 (−0.15 s). At p = 32771 it is
  −0.68 s, 8.1% of the mean wall, and unmentioned.
- The explanation says relation and LA counts "both fall with p and rise with cc in opposite
  directions". That is garbled. The fits are per prime, so p does not vary within a fit, and both
  counts rise with p. Within a prime, rel_ops falls and la_ops rises as cc grows, which is the
  collinearity.
- The mean residual by cc alternates in sign: +0.011, −0.032, +0.029, −0.008 s at 16411. The fixed
  cost per operation is an average over cc 1.0–2.8, not a constant. LA cost may depend on matrix
  size.

**F5 (low, frozen vs post hoc wording).** RESULTS labels only the cross-check and the per-cc table as
post hoc. The following are also derived outside `analyze.py` and are not labelled:
- "Where the time goes" (0.85 / 0.28 / 1.5 s);
- "What this changes", bullet 1: the 0.4–0.6% figure (from the post hoc cross-check) and the 25×
  figure;
- "What this changes", bullet 3: "No verdict changes";
- the sensitivity paragraph;
- the explanation of the intercept.

"Tables come from analyze.py" is accurate for the gates, R1, R2 and A/A. The headline table is a
hand transcription of `analysis.json`; I compared it cell by cell and every cell matches.

**F6 (low, scope).** The frozen decision is scoped to p = 16411 and 32771 (curves 373/455/355/882,
cc 1.0–2.8). RESULTS extends it twice:
- "The w = 1 headlines of the validated genus-4 studies are therefore conservative";
- "Every genus-4 IC row that was below the ideal negation bound at w = 1 stays below it".

That reaches other studies, for example `genus4_followup_20261118` at p = 8209 and cc 0.5–0.7. The
monotonicity argument plus the model-free bounds make this very likely, but it is an extrapolation
and should say so. "The w = 0.1 column overstates native cost by about 25×" holds for w_la (22–24×).
For w_rel the factor is 39–41×.

**F7 (low, sensitivity paragraph).** The arithmetic reproduces: 1.90 s / 47.7 s = 0.040, w_la 0.042.
Three small problems:
- It uses 0.86 s for group time where "Where the time goes" says 0.85 s (recomputed 0.851 s).
- It ignores the −0.15 s intercept. From the measured wall it is 1.75 s, ratio 0.037, the same
  conclusion.
- It says optimized arithmetic "(explicit formulas, Montgomery F_p) … would raise w_rel and w_la".
  Montgomery F_p would also speed up the relation-phase F_p operations, so w_rel need not rise.

**F8 (low, certificate discipline).** No record carries the recovered logarithm or the target point.
G1 "certified and correct" is the binary's own check: `ok = recovered == t_secret`
(`src/bin/genus_ladder.rs` at ca72267899: line 617 for the IC arms, line 539 for rho). I could not re-verify it independently from the records. The supporting
evidence is:
- the binary is the validated prime-order binary;
- my replay reproduced `ok` and `certified` on 24 rows;
- the instances equal curves the prime-order validator certified independently.

None of the prime-order validator's recovered-log certificates covers these exact (curve, k = 0) keys.
The study claims no new solve, so this does not affect the calibration. Do not cite these rows as
independently certified discrete-log results.

**F9 (low, interval coverage).** R1's cluster bootstrap has 4 clusters (2 curves × 2 seeds), so at
most 35 distinct resamples. My PCG64 bootstrap lands on exactly the frozen interval's endpoints, so
the endpoints are support points of a coarse discrete distribution. Two things follow:
- **The intervals understate uncertainty.** For example, [0.0052, 0.0054] for x at 16411.
- **t_G uncertainty is excluded, as the protocol says.** At 32771 there is one rho run, so no
  spread exists there. At 16411 the run-to-run spread is 1.9%, comparable to R1's half-width.

None of this is decision-relevant: P3's band is 10× wide.

**F10 (low, wording).**
- **"Lower bound".** "The ideal negation bound is a lower bound on any rho with the negation map":
  sqrt(πr/4) is the expected step count of an idealised random walk on r/2 classes. It is a
  heuristic reference in expectation, not a theorem and not a per-run bound.
- **"1 GAE is one Jacobian group operation (measured at 2.0 µs)".** t_G is rho wall time per group
  operation. That includes the per-step partition hash and two mod-r coefficient additions, about
  1% at t_la ≈ 8.4 ns. This is the right quantity for the ideal-rho time, but slightly more than a
  bare Cantor operation.
- **Missing context.** The measured plain-rho mean wall is not reported: 645 s at 16411 (x / measured
  plain rho = 0.0039) and 5338 s at 32771 (one run, 1.96× its expected steps).

**F11 (low, exploratory heading).**
- At 16411, "Tuning flips" holds: x\*/x paired wall is 1.064, and all 8 pairs fall in
  [1.022, 1.094].
- At 32771 it does not flip: x\*/x is 0.999 [0.979, 1.040], and cc 1.4 is still tied for
  wall-optimal. The body text says so, but the heading and "the w = 1 convention was steering tuning
  toward smaller bases" generalise from one prime on a 4-point grid.
- Each pass runs the cc settings in sequence, so drift is confounded with cc. The two passes,
  2.5 h apart, agree: A/A mean signed ln is +0.0007.

**F12 (info, disclosed deviations).** The protocol (§8, frozen before data) discloses:
- a local host, not isolab-eks (owner default);
- no S3;
- no PMU: timing tier, wall time only.

"Nothing else is started during a block" cannot be checked independently. File times and the bench
record are consistent with it: IC1 06:35:02–06:58:33, RHO16 06:58:48–07:31:32, RHO32
07:31:47–09:02:27, IC2 09:02:34–09:26:15. Lead C's searches started at 09:27:18. My replays ran with
4 search processes on cores 0–3, which affects wall time only.

## Not applicable or checked clean

- **L-546d37ca7750 (§5a band from canonical rule).** The study's scripts compute no §5a band and
  there is no ecbench session. Not applicable. The w = 1 ratios I recomputed on these rows agree with
  the validated prime-order figures, and rho's per-step hash and coefficient additions sit inside the
  measured t_G.
- **L-99ee952350b2 (real keys).** analyze.py indexes every count field with `[]` in G2 and G3.
  G1's `.get("certified")` and `.get("ok")` fail closed. My script indexes every key with `[]`.
- **L-26895912385b (unrounded ratios).** The decision thresholds and P1–P5 were recomputed from
  unrounded values, with paired ratios reported (x\*/x).
- **L-3f39e1d8b3a9 (binary at dispatch).** The binary was hashed and copied before any replay; its
  hash equals `binary.sha256`.
- **L-d12b64af75ed (shared seed across strata).** No cross-stratum interval is reported.
  Analyze.py reuses its stream across primes and arms, which only correlates the marginal intervals.

## Errata the producer should add

Put these in a sibling `ERRATA.md` and leave `RESULTS.md` byte-equal (L-8d86e8fc352c).
1. **F2:** drop "within 10%" and "so the counted operations account for the wall time". Report
   R² 0.942 / 0.986 and the intercept shares, and say that the cross-check is in-sample.
2. **F3 and F4:** state that the w intervals hold only under one linear model. Give the
   drop-one-cc ranges, the −0.68 s intercept at 32771 and the model-free upper bounds. Correct
   the collinearity sentence.
3. **F5:** label the post hoc statements.
4. **F6:** scope the "validated genus-4 studies" sentence to the measured primes and curves, and
   call the rest an extrapolation. Give the 25× figure per class.
5. **Optional:** F7, F10 and F11 wording.

## Validator outputs

| file | sha256 |
|---|---|
| `validation/recompute.py` | 75e23e2dc23093a2bd9bd6c189d056c3e88ddf79e5ac68e96899ba2fc80b4b06 |
| `validation/recompute.json` | b856e51aad4eca99e42481422c14fb395cee831ef9a14b3a793d5012d26ff378 |
| `validation/replay.sh` | 85667a0c5606e2627beaa5c02f03b4ee7635cfbc5d9d853aa58d2bfd9d79a5ce |
| `validation/compare_replay.py` | 53dd56152a70cc1fa305c0e5c3abf46eeea4510a3a4bb66b9cbd06097ed2cd08 |
| `validation/replay.json` | f223511ab07ea4e91b5e080265afd6d03b111ad3188b0b0161c54330e9a9a45e |
| `validation/replay/replay_p16411_cc2.0.jsonl` | 87bca7273ffc0a4799f6769d740565dc21a135055560f7be41f8965155a1288a |
| `validation/replay/replay_p16411_cc2.8.jsonl` | f73057e8128e1c6ea587025f8385a28be2f6f828aff3c167f7816c8cec5b1b72 |
| `validation/replay/replay_p32771_cc1.4.jsonl` | 9fe532f19823a7a5f46de2843cdf95cb56ead4653d2febdd33bccd37b75aa848 |
| `validation/artifacts_at_validation.sha256` | 1930bd382ec8a289d91470670aac244ca9296ededbab7dfa8d3b994d34c39c56 |

`validation/.gitignore` un-ignores the small replay `.jsonl` and `.log` files, each under 64 KiB
(L-81737122cefc).

To reproduce, run these from any directory:
1. `python3 -I research/genus4_cost_calibration_20261119/validation/recompute.py`
2. `bash …/validation/replay.sh`, which needs the binary with hash `493ee294…`
3. `python3 -I …/validation/compare_replay.py`

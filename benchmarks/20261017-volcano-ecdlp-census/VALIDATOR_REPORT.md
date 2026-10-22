# Validator report: volcano ECDLP census

Study `research/volcano_ecdlp_census_20261017/`, PR aburan28/crypto#2402, branch
`claude/volcano-ecdlp-census-phase-b-20261022` at `fbbf7becf708b0565f13a41055e26051e9ee1262`.
Validation ID `VAL-20261022-7824f7`. The validator did not write the study and did not open or
run any producer code (`analysis/*.py`); every number below was recomputed from
committed raw data with validator-written shell, mawk and jq in
`research/volcano_ecdlp_census_20261017/validation/`.

## Verdicts

| file | status | why |
|:--|:--|:--|
| `replay-certificate.json` | **verified** | 19 of 79 sessions replayed (seeded stratified sample: every volcano x family stratum, plus the three V3 crater sessions with rho-cubic); 109168 of 109168 measured runs re-executed with identical counts; 173808 of 173808 records scalar-verified ([k]G = Q); 19 sessions with --replay-all; --replay 50 fallback: none; archive hashes and receipt file maps all match |
| `validator-report.json` | **fail** | every recomputed number matches results.json, but material reporting errors were found: F1, F2 |
| `VALIDATION.json` | **not written** | the gate needs both files to pass |

The replay ran on the same shared 4-vCPU cloud VM type and host class as the sessions
(Intel Xeon @ 2.80 GHz, 4 logical CPUs, same ISA features, virtualised) and with
the same ecbench binary (`ac4bc5b26f858a4d...`, equal to the `runs/progress.jsonl` header).
ecbench's own env and hardware class ids differ from the session header's (kernel
6.18.44-fc-v139 then, -v140 now; the Phase B audits already span three of each).
Operation counts do not depend on the host, so this is an independent re-execution
of the counts, not an independent implementation or independent hardware. For the
same sessions, `total_gae`, `S` and the A2.1 uncharged count of all 109168
measured records were re-derived from `records.jsonl` and compared with the
committed extracts row by row: 0 mismatches.

## Recomputed headline numbers

192 numbers compared, 192 agree, 0 disagree; largest relative difference of a point estimate 3.846474796376052E-14; bootstrap endpoints within 1.47 Monte Carlo SD of the difference (tolerance 4).

| quantity | claimed (results.json) | recomputed |
|:--|--:|--:|
| (a) positive control rho-cubic / rho-neg, V3 crater | 0.5732765783660192 | 0.57327657836601909 |
| (b) R(L), V1 rho-neg L1 | 0.9923644029377502 | 0.99236440293774852 |
| (b) R(L), V1 ic L4 | 1.011350659305524 | 1.0113506593055195 |
| (b) R(L), V2 rho-plain L1 | 1.0299873539379816 | 1.029987353937982 |
| (b) R(L), V2 ic L3 | 0.9830581408669793 | 0.98305814086698295 |
| (c) IC / rho-neg, V1 all levels, 0 GAE | 42.41915345070031 | 42.419153450700051 |
| (c) IC / rho-neg, V1 all levels, 0.1 GAE | 46.373334748030274 | 46.373334748030622 |
| (c) IC / rho-neg, V1 all levels, 1 GAE | 65.13098467961635 | 65.130984679616432 |
| (c) IC / rho-neg, V2 all levels, 0 GAE | 61.526682818519056 | 61.526682818520122 |
| (c) IC / rho-neg, V2 all levels, 1 GAE | 92.67408788077574 | 92.67408788077681 |
| (c) IC / rho-neg, V3 all levels, 0 GAE | 58.8143277393736 | 58.814327739373596 |
| (d) measured runs | 403168 | 403168 |
| (d) verified runs | 403168 | 403168 |
| (d) warm-up runs dropped | 265952 | 265952 |
| (e) BSGS identical-ops fraction, V1 bsgs | 1.0 | 1 |
| (e) BSGS identity triples, V1 bsgs | 3600 | 3600 |
| (f) bootstrap 95% CI, V1 rho-plain L1/L0 | [0.973, 1.0617] | [0.9722, 1.0618] |
| (f) bootstrap 95% CI, positive control | [0.5619, 0.585] | [0.562, 0.5847] |
| (f) bootstrap 95% CI, IC / rho-neg V1 0 GAE | [41.925, 42.926] | [41.921, 42.933] |

All 35 R(L) values, all 51 IC/rho cells (levels and pooled, three prices), the 24
paired-source ratios, the 20 V3 descriptive ratios, the 6 curve-effect F statistics,
the A/A ratios, L2 fractions and uncharged counts are in `validator-report.json`
(`headline`). REPORT.md's 110 table cells in sections 4, 6 and 7 equal
results.json as printed.

## Findings

- **F1 (material)**, REPORT.md s7 (V3 table, level 0) and s9 (last paragraph). Report: On the V3 j = 0 crater IC's gap to rho is 49.0x / 53.3x / 73.3x (0 / 0.1 / 1 GAE), and 'it makes IC's gap to rho smaller on that curve (49x against about 60x at 0 GAE)'. Evidence: The gap is measured against rho-neg. On the j = 0 crater the strongest available rho is the automorphism fold rho-cubic, measured in the same sessions, binary and unit (the study's own positive control: 0.5733 x rho-neg). Against it IC is 84.9x / 92.8x / 131.1x on the same 2,000 D+N pairs (0 GAE 95% CI [82.7, 87.1]): larger, not smaller, than the 59.9-62.6x / 65.2-68.2x / 90.1-94.4x at the non-crater V3 levels (against rho-neg, which is the strongest reference there). AGENTS.md (validation gate) and docs/VALIDATION_GATE.md require the gap to the strongest available reference for that curve; the report states the opposite of what the matched reference shows. *Fix:* erratum: report the V3 crater row against rho-cubic (84.9 / 92.8 / 131.1) and withdraw the 'gap smaller on that curve' sentence; s1.5 'No level improves IC's position' then holds against the matched reference.
- **F2 (material)**, REPORT.md s9 ('Observed, not tested' paragraph), s6 V3 transport cells. Report: 'ic is about 22 to 25% more expensive at every non-crater V3 level than at the j = 0 crater ... This fits IC using the 6 automorphisms of j = 0 on the crater'. Evidence: The study's own isomorphic negative control on the same crater curve contradicts the j = 0 explanation. IC mean S on the V3 crater: D/N model (slug eeb6174a, 2,000 rows) 57.44 +- 0.50 (SE); the two isomorphic re-scalings, which have the same automorphism group: 67.00 +- 0.87 (a41c0795) and 72.59 +- 0.97 (d53b367d), mean 69.79, in line with the non-crater levels (isogenous 70.4-72.0, ctl 65.2-71.7). The crater 'discount' is a property of one Weierstrass model, i.e. a coordinate effect of the prime-abscissa factor base (cf. the coordinate effect reported in benchmarks/README.md row 20261017-isogeny-volcano-cost), not of the level or of j = 0. The same model drives the V3 D and Ua 'transport then solve' cells (about 1.2). The report presents a level-shaped observation and a mechanism without the negative control that the plan put in the design for exactly this purpose. *Fix:* erratum: state that the crater effect is not present on the isomorphic re-scalings (model effect), drop the automorphism explanation or label it refuted by the control.
- **F3 (minor)**, REPORT.md s3 and s11. Report: '(3 generators; rho-cubic was run on one of the two re-scalings)'; 'its positive control uses one of the two ctl re-scalings for rho-cubic'. Evidence: specs/V3-ctl-L0-c0.json lists both re-scalings and the rho-cubic arm; runs/extract has 1,000 measured rho-cubic rows on each (a41c0795, d53b367d); the 4,000 pairs are D 1,000 + N 1,000 + ctl 2 x 1,000. '3' is the number of distinct curve models (D and N share the crater model), not of generators (4). *Fix:* erratum text only; numbers unchanged.
- **F4 (minor)**, REPORT.md s6, transport-then-solve paragraph. Report: 'The V2 rho-plain Ua cell is the same at every level including level 0 (no transport at all)'. Evidence: Level-0 destinations are reached by transport from floor source vertex 364 (results.json transport_then_solve V2|Ua|rho-plain levels.0.mean_velu_evals = 13.0). The only zero-transport rows are those at the source vertex itself. The conclusion (a source-mean artifact, not a signal) stands: the ratio is ~0.6 at every level while Velu evaluations rise from 13.0 to 18.9. *Fix:* erratum text only.
- **F5 (minor)**, REPORT.md s5 table header. Report: 'Holm p (across 10)'. Evidence: results.json curve_effect['V2|rho-plain'].p_perm_holm = 0.12959 = 6 x 0.021598: Holm is across the 6 non-degenerate tests; the 4 BSGS cells are degenerate and untested. Across 10 the smallest adjusted p would be 0.216; no conclusion changes. *Fix:* erratum text only.
- **F6 (minor)**, AMENDMENTS.md A2 preamble and A2.3 first bullet. Report: 'These are written before analysis/analyze.py is run'; 'The five isogenous families give five slugs (five generators) per vertex'. Evidence: A2.3-A2.4 were appended after the final run (see amendment_A2_audit.timing; REPORT s11 discloses this correctly). A curve_slug identifies the model (p, a, b), not the generator; there are 2-4 distinct slugs per vertex on V1/V2 and 1-3 on V3 across the five isogenous families. The vertex resampling unit itself is right. *Fix:* a new amendment record (A3) correcting both statements; AMENDMENTS.md is not rewritten.
- **F7 (minor)**, REPORT.md s7, last paragraph. Report: 'The gap to rho is therefore about 43x unpriced and 65x at 1 GAE (V1)'. Evidence: 42.42x (s1 and the table say 42.4x). *Fix:* cosmetic.
- **F8 (minor)**, REPORT.md s9. Report: 'while the rho and BSGS arms show nothing'. Evidence: BSGS shows a constant 1.014 (bsgs) and 1.022 (bsgs-neg) at every non-crater V3 level. It is a scalar-mix artifact: crater rows are D + N (N uses target seed 20261115; mean S(N)/S(D) = 0.946), other levels D + Ua + Ub + N; (3 + 0.946)/4 / ((1 + 0.946)/2) = 1.0139, matching 1.01382. Not a level effect, but unexplained in the report. *Fix:* one explanatory sentence.
- **N1 (methodological note)**, ANALYSIS_PLAN s3 / s5 bootstrap (followed as frozen). Report: achieved 95% half-widths of 3-4% for rho and IC level ratios, and the inconclusive verdicts. Evidence: The frozen two-stage bootstrap (resample curves, then rows within each curve) counts the within-curve variance twice when the ICC is ~0 (mixed-model ICC <= 0.0074 here): on V1 rho-plain S/D my two-stage SD is 0.01567 versus 0.01134 resampling triples alone (ratio 1.38, about sqrt 2). The CIs are therefore conservative by about sqrt 2, which pushes knife-edge cells toward 'inconclusive': V1 rho-plain L1/L0 (90% upper 1.0542 against the margin 1.0526; V2 rho-plain L1/L0 was not re-run) and V1 rho-plain S/D (whose triple-only 95% CI [1.0061, 1.0506] excludes 1 while its 90% CI is inside the margin). This is the pre-registered procedure, so it is not an error; the report should not read 'inconclusive' there as a precision limit of the data. *Fix:* none required; optional sensitivity table.
- **N2 (methodological note)**, AMENDMENTS.md A2.3 s5 transport-then-solve. Report: the definition of 'exceeds the control spread' for transport-then-solve. Evidence: defined post hoc and decisive for the V2 rho-plain Ua cells; cannot be shown to predate the smoke run; conclusion robust (amendment_A2_audit.outcome_driven_assessment). *Fix:* none required.

## Audits

- **Verdict vocabulary** follows ANALYSIS_PLAN (equivalent / different / inconclusive, pass/fail, descriptive, distance remaining), apart from the F5 label.
- **Amendment A2** changes no estimand, margin, family, test or threshold. A2.3 and A2.4 were appended after the final analysis run: results.json pins the A2.1-A2.2 version of AMENDMENTS.md (`deb547a3...`, commit 89e88b1f94); A2.3-A2.4 first appear with results.json in 6ca47a183f. The one interpretation that decided a cell (the transport control) cannot be shown to predate the smoke run, but the conclusion does not depend on it.
- **Matched rho reference.** Negation is the strongest available on every V1 and V2 vertex and on the 15 non-crater V3 vertices (none has j = 0 or 1728). On the V3 j = 0 crater the strongest is the automorphism fold rho-cubic; the report uses rho-neg there (F1). IC / rho-cubic on the crater: 84.9x / 92.8x / 131.1x at 0 / 0.1 / 1 GAE.
- **AGENTS.md section 5a band** is applied to both arms with their named unpriced counters (ic: five counters; rho-neg and rho-cubic: canonicalisations). Rho is ahead at every price on every volcano and level; both arms remain lower bounds.

## What passing needs

An erratum record for REPORT.md (and an A3 amendment for the AMENDMENTS.md
statements in F6) that reports the V3 crater gap against rho-cubic and withdraws
the j = 0 interpretation (F1, F2), with the minor corrections F3-F8. Every number
already recomputes, so re-validation is `validation/build_validation.sh` on the
corrected snapshot.

## Limitations

- Replay and raw-record re-derivation cover 19 of 79 sessions (seeded stratified sample, every volcano x family stratum and all three rho-cubic crater sessions); the remaining 60 rest on their Phase B audits (--replay 12) and on the extracts.
- The replay used the same shared cloud VM type and CPU model (host class) and the same ecbench binary as the sessions; ecbench's env and hardware class ids differ (kernel 6.18.44-fc-v139 then, -v140 now). It is an independent re-execution of operation counts, not an independent implementation or independent hardware.
- Not independently re-run: the s3 mixed model (statsmodels MixedLM), the s4 permutation p-values, the s5 transport-then-solve CIs and the remaining bootstrap CIs; five CIs were spot-checked (positive control, V1 rho-plain L1/L0 at 95% and 90%, V1 IC/rho pooled at 0 GAE, V1 rho-plain S/D).
- Wall-time figures are descriptive in the study and were checked only as pooled ratios and L2 fractions.
- The cost tables declare their unit (S, with GAE pricing) but report no memory; at 26-bit group orders memory is not binding, and no time-memory tradeoff is claimed.
- A passing validation of this study would make its receipts admissible evidence; it would not support an ECDLP claim, a speedup, or promotion.

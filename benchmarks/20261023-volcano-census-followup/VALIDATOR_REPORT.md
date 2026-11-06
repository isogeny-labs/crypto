# Validator report: volcano ECDLP census follow-up

Study `research/volcano_ecdlp_census_followup_20261023/`, PR isogeny-labs/crypto#3476, branch `claude/volcano-census-followup-20261023` at `12e5eb3f7de9e764564bd7086e774a052887061a`. Validation ID `VAL-20261106-54796a`, round 1. The validator did not write the study and did not open or run any producer code (`analysis/*.py`, `analysis/build_pdf.sh`) or use its derived `runs/slugmap/`; every number below was recomputed from the committed session archives with validator-written code in `research/volcano_ecdlp_census_followup_20261023/validation/` (the census's `draw_sample.sh` and `replay_sample.sh`, adapted; the Python is new).

## Verdicts

| file | status | why |
|:--|:--|:--|
| `replay-certificate.json` | **verified** | 19 of 147 sessions replayed (seeded stratified sample: all 14 volcano x family strata, the 3 V5 crater rho-cubic sessions, 3 Tcr, 2 M, 1 redone `.r2`); 93248 of 93248 measured runs re-executed with identical counts; 134752 of 134752 records scalar-verified; 19 sessions with `--replay-all`, fallback: none; archive hashes and receipt file maps all match |
| `validator-report.json` | **fail** | 1587 of 1588 recomputed numbers agree; the H-M verdict does not reproduce (F1, material) |
| `VALIDATION.json` | **not written** | the gate needs a passing validator report |

Raw-record re-derivation, all 147 archives: 670688 records (428976 measured, 241712 warm-up) streamed from the committed archives (each archive's SHA-256 equal to its progress line); [recovered]G = Q, Q on the curve and [r]G = O checked with validator-written affine arithmetic: 0 failures; S = total_gae / sqrt(r): 0 failures; 0 unverified measured runs. Every row equals `runs/extract` and `runs/uncharged` (the A2.1 uncharged count re-derived independently): 0 mismatches over 670688 extract rows and 428976 uncharged rows. Census-vertex mapping by recomputed j-invariant (D rows carry the vertex's own model, ctl rows one of its two re-scalings, M rows one of its 8 models, Tcr rows the crater model with the source's generator): 0 failures; level from the vertex equals the spec's level everywhere.

The replay ran on the same shared 4-vCPU cloud VM type with the same ecbench binary as the sessions; it re-executes operation counts, it is not an independent solver implementation or independent hardware.

## Recomputed headline numbers

1588 numbers compared, 1587 agree, 1 disagree (H-M.n_significant_holm.ic). Point estimates agree to 3.7e-14 relative; bootstrap endpoints within 2.82 Monte Carlo SD of the difference (tolerance 4); permutation p within binomial Monte Carlo error.

| quantity | claimed (results.json) | recomputed |
|:--|--:|--:|
| positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl | 0.6112715895 | 0.6112715895 |
| positive_control.row_bootstrap.ci_hi | 0.6236714145 | 0.623536759 |
| level_ratio.R(L).V6.rho-neg.L2/L0 | 1.02167709 | 1.02167709 |
| level_ratio.V6.rho-neg.L2.ci90.ci_hi | 1.05353332 | 1.053196086 |
| level_ratio.R(L).V6.ic.L4/L0 | 0.9673268547 | 0.9673268547 |
| level_ratio.R(L).V5.ic.L6/L0 | 1.011130096 | 1.011130096 |
| level_effect.verdict_count.V6.equivalent | 19 | 19 |
| level_effect.verdict_count.V5.equivalent | 15 | 15 |
| H-T.overall.T.price_0.0 | 0.5946399614 | 0.5946399614 |
| H-T.overall.T.price_0.1 | 0.5917397989 | 0.5917397989 |
| H-T.overall.T.price_1.0 | 0.578119855 | 0.578119855 |
| H-T.overall.T.price_1.0.ci_hi | 0.6063349276 | 0.6071515561 |
| H-T.overall.T0.price_1.0 | 0.966974979 | 0.966974979 |
| H-T.overall.T0.price_1.0.ci_hi | 1.016384059 | 1.015743163 |
| H-T.cubic_over_neg_S_on_Tcr | 0.6147777292 | 0.6147777292 |
| H-M.V6:98.ic.F | 2.813305313 | 2.813305313 |
| H-M.V6:98.ic.p_perm | 0.007299270073 | 0.006399360064 |
| H-M.V6:214.ic.max_over_min | 1.576596623 | 1.576596623 |
| H-M.n_significant_holm.ic | 5 | 6 **(differs)** |
| H-M.n_significant_holm.rho-neg | 0 | 0 |
| H-M.n_significant_holm.bsgs | 0 | 0 |
| ic_over_rho-cubic.V5_crater.D+N.price_0.0 | 89.32398184 | 89.32398184 |
| ic_over_rho-cubic.V5_crater.D+N.price_0.1 | 98.20576664 | 98.20576664 |
| ic_over_rho-cubic.V5_crater.D+N.price_1.0 | 141.1645196 | 141.1645196 |
| ic_over_rho-neg.V6.pooled.price_0.0 | 60.40592971 | 60.40592971 |
| ic_over_rho-neg.V6.pooled.price_1.0 | 90.79609628 | 90.79609628 |
| ic_over_rho-neg.V5.pooled_levels_ge1.price_0.0 | 53.28369275 | 53.28369275 |
| ic_over_rho-neg.V5.pooled_levels_ge1.price_1.0 | 81.76569882 | 81.76569882 |
| counts.rows_measured | 428976 | 428976 |
| counts.rows_warmup_dropped | 241712 | 241712 |
| counts.unverified_rows_total | 0 | 0 |
| bsgs_identity.V6.bsgs.fraction_identical_ops | 1 | 1 |
| bsgs_identity.V6.bsgs.n_triples | 6336 | 6336 |
| bsgs_identity.V5.bsgs.n_triples | 3064 | 3064 |
| curve_effect.V6.rho-neg.F | 1.04118928 | 1.04118928 |
| mixed_model.V6|ic.icc | 0.004969806333 | 0.004969806333 |
| wall.V5.aa_ratio(isogenous families, as results.json) | 1.042419084 | 1.042419084 |

All 50 R(L) values with their 90% and 95% CIs and verdicts, the 63 per-vertex and 6 per-level H-T values at three prices, every H-M F, permutation p, model mean and max/min, every IC/rho cell (levels, pooled, crater against rho-cubic, M curves), the 100 transport-then-solve point ratios and native controls, the curve-effect F and p, the paired-source ratios, the A/A and L2 fractions and the mixed-model ICCs are in `validator-report.json` (`headline`). REPORT.md tables: 840 cells checked, 840 equal to results.json at the report's rounding, 1 not reproduced by the validator (the V6:98 significance mark, F1).

## Findings

- **F1 (material; open)**, REPORT.md s1 item 5 and s8 (and the same text in report.typ / report.pdf). Report: 'H-M ...: INCONCLUSIVE. ic is significant (Holm, alpha 0.05) on 5 of 12 chosen curves ... IC's model dependence is therefore present on some curves and absent on others: it is neither a general property nor noise.' and s8 'With 5 of 12 the verdict is inconclusive by the plan's rule, which needed 6.' Evidence: The verdict turns on one curve that sits on the Monte Carlo boundary. V6:98 ic: F = 2.8133 (validator 2.8133, equal to 1e-15); it is the 6th-smallest p of 12, so its Holm multiplier is 7. Producer, 10,000 shuffles: p = 0.0073, Holm 0.0511 -> not significant -> 5 of 12 -> 'inconclusive'. Validator, independent code, same frozen seed (20261024) and count (10,000), different draw order: p = 0.0064, Holm 0.0448 -> significant -> 6 of 12 with rho-neg 0 and bsgs 0, which the frozen rule (ANALYSIS_PLAN s5c) reads as 'supported for IC'. The same test evaluated to high precision (validation/hm_precise.py, 2,000,000 shuffles): p = 0.007111 +- 0.000059 (MC SE), Holm 0.0498 +- 0.0004 -> 6 of 12; the parametric one-way ANOVA F(7, 504) gives p = 0.00699, Holm 0.0489 -> 6 of 12. Both 10,000-shuffle Holm values agree within Monte Carlo error (the numbers are not wrong); the categorical verdict does not reproduce. The report states 'inconclusive' as a settled outcome and interprets it ('absent on others', 'neither a general property nor noise'); a curve that is not significant after Holm is not shown to lack a model effect, and the five-versus-six split is within Monte Carlo noise of the frozen threshold. *Fix:* Erratum (a new AMENDMENTS record, REPORT.md and report.pdf; the frozen rule, seed and shuffle count are not changed): report the frozen-seed count (5 of 12, inconclusive) together with the fact that the 6th curve (V6:98, Holm p 0.0511) lies within Monte Carlo error of 0.05 and that a precise evaluation of the same permutation test (and the parametric F test) gives 6 of 12, which the rule reads as supported for IC; state that the H-M verdict is on the boundary between 'inconclusive' and 'supported for IC' and is not robust either way; withdraw 'absent on others' and 'neither a general property nor noise'. The figure 8 caption / s8 table can keep the frozen-seed values.
- **F2 (minor; open)**, REPORT.md s1 item 2 heading. Report: 'Q1, V6 ... level effect: no method depends on level.' Evidence: 1 of 20 V6 cells is 'inconclusive' (rho-neg L2: 90% CI upper 1.0535 against the margin 1.0526; validator 1.0532), so equivalence is not shown for every (arm, level). Two V6 TOST cells sit within about 2 Monte Carlo SE of the margin and were reproduced by the validator's independent bootstrap on the same side: rho-neg L2 (above) and rho-plain L1 (90% lower 0.9508, validator 0.9504, margin 0.95). *Fix:* Heading: '19 of 20 equivalent, 1 inconclusive, none different' (the sentence that follows already says so); optionally note the two near-margin cells.
- **F3 (minor; open)**, REPORT.md s1 item 4 (last sentence) and s12 (H-T bullet). Report: 'it is relative to rho-neg on the same instance, not to the strongest rho at the source' / 'it does not compare with the strongest rho at the source (on j != 0 vertices rho-neg is the strongest)' Evidence: Every one of the 63 Tcr source vertices has j not in {0, 1728} (validator recomputed j(a, b) for all 64 V5 vertices; only the crater has j = 0) and the field is prime, so Aut = {+-1} and rho-neg is the strongest available rho on every source curve: H-T's denominator is the matched, strongest rho at the source. The two sentences contradict each other and the first understates the comparison. *Fix:* Wording: 'relative to rho-neg at the source, which is the strongest rho available on every (j != 0) source curve'.
- **F4 (minor; open)**, REPORT.md s1 item 7 and s10; results.json wall_admission. Report: A/A ratio V5 1.042, V6 1.001; 'only 87% (V5) and 84% (V6) of rows reached isolation L2'. Evidence: The A/A ratio and the per-arm L2 fractions use the isogenous families only (pairs 14256 / 31680), while ANALYSIS_PLAN s8 lists D, Ua, Ub, S, N and ctl as the families that enter. With ctl: A/A 1.0393 (V5, 22384 pairs) and 1.0019 (V6, 44352), still inside [0.95, 1.05]; per-arm L2 fractions rise by 0.4-3.6 points and stay below 100%, so 'not admitted' holds either way. The 87% / 84% of s1 is over all rows of the volcano (M and Tcr included), the s10 per-arm values over isogenous rows only; neither is stated. *Fix:* State the row set used (or follow s8's family list); no verdict changes.
- **F5 (minor; open)**, AMENDMENTS.md A2.1. Report: 'Between 05:41Z and 05:43Z, ecbench refused to start 9 short Tcr sessions' Evidence: The 9 status 'failed' lines of runs/progress.jsonl are timestamped 05:32:32Z to 05:43:01Z (V5-Tcr-L5-c19 05:32:32, V5-Tcr-L5-c27 05:36:10, V5-Tcr-L6-c45 05:40:01, then six between 05:41:11 and 05:43:01). All 9 carry the refusal signature A2.1 names (rc_run 2, session_status 'missing', runs 0, wall 2-3 s); the refusal message itself is not preserved (no session log contains 'not quiet'; the redone sessions' logs replaced them). All 9 specs later ran and count once; no data is affected. *Fix:* A new amendment record correcting the time span (AMENDMENTS.md is append-only).
- **F6 (minor; open)**, REPORT.md s5 last sentence. Report: 'No variance-ratio test is significant and no curve is flagged: no V6 curve differs from its own isomorphic re-scalings beyond the control spread' Evidence: With 648 curves and the pooled-|z*| reference of s4 (3 label outcomes per vertex) the smallest attainable p_v is about 1/1944 and the smallest attainable Holm-adjusted p about 648/1944 = 0.33, so no curve can reach adj. p < 0.01 whatever the data. results.json: the most extreme rho-neg curve (vertex 545, level 4, z = 5.12) has Holm p 0.331. 'No curve is flagged' is true but uninformative; the claim about curves rests on the aggregate F tests (p 0.15 / 0.67 / 0.42), which do support 'no curve effect beyond the control spread' in aggregate. The design is frozen (and is the census's). *Fix:* Say that the per-curve flag cannot fire at this size and that the statement rests on the F test.
- **F7 (minor; open)**, REPORT.md s4 (V5 bsgs and bsgs-neg rows) and s1 item 3. Report: V5 bsgs R(L) = 0.988 and bsgs-neg R(L) = 0.982 at every level 1-6, listed without comment. Evidence: A constant ratio at every level is a family-mix artifact, not a level effect: Ua and Ub have no V5 level-0 rows, so the crater mean is D + N while every other level is D + Ua + Ub + N; BSGS cost depends only on k, D/Ua/Ub share k (target seed 20261123) and N has its own (20261125), with mean S(N)/S(D) = 1.0484 (bsgs) and 1.0753 (bsgs-neg). (3 + x) / (2 (1 + x)) = 0.98819 and 0.98186 reproduce the constants exactly. The same mix difference enters every V5 arm's R(L) (negligibly for rho and ic). The census validation flagged the same pattern (census F8). *Fix:* One explanatory sentence in s4.
- **N1 (methodological note)**, REPORT.md s8 table (max/min columns). The percentile bootstrap CI of the max/min of 8 model means is biased upward and does not cover the point estimate when there is no effect (bsgs, whose model means are identical: 1.000 with CI [1.03, 1.11]; rho-neg V6:2: 1.10 [1.11, 1.38]). These CIs cannot be read against 1; IC's max/min (1.12-1.58) must be read against rho-neg's on the same curves (1.10-1.26), the null object, as the report's sentence on rho-neg partly does.
- **N2 (cosmetic)**, ANALYSIS_PLAN.md s10. The frozen plan names the benchmarks copy benchmarks/20261023-volcano-ecdlp-census-followup/; the copy is in benchmarks/20261023-volcano-census-followup/ (the directory this validation writes to). Contents identical to research/.../report.pdf and analysis/figures (sha256 checked).
- **N3 (note)**, H-T (s5b, A3.2). The vertex-weighted pooling (A3.2) gives T = 0.5946 / 0.5781 (0 / 1 GAE); the row-pooled alternative gives 0.6000 / 0.5831. The verdict (supported) does not depend on the choice; the T0 control's 95% CI includes 1 at all three prices with its upper end about 0.014 above 1, many Monte Carlo SE away.

## Audits

- **Snapshot.** `12e5eb3f7d` is committed and pushed (local HEAD equals the origin branch), working tree clean. PLAN.md, ANALYSIS_PLAN.md, specs/ and census/ are unchanged since the freeze; ANALYSIS_PLAN.md (00:42Z) and A1 (00:50Z) precede the progress header (01:09Z) and the first session (01:15Z). results.json pins 521 inputs, all equal to the snapshot.
- **Verdict vocabulary** follows ANALYSIS_PLAN (equivalent / different / inconclusive / descriptive; pass/fail; supported / refuted / inconclusive; supported for IC; descriptive (not admitted); signal, not a speedup claim; distance remaining). The departures are interpretive sentences (F1, F2).
- **Amendments.** A1 and A2 change no estimand, margin, family, test or threshold; A1's H-T unit change (S to priced ops_gae) leaves the ratio unchanged because every V5 curve has the same r. A3 (post hoc, labelled so) fills gaps without changing an estimand, margin, family, test, seed or threshold; A1 text is a byte prefix of A1+A2, which is a byte prefix of the current file. A3.7 fixes the H-M generator scheme only after data existed; the frozen plan fixed the seed and the count, not the draw order, which is why F1 matters. A2.1's time span is wrong (F5).
- **Matched rho.** V5: p = 1 (mod 3) and r = 1 (mod 3), so the j = 0 crater's order-6 automorphism is defined over F_p and rho-cubic is the strongest rho there; the report compares IC with rho-cubic on the crater (89.3x / 98.2x / 141.2x). The other 63 V5 vertices and all 648 V6 vertices have j not in {0, 1728} (recomputed): negation is the strongest there, and it is the reference. H-T's rho-neg denominator at j != 0 sources is the strongest rho at the source (F3 is wording).
- **AGENTS.md section 5a band** applied to both arms with their named uncharged counters; rho ahead at every price, every level, both volcanoes and the crater; both arms remain lower bounds.
- **Controls.** Q2a positive control passes (0.6113, validator CI [0.5996, 0.6235]); the V6 ctl negative control's F tests reproduce exactly; H-T's T0 control includes 1 at all prices; H-M's rho-neg and bsgs controls are significant on 0 of 12 curves and bsgs ops are identical across all 8 models for every instance.
- **Redone sessions.** The three `runs/orphans/` partial sessions (3,013 / 5,211 / 547 records, status 'running') have every record identical (seq, arm, workload, total_gae, recovered) to the same record of the redone `.r2` session: the redo selected nothing.

## Limitations

- Replay (ecbench verify --replay-all) covers 19 of 147 sessions (seeded stratified sample: every volcano x family stratum, the three V5 crater rho-cubic sessions, 3 Tcr, 2 M and one redone .r2 session); the other sessions rest on their Phase B audits (12 replays each, 1,764 identical) and on the validator's raw-record re-derivation, which covers all 147 archives ([k]G = Q with independent arithmetic, S, uncharged counts, extracts row by row).
- The replay used the same shared cloud VM type and the same ecbench binary as the sessions; it is an independent re-execution of operation counts, not an independent solver implementation or independent hardware.
- Bootstrap CIs and permutation p-values were recomputed with independent generators: they agree within Monte Carlo error, not bit for bit; verdicts that sit within about 2 Monte Carlo SE of a threshold (F1, F2) can differ between implementations.
- Not re-run: the s5 transport-then-solve and paired-source bootstrap CIs and signal flags (all 100 point ratios and native controls recomputed exactly), per-vertex H-T CIs and the V5 per-family R(L); the s4 per-curve p-values were computed from the exact three-outcome reference distribution instead of 10,000 label shuffles; the s3 mixed model was re-run with the same library (statsmodels) for ic and rho-neg only.
- Wall-time results are descriptive in the study and were checked only as A/A ratios and L2 fractions.
- This validation does not support an ECDLP claim, a speedup, or promotion; a pass would make the receipts admissible evidence for the benchmarks row only.

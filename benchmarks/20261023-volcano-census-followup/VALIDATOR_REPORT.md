# Validator report: volcano ECDLP census follow-up (round 2)

Study `research/volcano_ecdlp_census_followup_20261023/`, PR isogeny-labs/crypto#3476, branch `claude/volcano-census-followup-20261023` at `fb7f81aac38733d4b72043624d92d38347588c43` (the round-1 erratum commit). Validation ID `VAL-20261106-1bc325`, round 2. The validator did not write the study, its sessions, its analysis, its report, the erratum or the round-1 validation, and did not open producer code (`analysis/*.py`, `analysis/build_pdf.sh`) or the round-1 validator's scripts; it did not use `runs/slugmap/`. Every number below was recomputed from the committed session archives with validator-written code in `research/volcano_ecdlp_census_followup_20261023/validation/round2/` (the census's `draw_sample.sh` and `replay_sample.sh`, adapted; the Python is new).

Previous validation `VAL-20261106-54796a` at `12e5eb3f7d` (validator commit `abd653551e`): **fail**, 1587 of 1588 numbers agreed; material F1 (H-M verdict), minor F2-F7, notes N1-N3. They are kept below with their resolution.

## Verdicts

| file | status | why |
|:--|:--|:--|
| `replay-certificate.json` | **verified** | 20 of 147 sessions replayed from the committed archives with `ecbench verify --replay-all` (new seeded draw, nonce = snapshot: all 14 volcano x family strata, the 3 V5 crater rho-cubic sessions, 3 Tcr, 2 M, 2 redone `.r2`); 98032 of 98032 measured runs re-executed with identical counts; 141168 of 141168 records scalar-verified; fallback: none; archive hashes, binary sha256 and receipt file maps all match |
| `validator-report.json` | **pass** | 995 of 995 recomputed numbers and 129 of 129 categorical outcomes agree; 745 of 745 REPORT.md cells equal results.json; F1-F7 resolved by the round-1 erratum |
| `VALIDATION.json` | **written** | pins both files by sha256 with the matched rho reference and the section 5a band |

Raw-record re-derivation, all 147 archives (streamed, nothing unpacked to disk): 670688 records; archive SHA-256 equal to the progress line 147/147; archived `spec.json` equal to `specs/` 147/147; [recovered]G = Q, Q on the curve, [r]G = O (5077 generator checks), S = total_gae / sqrt(r), outcome verified, frozen binary in every record, census vertex by recomputed j-invariant, level = spec level: failures 0. Equal to `runs/extract` and `runs/uncharged` row by row: 670688 extract rows and 428976 uncharged rows, 0 missing, mismatches 0. Phase B audits: 147/147 OK, 1764/1764 replays reproduced.

## Recomputed headline numbers

995 numbers compared, 995 agree. Point estimates agree to 3.7e-14 relative; bootstrap endpoints within 2.86 Monte Carlo SD of the difference and permutation p-values within 2.65 binomial SD (tolerance 4). Values quoted in the erratum from the round-1 validator (with-ctl A/A, F7 ratios, row-pooled H-T) agree at their printed rounding.

| quantity | claimed | recomputed |
|:--|--:|--:|
| positive_control.ratio_rho-cubic_over_rho-neg.V5_crater.D+N+ctl | 0.6112715895 | 0.6112715895 |
| positive_control.row_bootstrap.ci95_hi | 0.6236714145 | 0.6238027238 |
| level_effect.R(L).V6.rho-neg.L2 | 1.02167709 | 1.02167709 |
| level_effect.V6.rho-neg.L2.ci90_hi | 1.05353332 | 1.053049684 |
| level_effect.V6.rho-plain.L1.ci90_lo | 0.9507636133 | 0.9502813543 |
| level_effect.R(L).V6.ic.L4 | 0.9673268547 | 0.9673268547 |
| level_effect.R(L).V5.ic.L6 | 1.011130096 | 1.011130096 |
| level_effect.verdict_count.V6.equivalent | 19 | 19 |
| level_effect.verdict_count.V6.inconclusive | 1 | 1 |
| level_effect.verdict_count.V5.equivalent | 15 | 15 |
| H-T.overall.T.price_0.0 | 0.5946399614 | 0.5946399614 |
| H-T.overall.T.price_0.1 | 0.5917397989 | 0.5917397989 |
| H-T.overall.T.price_1.0 | 0.578119855 | 0.578119855 |
| H-T.overall.T.price_1.0.ci95_hi | 0.6063349276 | 0.6062326767 |
| H-T.overall.T0.price_0.0 | 0.9672438235 | 0.9672438235 |
| H-T.overall.T0.price_1.0 | 0.966974979 | 0.966974979 |
| H-T.overall.T0.price_1.0.ci95_lo | 0.9204398007 | 0.9207644775 |
| H-T.overall.T0.price_1.0.ci95_hi | 1.016384059 | 1.015979817 |
| H-T.cubic_over_neg_S_on_Tcr | 0.6147777292 | 0.6147777292 |
| H-T.transport_share_of_numerator.price_1.0 | 0.002072465525 | 0.002072465525 |
| H-M.V6:98.ic.F | 2.813305313 | 2.813305313 |
| H-M.V6:98.ic.p_perm | 0.007299270073 | 0.007799220078 |
| H-M.V6:98.ic.p_holm | 0.05109489051 | 0.05459454055 |
| H-M.V6:214.ic.max_over_min | 1.576596623 | 1.576596623 |
| H-M.V6:214.ic.F | 5.905882353 | 5.905882353 |
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
| wall.V5.aa_ratio(isogenous) | 1.042419084 | 1.042419084 |
| wall.V5.aa_ratio(isogenous+ctl) (erratum F4, rounded) | 1.0393 | 1.039256391 |
| wall.V6.aa_ratio(isogenous+ctl) (erratum F4, rounded) | 1.0019 | 1.001910434 |
| mixed_model.V6|ic.icc | 0.004969806333 | 0.004969806333 |
| H-M.n_significant_holm.ic | 5 (frozen seed) | 5 (own 10,000 shuffles), 6 (1,000,000 shuffles), 6 (parametric) |
| H-T.verdict | supported | supported |
| positive_control.pass | True | True |

All 50 R(L) cells with their 90% and 95% CIs and verdicts, the 6 per-level and 63 per-vertex H-T values with M_v, every H-M model mean, F, permutation and Holm p and max/min, every IC/rho cell (levels, pooled, the crater against rho-cubic, the 12 M curves), BSGS identity, the curve-effect F tests, the A/A ratios and L2 fractions and the mixed-model ICCs are in `validator-report.json` (`headline`, `categorical`).

## Round-1 findings and their resolution

- **F1 (material; resolved).** Round 1: H-M verdict not reproducible: V6:98 ic Holm p 0.0511 (producer, 10,000 shuffles) vs 0.0448 (round-1 draw), 0.0498 +- 0.0004 (2,000,000 shuffles), 0.0489 (parametric F(7,504)); REPORT stated 'INCONCLUSIVE' as settled and interpreted beyond it ('absent on others', 'neither a general property nor noise'). Resolution: Erratum (round 1, post hoc) in REPORT.md (top section, s1 item 5, s8, s12), report.typ/report.pdf (rebuilt) and AMENDMENTS.md (appended record): the frozen-seed count (5 of 12, inconclusive) is kept and the verdict is stated to sit on the Monte Carlo boundary with 'supported for IC', not robust either way; both phrases withdrawn; the IC model-dependence statement limited to 'significant on 5 or 6 of 12 curves, rho-neg and bsgs on none'. Round-2 check: Independent again: V6:98 ic F = 2.813305313 (equal). This validator's own 10,000-shuffle draw (seed 20261024, own draw order): p = 0.0078, Holm 0.0546, ic count 5 of 12 -> 'inconclusive'; 1,000,000 shuffles: p = 0.007054 +- 0.000084, rank 6, Holm multiplier 7, Holm 0.0494 +- 0.0006 -> 6 of 12; parametric Holm 0.0489 -> 6 of 12 ('supported for IC'). Three independent 10,000-shuffle draws now give 5, 6 and 5; precise evaluations put Holm about one Monte Carlo SE below 0.05. The corrected report statement (boundary, not robust either way) is exactly what the recompute shows. rho-neg and bsgs: 0 of 12 in every evaluation.
- **F2 (minor; resolved).** Round 1: s1 item 2 heading 'no method depends on level' although V6 rho-neg L2 is inconclusive; two cells within about 2 MC SE of the TOST margin. Resolution: Heading now '19 of 20 ratios equivalent, 1 inconclusive, none different'; the old heading is withdrawn; both near-margin cells named in the erratum. Round-2 check: Own bootstrap: V6 rho-neg L2 90% CI upper 1.0530 > margin 1.0526 (inconclusive); V6 rho-plain L1 90% CI lower 0.9503 > 0.95 (equivalent): same sides as the producer. V6 counts {'equivalent': 19, 'inconclusive': 1}.
- **F3 (minor; resolved).** Round 1: s1 item 4 said H-T is 'not relative to the strongest rho at the source', contradicting s12. Resolution: s1 item 4 and the s12 H-T bullet now say rho-neg at the source is the strongest rho available on every (j != 0) source curve. Round-2 check: census/V5/vertices.jsonl: j recomputed from (a, b) for all 64 vertices; j = 0 only at vertex [0], j = 1728 nowhere; prime field (p = 66678169), so Aut = {+-1} at all 63 sources.
- **F4 (minor; resolved).** Round 1: A/A ratio and L2 fractions computed on isogenous families although ANALYSIS_PLAN s8 lists ctl; row sets of 87% / 84% unstated. Resolution: Row sets stated in s1 item 7, s10 and s12; with-ctl values quoted; 'not admitted' unchanged. Round-2 check: Own recompute: A/A with ctl V5 1.0393 (22384 pairs), V6 1.0019 (44352 pairs); per-arm L2 fractions rise by 0.4-3.6 points with ctl and stay below 100%; all measured rows of the volcano at L2: V5 0.8724, V6 0.8404.
- **F5 (minor; resolved).** Round 1: AMENDMENTS A2.1 gave the busy-host refusal span as 05:41Z-05:43Z. Resolution: Appended AMENDMENTS record and REPORT erratum give 05:32:32Z-05:43:01Z and say the refusal message was not preserved; A2.1 text itself left unedited (append-only). Round-2 check: runs/progress.jsonl: 9 'failed' lines, V5-Tcr-L5-c19 05:32:32Z ... V5-Tcr-L6-c62 05:43:01Z, all rc_run 2 / session_status 'missing'; no sessions/*.log or runs/*.log contains 'not quiet'; AMENDMENTS.md at 12e5eb3f7d is a byte prefix of the current file.
- **F6 (minor; resolved).** Round 1: 'No curve is flagged' cannot be otherwise: smallest attainable Holm-adjusted per-curve p about 0.33 with 648 curves. Resolution: s5 erratum sentence: the per-curve flag cannot fire at this size; the claim rests on the aggregate F tests. Round-2 check: Own recompute: minimum attainable Holm p = 648 / (3 x 648) = 0.333; results.json minimum 0.3306 (vertex 545, z = 5.12). Aggregate F (own): rho-neg 1.0412 p 0.149, rho-plain 0.9776 p 0.662, ic 0.9897 p 0.425.
- **F7 (minor; resolved).** Round 1: Constant V5 bsgs / bsgs-neg R(L) listed without comment (family-mix artifact). Resolution: Explanatory paragraph after the s4 table. Round-2 check: Own recompute on the crater: mean S(N)/S(D) = 1.04838 (bsgs), 1.07528 (bsgs-neg); (3 + x)/(2(1 + x)) = 0.988192 and 0.981863, equal to R(L) at every level to 1e-14.
- **N1 (note; carried).** Round 1: max/min bootstrap CI of eight model means is biased upward; read IC against rho-neg. Resolution: Added to s8. Round-2 check: Own recompute reproduces every max/min point value exactly.
- **N2 (note; carried).** Round 1: benchmarks directory name differs from ANALYSIS_PLAN s10. Resolution: Recorded in AMENDMENTS erratum. Round-2 check: report.pdf and all 20 figure files in benchmarks/ equal the study copies (sha256).
- **N3 (note; carried).** Round 1: row-pooled H-T 0.6000 / 0.5831 vs vertex-weighted 0.5946 / 0.5781. Resolution: Recorded in AMENDMENTS erratum. Round-2 check: Own row-pooled T: 0.6000 (0 GAE), 0.5831 (1 GAE); verdict unchanged.

## Round-2 findings

None. Four notes, none affecting a number or a verdict:

- **R2-N1** (REPORT.md s8 (sentence before the erratum F1 text)): 'the V5 crater's own D/N model and its two ctl re-scalings give IC mean S 59.3, 59.2, 58.1' lists the values in the reverse order of the models: results.json crater_models_V5 has the D/N model at 58.10 and the two ctl re-scalings at 59.26 and 59.18. The sentence's point (no model gap on that curve) holds either way. Cosmetic.
- **R2-N2** (REPORT.md s1 item 2 (last sentence) and s8 ('the V6 curve-effect test (section 5) finds no curve differing from its re-scalings')): Both sentences predate the erratum and read slightly wider than the erratum allows: Q1 shows equivalence in 19 of 20 cells (one inconclusive), and the curve statement rests on the aggregate F tests only (erratum F6). The report's own rule ('where the original text below conflicts with this section, this section governs') covers both; no number is affected.
- **R2-N3** (REPORT.md s13 (Reproduce) and AMENDMENTS erratum): The erratum was applied to REPORT.md and report.typ by hand and the PDF rebuilt with pandoc/typst; analysis/build_report.py was not changed, so running the s13 reproduce chain would regenerate the pre-erratum text. AMENDMENTS says so; s13 does not. results.json and the figures are unaffected.
- **R2-N4** (ecbench record metadata (not the report)): The crater workload's metadata reports automorphisms_available = 2 although p = 1 (mod 3) gives the j = 0 curve an order-6 automorphism group over F_p, which rho-cubic uses (boundaries.automorphisms_used = 6). A metadata quirk of the frozen binary; no number depends on it.

## Audits

- **Snapshot.** `fb7f81aac3` is committed and pushed (local HEAD equals the origin branch), working tree clean. The erratum commit touches only REPORT.md, report.typ, report.pdf (both copies) and AMENDMENTS.md (appended). results.json is unchanged since round 1; 520 of its 521 pinned inputs hash equal now, and the one that differs is AMENDMENTS.md, pinned at its A1-A3 version, a byte prefix of the current file. PLAN.md, ANALYSIS_PLAN.md (frozen at dbd0f9f274, 2026-10-23T00:42Z, before the progress header 01:09Z), specs/, census/ unchanged; sessions/, runs/progress.jsonl, runs/extract unchanged since the Phase B close df7a0e5f50.
- **Amendments.** A1 and A2 change no estimand, margin, family, test or threshold (A1.4a's H-T unit S -> priced ops_gae leaves the ratio unchanged: one r per volcano). A3 (post hoc, labelled) fills gaps without changing an estimand, margin, family, test, seed or threshold. The round-1 erratum changes no estimand, margin, family, test, seed, shuffle count, threshold or results.json value; it adds wording, states row sets and quotes the round-1 validator's numbers with attribution. AMENDMENTS.md at 2f80fe625f (A1), 141d319090 (A1+A2), 12e5eb3f7d (A1-A3) are byte prefixes of the current file.
- **Verdict vocabulary.** REPORT.md uses the plan's words: PASS (Q2a), equivalent / inconclusive / different / descriptive (s3), SUPPORTED / refuted / inconclusive (H-T), inconclusive / supported for IC (H-M, now stated as on the boundary between the two), not admitted / descriptive (wall), signal, not a speedup claim (s5), distance remaining (s9).
- **Matched rho.** PLAN s2/s4 and ANALYSIS_PLAN s9 name rho-neg (rho.negation) as the matched rho, paired with ic on the same (workload, target, round), and rho-cubic (rho.cubic, the order-6 automorphism fold) on the V5 j = 0 crater (census A3 F1). rho-neg is the strongest available on all 648 V6 vertices and the 63 non-crater V5 vertices (j not in {0, 1728}, prime fields); on the V5 crater (p = 1 and r = 1 mod 3) the strongest is rho-cubic, and the report compares IC with it there (89.3x / 98.2x / 141.2x at 0 / 0.1 / 1 GAE). H-T's denominator is rho-neg at j != 0 sources, the strongest there.
- **AGENTS.md section 5a band** (mean priced S over matched pairs; both arms priced with their own named uncharged counters):

| population | price | ic | rho | ic / rho |
|:--|--:|--:|--:|--:|
| V6 (rho-neg) | 0 | 79.022 | 1.3082 | 60.4 |
| V6 (rho-neg) | 0.1 | 93.199 | 1.4205 | 65.6 |
| V6 (rho-neg) | 1 | 220.785 | 2.4317 | 90.8 |
| V5_levels_1-6 (rho-neg) | 0 | 57.258 | 1.0746 | 53.3 |
| V5_levels_1-6 (rho-neg) | 0.1 | 68.366 | 1.1730 | 58.3 |
| V5_levels_1-6 (rho-neg) | 1 | 168.339 | 2.0588 | 81.8 |
| V5crater (rho-cubic) | 0 | 58.100 | 0.6504 | 89.3 |
| V5crater (rho-cubic) | 0.1 | 69.376 | 0.7064 | 98.2 |
| V5crater (rho-cubic) | 1 | 170.855 | 1.2103 | 141.2 |

  Rho is ahead at every price, at every level of both volcanoes (every recomputed cluster-bootstrap lower end above 1) and on the crater against rho-cubic; both arms remain lower bounds. The ratio is the distance remaining, not a pass/fail.
- **Controls.** Q2a positive control passes (0.6113, own 95% CI upper 0.6238 < 0.85), so the equivalence verdicts are informative. H-T's T0 control: own 95% CI includes 1 at all three prices (upper end 1.0160 at 1 GAE); T's CI is below 1 at all three; verdict supported, reproduced. H-M's null objects: rho-neg significant on 0 of 12 curves in every evaluation; bsgs between-model variance 0 with identical ops across all 8 models for every instance. The V6 ctl negative control's F tests reproduce exactly (own permutation p 0.149 / 0.662 / 0.425 for rho-neg / rho-plain / ic).
- **Redone sessions.** `V6-D-L1-c0`: 3013 partial records (status 'running'), 3013 identical in `V6-D-L1-c0.r2`; `V6-D-L2-c0`: 5211 partial records (status 'running'), 5211 identical in `V6-D-L2-c0.r2`; `V6-N-L1-c0`: 547 partial records (status 'running'), 547 identical in `V6-N-L1-c0.r2`. The redo selected nothing; the nine busy-host `failed` lines (05:32:32Z-05:43:01Z) started no session, and their specs count once.
- **Report and PDF.** 745 of 745 REPORT.md cells (s1, s2, s4, s5, s7, s8, s9, s10 tables and verdict sentences) equal results.json at the report's rounding. report.pdf (study and benchmarks copies identical, sha256) contains the erratum section and corrected sentences (pdftotext); 20 figure files in benchmarks/ equal analysis/figures/ (sha256).

## Limitations

- Replay (ecbench verify --replay-all) covers 20 of 147 sessions (seeded stratified sample: all 14 volcano x family strata, the 3 V5 crater rho-cubic sessions, 3 Tcr, 2 M, 2 redone .r2); every session also has its committed Phase B audit (1,764 replays identical) and every record of all 147 archives was re-derived by this validator ([k]G = Q, S, A2.1 uncharged counts, extracts row by row).
- The replay used the same shared cloud VM type and the same ecbench binary as the sessions: it re-executes operation counts; it is not an independent solver implementation or independent hardware.
- Bootstrap CIs and permutation p-values were recomputed with independent draws: they agree within Monte Carlo error, not bit for bit. Categorical outcomes within about 1-2 Monte Carlo SE of a threshold (H-M V6:98; V6 rho-neg L2 and rho-plain L1 TOST) can differ between draws; the report now says so for H-M.
- Not re-run: the s5 paired-source and transport-then-solve bootstraps and signal flags, per-vertex H-T CIs, per-family R(L), the s4 per-curve p-values (only the attainable minimum), the slug-cluster CI is compared informally; the s9 CIs were recomputed with a 2,000-replicate vertex-cluster bootstrap and compared only as 'lower end above 1'; the mixed model was re-run for ic only.
- Wall time is descriptive in the study; it was checked only as A/A ratios and L2 fractions.
- This validation makes the receipts admissible evidence for the benchmarks row; it does not support an ECDLP claim, a speedup, or promotion.

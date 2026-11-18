# Track B: sweep follow-ups for prime-field ECDLP and index calculus (desk pass, 2026-11-14)

Standing: desk work plus code reading at `origin/main` 0c2cc1b408. Nothing was built or run, no
experiment was started, no Python was used, nothing outside this directory was edited. Every number
below is `[pcc:unverified]`: each is copied from the committed file named beside it or derived here
(arithmetic shown), and no pcc claim id covers any of them. The research-kb server was not queried
in this pass; that step is owed before any task card is cut from this file. Cost figures marked
(estimate) are mine; every other figure carries a path.

Inputs: `research/lead_sweep_20261110/` (SWEEP_REPORT.md, `phase2_index.json`, per-lead PROTOCOL.md and
RESULT.md), the inventory `LEADS_UNTESTED.md` (91 + 61 leads), `research/prime_field_lead_assessment_20261107/`
(atlas survey, dispositions quoted there), and the native studies named per lead.

## 0. Result in one paragraph

114 leads of the inventory have relevance yes or partly (64 of the 91, 50 of the 61 in Appendix B).
Eight are owned by other sessions and are not decided here (list in section 1). The ninth lead,
`ICE-rootfind-pairtable-oracle`, comes from the inventory's ranked table. That leaves **107 leads
decided**: **3 are (b) real test now**, **27 are (c) needs new native code**, **30 are (a) already tested
for real** and **47 are (d) closed at desk**. The inventory understated how much was already measured:
a native study already covers the real claim of at least 25 leads it listed as untested (corrections
in section 2). Only three leads have a measurement that is both unrun and runnable with code that exists
in the tree today, and none of the three can move the sub-sqrt(p) question. The first can add a
measured gain over the IC baseline at 20 bits (success criterion), the second sharpens a denominator
class, and the third reports the algebraic track's distance to matched rho. The (c) list ranks one
item well above the rest: wiring the j = 0 symmetric 3-summand solver into ecbench against `rho.cubic_v2`.

| decision | count |
|:--|--:|
| (a) already tested for real | 30 |
| (b) real test now | 3 |
| (c) needs new native code (or a desk lemma, flagged) | 27 |
| (d) closed at desk | 47 |
| decided | 107 |
| excluded (owned elsewhere) | 8 |

Machine-readable form: [`decisions.json`](decisions.json) (array of `{lead, decision, reason,
evidence_paths, spec_rank}`; `spec_rank` is 1 to 3 for the (b) leads and null otherwise).

## 1. Scope and exclusions

Rule used for scope: the lead's relevance in the inventory is yes or partly. Extension-field and
binary-flavoured leads that the inventory marks "partly" are decided, but flagged "not E(F_p)" in the
reason. Excluded because another session owns them (no decision is made here):

| lead | owner |
|:--|:--|
| `ICP-isogeny-transported-relation-yield` | PR #3961 (isogenous yield) |
| `STR-class-representative-ic-selection` | track A, `research/ic-representative-selection_20261105`, `research/ic_lead_queue_20261114/A_repsel` |
| `EXO-global-lift-height`, `TRF-global-lift-small-height`, `ICP-char0-lifting-xedni-smoothness` | track C (global-lift height family) |
| `ICP-xedni-function-field-sections`, `TRF-function-field-xedni-lift` | track C (function-field xedni) |
| `TRF-jv-nagao-genus2-cover-decomp` | `research/jv_nagao_hilbert_20261112` |

Overlaps that are decided here but cite another session's work as evidence only: `ICP-retained-setup-online`
and `RHO-dp-multitarget-precomputation` cite `research/mt_fieldops_v3_20261111` (the v4 directory is owned
elsewhere and is not used as the basis of any decision); `TRF-cm-canonical-lift-heights` is closed by the
atlas cost-model record and flagged as overlapping track C and PR #3970; `ICP-digit-presentation-solving-degree`
is flagged because PR #3970 owns the CM-lift and Dickson part of the digit probe;
`ICP-full-width-arity-grid` is flagged because its cheaper variant is M1 (`research/leads_chase_20261112`).

## 2. Corrections to the inventory (what a native study already measured)

These change a "still untested" reading to (a) or (d). Each is a path a reviewer can open.

| lead | inventory said | what exists |
|:--|:--|:--|
| `ICP-coppersmith-smallroot-semaev` | joint (4,2) lattice not run | `research/smallroot_followups_20261026/README.md`: the joint (4,2) lattice quantity nu_s/s was computed (>= 2.08 in the demand region for s <= 3, truncated Macaulay bound, "computed, not proved"); unequal bounds and non-convex monomial sets are covered by a derived bound eps < 3/4 against demand 8/9 (exact check, 9,600 cases, 0 violations). `research/coppersmith_s3_theta2_20261023/REPORT.md`: 0 of 960 recoveries at beta >= 0.20. |
| `ICP-coincidence-harvest-structured-base` | break-even ladder missing | `research/prime_coincidence_harvest_20261029/README.md` (native `examples/coincidence_ladder.rs`): every slope d ln kappa / d ln N in [-0.006, +0.008] at 16-32 bits, against break-even exponents 1/3, 1/5, 1/7. |
| `EXO-ml-learned-models` | linear learners at N <= 2^18 only | `research/ml_learnability_control_20261107/RESULTS.md`: 0 SIGNAL in 24 cells, m = 12..24 (Python harness, historical). |
| `ICP-prime-ic-constant-engineering` | prefetch open | `research/prime_ic_leads_20261110/README.md` section 7 row 5: the prefetch design is obsolete because `layout=lines` landed and was measured (invalid_measurement; 12% slower in valid sessions at fp28). |
| `STR-special-j-ic-semaev` | not measured | `research/ic_e222_m221_20261106/RESULTS_prime_fast_j0_sym.md`: symmetric 3-summand j = 0 IC measured; the matched automorphism rho was left unrun (this is the top (c) item). |
| `RHO-automorphism-class-walks`, `STR-automorphism-quotient-rho` | toy, steps only | `research/prime_rho_automorphism_20261015/README.md`: native ecbench j = 0 and j = 1728 sessions, rho.cubic / rho.negation 0.647 [0.543, 0.770] (ceiling 0.577), rho.quartic / rho.negation 0.819 [0.722, 0.929] (ceiling 0.707). |
| `RHO-best-of-candidates-indegree`, `RHO-correlated-walks-shared-sums`, `RHO-cheaper-step-primitives`, `RHO-batched-inversion-shared-pm` | stdlib toy | `research/rho_walk_leads_20261014/README.md`: native Rust harness; best-of-two 1.129 [1.081, 1.184] steps (predicted 1.155) but 1.22x per-step cost on P-256 at 1024 walks; shared index 0.073x; pair table 1.027x. |
| `RHO-prime-gpu-rho` (counted part) | op-count lag only | `research/ecbench_rho_lanes_20261023/REPORT.pdf`: rho.negation_lanes, S a few percent above rho.negation at 8-32 lanes, 24-32 bits. |
| `ICE-rootfind-pairtable-oracle`, `ICE-charged-toy-ic-exponent-vs-rho` | needs pricing and replay | `research/ic_frontier_campaign_20261015/README.md`: no IC arm beats rho at any band weight, no fitted IC exponent below rho's; `research/ic_e222_m221_20261106/CAMPAIGN_SUMMARY.md` (single-target IC/rho 15-19x at 24 bits, 62-78x at 28, 443-528x at 32). |
| `ICE-curve-presentation-invariance`, `STR-isogeny-class-generic-invariance` | toy / timeout | `research/conductor_gap_cm_orbits_20261015/REPORT.md`, `research/isogeny_conductor_gap_ic_20261010/README.md`, `research/fp31_whole_field_census_20261111/README.md`, `research/isogeny-class-rho/README.md`. |
| `EXO-hashed-factor-base-tensor-ecdl` | audit step only | `research/semaev_tensor_treewidth_20261111/README.md`, `research/hashed_ecdl_tensor_audit_20261107/AUDIT.md`, `research/ecdl_tensor_ic_speed_20261030/README.md`. |
| `TRF-weil-restriction-jacobian-class` | toy | `research/weil_restriction_isogeny_census_20261018/REPORT.md` (native census plus theorems). |

### The rho family: what the three medium-tier studies actually settle

The sweep's 14 inconclusive rho rows are `advice-table-variants`, `automorphism-class-walks`,
`best-of-candidates-indegree`, `bsgs-family-memory`, `cheon-aux-acquisition`, `cheon-deployed-applicability`,
`cheon-toy-reproduction`, `correlated-walks-shared-sums`, `ecc2k130-iteration-function` (binary, out of scope),
`generic-sqrt-lower-bound` (theorem, not in the inventory), `kangaroo-interval`, `lhw-modular-filter-price`,
`lhw-polymem-collision-walks` and `prime-gpu-rho` (`phase2_index.json`).

| study | what it measured (verdict as written in the file) | which sweep leads it settles |
|:--|:--|:--|
| `research/prime_medium_tier_rho_20261107/RESULTS.md` | Verdict `inconclusive`: all six audits pass, positive control P4 misses its threshold in every job (J1 rho-plain / rho-neg 1.285 [1.177, 1.404], lower end under the preregistered 1.2). rho.negation M = 1.093 [1.033, 1.156] x floor, "on M alone, the row is `setup_excess`". | denominator for `matched-rho-calibration`; the negation factor of `pool-class-quotient`; partly `negation-map-fruitless-cycles`. |
| `research/prime_medium_rho_p4_confirm_20261108/RESULTS.md` | Verdict `inconclusive`: gate G3 failed, positive control 1.4639, 95% [1.4438, 1.4863], 99% [1.4384, 1.4938] (1.5 outside the 99% interval); every other gate passes (0 mismatched A/A pairs of 2,160). | confirms the P4 control is a detector (lower end 1.44 > 1.2) but not its prior value 1.5. |
| `research/prime_medium_rho_cpc_20261109/RESULTS.md` | Verdict `setup_excess`, every gate passing: M = 1.067 [0.999, 1.134] x floor over 432 runs at log2 r 32.4-49.0; exponent 0.506 [0.483, 0.529]; bsgs.textbook / bsgs.negation 1.489 [1.463, 1.516]; rho.plain / rho.negation 1.443 [1.311, 1.589] (M1). | `LEAD-RHO-matched-rho-calibration` (answered), `negation-map-fruitless-cycles`, `pool-class-quotient` (negation factor), the BSGS ratio in `bsgs-family-memory`. |

These three do not measure kangaroo, advice tables, correlated walks, best-of-two, Cheon, GPU or low-weight
walks. Those rows are decided in the table below on their own evidence (kangaroo is a (b) spec).

## 3. Summary table: every lead in scope and its decision

Decision key: (a) already tested for real, (b) real test now, (c) needs new native code (a desk lemma or desk read is flagged as such), (d) closed at desk. Reason is the first sentence of the reason in `decisions.json`; path is the first evidence path (all paths in the sections below). Sorted by decision (b, c, a, d), then lead.

| lead | dec | one-line reason | path |
|:--|:-:|:--|:--|
| `ICP-pkm-tower-algebraic-oracle` | b | Counted algebraic-track row is fully specified (3 Table B curves x 8 targets x 14 arms); blocked by an owner route decision and an image rebuild, not by missing code | `research/prime_ic_leads_20261110/algebraic_row_spec/spec.json` |
| `ICP-prime-ic-constant-engineering` | b | Only lead with a validated measured gain over the in-repo IC baseline (20-bit N 32->45, 0.818 [0.773,0.866]); N above 45 and the 16/18/22-bit rungs were not run; the prefetch item is obsolete (layout=lines landed and was measured) | `research/prime_base_oracle_tuning_20261029/specs/holdout-P20.json` |
| `RHO-kangaroo-interval` | b | kangaroo.vow_v2 exists in ecbench, but no sealed prime medium-tier bound for it (atlas value 2.203x floor comes from small sizes); the protocol and curves of the medium-tier study can be reused with one method swapped | `src/cryptanalysis/ecbench/methods.rs` |
| `EXO-coordinate-statistic-leakage` | c | coord_spectrum F3-RUN-02 measured x, y, Legendre and Dickson spectra at 2^16 and 2^20 (rule 3, closed); the atlas next step, the base-B digit-character family, is not in the tool (about 60-120 LOC to add) | `research/coord_spectrum_20261110/README.md` |
| `EXO-dl-character-sum-pseudorandom` | c | Exhausted at 2^20-2^22 against a negation-matched null (R 1.040 [0.999,1.092], 0.993 [0.972,1.017]); reopen condition is p beyond 2^30 and the repo has no native DL-sum tool at that size (about 300 LOC plus a 2^28-point table) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-elliptic-code-list-decoding` | c | Desk derivation to check (Guruswami-Sudan agreement threshold sqrt(Bk) > k, so min-weight words are unreachable by decoding); Zhang-Liu full text unread; 0 Rust LOC | `research/prime_field_lead_assessment_20261107/README.md` |
| `EXO-ideas-corpus-algebraic-transforms` | c | Atlas open_blocked: 15 registry rows each need a named constructor or theorem; no code can be written until a constructor exists (0 LOC now, not worth) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-invertible-hint-subset` | c | Desk read of the underlying theorems (not read in the sweep); a nearby measurement (compiled_advice_robdd: ROBDD size 0.994-1.029x random sets) does not test the asymptotic claim | `research/compiled_advice_robdd_20261110/README.md` |
| `EXO-lange-winterhof-interpolation` | c | Desk read of the Lange-Winterhof theorem (text not read in the sweep) to decide whether it closes bounded-degree statistics; 0 LOC | `research/prime_field_lead_assessment_20261107/README.md` |
| `EXO-lossy-object-propagation` | c | Atlas next step is a disjoint projection panel in the probabilistic regime; no native instrument found (about 300 LOC, low expected value, GGM-simulable family) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-low-degree-algebraic-query-bound` | c | Conjecture T >= sqrt(N)/D with a proof program; what remains is a proof, not an experiment (0 Rust LOC) | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICE-gag-contraction-planner` | c | Needs prime-field m=3 and m=4 chain-vs-tree matrix-cell counts below 2^16 with msolve (about 300 LOC); counted-matrix proxy, no ECDLP cost claim; low value | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICE-syzygy-cascade-signatures` | c | Binary-field F5 engine lead; no prime-field instance and no native F5-signature counter (est | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-construction-normal-form` | c | Instrument-hygiene lead (normal-form dedup, estimator-bias check): about 150 LOC in factor_base_sweep; no exponent content | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-digit-presentation-solving-degree` | c | Needs the m=3 source-bound CDCL(T)/F_p solver (about 2-4 kLOC); closed-form d_ff is curve-independent and measured at m=2 s<=7; PR #3970 owns the CM-lift/Dickson part of the digit probe | `research/prime_digit_presentation_20261029/README.md` |
| `ICP-full-width-arity-grid` | c | 63-bit panel needs launcher admission and RSS-monitor repair (about 100-200 LOC) and is expensive; the cheaper certified-LFD m=3 grid is M1 (owned by leads_chase_20261112) | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-gfp5-oa-sdhp-jv` | c | E(F_{p^5}), not E(F_p); needs an n=5 Joux-Vitse oracle (about 1.5 kLOC); skip for the prime-field queue | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-gfp5-odd-char-cover` | c | E(F_{p^5}); odd-characteristic cover genus computation, no existing tool (est | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-hecke-cm-class-point-rank-completion` | c | The repo has only a generic shift router (hecke_target_router.rs), not a Hecke relation generator; the cited D79 records were not found; marginal Hecke rank over pair-index reuse is 0, 10, 19, 0 on four cells (about 400 LOC, low value) | `examples/hecke_target_router.rs` |
| `ICP-lattice-ic-relation-construction` | c | No theory and no pipeline (expensive new build); next step is fixtures F43/F97/F101 plus an F25 negative control; Hecke rank closure was reproduced by doubling rows alone | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-prym-witness-relation-diversity` | c | Atlas: run nothing until the owner-cited 4.59M-attempt records are located and a mechanism for attaching a prescribed target exists | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-shared-coefficient-membership` | c | No measurement exists; needs chart-correspondence proof and an m=3 protocol with a CDCL(T)/F_p solver (medium-large); m=2 cells can never be exponent evidence | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-split-graph-secant-presentation` | c | Needs a native triangular-factorization / FGLM root extractor and a three-cell prospective panel (about 700 LOC); degree d+1 vs 2d+1 is a theorem but no complete-cost gain over the dense presentation yet | `research/prime_field_lead_assessment_20261107/README.md` |
| `RHO-advice-table-variants` | c | Atlas records a T/2 reselected table at -0.064 [-0.080,-0.049] (a loss) at 2^24; ecbench rho.preprocessing has no reselection option (about 150 LOC to add); low value | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `STR-class-group-ecdlp-bridge` | c | Needs the simulation lemma H-CLGP-63f471 and the Lean obligation cm_closed_walk_scalar_action (desk, about 100-200 Lean lines, 0 Rust); would close the lead at exponent 1/2 | `research/leads_20261112/README.md` |
| `STR-level-structure-torsor` | c | Expected closure by size (X_1(N) genus about N^2/24 with N near p); the quick-kill has no record yet (desk arithmetic, 0 Rust LOC) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-kernel-of-reduction-lemma` | c | Desk lemma expected to collapse the revisit clauses of five lead families into one obstruction; 0 Rust LOC | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-odd-char-descent-smith-quartic` | c | Atlas open_blocked: implement Smith's Props | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-odd-prime-ghs-cover-floor` | c | Approved n=7 EXP-PEXT-212d5c plan with Lean finite lemmas not run; extension-field; no existing tool found | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-small-genus-cover-general` | c | Needs a non-Kummer cover family (Hess Artin-Schreier analogue) constructed first; genus floors 9/49/41 at k=6/10/14 recorded (about 500 LOC); extension-field | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-hashed-factor-base-tensor-ecdl` | a | Real native tests: reduction audit found no correctness error; treewidth lower bound exceeds b/2 by 4-5x at b=8..40; native tensor IC speed panels exist (ic.tensor_contraction_v1) | `research/semaev_tensor_treewidth_20261111/README.md` |
| `EXO-minors-guess-and-determine` | a | Real six-arm 20-40 bit prime-field panel vs matched negation rho: every arm costs more than rho, exponent upper bounds 0.815-1.065; lead CLOSED for the reconstruction implemented | `research/minors_guess_determine_20261107/RESULTS.md` |
| `EXO-ml-learned-models` | a | Learnability control with planted leak: 0 SIGNAL in 24 cells at m=12..24 (22 NULL, 2 UNDERPOWERED) | `research/ml_learnability_control_20261107/RESULTS.md` |
| `EXO-padic-jet-canonical-lift` | a | Real study: torsion-section p-adic digits are generic on Kummer bases (42 cells, 17 curves); jet exit adds only free F_p unknowns; Teichmuller contrast at chance | `research/prime_ic_leads_20261110/jet_exit/README.md` |
| `EXO-weak-class-instrument-calibration` | a | Measured: structure instruments do not detect anomalous or small-embedding classes (36 per class, p<=2^14); coord_spectrum carries an anomalous positive control that passes (P2) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICE-charged-toy-ic-exponent-vs-rho` | a | Same campaign: no IC arm has a fitted cost exponent below rho's; bitop ladder 16-40 bits ic-pair-rule exponent 0.724 [0.707,0.736] (33-40 bit) vs rho-neg 0.548 | `research/ic_frontier_campaign_20261015/README.md` |
| `ICE-curve-presentation-invariance` | a | Real prime-field conductor-gap IC studies: floor/crater query ratio 0.878 [0.764,1.009], CPU 0.980 [0.866,1.111]; IC floor/crater 0.975 and 1.127 (no separation); whole-field census finds only Lenstra divisibility bias | `research/conductor_gap_cm_orbits_20261015/REPORT.md` |
| `ICE-gpu-rootfind-oracle` | a | Real CUDA root-finding decomposition runs exist (40/56/61-bit validations) and a GPU end-to-end IC study; the sweep toy had no GPU | `research/gpu_rootfind_decomposition_20261012` |
| `ICE-rootfind-pairtable-oracle` | a | Prime pair-table and root-finding IC measured against matched rho: no IC arm beats rho at any band weight, none has a fitted exponent below rho's; single-target IC/rho 15-19x (24 bits), 62-78x (28), 443-528x (32) | `research/ic_frontier_campaign_20261015/README.md` |
| `ICP-coincidence-harvest-structured-base` | a | Native break-even ladder 16-32 bits: every slope d ln kappa / d ln N in [-0.006,+0.008] against break-even exponents 1/3, 1/5, 1/7; planted controls recovered (SUB_BREAK_EVEN_AT_SCALE) | `research/prime_coincidence_harvest_20261029/README.md` |
| `ICP-residual-partial-relations` | a | Measured: 39 sessions at 20-44 bits, slope -0.025 [-0.034,-0.017] same as rho/BSGS/kangaroo (a birthday search); S 2.28-11.89 at 36 bits vs 1.29 rho-neg | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-retained-setup-online` | a | Closed_dominated: the 13.9x / 18.5x claims were against empty-table rho; preprocessing rho at equal advice is 53-317x faster online; multi-target IC measured in counted ops (mt_fieldops, owned elsewhere; cited as evidence only) | `research/20261016-n83-precomputed-rho-control` |
| `RHO-automorphism-class-walks` | a | Native ecbench prime j=0 and j=1728 sessions: rho.cubic/rho.negation 0.647 [0.543,0.770] (predicted 0.577), rho.quartic/rho.negation 0.819 [0.722,0.929] (predicted 0.707); ratio to the sqrt/Aut/ ceiling 1.0043 and 1.0442 at p=100057 | `research/prime_rho_automorphism_20261015/README.md` |
| `RHO-best-of-candidates-indegree` | a | Native measurement: best of P+/-R steps ratio 1.129 [1.081,1.184] (predicted 1.155), but 1.22x per-step cost on P-256 at 1024 walks is a net loss; the W=16-64 shared-inversion case needs the GPU kernel or a counted model with shared inversion (about 60-100 LOC) | `research/rho_walk_leads_20261014/README.md` |
| `RHO-cheon-deployed-applicability` | a | Priced Cheon arms measured natively (cheon.pminus1_comb, cheon.pplus1_comb) and the deployed-parameter shape census exists; recorded reading: values need d-degree supply, not a vulnerability | `research/cheon_ecbench_wide_20261106/REPORT.md` |
| `RHO-cheon-toy-reproduction` | a | Real ecbench Cheon comb arms with memory charged on prime curves (96/96 runs equal the medium-study runs); toy reproduction superseded | `research/cheon_ecbench_wide_20261106/REPORT.md` |
| `RHO-correlated-walks-shared-sums` | a | Native measurement: shared index sequence 0.073x [0.070,0.076] (lower bound, 176/1000 censored); precomputed pair table 1.027x [0.982,1.073] | `research/rho_walk_leads_20261014/README.md` |
| `RHO-dp-multitarget-precomputation` | a | Real native evidence: preprocessing rho at equal advice is 50-500x faster online than IC; retention and DP multi-target rho arms in counted field operations (mt_fieldops v3/v4, the latter owned elsewhere; cited as evidence only) | `research/20261016-n83-precomputed-rho-control` |
| `RHO-matched-rho-calibration` | a | Sealed prime medium-tier bound: rho.negation M = 1.067 [0.999,1.134] x floor over 432 runs at log2 r 32.4-49.0, setup_excess, exponent 0.506 [0.483,0.529]; all gates pass | `research/prime_medium_rho_cpc_20261109/RESULTS.md` |
| `RHO-native-step-cost` | a | Native ecbench rho versions v3-v6 and chained are sealed in counted units; recorded instructions per step 1,196 to 718, floor about 6-7 field multiplications; PMU timing needs a PMU host | `research/ecbench_rho_v5_20261107` |
| `RHO-negation-map-fruitless-cycles` | a | Real prime-curve measurement: rho.negation 1.067 [0.999,1.134] x floor at 32-49 bits with cycle escape; rho_walk_leads lead 4 measured the rule variants | `research/prime_medium_rho_cpc_20261109/RESULTS.md` |
| `RHO-pool-class-quotient` | a | Negation quotient measured on prime curves: rho.plain/rho.negation S ratio 1.443 [1.311,1.589] (M1) and 1.285 [1.177,1.404] (J1); atlas B3 negation walk 0.700 [0.673,0.728] of v2 steps | `research/prime_medium_rho_cpc_20261109/RESULTS.md` |
| `RHO-prime-gpu-rho` | a | Counted-op claim measured: rho.negation_lanes S is a few percent above rho.negation at 8-32 lanes, 24-32 bits; GPU throughput admission needs a GPU host (ecbench gpu-stage exists), no new code | `research/ecbench_rho_lanes_20261023/REPORT.pdf` |
| `STR-automorphism-quotient-rho` | a | Same native j=0 / j=1728 sessions: 0.647 [0.543,0.770] and 0.819 [0.722,0.929] of negation rho (sqrt/Aut/ ceiling 0.577, 0.707); atlas closed_known | `research/prime_rho_automorphism_20261015/README.md` |
| `STR-isogeny-class-generic-invariance` | a | Real censuses: 185,363 isogeny classes of F_(2^31-1) with only Lenstra divisibility bias; 38 curves x 100 seeds KW p 0.38; atlas 403,168-run census, floor/crater rho 1.004 [0.990,1.016] | `research/fp31_whole_field_census_20261111/README.md` |
| `STR-special-j-ic-semaev` | a | Native j=0 symmetric 3-summand IC measured: 2.0-2.4x faster than plain 3-summand IC after validator correction, but 3x-23x behind negation rho and growing; the matched automorphism rho is not wired (see c-list) | `research/ic_e222_m221_20261106/RESULTS_prime_fast_j0_sym.md` |
| `TRF-jv-sieve-fp6-weak-class` | a | Real follow-ups (extension-field weak class): 22-24.5% of curves admitted; implemented walk reaches a weak curve from 0% at p=1511, 1823; walk charged 0.0155x rho at M=512 | `research/jv_weak_coverage_20261111/README.md` |
| `TRF-padic-canonical-section` | a | Theorem plus real study: the only homomorphic section is prime-to-p torsion (killed by the formal logarithm); jet_exit digits generic on 42 cells | `research/prime_ic_leads_20261110/jet_exit/README.md` |
| `TRF-teske-trapdoor-fp6-weak-class` | a | The public isogeny walk to the weak class is the quantity measured in jv_walk_charge (extension field, not E(F_p)) | `research/jv_walk_charge_20261111/README.md` |
| `TRF-weil-restriction-jacobian-class` | a | Native census and theorems exist: a prime-order E/F_(q^3) has no eigenvalue 1 on the relevant kernel (Corollary), recorded run results/run-20261018T1605Z | `research/weil_restriction_isogeny_census_20261018/REPORT.md` |
| `EXO-augmented-oracle-ggm` | d | Atlas disposition closed_obstructed (theorem): label-valued oracles (x, y, halving, endomorphism image = public scalar) have explicit GGM simulators; the sweep falsification was a toy artifact | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-cryptoscale-candidate-screen` | d | Atlas exhausted_at_scale: 13 screening batches FINAL_NO_CANDIDATE; revisit only with an externally sourced candidate and a constructor | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-curve-model-representation` | d | Recorded measurement: summation degree d_reg = 2 is model-invariant; Kummer and u = x/y classes tie at D = 2^((m-1)^2); no coordinate with degree below 2 is known | `research/prime_field_lead_assessment_20261107/README.md` |
| `EXO-elliptic-net-eds` | d | Recorded measurement: elliptic-net k-recovery exponent 1.97-2.80 against BSGS 1.17-1.32; the GGM-simulable locator carries no nonlinear gain | `research/prime_field_lead_assessment_20261107/README.md` |
| `EXO-four-list-ktree-join` | d | Closed on paper (a Wagner filter needs a subgroup-like set) and measured generic: k-sum k=5 at 40 bits is 9.7x rho-neg | `research/prime_ic_leads_20261110/README.md` |
| `EXO-hnp-advice-leakage` | d | The leak ([alpha^i]G auxiliary inputs) is assumed, not obtainable from public data; acquisition is closed_obstructed | `research/cheon_aux_leads_20261112/README.md` |
| `EXO-ksum-conjecture-s5-oracle` | d | Conditional obstruction under the k-SUM conjecture, with measured k-sum tables 9.7x rho-neg at k=5, 40 bits; remaining work is writing the reduction | `research/prime_ksum_table_20261028` |
| `EXO-low-degree-query-lower-bound` | d | H1 withdrawn (false under a full-rank condition, exact check 39/39 cells at p=251..2039); H1-prime derived in the cited file with Delta in place of D | `research/prime_field_lead_assessment_20261107/README.md` |
| `EXO-split-jacobian-projection` | d | Proof (KN-FIND-61347e): Hasse-Weil inflation outweighs genus-2 index calculus | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `EXO-torsor-deck-period` | d | Listed among rows closed by theorem or measured invariance in the prime-field assessment; no open test named | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICE-decomposition-probability-audit` | d | Per-trial relation R = aG + bQ is uniform in the group, so decomposition probability is target-independent by random self-reduction and its mean is fixed by conservation; learned predictors are null (ml_learnability_control) | `research/ml_learnability_control_20261107/RESULTS.md` |
| `ICE-generic-oracle-arity-cost-floor` | d | Closed_obstructed (cost model): zero in-domain sub-rho cells for enumeration/MITM oracles; prime analogue is the Corrigan-Gibbs-Kogan S*T^2 >~ N and Shoup T >~ sqrt(N) theorem | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICE-semaev-chain-cost-model` | d | Closed by cost model: optimal chain length t* = m* at every n in [250,600], charged cost topology-blind (EV-SEMBIN-1d36a3); tensor treewidth census agrees that Semaev buys constants | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICE-semaev-cover-monodromy` | d | Proof: generic monodromy (Z/2)^(m-2) x/ S_(m-1), uniform in the curve for p>3; cycle-type census is a no-go as a relation-rate probe at every m>=3 | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICE-semaev-yield-model-coset-typing` | d | Mean yield is C(B+m-1,m)/N for every base of size B (conservation); six geometries incl | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-algebraic-membership-factor-base` | d | Theorem-kind (conservation KN-FIND-007 plus the bounded-degree theorem); six geometries measured within 0.0003 bits of a random base at m=2, N up to 2^26; reopens only on KN-OPEN-020 | `research/prime_field_lead_assessment_20261107/README.md` |
| `ICP-coppersmith-smallroot-semaev` | d | Derived reach bound <= 0.3536 per block vs demand 8/9; derived eps < 3/4 for any lattice from (p,f)^t on S_3 and for every monomial set; joint (4,2) lattice computed nu_s/s >= 2.08; 0 of 960 recoveries at beta >= 0.20 | `research/smallroot_optimal_reach_20261026/README.md` |
| `ICP-extension-field-embedding` | d | Cost model: Gaudry p^(2-2/n) >= p; measured genus-2 IC after chained transfer is 9.8x-42.9x matched rho (about p^1.0) | `research/chained_prime_transfer_20261111/README.md` |
| `ICP-gfp5-rho-reference-audit` | d | Atlas closed_known; extension-field rho reference, not E(F_p) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-gfp5-symmetrized-gaudry` | d | Atlas closed_dominated: m=5 quotient solved at three primes, c_cyc 2^46.30-2^46.63 vs closure bar 2^46; extension field | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-representation-subset-sum` | d | Closed_obstructed (theorem): generic, Shoup T >~ sqrt(N) and S*T^2 >~ N; the filter has no non-generic handle | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-self-pairing-prime-order` | d | Closed_obstructed (theorem): self-pairings are 1-dimensional, trivial for embedding degree > 1 | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `ICP-xline-symmetry-special-j` | d | Closed on paper: the j=0 omega-conjugate equation adds nothing; measured j=0 symmetric IC gains a constant only | `research/prime_ic_leads_20261110/README.md` |
| `RHO-batched-inversion-shared-pm` | d | Exact operation-count identity (2M+1S+3(W-1)/W M+I/W per step) and recorded P-256 timing (179 ns/step at 1024 walks); ecbench rho.negation_lanes counts it | `research/ecbench_rho_lanes_20261023/REPORT.pdf` |
| `RHO-bsgs-family-memory` | d | Theorem (Shoup T >~ sqrt(N), S*T^2 >~ N) plus measured claw.pair_table 2.20x strong rho and bsgs.negation exponent 0.529; positive control bsgs.textbook/bsgs.negation 1.489 [1.463,1.516] in the prime medium tier | `research/prime_medium_rho_cpc_20261109/RESULTS.md` |
| `RHO-cheaper-step-primitives` | d | Exact operation counts (8M+3S Jacobian mixed add vs 2M+1S+3M+I/W batched affine) and the theorem that a tag of an unnormalised representation is not a function of the point, so walks do not merge | `research/rho_walk_leads_20261014/README.md` |
| `RHO-cheon-aux-acquisition` | d | Atlas closed_obstructed (theorem): auxiliary inputs [x^d]P cannot be acquired below the cost they save; EXP-AUXIN-25a29a integer-arithmetic run succeeded | `research/cheon_aux_leads_20261112/README.md` |
| `RHO-lhw-modular-filter-price` | d | Exact residue-collision count on integers; the object is low-Hamming-weight DLP in Z_P^*, not E(F_p), so there is nothing to test on a curve | `research/lead_sweep_20261110/LEAD-RHO-lhw-modular-filter-price/PROTOCOL.md` |
| `RHO-lhw-polymem-collision-walks` | d | Low-weight DLP in Z_P^* with O(sqrt(S)) collision walks is the standard split-and-collide bound, not an elliptic-curve claim | `research/lead_sweep_20261110/LEAD-RHO-lhw-polymem-collision-walks/PROTOCOL.md` |
| `RHO-lhw-preprocessing-frontier` | d | The product law eps = p_split*S*T/(L*R) is an exact identity for the Coppersmith split; Z_P^* object, not E(F_p) | `research/lead_sweep_20261110/LEAD-RHO-lhw-preprocessing-frontier/PROTOCOL.md` |
| `RHO-non-automorphism-equivalence` | d | Arithmetic: canonicalisation needs h-1 generic scalar multiplications (about 1.5 log2 N group operations each), above the sqrt(h) saving for every h > 2; break-even image cost < 1/(sqrt(h)+1) step | `research/lead_sweep_20261110/LEAD-RHO-non-automorphism-equivalence/PROTOCOL.md` |
| `STR-endo-scalar-action-generic` | d | Theorem: End(G) = Z/N on a prime-order group, GGM-simulable; the sweep falsification is an undiagnosed toy artifact | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `STR-isogeny-transport-preprocessing` | d | Proof (closed_obstructed): table transport reduces to single-curve preprocessing; measured loss 1.5x-21x at odd l 3..31 (EV-ENDO-10109d N4) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `STR-jmv-random-self-reduction` | d | Literature verdict plus a derived obstruction: random self-reduction across an isogeny class shows hardness equivalence, not a cost reduction | `research/jmv_self_reducibility_20261008/README.md` |
| `STR-p192-class-cm-relations` | d | Recorded: 103/103 CM relations are scalar (closed_obstructed); P-192 isogeny tower studies carry no relation | `research/prime_field_lead_assessment_20261107/README.md` |
| `STR-safecurves-parameter-audit` | d | An exact audit of public parameters, not a research hypothesis; the facts are curve-database lookups (ecdb) and docs/curves; nothing to measure | `examples/safecurves_orbit_census.rs` |
| `STR-secp256k1-cm-class-audit` | d | Recorded: secp256k1 CM structure gives only the sqrt(6) automorphism ceiling (j=0, h=1); the sweep crash was an instrument failure | `research/prime_field_lead_assessment_20261107/README.md` |
| `STR-small-cm-disc-nonunit-endo` | d | Elementary orbit bound h >= log N / log n; net factor <= sqrt(h)/kappa needs canonical representative cost o(h), no such algorithm; native CM-orbit IC campaign shows no floor/crater separation | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-anomalous-padic-lift` | d | Closed_known: Smart's attack applies to anomalous curves only (certificate p=771853, C_special 157 vs matched rho 779) | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-cm-canonical-lift-heights` | d | Atlas cost_model closure: twelve audited L9 mechanisms, Pr[Q in red(S)] <= /S//N | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-curve-parameter-transfer-audits` | d | Atlas closed_known; twist evidence recorded | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-mov-small-embedding-degree` | d | Closed_known: MOV/Frey-Ruck apply only at small embedding degree; the sweep falsification (4 hits vs 3.27 expected) is a threshold artifact | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-one-point-self-pairing` | d | Theorem (closed_obstructed): self-pairings of a single point are trivial or degenerate | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-pairing-target-field-recost` | d | No ECDLP content for non-pairing curves (atlas); the BLS12-381 exTNFS recost is a cost-model statement outside prime-field E(F_p) ECDLP | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-same-field-genus-g-embedding` | d | Theorem-kind: #Jac = N*Theta(N^(g-1)), index calculus O~(q^(2-2/g)) >= q; chained prime transfer measured 9.8x-42.9x matched rho | `research/chained_prime_transfer_20261111/README.md` |
| `TRF-second-torsion-direction` | d | Theorem (Balasubramanian-Koblitz); the sweep falsification is a toy-size coincidence | `research/prime_field_lead_assessment_20261107/atlas_survey.md` |
| `TRF-teske-secret-isogeny-trapdoor` | d | Tate: isogenous curves share the order, so no trapdoor on F_p; measured class homogeneity (census, isogeny-class-rho) | `research/fp31_whole_field_census_20261111/README.md` |

## 4. The (b) specs, ranked by information per cost

**Update 2026-11-14:** specs 1 and 2 were made freeze-ready and superseded by `B1/` and `B2/` (PROTOCOL.md, spec JSON, PLAN.txt, SMOKE.txt). Where they differ (B1: 96 targets x 6 rounds, positive-control direction `--a inc-n32 --b n45`; B2: 24 targets per curve plus a separate PC job), `B1/` and `B2/` govern. The draft files in `specs/` are retained only as history.

Only three leads have a runnable, unrun, real measurement with code that exists in the tree today. They
are ranked below by information per CPU-hour (estimates). None is an attack and none can change the
sub-sqrt(p) question; each reports a distance or a baseline. Draft spec files are in
[`specs/`](specs/) (built with `jq` from frozen specs of earlier studies; `ecbench plan` has not been
run because nothing was built in this pass, so each file is a draft until `plan` accepts it).

Common launch requirements (AGENTS.md; none done here): a Conductor task and scope check, a filled
`.resume/RESUME.md` before any run, a `bench.record/v1` record with raw data in S3 and the regenerated
progress report in the same PR, and the isolab-eks route with replay on a second host for anything that
is to reach a leaderboard. Seeds below are the first 15 hex digits of `sha256("<label>")` modulo 2^31
(derived, fixed before any data; this differs from the "first 8 bytes" convention of the algebraic-row
spec only in using 15 digits to avoid integer overflow in the shell).

### Spec 1 (rank 1): `ICP-prime-ic-constant-engineering`, factor-base size above 45 at 20 bits

- **id and real claim.** `LEAD-ICP-prime-ic-constant-engineering`. The prime-field factor-base IC with the
  pair-table oracle `mitm:negation_folded=1` has a cost optimum in the base size N. At 20 bits the
  validated holdout moved N from 32 to 45 and read S ratio 0.818 [0.773, 0.866] at w = 0
  (`research/prime_base_oracle_tuning_20261029/README.md`, `VALIDATION.json`). The open question named by
  the lead is whether N above 45 moves the 20-bit baseline again. The prefetch item is obsolete (section 2).
- **H1.** At least one N in {56, 64, 80, 91} has a paired S ratio against N = 45 whose 95% cluster-bootstrap
  upper end is below 1 at each of w = 0, 0.1 and 1, on both fresh curves.
  **H0.** For every N in the grid, the ratio is at least 1 at some weight, or its interval contains 1.
- **Prediction (number and source).** H0. On the train curve `icv1-fp20-tm1195-26505147` (8 targets, one round),
  S(w = 0 / 0.1 / 1) is 10.036 / 11.243 / 22.103 at N = 45 and 10.519 / 11.413 / 19.464 at N = 64
  (`research/prime_base_oracle_tuning_20261029/select/band-train-P20.md`), so N = 64 / N = 45 is
  1.048 / 1.015 / 0.881 (derived here). The order flips across the band, which under section 5a leaves the pair unranked.
  Second source: the three selected sizes 45, 115, 288 at 20, 24, 28 bits grow by 2.56 and 2.50 per
  4 bits against 2^(4/3) = 2.52 for N proportional to r^(1/3) (derived from the README decision table),
  which places the optimum at 45 for this curve.
- **Falsifier (kills H1).** On either curve, every N in the grid has at least one weight at which the ratio to N = 45
  is at least 1 or its interval contains 1. **Kills H0 (H1 outcome):** one N with upper ends below 1 at all three
  weights on both curves, with the positive control in range and the A/A arms exact.
- **Controls.** Negative control (null arm): `n45-aa` against `n45` and `rho-neg-aa` against `rho-neg`, identical
  methods, paired counts must be identical (ratio exactly 1.0000, as in the frozen P7 of the medium-tier study).
  Positive control (instrument must see an effect): `inc-n32` against `n45`, must read inside [0.73, 0.90] at
  w = 0 (the prior holdout 0.818 [0.773, 0.866] widened for a fresh curve); outside means the curve or instrument is
  off and the run is void. A/A: the two arms above, plus `ecbench verify --replay-all`.
- **Instrument and exact command lines.** `src/bin/ecbench.rs`, method `ic.pipeline` (factor base
  `prime-abscissa:size=N`, oracle `mitm:negation_folded=1`, `max_trials 1410236` as in the frozen holdout spec),
  matched `rho.negation`. Spec: `specs/n-ext-p20.json` (derived with `jq` from `.../specs/holdout-P20.json`).
  ```
  cargo build --release --bin ecbench
  target/release/ecbench plan   --spec research/ic_lead_queue_20261114/B_sweep_followups/specs/n-ext-p20.json
  target/release/ecbench run    --wait --cpus inherit --spec research/ic_lead_queue_20261114/B_sweep_followups/specs/n-ext-p20.json --out runs/n-ext-p20
  target/release/ecbench verify --dir runs/n-ext-p20 --replay-all --exit-code
  for A in n56 n64 n80 n91 inc-n32; do for W in 0 0.1 1; do
    target/release/ecbench compare --dir runs/n-ext-p20 --a n45 --b $A --weight $W --save; done; done
  target/release/ecbench band --dir runs/n-ext-p20 --weights 0,0.1,1 --ranks
  ```
- **Sizes, seeds, runs.** Two fresh 20-bit prime-order curves (`prime_search` seeds 1862886355 and 2094636325, labels
  `ic-lead-queue-B-N20ext|curve_seed` and `|curve_seed_2`), 48 public targets each (target seed 1098106079), 3 rounds,
  1 warmup, alternating order, measurement seed 1082707149. 9 arms: 2 curves x 48 x 3 x 9 = 2,592 measured executions.
- **Matched rho and band.** Arm `rho-neg` (`rho.negation`) on the same targets. Report S at w = 0, 0.1, 1 (section 5a)
  for every arm including rho; rank two arms only where the order holds at all three weights. The distance to rho is
  reported as S(IC) / S(rho) (the validated holdout had 9.47 to 7.75 at w = 0 for N 32 to 45; a new row is read
  against that).
- **Cost (estimate).** Under 0.1 CPU-hours: recorded median wall per run at 20 bits is 1.7 to 2.5 ms
  (`research/prime_base_oracle_tuning_20261029/README.md`), 2,592 runs. Local 4-core host: minutes. isolab-eks:
  about 1 hour wall, dominated by pod admission and the L2 A/A requirement, 2 vCPU.
- **What each outcome changes.** H1 outcome: a second measured gain over the 20-bit IC baseline, eligible for
  the validation gate (replay certificate, validator report, matched rho with the band) and then a leaderboard
  row. H0 outcome: nothing changes; the N = 45 baseline is bracketed from above, and the flip at w = 1 is
  recorded as relabelling. Neither moves the atlas disposition (`open_next_step`, constant factor only).

### Spec 2 (rank 2): `RHO-kangaroo-interval`, full-group kangaroo against matched rho at the prime medium tier

- **id and real claim.** `LEAD-RHO-kangaroo-interval`. On a prime-order prime-field curve, a full-group
  two-kangaroo solver with distinguished points costs more group operations than matched r-adding rho; its
  advantage is confined to short intervals.
- **H1.** The pooled paired ratio R = S(`kangaroo.vow_v2`) / S(`rho.plain`) has a 95% interval inside [1.3, 2.4]
  at log2 r 32.4 to 40.8. **H0.** R is at most 1.0 (kangaroo is not dominated by rho on the full group).
- **Prediction (number and source).** R about 1.5, range 1.4 to 1.65. Sources: ecbench declares the
  expected S of `kangaroo.vow` as 2.0 (`src/cryptanalysis/ecbench/methods.rs`, `expected_s`) and rho.plain's
  floor is sqrt(pi/2) = 1.2533, ratio 1.60 (derived); the atlas records kangaroo at 2.203 x floor
  (`research/prime_field_lead_assessment_20261107/atlas_survey.md`) and the medium-tier study records rho.plain at
  1.487 [1.392, 1.577] x the negation floor (`research/prime_medium_rho_cpc_20261109/RESULTS.md`), so 2.203 / 1.487
  = 1.48 (derived). Against `rho.negation` the same sources give about 2.06 (2.203 / 1.067).
- **Falsifier (kills H1).** The pooled 95% interval for R reaches below 1.3 or above 2.4. If its lower end is at or
  below 1.0 the claim "dominated" is withdrawn and the atlas row `closed_dominated` is reopened.
- **Controls.** Negative control (null arm): `kang-aa` against `kang` and `rho-neg-aa` against `rho-neg`, paired
  counts identical. Positive control, in session: `rho-plain` against `rho-neg` must read inside [1.31, 1.59] (the
  M1 job of the medium-tier study read 1.443 [1.311, 1.589]); second positive control, a separate job: the frozen
  `research/prime_medium_rho_cpc_20261109/specs/PC.json` re-run unchanged must read `bsgs.textbook / bsgs.negation`
  inside [1.463, 1.516]. A/A: the two A/A arms.
- **Instrument and exact command lines.** `src/bin/ecbench.rs`, methods `kangaroo.vow_v2`, `rho.plain`,
  `rho.negation`. Spec `specs/kangaroo-medium.json` (derived with `jq` from `.../specs/M1.json`: same six curves,
  new target and measurement seeds).
  ```
  cargo build --release --bin ecbench
  target/release/ecbench plan   --spec research/ic_lead_queue_20261114/B_sweep_followups/specs/kangaroo-medium.json
  target/release/ecbench run    --wait --cpus inherit --spec research/ic_lead_queue_20261114/B_sweep_followups/specs/kangaroo-medium.json --out runs/kang-medium
  target/release/ecbench verify --dir runs/kang-medium --replay-all --exit-code
  target/release/ecbench compare --dir runs/kang-medium --a rho-plain --b kang --save
  target/release/ecbench compare --dir runs/kang-medium --a rho-neg   --b kang --save
  target/release/ecbench band    --dir runs/kang-medium --weights 0,0.1,1 --ranks
  target/release/ecbench run    --wait --cpus inherit --spec research/prime_medium_rho_cpc_20261109/specs/PC.json --out runs/pc-control
  ```
- **Sizes, seeds, runs.** The six curves of `M1.json` (log2 r about 32.4, 32.7, 36.0, 36.4, 40.4, 40.8), 8 planted
  targets per curve (target seed 1453176143), 6 rounds, 1 warmup, alternating, measurement seed 1085117694. 5 arms:
  6 x 8 x 6 x 5 = 1,440 measured executions, plus the PC job.
- **Matched rho and band.** `rho-neg` is the matched reference, `rho-plain` the lead's own comparator; report
  S at w = 0, 0.1, 1 for all arms (`ecbench band`), rank only where the order holds across the band. Kangaroo's
  restart multiplications and rho.negation's canonicalisations are unpriced native counters and are priced at
  0, 0.1, 1 alike.
- **Cost (estimate).** About 0.5 CPU-hours: about 1.4 million steps per run at the top size, roughly a second for
  rho and 2 to 3 seconds for kangaroo (estimate), over 16 workloads at the two top sizes plus 32 at the smaller
  four, 6 rounds. Local 4-core host: about 15 minutes. isolab-eks: 1 to 2 hours wall (2 vCPU, L2 admission).
- **What each outcome changes.** R inside [1.3, 2.4]: the atlas row stays `closed_dominated`, now with a sealed
  medium-tier number, and the rho-family frontier gains one bound (`ecbench bound fit`); nothing changes for the IC
  baseline. R at or below 1: reopens the row and would be the first full-group method below matched rho on a prime
  curve; it would need the full validation gate before it is described as anything.

### Spec 3 (rank 3): `ICP-pkm-tower-algebraic-oracle`, the first counted algebraic-track row

- **id and real claim.** `LEAD-ICP-pkm-tower-algebraic-oracle`. What does the native prime-field algebraic [pcc:unverified]
  oracle (`pkm-semaev`: S_(m+1) with the isogeny-tower membership equations, F4 over the tower quotient ring) cost,
  whole pipeline, in S = priced GAE / sqrt(r) across the section 5a band, against matched `rho.negation` and the
  generic incumbent on the same targets? This reports the track's distance; it is not an exponent claim.
- **H1 (as pre-stated in the spec, `research/prime_ic_leads_20261110/algebraic_row_spec/README.md` section 7).**
  The best `iso-*` arm costs at least 1e4 x matched rho at w = 0.1 on every curve. **H0.** On some curve the best
  `iso-*` arm is below 1e4 x rho (the F4 cost model of the spec is wrong).
- **Prediction (number and source).** At least 1e4 x rho. Data seen: the k = 20 end-to-end row at 1.9e8 / 1.84
  at w = 0.1 (measured, quoted in the same README section 7, from
  `research/curve25519_algebraic_ladder_20261102/sessions/e2e-iso-t6-k20`). Secondary: `iso-t6 / inc` within
  [2, 10] at w = 0 on fp20 (calibration 5.4, 42.4 / 7.92, same source).
- **Falsifier (kills H1).** A curve whose best `iso-*` arm is below 1e4 x rho at w = 0.1. Any ratio below 1
  is a parity candidate that must pass the full validation gate before it is described as anything.
- **Controls.** A/A: `rho-neg-aa` and `iso-t5-m2-aa`. Negative control: the generic-track arms `inc-n20`, `inc-n32`,
  `inc-n51` must show the recorded Table B behaviour (the spec's H3 band). Grid cross-check on every t
  (`*-grid`: the counted V^2 grid, stream-identity check C1 and the cost F4 must beat C4) and refutation check
  (`*-xchk`, counter `pkm_crosscheck_disagreements` must be 0). **Positive control: none exists.** ecbench cannot
  plant a decomposition into `ic.pipeline` and there is no `oracle-check` for pkm-semaev (BLOCKERS item 6); this is
  a limitation to carry into the report, and a planted-decomposition check (about 150 LOC, a (c) item) is the
  prerequisite for reading any null.
- **Instrument and exact command lines.** `src/cryptanalysis/ic_framework/pkm_semaev.rs` through `ecbench`
  `ic.pipeline` with `prime-isogeny:t=T` and `pkm-semaev:m=2,mode=full`. Spec
  `research/prime_ic_leads_20261110/algebraic_row_spec/spec.json`; pod command
  `research/prime_ic_leads_20261110/algebraic_row_spec/job-algebraic-row.sh run SPEC SESSION`, replay on a second
  host class with `job-algebraic-row.sh replay S3_PREFIX SESSION`; local plan check
  `target/release/ecbench plan --spec .../algebraic_row_spec/spec.json`. The m = 3 chain cell is
  `spec-m3.json`, gated on a 2-target preflight below 3e11 F4 multiply-adds per target.
- **Sizes, seeds, runs.** Table B curves fp16, fp18, fp20 (`prime_roster`), 8 public targets per curve,
  `target_seed` 1504454584, `measurement.seed` 976050496, 14 arms, 1 round: 336 executions, per-execution timeout
  7,200 s.
- **Matched rho and band.** `rho-neg` on the same targets; S at w = 0, 0.1, 1 with every unpriced F4 counter
  (`solver_F_p_multiply-adds_(elimination_only)_uncharged` is the dominant one) priced alike. The F4 and
  grid counters are different units and the grid arms are read descriptively. Distance is S(arm) / S(rho) and
  nothing else.
- **Cost (estimate).** From the spec's derivation: 4.2 hours single-thread, 46 vCPU-hours ceiling including replays,
  under USD 10. Local 4-core host: about 1.5 hours for the run once an `ecbench` with the isogeny membership is built
  (the deployed image predates it). isolab-eks: about 23 pod-hours of 2 vCPU, which is the spec's own figure.
- **Blockers, stated so rank 3 is not misread as "runnable today".** The owner must choose the route (command Job
  with counts only, versus a larger L2 spec) and the image must be rebuilt at a pushed commit with the isogeny membership
  (`algebraic_row_spec/BLOCKERS.md` items 1 to 3). The decisive PKM cells (t = 14, m = 4 and N = 20) belong to
  `research/pkm_falldegree_unblock_20261107` and are not part of this spec.
- **What each outcome changes.** H1 outcome: the algebraic track records its baseline distance (about 1e8 x rho at
  k = 20 on the data seen) and nothing else; atlas `open_blocked` stays. H0 outcome: the spec's F4 cost model is
  withdrawn before any larger curve is planned. A ratio below 1: parity candidate, validation gate first. It does not
  touch the generic-track IC baseline.

## 5. The (c) list: what new native code would be needed, ranked by information per cost

Sizes are my line-count estimates (estimate). "Worth" weighs each item against the closure evidence in section 6.
Desk items (a lemma, a paper read, a Lean obligation) are flagged because they need no Rust, but they are
still the only way those leads move. Items 1 to 3 are the ones worth cutting a task card for.

| rank | lead | what has to be written | LOC (estimate) | worth |
|--:|:--|:--|:--|:--|
| 1 | `STR-special-j-ic-semaev` | Register the j = 0 symmetric 3-summand solver (`sym_solve(summands=3)` of `examples/prime_fast_j0_sym.rs`, wall-clock only today) as an ecbench method, so it is counted and replayable, and pair it with the existing `rho.cubic_v2` (the matched automorphism rho, `aut_folded_rho.rs` named in the results note does not exist). Then one band-priced session on the 26-30 bit j = 0 curves. | 250-350 | High. The recorded gain is 2.0-2.4x over plain 3-summand IC, 3x-23x behind negation rho and growing (`RESULTS_prime_fast_j0_sym.md`); a counted row turns that into a success-criterion row with its distance to the matched rho (about sqrt(3) further ahead) and could pass the validation gate. Cost after code: under 1 CPU-hour (estimate). |
| 2 | `EXO-coordinate-statistic-leakage` (and `EXO-dl-character-sum-pseudorandom` at one size up) | Add the base-B digit-character function class (the atlas's named next step) and R24 rows (a 2^24 prime-order curve set) to the coord_spectrum tool, as a new frozen copy (the F3 tool is frozen). | 60-120 | Moderate. F3 closed x, y, Legendre and Dickson at r <= 2^20 by rule 3 and states that rule 3 does not bound other classes or larger p. A null is expected. Cost after code: order 60 CPU-hours (estimate: F3 took 15,500 s on 4 threads = 17 CPU-hours at 2^16 and 2^20; FFT length 2^24 is about 19x longer per curve, with 4 curves and 8 nulls instead of 8 and 20). |
| 3 | `STR-class-group-ecdlp-bridge`, `TRF-kernel-of-reduction-lemma` | Desk: write the simulation lemma H-CLGP-63f471 and the Lean obligation `cm_closed_walk_scalar_action`; write the kernel-of-reduction lemma that collapses five lead families' revisit clauses. | 0 Rust; 100-200 Lean lines | High for cost: each closes a family at exponent 1/2 and removes it from every later queue. No compute. |
| 4 | `RHO-best-of-candidates-indegree` | Charge a shared inversion for the two candidates in `src/cryptanalysis/ecbench/best2_model.rs` (today both candidates cost full additions), then run the frozen `research/nist_best2_small_batch_20261027/ecbench-model.json`. | 60-100 | Low. The native walk already measured 1.129 [1.081, 1.184] steps and 1.22x per step at 1024 walks; the only open piece is GPU W = 16-64, and BEST2 stays disabled by audit. |
| 5 | `RHO-advice-table-variants` | Add a pilot-walk reselection option to `rho.preprocessing`. | 150 | Low. The atlas records a loss (-0.064 [-0.080, -0.049] at 2^24 for T/2). |
| 6 | `EXO-lange-winterhof-interpolation`, `EXO-invertible-hint-subset`, `EXO-elliptic-code-list-decoding`, `EXO-low-degree-algebraic-query-bound` | Desk reads and derivations (paywalled theorems, the Guruswami-Sudan agreement threshold, a proof program). | 0 | Closure work; each can end as (d) once read. |
| 7 | `ICP-full-width-arity-grid` | Repair the launcher admission and RSS monitor for the 63-bit three-base panel (H-ICPERF-05021b). | 100-200 | Low per CPU-hour: single-target IC is already 443-528x rho at 32 bits, so a 63-bit panel is expected to censor; the certified m = 3 grid is M1 (owned elsewhere). |
| 8 | `ICP-pkm-tower-algebraic-oracle` (prerequisite of spec 3) | A planted-decomposition positive control for pkm-semaev (BLOCKERS item 6). | about 150 | Required before any null of the algebraic row is read; small. |
| 9 | `ICP-construction-normal-form` | Normal-form dedup and estimator-bias check inside `factor_base_sweep`. | 150 | Instrument hygiene, no exponent content. |
| 10 | `ICP-hecke-cm-class-point-rank-completion` | A Hecke / Atkin-Lehner relation generator on X_0(11) with setup pricing (`hecke_target_router.rs` is only a generic shift control); the D79 records are missing. | about 400 | Low: marginal Hecke rank over the pair-reuse control is 0, 10, 19, 0 on four cells and the control already leaves one free log. |
| 11 | `ICP-split-graph-secant-presentation` | Native triangular-factorization / FGLM root extractor plus a three-cell prospective panel. | about 700 | Low: degree d+1 versus 2d+1 is a theorem but no complete-cost gain over the dense presentation yet. |
| 12 | `ICP-digit-presentation-solving-degree`, `ICP-shared-coefficient-membership` | The m = 3 source-bound CDCL(T)/F_p solver (CaDiCaL F_p trail theory) and protocol. | 2,000-4,000 | Low: the closed form d_ff is curve-independent and equals a curve-free null; PR #3970 covers the CM-lift and Dickson part. |
| 13 | `ICE-gag-contraction-planner`, `ICE-syzygy-cascade-signatures` | Prime-field chain-versus-tree matrix-cell counts with msolve; an F5-signature counter. | 300 / 500+ | Low: counted proxies, no ECDLP cost claim; the F5 lead is binary. |
| 14 | `EXO-lossy-object-propagation`, `EXO-ideas-corpus-algebraic-transforms`, `ICP-lattice-ic-relation-construction`, `ICP-prym-witness-relation-diversity` | A projection panel; constructors that do not yet exist; a lattice relation pipeline with no theory; records that are not in any repository. | 300 / unbounded / unbounded / blocked | Not worth: nothing to build against until a constructor or the records exist. |
| 15 | `ICP-gfp5-oa-sdhp-jv`, `ICP-gfp5-odd-char-cover`, `TRF-odd-char-descent-smith-quartic`, `TRF-small-genus-cover-general`, `TRF-odd-prime-ghs-cover-floor`, `STR-level-structure-torsor` | Extension-field (E(F_{p^k})) tooling: an n = 5 JV oracle (about 1,500), odd-characteristic cover genus (300-500), Smith's construction (1,000-2,000), a non-Kummer cover family (about 500); the torsor item is a desk genus computation. | see left | Not E(F_p). Queue only if the extension-field programme is revived. |

## 6. Evidence for the (a) and (d) decisions

Reasons are the full text of `decisions.json`; every path was checked to exist at 0c2cc1b408. Evidence standing of every number is `[pcc:unverified]`. Three of the (a) studies used Python harnesses (`ml_learnability_control_20261107`, the analysis scripts of `minors_guess_determine_20261107`, and parts of the older rho and conductor-gap studies); under AGENTS.md "Implementation language: no Python" they stand as historical measurements and a rerun needs a native harness.

### 6a. Already tested for real (a)

- **EXO-minors-guess-and-determine**: Real six-arm 20-40 bit prime-field panel vs matched negation rho: every arm costs more than rho, exponent upper bounds 0.815-1.065; lead CLOSED for the reconstruction implemented. [pcc:unverified]
  Paths: `research/minors_guess_determine_20261107/RESULTS.md`
- **EXO-ml-learned-models**: Learnability control with planted leak: 0 SIGNAL in 24 cells at m=12..24 (22 NULL, 2 UNDERPOWERED). Harness is Python, so historical; a rerun needs a native harness.
  Paths: `research/ml_learnability_control_20261107/RESULTS.md`
- **EXO-hashed-factor-base-tensor-ecdl**: Real native tests: reduction audit found no correctness error; treewidth lower bound exceeds b/2 by 4-5x at b=8..40; native tensor IC speed panels exist (ic.tensor_contraction_v1).
  Paths: `research/semaev_tensor_treewidth_20261111/README.md`, `research/hashed_ecdl_tensor_audit_20261107/AUDIT.md`, `research/ecdl_tensor_ic_speed_20261030/README.md`
- **EXO-padic-jet-canonical-lift**: Real study: torsion-section p-adic digits are generic on Kummer bases (42 cells, 17 curves); jet exit adds only free F_p unknowns; Teichmuller contrast at chance.
  Paths: `research/prime_ic_leads_20261110/jet_exit/README.md`
- **EXO-weak-class-instrument-calibration**: Measured: structure instruments do not detect anomalous or small-embedding classes (36 per class, p<=2^14); coord_spectrum carries an anomalous positive control that passes (P2).
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/coord_spectrum_20261110/README.md`
- **ICE-curve-presentation-invariance**: Real prime-field conductor-gap IC studies: floor/crater query ratio 0.878 [0.764,1.009], CPU 0.980 [0.866,1.111]; IC floor/crater 0.975 and 1.127 (no separation); whole-field census finds only Lenstra divisibility bias. The EXP-DREG run is a degenerate binary toy.
  Paths: `research/conductor_gap_cm_orbits_20261015/REPORT.md`, `research/isogeny_conductor_gap_ic_20261010/README.md`, `research/fp31_whole_field_census_20261111/README.md`
- **ICE-gpu-rootfind-oracle**: Real CUDA root-finding decomposition runs exist (40/56/61-bit validations) and a GPU end-to-end IC study; the sweep toy had no GPU.
  Paths: `research/gpu_rootfind_decomposition_20261012`, `research/ic_gpu_e2e_20261113/REPORT.md`
- **ICE-rootfind-pairtable-oracle**: Prime pair-table and root-finding IC measured against matched rho: no IC arm beats rho at any band weight, none has a fitted exponent below rho's; single-target IC/rho 15-19x (24 bits), 62-78x (28), 443-528x (32). The 17-bit Koblitz row is binary.
  Paths: `research/ic_frontier_campaign_20261015/README.md`, `research/ic_e222_m221_20261106/CAMPAIGN_SUMMARY.md`, `research/ic_rho_floor_20261109/README.md`
- **ICE-charged-toy-ic-exponent-vs-rho**: Same campaign: no IC arm has a fitted cost exponent below rho's; bitop ladder 16-40 bits ic-pair-rule exponent 0.724 [0.707,0.736] (33-40 bit) vs rho-neg 0.548.
  Paths: `research/ic_frontier_campaign_20261015/README.md`, `research/prime_field_lead_assessment_20261107/crypto_survey.md`
- **ICP-retained-setup-online**: Closed_dominated: the 13.9x / 18.5x claims were against empty-table rho; preprocessing rho at equal advice is 53-317x faster online; multi-target IC measured in counted ops (mt_fieldops, owned elsewhere; cited as evidence only).
  Paths: `research/20261016-n83-precomputed-rho-control`, `research/equal_advice_wide_20261018/README.md`, `research/mt_bsgs_gap_20261111/README.md`, `research/mt_fieldops_v3_20261111`
- **ICP-coincidence-harvest-structured-base**: Native break-even ladder 16-32 bits: every slope d ln kappa / d ln N in [-0.006,+0.008] against break-even exponents 1/3, 1/5, 1/7; planted controls recovered (SUB_BREAK_EVEN_AT_SCALE).
  Paths: `research/prime_coincidence_harvest_20261029/README.md`, `examples/coincidence_ladder.rs`
- **ICP-residual-partial-relations**: Measured: 39 sessions at 20-44 bits, slope -0.025 [-0.034,-0.017] same as rho/BSGS/kangaroo (a birthday search); S 2.28-11.89 at 36 bits vs 1.29 rho-neg. A prime-field double-large-prime variant has no native code.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/ic_rho_floor_20261109/README.md`
- **RHO-automorphism-class-walks**: Native ecbench prime j=0 and j=1728 sessions: rho.cubic/rho.negation 0.647 [0.543,0.770] (predicted 0.577), rho.quartic/rho.negation 0.819 [0.722,0.929] (predicted 0.707); ratio to the sqrt|Aut| ceiling 1.0043 and 1.0442 at p=100057.
  Paths: `research/prime_rho_automorphism_20261015/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **RHO-best-of-candidates-indegree**: Native measurement: best of P+/-R steps ratio 1.129 [1.081,1.184] (predicted 1.155), but 1.22x per-step cost on P-256 at 1024 walks is a net loss; the W=16-64 shared-inversion case needs the GPU kernel or a counted model with shared inversion (about 60-100 LOC).
  Paths: `research/rho_walk_leads_20261014/README.md`, `research/nist_best2_small_batch_20261027/README.md`, `research/nist_best2_small_batch_20261027/ecbench-model.json`
- **RHO-cheon-deployed-applicability**: Priced Cheon arms measured natively (cheon.pminus1_comb, cheon.pplus1_comb) and the deployed-parameter shape census exists; recorded reading: values need d-degree supply, not a vulnerability. Protocol for a new deployed-curve run was retired 2026-10-17.
  Paths: `research/cheon_ecbench_wide_20261106/REPORT.md`, `research/cheon_aux_leads_20261112/README.md`, `examples/cheon_p224_shape.rs`
- **RHO-cheon-toy-reproduction**: Real ecbench Cheon comb arms with memory charged on prime curves (96/96 runs equal the medium-study runs); toy reproduction superseded.
  Paths: `research/cheon_ecbench_wide_20261106/REPORT.md`, `research/cheon_aux_leads_20261112/README.md`
- **RHO-correlated-walks-shared-sums**: Native measurement: shared index sequence 0.073x [0.070,0.076] (lower bound, 176/1000 censored); precomputed pair table 1.027x [0.982,1.073].
  Paths: `research/rho_walk_leads_20261014/README.md`
- **RHO-matched-rho-calibration**: Sealed prime medium-tier bound: rho.negation M = 1.067 [0.999,1.134] x floor over 432 runs at log2 r 32.4-49.0, setup_excess, exponent 0.506 [0.483,0.529]; all gates pass.
  Paths: `research/prime_medium_rho_cpc_20261109/RESULTS.md`, `research/prime_medium_tier_rho_20261107/RESULTS.md`, `research/prime_medium_rho_p4_confirm_20261108/RESULTS.md`
- **RHO-pool-class-quotient**: Negation quotient measured on prime curves: rho.plain/rho.negation S ratio 1.443 [1.311,1.589] (M1) and 1.285 [1.177,1.404] (J1); atlas B3 negation walk 0.700 [0.673,0.728] of v2 steps. The pool-specific default is in another repository.
  Paths: `research/prime_medium_rho_cpc_20261109/RESULTS.md`, `research/prime_medium_tier_rho_20261107/RESULTS.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **RHO-prime-gpu-rho**: Counted-op claim measured: rho.negation_lanes S is a few percent above rho.negation at 8-32 lanes, 24-32 bits; GPU throughput admission needs a GPU host (ecbench gpu-stage exists), no new code.
  Paths: `research/ecbench_rho_lanes_20261023/REPORT.pdf`, `src/cryptanalysis/ecbench/gpu.rs`
- **RHO-dp-multitarget-precomputation**: Real native evidence: preprocessing rho at equal advice is 50-500x faster online than IC; retention and DP multi-target rho arms in counted field operations (mt_fieldops v3/v4, the latter owned elsewhere; cited as evidence only).
  Paths: `research/20261016-n83-precomputed-rho-control`, `research/mt_fieldops_v3_20261111`, `research/mt_bsgs_gap_20261111/README.md`
- **RHO-native-step-cost**: Native ecbench rho versions v3-v6 and chained are sealed in counted units; recorded instructions per step 1,196 to 718, floor about 6-7 field multiplications; PMU timing needs a PMU host.
  Paths: `research/ecbench_rho_v5_20261107`, `research/ecbench_rho_v6_20261107`, `research/ecbench_rho_chained_vs_v5v6_20261111/table.md`
- **RHO-negation-map-fruitless-cycles**: Real prime-curve measurement: rho.negation 1.067 [0.999,1.134] x floor at 32-49 bits with cycle escape; rho_walk_leads lead 4 measured the rule variants.
  Paths: `research/prime_medium_rho_cpc_20261109/RESULTS.md`, `research/rho_walk_leads_20261014/README.md`
- **STR-automorphism-quotient-rho**: Same native j=0 / j=1728 sessions: 0.647 [0.543,0.770] and 0.819 [0.722,0.929] of negation rho (sqrt|Aut| ceiling 0.577, 0.707); atlas closed_known.
  Paths: `research/prime_rho_automorphism_20261015/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **STR-isogeny-class-generic-invariance**: Real censuses: 185,363 isogeny classes of F_(2^31-1) with only Lenstra divisibility bias; 38 curves x 100 seeds KW p 0.38; atlas 403,168-run census, floor/crater rho 1.004 [0.990,1.016].
  Paths: `research/fp31_whole_field_census_20261111/README.md`, `research/isogeny-class-rho/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **STR-special-j-ic-semaev**: Native j=0 symmetric 3-summand IC measured: 2.0-2.4x faster than plain 3-summand IC after validator correction, but 3x-23x behind negation rho and growing; the matched automorphism rho is not wired (see c-list).
  Paths: `research/ic_e222_m221_20261106/RESULTS_prime_fast_j0_sym.md`, `research/prime_rho_automorphism_20261015/README.md`
- **TRF-padic-canonical-section**: Theorem plus real study: the only homomorphic section is prime-to-p torsion (killed by the formal logarithm); jet_exit digits generic on 42 cells.
  Paths: `research/prime_ic_leads_20261110/jet_exit/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-jv-sieve-fp6-weak-class**: Real follow-ups (extension-field weak class): 22-24.5% of curves admitted; implemented walk reaches a weak curve from 0% at p=1511, 1823; walk charged 0.0155x rho at M=512.
  Paths: `research/jv_weak_coverage_20261111/README.md`, `research/jv_walk_charge_20261111/README.md`
- **TRF-teske-trapdoor-fp6-weak-class**: The public isogeny walk to the weak class is the quantity measured in jv_walk_charge (extension field, not E(F_p)).
  Paths: `research/jv_walk_charge_20261111/README.md`, `examples/jv_isogeny_walk.rs`
- **TRF-weil-restriction-jacobian-class**: Native census and theorems exist: a prime-order E/F_(q^3) has no eigenvalue 1 on the relevant kernel (Corollary), recorded run results/run-20261018T1605Z.
  Paths: `research/weil_restriction_isogeny_census_20261018/REPORT.md`, `examples/weil_restriction_isogeny_census.rs`

### 6b. Closed at desk (d)

- **EXO-augmented-oracle-ggm**: Atlas disposition closed_obstructed (theorem): label-valued oracles (x, y, halving, endomorphism image = public scalar) have explicit GGM simulators; the sweep falsification was a toy artifact. [pcc:unverified]
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/lead_sweep_20261110/LEAD-EXO-augmented-oracle-ggm/RESULT.md`
- **EXO-curve-model-representation**: Recorded measurement: summation degree d_reg = 2 is model-invariant; Kummer and u = x/y classes tie at D = 2^((m-1)^2); no coordinate with degree below 2 is known.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **EXO-four-list-ktree-join**: Closed on paper (a Wagner filter needs a subgroup-like set) and measured generic: k-sum k=5 at 40 bits is 9.7x rho-neg.
  Paths: `research/prime_ic_leads_20261110/README.md`, `research/prime_ksum_table_20261028`, `research/prime_field_lead_assessment_20261107/README.md`
- **EXO-low-degree-query-lower-bound**: H1 withdrawn (false under a full-rank condition, exact check 39/39 cells at p=251..2039); H1-prime derived in the cited file with Delta in place of D.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **EXO-cryptoscale-candidate-screen**: Atlas exhausted_at_scale: 13 screening batches FINAL_NO_CANDIDATE; revisit only with an externally sourced candidate and a constructor.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **EXO-elliptic-net-eds**: Recorded measurement: elliptic-net k-recovery exponent 1.97-2.80 against BSGS 1.17-1.32; the GGM-simulable locator carries no nonlinear gain.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/notes/cm-isogeny/RESEARCH_EDS_RESIDUE.md`
- **EXO-hnp-advice-leakage**: The leak ([alpha^i]G auxiliary inputs) is assumed, not obtainable from public data; acquisition is closed_obstructed.
  Paths: `research/cheon_aux_leads_20261112/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **EXO-ksum-conjecture-s5-oracle**: Conditional obstruction under the k-SUM conjecture, with measured k-sum tables 9.7x rho-neg at k=5, 40 bits; remaining work is writing the reduction.
  Paths: `research/prime_ksum_table_20261028`, `research/prime_field_lead_assessment_20261107/README.md`
- **EXO-split-jacobian-projection**: Proof (KN-FIND-61347e): Hasse-Weil inflation outweighs genus-2 index calculus.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **EXO-torsor-deck-period**: Listed among rows closed by theorem or measured invariance in the prime-field assessment; no open test named.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`
- **ICE-decomposition-probability-audit**: Per-trial relation R = aG + bQ is uniform in the group, so decomposition probability is target-independent by random self-reduction and its mean is fixed by conservation; learned predictors are null (ml_learnability_control).
  Paths: `research/ml_learnability_control_20261107/RESULTS.md`, `research/prime_field_lead_assessment_20261107/README.md`
- **ICE-semaev-chain-cost-model**: Closed by cost model: optimal chain length t* = m* at every n in [250,600], charged cost topology-blind (EV-SEMBIN-1d36a3); tensor treewidth census agrees that Semaev buys constants.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/semaev_tensor_treewidth_20261111/README.md`
- **ICE-semaev-cover-monodromy**: Proof: generic monodromy (Z/2)^(m-2) x| S_(m-1), uniform in the curve for p>3; cycle-type census is a no-go as a relation-rate probe at every m>=3.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **ICE-generic-oracle-arity-cost-floor**: Closed_obstructed (cost model): zero in-domain sub-rho cells for enumeration/MITM oracles; prime analogue is the Corrigan-Gibbs-Kogan S*T^2 >~ N and Shoup T >~ sqrt(N) theorem.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **ICE-semaev-yield-model-coset-typing**: Mean yield is C(B+m-1,m)/N for every base of size B (conservation); six geometries incl. quadratic-character cosets measured at yield ratio 1.000000; typing trades an m! accounting factor only.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/factor_base_excess_20261026/README.md`
- **ICP-extension-field-embedding**: Cost model: Gaudry p^(2-2/n) >= p; measured genus-2 IC after chained transfer is 9.8x-42.9x matched rho (about p^1.0).
  Paths: `research/chained_prime_transfer_20261111/README.md`, `research/prime_ic_leads_20261110/README.md`
- **ICP-gfp5-rho-reference-audit**: Atlas closed_known; extension-field rho reference, not E(F_p).
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **ICP-representation-subset-sum**: Closed_obstructed (theorem): generic, Shoup T >~ sqrt(N) and S*T^2 >~ N; the filter has no non-generic handle.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **ICP-algebraic-membership-factor-base**: Theorem-kind (conservation KN-FIND-007 plus the bounded-degree theorem); six geometries measured within 0.0003 bits of a random base at m=2, N up to 2^26; reopens only on KN-OPEN-020.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/prime_ic_leads_20261110/README.md`
- **ICP-coppersmith-smallroot-semaev**: Derived reach bound <= 0.3536 per block vs demand 8/9; derived eps < 3/4 for any lattice from (p,f)^t on S_3 and for every monomial set; joint (4,2) lattice computed nu_s/s >= 2.08; 0 of 960 recoveries at beta >= 0.20.
  Paths: `research/smallroot_optimal_reach_20261026/README.md`, `research/smallroot_followups_20261026/README.md`, `research/coppersmith_s3_theta2_20261023/REPORT.md`, `examples/smallroot_joint_lattice.rs`
- **ICP-gfp5-symmetrized-gaudry**: Atlas closed_dominated: m=5 quotient solved at three primes, c_cyc 2^46.30-2^46.63 vs closure bar 2^46; extension field.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/ecgfp5_quotient_hnf_finish_20261102/README.md`
- **ICP-self-pairing-prime-order**: Closed_obstructed (theorem): self-pairings are 1-dimensional, trivial for embedding degree > 1.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **ICP-xline-symmetry-special-j**: Closed on paper: the j=0 omega-conjugate equation adds nothing; measured j=0 symmetric IC gains a constant only.
  Paths: `research/prime_ic_leads_20261110/README.md`, `research/ic_e222_m221_20261106/RESULTS_prime_fast_j0_sym.md`
- **RHO-batched-inversion-shared-pm**: Exact operation-count identity (2M+1S+3(W-1)/W M+I/W per step) and recorded P-256 timing (179 ns/step at 1024 walks); ecbench rho.negation_lanes counts it.
  Paths: `research/ecbench_rho_lanes_20261023/REPORT.pdf`, `research/rho_walk_leads_20261014/README.md`
- **RHO-bsgs-family-memory**: Theorem (Shoup T >~ sqrt(N), S*T^2 >~ N) plus measured claw.pair_table 2.20x strong rho and bsgs.negation exponent 0.529; positive control bsgs.textbook/bsgs.negation 1.489 [1.463,1.516] in the prime medium tier.
  Paths: `research/prime_medium_rho_cpc_20261109/RESULTS.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **RHO-cheon-aux-acquisition**: Atlas closed_obstructed (theorem): auxiliary inputs [x^d]P cannot be acquired below the cost they save; EXP-AUXIN-25a29a integer-arithmetic run succeeded.
  Paths: `research/cheon_aux_leads_20261112/README.md`, `research/cheon_ecbench_wide_20261106/REPORT.md`
- **RHO-lhw-modular-filter-price**: Exact residue-collision count on integers; the object is low-Hamming-weight DLP in Z_P^*, not E(F_p), so there is nothing to test on a curve.
  Paths: `research/lead_sweep_20261110/LEAD-RHO-lhw-modular-filter-price/PROTOCOL.md`
- **RHO-lhw-polymem-collision-walks**: Low-weight DLP in Z_P^* with O(sqrt(S)) collision walks is the standard split-and-collide bound, not an elliptic-curve claim.
  Paths: `research/lead_sweep_20261110/LEAD-RHO-lhw-polymem-collision-walks/PROTOCOL.md`
- **RHO-lhw-preprocessing-frontier**: The product law eps = p_split*S*T/(L*R) is an exact identity for the Coppersmith split; Z_P^* object, not E(F_p).
  Paths: `research/lead_sweep_20261110/LEAD-RHO-lhw-preprocessing-frontier/PROTOCOL.md`
- **RHO-cheaper-step-primitives**: Exact operation counts (8M+3S Jacobian mixed add vs 2M+1S+3M+I/W batched affine) and the theorem that a tag of an unnormalised representation is not a function of the point, so walks do not merge.
  Paths: `research/rho_walk_leads_20261014/README.md`, `research/ecbench_rho_lanes_20261023/REPORT.pdf`
- **RHO-non-automorphism-equivalence**: Arithmetic: canonicalisation needs h-1 generic scalar multiplications (about 1.5 log2 N group operations each), above the sqrt(h) saving for every h > 2; break-even image cost < 1/(sqrt(h)+1) step.
  Paths: `research/lead_sweep_20261110/LEAD-RHO-non-automorphism-equivalence/PROTOCOL.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **STR-endo-scalar-action-generic**: Theorem: End(G) = Z/N on a prime-order group, GGM-simulable; the sweep falsification is an undiagnosed toy artifact.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **STR-isogeny-transport-preprocessing**: Proof (closed_obstructed): table transport reduces to single-curve preprocessing; measured loss 1.5x-21x at odd l 3..31 (EV-ENDO-10109d N4). The sweep failure was a reversed monotonicity check.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/lead_sweep_20261110/LEAD-STR-isogeny-transport-preprocessing/RESULT.md`
- **STR-safecurves-parameter-audit**: An exact audit of public parameters, not a research hypothesis; the facts are curve-database lookups (ecdb) and docs/curves; nothing to measure.
  Paths: `examples/safecurves_orbit_census.rs`, `research/prime_field_lead_assessment_20261107/README.md`
- **STR-secp256k1-cm-class-audit**: Recorded: secp256k1 CM structure gives only the sqrt(6) automorphism ceiling (j=0, h=1); the sweep crash was an instrument failure.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/prime_rho_automorphism_20261015/README.md`
- **STR-jmv-random-self-reduction**: Literature verdict plus a derived obstruction: random self-reduction across an isogeny class shows hardness equivalence, not a cost reduction.
  Paths: `research/jmv_self_reducibility_20261008/README.md`
- **STR-p192-class-cm-relations**: Recorded: 103/103 CM relations are scalar (closed_obstructed); P-192 isogeny tower studies carry no relation.
  Paths: `research/prime_field_lead_assessment_20261107/README.md`, `research/pkm_isogeny_tower_p192_20261015`, `research/isogeny_walk_p192_20261004`
- **STR-small-cm-disc-nonunit-endo**: Elementary orbit bound h >= log N / log n; net factor <= sqrt(h)/kappa needs canonical representative cost o(h), no such algorithm; native CM-orbit IC campaign shows no floor/crater separation.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/conductor_gap_cm_orbits_20261015/REPORT.md`
- **TRF-anomalous-padic-lift**: Closed_known: Smart's attack applies to anomalous curves only (certificate p=771853, C_special 157 vs matched rho 779).
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-curve-parameter-transfer-audits**: Atlas closed_known; twist evidence recorded.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/quadratic_twist_evidence_20261021`
- **TRF-mov-small-embedding-degree**: Closed_known: MOV/Frey-Ruck apply only at small embedding degree; the sweep falsification (4 hits vs 3.27 expected) is a threshold artifact.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/lead_sweep_20261110/SWEEP_REPORT.md`
- **TRF-one-point-self-pairing**: Theorem (closed_obstructed): self-pairings of a single point are trivial or degenerate.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-second-torsion-direction**: Theorem (Balasubramanian-Koblitz); the sweep falsification is a toy-size coincidence.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-same-field-genus-g-embedding**: Theorem-kind: #Jac = N*Theta(N^(g-1)), index calculus O~(q^(2-2/g)) >= q; chained prime transfer measured 9.8x-42.9x matched rho.
  Paths: `research/chained_prime_transfer_20261111/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-pairing-target-field-recost**: No ECDLP content for non-pairing curves (atlas); the BLS12-381 exTNFS recost is a cost-model statement outside prime-field E(F_p) ECDLP.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/lead_sweep_20261110/LEAD-TRF-pairing-target-field-recost/RESULT.md`
- **TRF-teske-secret-isogeny-trapdoor**: Tate: isogenous curves share the order, so no trapdoor on F_p; measured class homogeneity (census, isogeny-class-rho). The 20-arm EXP-ECTD-0cf008 panel is the isogenous-yield work of PR #3961 (owned elsewhere).
  Paths: `research/fp31_whole_field_census_20261111/README.md`, `research/isogeny-class-rho/README.md`, `research/prime_field_lead_assessment_20261107/atlas_survey.md`
- **TRF-cm-canonical-lift-heights**: Atlas cost_model closure: twelve audited L9 mechanisms, Pr[Q in red(S)] <= |S|/N. Overlaps track C / PR #3970 on lifts; treated here as closed and not re-owned.
  Paths: `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/prime_ic_leads_20261110/jet_exit/README.md`

## 7. What this pass does not establish

- No experiment was run; every (b) spec is a plan. `ecbench plan` has not accepted the draft spec files.
- The (a) and (d) rows rest on the cited files, and each of those carries `[pcc:unverified]`. A "closed at desk" row
  means a recorded derivation or measurement exists, not that an independent validator has re-derived it.
  Rows closed only by an atlas disposition (`prime_field_lead_assessment_20261107/atlas_survey.md` quotes the
  atlas YAML, which is vaulted and was not re-read here) are the weakest and are marked by that path.
- Relevance labels (yes, partly) are the inventory's own classification; extension-field and binary-flavoured
  leads are decided but flagged "not E(F_p)".
- The research-kb pass is owed before any card is written from this file, as the sweep report's own item 4 says.
- Nothing here changes an atlas disposition, an IC baseline or a leaderboard row.

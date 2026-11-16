# Transfer family: coverage digest for the IC treatise

Surveyor: transfer agent, 2026-11-06. Read-only, from `origin/main` (crypto, crypto-autoresearcher) and `HEAD` (cryptanalysis). No experiments run.

## Conventions

- **Sources.** `crypto:` means `/home/user/crypto` origin/main, `car:` means crypto-autoresearcher origin/main, `ca:` means cryptanalysis HEAD.
- **Standing.** The pcc ledger began on 2026-11-04 (159 claims in `crypto:pcc/claims/`). Nearly every result below predates it, so each number is an **unverified assertion** (README/REPORT text) unless the entry says otherwise. The family-relevant pcc claims found are:
  - PCC-20261106-3e6fc4, PCC-20261106-74c86c and PCC-20261106-b5904a (lift-dense follow-ups);
  - PCC-20261105-7179e4 (twist chord preflight, a proposal);
  - PCC-20261105-a43948 and PCC-20261105-a6723f (cubic-extension transfer, development and proposal);
  - PCC-20261105-cfd582 (twist-lift construction, a proposal).
  I did not compute their standing: `pcc standing` reads the working tree, which this survey was told not to use.
- **VALIDATION.json.** "VAL" marks a directory that has a `VALIDATION.json` (validation gate).
- **Coverage labels.** COVERED / PARTIAL / ABSENT are judged against the treatise sections in `crypto:papers/ecdlp-index-calculus-treatise/sections/`, mainly:
  - `prog_transfer.tex` (Ch. "Covers, isogenies and other transfers");
  - `obstructions.tex`;
  - `prog_frontier.tex` §ladder3;
  - `summer.tex`;
  - `extension.tex`.
- **Lead atlas.** `car:knowledge/leads/ecdlp/transfer/` holds 24 leads, by disposition:
  - closed_known: 4;
  - closed_obstructed: 6;
  - closed_dominated: 1;
  - open_next_step: 9;
  - open_blocked: 2;
  - open_unexamined: 2.

---

## 1. Isogeny transfer to a weak curve on prime fields (Tate closure, walks of standard curves)

**What was tried**

- **The hope.** Walk the class of a generic or standard prime curve to an anomalous, low-embedding-degree or descent-friendly member.
- **The closure.** Tate: F_p-isogenous iff same #E. Trace one and k = ord_N(p) are functions of (#E, p), and prime fields admit no GHS descent. So the special set inside a generic class is empty. Records: EV-IT-511f3d (strong), DEC-20260807-d4f2a9 (reject_scoped). DEC-20260808-86ec3c then restored the status to "weakened" under the "no pausing" directive. That is an administrative reversal, not a scientific one.
- **Measured.** rho_special = 0 at 20 bits (exhaustive), 24 and 28 bits (EXP-IT-001, EV-IT-aefd12).
- **Tooling on standard curves** (all "no ECDLP cost measured"):
  - `crypto:research/isogeny_walk_20261004`, `isogeny_walk_p192_20261004`, `isogeny_walk_traits_20261004`;
  - `p256_isogeny_walk_20261004`: degree-11 feasibility gate passed;
  - `p256_isogeny_multigraph_64k_20261006`: 65,536 curves; every curve has 22 rational outgoing isogenies; anomaly screen negative; S3 marker absent, so "locally verified";
  - `p256_isogeny_s3_cairn_20261004` and `p256_isogeny_shared_task_20261005`: control-plane protocols only;
  - `p192_p224_large_degree_isogenies_20261008`: screening through degree 4093; constructions beyond 1009 unresolved where receipts time out;
  - `p192_p224_hundred_large_prime_isogenies_20261010`: 100 verified maps, degrees 2341 and 1117, 2000 scalar-transport checks;
  - `p192_p224_2000_more_large_prime_isogenies_20261010`: protocol.
- **Weak-class battery** on NIST b (`b_weak_class_battery_20261022`, frozen protocol, 2026-10-22):
  - no curve × statistic flagged at Bonferroni over 85 tests (threshold p ≤ 5.88e-4);
  - smallest p was 0.034 (P-256, smooth_c);
  - 11 of 85 cells were inconclusive.
- **Designs never run:**
  - `isogeny_class_difficulty_20261008` (preregistered, nothing run);
  - `isogeny_class_ic_yield_20261102` (design for the ECCp-131 and ECC2K-130 classes);
  - `car:analysis/isogeny-dreg-search`, `car:analysis/rl-isogeny-search` (toy instruments, no RUN records);
  - `car:analysis/isogeny-class-screening` (P-256 class structure).

**Verdict.** Closed by theorem (prime fields). The walks are tooling.

**Coverage: PARTIAL.**
- Present: Lemma `lem:conservation`; summer.tex cites DEC-20260804-2fae6a.
- Missing: the explicit Tate argument, the 20/24/28-bit null, the P-256/P-224/P-192 walk and large-degree-map campaigns, and the b-battery.

---

## 2. Isogeny volcano levels and generic rho cost

**What was tried.** One ECDLP instance is carried by verified Vélu isogenies to every stratum, and rho/BSGS/IC is measured per curve.

- **`crypto:research/isogeny-volcano-rho`** (Oct 16–17).
  - Curve A, ℓ = 3: mean 659,263 ± 12,422 (95% CI), max 2,099,707 group operations.
  - Automorphism craters: about 1.32 ± 0.06× fewer operations on j = 1728 (√2 predicted); j = 0 likewise (√3 predicted).
  - Redrawing k and P does not change cost.
- **`isogeny-volcano-cost_20261017`** (v1, not the cited v2).
  - Scale: 43,200 measured runs (46,912 with controls); 408/408 replays exact.
  - BSGS floor/crater 1.000 [1.000, 1.000].
  - rho.negation 1.022 [0.983, 1.062], p = 0.28: "not equivalent within ±5%".
  - A/A 1.001 [0.965, 1.039].
  - IC relation yield 0.980 [0.957, 1.004]: inconclusive, because the planted +5% control was not detected.
  - Coordinate (isomorphic-model) effect on IC yield: τ 4.7% (null 2.5%), p = 0.0005.
  - IC is about 30–60× rho at 24 bits.
  - Correction: the rho.cubic/quartic setup was left out of totals; the corrected j = 0 control is 0.716 [0.598, 0.857], and j = 1728 is 0.723 [0.621, 0.843].
- **`volcano_tall_layers_20261018`** (preregistered).
  - Generic cost is the same on every level within ±5%.
  - Positive control plain/negation: 1.3485 [1.2380, 1.4780] (tier a).
  - a8 floor/crater 1.079 [0.973, 1.188]; a7 0.961 [0.879, 1.050].
  - S of rho.negation: 0.947 [0.936, 0.958] against the floor √(π/4) = 0.8862, constant from r ≈ 2^42 to 2^51.
- **`volcano_prime_layers_20261018`.** Two depth-8 ℓ = 2 volcanoes, 864 records. 14 of 16 intervals contain the null. The two exclusions are rho.plain on m5, both involving the single crater curve.
- **`volcano_koblitz_layers_20261018`** (n ∈ {19, 23, 37, 41, 61}).
  - Level-1/crater rho.negation intervals all contain 1.
  - Frobenius solver/negation on the crater: 0.150 [0.132, 0.170] at n = 41, against the ideal 1/√n = 0.156.
- **`volcano_koblitz_n83_20261018`.** Φ_6473(X, j0) built from 9,864 CRT primes, 8,590 s; 78 Frobenius orbits.
- **`isogeny-class-rho`** (4 classes P1/P0/B1/K, 35–40 bits; class sizes up to 708,153) and **`isogeny-class-rho-n45`** (46-bit replica, 38 isogenous curves × 100 seeds). The replica did not reproduce the first study's flagged generator. Generator heterogeneity is left unresolved.
- **`photo_volcano_restart_20261017`.** 4,600 rho + 4,600 BSGS cases on a 46-curve, 90-map, 5-level component. Includes a counter-semantics correction (Montgomery mul/sqr counters disjoint). `photo_volcano_recovery_20261017` is blocked.
- **Graphs and certificates:**
  - `full_volcano_classes_20261016/17`: 24,823 vertices, 69,300 maps, 51 strata (graph-wide IC halted; cited via `full_volcano_graphwide_ic`);
  - `curve_traits_levels_20261022`: End(E) established 2093 → 2700; open levels 668 → 34;
  - `curve_traits_tower_cost_20261022`: 13 more levels decided;
  - `curve_traits_descent_levels_20261028`: last 17 open levels closed, 19,893 End(E) proved, 0 open;
  - `curve_traits_level_certificates_20261023`;
  - `volcano_metadata_20260922/23`: accounting corrections; a wrong point-count certificate at p = 65537 was fixed;
  - `volcano_descent_dataset_20261011`: legacy Python;
  - `volcano_n53_n83_20260924`;
  - `volcano_regularity_control_20260924`;
  - `deep_volcano_prime_gap_20261022`: artifacts only.
- **`ca:experiments/ecc2k130-isogeny-class`.**
  - Native rho log2: E0 60.81; the 262 conductor-263 floor curves 64.33, but effective 60.81 via a verified 263-isogeny (≤ 2^15.6 F_q mults).
  - Levels p and 263p lie in [60.81, 64.33].
  - Closing them needs a char-2 Kani pipeline, about 2^70 F_q mults.
  - "No curve in the class is easier than E0."
- **`ca:experiments/volcano-ic`** (n = 19 analogue, 456 floor descendants). Yield differences "mostly cancel in total work".
- **Flag.** `synthetic_volcano_ecdlp_20261018` is **invented data** by its own banner. It must not be cited as evidence.

**Verdict.** Null. The only level effect is the automorphism group (√2, √3).

**Coverage: PARTIAL.**
- Present: volcano_ecdlp_census (403,168 runs), conductor_gap_rho_large_sample 0.998, drastic round3 √3, cost-v2 5.8%.
- Missing:
  - the v1 43,200-run table, including the coordinate effect on IC yield;
  - the tall-layer ±5% equivalence;
  - Koblitz layers (√n Frobenius gain);
  - the ECC2K-130 class level map (60.81/64.33);
  - isogeny-class-rho and its n45 non-replication;
  - curve_traits level certification (0 open levels);
  - the invented-data warning.

---

## 3. Conductor gaps and fibre-aware index calculus

**What was tried.** Curves with the same order but End(E) conductors differing by a large prime (JMV leaves this open).

- **`conductor_gap_cm_pairs_20261010`.** ℓ = 8209 and 131101 pairs, five ecbench sessions. **Correction chain:**
  - the banner "no implemented method sees the gap" was withdrawn in §5 on 2026-10-17 (SHF-20261017-e4d7f6, L-N2);
  - the banner was corrected on 2026-10-28 (SHF-20261028-0f3f64).
- **`isogeny_conductor_gap_20261007`.** E8 char-2 Kani preflight only.
- **`isogeny_conductor_gap_ic_20261010`** (fibre-aware relation generation, Part A; median [range]):

  | family | arm | S ÷ blind | yield ÷ blind | per-relation cost ÷ blind |
  |---|---|---|---|---|
  | prime, h ≤ 25 | closed | 0.50 [0.21, 2.36] | 11.7 | 0.094 |
  | prime, h ≤ 25 | lifts | 1.83 | | |
  | prime, h ≥ 50 | closed | 4.7 [2.3, 75] | 116 | 0.017 |
  | random binary | closed MITM | 5.8 | | |

  No fibre arm beats rho.
- **`isogeny_conductor_gap_round2_20261011`** (3,870 records). Accounting correction A0: round 1's floor/crater 3.10 becomes 1.00. Best IC is 12.8× rho; rho S is 1.1–2.1. This is the "12.8×" the treatise quotes without the correction.
- **`drastic_prime_conductor_gap_20261012`** (round 1): six CM pairs, D = −19 and −43, gaps 508,433 / 1,016,839 / 2,033,609.
- **`drastic_prime_conductor_gap_round2_20261012`.** The j = 0 crater with Aut order 6. Both pairs read within 3% of √3 = 1.7321, and both 99% intervals exclude 1. The treatise cites round 3 at 1.86–1.92.
- **`conductor_gap_cm_orbits_20261015`.**
  - Post hoc generic rho CPU crater/floor 1.17433 [0.99897, 1.38318].
  - Charged-work ratio 1.15967 [0.98616, 1.36656].
  - A 96-bit rational-family floor certified; ECDLP timing still 53-bit.
- **Not yet summarised:** `conductor_gap_followups_20261016` (five follow-ups, PR #1888), `conductor_gap_paired_20261011` (legacy Python, "NOT FOR MERGE"), and `conductor_gap_replication_20261025`, `conductor_gap_size_ladder_20261026`, `conductor_gap_crater_design_20261025` (artifacts and PDFs only, no README).
- **No README (records/PDFs only):** `isogeny_conductor_gap_round5_20261012`, `round6_20261018`, `round7_20261019`.
- **Archival imports, no admitted row:** `prime_conductor_gap60_20261031`, `prime_conductor_gap80_20261031`.
- **`isogeny_fibers_20261010`** (949 files; A1 at m = 31) and **`isogeny_torsion_fibers_20261012`** (chi-polynomial runs). PDFs only.

**Verdict.** No conductor effect. Any apparent effect was accounting (3.10 → 1.00) or automorphisms.

**Coverage: PARTIAL.**
- Present: ecbench_conductor_gap 0.975/1.127, round3 (234–529× descent, 4.1–4.3× symmetrisation then retracted), fiber_ic_scaling (#E^{1/6}).
- Missing: the SHF correction chain, the A0 correction, the Part A table, drastic rounds 1–2, the cm_orbits panel, the 96-bit floor, and rounds 5–7.

---

## 4. Isogeny-derived factor bases and presentations

- **`prime61_isogeny_20261016`.** All rational isogenies of prime degree ≤ 61 from the source. Null relation stage ("not a measured improvement").
- **`isogeny_vs_mu_factor_base_20261102`.** Petit μ_k against isogeny-kernel bases at m = 2. No rho row.
- **`curve25519_isogeny_seed_20261103`** (T4/T5).
  - Seed selection of the PKM isogeny tower.
  - 6-target interval [0.379, 1.084] includes 1, and V2 failed.
  - "T9 later replicated it on 16 targets."
  - Does not change the order against rho.
- **`isogeny_invariant_baseline_20260926`.** Historical, not preregistered; S and ratio null.
- **Atlas:** `LEAD-TRF-teske-secret-isogeny-trapdoor` cites `crypto:RESEARCH_ISOGENY_FIBERS_ECDLP.md` Thm 4.2: summation polynomials pull back along isogenies, so decomposing on E2 equals decomposing on E1.

**Coverage: PARTIAL.** prog_prime has "Small-abscissa bases and isogeny classes" and PKM tower rows. Missing: prime61, μ_k vs kernel, Curve25519 seed.

---

## 5. Isogeny computation engineering (√élu, modular polynomials, large degrees)

- **`fixed_degree_isogenies_20261016`.** Benčina et al. (fixed-degree supersingular) and all of √élu (Bernstein–De Feo–Leroux–Smith) implemented end to end, except the quantum speed-ups.
- **`cm_volcano_crt_20261016`.** CM modular polynomials by explicit CRT, odd prime degrees 3–31, exact-equality acceptance. No speed claim.
- **Performance and frontier campaigns:**
  - `isogeny_performance_20261014` (64 timing cells, constructor-only);
  - `isogeny_coverage_optimization_20261009`;
  - `isogeny_catalogue_setup_20261009` (349 models);
  - `isogeny_ci_closeout_20261009`;
  - `isogeny_walker_engineering_20261008` (protocols);
  - `isogeny_degree_challenge_20261015` and `isogeny_world_frontier_20261016` (prime-degree "world frontier"; 2,427 files; no IC/rho ratio);
  - `large_prime_isogeny_degree_20261008` (protocols; C7 withdrawn as not new).
- **`koblitz_isogeny_descent_37_20260925`** (+ `_results`). Separable degree-73 isogeny over GF(2^37): 74 rational cyclic kernels, a degree-36 kernel polynomial. Boolean stage diagnostic; "negative verdict unchanged".
- **Treatise.** Quotes the √élu kernel exponent 0.822 [0.805, 0.838] only.

**Coverage: ABSENT** (one number). Suited to an appendix.

---

## 6. Isogeny-loop endomorphisms, GLV/GLS (arithmetic, not DLP)

- **`isogeny_loop_glv_20261010`** (round 1) and **`_20261011`** (round 2).
  - Round-2 erratum: Vⁿ = [2ⁿ]∘π⁻¹, so the Vⁿ ratio is ≥ 2(c_dbl + 2)/c_dbl = 2.5 at c_dbl = 8, a provable loss. That supersedes the round-1 "λ = t − 1 via Vⁿ ratio 0.49".
  - Also: 4-D GLV–GLS.
- **`cryptopro_b_glv_chain_20261005`.** CM D = −619, h = 5; endomorphism 4 + ω of norm 175 = 5·5·7; GLV-2 against wNAF.
- **`cryptopro_b_cm_map_checks_20261005`.** Degree-155 map checks.
- **`cp6_782_endomorphism_chain_20261005`.** D = −339, h = 6, 5·5·7 chain verified on points. No timing.
- **`glv_gls_v4_20261018`, `glv_k4_pipeline_d4_20261017`.** Probably owned by the endomorphism family.

**Coverage: PARTIAL.** isogeny_loop_glv_20261012 (1.09–1.15×) is cited, and CryptoPro is named. The erratum and the CP6-782 chain are missing.

---

## 7. Quadratic twists

- **`twist_obstruction_20261026`.** COVERED (obstructions §"Twist points carry no relations").
- **`twist_index_calculus_20261020`** (exact toy).
  - Adding twist abscissas gives **0** extra width-two hits at p = 101/211/233/307, despite 876 extra candidate pairs per target.
  - Twist-target controls: T + T hits 570/569/573/573.
  - Trace-map correction recorded.
- **`twist_ecdlp_20261027`** (preregistered, up to 40 bits). No twist arm beat its baseline. Ideas:
  - E0 separation lemma: 0 violations in 8,254 + 34,498 classes;
  - E1 + G3 mixed-solution census: ratio 0.9965 (12 bit), 0.9935 (16 bit), 0.85 [0.29, 1.47] (20 bit);
  - E2 joint E + E′ table: 1.000–1.062× separate runs (gain refuted);
  - E3 F_{q²} eigen-decomposition: 53.3–2,812× more work;
  - E4 Koblitz K0↔K1 τ-orbit: identical 773,404 relations at 2.0246× the solves;
  - E5 Kummer-line walk: inconclusive, falsifiers fired;
  - E6 genus-2 gluing E × E′: #Jac = #E·#E′ on 580/580, construction only.
- **`quadratic_twist_evidence_20261021`** (10,083 files). Uniform S3 census, F37 size-three arms. "Stronger original actual-twist useful-yield/degree/main-DLP goal remains OPEN"; 1,921 study rows missing from the registry.
- **Proposals only (pcc unverified):** `quadratic_twist_followup_20261105`, `quadratic_twist_chord_preflight_20261105` (PCC-20261105-7179e4).
- **`twist_relations_tr04_20261029`.** Protocol v3 frozen, not executed.
- **`twist_walk_blocked_towers_20261014`.** 3-isogeny walks out of blocked twist towers; class-ceiling counts N = 8…22 (PDF).

**Coverage: PARTIAL.** Only twist_obstruction and lift_dense are covered. twist_ecdlp's E0–E6 table and twist_index_calculus are each worth about a page.

---

## 8. Lift-dense / twisted-line factor bases (twist-avoiding subspaces)

- **`lift_dense_factor_base_20261027`.** COVERED (obstructions).
- **Uncited follow-ups:**
  - `lift_dense_followups_20261027`: four subagent follow-ups; "the c2pnb and Oakley gains remain per-trial yield statements";
  - `c1_twisted_line_ladder_20261027`: n = 4 toy ladder; exact yields k ∈ {5, 7, 9, 11}; local, EKS confirmatory run open;
  - `lift_repulsion_mechanism_20261027`: Walsh spectra are Kloosterman spectra; sd of ratio₄ 0.0182 vs nulls 0.0474 at N = 11;
  - `oakley_lift_dense_20261027`: protocol;
  - `lift_dense_orbit_parity_20261106`: odd char; PCC-20261106-3e6fc4: all 1,287 criterion lines are all-lift or all-twist;
  - `lift_dense_torsion_pairing_20261106`: PCC-20261106-74c86c (c2pnb counts reproduced, none T-stable) and PCC-20261106-b5904a (T-stable line exists iff Tr_{N/d}(b^{1/4}) = 0; 48/90 curves).

**Coverage: PARTIAL.** These are the only family items with pcc claims.

---

## 9. Order-24 twist automorphism on j = 0 (E29/E34/E41)

- **`e29_twists_20261030`.**
  - ψ_p has order 24 on all 8 E18 curves (p = 241…1069).
  - As a rho walk: steps order-6/order-24 pooled 1.701; F_p cost 0.962 pooled (schoolbook). "Not a stronger reference as built."
  - As a base fold: cuts j = 0 linear algebra about 14× (LA ratio 13.3–15.2) and moves the modelled S_5 handover.
- **`e34_twist24_cheap_rep_20261031`.**
  - Cheaper representative: 15.17 products per call, down from 86–92.
  - Band (E29/cheap): 1.563 / 1.254 / 1.126 at w = 0/0.1/1; order 24 ahead on 7/8 curves.
- **`e41_twist24_shortfall_20261102`.**
  - The 1.701 (vs ideal 2) was sampling noise, not a walk defect: order-6 walk at 0.924 ± 0.046 of ideal, order-24 at 1.086 ± 0.045.
  - After the fix: O6B 0.969 ± 0.016, O24 1.006 ± 0.016 (1,024 walks per arm).
- Has a replay certificate. No VALIDATION.json.

**Coverage: ABSENT.** Needs a "stronger rho reference" subsection, since it defines the matched rho on j = 0 curves.

---

## 10. MOV / Frey–Rück pairings, embedding degree, self-pairings, second torsion direction

All theorem closures, in `car:` leads:

- **LEAD-TRF-mov-small-embedding-degree** (closed_known). k is class-invariant and about N generically (Balasubramanian–Koblitz). RQ-PAIR-21313f has zero runs.
- **LEAD-TRF-one-point-self-pairing** (closed_obstructed). A self-pairing on a cyclic prime-order line forces embedding degree 1; a CM endomorphism is not a distortion map. Sources: IDEA-20260807-30868b, -464ff8; IDEA-20260901-2d3f07.
- **LEAD-TRF-second-torsion-direction** (closed_obstructed). Trace-zero T_k(E)[N] = 0 unless k_emb | k; the Kani/Robert (N,N) graph kernel is rational iff k_emb = 1. Sources: IDEA-20260808-40aab9, `crypto:cd_attack/SUMMARY.md`.
- **LEAD-TRF-pairing-target-field-recost** (open_next_step). exTNFS recompute for BLS12-381/BN254: 33 EXP-PAIR contracts, **0 runs**.
- **LEAD-TRF-miller-cocycle-nonhom-transfer** (open_unexamined). Miller-cocycle primitive U(R) = f(R)^{1/N} when gcd(N, p − 1) = 1 (IDEA-20261017-e3a6f1, IDEA-20261009-c8433d, IDEA-20261016-09fd08).

**Coverage: ABSENT.** prog_prime.tex has one sentence ("only small-embedding-degree pairing curves…"); obstructions names Balasubramanian–Koblitz in a list. 2–3 pages of theorem-plus-scope would fit.

---

## 11. Anomalous (Smart) and p-adic lifts, canonical sections (CZLIFT)

- **LEAD-TRF-anomalous-padic-lift** (closed_known).
  - Smart certificate at p = 771853: C_special 157 vs matched rho 779 (ratio 0.202), positive control.
  - KN-FIND-3c7e19 (toy, replicated): Smart degenerates exactly on the canonical-lift class mod p².
  - DEC-20261009-44c288: evidence round not yet filed at that time.
- **LEAD-TRF-padic-canonical-section** (closed_obstructed).
  - The only homomorphic section is the prime-to-p torsion section, which the formal log kills (KN-FIND-7e2b94, derivation, uniform p ≥ 5).
  - EV-CZLIFT-ad28f5: 1116/1116 and 576/576 instrument pairs.
  - EV-CZLIFT-f2a401 (strong): naive F_p^* section rejected; the "falsification" is a size effect for g^d < p.
  - Open: KN-OPEN-d3a6f0 (Fourier mass decay vs n^(−1/4)).
  - DEC-20261024-23d64d: RQ-CZLIFT-b9f765 cannot close yet.
- **`crypto:p2_lift_defect_20261101`** (H110). 9,096/9,096 targets pass checks. 28/28 scalar words have degree r − 2 (81–945) against gates 49–100, so 0/28 pass. The scoped bridge is rejected.
- **`crypto:torsion_division_derivative_20261031`** (H109). 10,674,040 F mults charged. Exact formulas 0/28; candidate 0.500–0.611 vs controls 0.500–0.630. Scoped null.

**Coverage: ABSENT.** "Smart" never appears in the treatise. Est. 3 pages.

---

## 12. Global lifts / xedni / heights / kernel-of-reduction

- **Function-field xedni:** P_lift(p) = (1 − 2/p + p^{−3})/(2p³), α = 3. COVERED (obstructions).
  - Missing nuance (LEAD-TRF-function-field-xedni-lift): max|coeff| = 1 is forced by the lattice (norm-2 vectors at support 3), so the coefficient census had no discriminating power (IDEA-20260805-bd61c3).
- **LEAD-TRF-global-lift-small-height** (open_next_step).
  - JKSST Thm 4.1 is conditional on several premises.
  - CZLIFT round 2026-10-23: H-CZLIFT-9a32e9, -cb95b8, -27a184 weakened; two run sets inadmissible.
  - EV-CZLIFT-ee9b23: 1 relation (the tangent relation at p = 101) in 3,200 valid trials with max|m_i| ≤ 6; 0/1,600 at r = 3; a planted dependence was detected.
  - DEC-20261023-26a6de: its KN-LIT-7593 citation resolves to an unrelated AES paper (citation defect).
- **`crypto:jksst_xedni_audit_20261101`** (Lean, kernel-checked scoped audit). Conditional theorem repaired; several overbroad inferences refuted. For example, rank, radius and minimum separation do not determine a lattice ball count (explicit counts 5 vs 3), and no fixed discriminant bounds the heights of all multiples.
- **LEAD-TRF-cm-canonical-lift-heights** (closed_obstructed, lane L9). The closure's load-bearing argument was corrected (CORR-20260810-930c42, DEC-20260810-3110d5). Point-lifting routes give Pr ≤ p^{−1+o(1)}, an instance exponent of 1 against rho's 1/2.
- **LEAD-TRF-kernel-of-reduction-lemma** (open_next_step). A design draft only.
- **`car:analysis/xedni-cheon`** (H-XEDN-0ea2b6 proposed lemma) and **`xedni-prescribed-points`** (proposals; no lift-search runner).

**Coverage: PARTIAL.** Only the function-field family is covered. Est. 3–4 pages.

---

## 13. Binary GHS / Weil descent screens on registry curves (composite N)

- **`jacobian_transfer_scan_20261010`.** 321 curves, 1156 routes. Non-boundary: only Oakley 3/4 and c2pnb.
  - Oakley Group 4: +10.9 bits (genus 15 / GF(2^37)).
  - c2pnb368w1: +10.1 bits.
  - Same-field genus-2 cover: −2.1 to −2.0 bits.
  - The w1 rows were corrected on 2026-10-13 by `docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md`.
- **`jacobian_transfer_lemma6_20261013`.** MMT Lemma 6 across 2091 registry curves; condition (8) derived; two curves change verdict.
- **`ghs_counterfactual_n130_20261013`** (model).
  - m = 8 is impossible for N = 130; least admissible is genus 255 (type I).
  - Cost 2^55.60 vs rho 2^60.31, so −4.71 bits; the old genus-128 row read −23.91.
  - Crossover between genus 288 (2^59.91) and 296 (2^61.34).
  - The treatise has only the later measured repricing to −1.83 bits.
- **`ghs_curve_comparison_20261009`.** "Higher-genus GHS solver not implemented." Rho vs BSGS raw ratios 2.35 and 4.13.
- **`ghs_ipsec_isogeny_class_20261023`.** Archived Python search (24,492 checkpoint records); Oakley group 3 n = 5 genus 16; no security conclusion.
- **`ghs_generalized_20261024`.** Explicit generalized-binary-GHS maps.
  - 147 genus ≤ 2 profiles at degree 12; 232 index-two genus-2 profiles.
  - "Do not support a calibrated GHS advantage over rho."
  - Known families (GHS 2002, Hess 2004).
- **`ghs_cover_decomposition_20261025`.** Lean complementary-projector theorem plus 13 subgroup-transfer theorems; still assumes pull/push composition.
- **`weak_isogeny_classes_20261013`.** Weak-class fractions under GHS and 3-isogeny reach (toy); Smith rational tractable subgroup share 0.50159.
- **`teske_variants_field_types_20261018`.**
  - Prime fields: none of Teske's three ingredients.
  - Binary: the sparse regime is at n = 7 (type-I genus 7, δ = 4, margin +0.5, i.e. +11.5 bits at N = 161).
  - Type-II public sub-descents (n = 5 in n = 15) already beat rho.
- **`ghs-c2pnb`.** A single PoC Python file.
- **ca:** `experiments/ghs-sweeper` (Rust sweeper port).
- **Problem record:** `crypto:problems/ghs-higher-magic-descent-not-executable.md`.

**Verdict.** Only composite-N literature cases are reachable. Formula margins only; no GHS solve.

**Coverage: PARTIAL.** Covered: cover_route_prime_binary, ghs_oakley_extended (genus-16 relation phase 5.2 bits slower), the N = 130 measured trial. Missing: the 321-curve scan table, the MMT correction story, the counterfactual model numbers, generalized GHS, Teske variants.

---

## 14. Binary cubic GHS over GF(4), GF(8), GF(16): the late-October catalog campaign (about 25 dirs, Oct 26–28, Conductor T-53xx..T-56xx)

**What was tried.** An exhaustive classification of which elliptic sources over K = GF(q³) (q = 4, 8, 16) have a genus-3 or genus-4 Jacobian transfer, with explicit maps, and end-to-end attacks against matched source rho on a few.

**Structural results:**
- `ghs_cubic_catalog_q16_20261026`:
  - 8,190 normalized sources, 128 orders;
  - LMFDB has a genus-3 Jacobian in 126/128 cubic Weil-restriction classes (8,142 sources);
  - 5,534 have no direct cubic genus-3 GHS cover even after K-isogeny walks;
  - 5,302 of them pass the simple-root gate;
  - "eligibility bound, not a new attack".
- `ghs_cubic_hess_closure_q16_20261027`: complete Hess genus-3 criterion; residual classes.
- `ghs_q16_simple_restriction_bound_20261027`: any nonzero homomorphism Res E → Jac(C) needs genus ≥ 3 when F_E is irreducible.
- `ghs_q16_odd_isogeny_screen_20261027`: no cubic divisor of F_N mod ℓ excludes every one-step (ℓ,ℓ,ℓ) isogeny.
- `ghs_scalar3_frobenius_gate_20261027`: self-labelled a **rediscovery** of the prime-characteristic scalar-3/Jordan screen in `cubic_extension_transfer_20261025`.
- Cover exclusions:
  - `ghs_cubic_quotient_obstruction_20261026` (order 3974: no separable degree-2 map to an elliptic curve);
  - `ghs_automorphism_screen_q16_20261026`;
  - `ghs_degree3_neighbor_screen_q16_20261026`;
  - `ghs_degree3_q4_screen`, `ghs_degree3_q8_screen`, `ghs_degree3_q8_tail` (orders 526/542);
  - `ghs_q16_4126_degree3_map_20261027`;
  - `ghs_q8_548_geometric_cover_20261026`.
- New witnesses:
  - `ghs_binary_cubic_isogeny_20261026` (GF(4): 2,048 quartics, 55 smooth, 48 Weil matches, 8 classes without a direct genus-3 source);
  - `ghs_binary_cubic_isogeny_q16_20261026` (three class-C quartic witnesses, absent from LMFDB `curves`);
  - `ghs_quartic_new_classes_q8_20261026` (order 548);
  - `ghs_q16_residual_quartic_search_20261027` (30,000 draws);
  - `ghs_q8_quartic_cover_panel_20261026` (Frobenius-cyclic quartic; one end-to-end attack against rho at 0.0005539165 s).
- Transfer-eligible families:
  - `ghs_cubic_tate_bridge_20261026`: orders in [3969, 4225]; the number of curves for which a complete attack beats rho is between 0 and 2,608;
  - `ghs_twisted_v4_triangle_20261027`;
  - `ghs_nonsplit_v4_cubic_20261027`;
  - `ghs_q16_tracezero_genus2_20261028` (structural; no attack).
- Explicit quotients: `ghs_q16_hess_elliptic_quotient_20261027` (elliptic quotients of four Hess genus-4 curves, Lean identities).

**End-to-end attacks vs matched source rho (all lose):**

| study | attack / matched rho |
|---|---|
| `ghs_q16_4126_hess_g4_20261027` | 512–698× |
| `ghs_q16_genus4_frontier_20261028` | GHS/rho CPU 216× at order 3974 (r = 1987; 13 relations; 0.22 s vs 0.00102 s); "Rho wins" on all representatives |
| `ghs_nonsplit_order4222_20261028` | 271× (precomputed-divisor core 241×) |
| `ghs_nonsplit_v4_cubic_20261027` | 225× (rho median 159 steps) |
| `ghs_hess_c_parameter_audit_20261026` | median-cost ratios 2,304 (b = 155, r = 137) and 2,784 (b = 32, r = 139) |

All are Magma wall times at toy q (L0).

**Coverage: ABSENT.** Only `ghs_q16_genus3_subcovers_20261028` is cited. This is a self-contained chapter: an exact classification plus uniformly negative toy attacks. Est. 8–10 pages.

---

## 15. Odd-characteristic cubic and quartic GHS: genus floors and explicit genus-3 builds

- **Genus-3 builds:**
  - `ghs_cubic_genus3_build_20261012`: explicit plane quartic over F_q for E/F_{q³}; Frobenius checked; no DLP;
  - `ghs_cubic_hyperelliptic_genus3_20261013`: m = 3, r = 5 hyperelliptic model; transfer executed with Cantor arithmetic.
- **`ghs_n3_genus3_census_20261024`.** Binary F_{2^{3l}} genus-3 census. The treatise cites the 2^{−l} fraction via ladder3 only.
- **`ghs_cover_census_n4_20261024`.**
  - Genus floors: odd #E gives 13 (37 if j ∈ F_{q²}); E[2] = Z/2 gives 25; (Z/2)² gives 5 (26/28 at q = 3, 158/210 at q = 5).
  - **Two preregistered predictions failed** (21 → 25; {13, 25} → {13, 37}).
  - The treatise gives floor 13 only.
- **`odd_char_descent_genus3_20261014`.** Genus 3 only for (m, r) ∈ {(2, 6), (3, 5)}.
- **`cover_descent_genus_floor_20261015`** (correction to `docs/curves/jacobian-transfers.json`).
  - No GHS cover of E/GF(p^k) with F_p(j) = GF(p^k) has genus below 9/49/41 at k = 6/10/14.
  - Of 846 formula_advantage rows, 599 remain, 41 become marginal and 206 boundary.
  - Curve verdicts move from 848/50/1201 to 601/67/1431.
- **`cubic_extension_transfer_20261025`.** Historical Sage import plus a native follow-up: rational 3-torsion action, 58 stage runs. Complete native transfer open; pcc PCC-20261105-a43948 and -a6723f (development and proposal).
- **`LEAD-TRF-odd-prime-ghs-cover-floor`** (PEXT n = 7). Hypothesis approved, not evidenced; source-binding mismatch (`car:problems/pext-n7-frozen-source-binding.md`).

**Coverage: PARTIAL.** ghs_cubic_cover_census and the n = 5 census are covered. The genus-floor repricing (a correction) and the failed n = 4 predictions are missing.

---

## 16. Plane quartics, Diem IC, Smith, Tian: the n = 3, 4, 5 Weil-restriction Jacobians

- **Covered:** quartic_n4, quartic_n4v2, smith_elliptic_walk, smith_prop24_native, smith_sign_scaling, quartic_pic_canonical, plane_quartic_program.
- **`quartic_n4_la_20261021`** (N4 iteration 2, the step between N4 and N4 v2).
  - B2/B0 0.355 [0.324, 0.392] at q = 4051, 0.224 at 65479, 0.297 at 262111.
  - IC/rho 0.24 → 0.024 (0.02–0.03) at q = 1,048,507 (ℓ ≈ 2.8e17).
  - Exponent 1.045 [1.037, 1.052] against rho 1.476 [1.438, 1.508]; plain rho; shared VM.
- **`diem_quartic_relations_20261018`.** Amendments only. N2 and N3 exhaustion not run.
- **`smith_iso_granularity_20261015`, `smith_class_invariant_20261015`, `smith_quotient_composition_20261016`, `smith_serre_sign_20261016`.** Frozen protocols on whether Smith's full_success is an isogeny-class or Galois-module invariant (p = 7, 11; null bands [0.4545, 0.4611] / [0.4858, 0.4871]). Their results are in PDFs, not READMEs.
- **`weil_restriction_isogeny_census_20261018`** (Theorem 4, [T, E]). For n = 3 a Tian kernel exists iff χ_φ has a root in F_ℓ and every root is a cube, with a Jordan-case condition ℓ ≡ 1 (mod 3). Nonsplit φ never qualifies; scalar φ always does.
- **`tian_n3_quartic_route_20261024`** (Sonnet lead agent).
  - 26/26 flagged prime-order classes have a plane-quartic Jacobian in Res E's class (q = 7…17). Unflagged classes are hit too, so this is not evidence of a Tian kernel.
  - Diem DLP IC on generic quartics up to q = 8191: 180–340 additions vs rho's 5,000–300,000 at w = 0, but it loses at w = 0.1 and 1 (dense LA ~q^1.4).
  - **Unresolved across the band.**
- **`genus4_n4_census_20261024`.** Prime-order E/F_{q⁴}: Res E's class contains a non-hyperelliptic genus-4 Jacobian at every trace (q = 3), 13/14 (q = 5), 20/20 (q = 7). Theorem 6 is an "[agent proof]", unreviewed; no map computed.
- **`humbert_genus5_cover_20261021`.** Quartic-extension genus-5 cover reproduced; "matched rho is faster at every q". Benchmark copy at `benchmarks/20261026-humbert-genus5-cover/`.

**Coverage: PARTIAL.** Tian/Theorem 4, the genus-4/5 Jacobians, quartic_n4_la and the Smith invariant studies are absent. Est. 5–6 pages.

---

## 17. Joux–Vitse genus-3 cover of E(F_{p⁶}): November work

**Covered in the treatise** up to 24 Oct:
- the Nagao arm at 1.59–1.98× pooled rho at p = 1009;
- the sieve arm with comparisons withheld;
- the e2e 0.058 replay;
- sparse FGLM, the n−1 variant, jv_weak_class_law.

**Uncited (supersede or extend the treatise):**
- **`jv_cover_crossover_20261101`** (Nagao arm, p = 1301–1601):

  | p | S/rhodp | S/rhostrong |
  |---|---|---|
  | 1301 | 0.793 [0.587, 1.119] | 1.349 [1.085, 1.699] (above) |
  | 1451 | 0.775 [0.645, 0.970] (below) | 1.063 [0.819, 1.477] |
  | 1601 | 0.587 [0.448, …] | |

- **`jv_cover_negrho_rescore_20261101`** (frozen re-scoring vs negation rho). S/rho 1.405 (p = 1301), 1.160 (1451; lower end 1.020), 0.870 [0.697, 1.183] (1601, unresolved). In textbook F_p mults the cover is more expensive at every prime (3.38, 2.79, 2.09).
- **`jv_cover_fbline_20261102`** (factor-base line search). S_line/S_plain 0.587 [0.530, 0.651] = 1.71× [1.54, 1.89]; the model predicted 0.587; "passes the negation-map rho in group additions". **No VALIDATION.json.** A candidate IC-baseline improvement.
- **`jv_sieve_negrho_20261102`** (**VAL**). The sieve route is below matched negation rho in every cell: ×155 [125, 184] (S, U1), ×64 (S, U2), ×37 (S+, U1), ×15.5 (S+, U2); S/rho U1 0.00738 [0.00563, 0.01021].
- **`jv_sieve_fbline_20261102`.** Frozen; 192 runs; no VAL.
- **`jv_sieve_line_vs_rho_20261103`** (frozen, three primes). Pooled S/rho U2: P 0.0244 [0.0206, 0.0292], H 0.0154, U 0.0141. S+/(rho S+) U2: 0.0624 / 0.0344 / 0.0324. Distance remaining ×41 / ×65 / ×71.
- **`jv_sieve_adds_20261103`** (L8). S+ ratio 0.621 [0.614, 0.629]; S 0.990. The bitop/v1 model effect is smaller: 0.880 or 0.953.
- **`jv_sieve_strong_rho_v2_20261020`** (replication). **Acceptance A4 failed; all decisions withheld** (R0 A/A 0.792 [0.572, 1.073]). Descriptive sieve/R3: 0.647 (p = 503) … 0.00404 (p = 1823).
- **`jv_sieve_prospective_20261023`.** Protocol only.
- **`ic_acct1_cover_bitop_20261021`.** Weak-class row re-priced in bitop/v1, both arms executed. IC/rho 0.0098 [0.0069, 0.0144] at p = 1511 (ℓ = 2^61.4).
- **`ic_cover_weakclass_20261023`** (**VAL**; benchmark `20261029-jv-cover-hilbert-stop`).
  - Hilbert early stop: S_A/S_B 1.848–1.875 over 25 cells (engineering).
  - The as-found B0 (2.53–2.64×) was **relabelling**: native ops ×8.3–8.5.
  - S/rho of baseline A: 1361 → 205 → 35.2 → 7.19, slope −2.27 ± 0.13.
- **`jv_cover_large_prime_20261018`** (lead L6). Negative for p ≤ 12k: LA share 2.5e-3 of S at p = 12000; one large prime saves at most 0.02%.
- **`jv_cover_next_leads_20261102`** (LEADS.md). The sieve, not Nagao, is the cheapest route.
- **Weak-class structure:**
  - `jv_weak_generalization_20261008`: g ∈ {1, 3, 7, 9, 13}; JV complete among 2-torsion covers with g ≤ 3;
  - `iso1_weak_classes_20261007`: norm-one conductor theorem; random full-2 curves in the weak class 0.5908 (p = 11) … 0.6320 [0.6169, 0.6468] (p = 47); census p ≤ 53;
  - `weak_curve_database_20261012`: 55–63%;
  - `LEAD-TRF-teske-trapdoor-fp6-weak-class` (open_unexamined).

**Verdict.** The weak class only, never prime order. Nagao is near parity with negation rho at p = 1301–1601. The sieve is validated (VAL) at 0.007 of negation rho at p = 1511.

**Coverage: PARTIAL.** The treatise's "sieve comparisons withheld" is **outdated** by `jv_sieve_negrho_20261102` (VAL) and `jv_sieve_line_vs_rho_20261103`. Est. 5 pages.

---

## 18. Same-field genus-g embeddings: genus 2, Kummer, pencils, bielliptic, binary genus 2/3 covers

- **Theorem closure:** LEAD-TRF-same-field-genus-g-embedding, DEC-20260902-4c82fd. #Jac = N·#A with #A = Θ(N^{g−1}); genus-g IC costs O~(q^{2−2/g}) ≥ q, above rho's N^{1/2}.
- **Binary genus 2 and 3 (n = 83 and toys):**
  - `ca:research/binary_genus2_speed_20261014`: J(C) 1,160.1 s vs elliptic 9.27 s at 10,442,347 equal steps ("the transfer costs, never gains");
  - `crypto:binary_genus2_cover_20261014`, `binary_genus2_speed_20261014`: PDFs only;
  - `ecdlp_surfaces_n83_20261012`: correctness only;
  - `n83_genus3_image_solver_20261031`: 400/400 destination attempts verified, 720 replays identical; generic destination solvers only; gap about 70.17 exponent bits;
  - `n83_jacobian_projector_20261031`: structural.
- **Odd characteristic:**
  - `bielliptic_quartic_20261005` (quartic-ic line relations);
  - `enge_primefield_cover_20261024`: C_{d,s}: y² = f(t^d + s), genus (3d − 1)/2; Enge's bound with d = Θ(log p) has an exponent above rho's;
  - `hyperelliptic_cover_infrastructure_20261006` (correction on one-infinity models);
  - `genus2_external_gate_20261021`: Lindner's upstream add(D, D) returns the identity on a non-2-torsion D, so it is unusable without dispatch;
  - `jacobian_op_count_20261014`: counted Cantor vs the 10g² model, 10,400 rows;
  - `prime_ic_jacobian_benchmark_20261101`: protocol, no rows.
- **Kummer collector** (Scholten cohort, 21 models, 1,260 targets):
  - `kummer_coordinates`, `kummer_theta`, `kummer_walk`: gates;
  - `kummer_recover_20261022`: published Eq. (11) produced 1,257 incorrect signed divisors, which were corrected;
  - `kummer_differential_walk`;
  - `kummer_differential_collector_20261022` (**VAL**): full-cold candidate/rho 8.96–55.94; ratios 14.36 (uncached) and 12.95 (cached).
- **Pencil** (point-driven pencil relations):
  - `pencil_sparse_20261022` (**VAL**): 0/19,440 price cells below rho;
  - `pencil_markowitz_20261022` (**VAL**): ratio 1.204855 vs peel; 0/36 below peel or rho; failure code sparse_matrix_cap;
  - `pencil_large_prime_20261029` (p739 gate, 60 workloads);
  - engineering: `pencil_early_rejection`, `_progress`, `_runner_diagnostics`, `_lifecycle`, `_source_work_inventory`, `_ci_reconciliation`.
- **ecbench:**
  - `hyperelliptic_ecbench_20261024` (round 16): rho.negation/plain 0.840 / 0.895 / 1.394 across the band, unresolved;
  - `ecbench_jacobians_transfers_20261025`:
    - genus 2: ic.hyperelliptic/rho 1.473 [1.158, 1.874], above rho at every weight; bsgs.negation 0.737 below;
    - plane quartic: bsgs/rho 0.497;
    - transfer.ghs_cover/rho 6.62 [5.13, 8.54];
    - "new capability, not an improvement".
- **Problem records:** `crypto:problems/genus2-*`, `prime-cover-projected-relation-cost-missing.md`, `car:problems/prym-assisted-source-labelled-router.md`, `p256-cover-target-component-solver.md`.

**Verdict.** Closed by theorem on prime fields. Every measured collector lies above rho.

**Coverage: ABSENT.** One paragraph cites RESEARCH_HYPERELLIPTIC_IC_RHO. Est. 6–8 pages.

---

## 19. Weil descent / decomposition over F_{q^n}, n = 2, 4 (overlaps the extension-field family)

- **`weil_descent_angles_20261018`.**
  - Coverage is Poisson for odd characteristic, n = 2…5 (|z| ≤ 0.94).
  - Proved exception: binary n = 2 has π = 3/8 vs 0.3935 (z = −4.72), via a trace-coset obstruction.
  - Genus-2 IC at n = 2: about p/3 Jacobian ops vs rho 0.886·p/√h; cost ratio spans 0.51–2.83 across the band (a constant-factor race).
- **`gaudry_quartic_d4_ecbench_20261013`.** IC 6.7·10^8× rho on E(F_{29⁴}) (S 7.17·10^8 vs 1.065). Covered in extension.tex.
- **`gaudry_quartic_speed_20261013`:** 6× wall, same arithmetic. **`gaudry_quartic_replay_20261013`:** shape replay a 2–5% lever (decision rule needed 10×).
- **`weak_subfield_hyperelliptic_decomposition_20261012`.** Bhati–Singh weak curves are subfield curves; all ten published relation points are F_q-rational on C0.
- **`p192_extension_cover_20261012`** (1,849 files). P-192 over F_{p⁶}: every Legendre cross-ratio of the nonzero 2-torsion has norm −1 to F_{p²}; residual hit bound below 2^{−131} per query.
- **`weil_factor_composition_20260914`.** Stage costs only.

**Coverage: PARTIAL.** The D4 system is covered. Poisson coverage, the binary n = 2 exception and the P-192 extension screen are missing.

---

## 20. Quotient maps, symmetry quotients, EcGFp5

- **`ecgfp5_quotient_solve_20261101`** ((Z/2)⁴ ⋊ S5 quotient, m = 5, toy p).
  - D = 2^16 = 65,536, exactly the Bézout bound.
  - c_cyc = 2^46.30 / 2^46.56 / 2^46.63 at p = 4111 / 262231 / 33554371.
  - G1 verdict CLOSE at all primes (margins 0.30–0.63 bits).
  - About two-thirds of c is msolve's parametrisation.
- **`ecgfp5_quotient_bw_finish_20261102`.**
  - Block-Wiedemann finish: 1.83× / 1.97× / 1.72× under W0.
  - "Measured improvement over the per-system IC baseline (W0) at all three primes… new per-system baseline." Engineering class.
  - No VAL.
- **`ecgfp5_quotient_hnf_finish_20261102`:** feasibility only. **`ecgfp5_quotient_presentation_20261102`:** preregistration (F4 is about 3/4 of c after BW).
- **Toy quotients:**
  - `toy_symmetry_quotient_20260924`;
  - `toy_quotient_controls_20260924` (GF(256));
  - `frobenius_quotient_followup_20260926`: SAT numbers supplied as a note and **not reproducible**; "no downstream claim may treat them as" evidence;
  - `quotient_calibration_20261025`: source preflight;
  - `n83_top_quotient_20261028`: 48 regularity certificates, degree ≥ 28.

**Coverage: ABSENT.** EcGFp5 appears only in summer.tex. bw_finish is a claimed measured improvement and needs a section, possibly in the Gröbner chapter. Est. 3 pages.

---

## 21. Cofactor / torsion quotients and lifts

- **Curve25519 miniature, k = 20** (component studies, no rho arm):
  - `curve25519_cofactor_algebraic_20261102`: lifts = 1;
  - `curve25519_cofactor_shared_20261102`: one shared F4 system for 8 lifts costs **30.7×–628×** more multiply-adds; solving degree rises 5 → 8;
  - `curve25519_scale_lifts_20261105`: k = 24, 28 ratios 1.00–1.013, so k = 20 conclusions carry; k = 32 refused;
  - `curve25519_torsion_leads_20261103`: T1–T3.
- **`ic_cofactor_clear_ab_20261103`.** Frozen rule verdict **invalid_controls**: the positive-control lower limit 1.149 < 1.2.
- **`ic_m2_torsion_n37/n41/ratio_n37_20261101`.**
  - Torsion anchors: 0.134 [0.082, 0.205] off/on at n = 37.
  - **Corrected** on the bootstrap recheck: on/off 7.46 [4.90, 11.92] at d = 12, a penalty.
  - n = 41 0.975 [0.641, 1.420]; n = 31 13.3× more trials.
  - At the N83 ratio the proxy shows a penalty.
- **`car:` LEAD-TRF-cover-catalog-jacobian-transfer** (open_blocked). 2,572 maps / 977 EC1 bindings classified; 663 of 664 bindings lack proof-grade certificates.

**Coverage: ABSENT.** Covered elsewhere only as torsion_auxiliary_inputs. Est. 2 pages.

---

## 22. Transfer methodology, audits, weak-curve catalogue

- **`transfer_audit_20261011`** (Lead 5). No toy-Koblitz candidate is priced below rho. **v2 amendment 3 corrections:**
  - the baseline was 0.5 bit too low;
  - the pairing verdict was uncalibrated;
  - `computational_advantage = "refuted"` was over-claimed;
  - v1 certificates were overwritten in place (restored under `v1/`).
- **`transfer-methodology-validation-20261013`.** Defines an operational "weaker destination" and a three-arm cold net-cost verdict. Two fixtures support a valid subgroup transfer; two remain unknown.
- **Weak-curve catalogue:**
  - `weak_curve_catalog_20261010`: 21 families, 814 models;
  - `weak_curve_properties_20261010`: 360 certified 2-volcano levels; 219 of 512 large sources are certified 2-surface models;
  - `weak_curve_properties_lean_20261011`: 48 theorems;
  - `weak_curve_database_20261012`: sqlite;
  - `weak_curve_conjectures_20261012`: seven open hypotheses;
  - `weak_curve_case_index_20261012`: 36 Iijima–Momose–Chao cases;
  - `weak_line_7_4_followup_20261013`.
  Only `weak_curve_leaderboard` is cited.
- **LEAD-TRF-curve-parameter-transfer-audits** (closed_known). NIST seed provenance (`nist_seed_provenance_20261016`); the P-521 n + 1 cofactor resolved; one NIST extension-twist cofactor unsplit.
- **`integer_factoring_transfer_20261101`.** Imported factoring mechanisms. The collision idea runs at 3.8–4.9× counted rho (17.7× in another configuration); adapter correctness not established.
- **`ic_weak_assumptions_20261102`.** Literature-complete first pass.

**Coverage: PARTIAL.**

---

## 23. Teske-type trapdoors

- **LEAD-TRF-teske-secret-isogeny-trapdoor** (open_next_step).
  - EV-ECTD-8c098d: five classes homogeneous at 40-bit N (weaken). **A fabrication defect in the rho-substitution prose was disclosed.**
  - EV-ECTD-4c8a3b: vertical screen invalid (identical meter on all 32 curves).
  - EXP-ECTD-0cf008: all trials infrastructure_error.
- **`teske_variants_field_types_20261018`** (§13): no prime-field variant exists.
- **LEAD-TRF-teske-trapdoor-fp6-weak-class** (open_unexamined): JV weakness is not class-invariant (55–63%), so a trapdoor is structurally possible.

**Coverage: ABSENT.** The treatise mentions Teske only via Shantz–Teske and Hess–Menezes–Teske. Est. 2 pages.

---

## Retractions and corrections to carry into the treatise

1. Conductor-gap CM pairs: banner withdrawn (SHF-20261017-e4d7f6) and corrected (SHF-20261028-0f3f64).
2. Conductor round 2 A0: floor/crater 3.10 → 1.00.
3. GHS magic number (MMT correction 2026-10-13): N = 130 genus 128 → 255; margin −23.91 → −4.71 bits (model) → −1.83 (measured; in the treatise).
4. cover_descent rows repriced at the genus floor: 846 → 599 formula_advantage.
5. Isogeny-loop GLV round-1 Vⁿ erratum.
6. isogeny-volcano-cost v1: rho.cubic/quartic setup omitted; j = 0 control 0.697 → 0.716.
7. ic_cover_weakclass B0 classified as relabelling.
8. ic_m2_torsion n37: 0.134 gain reversed to a 7.46 penalty.
9. transfer_audit v2: baseline +0.5 bit; "refuted" over-claimed.
10. jv_sieve: Oct 19 review withheld; V2 A4 failed. Superseded for p = 1511 by the VAL jv_sieve_negrho. Treatise text is stale.
11. ghs_cover_census_n4: two preregistered predictions failed.
12. CM canonical lift L9: load-bearing argument corrected (CORR-20260810-930c42).
13. Provenance and self-labelling flags:
    - EV-ECTD-8c098d fabrication defect;
    - DEC-20261023-26a6de mis-citation (AES paper);
    - synthetic_volcano is invented data;
    - frobenius_quotient_followup SAT numbers are unreproducible;
    - ghs_scalar3 is a self-declared rediscovery.
14. curve25519_isogeny_seed: V2 failed (gain not established in-session).
15. kummer_recover: the published Eq. (11) gave 1,257 wrong signed divisors; corrected.

---

## Coverage table

| # | line of work | coverage | est. pages it deserves |
|---|---|---|---|
| 1 | Prime-field isogeny transfer to weak curves; standard-curve walks; b-battery | PARTIAL | 4 |
| 2 | Volcano levels and generic rho (v1 cost, tall layers, Koblitz layers, ECC2K-130 class, curve_traits) | PARTIAL | 6 |
| 3 | Conductor gaps and fibre-aware IC (correction chain) | PARTIAL | 5 |
| 4 | Isogeny-derived factor bases | PARTIAL | 2 |
| 5 | Isogeny computation engineering (√élu, CRT modpolys, large degrees) | ABSENT | 3 (appendix) |
| 6 | Isogeny-loop GLV/GLS endomorphisms | PARTIAL | 1.5 |
| 7 | Quadratic twists (twist_ecdlp E0–E6, yield, census) | PARTIAL | 4 |
| 8 | Lift-dense / twisted-line bases follow-ups | PARTIAL | 2 |
| 9 | Order-24 twist automorphism (E29/E34/E41) | ABSENT | 2 |
| 10 | MOV/FR, embedding degree, self-pairing, second torsion, pairing recost, Miller cocycle | ABSENT | 3 |
| 11 | Anomalous/Smart, p-adic sections, CZLIFT, H109/H110 | ABSENT | 3 |
| 12 | Global lifts / xedni / JKSST audit / kernel-of-reduction | PARTIAL | 3.5 |
| 13 | Binary GHS screens on registry curves (scan, MMT, N = 130 model, generalized GHS, Teske variants) | PARTIAL | 5 |
| 14 | Binary cubic GHS catalog campaign over GF(4/8/16) | ABSENT | 9 |
| 15 | Odd-char cubic/quartic GHS genus floors and explicit genus-3 builds | PARTIAL | 3 |
| 16 | Plane quartic / Diem / Smith invariants / Tian n = 3 / genus-4, -5 Jacobians | PARTIAL | 6 |
| 17 | Joux–Vitse cover, November (crossover, fbline, sieve VAL, Hilbert stop) | PARTIAL (stale) | 5 |
| 18 | Same-field genus-g embeddings: genus 2, Kummer, pencils, binary genus 2/3, ecbench Jacobians | ABSENT | 7 |
| 19 | Weil descent n = 2, 4 coverage, binary n = 2 exception, P-192 extension screen | PARTIAL | 2.5 |
| 20 | Quotient maps / EcGFp5 (BW finish improvement) | ABSENT | 3 |
| 21 | Cofactor/torsion quotients and lifts (Curve25519, m2 torsion) | ABSENT | 2 |
| 22 | Transfer methodology, audits, weak-curve catalogue | PARTIAL | 3 |
| 23 | Teske-type trapdoors | ABSENT | 2 |
| | **Total** | 8 ABSENT, 15 PARTIAL, 0 COVERED | **about 96** |

Note: COVERED sub-items (lemma `lem:conservation`, twist obstruction, function-field xedni, quartic N4/N4 v2, Smith walk, ghs3_vs_rho, volcano census) are folded into the PARTIAL rows above.

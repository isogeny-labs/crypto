# Exotic-family coverage digest (survey agent "exotic", 2026-11-06)

Read-only. Sources read with `git show origin/main:` (crypto, crypto-autoresearcher) and `git show HEAD:` (cryptanalysis, branch `claude/cryptanalysis-repo-setup-p59kek`, c273e62). Repo prefixes: **C** = crypto, **CAR** = crypto-autoresearcher, **CA** = cryptanalysis. Treatise = C `papers/ecdlp-index-calculus-treatise/sections/*.tex` (origin/main).

**Standing / sabotage rule.** Only one item in this family carries a pcc claim: `PCC-20261104-1df881` (hashed-ECDL entry-cost source audit), standing **unverified**. Every other number below is, under the repository's 2026-11-04 rule, an **unverified assertion** quoted from the named file. Where a study names its own validation (frozen protocol, independent validator, red-team) that is recorded, but it is not a pcc standing. No item here has a `VALIDATION.json` gate row; none is on the IC leaderboard.

**Atlas footprint.** `CAR:knowledge/leads/ecdlp/exotic/` has 32 lead records. `coverage.tsv` maps roughly 420 ledger records to EXO leads; largest: methodology-instruments 114, coordinate-statistic-leakage 64, lossy-object-propagation 50, four-list-ktree-join 46, curve-model-representation 36, invertible-hint-subset 22, padic-jet 16, cryptoscale-screen 14, augmented-oracle-ggm 11.

**Treatise keyword check** (grep over all sections): zero hits for eigen, machine learning, neural, reinforcement, dataset, MaxSAT, kissat/CaDiCaL, Akshima, Bochum, tensor, contraction, treewidth, k-SUM, Winterhof, quantum, character sum, QTT, holant, elliptic net, list decoding, Wagner, hidden number/HNP, Hessian, Gowers/quadratic phase, invertible, split Jacobian. One-line or one-paragraph hits only for xedni, GGM/generic group, H-PSEUDO, jet/canonical lift/Teichmüller, Cheon, Coppersmith, lit2026, Mahalanobis, Edwards, WDSat.

---

## A. Machine learning and learned components

### A1. ML ECDLP/PDP dataset campaign (`ml_dataset_*`, about 45 dirs, 2026-10-27 to 11-01)
- **What.** A native `crypto dataset` generator and custody pipeline for a corpus meant to train models on ECDLP/PDP: Koblitz, ordinary binary and prime curves, paired twists, arities m = 2..15, n = 23..31 and 47..61 (p = 2^61-1 at n = 61), multiple generators and keys, every native method. Design: C `research/ml_dataset_20261028/design-report.typ`, C `docs/ml-dataset/DESIGN.md` (committed before implementation at a5062b8ce4f), `docs/ml-dataset/training-protocol-20261101.md`.
- **Numbers.** Default plan 165,150,720 *planned* method cells (24 x 14 x 3 x 2 x 16 x 8 x 64 x 10); explicitly "planned cells, not completed solutions". Campaign inventory (C `research/ml_dataset_campaign_inventory_20261101/README.md`): 124 selected terminal method cells across seven cohorts, e.g. Koblitz n23 m3-15 26 cells (4 finite-domain negatives, 22 timeouts); binary n23 26 cells (6 negatives, 20 budget-exhausted); Koblitz n47 m4 16 verified; prime61 m2 4 cells (2 verified, 2 root timeouts); n26-31 m2 36 cells (35 known-answer-replayed, 1 timeout); separate 252-cell compatibility matrix. Inventory states it "is a coverage inventory, not a pooled estimate or a training set".
- **Label design points worth a treatise section:** known-answer is not recovery; a PDP witness is not an ECDLP solve; first/last fall degree are properties of stated systems, a solver's max degree is separate; censored outcomes (timeouts, unsupported) must be kept; splits must group presentations, signs, Frobenius orbits and twists.
- **Open problem.** C `problems/ml-ecdlp-dataset-durable-launch.md`: worker lacks durable S3 publication and launch admission.
- **Verdict.** Infrastructure; no model result. **IC-relevant** (labels are IC phases). **ABSENT.**

### A2. ECDLP dataset models, development pilot (C `research/ecdlp_dataset_models_20261102/REPORT.typ`, 2026-11-02)
- Native Rust evaluator/trainer for the protocol's resource/success model; frozen protocol `655f52ecf` (sha256 5740dd6d...). Development-only by its own protocol: may not select a winner or estimate generalisation.
- 960 cells (n in {23,25,27,29,31}, m in {2,3}, 3 families, 2 twists, 2 seeds, 2 keys, 4 arms: enumerate, bsgs, root, double-large-prime; work limit 100,000). Outcomes: 665 verified, 96 partial, 51 budget_exhausted, 8 timeout, 120 unsupported, 20 error (all `root`, prime, m = 2, trace > 64 MiB). 425 independently solved. 819 training rows. Leakage audit: 0 forbidden keys, 0 scalars found.
- Pooled Brier (leave-one-field-size-out): completion majority 0.0603, logreg 0.0205, MLP 0.0352; permuted 0.0527/0.0528/0.0565. Leak canary learned to Brier <= 0.0001.
- Reading in the report: the arm nearly fixes the label (root always censored; enumerate completes 240/240, recovers 0; bsgs 237/240). Only `double-large-prime` has signal: logreg 0.050 vs majority 0.192, permutation removes it (0.168); failures cluster by curve, not n. Labelled a hypothesis.
- Deviations disclosed: ecdlp endpoint added after first 118 cells, not in frozen protocol; ran in session container, not EKS.
- **IC-relevant (weakly).** **ABSENT.**

### A3. ML-aided SAT ceiling for PDP (C `research/pdp_ml_sat_ceiling_20261027/README.md`, 2026-10-27)
- Closed by preregistered rule. Symmetrised S4, m = 3, native CDCL+XOR, Trimoska corpus. Three levers: triage, cutoff, branching order.
- 4,200 labels agree with exhaustive search. Refutation costs 0.963 (n15l5) and 0.998 (n17l6) conflicts per sorted factor-base triple; every refutation within 4.1% of the family median. Perfect UNSAT predictor ceiling: 5.2-9.0x relations per conflict. Test AUC within noise of 0.5 for every feature group (e.g. n15l5 F-x 0.515 [0.478, 0.552]); permuted 0.45-0.54; positive control 0.72-0.79. Only CI excluding 0.5: telemetry@512 0.556 [0.518, 0.599], gain 0.99x. Train gains up to 4.7x fall to ~1x held out.
- **Verdict:** "the ceiling is real, the signal is not". **IC-relevant (strongly: the decomposition oracle is an enumerator).** Treatise prog_binary.tex says "SAT, SMT and WDSat are enumerators" but does not report the ML ceiling. **ABSENT** (PARTIAL context).

### A4. Learned WDSat branching, learned pivot selector, prefilter, degree surrogate (CA `experiments/ideas_learning_20261018/REPORT.md`; C `research/ideas_execution_20261018/EXECUTION.md` items 8-12, 2026-10-18)
- Item 8: 88 WDSat calls (upstream 61c6ff3f), symmetric S4. Held-out total conflicts on 8 targets: paper default with symmetry 3,092; block core without symmetry 16,097; learned static core without symmetry 16,097; interleaved 16,785; random 16,676. **Caveat for the treatise:** the learned policy selected the ordinary ascending order, i.e. it equals the block-core arm; the 16,097-vs-3,092 gap is mostly the disabled symmetry code, not a learned-policy effect. The atlas cites it as "the only learned-branching negative".
- Item 9: learned F4 pivot selector returns constant min-weight; no improvement. Item 10: restricted predicate search re-selected the incumbent (820 counted units both). Item 11: 7,254 exact subgroup targets, 27 cells; one-class prefilter rejects genuine decompositions, disabled. Item 12: degree surrogate on 1,718 records, group holdout MAE 0.10041, honest 90% group interval unbounded.
- **IC-relevant.** **ABSENT.**

### A5. RL pair selection in Buchberger for Semaev systems (C `research/gbrl/PAPER.md`, undated)
- PPO pointer policy (~10K params). Self-reported: cyclic-6 steps 379 -> 358.0 +- 2.0 (-5.5%), arith ops 14,864 -> 13,508 +- 136 (-9.1%); Semaev m = 2 held-out: steps -3.9%, arith ops +5.4%; strictly dominates sugar and normal on 15/30 held-out; out-of-distribution |F| = 20 both metrics on all 3 seeds; Semaev m = 3 held-out |F| = 6 worse (496/28,155 vs sugar 459/24,252). Toy ECDLP at p = 1,000,003, subgroup 1571. Buchberger only; F4 scaffold not implemented.
- No frozen protocol or ecbench record found; paper-style self-report. **IC-relevant (engine heuristic).** **ABSENT.**

### A6. Other learned-model plans (no data)
- C `research/ec_hardness_predictor_20261010/README.md`: geometric ML hardness predictor over isogeny classes; "plan frozen 2026-10-10; nothing run"; dataset 1,608 + 300 holdout curves referenced in an external learner.
- CAR `analysis/rl-isogeny-search/DESIGN.md`: RL isogeny-class search instrument, smoke runs only.
- CAR `experiments/EXP-ECDLP-9f7772/specification.yaml`: frozen trained candidate (x_mod_7 / exact / beam 8) on 48 fresh 12-18-bit curves with honest rho fallback; approved, no evidence.
- CAR `ledger/proposals/IDEA-20260806-94676a.yaml`: representation learning of the group law; costed: an exact latent linearisation is the DLP.
- L05 policy pilot (C `research/ic_prime_eth_leads_20261031/leads/L05/analysis_b8.txt`): renaming inert in kissat by construction; 350 rows; slope 2.17 [1.93, 2.38] bits per window bit (SAT-only 1.41); "Do not fund an LLM search on renaming".
- Lead disposition: `LEAD-EXO-ml-learned-models` open_next_step (learnability control with planted leak and relabelling null at N up to 2^24).
- **ABSENT.**

### A7. Not ECDLP: neural distinguishers and symmetric ML
- C `research/neural_distinguishers_20261102/REPORT.typ`: Speck32/64 and AES-128, frozen protocol d7e1cc432, n_test 131,072. MLP: Speck r5 0.8775 [0.8758, 0.8793], r6 0.6938, r7 0.5190 [0.5163, 0.5217]; AES r3: only difference-table NB distinguishes (0.5221); AES r4 none. C `research/aes_neural_distinguisher/`, `research/aes_distinguisher_20261017/`.
- **Unrelated to ECDLP IC.** ABSENT, correctly. At most a footnote on the lab's ML control discipline.

---

## B. Tensor contraction, hashed factor bases and the "ETH is false" route

### B1. Hashed-factor-base tensor ECDL (Akshima et al., Bochum/MPI v0.2, pinned 1ee44e56), conditional on OpenAI "The Exponential Time Hypothesis is False" (Oct 25, 2026), Appendix D.1
- Lead `CAR:knowledge/leads/ecdlp/exotic/LEAD-EXO-hashed-factor-base-tensor-ecdl.yaml` (open_next_step, 2026-11-02). Literature notes CAR `knowledge/literature/KN-LIT-eb8749.md` (Bochum), `KN-LIT-caa51d.md` (openai/math; "unverified proof").
- **Prototype** (C `research/btn_ecdl_20261029/README.md`, 10-29): GF(2^5) count equals brute force (42 = 42, 40 = 40); GF(2^7) count 112 = 112 with min-fill width 34 (vs 23 at n = 5), 336 s with cutset conditioning; toy m = 2 IC on GF(2^5), r = 11, 17 relations, log recovered. Dropping P2 membership gives 237 instead of 112 (recorded error).
- **Native engineering** (10-29 to 11-04): `hashed_ecdl_speed_20261029` (exact primality, functional witness; 34 native tests; matched perf blocked), `hashed_ecdl_hash_prime_20261030` (corrects a user attribution: 54-minute setup hang was AKS primality above u64, not Fourier-network construction), `hashed_ecdl_primality_fast_paths_20261030` (13 strong witnesses below 3317044064679887385961981; bounded Pocklington), `hashed_ecdl_cpu_20261030` (two matched panels of 120 executions each, byte-identical certificates; medians p65521 m3 tau80 45.99 ms -> 3.90 ms, p65521 m4 tau64 80.54 ms -> 4.12 ms; descriptive L0 on EPYC 9R45 EKS VM), `hashed_ecdl_finite_size_20261030` (proposals only), `hashed_ecdl_leads_20261030` (assessment: priorities are independent validation of the contraction route and complete-cost evaluation), `hashed_ecdl_counts_20261031` (counts-only protocol v2; v1 withdrawn before data; L2 -> L0 forced because rho solves < 1 ms grade L1), `hashed_ecdl_entry_cost_20261104` (**PCC-20261104-1df881, unverified**: source-inspection map of entry traversal; D.1 permits exponential entry-work allowance). CA `docs/hashed-ecdl.md` (native tensor implementation, no performance claim).
- **Key result:** ECC2K-130 and ECCp-131 attempts exceed their exact-leaf budget without a relation; no runtime, S or S/rho. Open: C `problems/hashed-ecdl-tensor-backend.md` (independent correctness/complexity audit).
- **IC-relevant: yes, directly (the only claimed subexponential prime-field route in the programme).** **ABSENT.**

### B2. ecdl-tensor-ic Rust port and speed instrument (C `research/ecdl_tensor_ic_rust_port_20261029/`, `research/ecdl_tensor_ic_speed_20261030/`)
- Port of the Go `ecdltensor` prototype: 19 Go tests ported and passing, `ecdlscale` output byte-identical. Speed: ecbench arm `ic.tensor_contraction_v1`; prediction S grows ~ r (one relation ~ |F1||F2| ~ 2r additions, ~2(2r)^(1/2) relations); 16-bit solve 417 s descriptive. "Instrument ready; the measurement has not run yet."
- **ABSENT.**

### B3. Twelve prime-field leads under the ETH route (C `research/ic_prime_eth_leads_20261031/SYNTHESIS.md`, `leads/SKEPTIC_DIGEST.md`, 10-31, reconciled 11-02)
- Verdict: "Nothing moved the exponent question for prime-field ECDLP index calculus." Two skeptics per lead.
- L01 char-2 gadget SAT slope ~2.58 bits of conflicts per window bit (bar 0.5; standard 3.16 [2.95, 3.44]); CPU 3x worse at l = 7. L03 sparse-parity bases paired ratio to null 1.000 +- 0.0003 (b <= 22). L04 backdoor offset 2 log2(cap) - 1, no backdoor below (m-1)s at s <= 7. L05 above. L06 MMD+ lower bound 15..51 vs enumeration 2..10.7 at b <= 16 (on a Python structural model, not a dump). L07 1.000 +- 0.005; shared negation-closed x-window 0.713/0.833/0.916 at m = 4. L08 Frobenius fold 2^65.5 -> 2^58.5 relations vs rho floor 2^60.81, each relation ~2^(1.8e5) at c = 1; corrected margin 1.4x not 3.42x. L09 no encoding D.1-admitted (margin -16.1..-16.3 bits of log2 lambda; needs lambda >= 2^85); s/sqrt(log2 s) 37x to 41,000x the enumeration exponent. L10 Lean filings refuted as fileable (7 gaps). L11 pc/school 0.78 [0.67, 0.92] at b = 9 only. L12 clause retention 1/R = 40x but setup uncharged; refuted.
- **Corrections recorded:** "never beat rho whatever D.1's constant is" wrong as stated (Lean proves g > 16c; threshold log2 b > 256c^2); "~10^5 b edges per addition" is 9-30x high (3,364 edges per slot-bit freed binary, 11.9k for m = 2 prime at b = 131); m = 3 prime SAT censored above b ~ 12. Lean: CAR `docs/reports/ic-prime-eth-leads-20261102/rebuild_IcTensorRoute_20261102.txt` (standard axioms, no sorry; covers cost-model arithmetic only). Clustering loophole closed by non-admission (CAR `.../clustering_closure.md`).
- **IC-relevant.** **ABSENT.**

### B4. Tensor-Boolean "breakthrough" experiment programme (C `research/tensor_boolean_breakthrough_20261031/`, 10-31 to 11-01)
- Six gated experiments E0-E5 + E1S (SAT/BDD triangulation). E0 canonical IR and mutation gate implemented; E1 exact boundary-tensor rank census core implemented; production semantic cuts and equality-signal quotient implemented with four frozen natural cells (C `research/prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md`): legacy raw-edge contractor infeasible on F5, minimal typed rewiring leaves conditioned ranks unchanged. `RIGOROUS_BOUNDS.md`: nonzero-minor certificate gives a finite rank lower bound; uniform exponential family lemma open. E1 protocol still draft.
- **ABSENT.**

### B5. Prime-field tensor continuation: one-shot bet, packet variants, common kernel, jets, pole anchor (C `research/prime_one_shot_20261029/`, `prime_tensor_variants_20261030/`, `prime_common_kernel_20261030/`, `prime_jet_completion_20261030/`, `prime_pole_anchor_20261030/`)
- One-shot bet: growing-arity divisor-coefficient decomposition with shared quotient/remainder towers; 10-30 update records prior art: public preprints claim subexponential prime-field ECDLP via Boolean tensor contraction with bound 2^{O(b/(log b)^(1/4))}, b = ceil(log2 p). Variants change packet action and coefficient prime; "does not improve that exponent". Each README: "An essentially new sub-exponential prime-field ECDLP algorithm remains unproved."
- **ABSENT.**

---

## C. Generic-model, oracle and query-bound lines

### C1. Augmented oracles and GGM simulability (Aug to 2026-11-04)
- KN-FIND-002, -b7e091, -194294 (jet, endomorphism, incidence, halving simulable; halving = x-oracle); EV-ECDLP-fd61e4 (x-oracle yield = exhaustive yield). 11-04 owner split into `LEAD-EXO-augmented-oracle-ggm` (closed_obstructed: label-valued and public-data oracles) and `LEAD-EXO-coordinate-revealing-oracles` (open: x, y, full point, x-halving, field-valued jet and net, threshold C_t, degree-D tests; "GGM gives no bound"). Red-team CAR `docs/reports/leads-round2-20261104/augmented-oracle-review.md`; Lemma 3 (CAR `docs/reports/holant-h1-ggm-lemma-20261103/README.md`): two x-queries give the log on any curve. CORR-20260727-003 withdraws the jet O(1) theorem claim. EV-GGM-002 supersedes EV-GGM-001. Toy split saving B/A 0.0557 (preliminary).
- Treatise obstructions.tex §"Oracle-augmented index calculus stays generic" and summer.tex cover the August closures and C_t. **PARTIAL:** missing the 11-04 split, the observation that coordinate-revealing oracles are outside the GGM, Lemma 3, and the jet correction.

### C2. Low-degree query lower bound (GGM + degree-D coordinate tests) (11-01 to 11-04)
- Leads `LEAD-EXO-low-degree-algebraic-query-bound`, `LEAD-EXO-low-degree-query-lower-bound`. Conjecture H1: success <= O(T^2/N + T D / sqrt N). Desk note CAR `docs/reports/ecdlp-leads-desk-20261104/low-degree-desk.md`: H1 false under full rank, H1' proved, Kohel-Shparlinski verified. Exact rank experiment CAR `docs/reports/lowdeg-rank-20261104/README.md` (frozen LOWDEG-RANK-20261104): full rank in 39/39 cells, p = 251..2039; attack spends 180-490 group ops vs sqrt N 32-45, so no toy-size violation; would beat sqrt N/D only for D >= 3 and N ~ 2^28 (D = 5) to 2^48 (D = 3); building the test costs ~N^omega. Corrected after independent validation.
- **IC-relevant (bounds what algebraic factor bases can be).** **ABSENT.**

### C3. lit2026 W3: SGGM, non-adaptive preprocessing and multi-instance bounds against every IC configuration (C `research/lit2026_w3_ggm_audit_20261011/README.md`, 10-11)
- Papers: Corrigan-Gibbs-Henzinger-Wu ePrint 2026/384 (SGGM, T = Omega(min(sqrt q, 1/delta))), Dinur-Keller-Marmor arXiv 2505.00894 (non-adaptive S*T >~ N), Akshima et al. ePrint 2026/2253 (multi-instance). The EC section of 2026/384 could not be extracted (gap stated).
- 208 configurations, 0 below any bound. Closest: `icv1-fp14-tm43-158b6914/mitm_m2_negfold_walk_balanced` at 38.9x; balanced m = 2 prime rows 39-52x. 122/208 rows have delta_upper = 1 (bound certifies nothing; the SGGM treats decomposition search as free). 13 rows above counting ceiling up to 1.2x, none significant (min p 0.0087).
- Treatise cites lit2026 W4, W5, W8, W10, W11, W12 but not W3. **ABSENT** (high value for the lower-bounds section).

### C4. k-SUM: conjecture-level obstruction and k-sum tables (generic track)
- Lead `LEAD-EXO-ksum-conjecture-s5-oracle` (proposal, 11-03; CAR `knowledge/leads/ecdlp/notes/exotic-leads-20261103.md`): Tier X on prime fields needs a 4-SUM oracle on S_B = {P : x(P) < p^(1/5)} in time << sqrt|S_B|; under the k-SUM conjecture structureless k-SUM needs n^ceil(k/2); nothing measured.
- k-sum table engineering (C `research/prime_ksum_table_20261028/`, `prime_ksum_k5_20261028/`, `prime_ksum_vs_incumbent_20261029/`, `prime_ksum_pooled_20261029/`): k = 5 vs k = 4 S 8.51 vs 11.56 at 40 bits (0.74x); fitted exponent 36-48 bits 0.547 [0.543, 0.554] vs 0.565 [0.560, 0.571]; 9.7x rho at 40 bits, 11.0x at 48. Pooled on 16 fresh curves vs tuned incumbent: 0.700 [0.682, 0.718], 0.446, 0.324, 0.217 [0.212, 0.222] at 20/24/28/32 bits; 6.0-7.3x rho-neg; status UNQUALIFIED (one host, no claims record). Generic track: bounded by BSGS. (Also in the curve25519 autolab pcc claims, which are not this family.)
- Treatise distance.tex covers k = 4 only. **PARTIAL** (k = 5 and the k-SUM obstruction missing; likely shared with the "generic" survey agent).

### C5. Four-list / Wagner k-tree with sum-compatible filters
- CAR `analysis/o2-sum-compatible-filters/README.md`: no cheap sum-compatible character filter; j = 2 four-tree at exponent 0.4167 closed for character filters under group-law or quasigroup combining, toy N <= 65539. KN-FIND-ffe1df: O2 is conditional, class-restricted, bucket-averaged, not a closure. Planted-leak desk note + Lean (CAR `docs/reports/ic-prime-eth-leads-20261102/PlantedLeak.lean`, 4 theorems, standard axioms): a planted log-mod-M character is the Pohlig-Hellman leak, not a coordinate-computable filter. Log-domain FFT scans: 507 families, 13,528 combos, max 1.10x vs null.
- 46 ledger records. **ABSENT.**

### C6. Elliptic-code list decoding as a decomposition oracle (Zhang-Liu ePrint 2018/795; Cheng)
- Proposal only (11-03): counting predicts hits only if C(B,M) p^(k-M-1) >= 1 with M = sqrt(Bk), forcing k ~ B. **ABSENT** (short paragraph).

---

## D. Coordinate statistics, pseudorandomness and instrument calibration

### D1. H-PSEUDO: additive-character bias of the small-x factor base in log order
- Original: EV-ECDLP-f83a17, R ~ 1.28-1.34 constant; summer record fitted C(p) ~ p^0.055-0.079; KN-FIND-e7a3b1 closes standard analytic proof routes; KN-FIND-9d2f56 Betti-yield duality (sub-rho needs H-PSEUDO-type yield); KN-FIND-4c9e71 corrects an earlier O(1) max-C claim.
- **Correction (2026-11-04, CAR `docs/reports/hpseudo-symnull-20261104/README.md`, frozen HPSEUDO-SYMNULL-20261104):** verdict EXPLAINED. The (x, y)-keyed small-x base is negation-closed; the original null was asymmetric. Against a negation-closed null: out of sample 1.040 [0.999, 1.092] at 2^20, 0.993 [0.972, 1.017] at 2^22; pooled 56 curves 1.005 +- 0.008. Non-rejection rule; residual up to ~+9% at 2^20 not excluded. Desk Monte Carlo `checks/d1.json` reproduces R ~ 1.3 for random negation-closed sets. Lead now exhausted_at_scale.
- Treatise summer.tex line 89 still records H-PSEUDO as an open problem with the fitted C(p). **PARTIAL, needs correction.** **IC-relevant (directly: relation yield).**

### D2. Quadratic-phase (U^3-type) leakage of the small-x set (CAR `docs/reports/quadphase-20261104/`, `quadphase-2p16-20261104/`)
- Frozen QUADPHASE-20261104: NULL at N ~ 2^12-2^14 (9/9 decisive curves inside the null's 90%); detects coherent single-phase leakage of about 0.3 of the set or more. 2^16 settle: u = 0.50 against 40 draws (null mean 4.750, sd 0.229, max 5.581); the earlier exceedance came from a 5-draw null. **ABSENT.**

### D3. Coordinate-statistic census and Fourier flatness
- EV-ECDLP-81f4d4: every structured additive statistic PGL_2-unstable, stable multiplicative coset statistics at random level; recorded strong, **lowered to preliminary by CORR-20261026-c94668**. EV-ECDLP-5add2c bounded-degree subset restriction O-INCONCLUSIVE.
- EXP7 (C `research/prime_fourier_flatness_exp7_20261007/RESULT.md`, run 10-08 with a date erratum): six abscissa predicates, four prime-order curves (8-14 bits), 512 negation-paired nulls each: no adjusted candidate. (Its 12-bit curve has n = p = 2749, i.e. is anomalous; not flagged in the result.)
- 64 ledger records. **ABSENT.**

### D4. Holant / GAG / QTT view of the decomposition-count network (11-01 to 11-04)
- Leads `LEAD-EXO-gag-holant-semaev-chain` (open) and `LEAD-EXO-holant-decomposition-count-network` (closed_dominated). Step (a): H1 true only under a reading that excludes basis changes; transfer matrix is the O-deleted Cayley quotient (secular-equation spectrum); for j = 0 two thirds of the spectrum is character sums (11/11 curves). Step (b) (frozen HOLANT-STEPB-QTT-20261104): flattening and CP ranks are invariant under invertible bond maps (checked p = 61); QTT rank at p = 1021: x-labelling 1024 (ceiling), random 1023-1024, discrete-log labelling 109-121; NULL. Anomalous follow-up: 63 + 63 curves, p in [37, 127], NULL; the p = 61 anomalous hit was chance.
- **ABSENT.**

### D5. Weak-class calibration of the structure instruments (CAR `docs/reports/weakclass-20261104/`, `weakclass-i1-20261104/`, `paired-anomaly-20261104/`)
- Do the instruments that rule out structure see curves whose DLP is known to be easy (anomalous, small embedding degree, j = 0 with zeta-stable base)? WEAKCLASS: I1 character sum INVALID (miscalibrated A/A), I2 m = 2 sumset yield BLIND (mean u 0.56/0.44/0.64), I3 QTT BLIND (0.36/0.44/0.60). I1 rerun BLIND. Paired-anomaly replication (108 + 108 curves): NOT REPLICATED (z = 0.45, p = 0.65); anomalous and small-k not detected; **zeta-stable FB detected (p < 1e-5)** in the 36-curve comparison.
- Consequence for the treatise: a NULL from these instruments excludes only structure of their own type; they are blind to Smart- and MOV-weak curves. **ABSENT; methodologically important.**

### D6. Methodology instruments (114 ledger records)
- EV-INSTR-03fdcf (label-permutation null coupled to N; replicated), EV-INSTR-ffbb5a, EV-FIND-003 (promotion of KN-FIND-017..027), KN-OPEN-5b3a08 (s4_expr repair; 21 runs predate it, impact unmeasured), C `problems/benchmark-validation-gate-postfreeze-rows.md` (5/121 benchmark rows independently validated; IC leaderboard 19 validated, 9 pre-gate).
- Treatise prog_method.tex covers claim hygiene and the gate. **PARTIAL** (null-coupling and planted-control lessons missing).

---

## E. Lifts, transports and representation changes

### E1. Xedni and global lifts
- Treatise obstructions.tex and summer.tex: function-field family, P_lift = (1 - 2/p + p^-3)/(2p^3), alpha = 3, 596,268,874 slots, scope one family. **COVERED for EV-XEDN-001.**
- **Missing / corrections:** JKSST skeptical Lean audit (C `research/jksst_xedni_audit_20261101/README.md`, `PROOF_MAP.md`, 11-01): JKSST is a "strong conditional asymptotic obstruction"; Lang's height conjecture, height/discriminant comparison, random-relation and repeated-trial bridges are not proved; Lean counterexamples refute overbroad readings (no fixed discriminant bounds heights of all multiples; quadratic lattice countermodel for Lang's gap); exact useful-relation probability (q-1)/q^2; "at least O(p)" should be Omega(p). C `research/ecdlp_weakness_lean_20261104/README.md`: HazardSeparation proves the truncated-expectation conclusion without JKSST's hazard recurrence, and a separation example shows marginals do not give the recurrence. C `research/ecdlp_subexp_desk_20261101/README.md` header: "broad 'xedni is closed' statements below are superseded"; `LEAD-TRF-global-lift-small-height` and `LEAD-ICP-char0-lifting-xedni-smoothness` reopened. EV-XEDN-002..006: short Mordell-Weil relations with max|coeff| = 1 on non-isotrivial surfaces at p <= 31 (toy; validation valid_with_findings, red-team "conclusion requires narrowing"); no decomposition step. CAR `analysis/xedni-cheon/`, `analysis/xedni-prescribed-points/` (proposals; rank upper bound, not rank lower bound, gives dependence).
- **PARTIAL; correction needed** (treatise's "closed in exact form" must be scoped as conditional).

### E2. p-adic jets, canonical/Teichmüller lifts
- Treatise summer.tex ("the canonical lift carries no logarithm"; Teichmüller contrast) and obstructions.tex (jet simulable). Missing: KN-FIND-85845f hard-lift digit identity exact 1360/1360 but hard-lift cost unbounded; KN-FIND-1ba0fe valuation profile collapses to the +/- fibre indicator; CORR-20260727-003 withdraws the jet O(1) theorem claim; C `research/prime_jet_completion_20261030` is a different jet (tensor interpolation variable). **PARTIAL (minor).**

### E3. Elliptic nets / EDS (EV-NET-001, preliminary)
- Somos sieve: charged k-recovery exponent 1.97-2.80 vs BSGS 1.17-1.32; sieve/BSGS op ratio 24.2 -> 134.7 for p = 101 -> 1601; consistency sets mean |S| = 11-68 (random ~2); unexplained: permuted nets show 0.52-0.70x as many Q-involving collisions. Treatise mentions only "elliptic-net evaluation" as GGM-simulable. **PARTIAL.**

### E4. Miller cocycles, S-units, pairing return cycles; torsors and deck orbits; split-Jacobian projection; Lattès x-line dynamics; lossy tracked objects
- All idea-artifact gates and proposals: Miller cocycle primitive exists exactly when N does not divide p-1 (IDEA-20261017-e3a6f1, unreviewed); Chevalley classification for generalised Jacobians of modulus degree <= 4 (IDEA-20261009-61fc7f); scoped no-go for pairing return cycles; torsor deck-orbit router no-go (independently verified); split-Jacobian: KN-FIND-61347e Hasse-Weil order inflation, no route passes lambda, mu <= 0.45; Lattès: two Cauchy-accumulator closures (EV-ECDLP-0c0f51, -a8af77), IDEA-113 rejected; lossy objects (50 records): orbit argument (IDEA-20260806-c5d183), rigidity theorem (IDEA-20260807-df906f), KN-FIND-7d098b, EV-ECDLP-ce9b50 O-MIXED at F_17.
- **ABSENT** as a group (one survey table would do).

### E5. Curve models and representations (Edwards, Hessian, Montgomery, Kummer, x-only, u = x/y)
- EV-REP-001/002: model-native solving degree invariant (d_reg = 2 at m = 2, 3; Edwards constant larger, replicated toy). EcGFp5 desk audit (C `research/ecgfp5_representation_audit_20261102/README.md`): u-ring D = 16 at m = 3 (p = 4111, 262231), D = 512 at m = 4 (p = 4129, 262153), equal to the Kummer class; CORR-20261102-7e3a91 scopes "ring form unavailable" to phi(-P) = phi(P). Treatise prog_theory.tex has one Edwards mention. **PARTIAL.**

### E6. Invertible hint subsets and Lange-Winterhof interpolation bounds
- RQ-ECDLP-bc7a54; EV-ECDLP-924941 O-IMPEDIMENT (no A(1) cell measured); **EV-ECDLP-8a5106 planted generic-group null voided (O-ARTIFACT)**; IDEA-20260920-a92b0f (P*T = Omega(n) false in the non-uniform GGM). Lange-Winterhof (COCOON 2002 / AAECC 2003) and Kiltz-Winterhof degree bounds identified at abstract level only (CAR `docs/reports/leads-round2-20261104/amendments-literature.md`). **ABSENT.**

---

## F. Literature-claim reproductions and screens

### F1. lit2026 programme (C `research/lit2026_speedup_program/`, workstreams W1-W12, 10-11)
- W12 zero-minor (Mahalanobis arXiv 2607.09814v2): 48/48 planted, slope 0.905 [0.769, 1.021], 260x -> 6,715x rho; Hu ePrint 2025/015 not implementable. W10 Coppersmith S5 delta* = 1/80 vs ~1/5. W11 BSGS 16.75 -> 3.71 M-equivalents. **COVERED** (prog_prime.tex §"Literature claims that did not survive", §Lattices).
- W11 details not in treatise: (b) x-only binary isogeny 1.50x fewer F_{q^8} mults, end-to-end no change; (c) k = 2 cubical pairing 17-19% fewer M-equivalents; (d) DP trails truncated at dp = 12 (mean 0.43-0.48 of geometric, 1.37-1.58x ideal); (e) secpkk1 ladder 720 solves, iterations / sqrt(pi n/4) = 1.077 +- 0.020, and the source paper's cards use the opposite prime-search direction from its Algorithm 3. W4 (Horak-Semaev early abort: mode 1 no-change, mode 2 negative) and W5 (gate-first F6-IC 1.57-1.78x lower stage units; lane batching 2.2-4.0x worse) cited only by path. W1, W2, W6, W7, W9 not started. W3 see C3. **PARTIAL.**
- Minors / guess-and-determine lead (CAR `coordination/design/SPECIAL-STRUCTURE-LEADS-20261031/minors-guess-and-determine/design.md`, draft): extends W12 to the Abdullah-Mahalanobis-Mallick line; no run.

### F2. GOAL-CRYPTO-001 crypto-size candidate screen (exhausted_at_scale)
- 12-batch budget exhausted (EV-CRYPTO-012, "not an impossibility result"); BATCH-013 FINAL_NO_CANDIDATE (EV-CRYPTO-013); hyperplane-signature zero-minor route closed as non-sub-rho (KN-FIND-3d6f78); EV-CRYPTO-ad7fa2 contradicts gate survival of H-HZM-001 (replicated). **ABSENT.**

### F3. ECDLP-IDEA corpus (~436 generated ideas) (CAR `ideas/`)
- Active/deferred/rejected registry; no experiment ran; constructors missing (spinor-matchgate addition, block-Krylov, Cremona toric, aggregate divisor compression...). Direct cancellation pilots (EXP-ECDLP-7c0b1e, -fe3337): no EC-specific constructive cancellation rule. **ABSENT** (worth one page as "what the idea generator produced and why none ran").

### F4. Sub-exponential desk evaluation (C `research/ecdlp_subexp_desk_20261101/`, 11-01)
- Sixteen "exotic" candidates (tag tracing, Lange-Winterhof, Tate-curve transport, twist trace map, sum-product, special primes, Weil restriction, x-only, SIMD, black-box fields, Maurer-Wolf, Coleman integration, ...) each matched by `lead_atlas.py --match`; none new. Route-by-route obstruction table (algebraic membership degree >= B/3; Coppersmith reach 0.3293 per S3 block vs 0.8889 needed; genus-g over F_p costs p^(2-2/g); etc.). Snapshot of 154 leads; later reconciliation 160 leads. **ABSENT** (good source for a structured "routes and obstructions" chapter).

### F5. Twenty-idea execution round (C `research/ideas_execution_20261018/EXECUTION.md`, 10-18)
- All 20 items partial. Besides the learned items (A4): item 1 native F_p^2 controls 23/24 two-sum hits; item 3 Boolean controls (Dinur parity 6/6, LPTWY 8/8); item 7 disk-pivot engine hit an 80 GB address-space cap at ~79.6 GB RSS; item 14 prime m4 480 natural targets, 40 decompositions; item 20 classical/quantum boundary 75 resource rows. **ABSENT.**

### F6. Integer-factoring transfer (C `research/integer_factoring_transfer_20261101/`)
- Imports canonical residual identities and unit-aware cancellation as a design contract; query reductions are not wall-clock speedups; prime-residual certificate has no EC analogue. **ABSENT** (minor).

---

## G. Auxiliary inputs, advice and leakage

### G1. Cheon auxiliary inputs and deployed SRS census
- C `research/torsion_auxiliary_inputs_20260918/`: measured exponent 0.23 vs rho 0.50 from 24 bits up, only when [alpha^i]G is leaked; torsion points are not a lever. C `research/srs_auxiliary_census_20260927/`: deployed powers-of-tau setups priced at their actual leak sizes (generic floors at constant 1, not measurements; p-1 algorithm measured ~20x its floor over 24-48 bits). Treatise prog_prime.tex: one sentence. **PARTIAL.** IC-relevance: not IC (a different problem, DLPwAI).

### G2. HNP / advice leakage (`LEAD-EXO-hnp-advice-leakage`, open_blocked)
- Conversion threshold 2*beta vs 1/2 deferred; advice-length proposal. **ABSENT** (short).

---

## H. Quantum

### H1. secp256k1 reversible point-addition circuit for Shor (C `quantum_circuit_secp256k1/README.md`)
- HEAD 1,739,149 avg Toffoli x 1,434 qubits = 2,493,939,666; best 2,490,847,962 (1,736,993 x 1,434), -0.124%, valid on the 9024-shot island only. CA `suite/src/cryptanalysis/shor.rs`, `quantum_estimator.rs`. Plus ideas item 20 (75 resource rows). **Unrelated to classical IC.** ABSENT; at most a paragraph in "what is outside scope".

---

## I. Not ECDLP

- **Eigen-Séta** (isogeny-based PKE; C `research/eigen_seta_toy_20261011`, `_prototype_`, `_rust_`, `_assessment_20261017`, `_speedups_20261017`, `_whitepaper_20261017`, `_ifma_20261018`; papers `eigen-seta`, `eigen-seta-whitepaper`). Notable facts: the `walk` variant was found insecure (5/5 ciphertexts decrypted from public data, correction 10-11); L1/L3/L5 meet NIST categories only in the gate-count metric (~2^64.5 and 2^96.5 quantum queries at L1/L3); hidden orientation ranked as the largest risk; chain-step cost corrected 2026-10-18 to 188.9 F_p^2 multiplications per theta step; decaps ~89,000x slower than ML-KEM at L1; IFMA decaps 1.65x/3.11x/3.31x faster. **Unrelated to ECDLP IC.** ABSENT, correctly.
- pqc_ml_kem_* (3 dirs), pqc_cross_platform, csidh_toy, sidh_toy, tii_mceliece, AES/Speck distinguishers: unrelated.

## J. Boundary with other families (not digested here)
- Binary/Koblitz SAT and WDSat lanes (n83_chain_sat_*, f6_n83_*_sat, full_binary_norm_sat, wdsat_rr): binary/engine families. Prime SAT design dirs (sat_smt_prime_ic_design_20261030, pdp_sat_smt_survey_20261010, sat_factor_base_review_20260908) have no top-level README on main; prime_shared_coefficient_sat_20261031 and ecbench_sat_pdp_20261009 are engine-family harnesses (the latter's pilot exposed an ic.pipeline cost-replay defect). Fall-degree Lean refutations (disprove_easy_targets_20261102, lean_attacks_20261102) belong to the fall-degree chapter. Small-root reach (prime_eform_reach_20261029) belongs to prime.

## Retractions and corrections in this family (flag list)
1. H-PSEUDO R ~ 1.3 explained as an asymmetric-null artifact (HPSEUDO-SYMNULL-20261104). Treatise summer.tex still lists it as open with C(p) ~ p^0.055-0.079.
2. Classic xedni obstruction is conditional (JKSST Lean audit 11-01; desk header: "xedni is closed" statements superseded). Treatise obstructions.tex/summer.tex wording too strong.
3. EV-ECDLP-81f4d4 coordinate census lowered strong -> preliminary (CORR-20261026-c94668).
4. CORR-20260727-003 withdraws the jet O(1) theorem; EV-GGM-002 supersedes EV-GGM-001.
5. EV-ECDLP-8a5106 planted generic-group null voided (O-ARTIFACT).
6. WEAKCLASS I1 INVALID (miscalibrated A/A gate); paired anomaly not replicated.
7. ETH-route leads: L05, L06, L08, L10, L12 refuted as stated; three LEADS.md sentences corrected (D.1 constant threshold, edges per addition 9-30x high, clustering).
8. hashed_ecdl_hash_prime: user-reported "Fourier network is impractical" attribution corrected (the hang was AKS primality).
9. Learned WDSat negative (16,097 vs 3,092) is confounded with disabled symmetry; the learned order equals the block-core arm.
10. Eigen-Séta `walk` variant insecure; unqualified NIST-category claim withdrawn; chain-step cost model corrected (non-ECDLP).
11. CORR-20261102-7e3a91 scopes the curve-model "ring form unavailable" sentence.
12. lit2026 W11(e): source paper's challenge cards use the opposite prime-search direction from its own Algorithm 3.

---

## Summary table

| line of work | IC-relevant? | coverage | est. pages it deserves |
|:--|:--|:--|--:|
| A1 ML ECDLP/PDP dataset campaign | yes (infrastructure, labels = IC phases) | ABSENT | 5 |
| A2 dataset models dev pilot | weakly | ABSENT | 2 |
| A3 ML-aided SAT ceiling (PDP) | yes, strongly | ABSENT (PARTIAL context in prog_binary) | 3 |
| A4 learned WDSat / pivot / prefilter / surrogate controls | yes | ABSENT | 2 |
| A5 RL Buchberger pair selection (gbrl) | yes (engine heuristic) | ABSENT | 2 |
| A6 hardness predictor, RL isogeny, trained candidate, latent linearisation (plans) | marginal | ABSENT | 1 |
| A7 neural distinguishers Speck/AES | no | ABSENT (correct) | 0 |
| B1 hashed-factor-base tensor ECDL (Bochum v0.2 / OpenAI D.1) | yes, directly | ABSENT | 8 |
| B2 ecdl-tensor-ic port and speed instrument | yes | ABSENT | 1 |
| B3 twelve prime-field ETH-route leads | yes | ABSENT | 6 |
| B4 tensor-Boolean E0-E5 rank census programme | yes | ABSENT | 3 |
| B5 prime tensor continuation (one-shot, packets, common kernel, jets, pole) | yes | ABSENT | 3 |
| C1 augmented oracles and GGM simulability, 11-04 split | yes | PARTIAL | 3 |
| C2 low-degree query lower bound | yes | ABSENT | 3 |
| C3 lit2026 W3 SGGM/DKM/multi-instance audit | yes | ABSENT | 3 |
| C4 k-SUM obstruction and k = 5 tables | yes (generic track) | PARTIAL | 2 |
| C5 four-list k-tree / sum-compatible filters | yes | ABSENT | 2 |
| C6 elliptic-code list decoding | yes (proposal) | ABSENT | 0.5 |
| D1 H-PSEUDO character sums (with correction) | yes, directly | PARTIAL, needs correction | 3 |
| D2 quadratic-phase leakage | yes | ABSENT | 1.5 |
| D3 coordinate-statistic census, Fourier flatness | yes | ABSENT | 2 |
| D4 holant / GAG / QTT | yes | ABSENT | 3 |
| D5 weak-class instrument calibration | yes (methodology) | ABSENT | 3 |
| D6 methodology instruments | yes (methodology) | PARTIAL | 1.5 |
| E1 xedni and global lifts (with JKSST audit) | yes | PARTIAL, needs correction | 4 |
| E2 p-adic jets / canonical lifts | yes | PARTIAL | 1 |
| E3 elliptic nets / EDS | yes | PARTIAL | 1.5 |
| E4 Miller cocycles, torsors, split Jacobians, Lattès, lossy objects | marginal | ABSENT | 3 |
| E5 curve models and representations | yes | PARTIAL | 2 |
| E6 invertible hint subsets, Lange-Winterhof | yes | ABSENT | 1.5 |
| F1 lit2026 programme (W12/W10/W11 covered; W3-W5, W11 detail) | partly | PARTIAL | 2 |
| F2 GOAL-CRYPTO-001 screen | yes | ABSENT | 1 |
| F3 ECDLP-IDEA corpus | marginal | ABSENT | 1 |
| F4 sub-exponential desk evaluation (routes vs obstructions) | yes | ABSENT | 2 |
| F5 twenty-idea execution round | partly | ABSENT | 1.5 |
| F6 integer-factoring transfer | marginal | ABSENT | 0.5 |
| G1 Cheon auxiliary inputs, SRS census | no (DLPwAI) | PARTIAL | 1.5 |
| G2 HNP / advice leakage | marginal | ABSENT | 0.5 |
| H1 quantum point-addition circuit | no | ABSENT | 0.5 |
| I Eigen-Séta and other non-ECDLP | no | ABSENT (correct) | 0 |

Totals: 40 rows (38 ECDLP-related lines plus 2 non-ECDLP rows A7 and I); 28 ABSENT, 10 PARTIAL (D1 and E1 also need corrections), 2 correctly absent; nothing fully COVERED (F1's W12/W10/W11 headline is covered, its remainder is not). Page estimates sum to about 87.

## Suggested structure (about 90 pages)
- **Ch. "Unconventional routes: a map" (~6 pp):** F4 route-by-route obstruction table, F2, F3, F5, atlas footprint.
- **Ch. "The tensor-contraction route" (~20 pp):** B1 (Bochum/OpenAI claims, what is conditional), B2, B3 (twelve leads with skeptic corrections), B4, B5; the D.1 admission yardstick and edge constants.
- **Ch. "Generic-model and query lower bounds" (~14 pp):** C3 SGGM audit, C1 oracle split, C2 low-degree bound, C4 k-SUM, C5 k-tree, C6, E6 (merge with obstructions chapter or follow it).
- **Ch. "Does the factor base look random? Pseudorandomness tests and their instruments" (~14 pp):** D1 (with correction), D2, D3, D4, D5, D6.
- **Ch. "Learning inside index calculus" (~15 pp):** A1-A6 (dataset design and label hygiene, ML-SAT ceiling, learned branching, RL pair selection, dev pilot).
- **Ch. "Lifts, transports and models" (~13 pp):** E1 (xedni with JKSST), E2, E3, E4, E5.
- **Appendix "Outside the scope" (~3 pp):** G1, G2, H1, Eigen-Séta, symmetric ML; F1 W11 details.

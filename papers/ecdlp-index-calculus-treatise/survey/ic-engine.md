# ic-engine coverage digest (survey agent, 2026-11-06)

Scope: the machinery of index calculus: Groebner engines (matrix F4/F5 over F2, inherited F4, F6-IC, F_p F4, signature/tower F4), Boolean generic solvers, MutantXL closure/dreg instruments, trace replay, linear algebra, relation-collection and root-finding engines, SAT/XOR oracles, sieving engines, ecbench harness and its accounting, autolab/planner infrastructure, GPU kernels, hashed-ECDL/tensor engines.

Sources read: crypto-autoresearcher `knowledge/leads/ecdlp/ic-engine/*.yaml` (20 leads), crypto `research/<dir>` READMEs/RESULTs/REPORT.typ for ~310 uncited family dirs (first lines and verdict lines; not every file in full), `docs/ic/`, `docs/ecbench/` listings, `benchmarks/README.md`, treatise `sections/*.tex` on origin/main (grepped for each topic), cryptanalysis `experiments/groebner-perf-20260924`, `experiments/pdp-degree-heuristics`, crypto-autoresearcher `analysis/` (fall-degree items).

Standing note (sabotage rule): almost no number below has a pcc claim id. Unless a VALIDATION.json or pcc id is named, every figure is an **unverified assertion recorded by its producer** (counts mostly L0, same-host replay). Where a lead or README gives a ledger evidence id (EV-…), that is quoted as the lead's own standing, not re-verified here.

"Coverage" refers to the treatise on crypto origin/main (`papers/ecdlp-index-calculus-treatise/sections/`).

---

## 1. Boolean generic-search solver campaign (22 Sep – 3 Oct 2026)

- **What**: complete generic Boolean solving of bounded generated systems (n up to 36 variables) used as the decision engine below the descended Semaev system: packed direct construction, quadratic coefficient state, packed residuals, affine substitution, linear tails/fibers, projected fibers, implicit minors, byte sieves, product schedules, half-word specialisation, rank coordinates, envelope indexing, graded tail reuse, pivot-trace screens.
- **Numbers**: packed residuals **1.56–1.67x** vs pointwise fastest of 8 retained methods, cumulative 2.63–4.02x vs historical seven, universal 2x gate rejected (17/18), 5,832 timed observations on 72 systems (`boolean_quadratic_frontier_20260923`); fixed quadratic coefficients **2.40–2.51x** on primary holdouts, 6,144 verified solves (`boolean_quadratic_state_20260923`); large-active linear tail **3.59x** lower cold cost on its declared mixture, promotion guarded (`boolean_linear_tail_20260923`); packed direct construction passes 2x gate on construction only, no full-solve claim (`boolean_packed_construction_20260922`); construction + reduction: staged path 1.38–1.50x at 36 vars (`boolean_construction_reduction_20260922`). Rejected: affine substitution (15/18), basis transport (full-RREF 0.199–2.667 paired range), linear fibers (0/18 dramatic), projected fibers, byte sieve, initial restriction, implicit minors, schedule-construction, product schedule; `boolean_solve_trace_20260923`: specialised-kernel gains do not transfer to whole solves; `boolean_pivot_screen_20261003`: sibling restriction defeats pivot-trace replay.
- **Verdict**: component speed-ups (solver stage), no IC/rho claim; lead LEAD-ICE-boolean-f4-f5-f6-engines says "same search-tree exponent".
- **Sources**: crypto:research/boolean_*_2026092x, boolean_*_202610xx.
- **Coverage**: PARTIAL. prog_groebner mentions "about 90 directories … clusters 'boolean' …, most failed" but gives none of these numbers or the universal-gate methodology.

## 2. Matrix F5/F4 over GF(2): micro-optimisation campaign (27 Sep – 29 Oct)

- **What**: ~30 preregistered same-binary paired A/B studies on the n24 degree-4 complete F5 call and Boolean F4: unpack variants (AVX2 gather 0.9652x, nibble 0.912x, nibble-subset **0.649x** regression, AVX-512 no gain, bounds-free 1.004x, POPCNT 1.0005x, one-pass 0.992x, compact-cols/unrolled 0.9998x, x86-guarded 0.9984x), pivots (bounded/stratified weight: structural gate failed, ≥96.4% of terms), row layout (flat rows 0.978x, byte-colex 1.023–1.026x vs 1.05 gate, initial density order 1.006x), echelon output (fails no-regression gate: n12_m12_d4 0.82–0.83), fused row build 1.522x [1.468,1.569] stage, mainline rank port 1.450x, support split 2.825x (ARM, nonpromoting), support cut-19 **3.006–3.049x** complete call (CI recheck PASS, `f5_support_cut19_final_recheck_20261003`), support-rank-bits width 6 lowers certificate XORs; F4: batched pairs (cleared gate), degree-bucket queue (missed 1.05), unsorted products (negative), wide tables (6-column gain, holdout regression 0.881).
- **Defaults**: `f5_gf2_defaults_20261017` (direct pack/unpack/table reuse made default on second host); `arm64_f4_defaults_20261022` (eager rewrite via shift runs, DEFAULT_TABLES=4, table reuse on Neoverse-N2 and M1); `f4_f5_followups_20261029`: four further candidates, **none promoted** (44% of instructions in `add_lazy_avx2` on quad_n9).
- **Verdict**: stage engineering; the 2x complete-call goal "remains unproved" except the cut-19 call. No IC distance moved.
- **Coverage**: PARTIAL. prog_groebner table has cut-19 (3.02–3.07x), combined options 1.530x, graded reuse 1.78–1.84x, bitmap 1.199x, branchless strip 1.210x, and says "about thirty variants were refuted". Missing: the refuted-variant table (useful as a negative-results appendix), arm64 defaults, Oct 29 follow-ups, the gate methodology (frozen protocol, A/A range, exact 3,125-resample bootstrap, holdout seeds).

## 3. Instruction-count (Callgrind) engineering of Boolean F4/F5

- Fused construction/readback −5.28% F4 instr, −11.02% F5 (`f4_f5_fused_readback_20261012`); native hashing −3.71% F5, −45.14% criterion (`f4_f5_hash_20261012`); bounded direct indexing −3.947% (`f4_f5_dense_packing_20261012`); allocation 42–50% fewer bytes (`f4_f5_row_build_20261011`); history plots (`f4_f5_history_20261012`); `f4_f5_boolean_perf_20261018`, `f4_f5_boolean_runtime_20261019`, `f4_f5_ecbench_20261019` (PDF-only run reports).
- **Coverage**: COVERED (prog_groebner table rows), though the 2026-10-18/19 runtime/ecbench PDF reports are not cited.

## 4. Inherited F4 (default algebraic oracle from 22 Sep)

- Design: DPLL tree over partial assignments; child inherits parent echelon (`inherited_f4_20260922`). Overhead round **0.847x** instructions at m=19, passed (`inherited_f4_overhead_20261005`); remap round 0.923x, passed but superseded, not merged (`inherited_f4_remap_20261007`); eager rewrite through shift runs (non-BMI2) 1.22–1.70x lower CPU on 6/9 cells (`inherited_f4_eager_runs_20261016`); lane-batched replay **negative**, 2.2–4.0x more stage units (`lit2026_w5_lane_batched_f4_20261011`); harvest default re-measure: sign depends on size, +10%/+7% S at n=11/13, −4.1% at n=19, +0.3% at n=17 (`f4_harvest_remeasure_20261024`, 5×144 records, 144/144 replays identical); replay determinism root-cause search negative/partial (`inherited_f4_replay_determinism_20261018`); cross-ISA: 48 replay mismatches from x86-only BMI2 word-wise rewrite charging different word XORs (validation_sweep D item 1); pricing of word XORs (#1774) moved priced n13 F4 16.27 → 488.89 and reversed the ranking vs root finder.
- Online: F5 / inherited-F4 online ratio 208.49x (181.29–223.86); inherited F4 ≈315x slower than rho on first public panel (`ic_solver_online_20261003`).
- Lead verdict: priced inherited F4 ≈22x–33x strong rho at n13; pair table 17x–22x cheaper than F4 (closed_dominated).
- **Coverage**: PARTIAL. Design, eager/lazy 1.40–1.62x, #1774 pricing and cross-ISA correction are in prog_groebner/prog_method. Missing: overhead/remap rounds, harvest size-dependence, online 208x/315x panel, determinism null, the 22–33x priced rho distance as a headline.

## 5. F6-IC (inherited F4 + factor-base domain gate + MITM leaves)

- Geometric closure design (`f6_ic_geometric_closure_20261003`); E_0 ladder: closure capped at 256 points, "no exponent signal", decision pending m=31 (`f6_ic_ecbench_ladder_20261005`); T7 phase profile: Macaulay construction dominates (`f6_ic_phase_profile_20261006`); inherited basis cost is child specialisation (`f6_ic_inherited_basis_profile_20261006`); same-target online vs rho on n17: rho/IC online ratio **0.0219–0.0221 (T1), 0.000515–0.000530 (T7)** (`f6_ic_rho_online_gap_20261006`); rejected: compact refuted bases (`f6_ic_compact_refuted_20261006`), bitmap columns, direct fused pack, generic direct pack, support-local stream, adaptive pair-sum index pilot (negative); gate-first ordering 1.57–1.78x lower stage unit (W5), made default after x86 confirmation (`f6_gate_first_x86_20261016`).
- **New, 5 Nov** (`f6_few_candidates_20261105`): few-candidate closure (≤64 base codes → exact closure decides subtree) CPU off÷on **2.67–10.29x** (median of 5), word XORs 1.84–10.13x, verdicts identical, inherited-F4 control 0.87–1.05x; the README itself says it "does not beat the MITM oracle … because in the limit it is that oracle [pcc:unverified]"; parallel split of one target's tree 1.59–2.88x wall; Four-Russians batching of displaced rows costs more than it saves. `f6_ic_pair_index_v2_20261101`: protocol under accounting v2 (no README result found).
- Priced verdicts: F6-IC 55x–6,483x rho (lower bound) per lead; group-additions-charged S 64.6–68.8x (m=13), 1,303–1,353x (m=19); priced K13 ≈2.5x worse than inherited F4 (prog_groebner).
- **Coverage**: PARTIAL. prog_groebner covers the design and S ratios. Missing: online gap 0.02/0.0005, phase profile, the Nov 5 few-candidate closure and its reduction to MITM (an instructive "algebraic engine converges to the generic oracle" result), parallel split.

## 6. F6 at n = 83 (ECC2K-130 field) engine campaign (4–5 Oct, ~27 dirs)

- Pair-kernel components retained: sign-folded pair index 1.261x (`f6_n83_sign_fold_20261004`), PMULL x-only residuals 1.402x index build (`f6_n83_pmull_xonly_20261005`), packed pair-index construction, compact signed-pair storage, x-only query, two-word geometric closure 3.00x/7.36x (`f6_wide_n83_20261004`), union-domain width correction (correctness barrier fixed, `f6_n83_domain_20261004`). Rejected: residual batch size 0.934x, x bitmap prefilter 0.888x, PMULL inversion. Algebraic: five-summand dim-18 system exact but root pruning fails; five-source prolongation hits column cap; six-summand direct system admitted, root gate inconclusive; one fixed source summand does not unlock root reduction; private quartic core smaller but unresolved. SAT encodings: basis-aware S3 SAT no ordinary model; factored S3 circuit compact but planted Kissat times out; recursive carry-less (Karatsuba) circuit gate passed, solver timed out.
- **Verdict**: component speed-ups and open/negative algebraic gates; no n=83 relation from F6.
- **Coverage**: PARTIAL. prog_koblitz cites the clusters "n83" and "f6-n83" in one sentence only.

## 7. F_p F4 engines (`f4_fp`, tower F4, signature tower) and external CAS comparisons

- Narrow 32-bit arithmetic: missed 1.2x gate (`fp_f4_narrow_20260926`); lazy deferred reduction AVX2 1.059x [1.055,1.062] vs current, 1.756x vs wide (`fp_f4_lazy_avx2_20260928`); AVX-512 1.047x / 1.513x (`fp_f4_lazy_avx512_20260928`); generic-prime tower engine (`f4_generic_field_20261015`); `f4_fp` 29.8–54.1x over predecessor, msolve parity 0.96–1.22, blocked FL pairs 0.75–0.78x on two largest (cited, COVERED).
- End-to-end effect of faster `f4_fp` (PR #1717): F4 multiplications 7.7x fewer, C_cov 5.0x fewer per decomposition test; S before÷after ≈5.0 (4.99–5.08); S/rho **4297→850, 1843→370, 981→194, 1897→373** (p=53,101), lower-bound comparison, unpriced native work (`f4_fp_ic_effect_20261018`).
- Magma F4 protocol registered 13 Oct (Magma reported 12–24x faster than `f4_fp` on square systems a day earlier); bundles and ours_*.jsonl only, no Magma result committed found (`magma_f4_comparison_20261013`).
- cryptanalysis groebner-perf-20260924: F_p F4 symbolic preprocessing 2.34x (fb=3), 1.99x (fb=4); F5B 1.49x/1.36x; GPU RREF Metal prototype; "local progress, not a speed record".
- `ic_sweeper_20261024` §2.5 desk review: in-house F4 needs 10^3–10^7 more operations than a brute grid on PKM systems, so root-finding-vs-F4 ratios are not evidence about published Groebner solvers.
- **Coverage**: PARTIAL. Missing: lazy AVX2/512, the 5x end-to-end S effect with its S/rho rows, the Magma protocol status, and the important sweeper caveat about in-house F4 quality.

## 8. Engine suites, solver selection, crossbred, lit2026 engineering

- `ic_framework_engines_20260922`: matched engine-suite contract (timeout never UNSAT, units named, missing = null, rows carry counted-rho and pair-table references).
- `crossbred_20260920` e2e panel: n=7,9, m=2,3: enumerate dominates relations/s (e.g. n9 m3: enumerate 19,696 rel/s vs SAT 51.7, Groebner 33.2, crossbred 11.7); relation collection stays Θ(2^n).
- `ic_solver_selection_20260915`: fixed solver routing, no routing gain; default stays pairs.
- `lit2026_w11_generic_engineering_20261011`: generic-method speed-ups from five 2025–26 papers (e.g. BSGS paired/co-Z step kernels 1.15–1.67x wall, L0).
- `groebner_stage_20260915`, `groebner_base_sweep_20260915`: early stage/base sweep JSON only.
- **Coverage**: PARTIAL (prog_binary/prog_koblitz mention crossbred; suite contract and selection null absent).

## 9. Root-finding / pair-table oracle engine (engineering side)

- Cited and covered in prog_rootfinding: 383x–46,911x per decomposition vs F4; rootfind-negfold m=2 ≈1.54x strong rho at 17-bit Koblitz (L0, natives unpriced); prime Table B 5.47x–13.5x.
- Uncited engineering: pair-table size tuning on 20-bit prime cohort (rho.negation mean S 1.296), arity at 64 abscissae, batched pair-table construction (batch/rho 16.229 GAE, cluster interval 12.975–20.964), buffer reuse (no wall gain), "cheaper targets" (≈half the counted cost; one cell 0.770 [0.720,0.824] of rho-strong at d=4 but stays 4.7x above elsewhere; rho-frob 6x below every IC arm at n=13) (`ecbench_rootfind_speedup_20261014`), generalized all-roots: uniform 5x batch hypothesis refuted (1.383x), 8x all-roots not qualified.
- Open blocker: native work unpriced, all wall L0, same-host replay (problems/rootfinding-complete-cost-calibration.md); owner track ruling: prime-table pipelines reference only, rootfind-subtract unclassified; pending Option C "generic: bounded by sqrt(r); cannot beat rho".
- **Coverage**: PARTIAL (results COVERED; the tuning studies, the 0.770 sub-rho-strong cell and its scope, and the track-classification ruling are missing).

## 10. SAT / native-XOR point-decomposition oracles

- ecbench_sat_pdp PROTOCOL-3: native-XOR SAT S 3.413 (lower bound) vs strong rho 3.998, i.e. **S/rho 0.854 lower bound, conflicts unpriced**, n=17 Koblitz, 40/40 verified; MITM 96.9x, Frobenius MITM 4.27x; derived identity: pdp CNF-XOR is a complete enumerator of V^m mod S_m with no above-leaf pruning. pdp-split priced 46x–2,919x rho. WDSat static-allocation ceiling ~24M entries. SAT IC full solve 8.98M conflicts, 53 conflict-capped targets.
- `pdp_ml_sat_ceiling_20261027`: ML-aided SAT closed by preregistered rule ("ceiling is real, the signal is not"), 4,200 labels agree with exhaustive search.
- `sat_smt_prime_ic_design_20261030`: design + probe; verdict: crossover at p≈2^100–2^150 and "the whole method stays above rho by p^{1/3} or more".
- `prime_shared_coefficient_sat_20261031`: discriminator, no IC speedup.
- L04 prime strong-backdoor search: offset (m−1)s − B* = 2·log2(cap) − 1 (enumeration moved inside the propagator).
- `n83_chain_sat_*` (31 Oct – 1 Nov): chained-S3 SAT on N83, m=2…9: root-finder tables refute every public target with 0 conflicts at m=3, median 156 conflicts / 0.98 s at m=4; MITM lookup; rebuild by peeling; parallel back-enumeration 41.7→11.4 s (m=8), 57.0→13.7 s (m=9); parallel fingerprint build 3.50x (depth 5, 372,406,144 entries); 0 soundness flags. L0, no rho.
- **Coverage**: PARTIAL. SAT appears throughout; missing: the 0.854 lower-bound row and why it is not sub-rho (unpriced conflicts + enumerator identity), ML-SAT closure, prime SAT design crossover, the N83 chain-SAT series.

## 11. MutantXL closure-degree / hybrid-guess instruments (dreg_closure series)

- Covered: mutantxl (92 UNSAT draws refute at 3–4 where XL needs 5–7), driver (N does not set degree), m4l6 ((4,6) refutes at 4), hybrid guess (3,7): 10/10 refute at 4 with k*≤2, 1/116–1/3,400 of projected degree-5 cost, ECC2K-130 m=4 +8.43 bits (constant k) or +30.7…+58.3 (k growing with ℓ).
- **Uncited and newer than the treatise text** (treatise prog_frontier says the ℓ=8 run "did not complete"):
  - `dreg_closure_guess_20261022`: (m,n,ℓ)=(3,22,8), N=46: **four guessed variables bring every ℓ=8 draw back to degree 4** (4 UNSAT + 3 SAT draws, peak RSS 10.6 GB, 124,314 deg-4 columns/branch).
  - `dreg_closure_guess3_20261023`: **three guesses do not suffice at ℓ=8 for this guess set**.
  - `dreg_closure_null22_7_20261028`: unspecialised (22,7) does not refute at degree 4 on 4/4 draws (guessed versions do).
  - `dreg_invariant_guess_20261029`: invariant (linear) guess never worse than raw, does not reliably suffice at ℓ=8; `dreg_invariant_guess_more_20261029`: invariant beats raw on 5/5 discordant draws (p=0.031); `dreg_invariant_pair_guess_20261029`: second invariant on the pair no better (0/12); `dreg_pair_invariant_20261028`: summand-swap invariant does not refute below 4 at ℓ=6; `dreg_second_link_guess_20261105`: no detected difference (6 vs 7 of 12), **pcc:PCC-20261106-607061, unverified assertion**.
  - `dreg_closure_specialise_20261017` (in lead, not treatise-cited), `dreg_fixed_surplus_20260923` (inconclusive), `dreg_closure_slope_20261018`, `dreg_closure_m4_20261018`, `dreg_high_arity_20261101` (protocol v4 frozen, launch blocked), `closure_degree_predictor_20261018`.
- **Reading** (mine, not a recorded verdict): k* at the raw guess set rose from ≤2 at ℓ=7 to 4 at ℓ=8; by the lead's own discriminator ("one more guess per ℓ moves it to S_lin … and closes this lead") this points toward the k-grows reading, but no pricing study of the ℓ=8 result was found, and the invariant guess partly reduces k. **The treatise's statement that ℓ=8 has not run is out of date.**
- Blocked: C1 (3,19,7) degree-5 closure capped at 169 GB of 184 GB (projection 217 GB); E12b (3,24,7) cannot launch (On-Demand RunInstances denied).
- **Coverage**: PARTIAL, with a staleness correction needed.

## 12. Degree-of-regularity and syzygy instruments (autoresearcher GOAL-DREG / GOAL-SIG / SEMBIN)

- GOAL-DREG-001: n=12, D=6 genuine deficit 17,947 vs restricted null (EV-DREG-008, preliminary; raw headline quarantined_confounded); n=15 D=6 runs failed at 11.7–12.2 GiB; C2 n=21 D=5 rank matched prediction (EV-DREG-4500cd); KN-FIND-9cba50: ladder measures Frobenius-invariant subspaces, not Assumption-1's object.
- GOAL-SIG-001: D-born non-rewritable signatures; D≤5 admissible, D≥6 null needs recalibration; EXP-SIG-008 artifacts destroyed by infra incident (EV-SIG-008).
- `ic_semiregular_law_20261005` (cited): verdict "neither".
- First-fall-degree audit (Nagao Lemma 4 false over F2, KN-FIND-936151; uniform degree-two bridge refuted in finite model, KR-IC-139da0, 2026-10-25 correction); Lean: raw binary two-S3 last-fall upper bound instantiated (`analysis/rigorous-lfd-solving-20261105`, 27 axiom-audit declarations pass); boolean fall certificates tranche (`analysis/boolean-fall-certificates-20261104`, nine Lean theorems); `semaev_t2_trace_20261102`: Semaev 2015 t=2 first fall degree 2 not 4, solver needs 2 or 3 ("SPLIT" on earlier inference).
- **Coverage**: PARTIAL — falldegree.tex is long (3,816 words) and has syzygy and semi-regular mentions; the Nov 4–5 Lean tranche and `semaev_t2_trace` are not cited; the DREG null-choice controversy should be checked against falldegree.

## 13. Presentation / topology of the decomposition system (tree split, tree gluing, GAG planner, chain cost)

- `ic_tree_split_cutout_20260930` and `ic_tree_split_structured_v_20261011`: is the m=4 tree-split set cut out at low degree? Registered verdict **CLOSED** (poly, frob, rand; one gap table corrected 2026-10-16: 15×0, 5×−1).
- `ic_tree_gluing_20261103` (new): tree-glued cherry-symmetric encoding T vs chain C (msolve 0.6.5, prime field): m=2 D stays 4 for t=3..7 while C rises to 5 at t=6,7, matrices 1.8–3.6x narrower; m=3 t=5 T finishes D=6, C cut off at 600 s in degree 7; m=4 t=4 T D=7 (697 s). Kill criterion not met. Stage diagnostic, no S, no rho.
- GAG contraction planner (autoresearcher run-001, n=m=3 over F_{p^3}, p∈{1009,4099,32749}, 30 planted instances): monolithic S_4 used **0.110x** the chained F4 matrix cells on every instance (supported_within_scope, descriptive); tree 1.095x inconclusive; independent review-run recommends replicate.
- Semaev chain cost model: t*=m* at every n∈[250,600] (EV-SEMBIN-1d36a3, replicated); topology-blind charged cost (N=(t−2)n+tk invariant).
- **Coverage**: PARTIAL (prog_binary mentions tree split once). Tree gluing, GAG planner and chain-cost obstruction are ABSENT.

## 14. Linear algebra engines

- `la_engines_20261014`: LA is 0.11–1.9% of real `ic logs` precomputes (27 runs); N73 600-column matrix took 5.1 h to collect, milliseconds to solve; our block Wiedemann beats CADO-NFS bwc on the identical matrix (1.86x at 131,072 columns); CADO filter leaves 5–8x smaller matrix; ported filter 86.57 s → 8.35 s (**10.40x [9.99,11.06]**) at D=262,144 and **1.93x [1.74,1.96]** faster than CADO pipeline (7.83 vs 14.73 s); slower below ~4,096 columns (0.40x at 1,024); `IncrementalGauss` grows 6–9x per doubling (19.1 s at 8,192).
- `sparse_la_timing_20261018`: **null/negative**: deferred sparse matrices cost 1.206x more end to end on k=4 ladder, 5.72x→1.030x on LA-scaling (128→4,096 columns); cause is the completion rule, not LA.
- `quartic_n4_la_20261021`: double large primes (B2) and structured elimination (B1) vs B0 on k=4 N4; interval below 1 at every q≥4051 (0.297 [0.295,0.299] at 262111 for one ratio; read in full before quoting).
- 2LP cycle-basis enrichment 9/9 cells, inconclusive (EV-RELN-3c9a24); genus-3 2LP LA exponent 0.495→0.420 but whole solve 2.31x costlier (k=4 ladder); LA <0.5% of Table B S, relations 68–95%.
- Free-relation census: rank stage is not free (`ic_free_relation_census_20261011`).
- **Coverage**: PARTIAL. prog_frontier has one table row (Filter+Wiedemann 1/10.40, 1/1.93 vs CADO, "LA is 0.1–1.9% of runs") and a sparse-CADO row; model.tex lists the levers. A proper LA section (bwc comparison, filter port, crossover below 4k columns, sparse null, 2LP) is missing.

## 15. GLS / F_{p^4} chain-solve Macaulay engine levers (E21–E42, 31 Oct – 3 Nov)

- Research note RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.x. Solver A (ranked chain solve), B (sparsest pivots: multiplies 1.82–1.86x less than A at 0 GAE).
- E33 (`e33_solver_b_pivot_count_20261031`, VALIDATION.json present): counting B's selection (≈1.05e6 native ops/call) makes B unranked: B/B′/B″ vs A unresolved across band; B″ falsified vs E25 at 1 GAE (0.981–0.997). Distance to rho at 0 GAE 3.45e3–2.19e5.
- E35 (`e35_chain_trace_20261102`): learned-per-curve pivot trace T: no call's pivot sequence equals its curve's trace; contested-column agreement 69.8–88.4%; T vs A unresolved (ahead at 0/0.1, 14/20 rows at 1 GAE); best distance 5.11e3x (T), A 6.39e3x. "There is no per-curve pivot order to replay."
- E40 (`e40_macaulay_entries_20261102`, VALIDATION.json present): store each row from its lead (41.6% of 176,064 cells left of lead never read): S_A/S_V 1.0000 / 1.2518–1.2618 / 1.3629–1.3807 at 0/0.1/1 GAE; ranked ahead of A at positive weights only; rho ahead (best 6.00e3 / 1.58e4 / 6.17e4).
- E42/E42b (`e42_macaulay_tails_20261103`): E42 killed by its own mis-derived control prediction; E42b W (`chain-tails`) ahead of V by 1.022–1.023x (0.1 GAE), 1.032–1.034x (1 GAE), equal at 0; S_W/rho 6.00e3–2.29e6. Not validated.
- **Verdict**: band-dependent component improvements of the native-work price, not of GAE; rho 10^3–10^6 ahead.
- **Coverage**: ABSENT (extension.tex mentions "sparsest"/solver B once). Belongs in extension/k=4 or an engine chapter; good worked example of the §5a band changing rank.

## 16. Curve25519-family algebraic track: PKM-Semaev oracle, F4 trace replay, autolab (2–5 Nov)

- k=20 miniature (p=1048573, r=131203): `pkm-semaev` (t=6, G=4) behind generic baseline 2.6x / 1.4e5x / 8.2e5x at w=0/0.1/1 and behind rho-neg by **18x / 9.6e5x / 5.9e6x** (`curve25519_algebraic_ecbench_20261102`, L0).
- F4 trace: learn once per branch system, replay per target, fallback to plain F4 (`curve25519_f4_trace_20261102`; `f4_fp_tower.rs` record_trace/replay).
- T12 winner (t=5, G=4) 18.65x below incumbent; T13: t=4 + trace replay **0.2837 [0.2413,0.3335]** of T12 winner at w=0.1 (16/16 workloads), but 1.567x [1.359,1.807] at w=0 (order flips) (`curve25519_t4_candidate_ecbench_20261104`).
- Scale S1/S2 (k=20–32): algebraic arm behind baseline at every measured weight; at k=24 (2 of 6 solves finished) 44x / 9.2e5x / 5.3e6x behind rho-neg; generic gap to rho grows with k.
- `curve25519_autolab_20261104`: native autolab loop (`src/bin/ic_autolab.rs`) with ledger/ratchet; `curve25519_autolab_parity_20261105`: "run until parity with rho", results unverified pending replay.
- **Coverage**: ABSENT (no "Curve25519" in the treatise).

## 17. ecbench harness: accounting, statistics, overhead and instrument corrections

- Calibration (`ecbench_calibration_20261002`): seven generic methods reach their analytic constants at r≈2^25.4 (prime) and 2^39 (Koblitz); two round-1 predictions failed, traced to sample size.
- Accounting v2 (`ecbench_accounting_v2_20261023`, PR #2465): rho.cubic_v2 S/floor 1.419 [1.132,1.761] on j=0; strong v2/frozen S 0.182, 0.223, 0.678 at n=13,19,37; set-up share 83.3%→39.9% etc.; explicitly "not an improvement".
- Statistics v2 (`ecbench_statistics_v2_20261105`, PR #2442): linearised t(K−1) interval, ns/GAE wall statistic, censored timeouts/exhausted runs, tail flags, A/A bias correction; point estimates unchanged over verified pairs.
- Allocator confound (`ecbench_allocator_confound_20261023`): PR binary 0.137–0.330 ms vs 5.26–6.77 ms on small-set arms (allocator); IC online window not distinguished (1.067); claim-koblitz band 16.85 vs 4.00 (w=0), 21.16 vs 5.36 (w=1).
- Child overhead, alias-table/registry scan removal, tuned-walk bookkeeping (`ecbench_child_overhead_20261024`, `ecbench_child_walk_ir_20261101`), walk round 3 −9.7% instr per rho solve (`ecbench_walk_round3_20261102`), asm Euclid inversion −11.2% on default IC oracle (`ecbench_ic_asm_euclid_20261103`), overhead review (`ecbench_overhead_review_20261022`), NUMA VM pinning, dedicated two-socket hardware protocol (no hosts), same-machine replay changes env class (`ecbench_independence_20261009`, correctness regression), Callgrind n37 census.
- Registry-wide optimisation loop (`ecbench_registry_optimization_20261101`): 79 methods, 11 system solvers, 9 PDP3 engines, 4 PKM tower engines; 114 specs.
- Frontier v2 (procedure 2, scoreboard change), e2e frontier refresh 31 Oct: 74 live method ids (31 IC); no admitted result beats matched cold rho; n=29 one-column generator-orbit crossover 0.366 [0.241,0.523] of rho-frob classified as Frobenius-folded BSGS, setup-driven, gap grows with r.
- docs/ecbench: BACKEND_FRONTIER, CLASS_SETUP_AUDIT, DEFECTIVE_SESSIONS(.tsv), INCOMPLETE_SESSIONS.tsv, cpu-reduction, explicit-bases.
- **Coverage**: PARTIAL. prog_method describes ecbench v1 (S, GAE, sealing, replay, L0–L3, paired bootstrap, bounds, band). Missing: calibration result, accounting v2 and statistics v2 (censoring!), allocator confound, defective-session registry, registry inventory, e2e frontier refresh and the n=29 "crossover" classification.

## 18. ecbench as a method engine: measured arms vs rho (uncited sessions)

- All candidates one harness: no IC arm below matched rho; ic-frob-m2 4.30x strong rho; folded MITM prime 7.7x (18 bits), 40x (24 bits) (`ecbench_all_candidates_20261003`).
- Pair claw (cryptanalysis#175): 2.20x [1.90,2.55] strong rho at best shape, 32.6x [25.3,41.0] at the n=83 shape; does not shrink with r (`ecbench_pair_claw_20261003`).
- Frobenius-orbit IC fully priced: not below rho-frob on 8 Koblitz curves; IC÷rho-frob slope β=0.165 [0.117,0.213] per ln r (model 1/6); up to 15x constant vs subspace base at n=23 (`ecbench_frobenius_orbit_vs_rho_20261014`).
- Lazy orbit-member index 1.12–1.44x less GAE (n=13–37), 1.014x at n=41 (`ecbench_lazy_orbit_members_20261018`); columns=2 cheaper (tuning, needs fresh seeds); generator-orbit + orbit-layout net 1.26–3.20x over lazy baseline (VALIDATION dir present).
- batch_trials screen: batch16 0.69–0.80x of batch64, 2/5 predictions held; isolated confirmation PROPOSED only.
- Known-log two-sum: cheaper than abscissa pipeline across band at 16/20/24 bits; negation rho cheaper at every size.
- ic.residual_batch vs rho.multitarget_pooled (p36): per target 3.3–3.4x (L=16), 6.6–6.7x (L=64) cheaper than single-target rho; matched reference is multi-target rho (parity rule); 320/320 verified, L0.
- Jacobians/transfers first sessions (genus-2, plane quartic, GHS F_{p^6}, transfer.isogeny): new capability, not an improvement.
- Cheon DLPwAI: comb arms 0.552 (n−1) and 0.279 (n+1) of matched rho at w=0 over 36–56 bits, oracle counted separately; medium-tier protocol frozen 6 Nov.
- m=83: strong-rho reference cannot run as committed; escape variant; wide GF(2^m) support makes the gate runnable; m83 coefficient gate.
- Yield sweep (Oct 4): ledger populated across subtract/mitm/descent/sat-cdcl/buchberger/f4/crossbred; 11 sat-cdcl timeouts.
- Bounds challenges epoch 1: rho.signed_frobenius vs strong reads "advances at the constant level".
- **Coverage**: PARTIAL (Koblitz/prime chapters carry the leaderboard rows; the method-by-method ecbench record, slope β≈1/6 measurement, pair claw, residual batch and Cheon numbers are mostly absent; Cheon has one mention in prog_prime).

## 19. IC planner (`ic plan`, ic-planner/1–3)

- ic-planner/1 validation (cited, COVERED: rootfind alternative 150x–637x the pair-table plan).
- ic-planner/2 weight-aware base size (1.41·N0 at w=1) and ic-planner/3 (Koblitz 0.71·c0 at w≥0.3): **both withdrawn 2026-10-26** after failing preregistered tests.
- `ic_planner_exhaustive_20261026`: re-test with arithmetic checked in Lean: residual predictions R1–R5 held; null N1 held (1/16 intervals excludes 1); Koblitz routing K1 held, K2 failed narrowly; weight rules W1 (prime), W2 (binary), W3 (Koblitz) **failed**; 2,428 band verdicts restated; residual collector cheaper than the plan as a generic collision search from 36 bits.
- **Coverage**: PARTIAL; the withdrawal is a correction that the treatise does not record (no "weight rule" text).

## 20. Autolab and IC autolab infrastructure

- Sept 15 AutoLab/Harbor harness: 26.9% reduction in complete cold-solve instructions (1.367x; 95% 24.0–29.4%) vs batch16 baseline, 1,356 verified pairs; final candidate 4.98x matched rho's instructions (prog_koblitz cites next30 dir).
- Autolab cycles 17 Oct v2–v10 (uncited; v1/v3 of 19 Oct cited): native GF(2^83) norm predicate: cutoff-8 cuts AND 60.485%, clauses 32.914%; 60 s hard norm-primary searches inconclusive (0 verified models; 7,167–12,208 conflicts; Gaussian matrix peak 9,169×22,491); fixed-U transfer refuted (dimension 73, U-rank 52); reports kept local because approval review rejected publication.
- ic_sweeper plan (24 Oct, proposed), dreg-triage forecasts, Curve25519 autolab (item 16).
- **Coverage**: PARTIAL (prog_binary has the norm predicate in a few sentences; the autolab method itself is not described).

## 21. Hashed-ECDL / tensor-contraction engines (29 Oct – 4 Nov)

- Conditional manuscript route: hashed factor base with certified hash modulus; general contraction recursion whose asymptotic conclusion is conditional on a cited Boolean tensor-contraction claim (not validated here).
- `hashed_ecdl_cpu_20261030` (crates/hashed-ecdl-cpu): 120-execution matched panels, byte-identical certificates; cached batch64 vs original: p65521 m3 45.99 → 3.90 ms, p65521 m4 80.54 → 4.12 ms, p211 0.316 → 0.283 ms (L0, EKS VM); larger-field ladder expired with zero executions.
- `hashed_ecdl_finite_size_20261030`: five finite-size routes proposed (no measurement); ECCp-131 fixture; ECC2K-130 separate.
- `hashed_ecdl_entry_cost_20261104`: implicit tensor entries audit, claim **PCC-20261104-1df881** (standing: statement record, no proof artifacts, no signed verdict).
- `hashed_ecdl_speed/primality_fast_paths/hash_prime_20261030`: exact primality, certified hash prime for native ECCp-131; no speedup established.
- `hashed_ecdl_counts_20261031`: coordinator approve (conditional) of counted-S-only v2 protocol; nothing run.
- `tensor_boolean_breakthrough_20261031`: staged plan (E0 IR equivalence, E1 boundary-tensor rank census vs rewired nulls).
- `hashed_ecdl_leads_20261030`: program snapshot (152 atlas leads: 52 open, 17 blocked, 9 unexamined, 74 closed).
- **Coverage**: ABSENT ("hashed" appears in prog_method only for public target hashing).

## 22. GPU engines for IC

- Explicit relations on RTX PRO 6000 / RTX 4090 (30 Oct – 1 Nov): GPU factor-base construction + generic MITM relations + CPU elimination; full recovery v8 12/12 through 60-bit fields (770.270 s, 845.180 s cold), v9 one 61-bit recovery, one memory refusal; normalization product-tree 765 vs 4,608 field mults; packed query; full-cache queries. L0, wall_valid=false, no matched rho; benchmarks/20261030-explicit-relations-gpu is "not an admitted benchmark result".
- GPU root-finding oracle: host emulation 1,331 relations 0 missing/spurious; sm_120 throughput estimate only; five GPU pods failed at device init (cryptanalysis branch) (cited dir).
- `gpu_ic_baseline_20261027`: design only, blocked on lab change; `nist_gpu_arithmetic_20260926`: infra non-result.
- **Coverage**: PARTIAL for GPU rootfind (cited); explicit-relations GPU line ABSENT.

## 23. Relation-yield, decomposition-probability and learned-predictor audits (engine-adjacent)

- KN-FIND-007: Σ_targets c_D(r) = C(B+m−1,m) exactly; learned predictors inconclusive ×3 at N≤2^20; reopening test at N=2^24..2^26 **failed** (+0.00001 bits [−0.00022,+0.00021], headroom 0.0011; external ml-cryptanalysis repo, unverified paths); L07 negation-closed x-window deficit 0.713/0.833/0.916 at m=4; L03 sparse-parity weight ≥5 within 3% of dense.
- Monodromy uniform ((Z/2)^{m−2}, ⋊S_{m−1}); coset typing / trace-parity confinement (EV-SEMBIN-0c8bf4 preliminary).
- `relation_yield_pruning_20261031` (checkpoint analyzer), `target_free_relations_20261018`, `ml_dataset_*` (design/pilot only).
- **Coverage**: COVERED for yield conservation and monodromy (cover.tex, obstructions.tex); learned-predictor nulls ABSENT (only "learned" in abstract/intro); ml_dataset ABSENT (low value).

## 24. Generic-oracle arity floor (cost-model obstruction)

- Product law relations × targets × oracle = m·2^n independent of dim V; zero sub-rho cells for ENUM/MITM/MITM_CAPPED (EV-SEMBIN-b6d042); free m=3 oracle on ECC2K-130 2^68.58 vs rho 2^60.81 (KN-FIND-aa2efc); MITM oracle 928x–2,187x over pairs-and-solve, collection still Θ(2^n); Lean IcTensorRoute rebuilt (cost-model arithmetic only); literature amendment (Golovnev et al. STOC 2020 3SUM-Indexing).
- **Coverage**: PARTIAL — obstructions.tex "Oracle-augmented index calculus stays generic" covers the idea; "product law" wording, the 2^68.58 free-oracle number and the 3SUM-Indexing scope note are not found.

## 25. Linearization-excess factor-base prior (cryptanalysis)

- pdp-degree-heuristics: for m=2 on ECC2K-130 family, solving degree D = 2 + [e<−3.5] + [e<−8.5] in e = n − dim V − dim V^(2), MAE 0.06 over 656 bases (7 families, 9 fields); lead open_unexamined: never applied to m=3/4 closure cells.
- **Coverage**: ABSENT.

## 26. Joux–Vitse cover sieve engine (weak 2-torsion class; transfer-adjacent)

- Uncited newer sieve-engine studies: factor-base line search H 0.623 [0.562,0.689], exhaustive T 0.560 [0.500,0.631] (paired arm/P, 48 pairs) (`jv_sieve_fbline_20261102`); addition cuts S+ 0.621 [0.614,0.629], S 0.990 (`jv_sieve_adds_20261103`); vs negation rho pooled S/rho U1 0.00655, U2 0.01573 (`jv_sieve_negrho_20261102`); line vs rho P 0.0244 / H 0.0154 (U2) (`jv_sieve_line_vs_rho_20261103`). Weak class only (#E=4l, genus-3 cover over F_{p^2}); nothing on prime-order curves.
- **Coverage**: PARTIAL (JV well covered as a transfer; these sieve-engine refinements are not). Likely belongs to the transfer agent.

---

## Corrections / retractions / staleness flags for the treatise

1. **ℓ=8 hybrid-guess cells have run** (dreg_closure_guess 22 Oct: 4 guesses suffice; guess3 23 Oct: 3 do not for that set; invariant-guess studies 28–29 Oct; second-link 5 Nov, PCC-20261106-607061 unverified). prog_frontier/prog_binary still say the ℓ=8 run did not complete.
2. **ic-planner/2 and /3 weight rules withdrawn 2026-10-26** (failed preregistered tests; exhaustive retest: W1–W3 failed).
3. Inherited-F4 harvest default (#1862) raises S by ~10%/7% at n=11/13; only helps at n=19 (f4_harvest_remeasure).
4. F6-IC few-candidate closure collapses to the existing MITM oracle in the limit (producer's own [pcc:unverified] note).
5. Sweeper desk review: in-house F4 needs 10^3–10^7 more operations than a brute grid on PKM systems, so root-finding-vs-F4 ratios (e.g. 383x–46,911x) are not evidence about published Groebner solvers. The treatise quotes those ratios without that caveat.
6. ecbench statistics v2 changed intervals/censoring for committed sessions (point estimates unchanged); accounting v2 changed strong-rho set-up accounting (v2/frozen S 0.182–0.678); treatise intervals predating these may need the v2 caveat.
7. E42 killed by its own mis-derived control; E42b re-registered (not a retraction of a result).
8. The n=29 generator-orbit "S below rho-frob" (0.366) is classified by the repo as Frobenius-folded BSGS driven by fixed setup, not an IC win.

---

## Coverage table

| line of work | coverage | est. pages it deserves |
|---|---|---|
| 1 Boolean generic-search solver campaign | PARTIAL | 4 |
| 2 Matrix F5/F4 GF(2) micro-optimisation campaign (incl. refuted-variant table, arm64, Oct 29 follow-ups) | PARTIAL | 6 |
| 3 Callgrind instruction engineering F4/F5 | COVERED | 1 |
| 4 Inherited F4 (rounds, harvest, determinism, online gap, priced rho distance) | PARTIAL | 5 |
| 5 F6-IC (profiles, online gap, refutations, few-candidate closure → MITM) | PARTIAL | 5 |
| 6 F6 at n=83 engine campaign | PARTIAL | 5 |
| 7 F_p F4 engines, end-to-end effect, external CAS (msolve/Magma), sweeper caveat | PARTIAL | 5 |
| 8 Engine suites, solver selection, crossbred, lit2026 | PARTIAL | 3 |
| 9 Root-finding / pair-table engineering and track ruling | PARTIAL | 3 |
| 10 SAT / XOR oracles (ecbench_sat_pdp, ML-SAT, prime SAT design, N83 chain SAT) | PARTIAL | 6 |
| 11 MutantXL closure / hybrid-guess series incl. ℓ=8 results | PARTIAL (stale) | 6 |
| 12 dreg / syzygy / first-fall instruments and Lean tranche | PARTIAL | 4 |
| 13 Presentation/topology: tree split, tree gluing, GAG planner, chain cost model | PARTIAL | 5 |
| 14 Linear algebra engines (CADO comparison, filter port, sparse null, 2LP) | PARTIAL | 6 |
| 15 GLS F_{p^4} chain-solve Macaulay levers E21–E42 | ABSENT | 5 |
| 16 Curve25519 PKM-Semaev track, F4 trace replay, autolab | ABSENT | 6 |
| 17 ecbench harness: calibration, accounting v2, statistics v2, overhead, registry | PARTIAL | 8 |
| 18 ecbench measured method arms vs rho (pair claw, β≈1/6 slope, residual batch, Cheon…) | PARTIAL | 6 |
| 19 IC planner and its withdrawn weight rules | PARTIAL | 3 |
| 20 Autolab / IC-autolab infrastructure and GF(2^83) norm cycles | PARTIAL | 4 |
| 21 Hashed-ECDL / tensor-contraction engines | ABSENT | 5 |
| 22 GPU engines (explicit relations GPU, rootfind kernel, baseline design) | PARTIAL / ABSENT | 4 |
| 23 Yield / P(decomp) / learned predictors | PARTIAL | 3 |
| 24 Generic-oracle arity floor | PARTIAL | 2 |
| 25 Linearization-excess factor-base prior | ABSENT | 1.5 |
| 26 JV cover sieve engine refinements (weak class) | PARTIAL | 2 |
| **Total** | 22 PARTIAL, 4 ABSENT (plus 22 partly), 1 COVERED | **~118** |

## Suggested structure (engine part of the 300-page treatise)

Part "The machinery" (~110–120 pp), replacing the current 4-page prog_groebner chapter:

1. **Measuring an engine** (~14 pp): ecbench v1→v2 (S, GAE, sealing, replay, isolation tiers), calibration, accounting v2, statistics v2 with censoring, allocator and overhead confounds, defective-session registry, the §5a band as a ranking device (E33/E40 worked example), claim standing (pcc). [items 17, 15-method]
2. **Boolean decision engines** (~20 pp): generic Boolean solvers; matrix F4/F5 and the GF(2) kernel; the gate methodology and the full refuted-variant appendix; inherited F4; F6-IC to MITM convergence. [1–5]
3. **The n=83 engine campaign** (~6 pp). [6, 20-norm cycles]
4. **Prime and extension-field Groebner engines** (~16 pp): f4_fp, tower/signature F4, msolve/Magma, trace replay (Curve25519), GLS chain-solve Macaulay levers, the sweeper caveat. [7, 15, 16]
5. **Degree instruments** (~12 pp): MutantXL closure, hybrid guess with ℓ=8 results, invariant guesses, dreg/syzygy nulls, presentation (tree gluing, GAG, chain cost), linearization excess. [11–13, 25]
6. **Non-algebraic oracles** (~12 pp): root finding / pair tables engineering, SAT/XOR (incl. N83 chain SAT, ML-SAT ceiling), generic-oracle floor, GPU kernels. [9, 10, 22, 24]
7. **Linear algebra** (~6 pp). [14]
8. **Planners and autolabs** (~8 pp): ic-planner and withdrawn rules, autolab loops, sweeper, dreg-triage. [19, 20]
9. **Hashed-ECDL and tensor contraction** (~5 pp, conditional/unvalidated). [21]
10. **Engine-level method record vs rho** (~8 pp). [18, 23, 26]

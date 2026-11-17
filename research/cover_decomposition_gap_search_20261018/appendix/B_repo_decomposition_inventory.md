# Appendix B: inventory of decomposition / summation-polynomial work already in this repository (2026-10-18)

Compiled by a read-only survey agent for the gap search in the parent directory.
Three existing documents already index most of this: `docs/ic/factor-base-encyclopedia/README.md`,
`reports/Index calculus leads next steps.md`, and `research_notes/Index calculus leads next steps/*.md`.
Several items those notes call "never run" were run later (mostly `research/ic_next10_20261014/`);
statuses below are the latest found.

## 1. Prime-field E(F_p) decomposition

| dir/file | what | curve/size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` | Generic ceiling S·T²≳n; Coppersmith reach for S_{m+1}; Probes A, B, C | 32–56 bit | negative (no route) | δ_max = 1/6, 1/24, 1/80 for S₃, S₄, S₅; first arity that can beat rho is m=4 and needs S₅ at δ≈1/5 | small roots beyond Coppersmith; solution-count falsifier |
| `RESEARCH_PRIME_FAST_ECDLP.md`, `prime_fast_bench_20261008` | Native IC-2 / IC-3 | 16–32 bit | engineering | IC-3 at 32 bit: 20.5 s vs rho 0.5 s | — |
| `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`, `semaev_*`, `rootfind_faster_e2e_20261015` | S₃ root = x(R∓F_i) in place of Gröbner | 16–83 bit; NIST curves subroutine | engineering | 35–47k× vs groebner_f4; e2e 50–205× vs msolve | m = 3, 4 and NIST end to end not measured |
| `RESEARCH_GPU_ROOTFIND_DECOMPOSITION.md` | CUDA S₃ root kernel | host emulation | blocked (no nvcc) | estimates only | — |
| `lit2026_w10_s5_coppersmith_20261011` | Exact LP over Coppersmith shift strategies S₃–S₅ | asymptotic | negative | ceiling 1/(m·d) | sublattice strategies, Gröbner-optimal shifts, non-box bounds, finite-p effects |
| `ic_next10_20261014/05-multipoly-lattice` | Several polynomials, shared targets, curve equations | — | negative | best S₅ δ* ≈ 0.01398 | unravelled linearisation "argument, not a theorem" |
| `ic_next10_20261014/02-solution-count` | F̄_p solution count of {S_{m+1}, L(x_i)} for Kummer, subgroup, Dickson, small-x bases | toy p | negative | ratio to random 0.93–1.03 over 172 cells | says nothing about whether a solver must walk the partial system |
| `RESEARCH_PKM_TOWER_ORACLE.md`, encyclopedia §0.1, `ic_next10/03-pkm-degree` | PKM16 tower bases (Kummer, isogeny, Dickson) with f4_fp_tower and msolve | p₁ = 15·2^27+1, N ≤ 22 | m=4 NEGATIVE (D ≥ 8 at N=20); m=3 inconclusive (D ≥ 7); m=2 "H0 at threshold" | D(m=2): 5,5,5,5,6,6; D(m=3): 6,6,7,7; D(m=4): 6,7,7,≥8 | m=3, N=21 needs 130–150 GB (card blocked) |
| `pkm_dickson_torus_20261015`, `f4_generic_field_20261015`, `pkm_isogeny_tower_*`, `pkm_supersingular_aux_20261015` | Dickson torus; generic-field F4 at NIST primes; isogeny-tower plug-in | up to P-256 solver only | H_same / accounting | D equal to small-prime values in 82 cells; t=48 search ≈ 11.2 core-years | no F4 run at P-192 yet |
| `ic_next10/04-kummer-fourier`, `ic_exponent_falsifiers_20261013` | Character spectrum of decomposition exponents | p ≤ 2^24 | negative | 0/31 cells above null | m ≥ 4, nonlinear structure untested |
| `icv1-fp256-…_round{1..24}` (P-256 Dickson-torus campaign) | Depth-18 Dickson trace base; bit-box; PGL₂ orbits | real P-256 | refuted | 394.5× rho (round 24) | — |
| Encyclopedia §2 A1–A13; `factor_base_evaluation_20261015`; `greedy_factor_base_yield_20261014`; `prime_small_x_p192_20261014` | Small-x, geometry, automorphism orbits, coset/subgroup, Eisenstein, AP, mod-ℓ, map images, isogeny-augmented | 12–32 bit; P-192 | refuted / yield conservation | pair-table law S=0.75·r^{1/6}; P-192 yield exactly 2B²/(N−1) | — |
| `pair46_factor_bases_20261015` | 14 families on a CM(−19) 53-bit pair: endo-chain, Deuring lift, floor pullback, QR window, PKM, ψ_m torsion-division, learned, CM-norm, bounded-degree, Montgomery/Edwards/Hessian, y-windows | toy and 53 bit | no advantage | — | ff-xedni "blocked, did not run" |
| `prime61_factor_base_20261016`, `prime61_isogeny_20261016`, `prime61_disk_walk_20261017` | Native collector; horizontal-isogeny panel | p=2^61−1 | null / partial | 0 relations in 1.2M trials | class not exhausted |
| `lit2026_w12_flagged_claims_20261011` | Zero-minor guess-and-determine (Mahalanobis); Hu 2025/015 | 16–26 bit | negative | 260–6,715× rho | — |
| `lit2026_w3_ggm_audit_20261011` | Configurations vs CGHW 2026/384, DKM, Akshima bounds | all IC configs | on the generic line | — | — |
| `prime_field_smt_20261005` | cvc5 QF_BV / QF_FF as PDP oracle | toy | inconclusive | — | — |
| `factor_base_designs_20261015` | 24 designs with Lean proofs | prime and binary | 5 measured no; 11 "open: no slope change"; 3 closed by Lean | — | designs 2, 3, 5–10, 12–16 open |
| `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`, `prime_rho_automorphism_20261015` | Endomorphism folds (j=0, 1728, GLS, CM) | 2^13–2^33 | engineering | fold = order of eigenvalue subgroup | — |
| `elliptic_snfs_survey_20261016` | What an elliptic SNFS would need | survey | towers = "shape C"; open asymptotic cost | — | — |

Extension-field calibration: `gaudry_quartic_d4_20261013` (48×), `d4_symmetrised_s5_20261015` (≈850× weighted; handover 2^118.9), `gaudry_quartic_d4_cost_20261014` (crossover model 2^103), `ic_next10/01-fp4-ladder` (negative), JV12 cover (0.039–0.069× rho, weak class only), genus-3 after cubic GHS (`ic_next10/07`: exponent 1.356 vs 1.5, still 1.62× rho at 2^15).

## 2. Koblitz decomposition

| dir/file | what | curve/size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `RESEARCH_KOBLITZ_INDEX_CALCULUS.md` | GGMP Frobenius-invariant bases; enumerate / F4 / SAT / pair-table oracles | n ≤ 41 | positive at toy size | savings ≈ n relations, n² LA; invariant dimensions at n=131, 163 only {0,1,n−1,n} | block/homogeneous Gröbner structure, multiple invariant bases |
| `RESEARCH_KOBLITZ_SCALING_TARGET.md` | Chained-S₃ unknowns m·ℓ+(m−2)·n | to n=63 | FFD ≤ 3; H5 falsified | 633 unknowns at n=63, ℓ=6 | symbolic S₄ links not done |
| `RESEARCH_ECC2K130_IC_FEASIBILITY.md` | Compact-orbit 4-sum extraction | n=61–83 | online-only wins | needs K ≥ 10⁹ at challenge scale | — |
| `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` | Generic bound (CGK/Shoup), symmetry inventory | — | analysis | IC online 2^25–2^27 above the S·T² line | §9 Do list |
| `RESEARCH_FREE_RELATIONS_CENSUS_20261012.md`, `ic_next10/09`, `ic_improve*` | Pair-root self-collisions | n=61, 83 | closed for rank collapse | homogeneous rank 49/599 (n=61), 6/599 (n=83) | — |
| `ic_next10/09-equal-advice` | Bernstein–Lange preprocessing rho at equal advice | n=41, 53, 61 | positive audit | BL rho 50–500× faster online | — |
| `RESEARCH_G2_EQUIVARIANT_F4_SCOPE.md`, `g2_equivariant_f4_20261008`, `ic_next10/06` | Faugère–Svartz C_n blocks | n=7–23 | positive constant | rank ratios 31–128×; complete pass bit-identical, 44–73× CPU | not wired into `reduce_system` |
| `ic_next10/06-equivariant-f4-m4` | msolve F4 degree at m=4 | (11,4), (15,5) | levels off at 4 / blocked | F4 degree 4 where XL reads 5–7 | (4,5), (4,6) blocked at 3 GB |
| `dreg_closure_*_2026101{6,7}` | MutantXL closure refutation degree | N ≤ 42 | closure 3–4; rises at m=3, ℓ=7 | m=4 at 2^71.13 | m=4, ℓ ≥ 6 not measured |
| `RESEARCH_DREG_MEASUREMENT.md`, `RESEARCH_FALL_DEGREE_BOUNDS.md`, `dreg_*` | XL refutation degree vs FFD | n ≤ 15 | XL degree grows 5, 6, 6–7, 7; FFD=3 everywhere | (15,5) at degree 7–8 needs ~128 GB | — |
| `RESEARCH_ECC2K130_DESCENT_DEGREE.md` | n=131 pricing | model | negative | m=3 loses even with a free oracle; m=4 spans +15.5 to +112 bits | — |
| `linearization_reach_20260930`, `xl_bilinear_core`, `xl_trilinear_t_unknown`, `geometric_v_linear`, `cheap_closures`, `graph_cycle_ic` | Linear pair oracle; XL cores; geometric V; cycle IC | n=131 model + toys | boundary | best gap 42.47 bits (55.22 charged) | — |
| `decomposition_programme_20261016/PLAN.md` | W1 ceiling (Hou–Leung–Xiang), W2 mixed conjugates, W3 4/5-summand test, W4 rho constants | n=131; toys | PLAN only, nothing run | budget ≤ 2^11 (same conjugate) or 2^25–2^34 (mixed) per attempt | mixed-conjugate 4/5-summand test |
| `cyclotomic_falsifier_20261015` | γ-weight base (μ_{2n+1}) | n=23 | yield-neutral | — | solver clause, n=83 |
| `lit2026_w8_summation_models_20261011` | Dryło E/⟨T⟩; Wu Kummer; Bariant Frobenius auxiliary; La Scala convolution | n ≤ 23, m ≤ 3 | eg 1.34–1.64× fewer ops; others negative | — | binary symmetrised S₅ absent |
| `RESEARCH_EXOTIC_COORDINATES.md`, `koblitz_symmetrised_e2e_20260927` | FGHR u-frame; Möbius frames | n ≤ 31 | 30–416× on the solve; arms 990–63,500× rho | — | — |
| `RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` | Shape axis | n=15 | trace artefact | — | — |
| Encyclopedia D3; `transfer_aware_ic_20261010` | Z[τ] multiplier closure | toy | engineering | 4.5× rho | — |
| Encyclopedia D8; `n83_*` | N83 compact orbit; chained-S₃ SAT m=5/6 | n=83 | online only | cold 218× rho; SAT m=5: 0 relations | — |
| Encyclopedia D9; `ic_techniques_20261012/double_large_prime` | Large primes | n=23–41 | 17.7× rho at n=41 | — | sweeps not run |
| `ic_m3_algebraic_vs_mitm_20261014`, `pdp_shape_replay_20261015` | F4/F6/SAT vs MITM floor; root replay | m=3/4 | negative | replay 0.95–0.98× | — |
| `ic_frontier_campaign_20261015`; `docs/ic/LEADERBOARD.md` Table A | Band-priced frontier | n=13–61 | rho cheaper everywhere | 3.8–30.3× | — |
| `ic_next10/08-quasi-subfield`, `RESEARCH_QUASI_SUBFIELD.md` | QSP search | prime n ≤ 61 | negative | β 0.778 vs 0.1025 | — |
| `RESEARCH_ECC2K130_ROUTES.md`, `_CROSSBRED.md`, `_WDSAT.md` | Routes 1–4 | n=131 | — | — | Route 2 (Crossbred on u-frame), Route 4 (symmetrised S₃ chain m ≥ 4) open |

## 3. Binary non-Koblitz decomposition

| dir/file | what | curve/size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `binary_mitm_oracle_l{8,9_l10,11_l12}` | Pair-table MITM vs Gaudry pairs-and-solve | n=25–37, l=8–12 | engineering | 94–4,244× vs pairs-and-solve; no rho ratio | ℓ = 13 probe; FFD-logged control |
| `docs/ic/LEADERBOARD.md` Table B | Random binary ladder | n=15–27 | above rho | 10.6–38.2× | — |
| Encyclopedia C1–C11; `shape_search_g1_g6_20261015` | Window, random, subfield, GG/ST, Hamming ball, quasi-subfield, affine plane, full-density | n ≤ 23 | G1, G3, G5 killed; G2 survives at n ≤ 9 | refutation degree set by l alone | G4 not built; G6 not run |
| `factor_base_dor_campaign_20261013` | DoR by shape | n ≤ 23 | empirical negative | — | — |
| `hamming_ideal_pdp_20260930` | La Scala Hamming ideals | n=11–19 | H1 refuted | — | — |
| `nagao_relations` | Riemann–Roch incidence and norm encodings | binary toy | incomplete | — | S/rho unmeasured |
| `full_binary_norm_*`, `actual_norm_*`, `autolab_cycle_20261017_v2..v7` | "Generalized all-roots" program: norm of P+(hX+k)Y on GF(2^83), CMS-Gauss SAT | GF(2^83) | inconclusive / active | 43 of 131 experiments inconclusive | long-XOR native interface |
| `gaudry_quartic_d4_char2_20261015` | k=4 D₄ system over F_{q⁴}, q=2^m | m ≤ 8 | engineering | crossover extrapolated near 2^68 | — |

## 4. Solver engineering

| dir/file | what | verdict | open |
|---|---|---|---|
| `boolean_f4_*`, `boolean_f5_*`, `f5_*`, `f6_ic_*`, `inherited_f4_*`, `fp_f4_*` | F₂/F_p F4/F5/F6 kernels | engineering constants | 48 inherited-F4 replay mismatches across hosts |
| `gaudry_quartic_replay_20261013`, `pdp_shape_replay_20261015` | F4Remake / trace replay | 2–7%; 0.95–0.98× | G1 trace reuse at n=31 binary unimplemented |
| `dreg_closure_mutantxl_20261016`, `xl_*` | MutantXL, XL | closure 3–4 vs XL 5–7 | hybrid guess only at ℓ=7 |
| `crossbred_20260920`, `RESEARCH_ECC2K130_CROSSBRED.md` | Crossbred oracle | slower than enumerate | lit2026 W6 not started |
| `RESEARCH_SAT_SEMAEV.md`, `wdsat_rr_20260920`, `pdp_smt_oracle_20261008`, `f6_n83_*_sat` | CDCL+XOR, WDSat, CryptoMiniSat, cvc5/Z3 | "SAT route is exhausted" | — |
| `lit2026_w4/w5/w11` | Early abort, lane batching, gate-first F6 | gate-first 1.57–1.78× | W1, W2, W6, W7, W9 not started |
| `SPARSE_ALGEBRA_FOLLOWUP.md` | Sparse vs packed | no gain shown | chordal study not done |
| `magma_f4_comparison_20261013` | Registered Magma comparison | waiting | — |

## 5. Symmetry levers measured

negation; Frobenius (GGMP; G2 31–128×); CM folds; FGHR 2-torsion 30–416×; D₄ 48× → ≈850×; S_m elementary-symmetric (S₄–S₈ unmeasured); 4-torsion/Klein negative; isogeny class identical ranks.

## 6. Explicitly stated gaps (decomposition side)

1. `decomposition_programme_20261016/PLAN.md`: W1–W4 have no result directory. Open: a test for mixed-conjugate 4- or 5-summand decompositions at ≤ 2^25–2^34 per attempt.
2. Encyclopedia §8: m=3, N=21 PKM cell (130–150 GB) blocked; F4 run and relation phase at P-192 open; m=3, N=15 at P-224 draft.
3. Shape search: l ≥ 5, frozen G1 cells (47–50 GB), G4 (trace-form non-subspace Frobenius-stable sets).
4. Cyclotomic base: solver-degree clause untested; n=83 not run.
5. Binary MITM oracle: FFD-logged control; ℓ=13; no rho ratio.
6. Large primes at n=41: 5- and 20-column sweeps not run.
7. Equivariant F4: (4,5), (4,6) blocked at 3 GB; not wired into `reduce_system`.
8. D₄ with double large primes stacked arm not run.
9. MutantXL closure at m=4, ℓ ≥ 6 not measured.
10. Solving-degree census with symmetries not run.
11. Coppersmith: non-full-rank/sublattice strategies have no general ceiling; Gröbner-optimal shifts; non-box bounds.
12. Prime-field D₄-symmetrised S₅ lattice probe at m=4 (leads note §9 item 5): no run found.
13. S₄–S₈ elementary-symmetric rewriting unmeasured.
14. Novel directions G3 (BooleanSolve/block Wiedemann), G4 (mixed-field F4), N1 (group-algebra lift F₂[Z/263]): no result.
15. Koblitz: block/homogeneous structure for m ≥ 3; multiple invariant bases; symbolic S₄ links.
16. Routes 2 and 4 of `RESEARCH_ECC2K130_ROUTES.md` open.
17. `pdp_preregistrations_20261007` D-1 (Artin–Schreier coordinates), D-2, D-3, E-1, E-2: no results.
18. Koblitz queue threads 1–12: no completion records.
19. `factor_base_designs_20261015` designs 2, 3, 5–10, 12–16 open.
20. lit2026 W1 (saturation exponent), W2 (multi-graded regularity), W6, W7, W9 not started.
21. `lit2026_w8`: K1/F_{2^17} m=3 unfinished; 4-torsion not attempted; no binary symmetrised S₅.
22. Generalised all-roots program: unresolved; card `XP-20261017-quintic-support-frobenius` blocked.
23. Rootfind: m = 3, 4 and NIST curves not measured end to end; GPU kernel never compiled.
24. ff-xedni (pair46 item 11): did not run.
25. Joux–Vitse system with deg μ ≥ 1 (weak subfield curves) not run.
26. `ic_next10/07` plane-quartic branch blocked.

Rejected on paper (no measurement): symmetric S₃ rewriting; multiplicative-subgroup bases; global lifting/xedni; embedding into an extension; further 4/5/6-sum extraction designs; first-fall-degree measurements; m=2 shape search; isogeny/volcano/Kani for Koblitz; Wagner; p-adic variants; BKK/Newton polytopes; monodromy censuses; Couveignes–Lercier (Hasse bound); height box; degree-d map bases.

## 7. Literature cited in the repository (for the not-consulted check)

Bibliographies and notes cite: Pollard; van Oorschot–Wiener; GLV; Semaev 2004, 2015; Gaudry 2009; Diem 2011 (×2), 2013; GGMP 2020; Petit–Kosters–Messeng 2016; FGHR 2014; FHJRV 2014; F4/F5; Trimoska–Ionica–Dequen; Jao–Miller–Venkatesan; McGuire–Mueller; Amadori–Pintore–Sala; Petit–Quisquater; Shantz–Teske; Huang–Kosters–Yeo; Galbraith–Gebregiyorgis; Galbraith–Gaudry; Corrigan-Gibbs–Kogan; GTTD; GHS; Bernstein–Lange; FPPR; Joux–Vitse (×3); Faugère–Svartz; BFSS; Kosters–Yeo; HKYY; Kousidis–Wiemers; HPST; HKPYY; Euler–Petit; Courtois; Karabina; Nagao; Tsakou–Ionica; Couveignes–Lercier; Caminata–Gorla; Jochemsz–May; Coppersmith; Boneh–Durfee; Herrmann–May; cuso; JKSST; Shoup; Kuhn–Struik; La Scala et al.; Dryło; Wu; Bariant; CGHW; DKM; Akshima; Kudo–Yokoyama; Ning–Ran–Samardjiska; Gorla–Trebiani; Hu; Mahalanobis; Bender et al.; Cifuentes–Parrilo; Kera; HATSolver; Bhati–Singh; Wenger–Wolfger; Momose–Chao; Menezes–Teske; Frey–Rück; Satoh–Araki/Smart; Stange/Shipsey; Kudo–Yokota.

Cited secondhand or not implemented: Momonari–Kudo 2018; Amadori–Pintore–Sala (doc comments only, no experiment); McGuire–Mueller (no implementation); Galbraith–Gaudry survey (abstract only); Gaudry JSC 2009, Joux–Vitse JoC, Petit–Quisquater, GG 2014, FGHR, GHS/Menezes–Teske, Faugère–Svartz, GTTD, Silverman–Suzuki, Huang–Kueh–Tan marked "[unverified-recall]".

Apparently not consulted (zero or near-zero hits): Huang–Raskind; Delaplace–May; Diem–Kochinke; Hess 2003 (read only via secondary sources); Howe–Nart–Ritzenthaler; Zhang–Liu; Cheng–Huang; Galbraith–Petit–Silva; Plantard; Kirchner; Verbel; Eberly; "ideal lift"; chordal elimination.

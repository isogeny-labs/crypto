# Coverage digest: family ic-binary (binary-field / Koblitz IC, Weil descent, N83, ECC2K-130, ECC2-131, GHS/composite degree)

Survey date 2026-11-06. Read-only. Sources read with `git show origin/main:` (crypto, crypto-autoresearcher) and `git show HEAD:` (cryptanalysis, branch `claude/cryptanalysis-repo-setup-p59kek`, not main). Treatise = `papers/ecdlp-index-calculus-treatise/sections/*.tex` on crypto origin/main. Abbreviations: cr = crypto, ar = crypto-autoresearcher, ca = cryptanalysis.

**Standing.** Unless stated otherwise every number below is quoted from a producer README/REPORT or an atlas lead. Under the 2026-11-04 "sabotage is assumed" rule, none of these carries a pcc verdict unless noted; treat them as **unverified assertions** with the stated validation status. Only one item in this family was found with a `VALIDATION.json` from an independent validator (line 16, fb-cost-model on EKS, per the atlas lead; I could not open that repo). `dreg_second_link_guess_20261105` carries claim `PCC-20261106-607061` (unverified).

Boundary notes: the Joux–Vitse cover/sieve dirs (`jv_cover_*`, `jv_sieve_*`) are E(F_{p^6}) prime-field work and go to the extension/transfer family. `e28_tau_20261030` is over F_{p^4} (not binary). The char-2 D4 studies (`gaudry_k4_char2_*`, `ic_d4_char2_lever_20261023`) are F_{(2^m)^4} and are already in `extension.tex` ("Characteristic two"). The small-field GHS structural series (`ghs_q16_*`, `ghs_q8_*`, `ghs_cubic_*`, 24–28 Oct) overlaps the transfer family; it is summarised briefly in line 24.

Uncited crypto research dirs matched to this family: about 400 of the 1,205 (the `n83_*` and `f6_n83_*` prefixes alone are ~270; the treatise already summarises most of the 4–21 Oct N83 dirs as clusters).

---

## 1. Frobenius-invariant / compact-orbit factor bases (the orbit fold)

- **Tried.** GGMP invariant bases from factors of x^n−1, divisor bases, SAT-optimised bases, rank-guided base (n=53), compact signed-Frobenius orbit base (incumbent of every Koblitz row). Sept–Oct 2026; re-check 2026-10-28.
- **Numbers.** Orbit collapse 7.86/10.89/12.96 at n=9,11,13 (relations), 61.7–168 (LA), against rho's 4.24–5.10. ecbench factor-base suite: compact orbit S=9.114 at n=17, 9.985× matched signed-Frobenius rho (full orbit 18.7×, subspace 47×). **Closure (lead):** for E/F_q over F_{q^n}, {x(P)∈V} is π-stable iff V is σ-stable, and the stable subspaces form a lattice fixed by the factorisation of X^n−1. The gain is exactly n on relations (n^2 on LA) and never more (H-FROB-6774fb supported, DEC-20260923-e6cd5d). The tau-factor-base pilot finds per-attempt decomposition probability at matched |FB| equal to the birthday value for orbit-union, normal-basis and polynomial bases (n=11..23). At prime n with ord_n(2)=n−1 (131, 163) the usable stable dimensions are empty (H-BINSTD-b1f016 replicated). Rho gets the matching sqrt(2n), so the matched gap is unchanged. Prime-field φ-stable bases (EQIC): +4% Ir at order 6 on j=0. **2026-10-28 recheck (fb-koblitz-orbit, n=15, 31):** best stable subspace beats the cost-model non-stable base by D=4.73 bits [3.44, 6.10] only through the orbit credit (log2 n = 3.91, 4.95, an accounting identity). Priced as a generic base, D_nofold=0.56 bits [−0.51, 1.52]. Gap to matched rho (formula) 22.6–33.0 bits.
- **Verdict.** closed_obstructed (proof). The fold is a fixed constant n.
- **Sources.** ar:knowledge/leads/ecdlp/ic-binary/LEAD-ICB-frobenius-orbit-factor-base.yaml; cr:RESEARCH_KOBLITZ_INDEX_CALCULUS.md; cr:research/ecbench_frontier_20261013/; ar:analysis/ecdlp2m-iteration-20261009/tau-factor-base-pilot; ar:analysis/frobenius-orbit-ecc2k130.
- **Treatise.** PARTIAL. prog_koblitz §"Frobenius-invariant and compact-orbit bases" gives the construction and the ecbench numbers. Missing: the closure theorem stated as a theorem (gain exactly n, never more; stable-subspace lattice; empty at ord_n(2)=n−1), the birthday-value pilot, and the 10-28 orbit-credit-vs-no-fold recheck. These are the main "why the Koblitz lever is a constant" statements.

## 2. Non-linear (non-subspace) Frobenius-stable bases at prime n

- **Tried.** Trace-spectrum V_E={x: Tr(x^e)=0}, Hamming balls in a normal basis, multiplicative subgroups, τ-adic patterns over an orbit-union base, cyclotomic-sparse bases, F_2-points of low-dimensional abelian varieties (Couveignes–Lercier). 2026-10-15 to 10-29.
- **Numbers.** Trace-spectrum per-attempt cost (CryptoMiniSat conflicts, matched |F|): R=10.2 [8.6, 11.8] (RC-1, n=17), 11.6 [8.3, 15.4] (Koblitz n=19), 12.5 [10.7, 14.4] (Koblitz n=17, a2=1). Above the 2× band, so the branch is closed. Hamming ball B_3 against 8 matched random subspaces (T=16): R=0.27 [0.13, 0.46], 0.32 [0.14, 0.55] at n=19; 0.43 [0.20, 0.76], 0.29 [0.11, 0.54] at n=17. NOT DECIDED (interval excludes 0.5 in 3 of 8). An earlier n=19 value of 1.10 came from one noisy denominator. **v2 separation (T=32, w=2..4):** per-attempt cost is a sparsity effect (random-basis balls R=0.22–0.40; normal vs random ball D=+0.20 bits [−0.05, +0.45]; null gate failed). Exact yield P_sys is higher for the stable ball at w=2 (×1.76, 8 of 8 units, 48 bases per class). A per-relation stability benefit of −0.66 bits [−1.10, −0.24] is post hoc. Encoding size at n=131: |B_6|=2^32.6; Sinz counter 780 aux vars and 1,684 clauses per block, against 98 native XORs. Cyclotomic-sparse bases are yield-neutral at n=23 (effects −0.49%..+0.65%, positive control p=0.001), cost ratio 0.88–1.12. Orbit-union buys exactly log2 n (7.03 bits at n=131). The τ-adic base is a Γ_w-orbit quotient that reaches rho only at w≥13. Couveignes–Lercier: dims 1–4 excluded, dims 5–6 impeded (EV-FROB-a85eaf; EV-BINSTD-2d41c3 preliminary).
- **Verdict.** open_next_step. Trace-spectrum is closed (negative). The Hamming ball is open: undecided, with a sparsity confound.
- **Sources.** ar:LEAD-ICB-nonlinear-frob-stable-bases.yaml; experiments fb-koblitz-hamming, fb-koblitz-hamming-v2 (repo not local); cr:benchmarks/20261015-cyclotomic-falsifier; ar:ledger/evidence/EV-BINSTD-1e1815.yaml.
- **Treatise.** ABSENT, except the cyclotomic γ-weight base row (1.022, yield-neutral) in the prog_frontier table. There is no account of the trace-spectrum closure, the Hamming-ball stability-vs-sparsity result, or the Couveignes–Lercier census.

## 3. Generic table oracles and the CGK obstruction (production pipeline)

- **Tried.** Pair tables, compact-orbit 4-sum, MITM/enumeration, cap-locus arity, known-log pools. A 4,848-execution frontier campaign.
- **Numbers.** Shoup T ≥ sqrt(r/|Aut|) and Corrigan-Gibbs–Kogan S·T^2 ≳ r/|Aut| apply. Balanced generic-IC cost is about r^{2/3}/n^{4/3}. Rungs sit 2^25–2^27 above the generic line. ln(IC/rho) grows 0.165 [0.117, 0.213] per ln r (ecbench_frobenius_orbit_vs_rho_20261014: constant-only on 8 Koblitz curves, up to 15× against the subspace base at n=23). At ECC2K-130 this route needs K* ≈ 4e11 columns and ≈ 8e9 core-years. Frontier campaign: compact orbit 3.8–15× signed-Frobenius rho at w=0. Table A one-target 5.23× (n47) to 21.0× (n61) as published, 9.13–41.1× at the canonical step.
- **Verdict.** closed_obstructed (theorem). Constant engineering remains valid.
- **Sources.** ar:LEAD-ICB-generic-table-oracle.yaml; cr:RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md; cr:benchmarks/20261015-ic-frontier-campaign; cr:research/ecbench_frobenius_orbit_vs_rho_20261014 (uncited dir); ca:experiments/pdp-scaling (+35 to +51 bits at n=131).
- **Treatise.** COVERED: obstructions §"Oracle-augmented index calculus stays generic", prog_frontier §"The measured frontier", prog_theory lower bounds. Minor gap: the ecbench_frobenius_orbit_vs_rho dir (the source of the 0.165 slope) is uncited, and the K*≈4e11 / 8e9 core-year extrapolation is not tied to this theorem.

## 4. Koblitz set-up phase, Table A rules, and pipeline routing

- **Tried.** Rules v3–v8 and v4 recipe (Table A), pipeline baselines v4→v8, per-size K retune, orbit fast path, cache-blocked pair table. Planner routing between the §23 one-target descent and the `ic.pipeline` plan (2026-10-26).
- **Numbers.** In Table A the reusable set-up is 98.5–99.7% of S and the descent at most 1.4%. Rule v4 (32 pairs a size, 16 held-out seeds): cold 0.953 [0.904, 0.998] at n57 only, 0.963–1.013 elsewhere, PARTIAL (1 of 6), a null for the board. The cryptanalysis n=83 ratchet reads cold IC / tuned rho 3.92. **Routing study (uncited):** in counted S the §23 descent costs 1.41–1.49× the `ic.pipeline` plan: k47 1.421 [1.310, 1.541], k41 1.489 [1.433, 1.547], k53 1.409 [1.382, 1.438]. Pipeline/rho is 9.46 [7.93, 11.30], 16.19, and 23.99 [18.92, 30.42]; descent/rho is 13.45–33.81. The follow-up at six sizes (36.6–47.2 bits) reads 1.31–2.04×. It explains why Table A read lower: there rho is priced in canonical step units, while ecbench's `rho.signed_frobenius_strong` counted S is 0.11–0.14.
- **Verdict.** Table A: a null on the board. Routing: the planner's existing recipe stands. The distance to rho grows with size.
- **Sources.** ar:LEAD-ICB-koblitz-setup-phase-cost.yaml; cr:research/ic_tool_program/rule/v4/README.md; cr:research/ic_koblitz_descent_routing_20261026/README.md; cr:research/ic_planner_exhaustive_20261026; ca:docs/ic/BASELINES.md.
- **Treatise.** COVERED for Table A (prog_koblitz §"Leaderboard Table A") and the 3.92 ratchet. ABSENT: the 10-26 routing/planner study, which gives the cleanest same-unit Koblitz IC/rho series (9.5–24×), and its explanation of the Table A unit mix.

## 5. The matched Koblitz rho reference

- **Tried.** Canonical-step pricing, a strong fixture vs ParallelRho, normal-basis canonicalisation, signed-σ GPU walk, a 131-bit walk (not built), volcano level-1 rho.
- **Numbers.** Rule v4 with same-process canonical rho: IC/rho 9.9× (n47) to 43.5× (n61), 1.76–1.94× the as-built Table A gap; §5a band 17.9–65.7. The strong fixture needs 0.845 ± 0.08 of ParallelRho's steps, and admissibility failed at 4 of 6 sizes, so there is no VALIDATION.json ("strongest_available" unresolved). Normal-basis canonicalisation makes signed-Frobenius rho 4.1× cheaper (ca:papers/frobenius-orbits). **volcano_koblitz_layers_20261018 (uncited):** level-1/crater negation rho intervals all contain 1 (n=19 1.089 [0.993, 1.195] … n=61 0.837 [0.625, 1.073]). The Frobenius solver over the negation solver is 0.150 [0.132, 0.170] at n=41 against the ideal 1/sqrt n = 0.156, about a sqrt n gain at n ≥ 41. volcano_koblitz_n83: Φ_6473 built (606,365-bit coefficients, 8,590 s), 78 Frobenius orbits of size 83.
- **Verdict.** open_next_step (accounting).
- **Sources.** ar:LEAD-ICB-koblitz-rho-reference.yaml; cr:research/validation_sweep_20261019/A-leaderboard/REPORT.md; cr:research/ic_validation_gate_20261024/README.md; cr:research/volcano_koblitz_layers_20261018/RESULT.md; cr:research/volcano_koblitz_n83_20261018/RESULT.md.
- **Treatise.** PARTIAL. The 1.7–2.0× understatement and the withheld v4 file are in prog_koblitz §Table A, and the September reversals in §"Against a stronger rho". Missing: the 0.845 strong-fixture finding, the measured sqrt n Frobenius gain on the volcano layers, and the "no 131-bit walk, so no ECC2K-130 matched rho" point.

## 6. Large-prime variants on Koblitz/binary IC (late October; ABSENT)

- **6a. icb_large_prime_n53_n61_20261029 (crypto PR #2850, 612 replays reproduced).** On the classic compact-orbit m=3 Frobenius pair-table pipeline, counted S:
  - n=53 (log2 r 44.3): best plain 6.38 (440 columns), `lp2-k40-c10` 3.86, paired 0.604 [0.599, 0.610] over 16 workloads; 0.604/0.635/0.730 at w=0/0.1/1; arm/rho 30/31/35.
  - n=61 (log2 r 47.2): plain 7.62 (880 columns), `lp2-k60-c10` 4.26, paired 0.559 [0.535, 0.578] over 3 workloads; 0.559/0.623/0.831; arm/rho 47/52/64.
  - One large prime is worse (1.47–1.73×).
  - Standing: replay certificate and validator report present, no VALIDATION.json found. The atlas marks the claim UNQUALIFIED: a component result, not Table A.
- **6b. ic_koblitz_dlp_tuning_20261023 (report 10-28).** Tuned double-LP vs frozen `ic-compact` baseline, 96 holdout targets per curve. Class **null**: unresolved inside the band at n=37, 43, 47, 41, and worse at w=0 and 0.1 at n=41 and n=47. n=37: 1.001 [0.945, 1.061] / 0.793 / 0.570 at w=0/0.1/1.
  - This contrasts with 6a: the large-prime gain appears against the plain pair table but not against the tuned compact baseline. Both studies are silent on reconciling the two. **Flag for the treatise.**
- **6c. orbit_aware_large_prime_20261011 (measured 10-26).** Quotienting large-prime vertices by signed Frobenius orbits (±λ^k phase kept) gave more verified relations than plain DLP on the same stream at n=17, 29, 31 (every seed). The gain is small and not clearly nonzero at n=19 and absent at n=23. All 17,865 emitted relations passed both replays. CPU per relation falls only through yield. Toy scale.
- **6d. certicom131_symlp_20261025.** Symmetry breaking plus double large primes on ECC2-131 and ECCp-131 at m=2:
  - Binary symmetric step: 28.7× (ECC2-131, 0.18→5.24 M/s), 20.8× (n=127 calibration), 11.6× (n=61); IC/rho per step 1.20, 1.74, 1.04.
  - Prime: no symmetry saving (0.5–0.6×).
  - Direct relations are about 5×10^-7 of decompositions. The measured trial slope is 0.647 [0.639, 0.655]. The distance to rho at n=131 stays about 2^{n/6} ≈ 2^22 steps.
- **Lead next step.** Combine 2LP with the aimed m=3 recipe of the 7.03×/11.5× rows. Stage 0 of EXP-BINSTD-e066d8 has not run.
- **Sources.** ar:LEAD-ICB-large-prime-variants.yaml; cr:research/icb_large_prime_n53_n61_20261029/README.md (+validator-report.json, replay-certificate.json); cr:benchmarks/20261029-icb-large-prime/; cr:research/ic_koblitz_dlp_tuning_20261023/REPORT.md; cr:research/orbit_aware_large_prime_20261011/RESULTS.md; cr:research/certicom131_symlp_20261025/REPORT.typ.
- **Treatise.** ABSENT for binary/Koblitz. Large primes appear only in the genus-3/extension chapters, and "two large" once in prog_groebner. This is the most significant measured constant gain in the family after LEAD-19.

## 7. Cofactor projection, torsion anchors, cofactor quotients

- **7a. LEAD-19 projected base (Table B rows).** S(A')/S(B) = 1.29 (1.12, 1.52) at f2m15 (h=2), 1.58 (1.35, 1.82) at f2m21 (h=2), 3.36 (2.80, 4.02) at f2m27 (h=6). Twenty repeats, paired. B stays 6.1×, 11.5× and 14.4× above matched rho. Next step: the validation gate and m=3 rows.
- **7b. ic_cofactor_clear_ab_20261103.** Clearing the cofactor from the IC base on census V2 curves: G_0 = 6.180 [5.915, 6.456], G_0.1 = 6.692, G_1 = 9.087 [8.674, 9.520]. The h=1 negative control is 0.989. The verdict by frozen rule is **invalid_controls**: the positive control rho-plain/rho-neg lower limit was 1.149 < 1.2. The author declines to reinterpret.
- **7c. ic_m2_torsion_n41_20261101.** Torsion anchors at a=0, n=41 (cofactor 4): paired geometric mean of trials 0.98 [0.64, 1.43], no detectable difference. **Correction:** this contradicts `ic_m2_cycle_leads_20261030` (PR #3009), which stated anchors "cost 1.5x (n=23) to 16x (n=31)". The n=31 figure is real; the generalisation is not.
- **7d. Cofactor/2-torsion quotients (closed by proof).** On ECC2K-130 gcd(4, ℓ)=1. Sum-compatible filters carry 2 bits, so Wagner k-tree has no quotient. The corrected ι-stable classification empties the (subspace base, 2-torsion quotient) cell. The production 2-torsion census adds 0 relations.
- **Sources.** cr:research/lead19_projected_base/README.md; cr:research/ic_cofactor_clear_ab_20261103/README.md; cr:research/ic_m2_torsion_n41_20261101/REPORT.md; ar:LEAD-ICB-cofactor-projected-ladder-base.yaml; ar:LEAD-ICB-cofactor-torsion-quotients.yaml; ar:ledger/evidence/EV-BINSTD-a692eb.yaml.
- **Treatise.** PARTIAL. 7a is in prog_binary §"Leaderboard rows" and obstructions mentions cofactor. 7b, 7c (a correction) and the 7d Wagner/k-tree obstruction are ABSENT.

## 8. Free relations from pair-table self-collisions

- **Numbers.** The relations exist but are correlated. Homogeneous rank 49 from 588 rows at n61/K600 (8.2% of 599 needed), 746 at K1200, 1,197 at K1500, 6 at n83/K600. Rank grows about K^4, so a free rank stage needs K ≈ 1,500–2,050 (n61) or ≈ 2,600 (n83, 36 GB). Production is a constant 0.960× [0.946, 0.976] pooled (n=41, 53, 61), and 0.957× with the trace filter vacuous. In the rule-v4 `ic price` pipeline the harvest is correct (6,187 relations, 0 rejected) but a net loss, cold +5–9% at every size.
- **Verdict.** exhausted_at_scale.
- **Sources.** ar:LEAD-ICB-free-relations-self-collision.yaml; cr:RESEARCH_FREE_RELATIONS_CENSUS_20261012.md; cr:benchmarks/20261015-ic-improve2-02-freerel-production; cr:research/ic_tool_program/rule/v4/README.md.
- **Treatise.** PARTIAL. The 0.960 and the qualitative account are in prog_frontier ("Koblitz. Free relations…"). Missing: the K^4 rank law, the n83 K600 rank of 6, and the rule-v4 net loss (+5–9%).

## 9. Batch IC, Table C and restart portfolios

- **Numbers.** The multi-instance generic bound sqrt(L r/|Aut|) covers generic IC including set-up, and Kuhn–Struik batch rho attains it. A restart portfolio cannot beat pure rho (H-BINSTD-32867d supported). The four-target union column is a rare-event linearisation. Table C per target (32 targets) is 3.62× (n41) to 14.9× (n19) above batch rho, with the reference from session S20. All 9 rows failed the 10-24 gate. ic_rho_koblitz_20260923 (uncited) is the boundary-ledger §19 correction that made batch rho the reference for 32-target figures.
- **Sources.** ar:LEAD-ICB-multi-target-and-portfolio.yaml; cr:research/ic_descent_20260930 (Table C source, uncited); cr:research/ic_rho_koblitz_20260923 (uncited); ar:ledger/evidence/EV-BINSTD-2d34cb.yaml.
- **Treatise.** PARTIAL. Table C is in prog_koblitz §Table A. The portfolio theorem and the §19 multi-target correction are absent, and the source dirs are uncited.

## 10. Retained-set-up online ladder (n=53–83)

- **Numbers.** n61 13.94× [9.81, 19.13] admitted then retired. BL preprocessing rho is 53–317× faster online at equal advice. N83 precomputed rho: 3.74× time-matched and 40.6× memory-matched at K3300, 81.5–395.5× at K600. The cryptanalysis online/tuned rho 0.0933 is measured against a rho with no table.
- **Verdict.** closed_dominated.
- **Treatise.** COVERED: prog_koblitz §"The one-target online ladder" and §n83 "The control".

## 11. The N83 campaign

### 11a. 4–21 October (F6 micro-studies, a=1 point, root-index capacity, factor-base selection, tier-1 online, BL control, cold, cryptanalysis ratchet)
- **Treatise.** COVERED in prog_koblitz §"The n=83 campaign" (145 dirs as clusters). Individual dirs remain uncited, which is acceptable; an appendix table could list the cluster→dir map.

### 11b. N83 summand sweep m=2…9 (n83_msweep_20261024) — **correction, ABSENT**
- **Instrument correction.** Every Weil-descent engine "proved UNSAT" on planted n=83 relations because `FieldStructure` truncates the field table to one 64-bit word. This covers matrix F4, matrix F5, inherited F4, F6-IC, F6-IC pair index, crossbred, three SAT-descent encodings and MQ-FES. The builders now refuse such fields (regression test `n83_planted_relation_is_never_refuted_by_a_truncated_field`). Any earlier n=83 "refutation" from those engines is void.
- **Results.**
  - Chained-S3 full-width SAT is the only decomposer that runs at n=83. It decides only m=2: 17.3 s per public query, projecting to 2^75.9 s, which is 2^75.3× rho.
  - The m=4 rootfinder (complete DLP, K=1107) takes 188.5 s (root11) and 175.2 s (root22) against strong rho at 1.509 s, so it is 116–125× from rho.
  - Companion n=61, m=2: native-XOR SAT is fastest at D ≥ 12 (0.79–2.41 ms/query). Inherited F4 is the fastest Gröbner engine (6.2–33.4 ms), and F5 runs at 0.78–1.05× F4.
- **Source.** cr:research/n83_msweep_20261024/README.md.

### 11c. N83 degree/regularity sweeps (27–28 Oct) — ABSENT
- **n83_ic_degree_sweep_20261027.** All 96 Singular attempts (Koblitz, general binary, prime; m=2..9; 40 s / 3 GiB caps) hit time or memory; no degree completed. Standard first fall: Koblitz and binary have first fall 2 at every m (32 exact witnesses). For prime the values 24, 17, 14, 12, 10, 9, 9, 8 are proof-assisted lower bounds.
- **n83_fixed_degree_sweep_20261028 (fixed ℓ=27).** Of 96 attempts, 4 completed affine bases (m=2 only), 63 hit the time limit and 29 the memory limit.
- **n83_top_quotient_20261028.** Certified top-form bounds, e.g. m=2: 28 ≤ dreg ≤ 55; m=3: 82–165; … m=9: 244–825 (Koblitz and binary). Prime m ≥ 3 has a positive-dimensional top quotient.
- **n83_fixed_fall_catalog_20261028.** 48 first-fall certificates and 32 degree-two closures.
- **n83_candidate_pdp_20261028.** Radix census over all 3,010 partitions of 27: the minimum exact first vanishing degree is 42 (all-binary presentation).
- **Reading.** These are certified bounds, not attack costs. They show the n=83 descended systems are far outside solver reach.

### 11d. Chained-S3 SAT at N83 (31 Oct–1 Nov) — ABSENT
- **n83_chain_sat_order_20261031.**
  - At m=2, all-summand ordering halves conflicts (median public 13,461 → 7,195; 12.8 → 6.2 s).
  - At m=3, root-finder tables refute every public target with 0 conflicts.
  - m=5–9 decide nothing.
- **n83_chain_sat_meet_20261101.** A MITM lookup ahead of SAT refutes 3/3 public targets at m=5–8 with 0 conflicts. Lookup builds take 0.68 s (m=5, 6), 26–27 s (m=7, 8) and 958.8 s (m=9, capped). Planted targets are 0/2 at every m. Per-query cost is about |B|^{m−d}; stage diagnostic only.

### 11e. N83 genus-3 image transfer (31 Oct–1 Nov) — ABSENT
- **n83_genus3_image_solver_20261031.** Executed same-field degree-3 cover. Typed certificates prove φ_*φ^*=[3]; the pullback is injective on the r-subgroup (r = 2417851639230796216685689). Rho3/BSGS solve in the Jacobian image at toy n=9, 13, 19, 23, and all 400 destination attempts verified. No claim is made that the destination is weaker.
- **n83_jacobian_projector_20261031.** The Prym has Frobenius polynomial X^4+q^2. It splits into elliptic factors of traces ±2^42 and has order coprime to r, so it carries no shared factor (structural, Lean identities).
- **n83_image_closed_degree2_20261101.** Degree-2 fibres give only [3]P = P + [2]P, so there is no independent log information. The scoped rejection fails the information gate.

### 11f. N83 capacity engineering after 21 Oct — ABSENT (minor)
- K4405 one-root index at its 75% load boundary: 2^31 five-byte slots (10.74 GB), 22.36 GB peak RSS.
- K5500 relative-ID split index: 25.61 GiB.
- Four-byte u32 slots; fp8 sidecar ABBA on K1107.
- Local five-base screen (one point, cold medians): K992 66.2 s, K1024 62.7 s, K1107 65.8 s, K2304 119.7 s, K3300 383.6 s; rho 2.0 / 1.0 s; pair spreads 4.7–19%. No winner selected.
- K4405 full control stopped at 873 s before the matrix. K2304 control capped at 1,200 s.
- No new ratio admitted.
- **Sources (11):** cr:research/n83_*_2026102x–2026110x READMEs as named; cr:research/koblitz_n83_factor_base_sweep_20261008 (v4 paired-base grid, 26.9e9 declared addresses); ca:experiments/koblitz-n83-*.

## 12. ECC2K-130 routes

- **Covered (prog_koblitz §ECC2K-130).** Decomposition bottom 2^132.58; orbit unions m=3 2^124.99; relation sweeps (+4.98, +13.59 bits); linearisation (71.82–87.16); Weil descent (+7.62 contradicted, +10.32, +45.71, hybrid +8.43…+58.27); covers (magic number 1/131; genus-130 Jacobian heuristic 2^37.17); isogeny class (262 curves); rho (4.3 GPU card-years; 13.2 M steps/s).
- **Missing (ABSENT/PARTIAL):**
  - **ca:experiments/frobenius-quotient-m4.** Per-arity table with the orbit quotient credited in full. MITM gap: m=3 +65.0 bits, m=4 +34.4, m=5 +44.0, m=6 +25.3. At m=2, LA alone exceeds rho. ABSENT.
  - **ca:experiments/pdp-scaling.** Independent extrapolation of +35 to +51 bits at n=131. ABSENT.
  - **ca:experiments/pdp-degree-heuristics.** Over 656 bases (7 families, 9 fields), the m=2 closure solving degree is D = 2 + [e < −3.5] + [e < −8.5], with e = n − dim V − dim V^(2) (MAE 0.06). For prime n, dim V^(2) ≥ 2l − 1, and only geometric progressions attain it. ABSENT. This is a clean structural law for a theory chapter.
  - **ca:experiments/ecc2k130-isogeny-class/REPORT.md.** Four conductor levels (1, 263, p, 263p). Best known 2^60.81 for E0 and all 262 floor curves; the level-p curves (≈ 2^57.02 of them) lie in [60.81, 64.33] pending a char-2 Kani pipeline. PARTIAL (the treatise has 262 curves and the 11.45× penalty).
  - **koblitz_ic_new_leads_20261018 / koblitz_ic_leads_build_20261024 (L1–L4, C1).**
    - L1: λ-coordinate bases are affine x-subspace bases in disguise (182 subspaces).
    - L2: partial enlargement / MXL variable restriction. It helps only on m=4 (11,5): 0.38–0.45, 1.8–2.5× fewer products.
    - L2's incremental engine holds 1.3–2.8 GB against 1.9–4.2 GB at degree 4.
    - **C1 measured (Amendment 7, 2026-11-04):** m=3 (n=19, ℓ=7), draws 0 and 3 both refute at degree 5. Peak 52.6 / 52.5 GB, 31,632 / 31,888 s, closure fills 0.80 of the columns. The dense engine aborted at 424 GB. About $53 of an $80 cap.
    - This supports the degree-5 rows of RESEARCH_ECC2K130_DESCENT_DEGREE.md at ℓ=7 for m=3 (not m=4).
    - ABSENT. It is the first measured degree-5 cell, directly relevant to the +10.32-bit row.
  - **LEAD-ICB-ecc2k130-jacobian-carrier toy ladder (cr:research/ic_next10_20261014/10-torelli-toy).** n=5: no carrier through genus 8 (≈ 0.42 classes predicted). n=7: 1,536 genus-7 hyperelliptic carriers (mass 0.5), from Weil-restriction structure that n=131 lacks. Next rungs n=11, 13. ABSENT (the treatise has only the codimension heuristic).
  - **Koblitz modular / low-genus carrier rounds 20–26 (cr:research/koblitz_modular_ic_20261018).** Char-2 five-root obstruction proved. Round 22: Smith factors 1, kernel rank 24. Round 26: diamond norms exclude the invariant quotient. No IC rank yet. ABSENT.
  - **September isogeny-transport/PDP-prescreen dirs** (ecc2k130_direction_review, dual_transport, endo_ring_263, factor_base_replication, pdp_features, pdp_chain_holdout, leaf_native_m3_gate, rho_audit, ecc2k_cycle_escape). Degree-263 kernels: 264 lines, 262 descending j-invariants in two Frobenius orbits of 131. End(E) = Z[τ] = O_{−7} certified. The affine-contradiction prescreen rejected 88/640 natural cases and retained 111/111 hits, but is "negative for deployment". Three-sum exact coverage is 0.3647%. PARTIAL (the treatise summarises transport at 3.6–4.0×).
  - **ecc2k130_leads_survey_20261023** (report.pdf, leads.json; cost-vs-effect map). ABSENT.

## 13. The degree question: closure, hybrid guessing, invariants (binary Weil descent)

- **Covered (prog_binary §"The degree question, measured", to 24 Oct).** FFD 3, refutation degree tracking ℓ within 1 of semi-regular, MutantXL closure at degree 3–4, rise at (3,7), (4,6) refuting at 4, hybrid guess restoring degree 4 at (3,7), and +8.43 vs +58.27 bits "which only a run at ℓ=8 can separate".
- **ABSENT, with a direct bearing on that open sentence:**
  - **dreg_closure_guess_20261022.** m=3, (n,ℓ)=(22,8), N=46, 4 UNSAT draws. **Four guessed variables suffice on 4/4** (16/16 branches refute at degree 4).
  - **dreg_closure_guess3_20261023.** **Three guesses do not suffice** on any draw (closure rank about 129k of 136,698 columns).
  - Combined with 1–2 guesses at ℓ=7, the number of guesses grows with ℓ on this family. That points to the linear-growth (+58.27-bit) branch for the m=4 ECC2K-130 row, if the m=3 behaviour transfers (an inference, not a measurement). The treatise still says ℓ=8 has not run.
  - **dreg_closure_null22_7_20261028.** The unspecialised (22,7) system does not refute at degree 4 on 4/4 draws.
  - **dreg_closure_specialise_20261017.** One fixed variable usually restores degree 4 at ℓ=7. Uninformative for H_sum vs H_max.
  - **dreg_invariant_guess_20261029 (+ _more).** The invariant guess is never worse than the raw guess but does not reliably suffice at ℓ=8: 2/4 suffice against 1/4. A post-hoc soundness amendment is disclosed, and 5 runs were OOM-killed in 16 GB.
  - **dreg_invariant_pair_guess_20261029.** A second invariant guess is never better on 12 draws.
  - **dreg_pair_invariant_20261028.** The summand-swap invariant does not refute below degree 4 at ℓ=6; ℓ=7 is unresolved.
  - **dreg_second_link_guess_20261105** (pcc PCC-20261106-607061, unverified). L3 suffices on 6/12 against S3 on 7/12, no detected difference.
  - **dreg_high_arity_20261101.** Protocol v4 frozen, launch blocked, no data.
  - **dreg_closure_m4_20261018, dreg_closure_slope_20261018, closure_degree_predictor_20261018.** Reports or PDFs only.
  - **cheap_closures_20261007, structured_closure_20261024, sym_s4_closure_base_20261024, lever2_closure_20261031** (the last is a prime-field derivation; the exact-count falsifier is an identity).
- **crypto-autoresearcher companion (PARTIAL).**
  - m3-closure-pilot: W_4 refutes 60/60, 40/40, 8/8 UNSAT at n=9, 11, 15.
  - m45-closure-pilot: refuting degree exactly 4 at m=4, 5 with |V|=8.
  - block-local-closure-pilot: refutation is locality-1 (124/125), with 39–69% fewer columns.
  - leads-pilot4: windowed closure.
  - The **criterion** that relation collection beats rho only if a failed attempt costs 2^{αℓ} with α < m/2 − 1 is not stated in the treatise.
  - F4 refutes at degree 4 through (11,4) and (15,5) where XL reads 5 to ≥ 7 (48/48 paired). The (4,5), (4,6), (13,4), (13,5) cells are blocked by memory (msolve segfaults at 1.6–1.8 GB; the m=5, |V|=8, D=4 cell needs 32–128 GB). Modelled n=131 m=4 price: +7.6 bits before K, +9.78 at median K 4.48 [3.96, 16.77]. Model, not measurement.
- **Sources.** cr:research/dreg_*_2026102x–1105/RESULTS.md; ar:analysis/ecdlp2m-iteration-20261009/{m3,m45,block-local}-closure-pilot, leads-pilot4, closure-rescope-audit; ar:LEAD-ICB-prime-degree-algebraic-decomposition.yaml; cr:research/k_conversion_20261018/README.md; cr:research/ic_next10_20261014/06-equivariant-f4-m4/README.md.
- **Treatise.** PARTIAL. The 22 Oct–5 Nov guess series and the α < m/2 − 1 criterion are absent.

## 14. Decomposition encodings, SAT and ML-guided SAT

- **Covered (prog_binary §"SAT, SMT and WDSat are enumerators", §"Algebraic oracles against the floor").** CDCL at 0.97–1.02 conflicts per triple; WDSat on Trimoska; m=3 slopes 0.53–0.58 vs floor 0.207; m=4 wide chain 0.805 vs 0.333.
- **ABSENT:**
  - **pdp_ml_sat_ceiling_20261027.** Closed by its preregistered rule, against 4,200 exhaustive labels. Refutation costs 0.963 (n15l5) and 0.998 (n17l6) conflicts per sorted triple. A perfect UNSAT predictor would raise relations per conflict 5.2–9.0×. Test AUC is about 0.5 for every feature group (permuted control 0.45–0.54; positive control 0.72–0.79). Train-fitted gains of up to 4.7× fall to about 1× held out. "The ceiling is real, the signal is not."
  - **ar:analysis/ecdlp2m-iteration-20261009/ctrial-harvest.** 411 rows and 1,554 per-target values extrapolate 50–150 bits above budget at n=131.
  - **m4-sat-cost-pilot.** The gate reproduces 2^{3ℓ}/3! (α 2.981 [2.973, 2.997]). At m=4, α is 4.33 [4.32, 4.34] (Gauss only) and 4.09 with full XOR.
  - Encoding constants: S_m symmetrisation saturates at m ≥ 5 (EV-BINSTD-e9b8ff), closure outcomes are basis-invariant (Proposition B), Nagao coset yield is weakened, and trace replay is a constant at most.
  - **semaev_t2_trace_20261102.** REFUTED: Semaev 2015 §4.5's "first fall degree is always 4" for S_3(x1,x2,z). On all 200 t=2 instances the FFD is 2, with trace certificates. The XL refutation degree is 2 on 154 instances and 3 on 46, never 4. This supersedes the treatise's summer-chapter Semaev 2015 audit bullet.
  - **ecbench_sat_pdp_20261009.** Pilot of 28 records; 4 ic-buchberger timeouts stopped the main panel.
  - **ml_dataset_* (30–31 Oct).** Native paired-method datasets (binary n23, Koblitz n23/31/47, planted PDP). Infrastructure, no results read.
  - **pdp_strategy_evaluation_20261031** (desk). Of 28 candidate PDP strategies, 17 collide with atlas leads, 7 partly overlap and 4 are new (rated least promising). On the ladder rows the walk is 61–82% of S and the matrix ≤ 0.10%, so algebraic-oracle levers have ceiling 1.00×.
- **Sources.** ar:LEAD-ICB-decomposition-encoding-and-sat.yaml; ar:LEAD-ICP-icperf-binary-pdp-solver.yaml (EV-ICPERF-390707: five "unverified SAT answers" were quadratic-twist decompositions); cr:research/pdp_ml_sat_ceiling_20261027/README.md; cr:research/semaev_t2_trace_20261102/REPORT.md; cr:research/pdp_strategy_evaluation_20261031/README.md.
- **Treatise.** PARTIAL.

## 15. F6-IC / Gröbner oracle engineering on Koblitz (engine overlap)

- **f6_few_candidates_20261105.** A few-candidate closure in the F6-IC gate, now on by default: CPU off/on 2.67–10.29× (worst pairing ≥ 1.66×), word XORs 1.84–10.13×. The inherited-F4 control is 0.87–1.05×. Verdicts are identical. Key finding: "the algebra was not ruling out any candidate x-coordinate"; with the pair index, F6-IC reduces to the pair-index MITM scan. A parallel tree split gives 1.59–2.88× wall. Four-Russians batching costs more than it saves.
- **f6_ic_ecbench_ladder_20261005.** The decision was not reached (m=31 capped at 3,600 s).
- **f6_ic_rho_online_gap_20261006.** rho_online/IC_online is 0.0219–0.0221 (T1) and 0.000515–0.000530 (T7), so rho is faster.
- **f6_ic_* micro-dirs (3–6 Oct), f5_*, gf2_* elimination kernels (26–29 Sept).** Engine constants.
- **Treatise.** PARTIAL: the oracle sequence is in prog_koblitz and the engines in prog_groebner. The f6_few_candidates finding that F6-IC degenerates to MITM is ABSENT; it reinforces the "algebraic oracle = enumerator" conclusion.

## 16. Factor-base subspace choice and the learned cost model

- **16a. Named subspace properties (exhausted_at_scale).** Greedy coverage gives 1.18× at r=2^13, shrinking to 1.0000 at r=2^39. Coset/random 1.020 [0.959, 1.083]. Geometric progression vs random at matched dim(V·V) is weakened (EV-BINSTD-78ca0d, -b176df). Spearman tests are inconclusive. NB vs PB Hamming-weight pools give no yield change at n=17. The Weil bound is vacuous for l ≤ 66 at n=131. 208-cell hyperparameter sweep. COVERED (prog_frontier greedy row; prog_binary "levers that did not move the degree").
- **16b. Learned gated ridge cost model (ABSENT).**
  - Stage: 1.82 bits/relation [1.36, 2.26] better than the hand rule (l ≤ 4, 13 cells), and 1.13 [0.45, 1.62] at l=5.
  - Cross-curve without refit: +1.84 [0.95, 2.68] on a2=1 Koblitz. On ordinary curves +0.92 [0.54, 1.34], NOT CONFIRMED on 16 fresh curves at l=4 (+0.43 [0.15, 0.74] < 1.08).
  - End to end: 0.93 bits [0.36, 1.48] per verified DLP (9 cells, n ≤ 41). At l=4 on ordinary curves, n=19: −0.45 [−0.98, +0.04] (null, sign reversed).
  - **Native EKS confirmation 2026-11-05, independently validated 2026-11-06 with VALIDATION.json**, 160/160 replays on a second CPU, K1 a2=1 Koblitz, l=3. log2(S_rule/S_ridge) in counted complete-DLP cost = +1.83 / +0.80 / +0.80 bits at n=17 and +0.96 / +2.47 / +2.47 at n=19, at 0 / 0.1 / 1 GAE. Every CI lower end is above 0. CONFIRMED.
  - The ridge pick stays behind rho.signed_frobenius_strong by 1.8 / 14.8 / 17.7 bits (n=17) and 3.3 / 15.1 / 17.9 (n=19).
  - **Correction:** the producer's first "about 9 bits, identical across the band" was a pinned-calibration point and was corrected.
  - Next: K1 n=23 (frozen 2026-11-06).
  - This is the family's only validated, measured improvement over a binary IC baseline. It is small, at toy scale, and its scaling in n is not yet known.
  - Sources: ar:LEAD-ICB-learned-cost-model-factor-base-selection.yaml; external aburan28/ml-cryptanalysis experiments/fb-cost-model*; experiments/fb-cost-model-curves-eks, fb-cost-model-e2e-l4 (repo not local); ca:experiments/pdp-degree-heuristics (label producer).
- **Treatise.** 16a COVERED; 16b ABSENT.

## 17. Quasi-subfield polynomials (QSP) at prime degree

- **Numbers.** Euler–Petit Theorem 1: every linearised QSP has β ≥ 3/4, so the exponent is ≥ 0.93, i.e. ≥ 2^122 at n=131 against rho 2^60.9. For F_2-coefficient λ only X^2+X (n0=1) and L_Φ131 (n0=130) exist at n=131. Multiplicative QSPs give |V| = 264 or p+1, β ≥ 1.02. The census shows HKPYY Lemma 4.1 tight (witnesses at n=7). Toy n=31, k=15: QSP yield 0.214 against random 0.204–0.230.
- **Sources.** ar:LEAD-ICB-quasi-subfield-polynomials.yaml; ar:analysis/qsp-ecc2k130/README.md; cr:research/ic_next10_20261014/08-quasi-subfield/README.md; ar:ledger/evidence/EV-QSP-a6aa4b.yaml.
- **Treatise.** PARTIAL. prog_binary mentions the F_{2^7} census and Lemma 4.1 tightness only. The Euler–Petit β ≥ 3/4 ⇒ ≥ 2^122 bound and the toy yield null are missing.

## 18. τ-Horner decomposition chain

- **Claim.** An m!-fold distinct-relation yield (H-FROB-855915 approved, never run). The discriminating test, IDEA-20261018-f2cd4d, compares Horner against the symmetry-broken chain at n ∈ {17, 19, 23}; equality would mean relabelling.
- **Verdict.** open_next_step, no data.
- **Treatise.** ABSENT. Worth a paragraph as an accounting trap: the m! may be the symmetry-breaking m! relabelled.

## 19. KIC lane: co-designed bases with exact decomposition

- **Numbers.** Affine 2-flats, orbit-canonical lookup, SAT vs exact MITM. N7 pilot (EXP-KIC-424885): flat SAT / direct MITM median wall 1.70× [1.02, 2.69] (SAT slower); candidate coverage 70 equals null coverage 70. Five runs (EXP-KIC-3df18c, -424885, -7bcef8, -c3c732, -d9c828) have no EV record and are unreviewed.
- **Treatise.** ABSENT (minor).

## 20. Composite-degree binary curves: c2pnb (Gaudry–Diem n=4) and Oakley (n=5)

- **c2pnb C1 cost model (ar:analysis/ecdlp2m-iteration-20261009/c1-gaudry-n4-c2pnb).** Time-only, below rho on all five rows for c_trial ∈ [2^15.5, 2^32.5]. It loses on time×memory against vOW on every row. A mesh area-time win appears only at 304w1 and 368w1 for c_trial ≤ 2^27. The F_{2^16}-rational base is structurally empty in G. Naive trace-zero extrapolation is a regime-invalid trap (EV-BINSTD-91ea96). The C1 toy ladder has not run.
- **Lift-dense lines.** +0.50 bits modelled on c2pnb208w1/368w1. COVERED in obstructions.
- **Oakley n=5.** Three approved contracts (EXP-OAKLEY-2c0552, -4276c4, -7f73ef) have no runs. GHS genus-16 prices +1.1 bits (group 3) and +10.9 bits (group 4) of advantage after the 10-11 correction (formula). oakley_lift_dense_20261027 (uncited; no README read).
- **Sources.** ar:LEAD-ICB-composite-degree-c2pnb.yaml; ar:LEAD-ICB-oakley-quintic-tower.yaml; ar:LEAD-TRF-ghs-binary-composite.yaml; cr:research/cover_route_prime_binary_20261008/README.md.
- **Treatise.** PARTIAL. Oakley GHS reachability is in prog_transfer and the c2pnb lift-dense line in obstructions. The c2pnb three-metric cost model (the one family where IC is below rho on time alone) and the unrun Oakley direct decomposition are ABSENT.

## 21. ECC2-131 / Certicom 131

- **Covered (prog_binary §ECC2-131).** Program, gates, falsifiers; D = 4, 5, 6 at ℓ = 1, 2, 3 equal to semi-regular; rho at 8.50 / 11.46 M steps/s; CUDA compiles but has not run.
- **ABSENT.** certicom131_symlp_20261025 (see 6d: 28.7× symmetric step, IC/rho per step 1.20, about 2^22 distance at n=131). certicom_131_stage_bench_20261023 (S3 quadratic-root query rates, 5 samples) and the details of certicom131_ic_program_20261020 (the dir itself is uncited; its doc is cited).

## 22. Linear algebra of the orbit-folded matrix

- **Numbers.** LA is 0.0–2.7% of S. CADO-style filter 2.48–10.4× on the phase; 1.93× faster than CADO-NFS at 262,144 columns; sparse-cado 0.25–0.72× LA wall at identical S; 10.40× [9.99, 11.06] engine.
- **Treatise.** COVERED (model, prog_frontier).

## 23. Random-binary ladder (Table B)

- **Numbers.** Five rows, F_{2^15}–F_{2^27}: 13.1, 10.6, 18, 23, 38.2× matched rho. All passed the 10-24 gate.
- **Treatise.** COVERED (prog_binary §"Leaderboard rows"). Note the treatise's own remark that the ledger paper contradicts the gate result.

## 24. GHS / Weil-restriction structural work on small binary fields (shared with transfer)

- **Numbers.**
  - ghs_q16_simple_restriction_bound: for any correspondence, irreducible F_E ⇒ k-simple ⇒ target genus ≥ 3, over all 8,190 ordinary curves over F_4096.
  - ghs_q16_genus4_frontier: 66 curves missed by the genus-3 V4 screens all map to genus-4 hyperelliptic Jacobians.
  - ghs_nonsplit_order4222: an end-to-end GHS transfer to a smooth nonhyperelliptic genus-3 curve over F_16.
  - ghs_ipsec_isogeny_class: 24,492 Frobenius orbits, 0 low-genus candidates.
  - ghs_generalized_20261024: full nonzero-c Hess screens at degrees 6, 8, 10, 12.
  - ghs_cover_census_n4, ghs_counterfactual_n130: the N=130 counterfactual is covered.
  - weil_descent_angles_20261018: coverage is Poisson except binary n=2, where π → 3/8 against the Poisson 0.3935 (z = −4.72, proved).
  - weil_restriction_isogeny_census_20261018.
  - koblitz_isogeny_descent_37: degree-73 neighbour reduces F5 XORs ≥ 10% in 4/8 cells, worst regression 2.00×.
- **Treatise.** PARTIAL (prog_transfer covers to 24 Oct plus ghs_q16_genus3_subcovers). The binary n=2 coverage exception (3/8) is a neat exact result and is ABSENT. The rest probably belongs to the transfer agent's digest.

---

## Retractions and corrections in this family (flag list)

1. **n83_msweep (10-24):** every F4/F5/F6/crossbred/SAT-descent engine silently returned "proved UNSAT" at n=83 because of 64-bit field truncation. Earlier n=83 engine refutations are void. ABSENT from the treatise.
2. **ic_m2_torsion_n41 (11-01):** contradicts the PR #3009 generalisation that torsion anchors cost 1.5–16×.
3. **semaev_t2_trace (11-02):** refutes Semaev 2015 §4.5 (FFD 4 → 2).
4. **fb-cost-model:** the producer's "about 9 bits identical across band" was corrected to 1.8–17.9 bits behind rho.
5. **ic_cofactor_clear_ab (11-03):** the verdict is invalid_controls despite G_0 = 6.18. It must not be quoted as a gain.
6. **Hamming ball:** the earlier n=19 R=1.10 came from a single noisy denominator.
7. **EV-ICPERF-390707:** five "unverified SAT answers" were quadratic-twist decompositions.
8. **certbin-k2-collision-audit:** the archived k=2 collision "excess" (17 and 38 pairs against a birthday 5.00) is an instrument artifact; structural identities explain 16/17 and 33/38.
9. **Already in the treatise:** September reversals; n61 13.94× and N83 18.53× retired; Table A gap understated 1.7–2.0×; m=4 degree-4 floor (+7.62) contradicted; dense root 18.5× retired.

## Coverage table

| # | Line of work | Coverage | Est. pages |
|---|---|---|---|
| 1 | Frobenius-invariant / compact-orbit bases; orbit-fold closure theorem | PARTIAL (theorem, birthday pilot, 10-28 recheck missing) | 4 |
| 2 | Non-linear Frobenius-stable bases (trace-spectrum, Hamming ball, Couveignes–Lercier) | ABSENT | 4 |
| 3 | Generic table oracles, CGK obstruction, 0.165 slope | COVERED | 0.5 (cite dir) |
| 4 | Set-up phase, Table A rules, descent-vs-pipeline routing | PARTIAL (routing study absent) | 2 |
| 5 | Matched Koblitz rho reference (strong fixture, sqrt n volcano gain) | PARTIAL | 2 |
| 6 | Large primes on Koblitz/binary (lp2 0.56–0.60; DLP tuning null; orbit-aware LP; Certicom symLP) | ABSENT | 5 |
| 7 | Cofactor projection, cofactor clearing A/B, torsion anchors, Wagner obstruction | PARTIAL | 3 |
| 8 | Free relations (K^4 rank law, v4 net loss) | PARTIAL | 1.5 |
| 9 | Batch IC / Table C / portfolio theorem | PARTIAL | 1.5 |
| 10 | Retained-set-up online ladder | COVERED | 0 |
| 11a | N83 campaign 4–21 Oct | COVERED | 1 (appendix dir map) |
| 11b | N83 m-sweep and the 64-bit truncation correction | ABSENT | 2 |
| 11c | N83 degree/regularity certificates | ABSENT | 3 |
| 11d | N83 chained-S3 SAT ordering and MITM lookup | ABSENT | 2 |
| 11e | N83 genus-3 image transfer, Prym projector | ABSENT | 3 |
| 11f | N83 capacity engineering after 21 Oct | ABSENT | 1 |
| 12 | ECC2K-130: Frobenius-quotient m-table, pdp-scaling, linearisation-excess law, isogeny class, L1–L4 and C1 degree-5, Torelli toy, modular carrier, Sept transport | PARTIAL (many items absent) | 8 |
| 13 | Degree question after 22 Oct (ℓ=8 guesses, invariants, α < m/2 − 1 criterion, ar closure pilots) | PARTIAL | 5 |
| 14 | SAT/encodings: ML-SAT ceiling, ctrial harvest, m=4 α, Semaev 2015 t=2 refutation, PDP desk eval | PARTIAL | 4 |
| 15 | F6-IC few-candidate closure (F6-IC → MITM), F6 ladders | PARTIAL | 2 |
| 16a | Subspace choice (greedy, coset, GP) | COVERED | 0.5 |
| 16b | Learned cost-model base selection (validated EKS) | ABSENT | 3 |
| 17 | QSP (Euler–Petit bound, toy yield) | PARTIAL | 1.5 |
| 18 | τ-Horner chain (unrun; accounting trap) | ABSENT | 0.5 |
| 19 | KIC co-designed bases (unreviewed) | ABSENT | 0.5 |
| 20 | Composite degree: c2pnb Gaudry–Diem cost model, Oakley n=5 | PARTIAL | 2.5 |
| 21 | ECC2-131 / Certicom 131 (symLP, stage bench) | PARTIAL | 1.5 |
| 22 | Orbit linear algebra | COVERED | 0 |
| 23 | Random-binary Table B | COVERED | 0 |
| 24 | Small-field GHS / Weil-restriction structure (binary n=2 3/8 law) | PARTIAL (shared with transfer) | 2 |
| — | Corrections/retractions appendix for this family | PARTIAL | 1.5 |
| | **Total new material** | | **≈ 75 pages** |

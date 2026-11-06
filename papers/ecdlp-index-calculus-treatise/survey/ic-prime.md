# ic-prime coverage digest (prime-field index calculus, plus the atlas's F_{p^k} odd-characteristic leads)

Survey agent: ic-prime. Date 2026-11-06. Read-only.

Sources read:
- Lead atlas `crypto-autoresearcher:knowledge/leads/ecdlp/ic-prime/*.yaml` (34 leads, origin/main).
- `crypto:research/<dir>/README.md` heads for 183 uncited family dirs (origin/main).
- Targeted full reads for the residual, tensor, PKM, EcGFp5, small-root, SAT and k-sum lines.
- The treatise on `crypto` origin/main `papers/ecdlp-index-calculus-treatise/sections/*.tex`. I read prog_prime.tex in full, and grepped distance, falldegree, obstructions, prog_frontier, audit, summer and openproblems.

Conventions:
- I quote numbers only where a file states them.
- "UNQUALIFIED" means the atlas says there is no claims/*.json or pcc record.
- `[pcc:unverified]` marks producer statements with unverified standing.
- Most rows in this family carry no pcc claim id. Under the 2026-11-04 sabotage rule they must be cited in the treatise as unverified assertions unless a VALIDATION.json or a claims record is named.
- Curve25519 (§12.7 on branch claude/treatise-curve25519-20261106) is treated as COVERED and skipped.

---

## 1. Generic decomposition track: enumeration, pair-table MITM, k-sum tables, m=4 MITM

**What was tried.**
- Exhaustive, subtract and pair-table (folded, walked) decomposition at m=2..7 on generated prime-order E/F_p.
- Table B prime ladder.
- k-sum tables with k=4 and k=5 (28–30 Oct).
- A direct m=4 "2+2" MITM over a random base B=p^{1/5} (31 Oct).
- An arity sweep m=2..9 (27 Oct).
- The first prime m=4 cost cells (blindspots, 18 Oct).
- An audit of an external "alternatives to Semaev" memo (4 Nov).

**Key numbers.**

Exponents and Table B:
- Exhaustive IC exponent 0.99 (m=2) and 1.01 (m=3).
- MITM exponent 0.67, 0.75 and 0.60 at m=3, 4 and 5, against rho at 0.50 [0.48, 0.52].
- Table B gap widens from 5.47x (log2 r=11.9) to 13.5x (23.4). The atlas says the rows now render "validated with caveats (pricing_ok False)".
- FRONTIER folded mitm exponent 0.841 [0.646, 0.953].
- bitop/v1 ic-pair-rule 0.646 [0.629, 0.659] against rho-neg 0.504 [0.449, 0.575] at 33–40 bits.

k-sum tables (generic track):
- k=4 at 40 bits (#2808): S 11.56, 13.1x rho-neg.
- k=5 (#2852): S 8.51 at 40 bits, which is 0.74x of k=4, and 0.67x at 48 bits.
- k=5 fitted exponent over 36–48 bits: 0.547 [0.543, 0.554], against 0.565 [0.560, 0.571] for k=4. The generic optimum is k/(2k-1).
- k=5 is still 10.6x BSGS at 48 bits.
- Pooled study, 16 fresh curves (#2904, UNQUALIFIED): k=5 costs 0.70/0.45/0.32/0.22x a re-tuned mitm incumbent at 20/24/28/32 bits.
  - It is below the incumbent on 16 of 16 curves.
  - Toy-tier exponent 0.496 [0.487, 0.510], against 0.635 [0.627, 0.645] for the incumbent and 0.457 for rho-neg.
  - It is still 5.8–7.3x rho-neg and 6.5–7.6x bsgs-neg at w=0.
- Precomputation-regime control (prime_precompute_regime): rho.preprocessing against a known-log k-sum is accounting only.
  - Online is 142–229x cheaper and the build 333–339x dearer.
  - Crossover at 9.5k–14.3k targets.

m=4 and arity sweep:
- m=4 2+2 MITM: planted-solve exponent 0.412 (R^2 0.9986, 8 sizes), and 0.404 on a three-rung check. The uniform relation is predicted at p^{3/5}.
- Arity sweep at 12–18 bits: best m=2 is 10.6x to 60.5x rho; best m=3–5 is 2.8x to 5.8x (different units, as the tool notes).
- Blindspots m=4 cells (16–28 bits, VALIDATION.json): full IC is 8.2x–29.5x negation rho at w=0.1, and 6.5–35.6x across the band. Verdict REGRESSES; own F4 grows as |F|^11.03.

Semaev-alternatives audit:
- The memo's gate "e < 1 at m=4" is wrong. A generic 2+2 MITM passes it (s=1.93, e=0.4) while costing p^{2/3}.
- The correct gate is s < m-3, i.e. e < (m-3)/4.
- Seven Lean theorems (Routes.lean, Roots.lean).

**Obstruction.** Any decomposition oracle built from enumeration or table lookup is generic: Shoup T >~ sqrt N, and Corrigan-Gibbs–Kogan S·T² >~ N (atlas LEAD-ICP-generic-table-decomposition, closed_obstructed).

**Verdict.**
- Negative against rho; closed by theorem.
- The k=5 table is a measured constant/exponent improvement *within the generic track* only (owner track split, DECISIONS.md R1).

**Sources.**
- crypto: research/prime_ksum_table_20261028, prime_ksum_k5_20261028, prime_ksum_pooled_20261029, prime_ksum_vs_incumbent_20261029, prime_precompute_regime_20261028.
- crypto: benchmarks/20261029-prime-ksum-*, research/ic_m4_mitm_scaling_20261031, ic_arity_sweep_20261027.
- crypto: benchmarks/20261018-blindspots-prime-m4-cells, research/semaev_alternatives_audit_20261104, docs/bounds/FRONTIER.md.
- AR: LEAD-ICP-generic-table-decomposition.

**Treatise coverage: PARTIAL.**
- Covered: Table B (prog_prime §1; distance §Prime curves), k=4 k-sum (distance Table), and S·T² (obstructions intro).
- Missing:
  - k=5, the pooled 16-curve test, the vs-incumbent result and the precompute regime;
  - the m=4 2+2 MITM exponent 0.412;
  - the blindspots m=4 cells;
  - the arity sweep;
  - the semaev-alternatives gate correction s<m-3 with its Lean theorems;
  - the FRONTIER/bitop exponents.

---

## 2. Constant engineering of the prime pair-table pipeline (generic track)

**What was tried.**
- Factor-base size tuning at 20 bits (N 32→45), with a second-host replay.
- N 64 against N 45.
- Wide target batching (16/64/256/1024 lanes).
- A blocked pair-table build.
- A packed-table one-bit membership filter.
- A cache-line layout tournament.
- A fixed-window aP+bQ generator.
- Planner weight rules.
- Stride-renew targets.
- A phase profile.
- O(√p) prime-order curve search.

**Key numbers.**

Base size and replay:
- N 32→45 at 20 bits: holdout sel/inc 0.818 [0.773, 0.866] at w=0, 0.798 at w=0.1, 0.726 at w=1 (96 pairs).
- Second-host isolab-eks replay: 1,008/1,008 records identical, 0.836 [0.806, 0.866] (240 pairs).
- Gap to rho-neg moved from 9.47x to 7.75x [6.92, 8.70].
- At 24 and 28 bits the same rule is relabelling; at 32 bits it is null.
- N 64 against N 45: 1.001 [0.979, 1.022] at w=0, unresolved; N 45 kept.

Other engineering items:
- Wide batching closed: 64/256/1024 lanes do not beat 16, with the knee at 16–48.
- Fixed-window generator: 1.161 [1.127, 1.198] over 54 toy solves (EV-ICPERF-92c130).
- Planner weight rules W1–W3 withdrawn out of sample.

Packed filter (validated in crypto TASK-20261029-654f73; UNQUALIFIED in AR):
- 32 bits: whole-solve wall per charged GAE 0.9601 [0.9569, 0.9633], i.e. −3.99%.
- S'' 1.0 / 1.0505 / 1.3586 at w = 0 / 0.1 / 1, so no counted improvement.
- 24 bits: regression 1.0032 [1.0006, 1.0058]. 28 bits: undecidable.
- Distance to rho at 32 bits: 31.47–31.63x.

Profile and other measurements:
- Hash-table floor: about 137 ns of a ~318 ns trial (derived).
- Line-layout tournament: blocked at the EKS L2 gate (v1 exit 42, v2 qualification failed, v3 frozen and undispatched).
- Phase profile at 20 bits, N 45: relations 71.8% of priced GAE and oracle setup 27.8%; S_ic 9.93 against S_rho 1.08.
- Blocked pair-table build: S unchanged by construction.
- Stride-renew (PR #2826, VAL-20261029-4aed57): srenew/walk 0.842 at w=0. I could not confirm it merged on main; only the deferred-guard PR #2835 is visible.

**Verdict.** Generic-track constant improvements; one validated (N 45). Nothing approaches rho.

**Sources.**
- crypto: research/prime_base_oracle_tuning_20261029 (+second-host), p20_large_n_20261029, ic_wide_batching_20261028, ic_wide_ext_20261028, ic_oracle_setup_blocked_20261020.
- crypto: research/ic_packed_filter_20261029 (+benchmarks/20261029-ic-packed-filter VALIDATION.json), ic_line_layout_tournament_20261031, ic_planner_exhaustive_20261026, ic_profile_20261029, prime_order_search_20261017.
- crypto: docs/ic/LEADERBOARD.md (component speed-up section).
- AR: LEAD-ICP-prime-ic-constant-engineering.

**Treatise coverage: PARTIAL.**
- Covered: N 45 and the second host (distance §Prime), planner validation (prog_prime).
- Missing: p20 N64, the packed filter (on the leaderboard as a component speed-up), the line-layout blockage, wide batching, the generator, the weight-rule withdrawal, stride-renew, the phase profile and the hash-floor derivation.

---

## 3. Residual / large-prime collectors (the prime arm closest to rho)

**What was tried.**
- Crediting the m=2 residuals x(R±P_v) that miss F as partial relations (H-PFDR-cb9606).
- Then a sequence of collectors: residual sweep, residual collector (|D|=r limit), fused residual step, DP and equivariant variants, chained restarts, batch against multitarget rho.
- A byte-counter pricing audit.

**Key numbers.**

Residual sweep (crypto PRs #2783/#2807, not on main):
- 39 sessions at 20–44 bits; 7,716 cross-binary replays identical.
- Slope of log2 S at fixed base: −0.025 [−0.034, −0.017] (B=2), equal to the generic searches.
- Collision rows match the birthday count, with observed/predicted 0.7–1.2.
- S at 36 bits: 2.28 (B=2) to 11.89 (B=128), against 42.2 for the tuned incumbent and 1.29 for rho-neg.

On-main studies, 26 Oct:
- ic_prime_residual_lp: res4/plan 0.119–0.426 at w=0 but 1.33–3.89 at w=1; slope 0.470 against rho at 0.480.
- ic_planner_exhaustive: res2/res4 below the plan at every weight at 36.0/36.25 bits (res4/plan 0.888, 0.865 at w=1). The improvement is generic (KeyedInvariance.lean).
- The leaderboard improvement log row is "Residual collector res2 vs rho.negation, fp36: 2.59x".

Fused, DP and equivariant collectors:
- Fused step (31 Oct): gae/step falls from 2.005 to 1.004 and fused/baseline is 0.56–0.92 at w=0, but the result reverses at w=1, so C1 is not met.
- DP/eq (1 Nov), best arms:
  - res1eq 1.54 / 1.75 / 2.66x rho (p36);
  - res1dp 2.13 / 2.14 / 2.22.
- Scale (p36, p40, p42), res1eq distance to rho at w=0/1: 1.32/2.27 → 1.72/2.99 → 1.39/2.42.
  - No growth with r.
- Correction after the validator (ic_byte_counter_pricing; owner option B, bytes no longer priced): the large w>0 gains were artefacts. C1 is met in 2 of 8 cells, not 6 of 8, and the 9-of-12 versus 8-of-12 tally is unreconciled.

Chained restarts, batch and multitarget:
- res1eq chained restarts: null, 0.994 [0.953, 1.037].
- ic.residual_batch against rho.multitarget_pooled at p36: L=64 gives 1.0077 [0.9811, 1.0350], so parity holds. L=16 fails parity.
- Multitarget exploratory: per-target cost 4.4x below T=1 at T=64. That is multi-target rho reproduced.

**Verdict.**
- A generic collision search with rho's exponent. Measured constant improvements over the pair-table incumbent: res2 is 2.59x rho on the leaderboard log, and res1eq 1.3–1.7x rho at w=0.
- Not an index calculus with a better exponent.
- Retraction: the w>0 gains were withdrawn as a byte-pricing artefact.

**Sources.**
- crypto: research/ic_prime_residual_lp_20261026, ic_planner_exhaustive_20261026, ic_residual_fused_20261031, ic_residual_dp_20261101 (superseded at w>0), ic_residual_dp_eq_20261101, ic_residual_dp_eq_isolab_20261101.
- crypto: research/ic_residual_scale_20261101, ic_res1eq_chain_20261101, ic_residual_batch_ecbench_20261102, ic_residual_batch_isolab_20261102, ic_byte_counter_pricing_20261101, ic_largeprime_multitarget_20261102.
- crypto: benchmarks/20261101-ic-residual-dp-eq (validator-report.json).
- AR: LEAD-ICP-residual-partial-relations.

**Treatise coverage: ABSENT.**
- No mention of res2, res1eq, residual collectors or the byte-counter correction.
- Highest-value absent generic-track item: the smallest measured prime distance to rho, and a pricing correction.

---

## 4. Cofactor-cleared (projected) prime factor base

**What was tried.** A/B test of `prime-projected-abscissa` with clearing (clear=1) against the raw base, on census V2 prime curves with h=32 (3 Nov, preregistered).

**Key numbers.**
- G_0 = 6.180 [5.915, 6.456], G_0.1 = 6.692, G_1 = 9.087.
- The h=1 negative control gives 0.989.
- The positive control rho-plain/rho-neg reaches 1.227 [1.149, 1.311], below its 1.2 lower-limit line.
- The frozen verdict is therefore `invalid_controls`.
- Distance to rho: ic-clear 9.80x [9.33, 10.30]; raw 60.6x.

**Verdict.** Descriptively a large gain over the census raw baseline, formally void because a control failed. The same lever on binary curves gave 1.29–3.36x (LEAD-19).

**Source.** crypto: research/ic_cofactor_clear_ab_20261103.

**Treatise coverage: ABSENT.** Distance §binary mentions the binary cofactor-projected base only.

---

## 5. PKM Kummer tower and algebraic prime track: the follow-ups

**What was tried, beyond the covered pilot, rounds 2–7, N=21, ecbench and isogeny-tower rows.**
- A symmetric PKM tower (Kummer membership combined in e_k before expansion).
- Lead 1 at m=3, N=18 and first m=4 symmetric cells.
- Kummer at P-224's 2-adic shape (Solinas primes, 20–61 bits, and P-224 itself).
- Radix gradings (EXP-PKM-RADIX-001).
- The t=14 fall-degree cell.
- An isogeny-kernel base against μ_k (m=2).
- P-256 44-step 2-isogeny chain encodings.
- F4/F5/F6 and root-finder rounds in ecbench.

**Key numbers.**

Symmetric tower:
- D one lower and matrices about 10x narrower at m=3.
- D at m=3 over N=6..18 reads 4,5,5,6,6, so the staircase persists; N=21 is the discriminating cell and needs more than 13 GB.
- m=4 symmetric at N=12 runs in 8–9 ms against 66–70 s for the plain chain, with 5.9x narrower matrices.

P-224 shape:
- In 89 tower cells D equals the recorded D, and the null matches in all 65 paired cells.
- m=2, N=24 was OOM-killed at 6.0 GB; a capped rerun gives D ≥ 6. Verdict: inconclusive on the solver axis, clean negative on price.

Radix gradings:
- `gain_constant` at m=2 radix 8 (r_k 0.173) and m=3 radix 4/8 (0.344/0.234).
- `no_material_effect` at m=2 radix 4 (0.538). `[pcc:unverified]`

t=14 cell:
- At cap 5 neither draw refutes, so d_F ≥ 6. The cap-6 cells are owed (a 112 GiB job).

Weak-assumption reanalysis:
- PKM m=4 random-target rows at N=8, 12, 16 have D = 6, 7, 7, but columns grow 1,049 → 9,452 → 76,837 and multiply-adds 4.84e7 → 8.55e10 → 2.70e13.
- So degree alone is not a cost certificate.

P-256 chain encodings:
- The chain encoding is 2x–654x slower than the dense encoding under Singular, and 8x to >79x under msolve (EXP-ECDLP-f49f75, -502ae2).

**Verdict.**
- Open_blocked. Every family's D grows with N, and the null matches.
- The constant gains (symmetric, radix) are stage diagnostics with no S.

**Sources.**
- crypto: research/pkm_symmetric_tower_20261026, pkm_symmetric_lead1_20261026 (+benchmarks/PKM_SYMMETRIC_UNADMITTED_20261026.md), pkm_p224_kummer_shape_20261022 (+benchmarks/20261022-pkm-p224-kummer-shape).
- crypto: research/pkm_radix_grading_20261104, prime_fall_degree_t14_20261101, isogeny_vs_mu_factor_base_20261102, pkm_tower_ecbench_20261011, pkm_tower_rootfind_20261018, pkm_tower_round4/5/6, pkm_kummer_wide_field_depth_20261018 (no README).
- crypto: research/ic_weak_assumptions_20261102.
- AR: LEAD-ICP-pkm-tower-algebraic-oracle; problems/pkm-isogeny-chain-factor-base-no-fast-decomposition.md.

**Treatise coverage: PARTIAL.**
- Covered: prog_prime §Kummer tower, falldegree §Prime-field Kummer towers (Theorem pkm), openproblems item 14.
- Missing: the symmetric tower, P-224 shape, radix gradings, t=14 d_F≥6, the matrix-growth counterexample, the chain-encoding slowdown, and the isogeny-kernel against μ_k comparison.

---

## 6. Amadori–Pintore–Sala partitioned summation polynomials

**What was tried.**
- 26 Oct–5 Nov. A native APS single-relation baseline and five extensions.
- Four solver and encoding leads (early stop, etc.).
- Independent transcript replay.
- An `ic.aps_v1` measurement instrument with a monotonic cold timeline.
- An export/audit tool, and four new hypotheses.

**Key numbers.**
- Bounded correctness only on tiny public fixtures (3 fixtures × 12 arms).
- Release suite 5,002 passed; 34 APS library tests; seven Lean lemmas.
- No benchmark launched. The paper's 11–22-bit ladder was not reproduced.
- The F4 counter omits base construction, decoding, sign search and verification (problems/aps-single-relation-complete-cost.md).

**Verdict.** Open, with no measurement. `[pcc:unverified]` (PCC-20261105-aca921, -ac1e93).

**Sources.** crypto: research/aps_extensions_20261026, aps_followup_20261028, aps_replay_20261029, aps_independent_replay_20261029, aps_measurement_20261029, aps_export_20261104, aps_leads_20261105.

**Treatise coverage: ABSENT.** APS appears only as the citation aps2018 in theory and rootfinding.

---

## 7. Digit (base-B) presentation as an F_p Weil-descent analogue

**What was tried.** Writing unknowns in s bounded digits, then measuring the first and last fall degree, staircase, syzygies and p-independence (Sept–Oct). Native ladder with independent primes (29 Oct, crypto #2899).

**Key numbers.**
- d_ff = m·2^{m−1} + ⌊(s − 2^{m−1})/2⌋ + 1, curve-independent and reproduced by a curve-free null (EV-PFDR-1394a4).
- d_last = ffd = 4 + ⌊s/2⌋ on 106/106 draws (m=2, s=2..8); ffd = 13 on 8/8 m=3 instances. The plain presentation has d_last = p+2.
- The Macaulay space spans 60–100% of 2^n monomials.
- deg S_{m+1} = m·2^{m−1} voids the bounded-LFD claim (EV-PFDR-f71d7f).
- KR-IC-28c357: d_reg depends on the presentation, and a lower d_reg is a regrading.
- Degree-five LFD frontier n=47–51 needs ≥1267–2820 GiB.

**Verdict.** Exhausted at the measured scale (m=2, s≤8; m=3, s=4,5). The m=3 CDCL(T)/F_p screen is separate (§8).

**Sources.**
- crypto: research/prime_digit_presentation_20261029, prime_fall_degree_20261027/DERIVED.md, p3_second_digit_carry_20261101.
- AR: LEAD-ICP-digit-presentation-solving-degree; EV-PFDR-1394a4/99c699/e67f06/acc71a/f71d7f.

**Treatise coverage: PARTIAL.** falldegree:632 and summer:165/201 mention the digit presentation in passing. The closed-form d_ff, the d_last ladder and the null are missing.

---

## 8. SAT / SMT / CDCL(T) solving of prime-field PDPs

**What was tried.**
- Three SMT formulations (covered).
- The 31 Oct lead review and the arity/solver-model correction (DEC-20261031-b51282, DEC-20261101-26f1fa).
- Twelve "ETH leads" L01–L12 with skeptic review.
- A shared-coefficient SAT discriminator.
- A hardened, unrun m=3 CDCL(T)/F_p screen with strict gates e<0.5 and a<1.5 (cryptanalysis PR #1347).

**Key numbers (ETH-leads synthesis).**
- L01: char-2 gadget slope about 2.58 bits of conflicts per window bit, against a bar of 0.5.
- L04: backdoor offset 2·log2(cap)−1.
- L05: reference slope 2.17 [1.93, 2.38].
- L03/L07: hashed bases 1.000 ± 0.0003 against a size-matched null; shared negation-closed x-window 0.713/0.833/0.916 at m=4.
- L09: no encoding is D.1-admitted (margin −16.1..−16.3 bits; ≥37x the enumeration exponent).
- L11: pc/school 0.78 [0.67, 0.92] at b=9 only.
- L12: clause retention 0.025 [0.014, 0.040] but setup uncharged; refuted.
- F17 SAT correctness panel: 36/36 model-set comparisons and 42/42 UNSAT proofs checked.

**Verdict.**
- The SAT lane is "closed for exponent purposes" (synthesis §1).
- An untested CaDiCaL ExternalPropagator with F_p Gaussian elimination remains open.

**Sources.**
- crypto: research/ic_prime_eth_leads_20261031 (SYNTHESIS.md, leads/L01–L12), prime_ic_lead_review_20261031 (+PDP_ARITY_CORRECTION.md), prime_shared_coefficient_sat_20261031, sat_smt_prime_ic_design_20261030 (no README).
- cryptanalysis: experiments/prime-field-m3-cdclt-20261031.

**Treatise coverage: ABSENT**, apart from the 20-bit SMT UNKNOWN result in prog_prime.

---

## 9. Divisor-norm / shared-coefficient decomposition and the "one-shot" direct IC

**What was tried.**
- A five-point relation as a norm A²−B²f=(X−r)U from L(6O), with membership through quotient remainders or Graeffe coefficient towers.
- A complete direct IC on E/F17 (29–30 Oct).

**Key numbers.**
- All 90 model-set cases agree on F17 (42 SAT, 48 UNSAT, 400 models).
- Mixed-target IC recovers all 18 nonzero targets. The panel retains 230 attempts with 217 PDP calls charged.
- The cost is exponential: each PDP call visits p(p−1) coefficient pairs, Θ(2^{2n}).
- Derived coverage bound for the Kummer-coset family: ρ_g ≥ 1/(192 m 16^m), under stated conditions.
- The oracle contract needs 2^{4√n+O(log n)} calls.
- Arity-two gates a<m/2 and e<m/2−1 (impossible at m=2).

**Verdict.** Open_next_step. No exponent was measured; this is a written proof, not formal.

**Sources.**
- crypto: research/prime_one_shot_20261029 (+continuation/), problems/prime-divisor-uniform-solver.md.
- AR: LEAD-ICP-shared-coefficient-membership.

**Treatise coverage: ABSENT.**

---

## 10. Hashed-factor-base Boolean tensor contraction (an external claimed sub-exponential prime-field ECDLP)

**What was tried, 29 Oct–5 Nov.**
- Triage and proof reading of the preprint *A Subexponential Time Algorithm for ECDLP* (Akshima et al., v0.2, bochum-crypto commit 1ee44e56). It claims 2^{O(b/(log b)^{1/4})}, conditional on Theorem D.1 of *The Exponential Time Hypothesis is False* (openai/math commit 7ef5c0cc).
- A Rust port of the Go `ecdltensor` prototype (19 tests, byte-identical output).
- An `ic.tensor_contraction_v1` speed study (instrument only, not run).
- The hashed_ecdl series: ECCp-131 hash prime, primality fast paths, counts protocol v2, finite-size, entry cost.
- Packet variants, common kernel, jet completion, pole anchor, companion operators and Schur certificates.
- Boundary-rank bounds in Lean.
- A D.4 admission audit.
- The prime-IC elimination campaign, which ranks this lead #1.

**Key numbers.**
- Two in-session readings of the decisive contraction proof "found no concrete gap". This is a scoped reading, not certification.
- Native GMP reconstruction: 72 cases, 308 Cauchy charts.
- The tensor crate passes 34 release tests.
- Elimination campaign:
  - "native finite fallback is rejected; symbolic guards first pass at λ=2^480";
  - semantic cuts show 154–354 raw fanout edges but only 7–15 exact signal classes.
- L09 census: no encoding D.1-admitted; needs λ ≥ 2^85.
- L06 contraction width: MMD+ 15..51 against enumeration 2..10.7 at b≤16.
- No ecbench count or wall result exists. hashed_ecdl_counts is protocol-only, and the speed study is unrun.

**Verdict.**
- Open. It is the program's best-supported *claimed* sub-exponential route, but it is external prior art, finite-size infeasible, and not measured.
- Most records are `[pcc:unverified]` (e.g. PCC-20261104-1df881, PCC-20261104-458dda).

**Sources.**
- crypto: research/prime_one_shot_20261029/continuation/tensor-review, ecdl_tensor_ic_rust_port_20261029, ecdl_tensor_ic_speed_20261030.
- crypto: research/hashed_ecdl_{speed_20261029, cpu_20261030, hash_prime_20261030, primality_fast_paths_20261030, finite_size_20261030, leads_20261030, counts_20261031, entry_cost_20261104}.
- crypto: research/prime_tensor_variants_20261030, prime_common_kernel_20261030, prime_jet_completion_20261030, prime_pole_anchor_20261030, primefield-companion-schur-20261101, primefield-companion-followup-20261102.
- crypto: research/tensor_boolean_breakthrough_20261031, ic_weak_assumptions_20261102 (FOLLOWUP.md), prime_ic_elimination_20261101, prime_ic_zero_filter_20261030, structured_ic_integration_20261104, structured_ic_postmerge_20261105, ecdlp_subexp_desk_20261101, hexagon_cscc_20261030.

**Treatise coverage: ABSENT.** There is no "tensor", "Exponential Time Hypothesis" or "hashed" anywhere in the treatise. This is the largest absent item.

---

## 11. Split-graph / secant and Boolean-offset presentations

**What was tried.** Prime bases from rational intersections of E with a low-degree ordinate graph, with two-point decomposition via secant variables (31 Oct–1 Nov).

**Key numbers.**
- Homogeneous degree d+1 against 2d+1 for the same-base dense presentation.
- Stock F4 handled degree 3 against 6 on 18/18 F17 targets, and d+1 against 2d or 2d+2 on F83/F101/F127 at d=3..5.
- 14,194/14,220 systems verified, with 26 F127/d5 cases capped. Offline, all 14,220 root sets proved.
- Boolean-offset successor: degree 7 against 9.
- Selector recodings: XOR 7→6, one-hot 7→5.
- No complete cost. The candidate used more reduced rows than its own dense presentation.

**Verdict.** Open_next_step: a degree theorem with no cost advantage.

**Sources.**
- crypto: research/prime_factor_base_degree_20261031 (SECANT_GRAPH.md, parallel_20261031, pencil_search_20261101, four_tracks_20261101).
- AR: LEAD-ICP-split-graph-secant-presentation.

**Treatise coverage: ABSENT.**

---

## 12. Structured and algebraic-membership bases, and the conservation law in measurement

**What was tried.**
- Subgroup, coset, Dickson, cube, quadratic-character, rational-map and learned bases.
- A pair-sum coincidence (κ) census and a break-even growth ladder.
- Fourier flatness of abscissa predicates.
- A four-family controlled comparison (coset, low-x, low digit weight, random; 48 curves, 16–24 bits).
- Hashed sparse-parity bases.
- The factor-base design register with Lean proofs.

**Key numbers.**
- Exact mean-yield ratio 1.000000 for six geometries at N 2^14..2^18 (EV-FBG-001).
- ml-cryptanalysis: 13 families at N 2^24..2^26 sit within 0.0003 bits of random (external, local).
- Coincidence ladder (crypto #2897), 16–32 bits: `SUB_BREAK_EVEN_AT_SCALE`. Slopes d ln κ/d ln N lie within [−0.006, +0.008] and every κ within [0.92, 1.08].
- EXP-PFDR-bb0d26: max |κ−1| = 0.041, and the planted κ ≥ 1.29 was recovered at z ≥ 21.9.
- Four-family comparison at 24 bits, rows per CPU second: low-x 1,352, coset 1,167, random 1,144, low digit weight 859. Coset/random 1.020 [0.959, 1.083].
- msolve on subgroup bases grows as deg^{4.4} at m=3.

**Verdict.** Closed in measurement (constant only), consistent with KN-FIND-007. The κ ladder discharges the step.

**Sources.**
- crypto: research/prime_coincidence_harvest_20261029, factor_base_evaluation_20261015, factor_base_designs_20261015, factor_base_traces_20261016, factor_base_excess_20261026, prime_fourier_flatness_exp7_20261007.
- AR: LEAD-ICP-algebraic-membership-factor-base, LEAD-ICP-coincidence-harvest-structured-base, KN-FIND-c81e5b.

**Treatise coverage: PARTIAL.**
- Covered: the conservation lemma (cover chapter) and EV-PFDR-f6b76c (prog_prime).
- Missing: the κ ladder, EV-FBG-001, the four-family table and the ml-cryptanalysis 0.0003-bit result.

---

## 13. Small-root (Coppersmith / Jochemsz–May) decomposition

**What was tried.** Reach computations for windowed S_3 blocks, a strategy-independent bound, unequal bounds (Theorem K), e-form asymptotes, a joint (4,2) lattice, and S_4/S_5 oracles (26–29 Oct).

**Key numbers.**
- Equal bounds: reach ≤ (n! vol Newt)^{−1/n}, giving 0.3293 (3 unknowns) and 0.3536 (2 unknowns). Attained in 11/11 cases.
- Unequal bounds are capped at 1/min W_i = 1/2 for S_3 (Theorem K).
- e-form asymptotes 1/6, 1/24, 1/80 (m=2,3,4) are optimal among convex (p,f)^t lattices.
  - Exception at z=0: a counterexample reaches 7/41 = 0.1707 > 1/6.
- S_4 and S_5 are capped at δ 1/21.8 and 1/70.8 against targets 1/4 and 1/5.
- IC demand at (m,w)=(4,2) is 8/9. The margin is −0.56 (proved) and at best −0.39.
- Correction CORR-20261024-b2a799: the tool compared the wrong exponent; JM extended reach 0.8408 → 0.2803.
- A Lean counting lemma.

**Verdict.**
- Closed for one block and convex sets.
- Open for multi-target ideals, non-convex sets and joint lattices.
- Includes a correction.

**Sources.**
- crypto: research/smallroot_optimal_reach_20261026, smallroot_followups_20261026, prime_eform_reach_20261029, prime_s5_small_root_leads_20261028.
- AR: analysis/smallroot-reach/CLOSURE.md and CORRECTION.md, LEAD-ICP-coppersmith-smallroot-semaev.

**Treatise coverage: PARTIAL.**
- Covered: lattices § (lit2026_w10 δ*=1/80, coppersmith_s3_theta2 β=1/4).
- Missing: the strategy-independent theorem, Theorem K, the e-form optimality with its z=0 exception, the S_4/S_5 caps and the JM-reach correction.

---

## 14. ECCp-131 / Certicom prime 131

**What was tried, beyond the covered BSGS/IC wiring, zero-relation probe and rho rates.**
- A full-width auxiliary 2-isogeny tower on the exact ECCp-131 field.
- A structured multiplicative-coset relation screen.
- A representation-filter (Wagner) search.
- The Certicom-131 IC programme gates (D≤3 at m≥4 is covered).
- A CUDA port of the walks (never run on a GPU).
- Hash-prime setup for the tensor route.

**Key numbers.**
- Auxiliary tower: 256 coset abscissae, 115 lifted to the published curve, zero natural rows, rank zero.
- Coset screen: prime cofactor r in p−1=2·5·r; zero natural relations missed by small-x.
- Representation filter: 3,450 configurations in each of 6 cells; verdict `no_qualifying_filter_in_frozen_family`. The two apparent winners needed Θ(B²) enumeration.

**Verdict.** Null.

**Sources.**
- crypto: research/eccp131_full_width_20261023, eccp131_structured_ic_20261022, eccp131_representation_filter_20261102 (+benchmarks/20261102-eccp131-representation-filter), certicom131_ic_program_20261020, certicom131_cuda_port_20261023, hashed_ecdl_hash_prime_20261030, eccp131_index_calculus_diagrams_20261026.

**Treatise coverage: PARTIAL.** prog_prime §ECCp-131 has the probe, the null (N/E 0.969–1.000) and the rho rates. Missing: the full-width tower, the representation-filter search, the CUDA port and the hash prime.

---

## 15. EcGFp5 / EcMasFp5 over F_{p^5}, p = 2^64 − 2^32 + 1 (atlas GFPN area)

**What was tried, 20 Sept–2 Nov.**
- A rho/SafeCurves audit.
- Symmetrised Gaudry with the rational (Z/2)^4 ⋊ S_5 quotient, solved by msolve.
- A u=x/y representation audit.
- A block-Wiedemann finish on the BNS22 chain matrix.
- An HNF-finish feasibility test.
- Joux–Vitse OA-SDH (designed, unrun).
- An odd-characteristic GHS cover (blocked on a source freeze).

**Key numbers.**

Audit:
- EcMasFp5 twist 101.9321 bits against 101.93 self-reported; EcGFp5 twist 120.66.

Quotient solve, m=5, at p' = 4111, 262231, 33554371:
- D = 2^16, confirming the law D = 2^{(m−1)²}.
- Whole-solve c_cyc 2^46.30 / 2^46.56 / 2^46.63, which is G1 CLOSE (≥2^46) by 0.30–0.63 bits.
- F4 alone 2^44.72–2^45.29.
- Five solver probes gave no improvement.

Block-Wiedemann finish:
- 1.72–1.97x lower CPU; c_cyc 2^45.43–2^45.84.
- The new point estimates sit below CLOSE, but the band straddles, so the verdict is unchanged.
- Distance to PROMOTE (2^38): 7.43, 7.58 and 7.84 bits.

HNF finish:
- A dev-scale memory-bound null at p=331.

**Verdict.** closed_dominated, with a measured per-system engineering improvement. Correction CORR-20261102-7e3a91 scopes "FHJRV ring form unavailable".

**Sources.**
- crypto: research/ecgfp5_lead_evaluation_20261031, ecgfp5_quotient_solve_20261101 (+benchmarks/20261101-ecgfp5-quotient-solve), ecgfp5_representation_audit_20261102, ecgfp5_quotient_bw_finish_20261102 (+benchmarks/20261102-ecgfp5-bw-finish), ecgfp5_quotient_hnf_finish_20261102, ecgfp5_quotient_presentation_20261102 (no README).
- AR: LEAD-ICP-gfp5-*, EV-GFPN-ba8e4d, EV-GFPN-fc5950.

**Treatise coverage: ABSENT.** summer:237 only says "no m=5 ideal degree was measured", which the 1 Nov solve has since superseded. That makes it a stale statement to correct.

---

## 16. Gaudry k=4 over F_{p^4} (odd characteristic)

**Covered** in prog_prime §Gaudry and Figure fig-prime (the D_4 system, C_4 47.5–48.4x, crossover 2^135.1–2^135.8).

**Missing.**
- gaudry_quartic_speed: 6x wall time with the same arithmetic.
- gaudry_quartic_replay: shape replay is a 2–5% lever, not 10x.
- gaudry_quartic_d4_ecbench: end to end at p=29 with every phase charged; rho-neg S 1.065 at L2; audit OK on 3/3.
- gaudry_allocation: allocation engineering.
- Leaderboard engineering row: sparsest pivots, chain eigensteps and shape replay cut the plain-S_5 C_4 from 1.209e12 to 8.690e10 (13.90x).
- fp4_d4_ladder (no README).

**Coverage: PARTIAL, small.**

**Sources.** crypto: research/gaudry_quartic_speed_20261013, gaudry_quartic_replay_20261013, gaudry_quartic_d4_ecbench_20261013, gaudry_allocation_20260914, docs/ic/LEADERBOARD.md.

---

## 17. Lifting, xedni, Hecke/CM, Prym, lattice IC, pairings, codes

**What was tried.**
- Classical xedni and the Cheon 2-isogeny lift.
  - p=353 example reproduced.
  - 2/2 historical pairs certified independent.
  - Xedni V2 found 0 certified dependencies among 20 pairs.
- A kernel-checked Lean audit of JKSST (1 Nov). It finds the C0/p obstruction conditional, refutes overbroad inferences, and reopens the lead as active.
- Function-field xedni: α=3 exactly (P_lift = (1−2/p+p^{−3})/(2p^3)), checked on 596,268,874 slots.
- Hecke/CM class points on X_0(11), D_K=−39:
  - Hecke leaves 12 free base logs against 32 scalar-only.
  - Pair-index reuse leaves 1, and Hecke's marginal rank over pair-only is 0, 10, 19, 0.
  - D79 evidence was not located.
- Prym witnesses: the owner-brief figures were not located.
- Lattice IC: a problem record; HNF memory blocks it.
- Self-pairing on a prime-order subgroup: closed by theorem.
- Elliptic-code list decoding (Zhang–Liu): desk check of the H1 heuristic against the Guruswami–Sudan bound; the paper text is unread.
- Representation/subset-sum: closed (generic).
- Embedding into F_{p^n}: closed (O~(p^{2−2/n})).

**Sources.**
- crypto: research/jksst_xedni_audit_20261101.
- AR: analysis/xedni-cheon, LEAD-ICP-char0-lifting-xedni-smoothness, -xedni-function-field-sections, -hecke-cm-class-point-rank-completion, -prym-witness-relation-diversity, -lattice-ic-relation-construction, -self-pairing-prime-order, -elliptic-code-list-decoding, -representation-subset-sum, -extension-field-embedding.
- cryptanalysis: experiments/hecke-sparse-target-20261030, hecke-d39-scalar-closure-20261030.

**Treatise coverage: PARTIAL.**
- Covered: obstructions §Lifting sections (function-field xedni α=3), summer table (xedni reject_scoped), lattices §rank-2 lattices.
- Missing:
  - the JKSST Lean audit, which corrects the treatise's "xedni closed" framing (flag as a correction);
  - Cheon certificates; Hecke/CM; Prym; self-pairing; the elliptic-code desk check; subset-sum; embedding.

---

## 18. Isogeny- and volcano-transported prime bases

**Covered**: prime61 (1.23e9 trials, zero relations), the P-256 million-curve walk, P-192/P-224 large-degree isogenies (prog_prime); conductor gaps and volcano levels (prog_transfer).

**Missing.**
- Arity-3 relation-oracle panel EXP-ECTD-0cf008: 17 of 20 arms lost to infrastructure.
- Conductor-gap floor/crater counted IC 0.975 [0.603, 1.451] (ℓ=8209) and 1.127 [0.922, 1.342] (ℓ=131101).
- A CM small-x window base exchangeable with random within ±0.04%.
- volcano_prime_layers: rho floor/crater 0.889 [0.661, 1.255].
- p192_p224 hundred maps (degrees 2341, 1117).
- P-256 64k multigraph (302,653 edges).
- P-192 extension cover: 680,474 lines impose zero constraints.
- prime_conductor_gap60/80 archival imports.
- prime61_disk_continue: 614,400,000 trials, zero relations.

**Coverage: PARTIAL**, overlapping the transfer family.

**Sources.**
- crypto: research/volcano_prime_layers_20261018, p192_p224_hundred_large_prime_isogenies_20261010, p256_isogeny_multigraph_64k_20261006, p256_j_windows_20261007, p192_extension_cover_20261012, prime61_disk_continue_20261018, prime61_isogeny_20261016, prime_conductor_gap60_20261031, prime_conductor_gap80_20261031, drastic_prime_conductor_gap_20261012 (+round2).
- AR: LEAD-ICP-isogeny-transported-relation-yield.

---

## 19. Retained setup, known-log and multi-target accounting (prime)

**Material.**
- ecbench_known_log_targets: a target-blind known-log two-sum method is cheaper in cold counted cost than the abscissa pipeline at 16/20/24 bits across the band. Negation rho is cheaper at every size; 12 bits is unresolved.
- The precompute regime (§1).
- Residual batch against multitarget rho parity (§3).
- L12 clause retention (§8).
- The retained-setup closure: BL rho at equal advice is 53–317x faster online.

**Coverage: PARTIAL.** The Koblitz retained-setup material is covered in distance and koblitz; the prime known-log targets and precompute regime are absent.

**Sources.** crypto: research/ecbench_known_log_targets_20261021, prime_precompute_regime_20261028; AR: LEAD-ICP-retained-setup-online.

---

## 20. Audits of standard prime curves (NIST, SECG, P-224)

**Covered**: seed provenance, basepoint, cyclotomic controls, P-256 trapdoor round 2, SafeCurves twists, SNFS families, P-192 singular cubic, P-224 invalid-curve, CryptoPro GLV, Cheon auxiliary inputs.

**Missing.**
- nist_seed_skip_filter: 21 new features, measured negative.
- nist_skip_criterion: a two-test empirical separator for the 21 eligible curves, with calibration that blocks a historical reading.
- nist_skipped_curve_dissection: 8 of 9 integers factored.
- nist_binary_skipped.
- nist_trapdoor_slot_program, round 0.
- nist_cyclotomic_point_heights.
- p224_hypothesis_ranking.
- p224_class_number: an estimate with an exact 2-part.
- p224_two_rank_null: the 91.5th percentile, upper tail 12.3% [10.3, 14.5].
- cheon_p224_shape: H3 gate failed (README not on main; only the run report).
- special_prime_curves.
- isogeny_walk_p192.
- nist_best2_small_batch: BEST2 stays disabled.

**Coverage: PARTIAL.**

**Sources.** crypto: research/nist_* (7 dirs listed above), p224_*, cheon_p224_shape_20261022, special_prime_curves_20261023.

---

## 21. Methodology objects specific to prime IC

**Material.**
- The six-obligation chain: geometry → solver degree → yield/variance → rank → non-generic interface → complete cost (ic_weak_assumptions, obligations.json).
- The m=2 yield sweep: observed/model 0.764–1.108 in nine cells.
- The construction normal form and estimator-bias check (open_unexamined).
- Owner track split, generic against algebraic (prime_ic_program DECISIONS.md R1, D1).

**Coverage: PARTIAL.** The track split is in distance §Prime; the obligation chain and normal form are absent.

**Sources.** crypto: research/ic_weak_assumptions_20261102, prime_ic_program_20261029/DECISIONS.md; AR: LEAD-ICP-construction-normal-form.

---

## 22. Binary-field items filed under ic-prime in the atlas

These are listed for completeness; other agents cover them.
- LEAD-ICP-icperf-binary-pdp-solver: SAT against Gröbner (EV-ICPERF-390707: m=3 slopes 0.53–0.58 against a MITM floor of 0.207).
- LEAD-ICP-icperf-writability-boundary: ANF minus rho budget is +40.8 bits at ECC2K-130 and −4.9 at n=409.
- LEAD-ICP-full-width-arity-grid: 63-bit panel blocked; six time-censored planted controls.

Probably handled by the ic-binary/engine agents; I did not check treatise coverage.

---

## Corrections and retractions to flag in the treatise

1. **Residual collectors.** The w>0 gains were a byte-counter pricing artefact. The owner adopted option B on 2026-11-01, and C1 falls from 6/8 to 2/8.
2. **Small-root.** tools/smallroot_reach.py compared the wrong exponent (CORR-20261024-b2a799): JM reach 0.8408 → 0.2803, so the earlier −0.048 margin is superseded.
3. **xedni.** The "closed" statements are superseded by the JKSST Lean audit: the obstruction is conditional, and the lead is active again (ecdlp_subexp_desk banner). The treatise's obstructions/summer framing should be qualified.
4. **EcGFp5.** summer:237 "no m=5 ideal degree measured" is now stale: D=2^16 was measured on 1 Nov.
5. **Cofactor-cleared prime base.** The verdict is `invalid_controls`; do not cite it as a win.
6. **Semaev-alternatives memo.** Its gate "e<1 at m=4" is wrong; the correct gate is s<m−3.
7. **The 20-bit N 64 attempt** is unresolved, and its 24/28-bit selections are relabelling.
8. **Hecke D79 and Prym figures.** The cited records could not be located; the figures are unverified.

---

## Summary table

| # | Line of work | Coverage | Est. pages it deserves |
|---|---|---|---|
| 1 | Generic decomposition track: k-sum k=4/5, m=4 2+2 MITM, arity sweep, S·T² gate correction | PARTIAL | 8 |
| 2 | Constant engineering of the prime pair-table pipeline (N45, filter, layout, batching, profile) | PARTIAL | 6 |
| 3 | Residual / large-prime collectors (res2 2.59x, res1eq 1.3–1.7x rho; byte-pricing correction) | ABSENT | 9 |
| 4 | Cofactor-cleared prime base (G 6.18, invalid controls) | ABSENT | 2 |
| 5 | PKM tower follow-ups (symmetric, P-224 shape, radix, t=14, matrix growth) | PARTIAL | 7 |
| 6 | APS partitioned summation polynomials | ABSENT | 3 |
| 7 | Digit presentation as Weil-descent analogue | PARTIAL | 4 |
| 8 | SAT/SMT/CDCL(T) lane (12 ETH leads, m=3 screen) | ABSENT | 6 |
| 9 | Divisor-norm / shared-coefficient direct IC (coverage bound, oracle contract) | ABSENT | 5 |
| 10 | Hashed-base Boolean tensor contraction (claimed sub-exp, audit, ports) | ABSENT | 14 |
| 11 | Split-graph / secant presentations | ABSENT | 4 |
| 12 | Structured bases and conservation in measurement (κ ladder, EV-FBG-001) | PARTIAL | 5 |
| 13 | Small-root reach theorems (Theorem K, e-form, S_4/S_5 caps) | PARTIAL | 7 |
| 14 | ECCp-131 full-width tower, representation filter | PARTIAL | 3 |
| 15 | EcGFp5 F_{p^5} quotient solve, BW finish, audits | ABSENT | 7 |
| 16 | Gaudry k=4 F_{p^4} engineering remainder | PARTIAL | 2 |
| 17 | Lifting / xedni (JKSST audit) / Hecke / Prym / lattice / codes / pairings | PARTIAL | 7 |
| 18 | Isogeny/volcano-transported prime bases (remainder) | PARTIAL | 3 |
| 19 | Known-log / precompute / multi-target accounting (prime) | PARTIAL | 3 |
| 20 | Standard-curve audits (NIST skip, P-224 class group, Cheon P-224) | PARTIAL | 6 |
| 21 | Prime-IC methodology (six-obligation chain, normal form, tracks) | PARTIAL | 3 |
| 22 | Binary items filed under ic-prime (writability, SAT vs GB, 63-bit grid) | not checked | (other agent) |

Totals for lines 1–21: 8 ABSENT, 13 PARTIAL, 0 fully COVERED. The existing prog_prime is about 6 pages. The estimated additional content is about 114 pages, which could be tightened to about 80–90.

### Suggested structure (prime fields, replacing the one-chapter prog_prime)

**Part: Prime fields (about 90 pp).**

- **Ch. P1. The prime-field problem and its two tracks (6 pp).** Lines 21 and 1-intro: S·T², the s<m−3 gate, the obligation chain, the owner track split.
- **Ch. P2. The generic track (22 pp).**
  - P2.1 Pair tables and Table B; base tuning (line 2).
  - P2.2 k-sum tables, k=4 and 5, pooled, and the precompute regime (lines 1 and 19).
  - P2.3 Residual collectors to 1.3x rho, with the pricing correction (line 3).
  - P2.4 Cofactor-cleared bases (line 4).
  - P2.5 Engineering record: filter, layout, profile (line 2).
- **Ch. P3. Algebraic presentations and solving degree (24 pp).**
  - P3.1 PKM towers and their follow-ups (line 5; link to falldegree).
  - P3.2 APS (line 6).
  - P3.3 Digit presentation (line 7).
  - P3.4 Split-graph/secant (line 11).
  - P3.5 Divisor-norm direct IC (line 9).
  - P3.6 SAT/CDCL(T) (line 8).
- **Ch. P4. The claimed sub-exponential tensor route (12 pp).** Line 10.
- **Ch. P5. Structure that did not help (14 pp).** Lines 12, 13, 17 and 18: conservation in measurement, small-root reach theorems, lifts and xedni with JKSST, Hecke/Prym, isogeny transport.
- **Ch. P6. Named targets (12 pp).** ECCp-131 (line 14); EcGFp5 over F_{p^5} (line 15); Gaudry k=4 (line 16, existing); standard-curve audits (line 20).

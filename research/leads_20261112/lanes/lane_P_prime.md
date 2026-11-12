> Agent report, verbatim (2026-11-12). Proposals only; nothing was run; every statement in this file is `[pcc:unverified]`.

# Lane P leads: prime fields E(F_p), cells 0.1, 0.2, A12 and new prime-field mechanisms (2026-11-12)

Author role: Idea Generator. Proposals only. Nothing was executed (no shell). Every number is either quoted from a file on disk (path given) or marked "derived" with the arithmetic shown. All standings are `[pcc:unverified]`; I looked up no pcc claim ids. No repository file was edited, no hypothesis status changed, no agent dispatched.

## Sources checked

- research-kb MCP: NOT available in this session (no kb_status / search_knowledge tool). Repository files and web search were used instead. The atlas bodies are vault-encrypted, so the exact definitions of cells 0.1, 0.2 and A12 were not readable. Assumptions about them are stated inside the proposals that use them.
- Dedup lists read: `atlas_lead_ids.txt` (176), `EXISTING_LEADS.txt` (173), `lane_A_falldegree.md`, `lane_C_prime_ext_cover.md` (binary-field fall degree and cover leads; no overlap with the prime chain).
- Fall-degree evidence (ml-cryptanalysis, read): `docs/proofs/N8-growth.md` (whole), `experiments/ic-bounds-k14/RESULTS.md` (whole), `docs/IC_THEOREM_LEDGER.md` lines 215-265, `docs/IC_BOUNDS_PROGRAM.md` lines 155-215, `docs/session-logs/2026-11-10.md` lines 165-225.
- Crypto repo (read): `docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` (Frontier table: `GAP-PF-PRIME-RELATION-SEARCH`, `GAP-PF-SPECIAL-PRIME-EXPOSURE`, `GAP-PF-TOWER-SOLVING-DEGREE-HT`, `GAP-PF-TRACE-PULLBACK-FACTOR-BASE` open; `THM-PF-REPO-KUMMER-FFD-FOUR` unreviewed); `research/prime_field_lead_assessment_20261107/README.md` (whole); `research/ic_planner_exhaustive_20261026/README.md` (whole); `research/joux_vitse_n1_20261015/README.md` (whole); `research/ic_e222_m221_20261106/RESULTS_multi_target.md`; `research/mt_fieldops_v3_20261111/RESULTS.md`; `papers/ecdlp-index-calculus-treatise/sections/prog_prime.tex` lines 30-200. `research/jv_nagao_hilbert_20261112` was not found on the checked-out tree; I cite only what exists.
- Literature (web, standard mode; abstracts and slide snippets only, full texts not read):
  - Petit, Kosters, Messeng, "Algebraic approaches for the ECDLP over prime fields", PKC 2016 ([PDF](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf), [slides](https://troll.iis.sinica.edu.tw/pkc16/slides/6-1--Algebraic_approaches_for_the_Elliptic_Curve_Discrete_Logarithm_Problem_over_Prime_Fields.pdf)): bases `{L(x)=0}` with L built from the factorisation of p-1 (maps X^q, X^q-1); P-224 has p-1 = 2^96 N'; an isogeny variant with an auxiliary curve having a subgroup of order S; "how to solve the system" is listed as open.
  - Yokoyama, Yasuda, Takahashi, Kogure, "Complexity bounds on Semaev's naive index calculus method for ECDLP", J. Math. Cryptol. 2020 ([link](https://www.degruyterbrill.com/document/doi/10.1515/jmc-2017-0022/html)): lower bound on the Groebner cost of point decomposition under statistical assumptions; naive IC not better than generic.
  - Amadori, Pintore, Sala, "On the discrete logarithm problem for prime-field elliptic curves" (CANS 2018, Finite Fields Appl.): hybrid exhaustive search plus Groebner, symmetries of summation polynomials; constant-factor.
  - Kousidis, Wiemers, [arXiv:1906.05594](https://arxiv.org/pdf/1906.05594): first-fall bound m^2-m+1 for binary Weil descent (not prime; context only).
  - Dual-space (Macaulay inverse system, Mourrain-type) methods: one search, no direct comparison of dual-method cost against Macaulay-matrix size was found. I rely on none of it.
  - A 2024-2026 search for a prime-field Kummer-base degree-growth lower bound or an F4 cost law returned nothing; this is a negative search, not proof of novelty.

## The evidence the proposals build on (quoted, then derived)

Quoted: [pcc:unverified]
- Kernel (N8-growth, IC_THEOREM_LEDGER): LFD <= k+2; measured plateaus LFD = 6 at k = 10..13, 7 at k = 14; proved unbounded, slope >= 1/14 (`N8-growth.md` T1, C1); open: slope between 1/14 and 1, t = 0, m >= 3, proper ideals.
- `ic-bounds-k14/RESULTS.md`: k=14, D=6 closure 4.69e15 multiply-adds, 8077 s on an H200 (6393-8077 s on the three instances), 499,178 columns, dim V_6 = 448,922, codimension 50,256; the D=6 -> 7 lift took 677-751 s, regular weight-7 columns 586,501 per part, dim T = 30,655; "Codimension of V_D in M_<=D at the first deficient depth 4D-10: 10, 189, 3131, 50,256 for D = 3, 4, 5, 6". Profile recursion p_j(k+1) = p_j(k) + 2 p_{j-1}(k) + p_{j-2}(k) holds k = 10..15 (D=5, predictions `d5-closed-form` 56,649 and 66,073 confirmed).
- Three-summand: p = 7681, k = 6: V_7 closed, dim 19,958 against 62,956 for the ideal part, so LFD >= 8 (session log 2026-11-10, 10:05 entry).
- Matched-rho facts: no prime row is below rho; multi-target ic5 is 1.07x-1.34x a retention rho at h = 0 and 1.11x-1.49x BSGS (`mt_fieldops_v3_20261111/RESULTS.md`); res2 collector 2.6-2.7x rho at 36 bits (`ic_planner_exhaustive_20261026`, exbytes note).

Derived (arithmetic shown, in-sample):
- Penultimate-degree quotient dimension. Let q(D,k) = |M_<=D(k)| - dim V_D(k) with |M_<=D(k)| = sum_{j<=D} C(2k, j), at the penultimate degree D = LFD(k)-1, and j = k-(4D-10) the position in the LFD plateau. Values of q / 2^k (columns and dims from `N8-growth.md` section 6.2 and `RESULTS.md`):

| D | j=0 | j=1 | j=2 | j=3 |
|---|---|---|---|---|
| 3 (k=2..5) | 10/4 = 2.50 | 35/8 = 4.38 | 84/16 = 5.25 | 165/32 = 5.16 |
| 4 (k=6..9) | 189/64 = 2.95 | 642/128 = 5.02 | 1456/256 = 5.69 | 2747/512 = 5.37 |
| 5 (k=10..13) | 3131/1024 = 3.06 | 10410/2048 = 5.08 | 23398/4096 = 5.71 | 44025/8192 = 5.37 |
| 6 (k=14) | 50256/16384 = 3.07 | not run | not run | not run |

  The columns agree across D to 1% at D = 4, 5 and the j = 0 entry converges (2.50, 2.95, 3.06, 3.07). The quotient is about 3 to 6 times B = 2^k, not about 100 times B. I found this after reading the numbers, so it is in-sample.
- Three-summand cell: q = 63,004 - 19,958 = 43,046 = 10.5 * B^2 with B = 64 (one cell).
- Columns against B at the measured LFD: 60,460 / 1,024 = 59x (k=10, D=6); 499,178 / 16,384 = 30x (k=14, D=6); 1,683,218 / 16,384 = 103x (k=14, D=7).
- Measured cost exponent of the D=6 closure from k=10 to k=14: log2(4.69e15 / 9.34e12) / 4 = 2.24 per doubling of B (derived from RESULTS rows; k=10..13 are full-space closures, k=14 deficient).

---

## Lead P1 (ranked first)

```yaml
id: LEAD-P-kummer-penultimate-quotient-law
title: The penultimate-degree quotient of the two-summand Kummer chain has dimension about 3-6 B, so a closure-type solver cannot undercut enumeration of the base
cell: 0.1 (flagship Kummer g*mu_{2^t}, F4 at P-224)
obstruction_attacked: >
  The fall-degree law LFD ~ (k+14)/4 makes the F4 matrix grow superpolynomially in B = 2^k, but the program has no statement of what a
  closure-type solver must pay at best, so "F4 at P-224" stays OPEN. The right baseline for a decomposition test is not rho: it is
  enumerating the B base points (cost ~B tests), because that already gives the relation. A closure solver is only interesting if it is
  below that. Name of the obstruction: the quotient left at degree LFD-1 is about the size of the enumeration set.
claim: >
  (C1, empirical law) For unit-ideal two-summand Kummer chains over F_p with 2^(k+1) | p-1, at D = LFD(k)-1 and
  j = k-(4D-10) in {0,1,2,3}, q(D,k)/2^k = c_j(D) with c(D=4,5) equal to (2.95-3.06, 5.02-5.08, 5.69-5.71, 5.37) to within 2%, and the
  predictions for the untested D = 6 cells are q = 167,665 (k=15), 378,423 (k=16), 715,797 (k=17), i.e. c = 5.12, 5.77, 5.46
  (derived from the profile recursion, so they test the recursion, not the law).
  (C2, corollary) Any solver that stores or eliminates the penultimate quotient costs at least about 3B per target; with the measured
  closure exponent B^2.24 the F4 route is above the enumeration cost 350*B field multiplications at every measured k and every
  tractable k (columns/B = 30x-103x at k = 10..14).
  (C3, conjecture to test) The same holds for m = 3 with B^2 in place of B (one cell: 10.5*B^2 at k = 6).
mechanism: >
  Conjecture, not established: the quotient at degree LFD-1 is a length-Theta(B) scheme equal to the points of the Semaev curve
  S_3(x1,x2,t) = 0 with ONE coordinate in the coset (2B points over the algebraic closure, 4B for both coordinates, minus the |Z|
  true solutions), because the relaxation at that degree has not yet coupled the second base constraint. The N8 locality theorem
  (low-weight ideal elements cannot connect exponents 2^k apart, `N8-growth.md` section 5) says exactly that the two constraints are
  decoupled below LFD. If this is right, the fall degree measures the cost of coupling the two halves of an enumeration set, and
  the same length-Theta(B) obstruction applies to any presentation (Lemma 2 of the sibling note reaches d_F = 2 only by adding
  Theta(B) extension variables).
prediction:
  direction: "q/B stays in [2.5, 6.0] for every D and every plateau position in the committed cells"
  numbers: "see C1; c_0 limit about 3.07 (3+1/16 = 3.0625 is a pattern, not a derivation)"
  columns_vs_B: "ratio >= 25 for every committed (D,k) with k >= 6 (computed: 59, 53.7, 46.4, 38.3 for D=6,k=10..13; 30 for D=6,k=14; 103 for D=7,k=14)"
discriminating_test:
  stage_0_desk: >
    Recompute the 12 committed q values from the committed profile tables (all numbers are on disk: N8-growth.md section 6.2 and
    RESULTS.md). Fit q = c*B + e on the plateau and report residuals. Seconds; no run. This repeats my in-sample computation with an
    independent tool (native, Rust or the kernel's own `decide`) and is the first gate.
  stage_1_out_of_sample_local: >
    D = 3 and 4 on fresh primes (p with 2^(k+1) | p-1, at least 3 curves, random and j=1728) for k = 2..9 with the existing C closure
    kernel: predicts the same c_j within 2%. CPU minutes (columns <= 4,048). Existing data already say instance-independence at D=5, 6.
  stage_2_gpu: >
    Direct closure D = 6 at k = 15 (columns 768,212): predicted q = 167,665 exactly (derived). About 6 H200-hours by scaling 4.69e15 *
    (768,212/499,178)^2.3 (derived estimate, not measured) at the earlier pod price ($5.29/h, about $32). Only if stage 0 and 1 hold.
  three_summand: "p = 7681, k = 7, D = 7 would need ~198,440 columns (C(21,<=7)); predicted q/B^2 in [8, 20]; skip unless stage 0 holds"
falsification: >
  C1 is killed if any stage-1 cell leaves [2.5, 6.0] for q/B, or the D=3,4,5 columns disagree with D=6 j=0 by more than 5%.
  C2 is killed only by a solver whose measured work is below ~350*B multiplications on a cell the closure already solves: for
  example a closure variant whose emitted mult-adds at k=14, D=6 are under 1.4e7 (350*16,384*2.4). C3 is killed by a three-summand
  cell with q/B^2 below 1 (the penultimate quotient is then smaller than the enumeration set for m=3).
cost_band:
  unit: "field multiply-adds; enumeration test = ~350 multiplications (one sqrt plus a 224-bit power at P-224 size; for the toy field k squarings suffice because N = 2^k)"
  arms:
    - "F4 closure to D=6 at k=14 (counted): 4.69e15 mult-adds (RESULTS.md); unpriced native work (tile spills, 1.03 TB moved) priced at w = 0, 0.1, 1 of a GAE per operation"
    - "enumeration of the B = 16,384 base points, 350 mult each: about 5.7e6 multiplications, no table, no spill"
    - "matched negation rho at p = 65,537: a few hundred group additions (about 4e3 multiplications); unpriced canonicalisation, hash, DP check priced at the same w (section 5a)"
    - "stage 2 closure at k=15: ~1.3e16 mult-adds (derived estimate)"
  ranking_rule: "F4 vs enumeration: order is 10^9 in favour of enumeration at every weight; claimed only because it holds across the whole band"
controls:
  - "negative: replace g by a random weight-2 element with the same chain; q/B should differ (the law should be specific to the Semaev g)"
  - "positive: the D=5 plateau cells reproduce 3131 / 10410 / 23398 / 44025 exactly"
  - "A/A: two curves per cell; q must agree to the digit (it does at D=5, 6 in committed records)"
risks:
  - "In-sample discovery; the column agreement D=4 vs D=5 is partly forced by the profile recursion plus the base codimension."
  - "C2 is a heuristic about closure-type solvers, not a theorem: an algorithm that never forms the penultimate quotient (resultant/gcd, lattice) is not covered. The Graeffe-resultant test costs O~(B) and also matches enumeration, so it does not undercut it."
  - "m = 3 rests on one cell."
closest_baseline: >
  Published: Petit-Kosters-Messeng 2016 (slides list the solving degree as open); Yokoyama-Yasuda-Takahashi-Kogure 2020 (lower bound on Groebner cost under
  statistical assumptions; no Kummer base). In repo: N8-growth.md T1/C1 (LFD unbounded), RESULTS.md codimension row (numbers without the ratio to B),
  `GAP-PF-TOWER-SOLVING-DEGREE-HT` (H_T open).
nearest_existing_lead_and_difference: >
  LEAD-ICP-pkm-tower-algebraic-oracle (open_blocked; measures solving degree D, the shape-matched null equals it) and LEAD-ICE-prime-field-semaev-solving-degree
  (degree growth). Both ask how large D is. This lead asks what the closure must store at D-1 and compares it with the enumeration baseline,
  turning degree growth into a size statement (cost >= 3B) that does not depend on the slope.
lean_obligation: >
  (a) Finite certificates: for every committed (D,k) cell, |M_<=D(k)| = sum C(2k,j) and the inequality columns > 25*2^k, plus the quotient
  values, closed by `decide` in the style of the planner study's `Certificates.lean` (recorded dimensions enter as named hypotheses exactly as in
  `DepthFourteen`). (b) None for C1-C3 as stated; they are conjectures. The conditional corollary "a dense representation of a quotient of
  dimension q has >= q coordinates" is trivial.
status: conjecture (C1 in-sample, C3 one cell); all numbers [pcc:unverified]
next_steps:
  - "1. Experiment-director (Sonnet) card: stage 0 recomputation of the 12 q values with a native tool; 30 minutes, local."
  - "2. Executor (Haiku): stage 1, D = 3 and 4 on 3 fresh primes, k = 2..9, existing C kernel behind a native driver; 2 hours, local."
  - "3. Idea Generator / frontier model: write the length-Theta(B) scheme conjecture as a checkable statement (count the points of S_3 = 0 with x1^N = c over the closure) and compare it with q at D=4, k=6..9; 1 day, desk."
  - "4. Director: price stage 2 only if 1-3 hold; owner authorises the rented GPU route (the k=14 runs used one, RESULTS.md 'Machine and cost'); I do not authorise it."
  - "5. Coordinator: if C2 survives, record `GAP-PF-TOWER-SOLVING-DEGREE-HT` as 'closure-type solvers: cost >= 3B per target' and demote cell 0.1 to engineering-only."
```

---

## Lead P2

```yaml
id: LEAD-P-kummer-freeext-lift-k15-18
title: Seed the degree-lift from the free-extension basis to measure LFD at k = 15..18, the first data past the conjectured plateau end
cell: 0.1 (exploits the exact degree law and the profile recursion)
obstruction_attacked: >
  The slope of LFD in k is the quantity every cost statement about cell 0.1 hangs on: proved >= 1/14, measured 1/4 up to k = 15, the
  conjecture floor((k+14)/4) predicts LFD = 7 through k = 17 and 8 at k = 18. No depth beyond 15 has data because the direct closure
  at k = 16..18 is out of reach (columns 1.15e6 at D=6,k=16; 2.5e6 at D=7,k=17, derived: C(32,<=6), C(34,<=7)). The program's own structure offers a way around:
  V_D(k) is a free extension of V_D(k0) for k >= k0+1 (Reduction B, `N8-growth.md` section 7.2; basis hash equality at D=5, k=12).
claim: >
  Computing V_6(k) for k = 15, 16, 17 as free extensions of V_6(14) (monomial shifts of the 14-bit basis, no new elimination) and then
  lifting D=6 -> 7 with the committed lift gives, on the unit instance over p = 65537:
  (i) at k = 15, the free-extension basis equals the direct closure V_6(15) by basis hash (the test of Reduction B at D=6);
  (ii) 1 in V_7(k) for k = 15, 16, 17 (LFD = 7 exactly), with dim V_7(k) = |M_<=7(k)| - |Z(k)|;
  (iii) stage 3, the decisive one: 1 not in V_7(18), i.e. LFD(18) >= 8.
  A failure of (ii) at k <= 17 or of (iii) at 18 changes the slope.
mechanism: >
  The plateau for D = 6 is k = 14..17 and the base of V_6 is k = 14 = 4D-10, so V_6(15..17) is already determined by the free-extension
  recursion (profile and basis) from the single closure that was run. The lift computes weight-7 data only (dim T = 30,655 at k = 14,
  677-751 s against 6393-8077 s for the closure), so the cost of the final degree is a fraction of the closure it replaces. At depths
  where the lower closure is too large, only the lift remains, and its input is implicit.
prediction:
  stage_1: "free-extension basis hash = direct V_6(15) hash; dim V_6(15) = 600,547 (p_6(14) + 2*73,587 + 4,451, derived)"
  stage_2: "1 in V_7 at k = 15, 16, 17 (probability about 0.9 by the conjecture and the plateau width four seen at D = 3, 4, 5, 6)"
  stage_3: "1 not in V_7(18); codim of V_7(18) about 3.07 * 2^18 = 8.0e5 (extrapolated from c_0(D); see P1)"
  q_check: "codim V_6(k) = 167,665 / 378,423 / 715,797 at k = 15 / 16 / 17 (derived)"
discriminating_test:
  stage_1: "direct V_6(15) closure (about 6 H200-hours, derived estimate) vs the free-extension basis: hash equality. If unequal, Reduction B fails at D=6 and the whole pipeline stops (it is then a refutation of the free-extension law, itself a result)."
  stage_2: "lift at k = 15, 16, 17: regular weight-7 columns 2*C(30,7) = 4.1e6, 2*C(32,7) = 6.7e6, 2*C(34,7) = 1.08e7 (derived), against 1.17e6 at k=14; memory is the risk, wall estimate unknown (scale 750 s by columns * dim T; dim T at k >= 15 unknown); stop at the first memory failure and report it as a resource limit, not a refutation."
  stage_3: "lift from V_6(18) (free extension of V_6(15)): 2*C(36,7) = 1.66e7 columns; only attempt if stage 2 at k=17 completed inside the budget."
falsification: >
  (i) hash mismatch kills the free-extension law at D=6 (strong result). (ii) any k in 15..17 with 1 not in V_7 shows LFD >= 8 earlier than the
  conjecture (slope above 1/4). (iii) 1 in V_7(18) shows the plateau is longer than four (slope below 1/4), which would lower the column
  exponent in P1's calculus and is the one outcome that reopens cell 0.1 on its slope. Not killed by memory or time-outs.
cost_band:
  arms:
    - "stage 1 closure: ~1.3e16 mult-adds (derived), ~6 H200-hours"
    - "lifts at k = 15..17: unknown; estimate 1-10 H200-hours each (derived, wide)"
    - "reference: committed k=14 run, 8077 s closure + 751 s lift, $5.29/h"
    - "ECDLP reference arm: none; this is a degree measurement. Any claim about cost against rho is made only through P1's enumeration baseline."
controls:
  - "positive: reproduce the committed k = 14 lift (dim 1,683,218, falls 494 / 12,260 / 12,382) with the same binary, hashes equal"
  - "negative: the committed control that the lift produces nothing outside the ideal (dim V_7 = columns - |Z|), here by enumerating |Z(k)| from the curve"
  - "A/A: two unit instances (p = 65537, p = 163841, both exist) must agree on dim T and dim K per round"
risks:
  - "Expensive; memory at k >= 16 may block (the k=14 basis alone was about 45 GB)."
  - "A confirmation (plateau width four) is the expected, low-information outcome; only stage 3 is decisive."
  - "The written proof of the lift (LIFT.md) has no independent certificate and no Lean file."
closest_baseline: >
  In repo: `ic-bounds-k14/RESULTS.md` (k = 14 closure, lift, certcheck), `N8-growth.md` Reduction B and Corollary 5.3 (free extension proved from D+Lambda(D), measured from 4D-9).
nearest_existing_lead_and_difference: >
  LEAD-ICE-descent-closure-degree-at-scale and LEAD-ICP-pkm-tower-algebraic-oracle (measure degrees by direct runs). This lead measures by seeding
  the lift with the free-extension basis, a different pipeline that does not require the lower closure; the nearest sweep ids do not use it. The
  prime-field analogue of "use a degree known in advance" (the Hilbert early stop on the Nagao arm), but on the dimension profile instead of the
  Hilbert function.
lean_obligation: >
  Reduction B: for every D, T_D (the wrap-free closure of S_3 under shifts) splits at 4D-9, equivalently V_D(k') = (+)_h x^(2^k h) (V_D(k) cap M_<=D-wt h)
  for k >= 4D-9. Proposition 7.2(c) is a written proof of its consequences once splitting holds; splitting itself is a finite check for each D and
  open for all D. For D=6 the finite statement is the one stage 1 tests. The lift's correctness lemma (LIFT.md) is the second obligation.
status: conjecture plus measurement; [pcc:unverified]
next_steps:
  - "1. Owner decision on the rented-GPU route (isolab-eks is the default for x86 CPU work; the k=14 runs were on a rented H200). 1 hour."
  - "2. Director card for stage 1 with pass/fail on hash equality; 1 day to freeze."
  - "3. Executor: stage 1, then stage 2 at k = 15 only; 1 day wall."
  - "4. Director decides stage 2 at 16, 17 and stage 3 from the measured memory and wall at k = 15."
  - "5. Frontier model: write the Lean statement of the free-extension equality for fixed D = 6 as a kernel obligation (`DepthRestriction` style); 2-3 days."
```

---

## Lead P3

```yaml
id: LEAD-P-kummer-radix-presentation
title: Coarse-radix Kummer chains: fewer variables at equal base size, measured cost against the binary chain
cell: 0.1 (a presentation of the same base; escapes the Boolean-variable count, not the degree law)
obstruction_attacked: >
  The binary chain has 2k Boolean variables, so the closure matrix at degree LFD has C(2k, <= LFD) columns, about 2^(1.09k) at the measured slope
  1/4 (derived: 2k*H(1/8) with H the binary entropy, 0.5436). That is above B = 2^k, so the column count alone defeats the solver.
  The presentation is a free parameter: x = v_0, v_j^r = v_(j+1), v_(last)^r = 1 with r = 2^rho has only 2k/rho variables per pair.
  Lemma 2 of the sibling note shows presentations can lower d_F arbitrarily at the price of variables; no one has measured the opposite
  direction (fewer variables, higher-degree chain relations) for the Kummer base.
claim: >
  (a, derived, to be proved) LFD_binary(k) <= max(LFD_radix(k), r): every radix-r derivation of degree D is a binary derivation of weight <= D, because
  a radix-r monomial of degree d has binary weight <= d, and the radix chain inputs are in the degree-r binary closure. So radix presentations cannot
  lower the degree below the measured floor((k+14)/4) once D >= r.
  (b, to measure) For rho = 2 and 3 (radix 4, 8) at k = 12, 16, 20, 24: D_rho(k) - D_2(k) lies in [0, r]; the Macaulay columns C(v + D, D) with v = 2k/rho
  at the measured D are below the binary C(2k, <= D) by a factor of at least 4 at k = 16 (derived for rho = 2, D_rho = D_2 + 1 = 8: C(16+8,8) = 735,471 against
  C(32,<=7) = 4,514,873, a factor 6.1), and the exponent of columns in B at the plateau slope s = D/k = 1/4 is (2/rho + s) H(s/(2/rho + s)): 1.087 (rho = 1), 0.902 (2), 0.775 (3),
  0.689 (4) (derived).
  (c, prediction) Even at the best exponent, dense elimination costs columns^2 = B^(1.38) (rho = 4) to B^(1.8) (rho = 2): above the enumeration cost B.
mechanism: >
  Coarsening is elimination of the odd-position variables: u_(2j+1) = u_(2j)^2. Elimination cannot lower the closure degree below the finer presentation
  (a), but it removes variables, so the Macaulay matrix at equal D is smaller. The trade is exactly one parameter (rho) and the product of two effects
  (smaller matrix, larger D offset r) is the experiment.
prediction:
  point: "D_rho(k) = max(r, D_2(k)) + 1 for k <= 24 (the relation degree r dominates at small k): D_4(12) = 7 (D_2(12) = 6), D_8(24) = 10 (D_2(24) = 9)"
  interval: "D_rho(k) - D_2(k) in [0, r]; columns at (rho = 3, k = 24, D = 10): C(26,10) = 5.3e6 = 2^22.3 against B = 2^24 (derived): columns below B, dense cost 2^44.6 against enumeration ~2^32"
  gate_m3_m4: "for m = 3 and 4 summands at k <= 12 the same measurement gives the cost exponent e against the program's gate e < m/2 - 1 (0.5 at m = 3, 1 at m = 4): predicted to fail by >= 3x at every cell"
discriminating_test:
  smallest_run: >
    Native Rust F4 (the existing `f4_fp_tower` path in crypto, not verified here) or the C closure kernel behind a native driver, p with 2^(k+1) | p-1 (unit instance
    y^2 = x^3 + a x with a non-square target abscissa, as in `UnitInstance`), rho in {1, 2, 3}, k in {8, 12, 16} first (columns <= 1.2e4 at rho = 2, k = 12, D = 7;
    <= 7.4e5 at rho = 2, k = 16, D = 8), two curves, LFD exact by the local-to-global certificate used in RESULTS.md. Each job under 20 minutes on 4 vCPU / 15 GB at k <= 12;
    larger k are separate jobs.
  control: "rho = 1 reproduces the committed binary values (LFD 6 at k = 10..13, 7 at k = 14); the radix-r chain with r = 2 is the binary system (identity)"
falsification: >
  Killed if D_rho(k) - D_2(k) exceeds r at two or more cells (the coarse presentation raises the degree faster than it shrinks the matrix), or if
  the measured columns at LFD for rho = 3, k >= 16 are not below the binary columns by at least 4x. The ranking claim (c) is killed by a measured dense
  cost below 350*B multiplications at any k.
cost_band:
  arms:
    - "binary chain closure (committed): 4.69e15 mult-adds at k=14, D=6"
    - "radix-4 / 8 closures: unknown; estimate columns^2.3 (derived), e.g. (7.4e5)^2.3 = 3e13 at (rho = 2, k = 16, D = 8)"
    - "enumeration baseline: 350*B multiplications"
    - "matched rho at the toy field: as P1; unpriced counters priced at w = 0, 0.1, 1"
risks:
  - "Expected outcome is a null for the ECDLP claim (c) and a small engineering gain in matrix size; the value is closing the presentation axis."
  - "A radix-r chain needs the non-Boolean quotient: monomial count C(v + D, D) overcounts reduced monomials by exponents >= r when D >= r."
  - "p must keep 2^(k+1) | p-1, so k <= 24 means 25-bit primes; the F_p arithmetic is cheap, the closure is not."
closest_baseline: >
  Published: Petit-Kosters-Messeng base mu_{2^t} (binary tower of squarings); the slides list digit-style maps X^q. In repo: Lemma 2 (extension-variable
  presentation, d_F -> 2), `D3 prime_field_m2_lfd_bound` (LFD <= 2k+1), PresentationTransport (same-budget transport).
nearest_existing_lead_and_difference: >
  LEAD-ICP-digit-presentation-solving-degree (base-B digit variables of the integer coordinate with d_ff = m 2^(m-1) + ..., curve-independent) and
  LEAD-ICE-quadratised-chain-representation (the finer direction, more variables). This lead keeps the multiplicative chain and varies its radix
  (the coarser direction); the digit lead's null result says that presentation Groebner measurements can be curve-free, so the P3 control rho = 1 plus
  a random-coefficient chain (same shape, random g) is mandatory.
lean_obligation: >
  Claim (a): `LFD_binary <= max(LFD_radix, r)`, a statement about a ring homomorphism K[v] -> K[u] sending v_j to u_(rho j), weight not exceeding degree, and the
  radix inputs lying in the degree-r binary closure; it fits `PresentationTransport.same_budget_exact_iff` with budget max(D, r). Not yet written.
status: derived (a, unproved), conjecture (b, c)
next_steps:
  - "1. Frontier model: prove (a) on paper, then specify the Lean statement; 1-2 days."
  - "2. Director card: rho = 1, 2 at k = 8, 12 with a random-g control; native F4; 1 day."
  - "3. Executor: run; 2 hours local."
  - "4. Director: extend to rho = 3 at k = 16..24 and to m = 3, 4 at k <= 12 only if step 3 shows D_rho - D_2 <= r."
  - "5. Coordinator: record the presentation axis as closed for the Kummer base or as an engineering gain (matrix size) with its measured offset."
```

---

## Lead P4

```yaml
id: LEAD-P-isogeny-transport-is-plain-base
title: The isogeny-transported base reduces to the plain base on the isogenous curve with a computable target; one lemma and one class-wide invariance test close cell 0.2's fall-degree question
cell: >
  0.2 (isogeny-transported base x(R'+<Q>), V at P-192). Assumption: I could not read the atlas body; I take the base to be phi(U), the image of a structured
  set U (a Kummer coset or a subgroup coset of E') under an isogeny phi: E' -> E of degree d coprime to the group order l, with E and E' of the same prime order l
  as every curve in the P-192 class.
obstruction_attacked: >
  Whether moving the structured base to an isogenous curve changes the closure degree or the yield. The census gives the yield side (isogeny-class generic invariance: 26-bit census,
  403,168 runs, 33 of 35 level ratios TOST-equivalent; crater and floor IC 0.975 [0.603, 1.451] and 1.127 [0.922, 1.342], `prime_field_lead_assessment_20261107` sections 2b and 3).
  The fall-degree side has no class-wide measurement: committed LFD values are on j = 1728 and random curves, never across one isogeny class.
claim: >
  (L, derived, algebra) For a nonzero homomorphism phi between the groups E'(F_p) and E(F_p), both cyclic of prime order l, phi is a bijection, so
  T = phi(P1') + phi(P2') iff P1' + P2' = T' := phi^-1(T) in E'(F_p); the transported decomposition problem for the base phi(U) is the plain problem for U on E' with target T'
  and the same unknown logarithm (DLP instance (phi_hat(G), phi_hat(Q)) on E'). Hence the closure degree, the quotient dimension q of P1 and the yield are those of the plain system on (a', b', t').
  (M, to measure) Across >= 20 members of one isogeny class and >= 2 targets each, the exact LFD and the profile of V_D(k) at k = 7, 8 (D = 4, 5) are identical, to the digit.
mechanism: >
  The Semaev system for (E', U, T') is the plain system. The only freedom the transport adds is the choice of (a', b') in the class, and the measured law says the profile is
  instance-independent (j = 1728 versus random curves: equal dimensions and profiles at D = 5, 6, three primes). The expected value of a representative search is therefore zero for
  closure degree and 5.8-7.2% for yield at 24 bits (`LEAD-STR-class-representative-ic-selection`, quoted in the 2026-11-07 assessment).
prediction:
  M: "identical dimension profile (and LFD) on 20 of 20 class members at every tested (D, k); in the unit-ideal case, the same 0/1 pattern for 1 in V_D"
  exceptional_targets: "the only deviations are the known exceptional classes (order-two, tangent 3t^2 + a = 0, t = 0 subcases): each with probability about 1/p per random target, so no class walk reaches them cheaply (derived: needs about p candidates per hit)"
discriminating_test:
  smallest_run: >
    A small prime with a large isogeny class (for example p = 12289 or 40961, where 2^(k+1) | p-1 for k <= 8 is available; the class has about 10^2 members), horizontal
    l-isogenies by Velu with small l from the existing `isogeny_algos` crate, 20 curves, 2 targets each, exact profile of V_D(k) at k = 7, 8 (D = 4, 5: columns <= 6,884).
    Native driver around the existing C closure kernel; no Python. Minutes.
falsification: >
  Killed by any class member whose dimension profile differs from the others at the same (k, D) outside the known exceptional classes (a representative effect on closure degree), which
  would reopen representative selection for the Kummer base.
cost_band:
  arms:
    - "closure at k = 7, 8: seconds per cell (D=5, k=8: 6,884 columns)"
    - "isogeny walks: Velu O(l) per step, small l; a few hundred field multiplications each"
    - "ECDLP reference: none claimed. Enumeration and rho baselines are those of P1; no cost against rho is asserted for this lead."
controls:
  - "positive: a j = 1728 curve in the same class set (if present) equals the others at the same (k, D); committed D=5 values at k=10..13 are the larger-depth analogue"
  - "negative: the same measurement on a curve that is NOT isogenous (different order) is allowed to differ only through the known exceptional classes"
  - "A/A: two targets per curve"
risks:
  - "Expected outcome: invariance. The value is the closing sentence for cell 0.2, not an improvement."
  - "If the atlas definition of 0.2 is different (for example x-coordinates of an arithmetic progression R' + jQ with Q of full order), the lemma still applies (the base is a set on E' with known logarithm) but the cell is then the generic known-log table, REF by conservation and by the BSGS/ic5 results (ic5 is 1.11x-1.49x BSGS)."
closest_baseline: >
  Published: Petit-Kosters-Messeng isogeny variant (auxiliary curve with a subgroup of order S; the auxiliary-curve search for a P-192-sized tower is priced at about 2^74 GAE in the treatise);
  Galbraith-Hess-Smart style walks (binary/extension). In repo: `research/pkm_isogeny_tower_ecbench_20261015` (isogeny tower 1.00-1.33x plain base once the auxiliary search is charged).
nearest_existing_lead_and_difference: >
  LEAD-ICP-isogeny-transported-relation-yield (yield, crater/floor, conservation), LEAD-STR-isogeny-class-generic-invariance (rho/IC level ratios), LEAD-TRF-prime-field-isogeny-to-weak
  (closed by Tate). Those test yield and difficulty; this one states the transport as a lemma and tests the closure degree and the quotient dimension of P1 across a class.
lean_obligation: >
  The group-level lemma only: for a nonzero homomorphism between cyclic groups of the same prime order, phi is bijective and transports sum relations
  (abstract over a group hom; mathlib has no isogenies, so nothing about curves is needed). About 50 lines. The statement that the profile is
  instance-independent is empirical, not a Lean obligation.
status: lemma derived; measurement planned
next_steps:
  - "1. Frontier model: write the transport lemma at group level (Lean, 1 day)."
  - "2. Director card: 20-member class at p = 12289, k = 7, 8, D = 4, 5; 1 day to freeze."
  - "3. Executor: run locally, under 2 hours."
  - "4. Coordinator: close 0.2's fall-degree question on a pass, or open a representative-selection sub-lead on a fail."
  - "5. (Optional) repeat at p = 40961, k = 9, 10 for D = 5, 6 columns <= 21,700."
```

---

## Lead P5

```yaml
id: LEAD-P-a12-degree-d-reach-and-quotient
title: A12 (height box, degree-d maps): the small-root route loses by a factor d in reach, and the F4 route inherits P1's quotient bound; one desk table and one small-k test
cell: >
  A12 (height box, degree-d maps). Assumption: the base is {P : x(P) = h(y), y in a box or a Kummer coset}, h of degree d; "height box" is the box |a|,|b| <= H for x = a/b.
obstruction_attacked: >
  A12 is OPEN; A10 (fixed-degree map images) and A5/A6 (Dickson, affine bit-box on P-256) are REF. What separates A12 from them is not recorded in anything I can read. The two
  routes that could solve a degree-d image system are (i) lattice small-root (Coppersmith-type) and (ii) closure-type Groebner. Both can be priced now.
claim: >
  (a, derived) For the bivariate polynomial S_3(h(y1), h(y2), t) with deg h = d, the Newton polytope is the square [0, 2d]^2 of area 4d^2; the strategy-independent reach for a convex
  monomial set is (n! vol)^(-1/n) = (2*4d^2)^(-1/2) = 0.354/d (0.354 at d = 1 matches the committed `LEAD-ICP-coppersmith-smallroot-semaev` figure, 8^(-1/2)). The yield threshold
  for one expected two-point decomposition per target is delta = 1/2 (box side X = p^(1/2) is needed for X^2/p >= 1). So the margin is 0.354/d - 0.5 = -0.146 at d = 1 and worse as d grows:
  -0.32 at d = 2, -0.38 at d = 3. No degree-d image base is reachable by the small-root route.
  (b, test) For d = 2, 3 maps h (monic, small coefficients) and y in a Kummer coset, the penultimate quotient of the closure at k = 6..9 satisfies q/B in [2.5, 6.0] like d = 1, i.e. the
  length-Theta(B) obstruction is independent of the map degree.
mechanism: >
  The reach of every lattice route is controlled by the Newton polytope, which grows with d; the yield threshold does not move. For the closure route, the quotient at LFD-1 is a statement about the
  Semaev curve with one coordinate in the base (P1), and composing with h changes the base but not the curve.
prediction:
  a: "the table of (d, reach, margin): (1, 0.354, -0.146), (2, 0.177, -0.323), (3, 0.118, -0.382) (derived)"
  b: "q/B in [2.5, 6.0] for d = 2, 3 at the plateau cells; the LFD plateau shifts by an additive constant (weight of S_3(h, h) is 4 at d = 2)"
discriminating_test:
  a: "desk; arithmetic on the committed convex-reach formula; minutes"
  b: "reuse the P1 harness with generator g = S_3(h(y1), h(y2), t) (exponents up to 2d per variable) at k = 6..9, D up to 6; columns <= 4,048 at D = 4; minutes local. Requires only a different g in the existing closure kernel"
falsification: >
  (a) is arithmetic; it is killed only by a non-convex or joint multi-block lattice, which the committed record names as the one escape (`LEAD-ICP-coppersmith-smallroot-semaev`, reopen conditions). (b) is killed by a plateau cell with q/B outside [2.5, 6.0] at d = 2 or 3: then the degree-d image base changes the quotient law and is worth a larger run.
cost_band:
  arms:
    - "small-root route: lattice dimension grows as the number of monomials, at least (2d+1)^2 per multiplicity level; no ECDLP cost is claimed because the route does not reach the yield threshold"
    - "closure route: as P1 (columns, mult-adds), reference arms enumeration and matched rho"
    - "section 5a pricing at w = 0, 0.1, 1 for the closure arm's spilled/unpriced work; enumeration and rho as P1"
controls:
  - "d = 1 reproduces P1 (q/B in the committed range)"
  - "negative: h = identity composed with a random weight-2 g: no law expected"
risks:
  - "Mostly a closure of A12 on paper; (b) tests only the presentation of the same chain."
  - "The Dickson (A5) and degree-d (A10) cells already say degree-d images tie the plain base; this lead adds the quotient statement, not a new base."
closest_baseline: >
  Published: Coppersmith / Jochemsz-May small roots; PKM (L of large degree); Semaev. In repo: `LEAD-ICP-coppersmith-smallroot-semaev` (reach 0.329 for 3 unknowns, 0.354 for 2; S_4/S_5 caps 1/21.8 and 1/70.8, from the 2026-11-07 assessment), `LEAD-ICP-algebraic-membership-factor-base` (rational-map images within 0.0003 bits of a size-matched random base at m = 2).
nearest_existing_lead_and_difference: >
  LEAD-ICP-coppersmith-smallroot-semaev (degree 1 only, convex monomial set) and LEAD-ICP-algebraic-membership-factor-base (yield, not solving). This lead extends the reach computation to degree-d maps and adds the closure quotient test.
lean_obligation: >
  (a) arithmetic only: the rational inequalities 0.354/d < 1/2 for d >= 1 closed by `norm_num` (reach as the committed formula, cited external); no Lean for Coppersmith itself.
  (b) none.
status: derived (a); conjecture (b)
next_steps:
  - "1. Idea Generator (Sonnet): the table (a) with the exact Newton-polytope volumes from the real S_3(h, h) supports; half a day, desk."
  - "2. Director card for (b) after P1 stage 0/1 pass; 1 day."
  - "3. Executor: run (b) locally; 2 hours."
  - "4. Coordinator: mark A12 closed on both routes, or open the degree-d quotient sub-lead if (b) fails."
```

---

## Ranking

1. **P1 (quotient law).** It converts the degree-growth obstruction into a size statement that does not depend on the unknown slope, it is checkable from numbers already on disk in half an hour, and it is the only item here whose confirmation or failure changes how cell 0.1 is triaged.
2. **P2 (free-extension lift to k = 18).** It is the only route to data past k = 15, it exploits the exact degree law, and its stage 3 is the one measurement that can reopen 0.1 on slope, but it is the most expensive (GPU hours, memory risk) and its likely outcome (plateau confirmed) is low-information.
3. **P3 (coarse radix).** A new presentation axis that is cheap at small k and can give a real matrix-size gain, with a derived proof obligation that bounds how far it can go; the expected ECDLP outcome is a null.
4. **P4 (transport is the plain base).** A one-lemma, one-hour closure of the fall-degree side of cell 0.2; I rank it below P3 only because the expected outcome is invariance.
5. **P5 (A12 degree d).** A paper closure of the lattice route plus one small closure test; it adds the least new information because the reach bound is already committed for d = 1.

None of these is predicted to beat matched rho. P1 and P3 say why, in numbers. The one measured improvement they can produce is an engineering one (smaller matrix, a cheaper pipeline to deeper k), and that is stage-gated, not claimed. [pcc:unverified]

## Considered and not proposed (with the reason)

- **Hilbert early stop on the prime Kummer chain.** The Nagao arm's stop works because the system is a complete intersection with a known Hilbert function. The Kummer chain's ideal is not (the quotient at LFD-1 is about 3-6 B, not the complete-intersection value), and P2 already uses the instance-independent profile as the analogue. A separate stop would repeat P2.
- **A dual (Macaulay inverse-system) closure that stores only the quotient of dimension about 5B instead of the 449k-row primal at k = 14.** The measured primal exponent B^2.24 sits below the naive dual q^3 = B^3 and near q^2.37 = B^2.37, so the saving is constants at best; my literature search found no dual-versus-primal cost comparison. If P1 holds, it is the engineering follow-up, not a lead of its own.
- **Target-adapted translation of the base (x -> x - c with c = t, which makes the chain's target abscissa 0, the exceptional locus where the N8 proof does not apply and the ideal has three weight-2 elements from k = 4).** Killed by the factor base: c depends on the relation's target, so the base differs between relations and no shared logarithms exist. Only one fixed target (a single decomposition) can be adapted.
- **Steering relations to exceptional targets (order two, tangent 3t^2 + a = 0, C1, C2) through a walk in the isogeny class.** Each exceptional class is a codimension-one locus, probability about 1/p per (curve, target) pair, so about p candidates per hit (derived). Cost is above rho.
- **Mixed or additive bases (arithmetic progression, interval) in the chain presentation.** A9 and A4 are REF; the digit presentation's d_ff is curve-independent (`LEAD-ICP-digit-presentation-solving-degree`); conservation (KN-FIND-007) fixes the yield.
- **Lattice or sieve test on a height box.** The reach bound (0.354) is below the yield threshold (1/2) at m = 2, and the linearisation lattice has H^36/p vectors against H^4/p true solutions (derived: 9 monomials of bidegree (2,2), a quartic form in four unknowns). Folded into P5.
- **Multi-target ic5 with k = 5 or 6.** At k = 4 the factor-base, relation and linear-algebra phases are 0.08% to 2.5% of cost (`mt_fieldops_v3_20261111/RESULTS.md`), so k = 5 has at most 2.5% headroom. The open item there is the retention or distinguished-point rho under a frozen rule, which is that study's own named next step.
- **Large-prime or double-large-prime variants on prime fields.** The existing residual collectors are a birthday search with rho's exponent (slope -0.025 [-0.034, -0.017], the same as rho/BSGS/kangaroo); nearest ids `LEAD-ICP-residual-partial-relations`, `LEAD-ICB-large-prime-variants`, `LEAD-ICE-relation-graph-rank-large-prime`. No new mechanism.
- **Graeffe-resultant decomposition test (Res_{x2}(S_3, x2^N - c) by repeated squaring, then gcd with x1^N - c).** It costs O~(N) = O~(B), the enumeration cost; cell 0.1 gains nothing. Kept as the second named non-closure alternative inside P1's risks.
- **Lucas V-doubling membership test for x^N = 1 (k squarings instead of a 224-bit power).** A real constant factor over a generic power test (about 3k against about 350 multiplications), but a table lookup is cheaper at toy k and the whole route is enumeration; no cost class changes. Recorded only because it prices the enumeration baseline in P1.
- **Deployed-curve statements.** None of the above concerns a deployed curve or key recovery; all instances are toy-size unit or random curves, as in the committed experiments.

## Process notes

- Everything above was derived by hand from committed numbers; the executing agent must recompute the binomial sums and the q values before relying on them (stage 0 of P1 is exactly that). [pcc:unverified]
- Gap to matched rho is unchanged by every proposal (distance remaining as quoted in the sources: res2 collector 2.6-2.7x at 36 bits; the IC rows 5-100x). No relabelling or accounting change is offered as an improvement.
- Implementation language: every test names a native route (Rust or the existing C kernel behind a native driver). The existing closure engines in ml-cryptanalysis (`csd/ic_bounds_big.py`, `scripts/ic_bounds_*.py`) are Python plus a C kernel; a native driver is part of each test's cost.
- Experiment routing: P1 stage 0-1, P3, P4 and P5 are local; P2 and P1 stage 2 need the owner's rented-GPU route, because the default for authorised x86 CPU and benchmark work is isolab-eks (CLAUDE.md, AGENTS.md 9a). I do not authorise any run.
- A stray scratch file `out/fix_p3.txt` was created by mistake (the Edit tool is unavailable here, so I rewrote the whole record instead of patching it); it holds no content and can be deleted.

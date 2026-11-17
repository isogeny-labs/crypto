# Structure family: coverage digest for the IC treatise expansion

Survey date 2026-11-06. Read-only, from `origin/main` of crypto (C) and
crypto-autoresearcher (CA), plus `HEAD` of cryptanalysis (X). Treatise sources
are `C:papers/ecdlp-index-calculus-treatise/sections/*.tex` (about 40k words).

**Standing.** Unless a line says otherwise, every number below is an
**unverified assertion** copied from the producer's own README/REPORT. I found
no pcc claim id on any of them except where one is quoted (PCC-2026110x-...,
all standing `unverified`). Several READMEs say "validation gate: not
admitted" or "not a leaderboard row". No row in this family is on the IC
leaderboard as a validated row.

Coverage labels: COVERED (section), PARTIAL (what is missing), ABSENT.
"IC-relevant" means the work changes, prices or bounds an index-calculus cost,
or supplies the matched rho an IC row needs. "ECC-general" means the work is
about curves, provenance or scalar multiplication, with no IC content.

Overlap warning: lines 5, 6, 13 and 23 also touch the transfer/isogeny/cover
family. Line 10 also touches the extension-field family. Line 21 also touches
the prime-field family. Coordinate with those digests so nothing is counted twice.

---

## 1. Automorphism-quotient rho, and the strongest rho on special-j curves

- **Tried:** rho on classes under Aut(E): negation, `rho.cubic` (j=0, |Aut|=6),
  `rho.quartic` (j=1728, |Aut|=4). The question was whether any walk beats
  sqrt(|Aut|). Also an order-24 class walk on the j=0 line over F_{p^4} (E29,
  E34, E41).
- **Numbers:**
  - CA lead atlas: the j=0 crater `rho.cubic/rho.negation` is 0.573 [0.562, 0.585]
    against a theory value of 0.577 (26-bit volcano census). EV-ENDO-10109d
    puts the ratio to the ceiling at 1.0043 (j=0) and 1.0442 (j=1728), over
    20 seeds at p=100057.
  - `C:research/prime_rho_automorphism_20261015/` (15 Oct, **erratum 24 Oct**):
    the class set-up (16–58 GAE per solve) had been left out of each total.
    After the correction, cubic/negation is 0.647 [0.543, 0.770] (it was 0.611)
    and quartic/negation is 0.819 [0.722, 0.929] (it was 0.791). The quartic
    interval no longer contains the predicted 0.707.
  - `C:research/drastic_prime_conductor_gap_round2_20261012/`, D=-3,
    ℓ=1016839 and 2033609: best-method floor/crater is 1.69167 (99%
    1.21689–2.36245) and 1.72972 (1.24690–2.46132), against a predicted
    sqrt 3 = 1.7321. This is the first time the frozen repeated-lead criterion
    was met in the conductor-gap programme. Same-method control: 0.947 and 1.060.
  - E29 (`C:research/e29_twists_20261030/`): ψ_p has order 24 on all 8 of
    E18's curves (p=241…1069). Stage R order-6/order-24 steps are 1.701
    pooled. The F_p cost ratio is 0.962 pooled (schoolbook) and 0.884
    (Karatsuba), so the order-24 walk as built is not cheaper.
  - E34 (`e34_twist24_cheap_rep_20261031`): a cheap order-24 representative.
    0 mismatches over 39,936 class members, and 768/768 walk identities hold.
    The cheap order-24 walk becomes the matched rho on E18's j=0 curves at
    every band weight.
  - E41 (`e41_twist24_shortfall_20261102`): the 1.701 against the ideal 2
    is sampling noise (order-6 arm at 0.924±0.046 of ideal, order-24 arm at
    1.086±0.045), not a defect of the walk.
- **Verdict:** the sqrt(|Aut|) constant is reproduced, and nothing exceeds it
  (CA `LEAD-STR-automorphism-quotient-rho`: closed_known). The order-24 walk is
  the correct matched rho on those curves.
- **Treatise:** PARTIAL. The background states the sqrt(|Aut|) quotient, and
  `prog_transfer` cites round 3 (1.86–1.92). Missing: the class-setup erratum
  (0.647/0.819), the round-2 repeated-lead result, the full-End orbit argument
  (below), and the order-24 matched-rho work E29/E34/E41.
- **IC relevance:** high. It sets the matched-rho reference for every IC row
  on j=0 or j=1728 curves.

## 2. Endomorphisms act as scalars: the generic obstruction and the orbit-size bound

- **Tried:** endomorphism-image oracles, witness lattices augmented by
  eigenvalues, full-End(E) quotients, and small-|D_0| non-unit endomorphisms.
- **Numbers and theorems:**
  - For G of prime order N, End(G)=Z/N, so every endomorphism oracle can be
    simulated with group operations and Shoup's bound applies (CA
    KN-FIND-b7e091, EV-GGM-79e710, H-ENDO-001).
  - A non-unit α of norm n acting as λ has orbit size h = ord_N(λ), with
    n^h ≥ (sqrt N − 1)^2, so h ≥ log N / log n (IDEA-20260901-6b63a4, an
    elementary proof). The net factor sqrt(h)/κ is a **cost-model estimate,
    never measured at the equality case**. A label note of 2026-10-31 says
    so, and IDEA-20261031-6610ed designs the measurement.
  - EV-ENDO-001: the witness lattice W_r has first minimum exactly 1 on J0
    cells, by Vieta (1+λ_2+λ_3=0). The vector is k-independent and leaves the
    exponent unchanged.
  - Lead dispositions: `LEAD-STR-endo-scalar-action-generic` closed_obstructed;
    `LEAD-STR-small-cm-disc-nonunit-endo` open_next_step; P-192 class CM
    relations, 103/103 scalar in the L≤2^48 box (EV-SCURVE-8e5c70, the census
    is not provably complete).
  - Source: `CA:analysis/endomorphism-isogeny-decomposition/DECOMPOSITION.md`
    (T1–T5, lanes L1–L14).
- **Treatise:** PARTIAL. `obstructions` and `prog_transfer` say that "a loop
  is an endomorphism and acts as a known scalar". Missing: the orbit-size
  theorem, the full-End quotient argument, the lane decomposition with its
  reachability gate T5 (vectorisation), and the open non-unit-endomorphism lead.
- **IC relevance:** medium. It is an obstruction and it bounds IC folds; it
  does not measure a cost.

## 3. GLV/GLS and isogeny-chain endomorphisms for scalar multiplication

- **Tried:**
  - `C:research/cryptopro_b_glv_chain_20261005/`: GOST CryptoPro-B, D=-619,
    h=5. The cheapest endomorphism is 4+ω of norm 175=5·5·7, built as a chain
    of 5-, 5- and 7-isogenies. It gives GLV-2 with |k1|,|k2|<2^128.
  - `C:research/cryptopro_b_cm_map_checks_20261005/`: a degree-155 CM map,
    39 known-input cases and 16 additivity pairs.
  - `C:research/cp6_782_endomorphism_chain_20261005/`: D=-339, h=6, chain
    5·5·7, verified on points. No timing.
  - `C:research/endomorphism_search_20261005/`: the native decision tree
    reproduces 500 order summaries and 15 maps, and verifies 64,589 scalar
    results. Tooling only.
  - CA `research/endosweep_*`, 16 dirs from 5 to 23 Oct: a corpus sweep, FourQ
    verified, and the native chain on CryptoPro-B at 1.25× median and 1.34×
    min. The lead atlas also gives chain GLV-2 at 1.525/1.521/1.492× over
    width-w NAF (Bandersnatch, Tom-384, CP6-782).
  - EV-ENDO-118b3a: a 3.07% one-shot k·P gain from the neighbour a=-3 model on
    BADA55-VPR-224 (one curve; H-ENDO-5a33ae weakened).
  - `C:research/isogeny_loop_glv_20261010/`, `_20261011/` (uncited): erratum,
    the Verschiebung split is a provable loss. On random challenge curves the
    cheapest loop costs 20–93× the doublings it saves.
- **Verdict:** real speedups of scalar multiplication, outside ECDLP cost
  (`LEAD-STR-glv-gls-isogeny-loop-scalar-mult`).
- **Treatise:** COVERED in brief (`prog_prime` §Audits: 1.25–1.50×;
  `prog_transfer`: 1.09–1.15× in four dimensions). The 20–93× loop-cost
  number and the Verschiebung erratum are absent.
- **IC relevance:** none directly. They belong in one paragraph explaining
  why these endomorphisms do not form rho classes or IC folds.

## 4. Exact CM data and the endomorphism level of the curve catalogue (`curve_traits`, endosweep D_K)

- **Tried:**
  - `C:docs/curves/traits.json` and `TRAITS.md`: the CM field, descent, twist
    and End(E) level for 2,572 registry curves plus 176 source curves.
  - Successive closures of the open volcano levels:
    - `curve_traits_levels_20261022`: prime-power point tests, horizontal
      cycles for ℓ≤19, class-group orders. Open levels 312 → 32.
    - `curve_traits_tower_cost_20261022`: tower overhead recalibrated
      (2.8× at degree 320). 13 levels decided.
    - `curve_traits_level_certificates_20261023`: 19 stored point
      certificates and 2 binary curves settled by walks (GF(2^61), ℓ=31543:
      the crater; GF(2^47), ℓ=31: the floor). Open levels 34 → 13.
    - `curve_traits_descent_levels_20261028`: 17 new curves settled by
      descending walks, matched by an independent PARI implementation.
    - `curve_traits_factor_hints_20261028`: 18 primes added. CADO-NFS split a
      C112 in 25,084 CPU-seconds. 8 curves go from End unknown to probable,
      and the unfactored Δ fall from 14 to 6.
  - CA `endosweep_dk_exact_*` (batches 1–5, 16–23 Oct): exact D_K, f and the
    least non-scalar endomorphism degree for 34 deployed curves, plus gost512
    in batch 5. Four 4q−t² composites remain open.
  - All of these REPORTs are `.typ`/PDF with no pcc id.
- **Treatise:** ABSENT.
- **IC relevance:** low (infrastructure). It is the data behind "on deployed
  curves |D_K| is large", which IC chapters cite implicitly. It deserves a
  table in an appendix.

## 5. Conductor gaps: does End(E) depth change ECDLP cost?

- **Tried:** crater (End=O_K) against floor (Z+ℓO_K) at large prime ℓ, for
  rho, BSGS and IC.
  - `conductor_gap_cm_pairs_20261010` (uncited, 10 Oct): five ecbench
    sessions. An early lead of 1.22 [1.005, 1.46] at ℓ=8209 (4 targets,
    seed 1) was later closed. Its banner was corrected twice by sheriff
    (SHF-20261017-e4d7f6 and SHF-20261028-0f3f64: "no implemented method sees
    the gap" was withdrawn).
  - `conductor_gap_paired_20261011`: legacy Python, **provisional, not for
    merge**; no claim rests on it.
  - `isogeny_conductor_gap_ic_20261010` (uncited): fibre-aware relation
    generation (engineering), and IC over a large gap as a boundary control.
    Every IC arm sits above matched rho.
  - `drastic_prime_conductor_gap_20261012`: D=-19 and D=-43, ℓ ∈ {508433,
    1016839, 2033609}. Rho floor/crater ranges 0.53–1.29. One 95% lead (0.5286
    on d43-ℓ1016839) **regressed** to 0.946818 (99% 0.662–1.347) with 32
    fresh targets in round 2.
  - Round 2 (D=-3): the sqrt 3 automorphism effect, see line 1.
  - `conductor_gap_cm_orbits_20261015` (15 Oct): 100 pairs. Generic-rho
    post-hoc CPU ratio 1.17433 [0.99897, 1.38318]; charged group-work ratio
    1.15967 [0.98616, 1.36656]. Neither establishes a difference. A 96-bit
    floor is certified.
  - `conductor_gap_followups_20261016`: five follow-ups (crossover, yield
    distribution, orbit geometry, size trend, partition controls). The README
    shows them queued or running; 2,902 files.
  - `C:papers/conductor-gap-rho/` (from `conductor_gap_rho_large_sample_20261018`):
    - floor/crater 0.998 [0.985, 1.011] at ℓ=8209 (12,288 paired solves);
      1.005 [0.960, 1.052] at ℓ=131101; 0.982 [0.940, 1.028] at ℓ≈2^20;
    - positive control rho.plain/rho.negation 1.36–1.45; twin negative
      controls 1.004 [0.990, 1.019];
    - **easy-gap tier** (ℓ≈2^25, 53-bit): floors read 8% [3%, 13%] cheaper,
      but between re-coordinatised endpoints the gap measures 1.040 [0.965,
      1.119]. The shift belongs to the coordinate model under the
      representation hash, not to End(E);
    - **Lean theorem** `RepresentationHashInvariance.lean` (Lean 4.34.1, no
      Mathlib, axioms propext and Quot.sound): representation-hashing rho is
      conductor-blind in distribution.
  - `prime_conductor_gap60_20261031` (archival 60-bit, quotient open) and
    `prime_conductor_gap80_20261031` (80-bit source/kernel, a PARI
    certificate for a 202-bit prime): construction only.
  - `conductor_gap_replication_20261025`, `_crater_design_20261025` and
    `_size_ladder_20261026`: artefact stubs on main (`.artifacts.json` and PDF
    only); the content lives in S3.
  - CA:
    - `conductor_gap_panel_20261014`: 43 certified prime conductor directions
      across 20 classes (NIST K curves and controls);
    - `conductor_relation_yield_20261014`: a design only (P29 ℓ=8209 and P37
      ℓ=131101, B=256, 32,768 targets), not run;
    - `ecc2k130_conductor_isogenies_20261007`: f_π = 263·146505763881528721,
      all 262 descending 263-isogenies computed. The second prime's
      isogenies exist but cannot be written down.
  - `C:research/ecc2k130_endo_ring_263_20260925`: τ^131 acts as −I on E[263].
    The least non-scalar endomorphism degree on a floor leaf is 121,046.
- **IC results:** IC floor/crater 0.975 (99% [0.603, 1.451]) at ℓ=8209 and
  1.127 [0.922, 1.342] at ℓ=131101 (cited). The F4 two-torsion
  symmetrisation saving vanished at matched columns (cited).
- **Verdict:** no conductor effect for generic methods or IC. The only real
  level effect is Aut at the j=0 crater. Correction history: one sheriff
  banner and two leads regressed.
- **Treatise:** PARTIAL. `prog_transfer` covers ECC2K-130's gap 263, the
  ecbench IC pair, the large-sample rho 0.998 and round 3. Missing: the
  D=-19/-43 drastic round and its regression, the cm_orbits 100-pair panel,
  the easy-gap representation-hash artefact (a methodological lesson worth
  stating), the Lean invariance theorem, the 60/80-bit constructions, and the
  CA panel and yield design.
- **IC relevance:** medium-high. It is the main structural test of "does
  End(E) matter for IC" and answers no.

## 6. Isogeny-class invariance of generic cost: the isogeny-class-rho paper

- `C:papers/isogeny-class-rho/` and `research/isogeny-class-rho`,
  `isogeny-class-rho-n45` (uncited):
  - 85,425 solves on 200 curves in 4 classes;
  - historical fold ratios: j=0 0.439 [0.401, 0.481]; Koblitz Frobenius
    0.155 [0.141, 0.170], and 0.113 with negation.
- **Review 2026-10-19:** the inferential null is **provisional**. The review
  found a KS defect ([1,1] against itself gives D=1), missing tie corrections,
  and unassessed covariance from shared seeds. "No test fired" is false: there
  is one B1 generator flag, Kruskal–Wallis p=0.039, and P0/P1 source–target
  flags. The 1.7–2.2× wall ratio is from a shared VM and is not admitted.
- Lead atlas: 33/35 level ratios equivalent (TOST ±5%), v2.1 rho
  floor/crater 1.004 [0.990, 1.016]. Transport costs ≤1.3% of a solve.
- **Treatise:** PARTIAL. The volcano census (33/35) is cited in
  `prog_transfer`. The isogeny-class-rho paper and its review and correction
  are absent, and belong in `audit`.
- **IC relevance:** low-medium. It is a generic-cost invariance and the
  matched-rho basis for class-representative IC.

## 7. Class-representative selection for IC (the one open structural IC lever)

- CA `LEAD-STR-class-representative-ic-selection` (open_next_step):
  - ℓ=2 IC yield floor/crater 0.942 [0.928, 0.956]; S floor/crater 1.045
    [1.023, 1.067] at 24 bits;
  - IC over matched rho 17–170× remaining (v2.1);
  - IC/matched rho 42.4× (V1) and 61.5× (V2) at 0 GAE (volcano census);
  - model effect τ 7.2% > 1.9% horizontal;
  - "yield is a set property": a pushed crater base gives exactly the crater
    yield (`pair46_factor_bases_20261015/03-floor-pullback`).
- **Treatise:** COVERED (`prog_transfer` §Volcano levels: 5.8% fewer
  relations, 99% of the contrast explained by cofactor classes). Missing: the
  model-choice effect (τ 7.2%) and the open-lead framing.
- **IC relevance:** high (small).

## 8. Closed structural obstructions: transport tables, JMV, torsion leaks, level structure

- `LEAD-STR-isogeny-transport-preprocessing`: table transport against
  correctly costed rebuild loses 1.5× (ℓ=3) to 21× (ℓ=31) at p=50021 and
  60013, with no crossover (EV-ENDO-10109d N4).
- `LEAD-STR-jmv-random-self-reduction` (KN-FIND-f293c6): the cost model is
  vacuous at q=2^256 for δ≤0.5. The modelled crossover against sqrt n is at
  192 bits for δ=2.0.
- `LEAD-STR-torsion-module-leak`: any homomorphic invariant from G to an
  ℓ-primary group is trivial. Partial information about k costs more than it
  saves by sqrt(M). The ψ_5 factor-degree multiset is k-independent
  (H-TORS-3bed8a, toy p=751).
- `LEAD-STR-level-structure-torsor` (X_1(N) diamond torsor): open, a draft
  design only. Three ideas were rejected (IDEA-112, -023, -026).
- `LEAD-STR-class-group-ecdlp-bridge`: open; no bridge map has been built.
- `LEAD-STR-isogeny-walk-weak-neighbour`:
  - transfer weaknesses are class invariants;
  - the P-256 million-model walk had no screen hit;
  - a secp256k1 self-isogeny of prime degree 2^44497−1 was replayed
    (EV-ISOU-263837). It is a capability, not ECDLP.
- **Treatise:** PARTIAL. `obstructions` §Transport conservation and the
  xedni entry exist, and JMV is cited. The table-transport numbers, the
  torsion-leak obstruction, the level-structure lead and the class-group
  bridge are absent.
- **IC relevance:** medium. These answer "can structure feed the relation
  phase?"

## 9. Special j-invariants in Semaev and IC systems

- **Numbers:**
  - S_3 support is 9 at j=0, 10 at j=1728 and 13 generically (EV-ENDO-e8d3e5,
    "the only surviving positive of BATCH-cb71b5, a structural fact not a
    cost");
  - a j=0 order-6 GLV base gives 3× signed points at +4% Ir, with no speedup
    (`C:benchmarks/20261017-perf-history-factor-bases`);
  - H-JINV-5a0b9e is blocked;
  - the matched-order N=19507 design (59 classes) is unrun;
  - quartic-twist polynomial IC is open (`C:problems/quartic-twist-polynomial-ic-pricing.md`).
- **Treatise:** PARTIAL. `prog_frontier` around line 618 mentions a prime CM
  base (j=0, 1728, D=-7, -8). The support counts and the matched-order design
  are absent.
- **IC relevance:** high (the lead is still open).

## 10. Endomorphism/automorphism-invariant factor bases over F_{p^k} (the GLV-invariant programme, E1–E41)

- **Sources:** `C:research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
  (§§1–9.50, 5,000+ lines), `research/glv_k4_pipeline_d4_20261017/`,
  `glv_gls_v4_20261018/`, the E29/E34/E41 dirs and `experiments/NN_*`.
- **Key rows (all producer-reported):**
  - **E19**: first end-to-end k=4 IC, 2.6–8.8·10^6× from rho.
  - **E22**: the V4 line exists; its solve is ≈820× cheaper and its yield
    ≈10× worse.
  - **E24**: on curves over F_{p^4} with full rational 2-torsion, a
    V4-stable base line plus S_5 written in V4^4_0⋊S4 invariants (weights
    (2,2,1,1), quotient 64, Macaulay degree 11) take one decomposition from
    1.41·10^9 to 1.66·10^6 F_p mults. End to end over 12 curves (p=101–521)
    it is **35–240× cheaper than the D4 baseline**. The modelled handover
    moves from 2^116.7 to 2^90.1. The exponent is unchanged
    (S/rho ∝ n^{−1/4}): engineering, a constant.
  - **E25**: the GLS ψ fold on the V4 line cuts k=4 IC by 1.35–2.67×
    (median 1.86×) over 10 curves at p=101–397. Against the matched ⟨±ψ⟩
    walk the ψ arm gains 1.26–1.97× at p≥197. Handover 2^93.2 → 2^88.0.
  - **E26**: no GLS curve carries the V4 line with h=4.
  - **E27**: the matched walk was 1.45× too expensive, so every row moves
    1.45× further from rho (a correction).
  - **E30**: trivial products are 0.31% of S (accounting).
  - **E31**: the P_2-chain solve cuts the k=4 solve 2.6× (A) and 5.1× (B),
    and S by 2.5× and 4.7×; validated with errata and a §5a qualification.
  - **E32**: distance to rho over 2^22.6–2^42.4 falls as n^{−0.24}, and no
    crossover is reached.
  - **E33/E35–E39**: pivot rules. Arrival-order pivots F cut A's per-call C
    1.47–1.55×. The E38 kill fired at the pilot.
  - **E29**: the order-24 base fold, with modelled handover 2^135.0–2^142.2
    against 2^145.7–2^151.9.
  - **§9.19 and §9.33**: errata found by the validation gate (§9.6, §9.14,
    §9.16).
- **Treatise:** PARTIAL. `extension` covers only FGHR's D4 (Bézout 512,
  47.5–48.4×, handover 2^135.1–2^135.8), the char-2 k=4 ladder, and one line
  "moves from 2^150 to about 2^103–2^119 depending on the frame". The V4 line,
  the GLS fold, the chain solve, the pivot programme, the matched-walk
  correction E27, E32's n^{−0.24} and the order-24 fold are absent.
- **IC relevance:** very high. This is where endomorphism structure actually
  moves an IC constant. Confirm with the extension-field agent which of you
  owns it.

## 11. Strongest rho references on extension-field curves (`ext_endo_rho_20261023`)

- Frozen protocol (23 Oct): `rho.frobenius_orbit` on E/F_p seen on E(F_{p^3})
  (A=6), and `rho.gls` on GLS twists over F_{p^2} (A=4).
- Band tables (`analysis/*.band.md`): on six fp14–fp18 k=2 curves, rho-gls S
  is 0.554–0.921 at w=0 against rho-neg's 0.904–1.165. At w=1 the order
  reverses: rho-gls 2.70–4.20 against rho-neg 1.74–2.31, because the
  canonicalisation work is unpriced. Rho-plain is 1.49–1.66.
- There is no README or verdict file on main, and no pcc id.
- **Treatise:** ABSENT.
- **IC relevance:** high. It decides which rho any F_{p^2}/F_{p^3} IC row must
  be matched against, and it shows the §5a band reversing the order.

## 12. Frobenius-orbit IC against rho-frob on Koblitz curves

- `C:research/ecbench_frobenius_orbit_vs_rho_20261014/`: fully priced, IC is
  not below rho-frob on any of 8 Koblitz curves. The slope is β = 0.165
  [0.117, 0.213] per ln r, against a model value of 1/6. The orbit buys up to
  15× against the subspace base at n=23.
- `frobenius_quotient_followup_20260926`: SAT numbers supplied as a note and
  not reproduced (raw logs absent).
- `quintic_cached_frobenius_20261017_v1`: a derivation with a GF4
  counterexample (the map is Frobenius-semilinear, not K-linear). Not executed.
- `frobenius_msweep_20261027`: artefact stub with a PDF only.
- **Treatise:** COVERED (`distance`, `obstructions`, `prog_frontier`: r^0.165).
  The quintic derivation and the unreproduced SAT note are absent. This is
  the Koblitz family's territory.
- **IC relevance:** high (covered).

## 13. Weak-curve atlas: catalogue, database, properties, conjectures, Lean (ISO-1 and cover-weak classes)

- **ISO-1** (`iso1_weak_classes_20261007`): a **theorem**. Every curve in
  the norm-one weak family over an odd extension of F_q, q≡1 (mod 4), has a
  Legendre twist with a 2-isogenous neighbour carrying full rational
  4-torsion, so t ≡ ±(q^n+1) (mod 16) and v2(f_π) ≥ 2. The p=7 census shows
  the condition is insufficient: the class t=±610 has conductor 72, depth 3
  and no weak representative. The p=53 census finds 72,540 weak rows among
  146,068 ordinary rows, none of depth 1, and 494 high-depth zero rows; 5,000
  GP controls agree. The census cost is (p^4+3p^2+8)/12 point counts.
- `weak_curve_catalog_20261010`: 21 family/condition entries and 814 exact
  models.
- `weak_curve_properties_20261010`: 814 models agree between native code and
  PARI; 360 ordinary 2-volcano levels certified and 454 bounded; a torsion
  proposition (one rational 2-point means the floor).
- `weak_curve_properties_lean_20261011`: 48 public theorems and 71 audited
  declarations. The conductor proposition is only partially formalised (its
  CM bridges are hypotheses).
- `weak_curve_database_20261012`: SQLite with 339,740 class observations
  (87,486 exact two-branch rows at p≤37 and 252,254 cubic norm-one diagnostic
  rows at p=41–47, never pooled). 537 large models have unmeasured
  selected-subgroup advantage.
- `weak_curve_case_index_20261012`: the 36 Iijima–Momose–Chao 2015 cases as
  metadata.
- `weak_curve_conjectures_20261012`: 7 open hypotheses. One exploratory
  boundary hypothesis is refuted by 87,486 rows.
- `weak_isogeny_classes_20261013`:
  - the GHS class closure exceeds the direct weak set by 1.8× to 1543×;
  - the magic-number function was **corrected** to the MMT definition
    (`docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md`);
  - Smith's 18.57% is reproduced (0.185738).
- `weak_line_7_4_followup_20261013`: explains the (7,4) weak line as an
  affine symmetry of g.
- `weak_subfield_hyperelliptic_decomposition_20261012`: the Bhati–Singh weak
  curves are subfield curves. The first run's records were duplicated and the
  corrected run is reported.
- `jv_weak_generalization_20261008`: genus g ∈ {1,3,7,9,13} for 2-torsion
  covers. The q=37 run was stopped by its operator and the deviation is
  recorded.
- `ic_cover_weakclass_20261023`: the Hilbert early stop in Joux–Vitse cover
  IC cuts counted S 1.848–1.875× at equal relations, 25/25 cells
  (engineering). The as-found B0 is a **relabelling**: S fell 2.53–2.64× while
  f4_native_ops rose 8.3–8.5×. S/rho falls with p, slope −2.27±0.13.
- **Treatise:** PARTIAL. `prog_transfer` cites the weak-curve leaderboard
  (922 models, 36 cases) and the JV weak-class law (depth-1 classes hold no
  weak curve over 4,871,062). The ISO-1 theorem, the catalogue, database and
  Lean properties, the GHS class-closure ratios with the magic-number
  correction, and the Hilbert-stop engineering/relabelling pair are absent.
- **IC relevance:** medium-high (cover-based IC on extension fields). It
  overlaps the transfer digest.

## 14. NIST/X9.62 seed provenance and the skip rule

- **C:**
  - `nist_seed_provenance_20261016` and `nist_basepoint_provenance_20261017`
    (cited): seeds regenerated as 1997–98 drand48 output.
  - `nist_seed_skip_filter_20261017`: 21 accepted candidates, 10 published.
    21 new preregistered features (H7–H11) separate nothing (0/21), after
    about 60 earlier features were ruled out.
  - `nist_skip_criterion_20261025` and its REPORT: an exact two-test
    empirical separator, *M = 2p+2−n mod 8 ∈ {5,7} and (b+1) mod 19 ≤ 13*,
    accepts 10/10 published and rejects 11/11 skipped. It is
    root-convention dependent (with the smaller P-224 root it rejects the
    published model), so it is not an intrinsic class property. Calibration
    shows such fits are common.
  - Papers: `papers/nist-strict-splitting-20261029` (calibration 23/4,480),
    `nist-expanded-cutoff-20261030` (only 691–718 and 769–772 separate all
    four paired fields; the 60,000-curve baseline is unrun),
    `nist-broader-census-20261030` (69 positions, 82 branches) and
    `nist-exact-pair-audit-20261030` (2U=117 tail).
  - `nist_skipped_curve_dissection_20261021`: 8 of 9 integers factored. A
    444-bit P-256 cofactor remains and the R05 sieve crashed (exit 139).
  - `nist_binary_skipped_20261020`: B- and K-curve census.
  - `b_weak_class_battery_20261022`: 85 tests, Bonferroni threshold
    5.88·10^-4. No flag; minimum p is 0.034. 74/85 positive controls pass and
    11 cells are inconclusive.
  - `nist_cyclotomic_point_heights_20261011`, `nist_trapdoor_slot_program_20261011`
    (round 0 only) and `nist_cyclotomic_controls_20261011` (cited, claimed
    n−1 outlier not established).
  - `nist_gpu_arithmetic_20260926`: an infrastructure non-result.
  - `nist_best2_small_batch_20261027`: BEST2 stays disabled. This is rho
    engineering, not provenance.
- **CA:** about 55 `research/nist_seed_*` dirs (16–31 Oct): 1996–97 drafts,
  ECPP, SEA, exact conductor and embedding degree, class number, invalid
  curve surface. The lead atlas concludes: about 30 bits of freedom per
  curve; no screen separates the 21 classes; "Elkies primes ≤691" does not
  replicate on 42 held-out curves.
- **Verdict:** exhausted_at_scale. There is no ECDLP bearing, and every
  README says so.
- **Treatise:** PARTIAL (`prog_prime` §Audits, one paragraph).
- **IC relevance:** none. It should stay a short section or appendix: "the
  structure audits found nothing an IC could use".

## 15. Standard-curve structural audits: SafeCurves, secp256k1 CM, P-192, structural completeness

- `C:papers/structural-completeness/` (about 15 pages):
  - a taxonomy of seven attack families, blocks B1–B7, two novel proposals
    (CGA-HNC and VFCG-ρ, both null), the Howe-gluing existence fact (with an
    errata on the naive construction), and an empirical PASS on secp256k1,
    P-256, brainpoolP256r1 and Curve25519;
  - **correction 2026-10-17** (SHF-20261017-e4d7f6 P-N4–N6): "proven" was
    changed to "argued by case analysis (proof sketches, not independently
    reviewed)", and the counts were fixed (26,278 instances = 336 distinct
    checks).
- `C:secp256k1_cm_audit/` with CA `LEAD-STR-secp256k1-cm-class-audit`: D=-3
  and h=1, so secp256k1 is the crater vertex. The sextic twists are not
  isogenous, and the only lever is the sqrt 6 constant.
- `LEAD-STR-safecurves-parameter-audit`: closed_known. It covers Ed448
  base-point certificates and P-192 x-only singular-companion recovery (an
  implementation weakness).
- Cited already: `safecurves_trapdoor_slots_20261017` (secp256k1 twist rho
  2^65.5), `p192-weak-curve-20261006`, `p224_attack`, `sect113r1`.
- **Treatise:** PARTIAL. The cited audits are in `prog_prime`. The
  structural-completeness paper, its correction and the secp256k1 crater
  argument are absent.
- **IC relevance:** low. The structural-completeness theorem is a useful
  framing for "which structural routes remain for IC".

## 16. LLL degeneracy note: retraction

- `C:papers/lll-degeneracy-note/`: the "secp256k1 fails LLL at 256 bits" and
  "uniform failure at ≥384 bits" findings were f64 Gram–Schmidt overflow
  bugs, **withdrawn** 2026-10-16 (SHF-20261016-3caa6f). On 2026-10-17 a
  second correction withdrew the P-521 mechanism sentence (SHF-20261017-e4d7f6).
  After the fix, 3/3 distinct seed pairs recover the key. P-521 recovers 3/3
  only with 2048-bit fixed-point Gram–Schmidt.
- **Treatise:** ABSENT.
- **IC relevance:** none (lattice/HNP). It is worth one line in `audit` as a
  "curve property that was a numerical bug" pattern.

## 17. Lean formalisations of ECDLP and structure facts

- CA `research/lean_retrospective_20261018`: 98 Lean source paths in 19 Lake
  projects; 5 mapped claims machine-checked and independently reviewed.
  `Transport.lean` (`curve_dlog_transport_iff` on Mathlib
  `WeierstrassCurve.Affine.Point`, conditional) and factor-base range
  containment (`closure_le_range`).
- CA `research/lean_foundations_20261019`: 498 → 539 audited declarations.
  NCP reachability; rho-collision and coprime-transport in
  `aburan28/crypto-lean`. New review pending.
- CA `research/endosweep_lean_review_20261016`: 49 endosweep targets reviewed
  for semantic fidelity by three reviewers, with fixes.
- C `weak_curve_properties_lean_20261011`: 48/71 (line 13).
- C `papers/conductor-gap-rho`: representation-hash invariance (line 5).
- C `papers/ic-negative-results`: 24 theorems, 0 trusted (lean-proof-gate
  run 41390917050, 2026-11-02). S2 counting core and S3 yield conservation.
  S4 monodromy (Z/2)^{m-2} is derived but unreviewed; S6 is not established.
- C `research/ecdlp_weakness_lean_20261104` (Lean/Mathlib 4.34.1):
  - `left_kernel_identity` and `exists_left_kernel`: with more relations than
    columns a left-kernel λ always exists, and failure density is exactly 1/q;
  - `HazardSeparation`: refutes `CLAIM-MARGINAL-IMPLIES-HAZARD` with a
    three-outcome counterexample, and re-proves E[min τ N] ≥ N/2 from
    marginals only (xedni waiting time);
  - `SolvingDegreeBound` for Petit–Quisquater Assumption 2.
- C `research/lean_attacks_20261102`, on Nagao 2015/984 fake degree over F_2:
  **A1 refuted** (outside, no function of d'_F bounds d_F; family
  W_k={X^kY, XY+X} has d'_F=2 and d_F=k+1). **A2 refuted** in the +1 form.
  **A3 survives** for three-generator systems.
- Already in the treatise: `FallDegreeIdentities.lean`,
  `GuardedModel.lean`, `LastFall.lean`, `S3Roots.lean`, the Lemma 4 note, and
  the 11 lift-dense theorems.
- **Treatise:** PARTIAL. Lean appears per result in `falldegree`,
  `prog_rootfinding` and `obstructions`. There is no consolidated account of
  what is formally proved (and with which axioms) against what is only
  argued, and A1–A3, the left-kernel and hazard theorems, and the transport
  theorem are absent.
- **IC relevance:** high for the IC-specific ones (A1–A3, left kernel, hazard,
  yield conservation, transport). Low for the catalogue proofs.

## 18. The IC hardness-assumption chain and its weak points (2–4 Nov)

- `C:research/ic_weak_assumptions_20261102/`: IC security is a **serial chain
  of six obligations**: geometry, solver degree and matrix growth, yield and
  variance, rank, non-generic interface, then complete cost and exponent.
  - The key counterexample to "degree certifies cost": in PKM m=4 rows,
    N=8,12,16 have F4 solving degrees 6,7,7, but the maximum columns go
    1,049 → 9,452 → 76,837 and the multiply-adds 4.84e7 → 8.55e10 → 2.70e13,
    a 558,304-fold growth from N=8 to N=16.
  - The follow-up adds a D.4 tensor admission audit and a unit-explanation
    proof kernel.
- `C:research/ic_weakpoints_20261102/` (exact counts; Gaudry/Diem descent
  over F_{p^n}, n∈{2,3}):
  - n=3 solving degree is 10 in 88/88 systems, flat in p from 2^5 to 2^30;
  - the quotient dimension is 64 = Bézout;
  - 32 rational solutions are found, of which 5 are genuine and 27 spurious
    (84%);
  - yield mean/λ_pred is 0.88–1.19; Poisson P(0) matches;
  - 15/15 complete solves give a correct log, with a rank shortfall of 0–6;
  - IC/rho is 0.29–3.4 at w=0, 2.8·10^4–3.2·10^5 at w=0.1, and
    1.5·10^5–1.7·10^6 at w=1. Rho is ahead on 7 curves, 8 are unresolved, and
    IC is ahead on none;
  - not ecbench, not admitted.
- `C:research/ecdlp_weakness_census_20261104/` ([pcc:unverified]):
  - `pq2_rank`, 119 instances at n=8–47: literal PQ Assumption 2 (degree 2)
    fails on every instance. The iterated degree is exactly 4 at n=17–24. The
    Kousidis–Wiemers value 3 is never reached at n=17–44;
  - `pf_yield`, 18 curves: once the pairing convention is fixed the zero
    class matches Poisson, but the variance/mean is about 1.45 for
    negation-closed bases (U(R)=U(−R));
  - also `rank_deficit`, `trace_pullback` and `ext_decomp` (1/n!).
- `C:docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md` (3 Nov): 583 claims,
  950 edges, 236 sources. It finds 17 status inflations, 16 route closures
  resting on weak support, and 246 ranked attackable claims.
- **Treatise:** ABSENT. The fall-degree chapter covers Semaev's Assumption 1
  and Nagao Lemma 4, but not this assumption graph, the six-obligation chain,
  or the November censuses.
- **IC relevance:** very high. It is the natural spine for a "what IC
  hardness rests on" chapter.

## 19. Theorems and audits about summation polynomials

- `semaev_t2_trace_20261102`: Semaev §4.5's "first fall degree always 4" is
  **refuted**; it is 2 on 200/200 instances. The solver needs degree 2 on 154
  and degree 3 on 46, never 4. COVERED in `falldegree` (Corollary "first fall
  degree 2", Corollary "Assumption 1 fails"). The 154/46 split may be missing.
- `papers/ic-negative-results` S1–S6: COVERED in `cover` (yield conservation,
  monodromy) and `falldegree` (Nagao).
- `semaev_alternatives_audit_20261104`: an audit of a 2026-11-04 program
  document.
  - Its gate "e<1 at m=4" is not implied by its own model. A generic 2+2 MITM
    passes it (measured s=1.93, e=0.4 at B=p^{1/5}) while costing p^{2/3}.
    The corrected gate is **s<m−3, i.e. e<(m−3)/4**.
  - Idea 10 is dead (1 squarefree collision for all B≤2^16). Ideas 2 and 3
    are m=2 objects. Eight patches P1–P8 were made.
  - Coverage: ABSENT.
- `summation_polynomials_20260914`: specialisation engineering on
  SymmetrisedS4 (six variants frozen; an accounting correction). ABSENT,
  minor.
- `curve25519_edwards_semaev_20261102`: S_3 in the Edwards coordinate at
  m=2, k=20. The per-relation ratio follows coverage_exact. No DLP, no rho.
  ABSENT.
- **IC relevance:** very high (theory of the decomposition step).

## 20. Twists and IC

- Covered: `twist_obstruction_20261026` (2B=0; 0 of 356,530,036 mixed sums)
  and `lift_dense_factor_base_20261027` (11 Lean theorems, +0.50 bits
  modelled), both in `obstructions` §Twist points.
- `twist_ecdlp_20261027` (to 40 bits):
  - E0 separation lemma: 0 violations in 8,254 + 34,498 classes;
  - E1 mixed census: 0 mismatches over 91,200 + 508,800 targets;
  - E2 joint E+E′ table: 1.000–1.062× separate runs, so the gain hypothesis
    is refuted;
  - E3 F_{q^2} eigen-decomposition: 53.3–2,812× more work per relation;
  - E4 K0↔K1: identical 773,404 relations at 2.0246× the solves;
  - E5 Kummer-line walk: inconclusive, with a statistics issue unresolved;
  - E6 genus-2 gluing: #Jac = #E·#E′ on 580/580;
  - G1: no K-mix row below 1;
  - E5 and G1 had unseeded randomness, since fixed.
  - Coverage: ABSENT.
- `twist_index_calculus_20261020` (prime field, T-69): twist abscissas add
  876 candidate pairs and no new width-two hits. The algebraic obstruction:
  the twist contribution is zero or rational 2-torsion. ABSENT.
- `quadratic_twist_evidence_20261021`, about 10k files on F13/F37, CM8 dual:
  - 19,440 verified recoveries with zero strongest-source joint gains (size
    two);
  - the size-three study is boundary-limited (0 recoveries through cap 7);
  - caps 9–12: 1,344 configurations, all verified;
  - 19/22 and 6/7 Lean receipts;
  - Coverage: ABSENT.
- `quadratic_twist_followup_20261105`, `quadratic_twist_chord_preflight_20261105`:
  proposals with PCC-20261105-1f629a, -585d85, -cfd582 and -7179e4, all
  unverified and unexecuted.
- `twist_relations_tr04_20261029`: frozen v3; the workflow failed its
  checkpoint gate and no data exists.
- `lift_dense_torsion_pairing_20261106` (6 Nov): checks the claim that lines
  L_T = b^{1/4}·F_q pair columns via the 2-torsion point. Title: "no lead line
  lies on it". Unverified.
- **Treatise:** PARTIAL (the obstruction is covered, the follow-ons are
  absent).
- **IC relevance:** high (all negatives, which matter for completeness).

## 21. Torsion and cofactor levers on factor bases (Curve25519 family, cofactor curves)

- `curve25519_torsion_leads_20261103` (E ≅ Z/8 × Z/r):
  - T1 joint frame/coset selection fails (1/20, gains 1.00–1.12);
  - T2 2-descent-filtered base: 0 odd-coset points; coverage per column² is
    1.89–2.02× plain (1.71 predicted); F4 cost per target is 2.1–5.1× higher;
  - T3 m=3 frame-selection gain holds on 13/15 (2.8–5.4× at t≤5).
- `curve25519_isogeny_seed_20261103`: PKM prime-isogeny tower seed selection.
  - Seed 1093 covers 528 against 172 (3.07×), and seed 0 sits at the 6th
    percentile. F4 cost is 1.75·10^8, 3.10× lower.
  - The band verdict favours the selected seed at every weight. The w=0
    interval is 0.885 [0.570, 1.373] (it includes 1), V2 failed, and T9
    replicated it on 16 targets.
  - The order against rho is unchanged.
- `ic_m2_torsion_n37_20261101`, `_n41`, `_ratio_n37` (binary a=0, n=37,
  cofactor 596): torsion anchors make the solve about **7.5× more
  expensive** (2.16 M against 14.8 M trials). The cycle-stop variant without
  torsion already solves it. Exploratory.
- `torsion_division_derivative_20261031` (H109): scoped null. 0/28 exact
  formulas over 9,096 rows; candidate best match 0.500–0.611 against controls
  at 0.500–0.630.
- Covered: `torsion_auxiliary_inputs_20260918` (Cheon) in `prog_prime`.
- **Treatise:** ABSENT. Curve25519 has 0 hits in the treatise.
- **IC relevance:** high (component results on the algebraic track).

## 22. Structured-base and structured-solver integration (prime field, binary)

- `eccp131_structured_ic_20261022`: a μ_5 multiplicative-coset base against
  an equal-size small-abscissa base on the published ECCp-131 field (12
  cosets). A bounded exact observation; no rho.
- `structured_closure_20261024`: preregistration of a structured closure
  solver. It records the symmetrised S4 at 987×–769,221× strong rho, against
  a pair table at 9×–131×.
- `structured_ic_integration_20261104` and `structured_ic_postmerge_20261105`:
  tensor-structured consumer integration and custody audit,
  PCC-20261104-458dda and PCC-20261105-e660a7, both **unverified**. Software
  only.
- `pair_cell_obstruction_20261019_v1`: linear-duality certificates of empty
  auxiliary cells (n=83, d=18), with an odd-characteristic pair chart. A
  global certificate has codimension ≥35, which is infeasible.
- `structure_aware_ic_20261019`: artefact stub (report.pdf only).
- **Treatise:** ABSENT.
- **IC relevance:** medium (engineering and stage-level).

## 23. Hyperelliptic-cover infrastructure and Weil-restriction census (pointer only)

- `hyperelliptic_cover_infrastructure_20261006` and
  `weil_restriction_isogeny_census_20261018` are uncited. They belong to the
  transfer digest and are listed here only for completeness.

---

## Corrections and retractions in this family

These belong in the `audit` chapter.

1. `prime_rho_automorphism_20261015`: class set-up omitted. Cubic 0.611 →
   0.647 and quartic 0.791 → 0.819; the quartic prediction is no longer
   inside the interval (24 Oct).
2. `conductor_gap_cm_pairs_20261010`: the banner "no implemented method sees
   the gap" was withdrawn (SHF-20261017-e4d7f6 L-N2, SHF-20261028-0f3f64
   PPN-1). The 1.22 lead at ℓ=8209 is closed.
3. `drastic_prime_conductor_gap_20261012`: the 0.5286 lead regressed to
   0.9468.
4. `conductor-gap-rho`: the 8% easy-gap floor advantage is a coordinate-model
   artefact under the representation hash (1.040 [0.965, 1.119]).
   The post-hoc 1.17 is closed.
5. `papers/isogeny-class-rho`: the KS defect, missing tie corrections and
   "no test fired" being false. The null is provisional (review 2026-10-19).
6. `papers/structural-completeness`: "proven" was changed to "argued"
   (SHF-20261017-e4d7f6 P-N4–N6).
7. `papers/lll-degeneracy-note`: the curve-specific LLL failure is withdrawn
   as an f64 overflow (SHF-20261016-3caa6f), and the P-521 mechanism sentence
   is withdrawn too (SHF-20261017-e4d7f6).
8. `weak_isogeny_classes_20261013`: the `ec_trapdoor::magic_number` legacy
   definition was corrected to MMT.
9. `ic_cover_weakclass_20261023`: the as-found Hilbert stop B0 is a
   relabelling (8.4× more unpriced F4 work).
10. GLV-invariant programme: E27 found the matched walk 1.45× too expensive,
    so every row moves further from rho. The validation gate errata are
    §9.19 and §9.33, and E38's kill fired at the pilot.
11. `nist_skip_criterion_20261025`: the separator is representation-dependent
    and is not an intrinsic property.
12. `isogeny_loop_glv_20261011`: erratum, the Verschiebung split is a
    provable loss.

## Coverage table

| # | line of work | IC-relevant? | coverage | est. pages it deserves |
|--:|:--|:--|:--|--:|
| 1 | Automorphism-quotient rho, strongest rho on j=0/1728 (incl. order-24) | yes (matched-rho reference) | PARTIAL | 5 |
| 2 | Endomorphisms act as scalars; orbit-size bound; decomposition lanes | yes (obstruction) | PARTIAL | 4 |
| 3 | GLV/GLS/isogeny-chain scalar multiplication (CryptoPro-B, CP6-782, endosweep) | no (ECC-general) | COVERED (brief) | 1.5 |
| 4 | Exact CM/conductor data of the catalogue (`curve_traits`, endosweep D_K) | low (infrastructure) | ABSENT | 2 (appendix table) |
| 5 | Conductor-gap programme (rho, BSGS, IC across large ℓ; Lean invariance) | yes | PARTIAL | 7 |
| 6 | Isogeny-class generic invariance; isogeny-class-rho paper and review | low-medium | PARTIAL | 2 |
| 7 | Class-representative IC selection (ℓ=2 yield, model effect) | yes | COVERED | 1 |
| 8 | Closed obstructions: table transport, JMV, torsion leak, X_1(N), Cl(O) bridge | medium | PARTIAL | 3 |
| 9 | Special j in Semaev/IC (S_3 support 9/10/13; matched-order design) | yes (open lead) | PARTIAL | 2 |
| 10 | GLV-invariant factor bases over F_{p^k}, E1–E41 (V4 line, GLS fold, chain solve, pivots) | very high | PARTIAL | 14 |
| 11 | Strongest rho on F_{p^2}/F_{p^3} (`ext_endo_rho`, band reversal) | yes | ABSENT | 2 |
| 12 | Frobenius-orbit IC vs rho-frob; Frobenius quotient notes | yes | COVERED | 0.5 |
| 13 | Weak-curve atlas: ISO-1 theorem, catalogue/DB/Lean, GHS closure, Hilbert stop | medium-high (cover IC) | PARTIAL | 7 |
| 14 | NIST seed provenance and the skip rule (C plus about 55 CA dirs, 4 papers) | no | PARTIAL | 3 |
| 15 | Standard-curve audits: structural completeness, secp256k1 crater, SafeCurves | low | PARTIAL | 3 |
| 16 | LLL degeneracy retraction | no | ABSENT | 0.5 |
| 17 | Lean formalisations (transport, yield, left kernel, hazard, A1–A3, catalogue) | yes (IC parts) | PARTIAL | 6 |
| 18 | IC hardness-assumption chain: six obligations, weak points, censuses, corpus | very high | ABSENT | 10 |
| 19 | Summation-polynomial theorems and audits (t=2 trace, alternatives audit, Edwards S_3) | very high | PARTIAL (core COVERED) | 4 |
| 20 | Twists and IC beyond the obstruction (twist_ecdlp E0–E6, prime-field twist IC, F37 study) | yes (negatives) | PARTIAL | 4 |
| 21 | Torsion/cofactor levers (Curve25519 T1–T3, isogeny seed, n37 anchors, H109) | yes | ABSENT | 4 |
| 22 | Structured-base and structured-solver integration (ECCp-131 coset, pair cells, tensor) | medium | ABSENT | 2 |
| 23 | Hyperelliptic-cover infra and Weil-restriction census (pointer to transfer) | (transfer) | ABSENT | 0 (theirs) |
| — | Corrections listed above (into `audit`) | yes | mostly ABSENT | 2 |
| | **Total** | | 3 COVERED, 15 PARTIAL, 5 ABSENT | **≈ 90 pages**, of which ≈ 55 are new IC-relevant material. Lines 10 and 13 may be shared with other digests. |

## Suggested placement

**A. New Part "Structure and the limits of structure" (≈ 40 pp).**

| Chapter | Content | Lines | Pages |
|:--|:--|:--|--:|
| S1 | What structure a curve has: CM, conductors, volcano levels, catalogue data | 4, appendix part of 14 | 6 |
| S2 | Structure that rho uses: Aut quotient, j=0/1728, order-24, the extension-field rho references, band reversal | 1, 11 | 7 |
| S3 | Structure that cannot help: endomorphisms as scalars, orbit-size bound, conductor gaps, isogeny-class invariance, transport, JMV, torsion leak, twists | 2, 5, 6, 8, 20 | 15 |
| S4 | Weak classes and covers: ISO-1, atlas, GHS closure | 13 | 7 |
| S5 | Standard-curve audits and provenance, condensed | 14, 15, 3 | 5 |

**B. Extend `extension` with the E1–E41 programme (line 10): +12 pp.**

**C. New chapter "What IC hardness rests on" (≈ 14 pp).**

- the six-obligation chain;
- the weak points;
- the November censuses and the hardness corpus;
- the Lean results (left kernel, hazard, A1–A3, yield, transport);
- the alternatives audit;
- placement: next to `falldegree`.
- Lines: 17, 18, 19.

**D. Prime-field chapter: + "Torsion and coset levers" (≈ 5 pp).**

- Lines: 21, 22.

**E. `audit` chapter: + about 2 pp of corrections.**

- Items 1–12 above.

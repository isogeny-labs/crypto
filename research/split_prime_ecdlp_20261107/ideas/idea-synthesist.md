# Idea synthesis: can the Frobenius field enter a classical ECDLP algorithm other than through scalars?

Role: idea-synthesist (breakthrough tier). Date: 2026-11-07. Nothing here assigns work, changes a status or edits evidence.
Every lab result below is quoted as recorded and is `[pcc:unverified]` unless a claim id is given. None is given here.
Statements marked **derived** are arguments made in this memo. They are not run and not proved in Lean.

**Sources checked.** KB unavailable (the research-kb MCP server is not reachable in this session). Repository paths I opened:
seed report text (`scratchpad/pdf/report.txt`, sections 1 to 5.1);
`AGENTS.md` (§§ "Preserve scope", "Name the obstruction", "Intermediate-cost", "boundary, table, ratio" 1 to 5);
`problems/prime-scalar-exact-splitting.md`, `problems/tree-glued-pdp-separator-coupling.md`, `problems/curve25519-family-algebraic-ic-tower.md` (first 40 lines of each);
`research/prime_ic_program_20261029/DECISIONS.md`;
`research/isogeny_class_difficulty_20261008/README.md` and `PROTOCOL-I5-end-ring-and-aut.md`;
`research/isogeny_class_ic_yield_20261102/{DESIGN.md, p2_prime_analogues/RESULTS.md, p3_ecc2k130/RESULTS.md, report/REPORT.md §5}`;
`research/p2_lift_defect_20261101/README.md` (H110), `research/torsion_division_derivative_20261031/README.md` (H109);
`research/prime_field_leads_20261031/C1_lift_torsion/REPORT.md`;
`research/semaev_alternatives_audit_20261104/leads/LEAD-ICP-sqf-global-relations.md`;
`research/pkm_supersingular_aux_20261015/README.md` and `q2/cells.tsv`;
`research/ic_tree_gluing_20261103/THEORY.md` §2;
`research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` §3;
`src/cryptanalysis/ic_framework/prime_tower.rs` (header, `isogeny_tower`); `examples/pkm_ss_tower.rs` (header).

## (a) Verdict: no usable non-scalar entry point. One narrow door remains, and it is presentation, not arithmetic.

| channel | how K = Q(√(t²−4p)) reaches it | non-scalar? | standing |
|:--|:--|:--|:--|
| the group | E(F_p) ≅ O/(π−1) as O-modules (Lenstra 1996). End(E), Cl(O), ℓ-torsion and Artin symbols act as α mod (π−1) | no | given this session |
| generic algorithms (rho, BSGS, generic IC track) | only n and Aut(E) are visible. Aut ⊋ {±1} only at j = 0, 1728 | no | Shoup bound; DECISIONS R1 ("generic: bounded by sqrt(r)") |
| endomorphisms evaluated as maps (DGM/GLV folding) | a scalar μ = α mod N, priced by the cost of α. For random curves the smallest non-integer endomorphism has norm of order \|D_O\|/4, which is of order p. Endomorphisms built from Cl(O) relations cost Σ\|e_i\|ℓ_i field operations, far more than a scalar multiplication | no (scalar, priced) | **derived**; I-5 states the same boundary |
| the equation (presentation) | which j (roots of H_O mod p) share the group. Transport is a bijection on E(F_p) | only via coefficients | P1: prime-field F4 counters (D_solve, first-fall, rank, field mults) within 3.85e-5 of 1 across 27 Elkies neighbours **and 27 random non-isogenous curves** (`report/REPORT.md` §5). P2: generic-track yield flat. P3 (binary): the one Dreg gap was a basis-sparsity presentation effect |
| factor-base structure | the PKM amplification needs F_p-rational fibres. For prime n, a rational kernel or homomorphism preimage meets E(F_p) in {O} or in all of it, and an isogeny fibre contains exactly one rational point. So Elkies (split) primes of E cannot build a tower on E. The tower must live on an auxiliary curve or on G_m, whose arithmetic is unrelated to K_E. On the shared x-line, the algebraic subgroups come from the twist (order N(π+1)), and all of their abscissae are off E | no | **derived**; consistent with `prime_tower.rs` (auxiliary E′) |
| transfers | the residue of π̄ mod (π−1), which is p mod n (embedding degree: MOV/FR), and t = 1 (π−1 equal to −π̄: anomalous; Smart/SASS). Both are class invariants | yes, the only known ones | classical; isogeny_class_difficulty README lists them as non-varying |
| CM lift / ray class field K_N | E(F_p) ≅ Ẽ[π−1] ≅ E_q(F_{q^f})[N] for any other CM reduction, including one constructible with full n-torsion over F_q (embedding degree 1). But this isomorphism is defined only up to Gal(K_N/K) ≅ (O/N)^*/O^*, which acts as **every** scalar (Shimura reciprocity). So it carries no information beyond "an isomorphism exists". Computing it needs a torsion point over a field of degree n^{1 to 2} (C1 report). Reduced elliptic units at N-torsion are Miller-function values, whose only bilinear invariant over F_p is the Tate pairing, trivial unless n \| p^k−1 | no | **derived sketch**; H110 (torsion lift has τ = 0), C1_lift_torsion |
| the seed statistic itself | split count and L(1,χ) set SEA cost and the size of the cheap horizontal neighbourhood (transport reach) | no | the transport theorem plus flat N(E′) (P1, P2) gives W ≥ 1 whatever the reach |

**The obstruction, named.** The action is scalar-only (Lenstra plus Shimura), and the decomposition systems have curve-independent top forms.
The second half is a derived lemma that I have not seen written down in the lab:

> **Lemma (derived).** Every Semaev polynomial S_m(x_1..x_m; a, b) is weighted-homogeneous (isobaric) in the weights x_i:1, a:2, b:3 (general Weierstrass: a_i:i). S_3 has weight 4, which checks term by term; S_m inherits the property through the resultant recursion. So its top x-degree form equals S_m(x; 0, 0), the cusp's polynomial, for every curve. The PKM tower equations come from a fixed auxiliary curve and do not involve E.

Consequences of the lemma:
- The top-degree Macaulay structure, and hence Dreg in the semi-regular sense, is the same for every target curve. This explains P1's 3.85e-5 flatness across unrelated Frobenius fields.
- Curve dependence can enter only through degree falls driven by lower-order coefficients.
- In short Weierstrass form over F_p, lower-order sparsity (a = 0 or b = 0) coincides exactly with CM by Z[ζ₃] or Z[i]. Arithmetic and presentation are therefore confounded at those two j values and nowhere else.

**Record note (scope, not contradiction).** The brief cites `p2_prime_analogues/RESULTS.md` for "yield / Dreg / F4 flat". That file records relation yield per attempt only (mitm, generic track). The prime-field F4 counters are in P1 (`report/REPORT.md` §5). Prime-field Dreg across Frobenius fields has been sampled only on 27 random curves at small cells. CM strata have measure zero and are never hit by random draws.

## (b) Two connections not written down here

**C1. Presentation × arithmetic factorial on prime fields.**
It joins four pieces of work:
- P3's binary presentation finding;
- I-5's End/Aut protocol (unrun);
- the top-form lemma;
- the seed's a = −3 curve family.

The new observations:
- For every a = −3 curve (every NIST/X9.62 prime curve), the shift x = X ± 1 gives the linear-term-free model y² = X³ ± 3X² + (b ∓ 2). Half of all curves have such a shift (the condition is that −a/3 is a square).
- A second natural shift, r = ξ₀ (the first kernel abscissa of the tower), makes the first Vélu map sparse instead. The two sparsities compete.
- A random dense shift of a j = 0 or j = 1728 curve is the control that separates End(E) from coefficient sparsity.

Why it was invisible lane by lane:
- The binary lane found sparsity as a basis property.
- The prime lanes use one fixed short model, where sparsity and CM coincide.
- The isogeny-class lanes hold the model fixed by design.

**C2. Exhaustive j-line census at p = 786433, with Frobenius-field covariates at the trace level.**
This is the prime of the tree-glued msolve runs and of the pkm q2 P0 rows. K depends only on t, so this p has about 3,547 traces, of which about 250 have prime order. The trace, not the curve, is the right statistical unit for a K effect. P1's 27 random curves sample only about 27 traces and none of the small-|D_K| volcanoes.
The census joins the seed's covariates (split count for ℓ ≤ 691, partial L(1,χ), exact h, conductor f) to exact F4 counters for every prime-order curve. It closes three questions at once:
- whether any IC-relevant quantity depends on K (the seed's open question), for m ≤ 3;
- the location of the exceptional locus that the lemma allows;
- the "one curve measured" limitation of the tree-glued line, for its native cheap cells.

## (c) Claims, cheapest tests, kill lines and priors: see the records below

| | exact claim (short) | kill line | prior: any effect | prior: improves the prime-field IC baseline at any scope |
|:--|:--|:--|:--|:--|
| C1 | (i) Any j = 0 or 1728 counter difference disappears under a dense random shift, so it is presentation. (ii) The a₄ = 0 shift lowers F4 muladds by ≥ 10% with D_solve not higher | (i) killed if the difference survives the dense shift (arithmetic). (ii) killed if muladds_sparse/muladds_dense > 0.90 on ≥ 2 of 4 cells at either prime | (i) arithmetic survives: 2%. (ii) gate passes: 10% | 3% (engineering, a = −3 family, algebraic track); 0% for an exponent change |
| C2 | Exact counters are identical across all prime-order curves at p = 786433, except at a = 0, b = 0 and pre-registered coincidences | H_K survives only if an unexplained exception reproduces on a fresh target set **and** is enriched in a K stratum (Bonferroni α = 0.0025) | 5% reproducible exception; 1% K-enriched | ≤ 0.5% |

## (d) Closest prior work
- Lenstra, "Complex multiplication structure of elliptic curves", J. Number Theory 56 (1996).
- Jao–Miller–Venkatesan, ASIACRYPT 2005 and J. Number Theory 129 (2009): random self-reducibility across an isogeny class.
- Shoup 1997.
- Duursma–Gaudry–Morain 1999; Gallant–Lambert–Vanstone 2001.
- Menezes–Okamoto–Vanstone 1993; Frey–Rück 1994; Smart 1999; Satoh–Araki 1998; Semaev 1998.
- Silverman–Suzuki, ASIACRYPT 1998 (index calculus by lifting); Silverman 2000 (xedni); Jacobson–Koblitz–Silverman–Stein–Teske 2000.
- Huang–Raskind, LMS J. Comput. Math. 12 (2009) (signature calculus / global duality).
- Stange, Pairing 2007 (Tate pairing via elliptic nets); Lauter–Stange, SAC 2008 (EDS-DLP equivalences). These are the nearest published closure of the "ray class units" route.
- Semaev 2004; Petit–Kosters–Messeng, PKC 2016; Faugère–Gaudry–Huot–Renault, J. Cryptology 2014 (symmetries in index calculus).
- Huang–Kosters–Yeo, CRYPTO 2015 (last fall degree); Elekes–Szabó 2012.
- Lab paths: `research/isogeny_class_difficulty_20261008/` (transport theorem, I-5); `research/isogeny_class_ic_yield_20261102/` (P1, P2, P3); `research/prime_field_leads_20261031/C1_lift_torsion/REPORT.md`; `research/p2_lift_defect_20261101/`; `research/pkm_supersingular_aux_20261015/`; `research/ic_tree_gluing_20261103/THEORY.md`.

---

## Idea records

```yaml
- id: SYN-20261107-C1-presentation-x-arithmetic-factorial
  title: >-
    Prime-field presentation x arithmetic factorial: separate End(E) from
    lower-order coefficient sparsity in the PKM decomposition system, and test the
    a=-3 linear-term-free shift as an algebraic-track engineering lever
  obstruction_attacked: >-
    Scalar-only action (Lenstra/Shimura) plus curve-independent top forms
    (derived lemma). The only remaining door for End(E) into F4 counters is
    lower-order coefficient structure, and in short Weierstrass form it is
    confounded with CM exactly at j in {0, 1728}.
  claim: >-
    (i) At p in {786433, P24} (P24 = least prime >= 2^24, p = 1 mod 12, with
    prime-order twists at j=0 and j=1728), on the PKM isogeny-tower systems built
    by examples/pkm_ss_tower.rs f4 (m=2, t in {3,4,5}; m=3 chain, t=2), any
    difference in exact counters (D, width_to_solution, steps_to_solution,
    muladds) between a j=0 or j=1728 prime-order curve and the random-j modal
    values vanishes when the same curve is presented in a dense random shift
    x = X + r (r drawn from a public seed). So the difference is presentation,
    not End(E). (ii) For prime-order a=-3 curves, the shift r = +1 or -1 (model
    y^2 = X^3 +/- 3X^2 + (b -/+ 2)) gives a paired median muladds ratio
    sparse/dense <= 0.90 with D not higher, on >= 3 of 4 cells at both primes.
  mechanism: >-
    S_m is isobaric (x:1, a:2, b:3; general Weierstrass a_i:i), so its top
    x-form is the cusp's S_m for every curve, and the tower equations do not
    involve E. Hence Dreg (top-form sense) is curve-independent and any curve
    dependence of the F4 solving degree or work must come from degree falls fed
    by lower-order terms. A shift is an affine substitution x_i = X_i + r.
    It preserves the ideal's solutions and the top forms, and changes
    lower-order sparsity. P3 (binary) recorded exactly this kind of effect
    (E0 Dreg 6 vs 7 under the standard V; erased by the b-adapted modulus).
  lean_obligation: >-
    Lemma: S_3 (and S_m via the resultant recursion) is isobaric with weights
    (x:1, a:2, b:3); its homogeneous top x-component equals S_m(x;0,0). State
    for S_3 in Mathlib polynomials first. Also: S_m^{E_r}(X) = c * S_m^E(X + r)
    for the shifted model E_r.
  prediction: >-
    (i) No surviving arithmetic difference: dense-shifted j=0/1728 counters equal
    the random-j modal counters exactly, in every cell. (ii) Declared interval for
    the median muladds ratio sparse/dense: [0.85, 1.05], point guess 0.97; D
    identical in every row. The tower-sparse shift r = xi_0 is predicted to be no
    better than (ii). Positive control: j=0 with a zeta_3-stable "cube tower"
    base (y_0 = x^3, then the tower on y_0) shows the known automorphism effect
    (orbit folding by 3 in the base, and lower muladds per relation-capable
    solution).
  discriminating_test: >-
    Arms per curve (exact counters, same public V from the recorded P0-ordinary
    auxiliary curves at 786433 in pkm_supersingular_aux q2/cells.tsv, and the
    searched auxiliary at P24): A0 original short model (the unmodified
    baseline); A1 dense random shift; A2 curve-sparse shift (r = +/-1 for a=-3,
    r = sqrt(-a/3) otherwise); A3 tower-sparse shift r = xi_0. Curves: every
    prime-order twist at j=0 and j=1728; 64 prime-order a=-3 curves; 64
    prime-order curves with -a/3 a square; one vertical-floor neighbour of the
    j=0 curve when its volcano has a small conductor prime (same n and K, dense
    coefficients, smaller End). 16 targets per curve; cells m=2 t=3,4,5 and m=3
    t=2. Every row re-verified exhaustively over V^m (the existing verify field).
    Implementation (Rust only): a general-Weierstrass S_3 and chain builder (or the
    affine substitution applied to the existing builder), a target-curve selector,
    and the cube-tower base for the positive control.
  falsification: >-
    (i) Killed (that is, End(E) does move F4 counters) if any j=0/1728 counter
    difference from the random-j mode survives A1 on the same curve and
    reproduces on a fresh 16-target set. (ii) The engineering lever is killed if
    the paired median muladds ratio A2/A0 > 0.90 in >= 2 of 4 cells at either
    prime, or if D_A2 > D_A0 in any row. Inadmissible: changing V, m, t or the
    F4 engine between arms; choosing r after seeing counters; dropping targets;
    counting unverified rows.
  cost_band: >-
    Implementation 1-2 engineer-days of Rust. Compute, derived from recorded
    q2 per-cell times at 786433 (2-21 ms per m=2 cell; m=3 assumed <= 100 ms; L0
    clock, descriptive): about 70 curves x 16 targets x 4 arms x 4 cells ->
    0.1-0.5 CPU-h at 20 bits, 1-5 CPU-h at 24 bits, < 1 GiB. Reference arm A0 is
    priced in the same counters. No S and no rho comparison at this stage
    (stage diagnostic, AGENTS.md section 2). Only if (ii) passes: stage 2, the
    whole-arm ecbench pkm-semaev S with A2 vs A0 and the matched rho, at 0 / 0.1 /
    1 GAE per unpriced operation (section 5a), ranked only where the order holds
    across the band; about 5-20 CPU-h more. Run obligations for whoever is
    authorized: RESUME checkpoint, bench.record/v1, director-frozen route.
  controls: >-
    A/A repeat of A0; the u-scaled isomorphic model (x -> u^2 x keeps sparsity;
    counters must equal A0); a planted-solution target per curve (the detector
    check, not a D comparison); the cube-tower positive control at j=0; a
    null-tower control with matched shape (the existing `null` arm).
  risks: >-
    If f4_fp_tower uses dense linear algebra, sparsity can reduce muladds only
    through matrix dimensions, and (ii) is then predicted null. The builder's
    "reduced presentation" may already normalize lower-order terms. Small cells
    may saturate (D = 5 plateau in q2), which hides degree-fall effects.
    The lemma is derived, not formalized.
  closest_baseline: >-
    P1 (report/REPORT.md section 5): prime-field F4 counters flat within 3.85e-5
    across 54 non-E0 curves at ECCp-131 width. P3 C2 (binary presentation
    effect). I-5 protocol (unrun). Faugere-Gaudry-Huot-Renault 2014 for the
    automorphism positive control.
  prior_improves_baseline: "3% (engineering, algebraic track, a=-3 family, toy scope); ~0% for an exponent change; 2% that an arithmetic End(E) effect exists"

- id: SYN-20261107-C2-jline-census-786433 [pcc:unverified]
  title: >-
    Exhaustive j-line census of PKM decomposition counters at p = 786433, with
    Frobenius-field covariates analysed at the trace level
  obstruction_attacked: >-
    Random-curve controls (P1: 27 curves) cannot reach the measure-zero CM and
    small-|D_K| strata, where an algebraic exceptional locus (a proper Zariski-
    closed set in (a,b), defined over Q) would most plausibly contain special
    points. K depends only on t, so curve-level sampling has low power for K.
  claim: >-
    For every prime-order curve E/F_786433 (all isomorphism classes, both
    twists), with fixed public V (the recorded P0-ordinary auxiliary towers,
    t = 3,4,5) and 8 seeded targets, the exact counters (D, width_to_solution,
    steps_to_solution, muladds) of the m=2 full and m=3 chain (t=2) systems equal
    the global modal values. The only exceptions are curves with a=0, b=0, or a
    pre-registered coincidence: V meeting the roots of x^3+ax+b, x_R in V, or
    x_R a root of a tower kernel polynomial.
  mechanism: >-
    The same derived lemma as C1: the top forms do not depend on the curve, so
    any exception lies in a lower-order degree-fall locus. That locus is
    algebraic in (a,b) and has bounded degree per cell, so it contains O(1)
    j-values and cannot follow a split-prime pattern or a class number. The
    census tests that prediction and, separately, whether the O(1) exceptional
    j-values are CM points of small |D|.
  lean_obligation: >-
    Same isobaric lemma as C1. No new obligation; the census is empirical.
  prediction: >-
    Zero unexplained exceptions in every cell. Trace-level Spearman |rho| with
    each K covariate (split count for l <= 691 with l not dividing n, partial
    log L(1,chi_DK) to 691, log h(D_K) exact by reduced forms, f > 1 flag,
    |D_K| <= 10^4 flag) is undefined (constant counters), or < 0.2 if variation
    exists.
  discriminating_test: >-
    Rust, native. (1) Enumerate all (a,b) classes over F_786433 and count points
    with the existing Mestre BSGS (about 1.57M curves). Keep the prime-order ones
    (about 1e5, over about 250 traces). (2) Compute the per-trace K covariates
    (factor t^2-4p, D_K, f, h by reduced forms, split count, partial L) and the
    per-curve flags (a=0, b=0, volcano level, coincidence covariates).
    (3) Run the 4 cells x 8 targets on every curve of the 20 most K-extreme
    traces (10 lowest and 10 highest partial L), on every curve of each
    |D_K| <= 10^4 volcano, and on 32 seeded curves of each remaining trace
    (about 2e4 curves). Escalate to the full 1e5 only if (3) shows any
    unexplained exception. (4) Re-run every exception on a fresh 8-target set.
    (5) Test whether reproduced exceptional j are roots of H_D mod p for
    |D| <= 10^4.
  falsification: >-
    The null (K does not enter the algebraic track at m <= 3, t <= 5, this p)
    is rejected only if >= 1 exception outside the flagged sets reproduces in
    (4) AND the exceptional traces are enriched in a K stratum (Fisher exact
    test, Bonferroni over 5 covariates x 4 cells = 20, alpha = 0.0025), or the
    reproduced exceptional j are CM with |D| <= 10^4 at a rate above the base
    rate of such j among prime-order curves (same alpha). Otherwise the null
    holds at this scope and the seed's "no IC-relevant quantity depends on K" is
    recorded as measured, not assumed. Inadmissible: changing V or the targets
    after (3); post-hoc covariates; counting rows that fail verify.
  cost_band: >-
    Implementation 1-2 engineer-days of Rust (enumerator, K invariants, target-
    curve selector for pkm_ss_tower f4). Point counting about 1.57M x 0.1-1 ms =
    3-30 CPU-min. F4 stage (3): about 2e4 curves x 8 targets x 4 cells x
    (2-100 ms) -> 1-5 CPU-h; full escalation about 1e5 curves -> 5-25 CPU-h;
    < 1 GiB. No S, no rho and no section 5a band (stage diagnostic without a
    cross-method ranking). The reference is the global modal counter vector,
    and the A/A repeat is priced in the same counters. Optional, non-native: the
    tree-glued cherry cells (m=3,4, t=4; recorded excess 2 and 3) on 8 census
    curves (j=0, j=1728, 2 small-|D_K|, 2 low-L, 2 high-L) with msolve 0.6.5 at
    the pinned inputs, a few CPU-h at <= 15 GB. Run obligations as in C1.
  controls: >-
    A/A repeat on 64 curves; u-scaled isomorphic models of 64 curves (counters
    must be identical); planted-solution targets on 64 curves (detector check);
    the shape-matched null tower; a negative-control V from a different public
    seed on 256 curves (exceptions should move with V if they are coincidences).
  risks: >-
    At m=2 the q2 plateau (D = 5 for N = 4..18) may make every counter constant
    by saturation. width_to_solution and muladds are then the informative
    counters. About 250 traces give limited power for a weak monotone K trend.
    One prime only; any positive result needs a second prime before
    interpretation. A positive CM enrichment would be a weak-curve finding for
    special curves, not a baseline improvement for random curves.
  closest_baseline: >-
    P1 random-curve arm (27 curves; ratio to E0 within 3.85e-5); P2 class-internal
    yield null; pkm_supersingular_aux Q2 (D identical across auxiliary families);
    tree-glued problem record ("one curve was measured").
  prior_improves_baseline: "<= 0.5%; 5% for any reproducible unexplained exception; 1% that it is K-enriched"
```

# Round 4 proposals: going below the 2^l scan for R = P1 + P2 + P3, x(Pi) in V (idea generator, 2026-11-16)

Status: proposals only. Nothing here was run; every number is a desk derivation or a quote
from a file on disk, and is `[pcc:unverified]`. I had no shell in this session, so
`tools/lessons.py brief` was not run (a grep of `docs/lessons/entries` for the scope
`idea-generator` found no entry), and no toy test was executed.
The `research-kb` MCP server and the lead atlas were not reachable (no `crypto-autoresearcher`
checkout under `/home/user`); the literature check below is web search plus on-disk files.
Tests are specified as standalone Rust files (AGENTS.md "Implementation language: no Python"
overrides the "or Python" option in the request).

## Evidence on disk that these build on

- `research/groebner_free_matched_20261113/README.md`: lin21-ht arm, counted work slope
  difference 0.0006 [-0.0004, 0.0016] against the pair-table floor, "does not grow more slowly
  than the floor, and by construction cannot"; S4 desk form with four non-affine monomials
  `e3^3, e2^2 e3, e1^2 e3^2, e1^2 e3`; open items 1 to 3 in section 4.
- `research/sat_smt_leads_20261114/README.md` (ideas 4, 11, 8; sub-scan follow-up): residual never
  affine before all of e1 is fixed; (leaves + conflicts)/2^l >= 2.48 at l = 8.
- `research/sat_smt_leads_20261114/04-xor-cubes-symmetric/README.md`: view E, S4 is F2-affine in (e1,e2)
  for fixed e3; XOR cubes about 2^{3l} conflicts.
- `research/sat_smt_leads_20261115/README.md` (leads A to F) and
  `.../F-close-4-and-11/part2/README.md`: min dim(V.V) = 2l-1; scan-following refutation about 1.4 bits per l;
  block refutation exponent 1.26 to 1.44; no cross-leaf sharing.
- `research/sat_smt_leads_20261114/07-homogeneous-allsat/README.md`: homogeneous (target-free) AllSAT
  killed in a bit-level SAT encoding; its "what a full study would need" asks for a homogeneous
  encoding that keeps the linear leaf. Proposal A is that, for a torsion target.
- `examples/ic_algebraic_floor/linear21.rs`, `examples/pdp_lin_cdcl.rs` (LinCdcl product gates).

Closest published work: Semaev 2004 (summation polynomials); Faugere-Perret-Petit-Renault, Eurocrypt 2012
(binary-curve decomposition by Groebner bases, first fall degree); Faugere-Gaudry-Huot-Renault 2014 (symmetry
groups, including 2-torsion translation); Galbraith-Gebregiyorgis, ePrint 2014/806 (binary Edwards,
symmetry-breaking factor base); Courtois, ePrint 2016/003 (point splitting on binary curves, about 2^{n/3}; I could
only read the search snippet, the page was not fetchable); Faugere-Safey El Din-Spaenlehauer 2013 (generalized
MinRank, bilinear systems); Bachoc-Serra-Zemor, arXiv 1501.00602 (linear Vosper theorem: equality
dim(ST) = dim S + dim T - 1 forces geometric-progression bases, for prime degree);
Hou-Leung-Xiang 2002 (linear Cauchy-Davenport). None of them states the torsion-target
factorisation of proposal A. That is a novelty claim from a thin search, to be rechecked in `research-kb`.

## Result of the counting filter

Five proposals survive (A to E). Nine candidate mechanisms were discarded by counting, listed with
their counts at the end. None of the survivors is predicted to change the exponent 1 (in l) of the scan for a
generic target R; A changes the constant (about 2) for a torsion target, and B, C, E are component or
diagnostic items. I could not honestly reach 6 to 8 live items without padding.

Notation: n = 3l + c, W = span(V.V) (dim 2l-1 for the polynomial span), V3 = span(V.V.V) (dim 3l-2),
G(m) = m^2 + m, Tr the absolute trace, "no wrap" means n >= 3l - 2, so a product of three polynomials of degree < l
in z is an exact product in F2[z], not reduced mod f.

---

## A. Torsion-target relations by F2[z] factorisation (no leaf scan)

```yaml
id: R4-A
title: "R = T0 (or other torsion) relations: e3 is an affine function of e1, so factor e3 in F2[z]"
claim: >
  For the target T0 = (0, sqrt b), the relation P1+P2+P3 = +-T0 with x(Pi) in V = span(1..z^{l-1}),
  n >= 3l-1, is equivalent to: e1 = x1+x2+x3 in V with Tr(e1) = 0 and
  e3 = x1 x2 x3 = sqrt(b) (e1 + H(e1) + eps), eps in {0,1}, H the half-trace (n odd).
  The set of candidate (e1, eps) with Lambda(e1) in V3 is an affine subspace A of F2^l of
  dimension kappa = 4l - 2 - n + O(1), found by one F2-linear solve. Each survivor is decided by
  factoring the polynomial e3 in F2[z] and grouping factors into three of degree < l with sum e1
  (polynomial time in l). Total 2^kappa factorisations: 2^(l-2) at n = 3l, against about 2^l leaves of the
  scan, a constant gain of about 4 in count (and 2^{3l-n} fewer at larger n); exponent in l unchanged (1).
mechanism: >
  S4 at xR = 0 collapses: S4 = e3^4 + b e3^2 + b^2 e1^4, from the desk form in
  groebner_free_matched_20261113/README.md section 4 with u = 0 (every u-term vanishes, e2 disappears).
  Then (e3/sqrt b)^2 + e3/sqrt b = e1^2, an Artin-Schreier equation, F2-affine in e1 after solving.
  e2 is unconstrained, so no scan over x1 is needed to fix it. In the no-wrap regime e3 = x1 x2 x3 is an exact
  polynomial identity, so existence of (x1,x2,x3) with product e3 is factoring in F2[z].
prediction: >
  Survivor count |A| in [2^(kappa-1), 2^(kappa+1)] for kappa = 4l-2-n, l = 4..11, n = 3l..3l+4;
  the set of triples recovered by factoring equals brute-force enumeration over V^3 on every cell;
  total operations (one Gauss on an l+1 by 3l-2 system plus 2^kappa factorisations) below the lin21-ht scan
  count 2^l in leaf-equivalents by a factor in [2, 8] at n = 3l.
discriminating_test: >
  Standalone Rust file (new, not edited in examples/ic_algebraic_floor): for n in {13,17,19,23,29,31,37},
  l = floor((n+1)/3), random b, V = polynomial span. (1) Enumerate V^3, list all triples with
  S4(x1,x2,x3,0) = 0, to know the truth. (2) Build A by linear algebra, count |A|, factor each survivor with
  Cantor-Zassenhaus over F2[z] (small degrees; a naive trial division is enough at these sizes),
  compare triple sets. (3) Report |A|, factorisations, and leaf-equivalents. One core, minutes.
falsification: >
  Any missed or extra triple (a bug or a wrong identity: kills the equivalence); or |A| outside the factor-2
  band on three or more cells (counting model wrong); or the cost of one factorisation, measured in
  F2 word operations, exceeding the cost of the lin21-ht leaf it replaces by more than the 4x count gain
  (then the constant gain is lost; report as "no gain in the unit").
cost_band: >
  A: one l-by-(3l) Gauss plus 2^kappa factorisations of degree <= 3l-3 polynomials, each O(l^2) to O(l^3)
  word ops. Scan reference (lin21-ht): 2^l leaves, each n-2l+2 equations in l+1 bits. Pair table: 2^{2l} time
  and memory. Priced at 0, 0.1, 1 GAE per unpriced native op for every arm (AGENTS.md 5a): the factoring is
  native word ops, the scan's group additions are priced the same way.
controls: >
  A/A: permute b and V basis order (counts must not move); a planted triple on a non-T0 target must
  NOT be found (target specificity); the same code with xR forced nonzero must fail the e2-free identity.
counting_check: >
  Unknowns: e1 in V (l bits) plus eps (1) minus Tr condition (1): about l bits. Equations: Lambda(e1) in V3
  is n - (3l-2) = c + 2 linear conditions on the n bits... solved as codim n-(3l-2) in F2^n: expected
  survivors 2^{l + (3l-2) - n} = 2^{4l-2-n}. Spurious: survivors with no triple, about 2^{4l-2-n} -
  2^{3l-n}. Degree: 1 (linear) for the prefilter, polynomial factoring afterwards. True triples expected
  2^{3l-n}/6 times a constant, so for n = 3l about O(1) per base: consistent with the homogeneous relation yield.
why_pilots_do_not_refute: >
  The pilots (ideas 4, 7, 2, leads A to F) attack generic R or the homogeneous S3 (xR = infinity). For
  xR = 0 the e2 monomials of S4 are absent, so "residual never affine before e1 fixed" does not apply: after e1
  is fixed the residual is e3 = const, a product equation, not an e2/e3 mixed one. Block-refutation and
  dual-rail pilots encode generic S4; none encodes the T0 target.
closest_baseline: "lin21-ht 2+1 scan, 2^l leaves; homogeneous S3 relations via the same scan; Courtois 2016/003 for 2^{n/3} splitting"
risks: >
  (1) T0 relations are homogeneous: they feed the log vector up to a scalar and do not replace a target
  relation (07-homogeneous-allsat notes the target relation is not costed). (2) For x_i real points the
  lift condition halves the count again; the constant gain may be about 2 rather than 4. (3) Needs no wrap
  (n >= 3l-2); with a wrapped V the factoring step is invalid. (4) Constant gain only; exponent unchanged.
lean_obligation: >
  Lemma "no-wrap product identity": for polynomials a,b,c of degree < l and n >= 3l-2, the field product
  of their images is the image of the polynomial product (needs only deg < n). Lemma "S4 at xR = 0 reduces to
  e3^4 + b e3^2 + b^2 e1^4". Neither is a security claim; both are small.
prior: "55% that the equivalence and the count hold; 25% that the measured gain in the AGENTS.md unit exceeds 2x; under 2% that the exponent in l changes."
```

---

## B. Dual (left-kernel) leaf: decide the leaf by m in V^perp, not by eliminating n x (3l-1)

```yaml
id: R4-B
title: "Leaf decision as one trace test against a left-kernel vector in V-perp"
claim: >
  For t = x(R-P1) the leaf (find s in V, p in W with p^2 + t p + t^2 s^2 = b) is solvable iff every
  m in V^perp with G(m) = m^2 + m in t.W^perp satisfies Tr(m sqrt(b)/t) = 0. The space of such m has dimension
  n - 3l + 1 + (0 or 1). At n = 3l the decision is a single bit.
mechanism: >
  Left kernel of M(t): lambda with Tr(lambda(w^2 + t w)) = 0 for w in W and Tr(lambda t^2 v^2) = 0 for v in V.
  With u = sqrt(lambda): u + t u^2 in W^perp and t u in V^perp; put m = t u. This changes the elimination from
  n rows by (3l-1) columns to a (2l-1) by (n-l) system over a precomputed basis of V^perp.
prediction: >
  Word operations per leaf (counted, not timed) at least 1.5x lower than the lin21 solve at n = 41..59 with l = n/3,
  and identical verdicts on every leaf (all 2^l leaves of 2 targets per cell); no change in leaf count.
discriminating_test: >
  Rust file next to linear21.rs's test harness (not editing it): for n in {13,..,41}, l = floor((n+1)/3),
  every x1 in V, compute the verdict by lin21-ht and by the dual test, compare, and count F2 word XORs for each.
falsification: "any verdict disagreement; or the dual cost within 1.2x of lin21-ht word operations at n = 41 (no component gain)."
cost_band: "dual leaf about (2l)(2l)(n-l) bit ops plus building t^{-1} multiples; lin21-ht about n*3l*3l/64 words. 2^l leaves both. Pair table 2^{2l}. Priced 0 / 0.1 / 1 GAE."
controls: "A/A: shuffled basis of W; a leaf from a planted decomposable target must give 'solvable'."
counting_check: >
  Unknowns m: n - l bits; equations: 2l - 1 (from W^perp), so kernel dimension n - 3l + 1 as the primal count
  (3l-1 unknowns vs n equations) predicts. Spurious solvable leaves 2^{3l-1-n} per leaf, the same as the primal.
  Degree 1.
why_pilots_do_not_refute: >
  The pilots changed the solver, never the algebra of the leaf test. This is a component change with the same
  leaf count, so it is not refuted by "no arm goes below the scan"; it does not claim to go below it.
closest_baseline: "lin21-ht (the repository's own); Kipnis-Shamir style kernel modelling for the dual form"
risks: "constant factor only; building the multiples of t^{-1} may erase the gain; stage diagnostic, not an end-to-end claim."
lean_obligation: "Duality lemma: left kernel of the leaf map equals {m : m in V^perp, G(m) in t W^perp}; Tr(a^2) = Tr(a) is the only fact used."
prior: "30% for a >= 1.5x counted-operation gain; 0% for an exponent change."
```

---

## C. Quadratic-correspondence t without y-lifting and group addition

```yaml
id: R4-C
title: "Compute t(x1) from S3(x1,t,xR) = 0 by one half-trace and a batched inversion"
claim: >
  t solves (x1+xR)^2 t^2 + x1 xR t + (x1 xR)^2 + b = 0, hence t = k w with k = x1 xR/(x1+xR)^2,
  w^2 + w = (x1+xR)^2 (1 + b/(x1 xR)^2). Per leaf this needs one inverse (batched by Montgomery's trick over
  the 2^l leaves, 3 multiplications each), one table half-trace (F2-linear) and the Tr condition, replacing the
  y-lift and the group addition. Both roots t, t+k are tried (the two signs of P1 give the same x(R-P1) set).
prediction: "ns per leaf of the t computation 2x to 4x lower than the lift+addition at n = 41 (stage ratio); leaf verdicts identical on all 2^l leaves."
discriminating_test: "Rust: for n in {13,23,31,41}, compare the multiset {t} from the existing group arithmetic and from the quadratic, then time both. One core, minutes."
falsification: "multiset mismatch; or stage ratio under 1.3x at n = 41."
cost_band: "scan 2^l leaves, each with about 5 field mults vs about 10 plus a lift. Pair table 2^{2l}. Priced 0/0.1/1 GAE."
controls: "A/A over target order; correctness on all 2^l leaves."
counting_check: "unknowns t (n bits), 1 equation of degree 2: 2 roots, Tr condition halves the leaves with a root: matches the known fraction of liftable x1."
why_pilots_do_not_refute: "a stage-cost item, not a sub-scan claim; the pilots measured conflicts, not leaf-preparation cost."
closest_baseline: "the existing lin21 trial (250 to 840 ns per trial at n = 13..59, groebner_free_matched README table)"
risks: "relabelling risk: the existing code may already avoid the y-lift; check linear21.rs before running. Constant factor only."
lean_obligation: "none needed beyond the algebraic identity (state it, optional)."
prior: "40% for a >= 1.3x stage gain; stage only, never an end-to-end claim."
```

---

## D. First fall degree of the bilinear (t; q) system at bidegrees (a, b)

```yaml
id: R4-D
title: "Macaulay-rank measurement for the 2n-equation bilinear system in (t: n bits; q = (x1,s,p): 4l-1 bits)"
claim: >
  The system S3(x1,t,xR) = 0, p^2 + s^2 t^2 + p t + b = 0 (2n bilinear equations; n + 4l - 1 unknown bits)
  has first fall at bidegree (d_t, d_q) with d_q >= 1 and d_t + d_q growing at least linearly in l; no bidegree
  with d_t + d_q <= 3 is solving for l >= 6. Equivalent to saying plain bilinear Groebner/XL does not beat 2^l.
mechanism: "linearisation: bidegree (1,1) has n(4l-1) + n + 4l-1 monomials against 2n equations; multiplying by q_j (bidegree (0,1)) gives 2n*4l equations against n * 8l^2 monomials: ratio about 1/l."
prediction: "the rank deficit (monomials - rank - 1) at (1,2) and (2,1) exceeds 2^{4l-1-n} (the true F2 solution count) by a factor growing with l; no bidegree <= (2,2) isolates the solution for l >= 5."
discriminating_test: "Rust: n = 13, 17, 19, l = 3..5; build Macaulay matrices over F2 with field equations, report rank by bidegree up to (2,2). Minutes, one core."
falsification: "a bidegree <= (2,1) at which the planted solution is the unique kernel vector for l >= 5 (then bilinear structure beats enumeration; escalate)."
cost_band: "Macaulay matrix cost (rows x cols x rank / 64); scan 2^l leaves; pair table 2^{2l} (all priced)."
controls: "random bilinear system with the same (n, 4l-1, 2n) as null; planted instance with known solution."
counting_check: >
  Equations 2n versus unknowns n + 4l - 1: underdetermined by 4l - 1 - n bits so at n = 3l the F2 solution count is
  2^{l-1} relaxed solutions (the leaf unknowns are relaxed to W, not split). The measurement is of Macaulay
  rank, not a solve, so counting does not kill it; it predicts failure and is cheap.
why_pilots_do_not_refute: "pilot 04 measured non-affine rank after XOR/literal cubes on S4 in x-bits; this measures the (t; q) bilinear lift with t kept as n free bits, a different variable split (MinRank-like)."
closest_baseline: "Faugere-Safey El Din-Spaenlehauer 2013 bilinear/MinRank bounds; Kipnis-Shamir kernel modelling"
risks: "most likely confirms the pilots; value is the negative with a measured fall degree."
lean_obligation: "none."
prior: "3% that a low bidegree solves it."
```

---

## E. Count-matched two-level hybrid: guess k bits of s, then the leaf is a pencil of rank k

```yaml
id: R4-E
title: "Is the relaxed leaf kernel (kappa = 3l-1-n) cheaper to split by linearisation than the scan it replaces, measured at l = (n+kappa+1)/3?"
claim: >
  At l = (n + kappa + 1)/3 the leaf has a kappa-dimensional kernel; the split z^2 + s z + p = 0 (z in V) is
  bilinear in (kernel coordinates a, z) with kappa*l + kappa + l + 1 monomials against n equations. Linearisation
  decides the leaf only for kappa <= 1 (equal at kappa = 2: 3l + 5 > n); per-relation cost is 2^{n-2l} * 2^{max(0, kappa-1)}.
  Prediction: no kappa >= 1 lowers the per-relation cost below the kappa = 0 value 2^{n/3}.
mechanism: "README groebner_free next experiment 3 (kernel regime d > n/3), stated with a linearisation count."
prediction: "per-relation cost 2^{(n+kappa)/3 - 5/3} for kappa >= 1, increasing in kappa: gain 0."
discriminating_test: "Rust: n = 23, 29, 31; kappa = 0..3; measure leaves per relation and Gauss cost with and without guessing; compare per relation."
falsification: "a kappa >= 2 cell where per-relation cost falls below the kappa = 0 cell in the AGENTS.md unit."
cost_band: "scan x1 over n-2l bits (restricted decomposition) plus a kernel split; pair table 2^{2l}."
controls: "kappa = 0 cell as the unmodified baseline; A/A over bases."
counting_check: "unknowns a (kappa), z (l), monomials a*z (kappa*l); equations n = 3l - 1 - kappa; consistent only for kappa <= 1 by the count above."
why_pilots_do_not_refute: "kernel regime was listed open (groebner_free README 4.3) and is untested."
closest_baseline: "FPPR 2012 hybrid approach (guess variables, solve rest); lin21-ht"
risks: "predicted negative; it closes an open item with a number."
lean_obligation: "none."
prior: "5% for a per-relation gain; the measurement itself is cheap and closes an open item."
```

---

## Discarded by counting (not proposals)

| mechanism (user's angle) | count that kills it |
|---|---|
| Frobenius-stable V, Koblitz orbit acts on (V, R) jointly | stable V is a sum of Frobenius modules, so V.V is stable of dim >= 2l (not 2l-1, by the linear Vosper theorem, bases in geometric progression are not Frobenius-stable for prime n). Random stable V at l ~ n/3 has V.V = F (l(l+1)/2 products >> n); isotropic V (Tr(vv') = 0) only gives V.V in a hyperplane (dim <= n-1). Unknown count l + dim W > n. |
| Frobenius orbit on one target with a non-stable V | squaring leaves V (V cap V^2 has dim ceil(l/2) for the polynomial span); no shared leaf between R and phi(R). |
| Shared XOR/dual certificate across a block of k x1 values or across N targets | certificate mu must kill (k+1)(3l-1) conditions (or N(3l-1)) on n bits; exists only for k = 0 at n ~ 3l (n = 4l at m = 4 also k = 0). This is the counting form of lead C's exponent 1.26 to 1.44. |
| Additive FFT / multipoint evaluation of the leaf invariant over V | t(x1) = x(R-P1) is a rational map in (x1, y1) with y1 a half-trace of x1 + a + b/x1^2: not low degree, not additive; the leaf invariant (Moore determinant of 3l elements) has degree about 2^{3l}. Gray-code rank-one updates fail: M(x+v) - M(x) = M(v) has rank 3l-1. At best a polynomial factor. |
| 2-descent / E/2E character as a free linear filter | E(K)/2E(K) = Z/2, character P -> Tr(x)+Tr(a); on a decomposition it yields Tr(e1) = Tr(xR), and the leaf's own Tr(s) = Tr(b/t^2) is equivalent given real points. One bit, implied. |
| 2-isogeny descent: decompose phi(R) over a base in E' | one E' triple lifts to 4 triples of R but the base is 4x larger (matrix and relations 4x): per-relation cost 2^{n-2l+2} against 2^{n-2l}. |
| Asymmetric slots (V1 != V2 = V3) | n = l1 + 2l, leaf determined needs 3l-1 <= n: scan 2^{n-2l} >= 2^{n/3}, optimum at l1 = l. |
| Meet in the middle at m = 4 through the leaf as a pair-inversion oracle (Z-relaxed sets) | |Z cap (R - Z)| = 2^{2l-2} relaxed collisions at n = 4l, output-size bound 2^{2l}, no better than the pair scan. |
| F2[z] lattice (Euclid) leaf with Frobenius-sparsity relaxation | relaxed unknowns (2l-1)+(4l-3)+(2l-1) = 8l-5 bits against n = 3l: 2^{5l} spurious. Parity/modular counting of solutions: solution count is Poisson with mean about 2^{3l-n}; computing it mod 2 needs the same enumeration (Valiant-Vazirani gives no cheaper sum). |
| Unconstrained quadratic relaxation of S4 in (e1,e2,e3) bits (V3 small) | 6l-3 unknown bits against n = 3l equations: 2^{3l-3} relaxed solutions against about 1 genuine; consistency (the cubic splits over V) is the hard part. |

## Priority

A first (cheap, a genuinely new reduction for torsion targets, constant gain); then B and C
(component gains, same unit as the lin21-ht table); then E and D (measurements that close open items).
None changes the exponent for a generic target; the exponent question stays open and the pilots'
reading (no arm below the scan) stands for generic R.

# N3-capacity-lp-converse: result (2026-11-01)

Outcome: SUPPORTS, narrowly and CONDITIONAL ON PREMISE P. P is unproved, ASSUMED at m=4 (and at m=3 for l>=2),
certified at small p only for m=2 (l<=3) and at m=3 only in the trivial l=1 case (sum k = 1); one m=3 l=1 cell
(p=13, seed 0) fails P and is undiagnosed. The m=2 point-count rule (#V > 4·Mmax) does NOT transfer to m=3.
[Repair item 4; the original first sentence, superseded, read: "Outcome: SUPPORTS, at accounting level and
conditional."] Given P: inside class C (defined below), no auxiliary-polynomial
support of any shape, at any l, reaches 0.169 at m=4. The ceiling there is 1/32 = 0.03125, which is 5.4x short.
That 1/80 (the box value) is itself optimal at m=4 is NOT shown. At m=2 the only evidence for it is numerical,
for supports inside [0, 2l+4]^2 at l=3 and l=5.
No security claim. Class: accounting. B·D^2 is unchanged.

## A0: the converse (theory; hypotheses checked numerically)
- Sectional lattice. L_l(M) is the set of g in Z[x] supported on M with g ≡ 0 mod p^l on all of U,
  where U = {a in Z_p^m : f(a) ≡ 0 mod p}. Its gain is c = log_p [L_l : p^l Z^M].
- Exact filtration. c(M,l) = Σ_{j=1..l} dim_Fp W_j(M), where W_j(M) = (L_j(M) mod p).
- Premise P (saturation). W_j ⊆ f̄^j F_p[x].
- Bound on each layer. Fix a graded order with LM(f) = (d,..,d), so LM(f^j h) = jL + LM(h). The lead monomials of
  a basis of W_j are distinct, and each satisfies s ≥ jL. Hence dim W_j ≤ #{s in M : s ≥ jL}.
- Summing over j: c ≤ Σ_{s in M} k(s), with k(s) = min(l, min_i ⌊s_i/d⌋). This holds for ANY finite M,
  including non-downset supports.
- Reach. δ ≤ Σk/Σ|s| ≤ max_s k(s)/|s| = 1/totdeg(f) = 1/(md).
  The values are 1/4, 1/12 and 1/32 for m = 2, 3, 4.
- Weighted bounds X_i = X^{w_i} give 1/deg_w(f).
- Hypotheses verified in raw/armA0.out (verbatim output of the runnable armA0_v2.gp; repair item 2), for S3, S4 and
  S5 specialised mod nextprime(2^61), with S5 built by resultant in PARI. Monomial counts 9, 125, 6561; each
  top-degree form is the single monomial x^(d..d) with coefficient 1:
  - per-variable degree d;
  - x^(d..d) is the unique top-total-degree monomial, with coefficient 1 (total degrees 4, 12, 32);
  - the constant term is nonzero.
- Novelty [repair item 3; the original line, superseded, read "novelty_unverified ... likely folklore ...; no
  search was done"]. Prior art inside the programme: the ceiling for lattices INSIDE (p,f)^l is a re-derivation of
  W10 / MAP_closed (strategy-independent ceiling for (p,f)^t lattices on convex supports, 1/32 at m=4; REPORT §3.3)
  and of MP-lattices-1 (leading-monomial / Hilbert-function ceiling for every full-rank lattice in (p,f)^l on ANY
  monomial set; REPORT §3, "1/32 ceiling is a 5-line theorem"). Externally the bound is folklore-shaped
  ("Coppersmith reach ≤ 1/deg f"). What is new here is only the extension from (p,f)^l to ALL auxiliary
  polynomials vanishing mod p^l on U, and that extension is exactly premise P. So the new content is
  only as strong as P.
- B1 is the LP over k(s) with only the pointwise cap. It returns exactly 1/(md) (0.25, 0.08333, 0.03125), and its
  dual certificate is k ≤ |s|/(md). It adds nothing beyond A0.

## A: sectional p-adic interpolation determinants (premise certification)
Method:
- Kernel of evaluation at sampled points of U mod p^l.
- Exact SNF over Z/p^l, with valuation pivoting.
- Points are added by doubling until 3 successive values agree.
- Sampling gives an upper bound on c, so equality with Σk certifies the cell.

m=2 (S3), 375 cells (240 pre-registered + 135 post hoc): seeds 0..2, p from 3 to 53, l = 1..3, boxes plus three non-downset shapes.
- When #V(F_p)_aff > 4·Mmax, every box cell has c = Σk(s) exactly.
  - This holds at all p ≥ 43 and at most cells for p = 29..41.
  - Calibration matches the closed form: 10, 34, 35, 83.
- When #V(F_p)_aff ≤ 4·Mmax, the cell is above Σk, by roughly 32 − #pts:
  - npts 26 gives +6, 28 gives +4, 30 gives +2, 31 or 32 gives +1;
  - all 8 such cells with p ≥ 29 are l=3, box8. [Repair item 1: originally "all 7"; the pre-registered cell
    seed 1, p=31, l=3, box8, npts=30, excess +2 was omitted. The other 7 come from the post-hoc m2hp run, whose curves
    differ from the pre-registered ones at the same (seed, p): seed 1, p=31 has 31 points there, 30 here.]
  - Diagnosis (raw/diag.out): the excess sits entirely at level j=1 (50 vs 49). The curve has no
    singular points.
  - This is the Riemann–Roch artifact: 4·Mmax is the degree of O(M,M) on the genus-1 (2,2) curve, and too few
    F_p-points cannot impose independent conditions. It is irrelevant at p ~ 2^256.
- Pre-registered A1 ("equality for p > Mmax") is FALSIFIED as stated. Its threshold was wrong. The corrected,
  empirical threshold is #V(F_p) > deg O(M)|_C, which is post hoc.
- Out-of-sample check of the corrected rule, post hoc: raw/check_threshold_m2.json applies it to the 240
  pre-registered cells.
  - Cells with npts > 4·Mmax: every box cell is equal except one, seed 1, p=13, l=1, box2 (+3).
  - That curve has rhs(x_R) = 0, so R is 2-torsion. f̄ is then a perfect square and all 12 points are singular
    (raw/diag2.out). This is the degenerate instance excluded by the smoothness argument.
  - Four cells with npts = 16 ≤ 4·Mmax come out equal, so the rule is sufficient, not necessary.
  - Corrected statement: on smooth curves, every box cell with npts > 4·Mmax is equal.
  - This rule is m=2-only. It does NOT transfer to m=3 (repair item 4). The m=3 p=13 seed-0 surface fails
    with 181 points, while p=11 (149 points) passes. The passing m=3 cells also have many singular points
    (55 at p=11, 91 at p=17), so neither smoothness nor a point-count threshold explains the failure.
- A2 (p = 3, 5, 7, plus the excess at p = 11..23) confirmed the small-p artifacts.
- Box equality certifies P on every sub-support. The triangular rows give dim W_j(box) ≥ N_j, and c = Σk forces
  equality at every level. So W_j(box) is its triangular part, which lies in f̄^j F_p[x]. Since W_j(M) ⊆ W_j(box)
  for M ⊆ box, the saturated cells certify c ≤ Σk for EVERY support inside [0, 2l+2]^2 (l ≤ 3), with no assumption.
  The non-box A3 cells are therefore consistency checks, not the evidence.
- A3: in every saturated cell, non-downset supports satisfy c ≤ Σk. They are often far below: holes8 at l=3 gives
  42 against 65.
- No cell with p in the saturated regime beats Σk.

m=3 (S4):
- l=1 box4: equal at p = 11 and 17; above at p = 5, 7 and 13 (seed 0).
- The p=13 cell (gain 3 against 1, with p > Mmax) is where the pre-registered falsifier fired.
- Follow-up, post hoc (raw/armA_m3_extra.json): two other p=13 curves (npts 253) and p = 19, 23, 29 are all equal.
  So the p=13 excess is isolated to that one curve, and its cause was not diagnosed.
- Note that l=1 with Σk = 1 is a near-minimal check.
- l=2: only p=5 ran, an artifact (1213 against 126). Saturating p (around 29 or more) was NOT run: the p=5 cell took
  about 9 CPU-minutes, and the 600 s guard skipped the rest.

m=4: not run, as pre-registered. The l=1 box has 6561 columns and its reach is 0.

So premise P is certified at m=2 (l ≤ 3, p in the saturated regime) and at m=3, l=1. The l=1 case is trivial:
Σk = 1, and the only element is f itself, so P's content (j ≥ 2) is untested at m=3. One m=3 l=1 cell (p=13, seed 0)
fails and is undiagnosed. P is ASSUMED at m=3 l ≥ 2 and at m=4, and it is unproved.

## B: exact search over supports (p = 2^31 − 1, the real specialised S3/S4, not a generic stand-in)
- B0 calibration, exact Dinkelbach plus DP over staircases. It reproduces the audit: 0.141287 (l=5),
  0.152667 (l=10), 0.161584 (l=30). The box closed forms match.
- B2 m=2, score ρ_D(M) = Σ_j D_j(M)/Σ|s|, where D_j = dim{h : supp(f̄^j h) ⊆ M}. Under P this is a valid upper bound
  on δ(M,l) for any M.
  - Families: boxes, rectangles, cut corners, cut tops, diamonds, hollow shapes, the staircase optimum.
  - Local search, pre-registered: simulated annealing, seed 0, 2000 moves, started from the staircase optimum.
  - Local search, post hoc: seeds 1..3, started from random and from diamond shapes.
  - Result: max ρ_D − staircase optimum = 0.0 at l=3 (0.128889) and at l=5 (0.141287).
  - The post-hoc SA best gap is −0.00079.
  - D_j ≤ N_j held in every evaluation (asserted).
- The per-monomial count Σk is loose for non-downsets. Diamond0.6 at l=5 has Σk/Σ|s| = 0.164, but ρ_D = 0.098.
  The real algebra (D_j) is what blocks those shapes.
- B3 m=3: boxes have ρ_D = Σk ratio exactly (0.01440 at l=2). No other shapes were run at m=3.
- Prediction B2 held: no counterexample was found. But the search is LOW-POWER: the post-hoc SA from random starts
  never even got back to the known staircase optimum (best gap −0.0008).
- This is weak evidence for downset-optimality at l ≤ 5 inside [0, 2l+4]^2, not a proof.

## Class C covered by the converse
- One polynomial f and one modulus p^l.
- Auxiliary polynomials vanish mod p^l on ALL of U. They are root-oblivious, so this covers every combination of
  p^{l−k}x^j f^k and anything else that vanishes on U, including polynomials outside (p,f)^l. Any support,
  downset or not, and any number of generators.
- Minkowski / determinant (HG) accounting with uniform bounds (or fixed weights: 1/deg_w f).
- p above the saturation threshold.
- Premise P (certified only where stated above).

## Not covered
- λ_1 beating det^{1/n}. MP-lattices-2 saw LLL reach 0.11 against a determinant estimate of 0.0955, because many
  vectors vanish over Z.
- Auxiliary polynomials that use instance-specific information.
- Several moduli or ideals beyond U, e.g. several congruences sharing a root.
- Changes of variables that change the archimedean geometry: unravelled linearisation, invariants whose bounds are
  not induced from x. For e-variables with e_i ≤ X^i the weighted bound is also 1/32, but P in Z[e] is unverified.
- Non-lattice methods.
- Small p.
- m=4 optimality of 1/80 (only the 1/32 ceiling is shown).

## Files
- PROTOCOL.md: pre-registered.
- n3lib.py.
- armA.py: m2, m2hp (post hoc), m3.
- armA0.gp.
- armB.py: B0, B2, B3.
- armB2_posthoc.py.
- diag.py.
- raw/*.json, raw/*.log.
- RESULTS.json.
- armA m2hp was killed by its 1200 s timeout at seed 2, p=37. Its JSON was rebuilt from the log, with point counts
  recomputed.

CPU: about 60 min in total, at most 3 processes.

## Next step
- Run m=3, l=2 sectional at p ≥ 29, using C/M4RI-style elimination over Z/p^l, to certify P beyond l=1.
- Write a proof of P for p ≫ deg. Hensel on smooth balls plus Riemann–Roch looks sufficient.
- Then the only open part of the m=4 question is whether 1/80 or 1/32 is the right ceiling within C, which no
  longer affects the 0.169 decision.

## Repair (2026-11-01; audit must_fix items 1-4 only, per user request "do 1-4")
Original numbers have not been deleted; superseded text is marked inline.
1. Missing excess cell. Added seed 1, p=31, l=3, box8, npts=30, +2, and changed "all 7" to "all 8". RESULTS
   `m2_box_above_when_npts_le_4Mmax` now has 8 entries, with the added one listed last. A provenance note
   explains why the npts differ from the m2hp entry at the same (seed, p).
2. A0 evidence. The old raw/armA0.out was elided, not verbatim. It is kept as
   raw/armA0.out.SUPERSEDED_not_verbatim. The new armA0_v2.gp sets parisizemax, enumerates every nonzero monomial
   and prints the full top-degree form. raw/armA0.out is now its verbatim stdout. S3/S4/S5 have 9/125/6561
   monomials, per-variable degrees 2/4/8, and top total degrees 4/12/32. Each top form is the single monomial
   x^(d..d) with coefficient 1, and the constant terms are nonzero and identical to the old run. armA0.gp is
   left as-is; on S5 it overflows the default PARI stack.
3. Prior art. Cited W10 / MAP_closed (1/32 at m=4) and MP-lattices-1. Stated that the only new content is the
   extension to all g vanishing on U, which is P. Dropped "no search was done".
4. Conditionality. The first sentence and RESULTS `outcome` now say the m=4 converse is conditional on P: P is
   unproved, assumed at m=4, certified at m=3 only in the trivial l=1 case, and the m=3 p=13 seed-0 failure is
   undiagnosed. Stated that the m=2 point-count rule does not transfer to m=3.
Items 5-8 (B2 disclosure and power, CPU and threads, the e-variable claim, diag.out labels, m2hp completeness,
__pycache__) are NOT addressed in this repair. They remain open as listed in AUDIT.md.

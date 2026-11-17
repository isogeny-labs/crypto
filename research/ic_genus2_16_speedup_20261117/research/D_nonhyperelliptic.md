# D. Index calculus in Jacobians of non-hyperelliptic curves (genus 3..16)

Status: IN PROGRESS (outline written first; sections appended as research proceeds).
Date: 2026-11-17. All claims are `[pcc:unverified]` literature assertions unless noted;
items marked UNVERIFIED were not checked against a primary source in this session.

## Outline
0. Notation and baseline (rho ~ q^(g/2); hyperelliptic IC exponent 2 - 2/g)
1. Diem 2006: plane curves of small degree, O~(q^(2-2/(d-2))); genus 3 quartics O~(q)
   1a. Diem-Thome 2008 genus-3 quartic practical algorithm and experiments
2. Diem general non-hyperelliptic genus g: exponent 2 - 2/(g-1)
3. Hess: Riemann-Roch relation generation for arbitrary curves
4. L(1/3): Enge-Gaudry-Thome (C_ab), Enge-Gaudry, Couveignes, Diem L(1/3)
5. Smith 2008 isogeny hyperelliptic g=3 -> quartic; genus 4+ analogues (Recillas, Donagi, Pryms, gluing)
6. Special structure: gonality, nodal plane models, C_ab, superelliptic, Picard, automorphisms
7. Canonical-curve methods g=4..16 vs low-degree plane models
8. Record computations
9. 2015-2026 work including negative results
10. Biggest speed-up levers by regime (table)
11. References

## Verification note (method)
In this session `eprint.iacr.org` and `www.math.uni-leipzig.de` were not resolvable from the
sandbox (network allowlist), so primary PDFs could not be fetched. Statements below were
checked against search-engine renderings of abstracts/snippets of the primary sources
(marked **[abstract-checked]**) or are from the author's prior knowledge of the papers
(marked **[UNVERIFIED-memory]**). The `research-kb` MCP server and `scripts/research-kb`
fallback were not available in this checkout (no `source_id` recorded). Every claim is
`[pcc:unverified]` in the lab's sense.

## 0. Notation and baseline
- Jacobian J_C(F_q) of a genus-g curve, #J ~ q^g; Pollard rho costs ~ q^(g/2) group ops
  (sqrt(pi q^g /2) for a prime-order group; automorphism/negation speed-ups give constant factors).
- Hyperelliptic IC with double large primes (Gaudry-Thome-Theriault-Diem 2007, Nagao 2007):
  O~(q^(2-2/g)) [abstract-checked via Diem 2011 Math. Comp. snippets; see refs].
- Non-hyperelliptic curves with small plane models beat this by shrinking the "residual" part
  of each relation: a relation comes from a function of low degree (a line), not from the
  degree-g reduced Mumford representation.

## 1. Diem 2006: plane curves of small degree
**Source.** C. Diem, "An index calculus algorithm for plane curves of small degree", ANTS-VII,
LNCS 4076 (2006) 543-557, doi:10.1007/11792086_38, https://link.springer.com/chapter/10.1007/11792086_38 ;
preprint "Index Calculus in Class Groups of Plane Curves of Small Degree", IACR ePrint 2005/119,
https://eprint.iacr.org/2005/119 .

**Statement [abstract-checked].** Heuristically, for curves given by plane models of fixed
degree d over F_q, the DLP in Cl^0 can be solved in expected time O~(q^(2-2/(d-2))) for
"essentially all" instances. For d = 4 (non-hyperelliptic genus 3, canonical plane quartic)
this is O~(q). The result is HEURISTIC (the ePrint abstract says "heuristically"; Diem's
later rigorous work in Math. Comp. 2011 covers other cases, see section 2).

**Relation mechanism [UNVERIFIED-memory, consistent with abstracts].**
- Factor base F = a subset of the F_q-rational points of the (smooth part of the) plane model,
  of size ~ q^r with r < 1; all other rational points are "large primes".
- Draw the line L through two points P1, P2 (factor-base or large-prime points). By Bezout,
  L . C = P1 + P2 + R where R is an effective residual divisor of degree d-2.
- A relation is recorded when R splits completely over F_q into rational points, and the total
  number of non-factor-base points among the d points is at most 2 (double large prime).
- div(l/l0) = L.C - L0.C for a fixed reference line L0, so each split line gives a relation
  in Cl^0 among points (the "hyperplane class" cancels).
- Splitting probability of the residual: a random degree-(d-2) effective divisor splits
  into rational points with probability ~ 1/(d-2)!; for quartics R has degree 2 and splits
  with probability ~ 1/2.
- Why the exponent is 2-2/(d-2): the effective "degree that must be smooth" is d-2 instead
  of g. In the hyperelliptic double-large-prime analysis the exponent is 2-2/g where g is
  the number of points that must land in the factor base; replacing g by d-2 gives
  2-2/(d-2). The balancing is the same graph (large-prime graph) argument:
  factor base size q^(1-1/(d-2)), relation collection ~ q^(2-2/(d-2)), sparse linear algebra
  ~ (q^(1-1/(d-2)))^2 = q^(2-2/(d-2)).
- For genus 3 quartics: d-2 = 2, factor base ~ q^(1/2), exponent 1. Compare hyperelliptic
  genus 3: 2-2/3 = 4/3. Rho for genus 3: q^(3/2). So quartic IC is q^(1/2) below rho.

### 1a. Diem-Thome 2008, genus-3 quartics in practice
**Source.** C. Diem, E. Thome, "Index calculus in class groups of non-hyperelliptic curves of
genus three", J. Cryptology 21(4):593-611 (2008), doi:10.1007/s00145-007-9014-6,
https://link.springer.com/article/10.1007/s00145-007-9014-6 ; HAL inria-00107290.
**[abstract-checked]** heuristic expected time O~(q) for DLP in Cl^0 of non-hyperelliptic
genus-3 curves; a Diem SHARCS'06 talk gives storage Theta(q^(3/4))... (search snippet; storage
figure UNVERIFIED against paper) http://www.hyperelliptic.org/tanja/SHARCS/talks06/Claus_Diem.pdf .
**[UNVERIFIED-memory]** The paper includes an implementation (relation search by lines through
pairs of points, large-prime graph with tree-building to eliminate large primes, structured
Gaussian elimination + Lanczos) and an experimental DLP in a genus-3 Jacobian of size
~2^(?)x; exact record size not confirmed in this session.

### 1b. Rigorous version for plane curves (Diem, JTNB 2012)
**Source.** C. Diem, "On the discrete logarithm problem for plane curves", J. Theorie des Nombres
de Bordeaux 24(3):639-667 (2012), doi:10.5802/jtnb.815, https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/ .
**[abstract-checked]** DLP for non-hyperelliptic genus-3 curves given by plane quartics:
expected time O~(q). More generally, for plane models of degree d containing a line that cuts
out d distinct F_q-rational points, expected time O~(q^(2-2/(d-2))); this holds in particular
for "reflexive" plane models. The abstract's phrasing ("can be solved in an expected time")
is that of a proven theorem for a randomized algorithm (the 2005/2006 versions were heuristic);
UNVERIFIED that no heuristic remains, since the full text could not be fetched.
Practical reading: the exponent depends only on the plane degree d, NOT on the genus. A plane
model with nodes (geometric genus g < (d-1)(d-2)/2) still has exponent 2-2/(d-2), so the
lever is "smallest-degree plane model", which is what makes singular models attractive (sec. 6).

## 2. General non-hyperelliptic genus g: exponent 2 - 2/(g-1)
**Sources.**
- Diem, ePrint 2005/119 / ANTS-VII 2006 (above): every "sufficiently general" non-hyperelliptic
  curve of genus g >= 3 has a plane model of degree g+1 (projection of the canonical model
  from g-3 points on the curve; a birational, generally nodal, plane model), so d-2 = g-1 and
  the exponent is **2 - 2/(g-1)**. **[abstract-checked]** via citations in Diem-Kochinke and
  in Galbraith-Gaudry style surveys: "running time roughly q^(2-2/(g-1)) for sufficiently
  general non-hyperelliptic curves of genus g >= 3"; a later generalisation (Diem, cited as
  [20] in the same source) to all sufficiently general non-hyperelliptic curves with
  q^(2-2/(g-1)+eps).
- Diem, "On the discrete logarithm problem in class groups of curves", Math. Comp. 80 (2011)
  443-475, doi:10.1090/S0025-5718-2010-02281-1,
  https://www.ams.org/mcom/2011-80-273/S0025-5718-2010-02281-1/ . **[abstract-checked]**
  RIGOROUS: for fixed g >= 2, DLP in Cl^0 of curves of genus g can be solved in expected time
  O~(q^(2-2/g)) (all curves, any model). This is the proven generic bound; the non-hyperelliptic
  improvement 2-2/(g-1) is heuristic in the 2005/2006 form.
- Diem, arXiv math/0606607, "Index calculus with double large prime variation for curves of
  small genus with cyclic class group", https://arxiv.org/abs/math/0606607 . **[abstract-checked]**
  O~(q^(2-2/g)) rigorous when the curve is given by a plane model of bounded degree and Cl^0 is
  cyclic; recommends constructing a plane model of degree g+1 or smaller.
- Oyono & Theriault, "Index calculus algorithm for non-planar curves", Finite Fields Appl.
  (2023) 102227, doi:10.1016/j.ffa.2023.102227,
  https://www.sciencedirect.com/science/article/abs/pii/S1071579723000692 . **[abstract-checked]**
  Works directly on the canonical model (degree 2g-2 in P^(g-1)) with hyperplanes through
  g-1 factor-base points; upper bound O(q^(2-2/(g-1)+eps)) for ALL non-hyperelliptic curves of
  genus g (removes the "sufficiently general" condition), i.e. asymptotically the same as Diem's
  planar algorithm. Relation search stops after ~B^2 hyperplanes (B = factor base size); three
  hyperplane-selection strategies; also adapts Nagao-style graph (fixed factor base) and
  "algebraic curve sieving" to the non-planar setting.

Why 2-2/(g-1) (canonical view): a hyperplane H in P^(g-1) through g-1 chosen points meets the
canonical curve in 2g-2 points (Bezout); the residual has degree (2g-2)-(g-1) = g-1. So
g-1 points must be smooth, exactly as the plane model of degree g+1 (line through 2 points,
residual g+1-2 = g-1). Exponent 2 - 2/(residual degree) = 2 - 2/(g-1).
Conditions: non-hyperelliptic (canonical map is an embedding); g >= 3 fixed; q -> infinity.
Status: heuristic (Diem 2006), heuristic-to-rigorous upper bound with +eps (Oyono-Theriault 2023,
abstract says "we obtain an upper bound"; rigor level UNVERIFIED).

### 2a. Beyond 2-2/(g-1): pencils / special linear systems (Diem-Kochinke)
**Sources.** C. Diem, S. Kochinke, "Computing discrete logarithms with pencils", preprint,
http://www.math.uni-leipzig.de/MI/diem/preprints/pencils.pdf ; expanded as "Computing discrete
logarithms with special linear systems", preprint (2013),
https://www.math.uni-leipzig.de/~diem/preprints/dlp-linear-systems.pdf ; Semantic Scholar entry
https://www.semanticscholar.org/paper/6e74dbf3ffed71cb0adabf3a9aae5d37a6480e64 .
**[abstract-checked]** Heuristic: for nearly all curves of fixed genus g >= 5 over F_q, DLP in
Cl^0 in expected time **O~(q^(2-2/(g-2)))**. Uses a family of pencils (special linear systems)
on the curve; the authors show the special-linear-system approach fails for hyperelliptic
curves. Publication venue not confirmed (preprint as of the search).
Mechanism [UNVERIFIED-memory/inference]: choose a linear system whose members through
prescribed factor-base points have residual degree g-2 rather than g-1 (one more degree of
freedom than the canonical system gives), e.g. using g^1/g^2 special divisors (Brill-Noether:
a general curve of genus g has a g^2_d with d = ceil(2g/3)+2, so plane models of degree
~2g/3+2 exist, but these have many nodes and their lines give residual d-2 ~ 2g/3).
Consequence: for g >= 5 the best known heuristic exponent is 2-2/(g-2), still well below
rho's g/2 for g >= 5 (e.g. g=5: 4/3 vs 2.5; g=8: 5/3 vs 4).

NOTE on low-degree plane models. If a curve has a (singular) plane model of degree d with
d-2 < g-2, Diem's plane-curve algorithm (sec. 1b) gives 2-2/(d-2), better than the pencil
bound. A general curve of genus g has minimal plane degree d_min = ceil(2g/3)+2 (Brill-Noether,
g^2_d exists iff rho(g,2,d) = g - 3(g-d+2) >= 0), so for general curves the plane route gives
exponent 2-2/(ceil(2g/3)) [inference, not a published theorem; the hard part is constructing
such a model over F_q and dealing with singular points and rationality of the g^2_d]. For
g=6: d=6 (plane sextic with 4 nodes), residual 4, exponent 3/2 vs 2-2/(g-1)=8/5 and
2-2/(g-2)=3/2 -- equal. For g=9: d=8, residual 6, exponent 5/3 vs pencils 2-2/7=12/7.
Such models need a rational g^2_d over F_q, which is not guaranteed.

General principle (inference, worth testing): for a linear system g^r_d on C, forcing r
factor-base points gives one divisor whose residual has degree e = d - r; there are ~B^r
candidate relations, and double-large-prime IC with residual e has exponent 2 - 2/e (needs
r >= 2 so that B^r candidates suffice). Brill-Noether: a general curve has a g^r_d iff
g - (r+1)(g-d+r) >= 0, i.e. e >= g*r/(r+1). Minimum over r >= 2 is r = 2, e = ceil(2g/3).
So for GENERAL curves the floor of this single-linear-system family (NOT of all methods:
Diem-Kochinke's family of pencils, which varies the linear system over a positive-dimensional
W^1_d, beats it at g = 5) is exponent 2 - 2/ceil(2g/3) -- if one can (a) find an F_q-rational g^2_d with d = ceil(2g/3)+2
and (b) run IC on the resulting singular plane model. Canonical (r = g-1) gives e = g-1;
Diem-Kochinke pencils give e = g-2. Exponents by g (canonical / pencils / min plane):
g=4: 4/3, -, 4/3 | g=5: 3/2, 4/3, 3/2 | g=6: 8/5, 3/2, 3/2 | g=7: 5/3, 8/5, 8/5 |
g=8: 12/7, 5/3, 5/3 | g=9: 7/4, 12/7, 5/3 | g=12: 20/11, 9/5, 7/4 | g=16: 28/15, 13/7, 20/11
[arithmetic by this note; the "min plane" column is NOT a published theorem].
Remark: for g >= 9 the min-plane column beats Diem-Kochinke's 2-2/(g-2); an F_q-rational
g^2_d exists for many curves when rho(g,2,d) > 0 (positive-dimensional W^2_d), but
computing one is the bottleneck. This is a concrete research lever (sec. 10).

## 3. Hess: Riemann-Roch based relation generation for arbitrary curves
**Sources.** F. Hess, "Computing relations in divisor class groups of algebraic curves over
finite fields", preprint ~2004 (cited in Enge-Gaudry-Thome 2011 and Derickx-Terao 2026 as the
standard index-calculus-style method; listed as "submitted to J. Symbolic Comp."), no open
copy located in this session (UNVERIFIED content). F. Hess, "Computing Riemann-Roch spaces in
algebraic function fields and related topics", J. Symbolic Comput. 33 (2002) 425-445.
**Content [UNVERIFIED-memory + abstract-level confirmation via citing papers].**
- Works for any curve given by a function field (any plane model, singular or not), using
  ideal arithmetic in the maximal orders and Riemann-Roch space computations.
- Relation generation: take a random divisor D = sum of factor-base places plus a random
  combination of the target, compute a basis of L(D + kP_inf) (or reduce D to a reduced
  representative D - g*P_inf + (f)), and test whether the resulting effective divisor of
  degree ~g factors over the factor base of places of degree <= b. The splitting test is a
  smoothness test of a norm polynomial.
- Complexity: for fixed genus and growing q this is the generic "degree-g residual" method,
  exponent 2 - 2/g with double large primes (as in Diem's rigorous Math. Comp. 2011 bound,
  whose algorithm uses a similar Riemann-Roch representation). For large genus relative to
  log q (g >> log q) it is subexponential L_{q^g}(1/2) (Hess's main theoretical result,
  extending Heß/Enge-Stein style results to arbitrary curves). Not competitive with the
  line/hyperplane methods for fixed g >= 3 because the residual degree is g rather than g-1
  (canonical), g-2 (pencils) or d-2 (plane).
- Practicality: each relation needs a Riemann-Roch computation (polynomial in g, log q but
  heavy: dense linear algebra over F_q[x]); line-based methods need only one univariate
  polynomial (the restriction of C to a line, degree d-2 after dividing out known roots) and a
  root/splitting test. Line methods are ~10^2-10^3 times cheaper per candidate in practice
  [UNVERIFIED-estimate].
- Modern implementation: Derickx & Terao, "Computing class groups and gonalities of algebraic
  curves over finite fields", ANTS XVII (2026) / arXiv 2602.17417,
  https://arxiv.org/abs/2602.17417 , code "CurveArith". **[abstract-checked]** practical
  algorithms for class group and gonality of curves over finite fields based on Hess's
  index-calculus-style method, "speedups of several orders of magnitude" over existing methods,
  mainly for large genus or residue field. Relevant as engineering for small q / large g
  (class group computation, not DLP records). Also Macri-Jacobson-Scheidler, "Improvements to
  Jacobian arithmetic in global function fields", WAIFI 2026, LNCS 16611:111-128, arXiv 2605.15323,
  https://arxiv.org/abs/2605.15323 (faster Hess-style reduction; group-law speed matters for
  rho baselines and for the "descent" phase of IC).

## 4. L(1/3) algorithms (Enge-Gaudry, Enge-Gaudry-Thome, Couveignes, Diem)
**Sources.**
- A. Enge, P. Gaudry, "An L(1/3+eps) algorithm for the discrete logarithm problem for low
  degree curves", EUROCRYPT 2007, LNCS 4515:379-393, arXiv cs/0703032,
  https://arxiv.org/abs/cs/0703032 . **[abstract-checked]** group structure in
  L_{q^g}(1/3, O(1)); individual DL in L_{q^g}(1/3+eps, o(1)).
- A. Enge, P. Gaudry, E. Thome, "An L(1/3) discrete logarithm algorithm for low degree curves",
  J. Cryptology 24 (2011) 24-41, doi:10.1007/s00145-010-9057-y, arXiv 0905.2177,
  https://arxiv.org/abs/0905.2177 . **[abstract-checked]** families of plane curves whose
  degrees in X and Y are low w.r.t. genus; base fields arbitrary but q must not grow too fast
  relative to g; heuristic L_{q^g}(1/3, O(1)) for both group structure and DL. Per a search
  snippet of the paper: if deg_X(C_i) * deg_Y(C_i) <= kappa * g_i, then complexity
  L(1/3, (64 kappa/9)^(1/3)) [snippet-checked]. **[UNVERIFIED-memory]** precise shape: d_X ~
  g^(2/3)/(log q)^(...) and d_Y ~ g^(1/3) (n = deg_Y ~ alpha g^(1/3) (log q)^(-1/3)-type
  balancing), i.e. C_ab curves with a ~ g^(1/3), b ~ g^(2/3) (C_ab genus (a-1)(b-1)/2); the
  relation search uses functions of bounded degree in Y (not lines) and a special-q-like
  descent for individual logs.
- J.-M. Couveignes, "Algebraic groups and discrete logarithm", in Public-key cryptography and
  computational number theory, de Gruyter (2001) 17-27. [UNVERIFIED content] Earlier
  L(1/3)-type heuristic for curves of large genus with suitable plane models; EG2007 cite it as
  an algorithm "based on similar ideas".
- C. Diem, "On the discrete logarithm problem in elliptic curves" is unrelated; Diem's
  L(1/3) statement for plane curves: UNVERIFIED (not found in this session).
**Relevance to g = 3..16.** None in practice. L(1/3) needs g -> infinity with log q = o(g^c)
(q small relative to g); at fixed g and growing q, L_{q^g}(1/3) is worse than q^(2-2/(g-1))
for all cryptographically relevant q. E.g. g=16, q=2^16 (group 2^256): an L(1/3) estimate
is far above 2^32-ish polynomial-time IC costs. The relevant asymptotic regime for the lab is
fixed g, q -> infinity, where the q^(2-2/e) family rules. The idea that transfers is the
*shape*: low degree in Y means fibre/degree constraints on relations; C_ab curves with small a
are trigonal/tetragonal and have small plane degree, which matters for sec. 6.

## 5. Isogenies from hyperelliptic to non-hyperelliptic Jacobians
### 5a. Smith 2008 (genus 3, (2,2,2)-isogenies)
**Source.** B. Smith, "Isogenies and the discrete logarithm problem in Jacobians of genus 3
hyperelliptic curves", EUROCRYPT 2008, LNCS 4965:163-180; full version J. Cryptology 22(4)
(2009) 505-529, doi:10.1007/s00145-009-9038-1; arXiv 0806.2995, https://arxiv.org/abs/0806.2995 ;
ePrint 2007/428. **[abstract-checked]**
- Explicit formulae for isogenies with kernel isomorphic to (Z/2Z)^3 (over F_qbar) from any
  hyperelliptic genus-3 Jacobian, char != 2,3, built from Recillas' trigonal construction
  (Donagi-Livne: a trigonal map for the subgroup S gives S as kernel of J_H -> J_X).
- The isogenies are F_q-rational for a positive fraction of curves: "around 18.57% of all
  hyperelliptic genus 3 curves over a given finite field" (subject to reasonable assumptions).
- Target X is generically a non-hyperelliptic genus-3 curve (plane quartic), so DLP cost
  drops from O~(q^(4/3)) (hyperelliptic double large prime) to O~(q) (Diem quartic IC).
- Cost of the transfer: polynomial in log q (explicit formulae; Frey-Kani say the
  correspondence is computable in polynomial time in log q) -- negligible vs. IC.
  **[snippet-checked]**
- The 18.57% comes from the probability that a suitable rational kernel/trigonal datum
  exists (a combinatorial count over Galois actions on the 2-torsion, i.e. on the 8 Weierstrass
  points) [UNVERIFIED-memory for the derivation].
### 5b. Frey-Kani (2011)
G. Frey, E. Kani, "Correspondences on hyperelliptic curves and applications to the discrete
logarithm", SIIS 2011, LNCS 7053, https://www.mast.queensu.ca/~kani/papers/fkwarsawarticlerevised.pdf .
**[abstract-checked]** Smith's result derived from Hurwitz spaces of covers of P^1 with
monodromy S4; valid for odd characteristic; in characteristic 2 applies at least to curves
with ordinary Jacobian. Recommends avoiding genus >= 4 and non-hyperelliptic genus 3 for crypto.
### 5c. Tian 2021 ((l,l,l)-isogenies)
S. Tian, "Translating the discrete logarithm problem on Jacobians of genus 3 hyperelliptic
curves with (l,l,l)-isogenies", J. Cryptology 34(3):32 (2021), doi:10.1007/s00145-021-09401-3,
arXiv 2007.03172, https://arxiv.org/abs/2007.03172 . **[abstract-checked]** computes
(l,l,l)-isogenies from genus-3 hyperelliptic Jacobians to non-hyperelliptic Jacobians, l odd
prime coprime to p, in time O~(l^3). Extends coverage beyond Smith's 18.57%: a curve whose
Jacobian has a rational (l,l,l)-kernel for some small l can be moved. Fraction of curves
covered for small l: UNVERIFIED (not in snippets). Since the generic isogeny class of a
hyperelliptic genus-3 Jacobian contains non-hyperelliptic Jacobians (hyperelliptic locus has
codimension 1 in M_3, dimension 5 of 6), the obstacle is only finding a short rational path.
### 5d. CM constructions
B. Dina, S. Ionica, "Isogenous hyperelliptic and non-hyperelliptic Jacobians with maximal
complex multiplication", Math. Comp. 92 (2023) 349-383, doi:10.1090/mcom/3776, arXiv 2104.04919,
https://arxiv.org/abs/2104.04919 . **[abstract-checked]** finds the sextic CM fields (14 of
547,156 in LMFDB) for which both hyperelliptic and non-hyperelliptic Jacobians with CM by Z_K
(heuristically) exist. Relevance: CM-constructed genus-3 hyperelliptic targets.
### 5e. Genus >= 4: a negative result
J.C. Naranjo, G.P. Pirola, "Hyperelliptic Jacobians and isogenies", Adv. Math. (2018),
doi:10.1016/j.aim.2018.07.025, arXiv 1705.10154, https://arxiv.org/abs/1705.10154 .
**[abstract-checked]** A very general hyperelliptic Jacobian of genus g >= 4 is NOT isogenous
to a non-hyperelliptic Jacobian. So the Smith route has no generic analogue for g >= 4:
only special (non-very-general, e.g. with extra endomorphisms or decomposable Jacobians)
hyperelliptic curves can be moved. Dimension count explains it: the hyperelliptic locus has
dimension 2g-1, M_g has 3g-3, A_g has g(g+1)/2; for g >= 4 Jacobians are a proper subvariety
of A_g and a countable union of isogeny-translates of a (2g-1)-dim locus generically misses
the Jacobian locus.
Prym / Recillas / Donagi tetrahedral: Recillas' trigonal construction identifies Jacobians of
TETRAGONAL genus-g curves with Pryms of etale double covers of TRIGONAL genus-(g+1) curves
(Jac(X) = Prym(C~/C); N. Bruin slides, ICERM 2019,
https://app.icerm.brown.edu/materials/Slides/sc19-1-aldav/Prym_varieties_of_genus_four_curves_]_Nils_Bruin,_Simon_Fraser_University.pdf ).
This is a correspondence between non-hyperelliptic curves, not a route out of the
hyperelliptic locus for g >= 4 (Naranjo-Pirola). Donagi's tetragonal construction relates
three tetragonal curves with isomorphic Pryms -- again non-hyperelliptic to non-hyperelliptic.
For the lab the relevant use is the reverse: moving a non-hyperelliptic DLP to a curve with
a *smaller* plane model or more automorphisms within the same isogeny class
[inference, no published DLP application found].
Decomposable / glued Jacobians: N. Bruin, A. Kulkarni, "On (2,2)-decomposable genus 4
Jacobians", Canad. J. Math. (2025), doi:10.4153/S0008414X25101557, arXiv 2309.01959
**[abstract-checked]** (genus-4 Jacobians 2-isogenous to a square of a genus-2 Jacobian; full
description of the hyperelliptic ones). "Generalized Howe curves of genus 4, 5, and 6 with
completely decomposable Jacobians", arXiv 2604.18074 (2026) (title only checked). Gluing
(Frey-Kani style: two lower-genus Jacobians glued along torsion give a higher-genus Jacobian)
is a route from SMALLER genus to a (often non-hyperelliptic) higher genus; for DLP it goes the
wrong way (makes the instance bigger), except that a split Jacobian lets the DLP be solved in
the factors (Frey-Kani), which is a reduction, not IC.

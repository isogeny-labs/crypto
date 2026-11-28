# D. Index calculus in Jacobians of non-hyperelliptic curves (genus 3..16)

Status: COMPLETE draft (see end of file for gaps).
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

## 6. Curves with special structure that make IC faster
The one parameter that controls the exponent in every fixed-g, q -> infinity method above is
the RESIDUAL DEGREE e (number of points that must land in the factor base once the relation
is pinned through two chosen points): exponent 2 - 2/e. Rho is q^(g/2). Special structure
helps if it lowers e, shrinks the factor base by symmetry, or adds free relations.

### 6a. Small plane models (smooth or nodal)
- Smooth plane curves of degree d have genus (d-1)(d-2)/2 and e = d-2 (Diem 2006/2012):
  d=4: g=3, exponent 1 (rho 1.5); d=5: g=6, exponent 4/3 (rho 3); d=6: g=10, exponent 3/2
  (rho 5); d=7: g=15, exponent 8/5 (rho 7.5). [exponents: direct substitution in the
  abstract-checked Diem formula]. Smooth plane quintics are codimension 3 in M_6, sextics
  codim 8 in M_10, septics codim 15 in M_15 (dimension count: plane curves of degree d,
  (d+1)(d+2)/2 - 1 - 8 moduli, vs 3g-3) [inference].
- Nodal plane models: degree d with delta nodes has geometric genus (d-1)(d-2)/2 - delta and
  the same e = d-2 (JTNB 2012 condition: a line cutting d distinct rational points; singular
  points handled by working on the normalisation). Genus 4: plane quintic with 2 nodes
  (projection of the canonical curve from a point on it), e=3, exponent 4/3. Genus 5: plane
  sextic with 4 nodes (g+1 model) or, for the general curve, Brill-Noether min degree 6 again;
  e=4, exponent 3/2. Genus 6: general curve is a plane sextic with 4 nodes, e=4, 3/2.
  Lines through a node meet the curve at the node with multiplicity 2: such lines must be
  avoided or handled (two branches = two places).
- Brill-Noether minimum plane degree for a general curve d = ceil(2g/3)+2 (see sec. 2a), the
  practical issue being an F_q-rational g^2_d.

### 6b. Gonality: trigonal, tetragonal
- Gonality alone does NOT lower e for the line method. What gonality gives:
  (i) free relations: each completely split fibre of the degree-k map is linearly equivalent
  to the fibre at infinity (~q/k! such relations; the hyperelliptic case k=2 is the usual
  P + iota(P) trick halving the factor base). For k=3 this eliminates ~q/6 of ~q unknowns;
  minor (constant-factor) lever. [inference]
  (ii) trigonal curves have a Maroni-invariant scroll model; the plane model of a
  trigonal genus-g curve of the form y^3 + a(x)y^2 + b(x)y + c(x) has degree about (g+3)/2..g+1
  depending on Maroni invariant [UNVERIFIED-memory], not better than Brill-Noether general.
  (iii) Diem-Kochinke show special-linear-system (pencil) methods FAIL for hyperelliptic curves
  (the g^1_2 composes everything); for trigonal curves the g^1_3 is a special linear system
  that could be used, but its exact payoff is UNVERIFIED.
- C_ab curves (affine y^a + ... + x^b, single place at infinity, genus (a-1)(b-1)/2):
  the semigroup <a,b> controls Riemann-Roch spaces L(n*inf). Using functions in L(n*inf)
  with l(n*inf) = r+1 >= 3, residual e = #gaps of <a,b> up to n. Example C_{3,4} (g=3):
  n=4, functions 1,x,y, gaps {1,2}, e = 2 -- the same as quartic lines. C_{3,5} (g=4):
  n=5 gives 1,x,y (pole orders 0,3,5), gaps {1,2,4}, e = 3, exponent 4/3. C_{4,5} (g=6): n=5
  gives 1,x,y (0,4,5), gaps {1,2,3}, e = 3, exponent 4/3 (C_{4,5} is a plane quintic with a
  singularity at infinity -- consistent with 6a). C_{3,7} (g=6): n=6: 1,x,x^2 is composed with
  the trigonal map (useless); n=7: 1,x,y,x^2 (0,3,6,7) r=3, gaps <=7: {1,2,4,5} e=4; so
  trigonal C_{3,b} are worse than C_{4,5} at the same genus. [semigroup arithmetic by this note;
  the general "gaps up to n" rule is the Riemann-Roch count, not quoted from a paper]
  Lesson: balanced a < b with a ~ b ~ sqrt(2g) minimises e (using L(b*inf) = <1,x,y>,
  e = b-2 = plane degree minus 2). That is the C_ab shape EGT exploit for L(1/3) (deg_X * deg_Y ~ g).
- Superelliptic y^n = f(x), deg f = m: genus ~ (n-1)(m-1)/2 (for coprime n,m); plane degree
  max(n,m); balanced n ~ m ~ sqrt(2g) gives e ~ sqrt(2g) - 2. These are special (very
  low-dimensional moduli) but give the best exponents per genus among explicit families:
  g=10 via y^5=f(x), deg 6? (genus (4)(5)/2=10, plane degree 6, e=4, exponent 3/2) [inference].
- Picard curves y^3 = x^4 + ... (genus 3, C_{3,4}): already quartics, e=2, O~(q); extra
  automorphism of order 3 when q = 1 mod 3 (6c).

### 6c. Automorphisms and Frobenius (orbit reduction)
- If an automorphism group G (defined over F_q) acts on the prime-order subgroup by scalars,
  the factor base can be taken modulo G-orbits: factor-base size /|G|, relation search
  ~/|G|, linear algebra ~/|G|^2, while rho gains only sqrt(|G|) [standard; Duursma-Gaudry-Morain
  1999 for rho; for IC e.g. S. Tsakou, S. Ionica, "Index calculus attacks on hyperelliptic
  Jacobians with effective endomorphisms", J. Math. Cryptol. / ePrint 2021/721,
  https://eprint.iacr.org/2021/721 **[abstract-checked]**: endomorphism used to shrink the
  factor base for ordinary elliptic and genus-2 Jacobians, gain on total complexity at the
  cost of extra factor-base work]. "Extending the GLS endomorphism" (arXiv 2106.09967)
  reports a factor-n speed-up of IC in a GHS setting and a factor-555 speed-up in its
  experiments (snippet-checked, setting not confirmed).
- Plane quartics with large automorphism groups (Picard: Z/3; Fermat/Klein-type: up to 168
  over F_qbar, but these are isolated in moduli, so only twists exist) -- for a lab benchmark
  family, Picard curves (Z/3 or Z/6 with the hyperelliptic-like involution absent) and
  superelliptic y^n = f(x) (Z/n) give constant-factor orbit gains |G| in relation search,
  |G|^2 in linear algebra.
- Subfield (Koblitz-type) curves defined over F_p, attacked over F_{p^n}: Frobenius orbits
  give a factor ~n. Not relevant to prime-field instances.

## 7. Canonical-curve methods for g = 4..16
- Canonical embedding: non-hyperelliptic C -> P^(g-1), degree 2g-2. Hyperplanes = canonical
  divisors (zeros of holomorphic differentials). A hyperplane through g-1 chosen points is
  unique for points in general position; residual e = (2g-2)-(g-1) = g-1 (Bezout/Riemann-Roch).
  Each relation: H.C - H0.C = div(h/h0), so sum of 2g-2 points ~ fixed canonical divisor.
- Oyono-Theriault 2023 (above) analyse exactly this: O(q^(2-2/(g-1)+eps)) for all
  non-hyperelliptic curves; relation search stops after ~B^2 hyperplanes even though
  hyperplanes are pinned through g-1 points (they choose hyperplanes through only 2 FB points
  plus a free (g-3)-dimensional parameter? -- three selection strategies; detail UNVERIFIED).
  NOTE the "B^2 hyperplanes" phrase means the candidate count is still quadratic in B; a
  hyperplane through 2 FB points has (g-3) free parameters and residual 2g-4, which would
  be worse; so the residual-(g-1) count requires pinning g-1 points with B^(g-1) candidates,
  of which only ~B^2 are needed. [interpretation; check against full text]
- Comparison with plane models: projecting the canonical curve from g-3 of its points gives
  the degree-(g+1) plane model and the same e = g-1, so canonical = Diem's g+1 plane model
  asymptotically (Oyono-Theriault's stated conclusion). Advantages of canonical: no
  singular points (the plane model has nodes from the projection), works for ALL
  non-hyperelliptic curves (removes "sufficiently general"), symmetric treatment.
  Cost per candidate: computing the residual requires intersecting a hyperplane with g-2
  quadrics/cubics (ideal of the canonical curve; Petri: generated by quadrics for g >= 5 unless
  trigonal or plane quintic), i.e. solving a 0-dimensional system of degree g-1 -- reducible
  to one univariate polynomial of degree g-1 via elimination: cost poly(g) per candidate,
  comparable to the plane-line case where one substitutes the line into F(x,y) and divides out
  the known roots (polynomial of degree d-2).
- Beating g-1 requires leaving the canonical system: Diem-Kochinke pencils (e = g-2 for
  nearly all curves with g >= 5) or a plane model of degree ceil(2g/3)+2 (e = ceil(2g/3),
  unproven as an algorithm, needs a rational g^2_d).
- Numbers vs rho (exponent in q; rho = g/2):
  g=4: canonical 4/3 (rho 2); g=5: 3/2, pencils 4/3 (rho 2.5); g=6: 8/5, pencils 3/2 (rho 3);
  g=8: 12/7, 5/3 (rho 4); g=12: 20/11, 9/5 (rho 6); g=16: 28/15, 13/7 (rho 8).
  As g grows, the IC exponent tends to 2 while rho grows linearly, so the absolute gap
  q^(g/2 - 2) explodes; the hidden constants (factorials in splitting probability 1/e!,
  poly(g) factors, and the linear algebra over a factor base of size ~q^(1-1/e)) dominate
  at practical q.
- Splitting probability of a degree-e residual: ~1/e!. For e = 15 (g=16 canonical) that is
  ~7.6e-13: each relation costs ~10^12 candidates beyond the q-power count. This e! factor is
  what makes high-genus IC slow in practice and is the main "hidden constant" to attack
  (sec. 10) [arithmetic].

Addendum to 2a: Diem's publications page lists, with Kochinke, "Ordinary plane models and
completely split divisors" (abstract not located), and Kochinke's thesis "Special Linear Systems
on Curves and Algorithmic Applications", Univ. Leipzig (2017) (catalogue record only). A search
snippet describes the pencils paper as using plane models intersected with lines through
factor-base points, i.e. still a line-based method on a cleverly chosen (singular) plane model
[snippet-checked; exact construction UNVERIFIED]. Diem's page:
https://www.math.uni-leipzig.de/~diem/preprints/english.html .

## 8. Record and reported computations (non-hyperelliptic IC)
Few public "records" exist; the field moved on once O~(q) for quartics was established.
- Diem-Thome (J. Cryptology 2008): experiments supporting the single heuristic assumption, and
  "experimental data showing a variant is faster than rho even for small group sizes"
  **[abstract-checked]**; exact sizes UNVERIFIED (full text not fetched).
- Laine-Lauter (J. Math. Cryptol. 9(2):95-114, 2015; ePrint 2014/346,
  https://eprint.iacr.org/2014/346): C++ implementation of a Diem variant; experiments for
  small q match predicted constants; ~370 bytes per graph vertex memory; time-memory trade-off
  and parallelisation **[abstract/snippet-checked]**. They make the hidden log-factors and
  constants in O~(q) explicit -- the best available source for calibrating a lab baseline for
  genus-3 quartics.
- Joux-Vitse, "Cover and decomposition index calculus on elliptic curves made practical:
  application to a previously unreachable curve over F_{p^6}", EUROCRYPT 2012, LNCS 7237;
  ePrint 2011/020, https://eprint.iacr.org/2011/020 . **[snippet-checked]** 149-bit elliptic
  group over a degree-6 extension, ~110,000 CPU-hours; combines a GHS/Diem cover to a
  higher-genus curve over a subfield with decomposition index calculus. Whether the cover
  curve is non-hyperelliptic: UNVERIFIED (memory: a genus-3 cover over F_{p^2}, likely
  hyperelliptic). It is the largest published IC computation touching genus-3 Jacobians.
- Gaudry 2000 (hyperelliptic, for contrast): a genus-6 hyperelliptic cryptosystem broken by IC
  (EUROCRYPT 2000, https://www.iacr.org/archive/eurocrypt2000/1807/18070019-new.pdf).
- No public DLP record found for non-hyperelliptic genus >= 4 via line/hyperplane IC
  (search negative; treat as "none known to this note").
Lab implication: a genus-3 quartic record at, e.g., q ~ 2^40..2^50 (group ~2^120..2^150,
rho ~2^60..2^75 vs IC ~2^40..2^50 x constants) would itself be new data.

## 9. 2015-2026 work, including negative results
- Laine-Lauter 2015 (above): constants and trade-offs for genus-3 quartics.
- Oyono-Theriault 2023 (FFA 102227): canonical/non-planar IC, same exponent 2-2/(g-1)+eps for
  ALL non-hyperelliptic curves; Nagao graph with fixed factor base; curve sieving adapted.
- Diem-Kochinke (preprints ~2013, thesis 2017): 2-2/(g-2) heuristic, g >= 5, nearly all curves.
- Tian 2021 (J. Cryptology): (l,l,l)-isogenies genus-3 hyperelliptic -> non-hyperelliptic, O~(l^3).
- Dina-Ionica 2023 (Math. Comp.): CM fields with both hyperelliptic and non-hyperelliptic
  Jacobians with CM by Z_K: 14 of 547,156 sextic CM fields.
- NEGATIVE: Naranjo-Pirola 2018 (Adv. Math.): very general hyperelliptic Jacobian of genus
  >= 4 is not isogenous to any non-hyperelliptic Jacobian -> no generic Smith-style transfer
  for g >= 4.
- NEGATIVE (implicit): Diem-Kochinke show special linear systems fail for hyperelliptic curves.
- NEGATIVE (regime): L(1/3) methods (EG 2007, EGT 2011) need g large relative to log q; no
  benefit at fixed g in 3..16 for cryptographic q.
- Tsakou-Ionica 2021 (ePrint 2021/721): endomorphism orbit reduction of the factor base
  (elliptic and genus 2; method transfers to any curve with an F_q-rational automorphism).
- Derickx-Terao 2026 (arXiv 2602.17417, ANTS XVII): practical Hess-style class group and
  gonality computation, orders-of-magnitude speed-ups for large genus / residue field.
- Macri-Jacobson-Scheidler 2026 (WAIFI, arXiv 2605.15323): faster Hess-style Jacobian
  arithmetic for arbitrary global function fields (affects rho baselines and descent).
- Bruin-Kulkarni 2025 (Canad. J. Math.): (2,2)-decomposable genus-4 Jacobians; generalized
  Howe curves of genus 4-6 with completely decomposable Jacobians (arXiv 2604.18074, 2026):
  decomposable Jacobians reduce DLP to factors (structure lever, not IC).
- Not found: any improvement of the exponent for genus-3 quartics below 1 (O~(q) appears
  optimal for double-large-prime IC because the factor base must contain ~q^(1/2) points and
  the relation graph needs ~q edges); any published IC exponent for general g below 2-2/(g-2).

## 10. Biggest speed-up levers by regime
Exponents are in q (cost O~(q^x) group/field operations, fixed g, q -> infinity). "e" is the
residual degree; the splitting constant ~1/e! is hidden in O~. Rho is q^(g/2). All rows
`[pcc:unverified]`. "Published" = appears in a cited abstract; "derived" = substitution or
Brill-Noether/semigroup arithmetic by this note, not a published algorithm.

| g | Best known method (status) | Exponent in q (rho) | Curve model needed | Dominant cost | Single most promising lever | Confidence |
|---|---|---|---|---|---|---|
| 3 | Diem 2006 / Diem-Thome 2008 double-large-prime line IC; rigorous O~(q) Diem JTNB 2012; Laine-Lauter 2015 trade-offs | 1 (rho 1.5); hyperelliptic g=3 -> 1 via Smith/Tian isogeny when rational | Plane quartic (= canonical); hyperelliptic only via (2,2,2)/(l,l,l)-isogeny | Relation collection over ~q lines + large-prime graph; memory ~q^(3/4) (Diem talk, unverified) and graph ~370 B/vertex | Constants and memory: Laine-Lauter trade-off + automorphism orbits (Picard Z/3) + sieving lines through points; for hyperelliptic inputs, Tian (l,l,l) transfer to widen the 18.57% Smith coverage | High (exponent), medium (constants) |
| 4 | Canonical/hyperplane IC (Oyono-Theriault 2023) = Diem g+1 plane model | 4/3 (rho 2) | Canonical (quadric cap cubic in P^3) or plane quintic with 2 nodes (project from a rational point); C_{3,5} special | Linear algebra on factor base ~q^(2/3) and relations ~q^(4/3); splitting 1/3! | Plane-quintic model + trigonal free relations (split g^1_3 fibres) + exploiting the two g^1_3 rulings; no exponent drop known | Medium-high |
| 5 | Diem-Kochinke pencils (heuristic, "nearly all" curves) | 4/3 (rho 2.5); canonical 3/2 | Special linear systems (pencils) on a plane model; else canonical (3 quadrics in P^4) | Constructing the pencil family; relations ~q^(4/3) | Implement Diem-Kochinke pencils (no public implementation found): 3/2 -> 4/3 is the largest exponent drop available at any g | Medium |
| 6-8 | Diem-Kochinke 2-2/(g-2) (published); ties with Brill-Noether-min plane model (derived) | g=6: 3/2; g=7: 8/5; g=8: 5/3 (rho 3, 3.5, 4). Special: smooth plane quintic g=6 -> 4/3; C_{4,5} g=6 -> 4/3 | General: nodal plane model of degree ceil(2g/3)+2 (6,7,8) or pencils; special: smooth quintic (g=6) | e = 4..6, splitting 1/24..1/720; factor base ~q^(1-1/e) | Choose/construct curves or models with smallest e: for g=6 target the plane-quintic locus (e=3); otherwise compute an F_q-rational g^2_d (rho(g,2,d) > 0 for g=7,8 so a positive-dim family exists) | Medium (published), low (derived) |
| 9-12 | Diem-Kochinke 2-2/(g-2) (published); min plane model 2-2/ceil(2g/3) (derived, beats pencils from g=9) | Pencils 12/7..9/5; plane 5/3 (g=9), 12/7 (g=10), 7/4 (g=11, 12); special smooth plane sextic / C_{5,6} at g=10 -> 3/2 (rho 4.5..6) | Nodal plane model degree ceil(2g/3)+2; special: smooth sextic (g=10) | e = 6..10: splitting 1/720..1/3.6e6 dominates; linear algebra ~q^(2-2/e) on ~q^(1-1/e) unknowns | Low-degree plane models (Brill-Noether g^2_d computation over F_q, e.g. via Derickx-Terao-style Riemann-Roch tools) and special families with e <= 4 | Low-medium |
| 13-16 | Same two routes; L(1/3) irrelevant at fixed g | Pencils 20/11 (g=13) .. 13/7 (g=16); plane 16/9 (g=13) .. 20/11 (g=16); special smooth septic / C_{6,7} at g=15 -> 8/5; nodal plane octic (g <= 21) -> 5/3 (rho 6.5..8) | Nodal plane models; C_ab with balanced a,b; superelliptic y^n=f(x) with n ~ deg f | Splitting probability 1/e! with e = 9..14 (~2.8e-6 .. 1.1e-11) and per-candidate cost poly(g); the q-exponent is near 2 so constants decide | Shrink e via special models (balanced C_ab, superelliptic, smooth/nodal plane curves of degree 7-8) combined with automorphism-orbit reduction (y^n = f(x) gives Z/n); sieving instead of per-line root finding | Low |

Note on the g = 10, 11 plane column: ceil(20/3) = 7 and ceil(22/3) = 8, so the plane exponent is
2-2/7 = 12/7 (g=10) and 2-2/8 = 7/4 (g=11); g=9 gives ceil(6) = 6 -> 5/3; g=12 gives 8 -> 7/4.
Pencils: g=9 12/7, g=10 7/4, g=11 16/9, g=12 9/5. g=13: plane ceil(26/3)=9 -> 16/9, pencils 2-2/11 = 20/11;
g=16: plane 11 -> 20/11, pencils 2-2/14 = 13/7.

### Cross-cutting conclusions
1. The exponent is governed by the residual degree e, not by g. Every concrete exponent
   improvement in the literature (hyperelliptic g -> canonical g-1 -> pencils g-2 -> plane d-2)
   is a reduction of e. The lab's biggest asymptotic lever beyond published work is lowering e
   to ceil(2g/3) via an F_q-rational g^2_d of minimal degree (derived here, not published as
   an algorithm), and choosing benchmark families (smooth plane quintic/sextic/septic,
   balanced C_ab, superelliptic) where e ~ sqrt(2g).
2. For g >= 4 the gap to rho is enormous in the exponent (rho g/2 vs IC < 2), so practical
   distance is set by constants: 1/e! splitting, poly(g) per-candidate cost, and memory of
   the large-prime graph. Sieving (Oyono-Theriault "curve sieving"), orbit reduction by
   automorphisms, and the Laine-Lauter time-memory trade-off attack these.
3. For g = 3, nothing beats O~(q); progress is in constants and in moving hyperelliptic inputs
   (Smith 18.57%, Tian (l,l,l)) to quartics. For g >= 4 no generic hyperelliptic ->
   non-hyperelliptic isogeny exists (Naranjo-Pirola), so the lab's hyperelliptic targets
   cannot be moved this way.
4. Rigour: only O~(q^(2-2/g)) (all curves) and O~(q^(2-2/(d-2))) for plane models with a
   completely split line (JTNB 2012) are proven per the abstracts; 2-2/(g-1) for all curves
   (Oyono-Theriault) has an +eps upper bound; 2-2/(g-2) (Diem-Kochinke) is heuristic.

## 11. References (as cited above; verification level in brackets)
- Diem, C. (2005/2006). Index calculus in class groups of plane curves of small degree. ePrint 2005/119, https://eprint.iacr.org/2005/119 ; ANTS-VII LNCS 4076, https://doi.org/10.1007/11792086_38 [abstract-checked]
- Diem, C., Thome, E. (2008). Index calculus in class groups of non-hyperelliptic curves of genus three. J. Cryptology 21(4):593-611, https://link.springer.com/article/10.1007/s00145-007-9014-6 ; HAL inria-00107290 [abstract-checked]
- Diem, C. (2006). Index calculus with double large prime variation for curves of small genus with cyclic class group. arXiv math/0606607, https://arxiv.org/abs/math/0606607 [abstract-checked]
- Diem, C. (2011). On the discrete logarithm problem in class groups of curves. Math. Comp. 80:443-475, https://www.ams.org/mcom/2011-80-273/S0025-5718-2010-02281-1/ [abstract-checked]
- Diem, C. (2012). On the discrete logarithm problem for plane curves. JTNB 24(3):639-667, https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/ [abstract-checked]
- Diem, C., Kochinke, S. Computing discrete logarithms with pencils; Computing discrete logarithms with special linear systems (preprints), https://www.math.uni-leipzig.de/~diem/preprints/dlp-linear-systems.pdf , https://www.semanticscholar.org/paper/6e74dbf3ffed71cb0adabf3a9aae5d37a6480e64 [abstract-checked via Semantic Scholar]
- Kochinke, S. (2017). Special Linear Systems on Curves and Algorithmic Applications. PhD thesis, Univ. Leipzig [catalogue only]
- Oyono, R., Theriault, N. (2023). Index calculus algorithm for non-planar curves. Finite Fields Appl., 102227, https://www.sciencedirect.com/science/article/abs/pii/S1071579723000692 [abstract-checked]
- Laine, K., Lauter, K. (2015). Time-memory trade-offs for index calculus in genus 3. J. Math. Cryptol. 9(2):95-114, https://eprint.iacr.org/2014/346 [abstract-checked]
- Hess, F. Computing relations in divisor class groups of algebraic curves over finite fields. Preprint (~2004) [not located; cited by EGT 2011]
- Hess, F. (2002). Computing Riemann-Roch spaces in algebraic function fields and related topics. J. Symbolic Comput. 33:425-445 [UNVERIFIED-memory]
- Enge, A., Gaudry, P. (2007). An L(1/3+eps) algorithm for the DLP for low degree curves. EUROCRYPT 2007, https://arxiv.org/abs/cs/0703032 [abstract-checked]
- Enge, A., Gaudry, P., Thome, E. (2011). An L(1/3) discrete logarithm algorithm for low degree curves. J. Cryptology 24:24-41, https://arxiv.org/abs/0905.2177 [abstract-checked]
- Couveignes, J.-M. (2001). Algebraic groups and discrete logarithm. In Public-key cryptography and computational number theory, de Gruyter, 17-27 [citation only]
- Smith, B. (2008/2009). Isogenies and the discrete logarithm problem in Jacobians of genus 3 hyperelliptic curves. EUROCRYPT 2008; J. Cryptology 22(4):505-529, https://arxiv.org/abs/0806.2995 [abstract-checked]
- Frey, G., Kani, E. (2011). Correspondences on hyperelliptic curves and applications to the discrete logarithm. SIIS 2011, LNCS 7053, https://www.mast.queensu.ca/~kani/papers/fkwarsawarticlerevised.pdf [abstract-checked]
- Tian, S. (2021). Translating the DLP on Jacobians of genus 3 hyperelliptic curves with (l,l,l)-isogenies. J. Cryptology 34(3):32, https://arxiv.org/abs/2007.03172 [abstract-checked]
- Dina, B., Ionica, S. (2023). Isogenous hyperelliptic and non-hyperelliptic Jacobians with maximal complex multiplication. Math. Comp. 92:349-383, https://arxiv.org/abs/2104.04919 [abstract-checked]
- Naranjo, J.C., Pirola, G.P. (2018). Hyperelliptic Jacobians and isogenies. Adv. Math., https://arxiv.org/abs/1705.10154 [abstract-checked]
- Bruin, N., Kulkarni, A. (2025). On (2,2)-decomposable genus 4 Jacobians. Canad. J. Math., doi:10.4153/S0008414X25101557 [abstract-checked]
- Bruin, N. (2019). Prym varieties of genus four curves (slides, ICERM) [snippet-checked]
- Frey, G., Shaska, T. (2018). Curves, Jacobians, and Cryptography. arXiv 1807.05270, https://arxiv.org/abs/1807.05270 [abstract-checked]
- Joux, A., Vitse, V. (2012). Cover and decomposition index calculus on elliptic curves made practical. EUROCRYPT 2012, https://eprint.iacr.org/2011/020 [snippet-checked]
- Tsakou, S., Ionica, S. (2021). Index calculus attacks on hyperelliptic Jacobians with effective endomorphisms. ePrint 2021/721, https://eprint.iacr.org/2021/721 [abstract-checked]
- Derickx, M., Terao, K. (2026). Computing class groups and gonalities of algebraic curves over finite fields. arXiv 2602.17417, ANTS XVII, https://arxiv.org/abs/2602.17417 [abstract-checked]
- Macri, V., Jacobson, M.J., Scheidler, R. (2026). Improvements to Jacobian arithmetic in global function fields. WAIFI 2026, LNCS 16611:111-128, https://arxiv.org/abs/2605.15323 [abstract-checked]
- Gaudry, P. (2000). An algorithm for solving the discrete log problem on hyperelliptic curves. EUROCRYPT 2000, https://www.iacr.org/archive/eurocrypt2000/1807/18070019-new.pdf [abstract-checked]

Status: COMPLETE (2026-11-17). Main gaps: full texts of Diem-Thome, Diem-Kochinke, Oyono-Theriault
and Hess not read (hosts blocked / paywalled); record sizes for Diem-Thome and Laine-Lauter unverified.

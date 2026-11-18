# A. Literature: index calculus in hyperelliptic Jacobians, genus 2..16

Status: COMPLETE draft (see end of file for limits).
Date: 2026-11-17. Scope: DLP in Jac(C)(F_q), C hyperelliptic of genus g, 2 <= g <= 16.
Reference cost: Pollard rho ~ sqrt(pi #J / 2) ~ q^{g/2} group operations (less sqrt(2) with negation map, less sqrt(2g) style factors with Frobenius in subfield cases).

Convention: "verified" = statement read in the primary source (abstract or body) during this session;
"unverified" = from memory or secondary source, not re-read here.

## Outline
1. Gaudry (2000): degree-1 factor base, random-walk relation search
2. Enge-Gaudry (2002), Enge (2002): subexponential L(1/2), g > c log q regime
3. Large primes: Thériault (2003), GTTD (2007), Nagao (2007), Diem; graph handling
4. Factor-base reduction, optimal fraction, relation vs linear algebra balance
5. Enge-Gaudry-Thomé (2011) L(1/3), C_ab; relevance to hyperelliptic g=2..16
6. Sieving: Vitse-Wallet, Joux-Vitse, Laine-Lauter, sieve replacing random walk
7. Record computations
8. Genus 6..16: crossover with rho
9. Recent work 2018-2026 (incl. negative results)
10. Biggest speed-up levers by regime (table)

## Method and verification limits (read first)
- WebFetch could not reach eprint.iacr.org, arxiv.org or hal.archives-ouvertes.fr from this session
  (DNS refusal by the session network policy: these hosts must be allowed in the environment's network
  settings for primary-source PDF reading). Web *search* worked, so statements below are marked:
  - **[verified-snippet]**: the statement appears in an abstract/snippet returned by search for the primary source.
  - **[memory]**: from my knowledge of the paper; not re-read this session = *unverified* in the lab's sense.
- No research-kb MCP server or `scripts/research-kb` fallback was available in this checkout; no source_id recorded.
- All claims here are literature assertions: `[pcc:unverified]`.

---

## 1. Gaudry (2000): degree-1 factor base and random-walk relation search

**Source.** P. Gaudry, "An algorithm for solving the discrete log problem on hyperelliptic curves",
EUROCRYPT 2000, LNCS 1807, pp. 19-34. https://doi.org/10.1007/3-540-45539-6_2 ;
HAL inria-00512401 (https://hal.archives-ouvertes.fr/inria-00512401).

**Setting.** Imaginary (one point at infinity) hyperelliptic curve y^2 + h(x) y = f(x), deg f = 2g+1,
over F_q, g fixed, q -> infinity. Elements of J(F_q) in Mumford form (u, v), deg u <= g.

**Factor base.** All prime divisors of degree 1, i.e. divisors <P> - <inf> with P in C(F_q) represented by
u = x - a irreducible of degree 1. Size ~ q (halved to ~ q/2 by identifying P with its opposite -P, since
<P> + <-P> ~ 0: one keeps one representative per x-coordinate and records a sign) [memory].

**Smoothness test.** A reduced divisor D = (u, v) is factor-base smooth iff u splits into linear factors over
F_q. For random D (deg u = g with prob ~ 1 - 1/q), Pr[u splits completely] -> 1/g! as q -> infinity
(the proportion of totally split monic polynomials of degree g). Testing = gcd(u, x^q - x) or root
finding: polynomial in g and log q [memory; standard].

**Relation search ("Harley/Gaudry random walk").** An r-adding walk (Teske-style) in J: precompute
R_j = a_j D_1 + b_j D_2 for j = 1..r (r ~ 20); iterate T <- T + R_{H(T)}, keeping track of (a, b)
with T = a D_1 + b D_2. Each step is ONE group addition (cheaper than a fresh random combination by
scalar multiplication). Each smooth T gives a relation sum e_i P_i = a D_1 + b D_2. Expected
group operations per relation ~ g!. The idea of using the rho-style walk to generate relations is
attributed to R. Harley in Gaudry's paper and in later surveys [memory, unverified attribution detail].

**Linear algebra.** ~#B relations, sparse matrix with <= g non-zeros per row, solved mod the large prime
factor of #J with structured Gaussian elimination + Lanczos/Wiedemann: O(g #B^2) ring operations.

**Complexity (fixed g).** Total O(q^2 log^gamma q) [verified-snippet: secondary citation of the paper
in search results, "solves the DLP in hyperelliptic curves of genus g defined on F_q in time O(q^2 log^gamma q)"].
The commonly quoted explicit form, from the paper's analysis, is

    O( g^2 g! q log^2 q  +  g^3 q^2 log^2 q )      [memory; the g! sits on the relation term,
                                                    the q^2 on linear algebra]

i.e. relation collection is linear in q but carries g!; linear algebra is quadratic in q and only
polynomial in g. Gaudry's abstract states the method "is faster than the Rho method for genus greater
than 4" [verified-snippet: abstract quoted by search; rho is q^{g/2} so beats q^2 only when g/2 > 2].
Because g! grows faster than q^{g/2}/q for small q, the "practical" regime is g roughly up to ~9
with q not tiny; for g large relative to log q the g! term dominates and one must change the
factor base (Section 2).

**Factor base reduction already in Gaudry/Harley.** Shrinking B to a fraction q^r (0<r<=1) reduces
linear algebra to q^{2r} but cuts smoothness probability by (q^{r-1})^g. Balancing
g! q^{g(1-r)} q^r  vs  q^{2r}  gives r = g/(g+1) and total

    O~( q^{2 - 2/(g+1)} )    (= q^{2g/(g+1)})

[memory; this "rebalanced" bound is the baseline that Thériault (2003) cites and improves; see Section 3].
Note the hidden constant: the relation side picks up g! times a factor (q^{1-r})^g, and only the
exponent is balanced; constants in g are NOT balanced in the published analyses.

**Smart's analysis.** N. P. Smart, "On the performance of hyperelliptic cryptosystems", EUROCRYPT 1999
/ follow-ups, and Smart's Eurocrypt 2001 discussion of Weil descent report that Gaudry's algorithm
"hides a very bad dependence on g" [verified-snippet via search summary; exact paper attribution
unverified].

---

## 2. Subexponential regime: Enge-Gaudry (2002), Enge (2002)

**Sources.**
- A. Enge, P. Gaudry, "A general framework for subexponential discrete logarithm algorithms",
  Acta Arithmetica 102(1) (2002) 83-103, doi:10.4064/aa102-1-6. https://eudml.org/doc/278301 ;
  HAL inria-00512717.
- A. Enge, "Computing discrete logarithms in high-genus hyperelliptic Jacobians in provably
  subexponential time", Math. Comp. 71(238) (2002) 729-742; preprint CORR 99-04 (Waterloo),
  https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/1999_04.pdf
- Earlier: L. Adleman, J. DeMarrais, M.-D. Huang, "A subexponential algorithm for discrete logarithms
  over the rational subgroup of the Jacobians of large genus hyperelliptic curves over finite fields",
  ANTS-I 1994 (heuristic L(1/2, c<=2.181) for log q <= (2g+1)^{0.98}) [memory]; Flassenberg-Paulus
  (1999) implementation of a sieve version [memory]; Müller-Stein-Thiel (1999) real quadratic models
  [memory].

**Factor base.** All prime divisors of degree <= b ("smoothness bound" on the degree), i.e. irreducible
u of degree <= b with a v making (u, v) a divisor. Size ~ q^b / b (about half of the irreducible
polynomials of degree <= b lift).

**Smoothness.** A reduced divisor is b-smooth iff its u is b-smooth as a polynomial. Since Mumford u
of a random reduced divisor behaves like a random monic polynomial of degree g, the smoothness
probability is the polynomial analogue of Dickman: Pr ~ (g/b)^{-(1+o(1)) g/b}, valid when
g/b -> infinity and b -> infinity (Odlyzko/Car/Panario-Gourdon-Flajolet estimates) [memory].

**Complexity statements.**
- Enge-Gaudry (Acta Arith 2002): for g/log q -> infinity, expected time
  L_{q^g}(1/2, sqrt(2) + o(1)) [verified-snippet: "expected time L_N(sqrt2 + o(1)) = L_D(1 + o(1))";
  cited also as L_{q^g}[1/2, sqrt2 + o(1)] by arXiv 2106.09967]. Here L_N(c) = exp(c sqrt(log N log log N)),
  N = q^g ~ #J. The equality L_N(sqrt2) = L_D(1) uses the "degree" variable D.
- For g >= vartheta log q with fixed vartheta > 0 (the boundary regime), the constant degrades;
  Enge (Math Comp 2002) proves rigorously
  L_{q^g}(1/2, (5/sqrt6) * ( sqrt(1 + 3/(2 vartheta)) + sqrt(3/(2 vartheta)) ) + o(1))
  [verified-snippet; note the CORR preprint renders the constant differently, so the exact constant
  is to be checked against the published PDF]. As vartheta -> infinity this tends to 5/sqrt6 ~ 2.04
  (rigorous), versus sqrt2 ~ 1.41 heuristic/with the more refined analysis of Enge-Gaudry.

**Crossover regime (the key practical question).** The L(1/2) bounds require g >= vartheta log q,
i.e. q^g >= q^{vartheta log q}: equivalently log q <= g / vartheta. For the lab's range g = 2..16 and
cryptographic q (log_2 q >= 40 for any interesting #J), log q (natural log ~ 28+) exceeds g, so
**vartheta < 1 for every case of interest: the L(1/2) regime is not reached**; the "q fixed, g -> inf"
analysis is asymptotically irrelevant there and its o(1) terms dominate. Concretely, b must be >= 2
for the subexponential method to differ from Gaudry's; a degree-2 factor base has size ~ q^2/2, so the
linear algebra alone costs ~ q^4 > q^{g/2} unless g > 8. Hence:
 - with b = 1 one is in the Gaudry / large-prime family (exponential, q^{2-2/g});
 - b = 2 can only pay off when g is large (g >~ 9..12) AND q is small enough that g! / (prob. of
   b=2 smoothness) savings outweigh the q^2 factor-base size. No published concrete crossover table
   for b = 2 vs b = 1 at g = 9..16 was found (see Section 8).
This is my derivation from the stated bounds [analysis, unverified by a published table].

**Rigorous vs heuristic.** Enge's 2002 result is rigorous (no heuristics) [verified-snippet].
Enge-Gaudry is a general framework for groups with "unique factorisation" and smoothness, giving
rigorous L(1/2) for finite fields and class groups in their regime [verified-snippet].

---

## 3. Large-prime variations

### 3.1 Thériault (2003): single large prime
**Source.** N. Thériault, "Index calculus attack for hyperelliptic curves of small genus",
ASIACRYPT 2003, LNCS 2894, pp. 75-92. https://doi.org/10.1007/978-3-540-40061-5_5

**Method.** Reduced factor base B (fraction of the degree-1 primes); points outside B are "large primes".
Keep "almost smooth" divisors with exactly one large prime; two with the same large prime combine
into a full relation (birthday among ~ q large primes).

**Complexity.** O(g^5 q^{2 - 4/(2g+1) + eps}) = O~(q^{2 - 2/(g + 1/2)}) [verified-snippet: "running time
of O(g^5 q^{2-4/(2g+1)+eps}) and an improvement for all curves of genus greater than 2"].
Paper's table: genus 3 -> q^{10/7} (~1.43), genus 4 -> q^{14/9} (~1.56), genus 5 -> q^{18/11} (~1.64),
vs q^2 for the original index calculus [verified-snippet]. (Compare reduced-base Gaudry q^{2-2/(g+1)}:
g=3 1.5, g=4 1.6, g=5 1.67.) Thériault also analysed ~ the use of more than one large prime as a
direction (left to GTTD).

### 3.2 Gaudry-Thomé-Thériault-Diem (2007): double large prime
**Source.** P. Gaudry, E. Thomé, N. Thériault, C. Diem, "A double large prime variation for small
genus hyperelliptic index calculus", Math. Comp. 76(257) (2007) 475-492; ePrint 2004/153,
https://eprint.iacr.org/2004/153 ; HAL inria-00000897.

**Method.** Factor base of size ~ q^{1-1/g} (fraction q^{-1/g} of the points); keep relations with up to
two large primes. Build the **graph of large primes**: vertices = large primes plus a special vertex
"1" (for the factor base), edges = 1- and 2-large-prime relations. A cycle in the graph (or a path to
vertex 1) yields a full relation by combining edges. GTTD give two algorithms: (1) a natural adaptation
of the double large prime variation (keep all edges, detect cycles with union-find); (2) a variant
that only keeps edges connected to the growing component of vertex "1" ("concentric" / "full-
relation-oriented" growth), which is the one amenable to a rigorous proof [memory for the algorithm
split; the rigorous-proof statement is verified-snippet: "complete proof ... without any assumption or
heuristic"].

**Complexity.** O~(q^{2-2/g}) for both relation collection and linear algebra, factor base ~ q^{1-1/g}
[verified-snippet]. Exponents: g=2: 1 (= rho, no gain); g=3: 4/3 vs rho 3/2; g=4: 3/2 vs 2;
g=5: 8/5 vs 5/2; g=6: 5/3 vs 3; g=8: 7/4 vs 4; g=16: 15/8 vs 8.
The O~ hides factors exponential in g (g! from smoothness of degree-g u, plus combinatorial factors for
the large-prime positions); "for fixed g" only [verified-snippet for "fixed genus"; genus-dependence
claim memory]. Experimental: "for hyperelliptic curves of genus three, our first algorithm surpasses
Pollard's Rho method even for rather small field sizes" [verified-snippet, ePrint abstract].

### 3.3 Nagao (2004/2007): double large prime, independently
**Source.** K. Nagao, "Index calculus attack for Jacobian of hyperelliptic curves of small genus using two
large primes", Japan J. Indust. Appl. Math. 24 (2007) 289-305, doi:10.1007/BF03167541,
https://projecteuclid.org/euclid.jjiam/1197909114 ; preprint ePrint 2004/161 "Improvement of Thériault
algorithm of index calculus for Jacobian of hyperelliptic curves of small genus".
Defines "2-almost smooth divisors"; complexity O(q^{2-2/g+eps}) [verified-snippet], proves the failure
probability negligible and gives expected complexity and storage [verified-snippet]. Nagao notes Gaudry-
Thomé present an algorithm of the same complexity [verified-snippet].

### 3.4 Diem: generalisation and rigorous analysis
- C. Diem, "Index calculus with double large prime variation for curves of small genus with cyclic class
  group", arXiv math/0606607: for fixed g >= 2, DLP in Cl^0 of a curve given by a plane model of
  bounded degree with cyclic Cl^0 in expected time O~(q^{2-2/g}) [verified-snippet].
- C. Diem, "On the discrete logarithm problem in class groups of curves", Math. Comp. 80 (2011) 443-475,
  https://www.ams.org/mcom/2011-80-273/S0025-5718-2010-02281-1/ : Thm 1, fixed g >= 2, all curves,
  O~(q^{2-2/g}); Thm 2 a bound in terms of #Cl^0 for genus >= g_0 (sub-square-root regime, answers
  "under which conditions on the genera ... o(#Cl^0(C)^{1/2})"); Thm 3 elliptic curves over F_{q^n},
  fixed n >= 2, O~(q^{2-2/n}) [verified-snippet].
- C. Diem, "An index calculus algorithm for plane curves of small degree", ANTS VII 2006, LNCS 4076:
  O~(q^{2-2/(d-2)}) for plane curves of degree d; for non-hyperelliptic genus 3 (plane quartics, d = 4)
  O~(q) [memory]. This is why non-hyperelliptic genus 3 is weaker than hyperelliptic genus 3 (q^{4/3}).
- C. Diem, "Discrete logarithms in curves over finite fields", survey, arXiv 0712.3916 [title verified
  by search; content memory].

### 3.5 Graph of large primes and practical constants
- The graph is the same as in NFS/QS double-large-prime (Lenstra-Manasse 1994; Leyland-Lenstra-Dodson):
  the number of independent cycles = #edges - #vertices + #components, tracked incrementally with a
  union-find structure [memory].
- In GTTD the growth of the component containing "1" is slow at first, then explosive (a "giant
  component" phase transition, as in random graph theory); most full relations appear late. This
  makes the tail of collection cheap but means early stopping is bad [memory].
- Full relations from cycles are long (weight grows with cycle length) -> the matrix is denser than
  the g-per-row of plain Gaudry; GTTD/Thomé note that linear algebra cost increases by the average
  cycle length factor, and structured Gaussian elimination ("filtering") is essential [memory].
- Constant-level: the smoothness probability of a degree-g u with at most 2 large primes and the rest
  in B (fraction q^{-1/g}) is ~ (1/g!) * [binomial(g,2) (q^{-1/g})^{g-2} + ...]; so the g! constant from
  Gaudry 2000 survives intact and is the dominant non-q cost at g >= 6 [analysis from definitions].

---

## 4. Factor-base size, optimal fraction, relation vs linear algebra balance as functions of g

Let N_B = #factor base, P_s = probability that a candidate is (almost-)smooth, c_rel = cost per
candidate (one Jacobian addition plus a split test), c_LA = cost of linear algebra on an N_B x N_B
matrix with w non-zeros per row (w ~ g for plain relations, larger after cycle-combination).

| Variant | N_B | Relation cost | LA cost | Balanced exponent | Source / status |
|---|---|---|---|---|---|
| Gaudry 2000 full base | ~q/2 | ~ g! * q (group ops) | ~ g q^2 | q^2 (LA dominates for q > g!) | Gaudry 2000 [memory] |
| Gaudry/Harley reduced base, 0 LP | q^r | g! q^{r + g(1-r)} | q^{2r} | r = g/(g+1): q^{2-2/(g+1)} | cited as baseline by Thériault 2003 [memory] |
| Thériault 1 LP | q^r | balanced with LA | q^{2r} | r = 1 - 2/(2g+1) (base q^{1-1/(g+1/2)}) -> q^{2-4/(2g+1)} | Thériault 2003 [verified-snippet for exponent] |
| GTTD / Nagao 2 LP | q^{1-1/g} | O~(q^{2-2/g}) | O~(q^{2-2/g}) | q^{2-2/g} | GTTD 2007 [verified-snippet] |

Observations (derived from the above; flagged as analysis):
- **The optimal base fraction is q^{-1/g}** in the 2LP algorithm: a fraction that tends to 1 as g grows.
  For g = 3 the base is q^{2/3} (a tiny fraction of the points); for g = 16 it is q^{15/16}, i.e. almost
  all points. So the large-prime machinery buys less and less as g grows: the exponent gap between
  "no reduction" (q^2) and 2LP (q^{2-2/g}) is only 2/g.
- **Balance.** In all published analyses, relations and LA are balanced *in the exponent of q only*.
  The g! (more precisely ~ g!/binom(g,2) for 2LP) multiplies the relation side. Consequently at fixed q
  and growing g, relation collection dominates; at fixed g and growing q, both sides scale identically
  and the constant decides. For practical sizes reported (genus 3, GTTD experiments), linear algebra
  was the bottleneck after filtering [memory, unverified].
- **Re-balancing with the constant.** Re-optimising r to include g! moves r *down* (smaller base, more
  LA-cheap) only if relation search is cheap; since relation cost carries g!, the constant-aware optimum
  moves r *up* (larger base) for g >~ 6. No paper found that publishes a constant-aware optimum per g;
  this is a candidate lever (analysis).
- **Linear algebra details.** Rows from 2LP cycles have weight ~ g * (average cycle length); structured
  Gaussian elimination reduces matrix dimension by large factors before block Wiedemann/Lanczos.
  Thomé's block Wiedemann implementations (later CADO-NFS `bwc`) are the tools used [memory].
- **Free 2x from the hyperelliptic involution.** P and -P (the image under the hyperelliptic involution)
  give the same u factor; identify them and record signs: halves N_B and the LA dimension. Standard in
  all implementations [memory].
- **Rho comparison constant.** Rho ~ sqrt(pi N/4) with negation map, N ~ q^g; the index calculus
  constant includes g! and polynomial factors g^3..g^5 (Thériault's bound shows g^5 explicitly).

---

## 5. Enge-Gaudry-Thomé (2011) L(1/3) and C_ab curves

**Sources.**
- A. Enge, P. Gaudry, E. Thomé, "An L(1/3) discrete logarithm algorithm for low degree curves",
  J. Cryptology 24 (2011) 24-41, doi:10.1007/s00145-010-9057-y; arXiv 0905.2177;
  HAL inria-00383941; https://members.loria.fr/EThome/files/l13.pdf
- A. Enge, P. Gaudry, "An L(1/3 + eps) algorithm for the discrete logarithm problem for low degree
  curves", EUROCRYPT 2007, LNCS 4515; arXiv cs/0703032;
  https://www.iacr.org/archive/eurocrypt2007/45150379/45150379.pdf
- C. Diem, "On the discrete logarithm problem in class groups of curves" (2011) (rigorous variants), and
  Diem's "An index calculus algorithm for plane curves of small degree" (ANTS 2006).

**Statement.** For families of plane curves C_i with deg_X C_i ~ g_i^alpha and deg_Y C_i ~ g_i^{1-alpha}
(more precisely degrees in an interval [~g^alpha, ~g^{1-alpha}], 1/3 <= alpha <= 2/3), heuristic expected
time L_{q^g}(1/3, O(1)); if deg_X * deg_Y <= kappa g then L(1/3, (64 kappa/9)^{1/3}) [verified-snippet,
arXiv/HAL abstract]. The Eurocrypt 2007 paper obtains L(1/3 + eps) for C_ab curves with the same flavour
[verified-snippet title; details memory]. The algorithm is a function-field-sieve analogue: relations come
from functions phi(X, Y) = a(X) + b(X) Y (and higher Y-degree) whose divisor norm is smooth on both sides,
with a factor base of places of degree <= b on the curve [memory].

**Does anything L(1/3)-like apply to hyperelliptic curves of genus 2..16 at practical q?**
- A hyperelliptic curve y^2 = f(x) with deg f = 2g+1 has deg_Y = 2 and deg_X = 2g+1. Then
  deg_X * deg_Y ~ 4g but deg_Y = 2 is NOT in [g^alpha, g^{1-alpha}] for alpha >= 1/3 once g > 8; in the
  EGT framework hyperelliptic curves are the "unbalanced" extreme and only get L(1/2) [analysis based on
  the verified abstract's degree condition; EGT themselves state hyperelliptic curves remain at L(1/2) in
  the high-genus regime -- memory, unverified].
- More fundamentally all L-results require g -> infinity with log q = o(g) or comparable; at g <= 16 and
  log q >= 30 the L(1/3) machinery has no asymptotic content. A degree-1 factor base already has size ~q,
  so any L(1/3) analogue degenerates to a q^{c} algorithm with c >= 1 (linear algebra over >= q unknowns
  costs >= q^2 unless the base is reduced). Conclusion: **no published L(1/3)-type method beats the
  double large prime O~(q^{2-2/g}) for hyperelliptic g = 2..16 at cryptographic q**. [analysis; no paper
  found that claims otherwise].
- What can transfer to fixed genus is the *relation-generation idea* of EGT/Diem: build relations from
  functions of low degree passing through factor-base points (lines through two points for plane
  curves), which makes the "unsplit" part of the divisor have degree d - 2 instead of g. For hyperelliptic
  curves, the analogue is choosing v of degree low and requiring f - v^2 (degree 2g+1) to split -- worse,
  not better. This is exactly why non-hyperelliptic genus 3 gets O~(q) (Diem 2006; Diem-Thomé 2008) while
  hyperelliptic genus 3 is stuck at O~(q^{4/3}). For hyperelliptic genus g the "function through
  factor-base points" relation search is the Sarkar-Singh / Nagao-style decomposition (Section 6), which
  does not change the exponent.
- **Isogeny/cover transfer** is the realistic way to reach the "low degree" regime: Smith (2008/2009)
  maps ~18.57% of hyperelliptic genus-3 Jacobians (via (2,2,2)-isogenies) to non-hyperelliptic Jacobians,
  reducing to O~(q) [verified-snippet; B. Smith, "Isogenies and the discrete logarithm problem in
  Jacobians of genus 3 hyperelliptic curves", EUROCRYPT 2008 / J. Cryptology 2009, ePrint 2007/428,
  arXiv 0806.2995]. Frengley-Smith? / "Translating the DLP on Jacobians of genus 3 hyperelliptic curves with
  (l,l,l)-isogenies" (arXiv 2007.03172, J. Math. Cryptol. 2021) extends the range with (3,3,3) etc.
  [title verified; author list and percentages unverified].

---

## 6. Sieving and relation-search replacements

### 6.1 Sarkar-Singh (2014-2017): decomposition with sieving for hyperelliptic Jacobians
- P. Sarkar, S. Singh, "A new method for decomposition in the Jacobian of small genus hyperelliptic
  curves", ePrint 2014/815 (https://eprint.iacr.org/2014/815); Des. Codes Cryptogr. (2017).
  Same factor base as Gaudry (degree-1 places); the new decomposition "admits a sieving technique which
  removes smoothness checking of polynomials required in Gaudry's method", and "the total number of
  additions in the Jacobian required by the new method is less than that required by Gaudry's method"
  [verified-snippet, ePrint abstract]. The journal abstract drops the sieving sentence and reports Magma
  timing examples [verified-snippet]. Idea [memory]: for a target divisor D = (u, v) of degree g, look
  for a function y - w(x) with w = v + u * t, deg w <= g + k, so that the residual divisor has Mumford
  polynomial (f - w^2)/u, whose roots can be *sieved* over the free coefficients of t (as in NFS line
  sieving) instead of tested one by one.
- P. Sarkar, S. Singh, "A simple method for obtaining relations among factor basis elements for special
  hyperelliptic curves", ePrint 2015/179: for special f, a relation in about (2g+3)! trials, combines with
  the Joux-Vitse sieve; for curves over F_{q^n} (Nagao decomposition setting) [verified-snippet].

### 6.2 Vitse-Wallet (2015): improved sieving
- V. Vitse, A. Wallet, "Improved sieving on algebraic curves", LATINCRYPT 2015, LNCS 9230, 295-307,
  doi:10.1007/978-3-319-22174-8_16; HAL hal-01203086 (https://hal.sorbonne-universite.fr/hal-01203086).
  Abstract [verified-snippet]: best algorithms for small genus are IC + large primes; for hyperelliptic
  curves relations come from reduced divisors with smooth Mumford representation; for non-hyperelliptic
  curves it is faster to use special linear systems (Diem; Diem-Kochinke). The sieving technique of
  Sarkar-Singh (inspired by Joux-Vitse) is reformulated so it applies to the non-hyperelliptic case with
  or without large primes, giving **a speed-up by a factor ~3 for relation search in Diem's and
  Diem-Kochinke's methods** (non-hyperelliptic). This is NOT a measured hyperelliptic speed-up.
  Wallet's slides: https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf (sieve: fix one factor-base
  point, compute slopes of lines through it to the factor base, sieve on the residual) [verified-snippet].
- Implication for hyperelliptic g >= 2: a sieve replaces "one Jacobian addition + split test per candidate"
  by "amortised O(1) memory updates per candidate", a constant-factor gain only. The g! smoothness
  probability is unchanged by sieving; sieving only reduces the per-candidate cost.

### 6.3 Joux-Vitse: decomposition and cover-and-decomposition
- A. Joux, V. Vitse, "Elliptic curve discrete logarithm problem over small degree extension fields.
  Application to the static Diffie-Hellman problem on E(F_{q^5})", J. Cryptology 26 (2013) [memory]:
  Nagao/Diem decomposition with a sieve-like "n-1" variant, record on a 149/150-bit E over F_{p^5}
  [memory, unverified numbers].
- A. Joux, V. Vitse, "Cover and decomposition index calculus on elliptic curves made practical.
  Application to a previously unreachable curve over F_{p^6}", EUROCRYPT 2012, LNCS 7237, 9-26;
  ePrint 2011/020. Combines Weil descent (cover by a genus-g curve over a subfield) with decomposition
  IC on that curve. Example size reported as 151-bit in the Springer abstract, 149-bit in the introduction
  and 130-bit in an early ePrint version [verified-snippet: the inconsistency itself]. The cover curve is,
  to my memory, a genus-3 hyperelliptic curve over F_{p^2} [memory, unverified]. This is the only
  "genus-3 sieving record" I could locate; it is a *decomposition* attack (factor base = points over the
  base field of a curve over an extension), not Gaudry-style IC over a prime field.

### 6.4 Laine-Lauter (2015): time-memory trade-offs (genus 3, non-hyperelliptic)
- K. Laine, K. Lauter, "Time-memory trade-offs for index calculus in genus 3", J. Math. Cryptol. 9(2)
  (2015) 95-114, doi:10.1515/jmc-2014-0033; ePrint 2014/346. A variant of Diem's IC for non-hyperelliptic
  genus 3 that lowers computational complexity at a moderate memory increase, improves complexity even at
  Diem's original memory, and analyses parallelisation [verified-snippet]. Mechanism [memory]: store
  more of the "lines through factor base points" structure (precomputed pairs) to reduce the number of
  failed split tests; the gain is in the constant / log factors, not in the q exponent.
- ECC 2015 slides: https://lfant.math.u-bordeaux.fr/ecc2015/documents/laine.pdf

### 6.5 Diem-Kochinke: pencils / special linear systems
- C. Diem, S. Kochinke, "Computing discrete logarithms with pencils" (preprint, 2017),
  https://www.math.uni-leipzig.de/~diem/preprints/pencils.pdf : for fixed genus g >= 5 non-hyperelliptic
  (heuristically, generic curves via Brill-Noether), O~(q^{2-2/(g-2)}) [verified-snippet]. Not applicable
  to hyperelliptic curves (the g^1_2 is the only low-degree pencil). This is the strongest evidence that
  the *hyperelliptic* model is the hardest case for IC at fixed genus.

### 6.6 Any sieve that replaces the random walk for hyperelliptic curves?
- Sarkar-Singh is the only published method explicitly designed to replace Gaudry's walk + split test with
  a sieve over the same degree-1 factor base [verified-snippet]. No published large-scale benchmark
  against GTTD 2LP with sieving was found; Vitse-Wallet's ~3x is for non-hyperelliptic curves.
- In the high-genus (b >= 2) regime, sieving in the Mumford/"v-polynomial" lattice was used by
  Flassenberg-Paulus (1999) and by Velichka-Jacobson-Stein (Math. Comp. 83 (2014) 935-963; ePrint
  2011/098), who add large primes, a smaller factor base and sieving to Enge-Gaudry in characteristic 2,
  and test on Weil-descent instances [verified-snippet for content summary].

---

## 7. Record computations (index calculus in Jacobians)

No maintained record table for Jacobian index calculus exists (unlike the finite-field DLP records).
What could be located:

| Genus / setting | Instance | Method | Cost | Source / status |
|---|---|---|---|---|
| g = 2 | No IC record found; genus-2 DLP records are generic (rho/Gaudry-Schost) | -- | -- | 2LP exponent q^1 = rho exponent; no reason to run IC [analysis] |
| g = 2 (endomorphism) | Experiments on genus-2 Jacobians with efficient endomorphisms: factor base reduced by the endomorphism orbit, "net complexity gain over the original attack" | Gaudry IC + orbit-reduced base | not quoted in abstract | Tsakou, Ionica, ePrint 2021/721; Math. Cryptology 1(2) (2021) 102-114, https://journals.flvc.org/mathcryptology/article/view/130882 [verified-snippet] |
| g = 3 hyperelliptic | GTTD: "for hyperelliptic curves of genus three, our first algorithm surpasses Pollard's Rho method even for rather small field sizes" | 2LP | sizes/timings not retrieved | GTTD 2007 [verified-snippet; numbers unverified] |
| g = 3 non-hyperelliptic | Diem-Thomé experiments on plane quartics, O~(q) heuristic | Diem's lines-through-two-points | numbers not retrieved | Diem, Thomé, J. Cryptology 21(4) (2008) 593-611, doi:10.1007/s00145-007-9014-6 [verified-snippet for paper; record size unverified] |
| g = 3 via cover | E over F_{p^6}, ~149-151-bit (130-bit in early ePrint) | Joux-Vitse cover-and-decomposition (Weil descent to a curve over a subfield, then Nagao/Diem-type decomposition with sieving) | "previously unreachable" curve | Joux, Vitse, EUROCRYPT 2012, LNCS 7237; ePrint 2011/020 [verified-snippet; cover genus memory] |
| g = 6 | Koblitz's proposed genus-6 cryptosystem broken in Gaudry 2000 | Gaudry IC, degree-1 base | field size not confirmed (search snippet garbled, possibly F_{2^23}) | Gaudry, EUROCRYPT 2000 [verified-snippet for "genus 6 ... proposed by Koblitz"; field unverified] |
| g ~ 31-32 over F_{2^5} | ECDLP on E/F_{2^155} via GHS Weil descent to a hyperelliptic curve over F_{2^5} | Enge-Gaudry style (b > 1) IC | "modest computer resources" | Jacobson, Menezes, Stein, J. Ramanujan Math. Soc. 16 (2001) 231-260; ePrint 2001/041 [verified-snippet for F_{2^155}; genus 31/32 inferred/snippet] |
| high genus, char 2 | improved EG with large primes, smaller base, sieving; applied to Weil-descent instances | VJS | not retrieved | Velichka, Jacobson, Stein, Math. Comp. 83 (2014) 935-963; ePrint 2011/098 [verified-snippet] |
| high genus, char 2, GLS | DLP on prime-order subgroup of a GLS curve over F_{2^{5*31}}, via GHS to genus-g hyperelliptic over the subfield; GLS endomorphism gives factor-n speed-up in IC | EG-style IC + endomorphism orbits, Magma | ~1035 CPU-days | Chi-Domínguez, Rodríguez-Henríquez, Smith, Finite Fields Appl. (2021), doi:10.1016/j.ffa.2021.101891; arXiv 2106.09967; ePrint 2021/676 [verified-snippet] |

Gaps: GTTD and Diem-Thomé experimental tables (field sizes, CPU time, matrix sizes) are in the full texts
that could not be fetched (HAL inria-00000897; core.ac.uk/download/50471813.pdf). **Action for the lab:**
read those two experimental sections once eprint/HAL/arXiv are allowed; they are the only published
genus-3 IC-vs-rho measured constants.

Lab-internal (not literature): `aburan28/crypto` PR #366, "Benchmark hyperelliptic index calculus
against Pollard rho" (https://github.com/aburan28/crypto/pull/366) reports genus-2 IC losing to rho with
the ratio S_ic/S_rho growing from 3.0 to 8.8 with field size [search snippet; `[pcc:unverified]`]. This is
what the literature predicts: equal exponent q^1, worse constant.

---

## 8. Genus 6..16: where IC overtakes rho, which method wins

**Published statements.**
- Gaudry 2000: faster than rho for g > 4 (degree-1 base, q^2) [verified-snippet].
- Thériault 2003: improvement for g > 2 and beats rho for g >= 3 asymptotically [verified-snippet].
- GTTD 2007 / Nagao 2007 / Diem 2011: O~(q^{2-2/g}), fixed g [verified-snippet].
- Diem cover-attack preprint: "if the genus of H would be >= 4 (and maybe >= 3), index calculus attacks on
  Cl^0(H/K) would be more efficient than generic attacks like Pollard rho; if the genus is 1 or 2, generic
  attacks are more efficient" [verified-snippet; https://www.math.uni-leipzig.de/~diem/preprints/cover-attacks.pdf].
- Enge-Gaudry / Enge: L(1/2) only once g >= vartheta log q (Section 2).
- Maurer-Menezes-Teske (LMS JCM 5 (2002) 127-174; ePrint 2001/084) contains running-time estimates of
  EG-type IC as a function of genus and field size for GHS instances (N in [100, 600]); I could not retrieve
  its tables [verified-snippet for scope only]. Menezes-Teske-Weng ("Weak fields for ECC", CT-RSA 2004)
  similarly [memory].
- **No published concrete crossover table for hyperelliptic g = 6..16 at prime q was found.**

**Derived crossover (analysis, not literature).** Take the 2LP cost as g! * q^{2-2/g} (ignoring polylog
factors and the binom(g,2) gain from allowing 2 large primes) and rho as q^{g/2}. IC wins when
log2 q > log2(g!) / (g/2 - 2 + 2/g):

| g | Gaudry reduced q^{2-2/(g+1)} | Thériault q^{2-4/(2g+1)} | 2LP q^{2-2/g} | rho q^{g/2} | log2(g!) | IC wins for log2 q > | at log2 #J ~ |
|---|---|---|---|---|---|---|---|
| 2 | 1.333 | 1.200 | 1.000 | 1.0 | 1.0 | never | -- |
| 3 | 1.500 | 1.429 | 1.333 | 1.5 | 2.6 | 15.5 | 47 |
| 4 | 1.600 | 1.556 | 1.500 | 2.0 | 4.6 | 9.2 | 37 |
| 5 | 1.667 | 1.636 | 1.600 | 2.5 | 6.9 | 7.7 | 38 |
| 6 | 1.714 | 1.692 | 1.667 | 3.0 | 9.5 | 7.1 | 43 |
| 8 | 1.778 | 1.765 | 1.750 | 4.0 | 15.3 | 6.8 | 54 |
| 10 | 1.818 | 1.810 | 1.800 | 5.0 | 21.8 | 6.8 | 68 |
| 12 | 1.846 | 1.840 | 1.833 | 6.0 | 28.8 | 6.9 | 83 |
| 14 | 1.867 | 1.862 | 1.857 | 7.0 | 36.3 | 7.1 | 99 |
| 16 | 1.882 | 1.879 | 1.875 | 8.0 | 44.3 | 7.2 | 116 |

So for g >= 5 at any cryptographic group size, exponential IC already beats rho by a huge margin in the
idealised model; e.g. g = 8, q = 2^32 (#J ~ 2^256): rho ~ 2^128 vs 2LP ~ 2^71 (incl. g!).

**Which method wins at each (g, log q): b = 1 (2LP) vs b = 2 (Enge-Gaudry).** Plugging the heuristic
L_{q^g}(1/2, sqrt2) with o(1) := 0 (optimistic, and outside its proven regime) gives (log2 costs):

| g \ log2 q | 4 | 8 | 16 | 32 |
|---|---|---|---|---|
| 6: rho / 2LP / EG | 12 / 16.2 / 14.0 | 24 / 22.8 / 22.0 | 48 / 36.2 / 34.1 | 96 / 62.8 / 52.1 |
| 8 | 16 / 22.3 / 16.9 | 32 / 29.3 / 26.5 | 64 / 43.3 / 40.7 | 128 / 71.3 / 61.8 |
| 12 | 24 / 36.2 / 22.0 | 48 / 43.5 / 34.1 | 96 / 58.2 / 52.1 | 192 / 87.5 / 78.7 |
| 16 | 32 / 51.8 / 26.5 | 64 / 59.3 / 40.7 | 128 / 74.3 / 61.8 | 256 / 104.3 / 93.1 |

Caveat: the EG numbers are NOT achievable as written when the optimal smoothness degree b is < 2:
b must be an integer, and with b = 2 the factor base has ~q^2/2 elements so linear algebra alone is ~q^4
(e.g. g = 16, q = 2^32: 2^128, worse than 2LP's 2^104). The L(1/2) formula is meaningful in this table only
where q is small (log2 q <~ 8-12) and g >= 8, i.e. **for g = 9..16 with q <= ~2^12, a degree-2 factor
base (EG/ADH, with large primes as in Velichka-Jacobson-Stein) is the likely winner; for log2 q >= ~16,
degree-1 base with 2 large primes wins at every g <= 16** (analysis). The intermediate design -- factor
base = degree-1 places, large primes allowed to be degree-1 *or degree-2* places -- interpolates between
the two and is not analysed in any paper I found for fixed g (VJS do large primes in high genus).

---

## 9. Recent work 2018-2026 (incl. negative results)

Explicit searches (ePrint, arXiv, Springer, HAL; terms: hyperelliptic index calculus 2018-2026, genus,
factor base, sieving, triple large prime) found **no new exponent improvement for hyperelliptic IC at
fixed genus since GTTD/Nagao (2007) and Diem (2011)**. The q^{2-2/g} bound has stood ~19 years. Items:

- **Faugère, Wallet (2018)**, "The point decomposition problem over hyperelliptic curves: toward efficient
  computations of discrete logarithms in even characteristic", Des. Codes Cryptogr. 86(10) (2018);
  HAL hal-01658573. Gröbner modelling of point decomposition (Nagao setting, curves over F_{q^n}); number of
  solutions grows exponentially with n; in even characteristic it can be cut by up to 2^{(n-1)(g+1)}
  (genus-2 over F_{2^93}: 4096 -> 64) [verified-snippet]. Applies to extension fields only.
- **Sarkar, Singh (2017)**, DCC version of the sieving decomposition (Section 6.1) [verified-snippet].
- **Tsakou, Ionica (2021)**, endomorphism-reduced factor base, genus 2 (and ordinary EC) [verified-snippet].
  Gain is a constant factor ~ (orbit size) in the base, partly offset by costlier per-element work.
- **Chi-Domínguez, Rodríguez-Henríquez, Smith (2021)**, GLS endomorphism -> factor-n speed-up of IC on the
  GHS Jacobian, 1035 CPU-day computation [verified-snippet].
- **(l,l,l)-isogenies for genus 3** (arXiv 2007.03172, "Translating the discrete logarithm problem on
  Jacobians of genus 3 hyperelliptic curves with (l,l,l)-isogenies", 2020/2021) extends Smith's
  hyperelliptic -> non-hyperelliptic transfer beyond (2,2,2) [title verified; details unverified]; and
  arXiv 2104.04919 "Isogenous hyperelliptic and non-hyperelliptic Jacobians with maximal complex
  multiplication" (2021) [title verified].
- **Bhati, Singh (SPACE 2025, LNCS 16406, 58-78)**, "A new perspective on the decomposition in the Jacobian
  of small genus hyperelliptic curve": weak curves over extension fields where decomposition is easy;
  time complexity "remains the same as that of the Sarkar-Singh decomposition" [verified-snippet]. A
  negative result in the sense that no exponent change is claimed.
- **Diem, Kochinke (2017 preprint)** pencils: O~(q^{2-2/(g-2)}) for non-hyperelliptic g >= 5 (heuristic,
  Brill-Noether) [verified-snippet]; *inapplicable* to hyperelliptic curves -- an implicit negative result
  for hyperelliptic models.
- **Galbraith, Gaudry (2016)**, "Recent progress on the elliptic curve discrete logarithm problem",
  DCC 78(1) 51-72, ePrint 2015/1022: survey; cautions about complexity claims for summation-polynomial
  index calculus; explicitly does not survey genus > 1 in depth [verified-snippet].
- 2026: ePrint 2026/2057, "Low-space quantum discrete logarithms on genus-two Jacobians" -- quantum
  resource estimate, not classical IC [verified-snippet]. Not relevant to classical speed-ups.
- **No "triple large prime" analysis for hyperelliptic IC was found** (search returned only 2LP work).
  For NFS, 3LP gives constant gains; for fixed-genus IC the analogous exponent would be q^{2-2/g} still
  (Diem's lower-bound-style arguments for the LP graph suggest no exponent gain beyond 2 LPs) [memory;
  unverified].

---

## 10. Biggest speed-up levers by regime

Notation: exponents are for group operations up to polylog factors; "2LP" = GTTD/Nagao double large
prime. Rho exponent is g/2. Confidence refers to the lever, not the baseline.

| g | Best known algorithm (hyperelliptic, prime-field q) | Exponent in q (rho) | Dominant cost | Single lever most likely to give a large constant or exponent gain | Confidence note |
|---|---|---|---|---|---|
| 2 | Rho / Gaudry-Schost (IC 2LP ties at q^1) | 1 (1) | n/a for rho; for IC: LA ~ relations | Endomorphism/automorphism-orbit factor-base reduction (Tsakou-Ionica 2021) to shrink IC constants -- but IC cannot beat rho in exponent; realistic goal is only narrowing the constant gap | High that IC cannot beat rho asymptotically (GTTD exponent = 1); lab PR #366 data agree |
| 3 | 2LP O~(q^{4/3}); for ~18.6% of curves (and more via (l,l,l)) isogeny transfer to plane quartic + Diem O~(q) | 4/3 (3/2) | Balanced in exponent; LA heavy in practice (dense cycle relations), memory ~q^{2/3+} | **Isogeny transfer to non-hyperelliptic** (Smith 2008; (l,l,l) 2020) changes exponent 4/3 -> 1 where applicable; otherwise sieving the relation search (Sarkar-Singh / Vitse-Wallet style, ~3x) | High for exponent gain on the applicable fraction (published); medium for 3x sieving transfer to hyperelliptic (measured only non-hyperelliptic) |
| 4 | 2LP O~(q^{3/2}) | 3/2 (2) | Relation search grows with g! = 24; LA ~ q^{3/2} | Sieve-based relation generation replacing walk+split-test (Sarkar-Singh), plus LA engineering (filtering + block Wiedemann) | Medium; constant gains only; no published hyperelliptic measurement |
| 5 | 2LP O~(q^{8/5}) | 8/5 (5/2) | Relation search (g! = 120) | Same; plus constant-aware re-optimisation of base fraction r (published optima balance exponents only) | Medium |
| 6-8 | 2LP O~(q^{2-2/g}) (1.67-1.75) | 5/3..7/4 (3..4) | Relation search: g! = 720..40320 per relation | Attack the g! term: sieving and/or allowing degree-2 large primes (hybrid of 2LP and EG) to raise smoothness probability; huge margin vs rho already | Medium-high that relations dominate (follows from g!); lever unpublished for fixed g |
| 9-12 | 2LP for log2 q >~ 16; EG/ADH degree-2 base with large primes (VJS-style) for small q | 1.78-1.83 (4.5-6) | Relations (g! ~ 2^18..2^29) | **Hybrid factor base: degree-1 base + degree-1/2 large primes, or b = 2 for small q** -- the only lever acting on the g!/smoothness term in the exponent-constant tradeoff | Medium-low: no published fixed-genus analysis; derived from EG and GTTD formulas |
| 13-16 | Same split; EG L_{q^g}(1/2) formally applies only if g >= vartheta log q (q <~ 2^{g}) | 1.85-1.875 (6.5-8) | Relations (g! ~ 2^32..2^44) | Degree-2 (or higher) smoothness base for q <~ 2^12; otherwise hybrid large primes; Weil-descent-style endomorphism orbits when the curve has them (factor-n, CRS 2021) | Low-medium: asymptotic EG constants (sqrt2 heuristic, 5/sqrt6 rigorous) have large o(1) at these sizes; no table published |

**Cross-cutting conclusions.**
1. The exponent q^{2-2/g} (GTTD 2007; Nagao 2007; Diem 2011) is the frontier for hyperelliptic curves at fixed
   genus; no improvement found 2007-2026. Exponent gains come only from *changing the curve model*
   (isogeny to non-hyperelliptic for g = 3; Diem plane-curve O~(q^{2-2/(d-2)}); Diem-Kochinke pencils for
   non-hyperelliptic g >= 5) -- none available for generic hyperelliptic g >= 4.
2. For g >= 5 the published exponents already beat rho by a factor q^{g/2 - 2 + 2/g}; the lab's "distance
   to rho" is dominated by implementation constants (g!, polylog, memory/LA) at small q, not by exponents.
3. The g! smoothness factor is the dominant hidden constant from g ~ 6 upward and is left untouched by
   large primes and by sieving; only enlarging what counts as "smooth" (degree-2 places, hybrid LPs) acts
   on it. This is the least-explored lever in the literature for fixed g.
4. Sieving levers are measured only for non-hyperelliptic curves (~3x, Vitse-Wallet 2015) and extension
   fields (Joux-Vitse); a hyperelliptic prime-field sieve benchmark against 2LP is absent from the
   literature -- an open, measurable target.

Status: COMPLETE (2026-11-17). All literature statements `[pcc:unverified]`; primary-PDF reading blocked
for eprint.iacr.org, arxiv.org, hal.archives-ouvertes.fr in this session.

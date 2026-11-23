# Round 7: directions outside the summation-polynomial framework

2026-11-04 · research design, reading and theory only. **Class: proposal. No
experiment was run, no algorithm is priced, and there is no `S` value, rho
comparison or leaderboard row.** Every factual or result sentence that is not
a citation carries `[pcc:unverified]` (repository rule). Paper citations are
marked *(confident)* or *(uncertain)* according to how sure I am that the
paper exists as cited. None of them was re-read for this note.

Inputs: rounds 1–5 of this study (`../README.md`, `../THEORY.md`,
`../round4/CROSSOVER.md`, `../round5/SOLVERS.md`), and the items surveyed in §1.
The lead atlas (`crypto-autoresearcher/knowledge/leads/ecdlp/`) is not checked
out in this container, so this survey does not cover it. [pcc:unverified]

## 0. The barrier these proposals must meet

Rounds 1–5 report four results. [pcc:unverified]

- **Gröbner degree.** D rises by about +1.5 per summand and +0.5 per tower
  level, because of the partial-sum separators X = x(P_i + … + P_j).
- **Counting bound.** Any solver that decides a separator by its value
  (branching, tabulation, MITM, a tensor bond) ranges over a set of size at
  least √p, so it pays at least √p.
- **Characters.** Character and rationality information about a separator is
  invisible over F̄_p.
- **Crossover.** The dense-Macaulay law crosses rho only beyond about 2^500.

Below, the counting bound is applied in a general form: **identify the
intermediate object the method fixes by value, and count its domain.** If that
domain has size at least √p for a random target, the method is generic in
effect, whatever its algebra looks like. [pcc:unverified]

## 1. Survey: the lab's nearest alternatives (15 bullets)

1. **`cryptanalysis/experiments/prime-ic-jacobian-relations-20261031`.**
   Despite the name, this is Jacobian *coordinates* (X:Y:Z) for the
   relation-collection walk, not the Jacobian of a curve. Correctness gates
   pass: 3,042 unit tests and all `ic_prime` integration tests. No timing was
   collected because the checkpoint upload failed. The best it can give is a
   constant factor. [pcc:unverified]
2. **`crypto/research/prime_ic_jacobian_benchmark_20261101`.** The paired
   affine-vs-Jacobian timing protocol. It is preregistered, and no rows have
   been collected. [pcc:unverified]
3. **`RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.1, Theorem 15.1 (the generic-group
   null theorem).** Adding oracles for every rational isogeny in the class,
   with their scalars λ_ψ, leaves success probability at most (Q+2)²/r + 1/r.
   So isogeny *maps* are worthless in the generic model, and only coordinate
   effects can matter. Prop. 15.2 (affine saturation) prices
   torsion- and automorphism-saturated bases as a class-factor constant.
   [pcc:unverified]
4. **Same report, H6 (Jacobian correspondences / genus-2 covers through a
   fiber).** By Riemann–Hurwitz, fiber products only raise the genus, and it
   records "over prime fields no cover attack exists for any E". Status:
   negative. [pcc:unverified]
5. **`../THEORY.md` Lemma A (proved) and the Ritt dichotomy (heuristic).** A
   tower that is compatible with the group law is an isogeny tower, which
   reduces to Theorem 15.1. Kummer and Dickson towers belong to the nodal
   group. [pcc:unverified]
6. **`research/prime_one_shot_20261029` (Riemann–Roch / divisor-coefficient).**
   For odd m = 2h−1, the system is A² − fC² = (X − r_x)·U₀ with U₀ | X^d − c,
   imposed by a quadratic remainder tower V_j² = Q_jU₀ + V_{j+1}. It has
   O(mt) variables and input degree 2. Status: 162 correctness checks on F₁₇
   fixtures. It was **never solved for unknown coefficients at a
   balanced size, and no degree-growth law was measured.** [pcc:unverified]
7. **`research/prime_shared_coefficient_sat_20261031`.** Same predicate. The
   E0 gate passes (18/18 exact model-set matches on F₁₇). No external solver
   has been run on a study instance. [pcc:unverified]
8. **`prime_one_shot_20261029/continuation/tensor-review`.** The October-2026
   preprints claim prime-field ECDLP in 2^{O(b/(log b)^{1/4})} through Boolean
   tensor contraction. The claim is conditional on Appendix D of *The
   Exponential Time Hypothesis is False*. A within-session review found no
   concrete gap; this is not independent verification. Round 5's rank bound
   covers exact contraction over a bond indexed by a separator's value. The
   preprint's encoding (Fourier-transform vertices of degree O(b^{3/4})) is
   claimed to avoid such a bond. Nobody has checked this. [pcc:unverified]
9. **`research/enge_primefield_cover_20261024`.** Covers C: y² = f(t^d + s)
   → E, with π_*(div h) = div(Norm h). In an exhaustive F₁₀₁ test, every
   cover relation from linear sections equals a direct elliptic norm relation,
   so the cover adds no new relation family. Enge's bound needs g ≥ θ log p,
   which gives an exponent above rho's. [pcc:unverified]
10. **`research/cover_route_prime_binary_20261008`.** F_p has no proper
    subfield. IC on Jac(C) of genus g over F_p costs Õ(p^{2−2/g}) ≥ p.
    Descending from F_{p^k} gives g ≥ k, and at best about p^{4/3}.
    [pcc:unverified]
11. **`teske_variants_field_types_20261018` §3, `cover_descent_genus_floor_20261015`.**
    No prime-field descent variant exists. The GHS genus floors concern
    extension fields only. [pcc:unverified]
12. **`jv_cover_crossover_20261101`, `ic_cover_leads_20261018`.** The
    Joux–Vitse genus-3 cover of E(F_{p^6}), in a weak class with full
    rational 2-torsion. At p = 1451 it is below `rhodp` but not resolved
    against `rhostrong`. It is not a prime-field route. [pcc:unverified]
13. **`smallroot_optimal_reach_20261026`, `prime_s5_small_root_leads_20261028`
    (lattices).** For any Coppersmith lattice built from (p, f)^t, the S₃
    reach is at most 0.33 against a required 0.89. At m = 4 and a feasible
    dimension, δ ≈ 1/313. [pcc:unverified]
14. **`jksst_xedni_audit_20261101` (lifting to Q).** A Lean audit of the
    conditional JKSST analysis. The xedni failure mechanism (heights,
    probability about (2C₂)^r/p) stands as conditional. [pcc:unverified]
15. **`ic_prime_eth_leads_20261031`.** Twelve SAT, backdoor and hashed-base
    leads, with no exponent movement. Lever-2 null: relation counts on
    structured bases equal those on random bases (5,760 instances, cited in
    `../THEORY.md`). [pcc:unverified]

## 2. Proposals, ranked

Ranking is by the information an affordable experiment would give. It is not
a chance of success; none is expected to beat rho at 256 bits.
[pcc:unverified]

### Rank 1 — Separator-free Riemann–Roch norm-divisibility system under F4

**Status: live and untested at the decisive point.** It is outside
*summation polynomials* (there is no S_n and no partial sums), but it is
still the point-decomposition problem. [pcc:unverified]

**Mechanism.** The relation is a single function A(X) + C(X)y in L((m+1)O)
whose divisor's finite part lies over V = {x : x^d = c}. With x-only targets
(as in arms T, C and F), the unknowns are:

- A: monic, degree h;
- C: degree ≤ h − 2;
- U: monic, degree m;
- the tower polynomials V_j, Q_j.

The equations are

    A² − f·C² = (X − x_R)·U,
    V₀ = X,  V_j² = Q_j·U + V_{j+1}  (j < t),  V_t = c.

Every equation is quadratic. X^d − c is separable, so U | X^d − c already
forces U to be squarefree. [pcc:unverified]

**Why it escapes separator coupling.** There are no partial-sum variables. All
m points are coupled at once, through ≈ m coefficient unknowns and a single
degree-2 identity, not through a tree of m − 1 S₃ nodes of degree 4. Round 5
attributes the +1.5 per summand to "couplings × node degree", and here both
factors change. The encoding is also automatically S_m-quotiented, because a
divisor is unordered. It is the only known encoding that combines full symmetry
with input degree bounded in m, which is the condition `../THEORY.md` §3 says
an L(1/2) law requires. [pcc:unverified]

The cost is the variable count: N ≈ (2m − 1)t + 2m against ≈ mt + O(m) for T.
At m = 3, t = 4 that is 33 against 9–13. [pcc:unverified]

**Counting bound.** The intermediate objects are the factors of U. A
branching or MITM solver that fixes a factor U₁ | U of degree ⌈m/2⌉ by value
ranges over C(d, ⌈m/2⌉) ≈ |V|^{m/2} ≈ √p candidates at balanced parameters.
So **only algebraic solvers can win**, exactly as for T. [pcc:unverified]

The Kronecker bound (round 5 (b)) applies if the membership equations are
eliminated first, because the membership variety has C(d, m) ≈ p points. F4
must therefore interleave the norm identity with the tower. [pcc:unverified]

**Cheapest falsifying experiment.**

- **Inputs.** The round-1 curve over F₇₈₆₄₃₃ (p − 1 = 2^18·3) and the planted
  targets of `../instances.txt` for odd m. Use x_R only, with V = g·μ_{2^t}.
  Cells m ∈ {3, 5, 7}, t ∈ {3, 4, 5, 6}. [pcc:unverified]
- **Tools.** A Singular generator in the style of `../gen.sing` that aborts on
  a nonzero planted evaluation. msolve 0.6.5, one thread, 600 s cap,
  `ulimit -v` 3.5 GB. Use the existing `../parse.sh` to read D, the widest
  matrix and the quotient dimension. [pcc:unverified]
- **Positive control.** Plant the F₁₇ fixtures of `prime_one_shot_20261029`
  with all variables unknown. Expect 28 standard monomials at m = 3, matching
  `canonical/unbound-check`. [pcc:unverified]
- **Comparator.** T and F rows from `../rows.tsv` and rounds 2–3 at the same
  (m, t, target). [pcc:unverified]
- **Kill criterion (pre-register in `.resume/RESUME.md`).** The direction is
  killed if either of these holds:
  - D_RR ≥ D_T in every comparable cell; or
  - D_RR's m-slope is ≥ 1.5 per summand at t = 4.

  "Trade, not improvement" if D_RR < D_T but the widest matrix is larger in
  every cell. An improvement needs a smaller D **and** a narrower widest
  matrix in at least one m ≥ 5 cell. [pcc:unverified]
- **Expected run time on 4 cores / 15 GB.** 12 cells at ≤ 600 s each, 4 in
  parallel, is about 30–40 min wall. The m = 7 cells will likely hit the cap,
  which gives lower bounds only. Under 1 h including the generator.
  [pcc:unverified]
- **Prior art.**
  - Semaev, *Summation polynomials and the discrete logarithm problem on
    elliptic curves*, ePrint 2004/031 *(confident)*.
  - Nagao, *Decomposition formula of the Jacobian group of plane curve*,
    ePrint 2013/548 *(confident it exists; the lab notes a retracted
    proposition)*. This is the Riemann–Roch decomposition.
  - Petit–Kosters–Messeng, *Algebraic approaches for the elliptic curve
    discrete logarithm problem over prime fields*, PKC 2016 *(confident)*.
    This is the tower membership.
  - Semaev, *New algorithm for the discrete logarithm problem on elliptic
    curves*, ePrint 2015/310 *(confident)*. This is auxiliary-variable
    splitting.
  - Galbraith–Gaudry, *Recent progress on the elliptic curve discrete
    logarithm problem*, DCC 78 (2016) *(confident)*.

### Rank 2 — Jacobians of curves that cover E, with relations by Mumford-divisor sampling

**Status: expected negative, but closes the lab's open "Jacobian relation"
item for degree-g divisors.** [pcc:unverified]

**Mechanism.** Take a cover π: C → E over F_p (for example C: y² = f(t² + s),
genus 2, as in `enge_primefield_cover_20261024`), or a genus-2 C with
Jac(C) ~ E × E′. Sample h = a(t) + b(t)y, or random reduced Mumford
divisors (u, v), on C. Test u for splitting over a degree-1 factor base by
univariate factorization, and project the relations to E by π_*. There is no
polynomial system and no separator variable. [pcc:unverified]

**Why it does not escape.** The obstruction is a different counting bound,
not separator coupling. [pcc:unverified]

- **Norm inclusion.** For the double cover, Norm(a(t) + b(t)y) is even in t,
  so it equals some A(x) + C(x)y on E. π_*(div h) = div(Norm h), so every
  projected relation is a relation of an elliptic function, with the factor
  base restricted to the half of E(F_p) where x − s is a square. The cover can
  only change *sampling efficiency*, by a constant. [pcc:unverified]
- **Smoothness.** A random-sampling generator fixes the reduced divisor by
  value: that is the separator bound applied to sampling. For a degree-1 base
  B, a degree-m norm splits with probability ≈ |B|^m/(m!·p^m) (genus 1) or
  1/g! with |B| ≈ p (genus g). This gives a cost of at least
  p·(m!)·(p/|B|)^{m−1} ≥ p at genus 1, and Gaudry/GTTD p^{2−2/g} ≥ p at
  g ≥ 2. Either way it is at least √p. [pcc:unverified]
- **Large genus.** Enge-type L(1/2) needs g ≥ θ log p. That puts the cost in
  L_{p^g}(1/2) = exp(Θ(log p·√(log log p))), above rho's ½ log p (lab item 9).
  [pcc:unverified]

**Cheapest falsifying experiment.**

- **Purpose.** Extend the Enge census from linear sections to full degree-2
  Mumford divisors and to h with deg a ≤ 3, deg b ≤ 1. [pcc:unverified]
- **Inputs.** E/F_p at p ∈ {101, 1009, 10007}, and C = y² = f(t² + s) with
  s chosen so that f(s) ≠ 0. [pcc:unverified]
- **Tool.** PARI/gp (installed: `/usr/bin/gp`), with exhaustive or ≥ 10⁶
  sampled h. [pcc:unverified]
- **Kill criterion.** Both of these hold:
  - every fully split projected relation is in the direct elliptic norm set;
    and
  - the split rate per sample is within 2× of |B_s|^k/(k!·p^k), where B_s is
    the square-class half-base.

  If both hold, the direction is closed for the lab. [pcc:unverified]
- **Run time.** Minutes at p ≤ 10⁴ on one core. [pcc:unverified]
- **Prior art.**
  - Gaudry, *An algorithm for solving the discrete log problem on
    hyperelliptic curves*, Eurocrypt 2000 *(confident)*.
  - Gaudry–Thomé–Thériault–Diem, *A double large prime variation for small
    genus hyperelliptic index calculus*, Math. Comp. 76 (2007) *(confident)*.
  - Enge, *Computing discrete logarithms in high-genus hyperelliptic Jacobians
    in provably subexponential time*, Math. Comp. 71 (2002) *(confident)*.
  - Diem, *On the discrete logarithm problem in class groups of curves*,
    Math. Comp. 80 (2011) *(confident)*.
  - Kani, *The number of curves of genus two with elliptic differentials*,
    J. reine angew. Math. 485 (1997) *(confident)*. This covers split
    Jacobians.

### Rank 3 — The relation as an AG-code decoding problem (list decoding and polynomial lattices over F_p[X])

**Status: expected negative at the Johnson radius. Its only upside is
structured decoding beyond Johnson.** [pcc:unverified]

**Mechanism.** Fix one sign class s(X) ≡ √f (mod X^d − c), the CRT of the
roots y_α. All m points of a relation carry the chosen signs with probability
2^{−m}, so the cost of the sign choice is subexponential. With s fixed, a
relation is a rational function A/C of degrees (h, h − 2) that agrees with the
received word (s(α))_{α ∈ V} in ≥ m of d positions. This is rational
Reed–Solomon / elliptic AG-code decoding. Its solvers are interpolation and
Popov/weak-Popov reduction of F_p[X]-modules. These are geometry of numbers
over F_p[X], with no partial sums. [pcc:unverified]

**Why it does not escape, as far as I can see.** The obstruction is a
list-size bound, not separators. [pcc:unverified]

- **Johnson radius.** Guruswami–Sudan succeeds only for agreement
  > √(d·(2h − 1)) ≈ √(dm). The relation has agreement m. Since d ≈ p^{1/m}
  ≫ m, the target lies far beyond the Johnson radius at every balanced
  parameter. This is derived and needs no run. [pcc:unverified]
- **Polynomial Coppersmith analogue.** Asking for A − sC ≡ 0 modulo an unknown
  degree-(m + 1) factor of (X − x_R)(X^d − c) has reach deg ≲ β²·d with
  β ≈ m/d. That needs h ≲ m²/d, so it fails by a factor of about d/(2m). This
  mirrors the lab's integer small-root deficit (item 13). [pcc:unverified]
- **Folding.** Folded codes (Guruswami–Rudra) reach list-decoding capacity,
  but only when agreements come in blocks of an automorphism orbit. On E, the
  orbits are translations by rational torsion or the μ₃/μ₄ automorphisms. By
  Prop. 15.2 / Theorem 15.1, these reduce to IC on E/⟨T⟩ or give a class
  constant. [pcc:unverified]
- **Counting bound.** Below Johnson, the number of agreeing codewords is the
  relation count C(d, m)/p^{m−1}-ish ≈ 1 per target, and lever-2 already
  measured it to equal the random count (item 15). Any decoder that tests
  candidate agreement sets by value pays ≥ C(d, ⌈m/2⌉) ≈ √p. [pcc:unverified]

**Cheapest falsifying experiment.** First, a derived Johnson-reach table for
(m, t) at 2^64 … 2^256. Then a structure test: at p = 786433, d = 2^t with
t = 6..9, m = 3, enumerate the agreement-m rational codewords exactly and
compare their number and degree profile with a random evaluation set of the
same size. [pcc:unverified]

- **Size and tool.** C(512, 3) ≈ 2.2·10⁷ subsets, a few minutes in gp or a
  short native program. [pcc:unverified]
- **Kill criterion.** The Kummer set's agreement-m list size matches the
  random set's within the Poisson 95 % band at every t. Also, no
  sub-Johnson algebraic structure is visible: the Popov degree of the
  interpolation module equals the generic value. [pcc:unverified]
- **Run time.** Under 30 min on one core. [pcc:unverified]
- **Prior art.**
  - Guruswami–Sudan, *Improved decoding of Reed–Solomon and algebraic-geometry
    codes*, IEEE Trans. IT 45 (1999) *(confident)*.
  - Guruswami–Rudra, *Explicit codes achieving list decoding capacity*, IEEE
    Trans. IT 54 (2008) *(confident)*.
  - Cheng–Wan, *On the list and bounded distance decodability of Reed–Solomon
    codes*, FOCS 2004 / SIAM J. Comput. 37 (2007) *(confident)*. Deep RS
    decoding is as hard as finite-field DLP; this is the reduction in the
    opposite direction.
  - Ben-Sasson–Kopparty–Radhakrishnan, *Subspace polynomials and limits to
    list decoding of Reed–Solomon codes*, IEEE Trans. IT 56 (2010)
    *(confident)*. Structured evaluation sets give super-polynomial lists just
    beyond Johnson.
  - Cheng, *Hard problems of algebraic geometry codes*, IEEE Trans. IT 54
    (2008) *(fairly confident; the ECDLP link to elliptic-code decoding is
    from memory)*.
  - Guruswami–Xing, folded codes from function-field towers, STOC 2012
    *(uncertain on title)*.

## 3. Examined and rejected (not ranked)

- **Mordell–Weil lattices of elliptic surfaces through the target (a xedni
  over F_p(t)).** Specialization MW(ℰ) → E(F_p) is a homomorphism. Once
  G ↦ v_G and Q ↦ v_Q are prescribed, every other image is either a known
  combination of them or a new point with an unknown log. So information
  arises only from (a) a by-value collision between images on different
  surfaces, which is birthday-bound at about √p/240 surfaces, or (b) forcing
  an (r + 1)-th prescribed point. [pcc:unverified]

  For (b), a rational elliptic surface with its fiber at t = 0 fixed has
  about 8 free parameters and rank ≤ 8 (Shioda–Tate). Forcing 9 points is
  overdetermined by 1, so it succeeds with probability ≈ 1/p per tuple. The
  forced condition is itself an algebraic condition Σc_iP_i = O, which is a
  summation polynomial in disguise. [pcc:unverified]

  The one loophole I found is that rank can exceed moduli over F_p:
  supersingular K3 surfaces have ρ = 22 in a ≤ 9-dimensional family (Artin),
  and Ulmer's y² + xy = x³ − t^d is rigid with large rank. Whether their
  sections can be made to pass through *given* F_p-points was not worked out.
  [pcc:unverified]

  Prior art:
  - Silverman, *The xedni calculus and the elliptic curve discrete logarithm
    problem*, DCC 20 (2000) *(confident)*.
  - Jacobson et al. (JKSST), DCC 20 (2000) *(confident)*.
  - Silverman–Suzuki, Asiacrypt 1998 *(confident)*.
  - Shioda, *On the Mordell–Weil lattices*, Comment. Math. Univ. St. Pauli 39
    (1990) *(confident)*.
  - Ulmer, *Elliptic curves with large rank over function fields*, Ann. Math.
    155 (2002) *(confident)*.
  - Huang–Kueh–Tan, ANTS-IV 2000 *(uncertain on details)*.
- **Isogeny-graph or modular-curve relation generation.** Theorem 15.1 makes
  the isogeny maps generic. Lemma A sends group-law-compatible coordinate
  towers to an isogenous curve. Over F_p, E carries no modular
  parametrization usable without a lift to Q, which is the xedni family. Not
  proposed. [pcc:unverified]

## 4. Summary table

| rank | direction | outside summation polys? | escapes separator coupling? | binding bound | kill test cost |
|--:|---|---|---|---|---|
| 1 | Riemann–Roch norm-divisibility, F4 | yes (no S_n, no partial sums); same PDP | yes, algebraically; enumeration still ≥ √p via factor splits | Macaulay size with N ≈ 2mt | ≤ 1 h, msolve |
| 2 | Jacobian covers, Mumford sampling | yes | yes, but smoothness counting gives ≥ p | norm inclusion + p^{2−2/g} | minutes, gp |
| 3 | AG-code list decoding / F_p[X] lattices | yes | yes, but the Johnson radius gives ≥ √p | list size beyond Johnson | ≤ 30 min, gp |

All entries in this table restate §2. [pcc:unverified]

Recommended order: run rank 1 first. It is the only direction whose decisive
measurement is missing from the lab record, and its kill criterion is sharp.
Ranks 2 and 3 are closure experiments for open lab items. [pcc:unverified]

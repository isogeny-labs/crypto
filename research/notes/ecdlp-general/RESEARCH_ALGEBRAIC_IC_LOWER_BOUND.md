# A conditional lower bound for algebraic index calculus over a prime field: what is proved, what is a gap

**Date.** 2026-10-31. **Status.** Theorem draft (lead L1 of
[`RESEARCH_PRIME_FIELD_LEADS_20261031.md`](RESEARCH_PRIME_FIELD_LEADS_20261031.md)).
No experiment. Every lemma is labelled **PROVED**, **PROVED given H** (H named in
the hypotheses table, section 2) or **GAP** (exact missing statement given).
Numbers marked *measured elsewhere* are quoted from the cited records and were
not re-measured here. The only arithmetic computed here is the threshold table
of section 5 (awk, bisection, data file beside the graph).
**Output directory.** `research/prime_field_leads_20261031/L1_lower_bound/`
(REPORT.md, proof-dependency graph, threshold graph, PDF).

**Sources read and cited by path.**
`research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` sections 1.1-1.3, 4.1;
`research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md` sections 1-2 (PKM summary);
`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` section 3.7;
`/Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md` (M005, M008, M021: Yokoyama-Yasuda);
the Yokoyama-Yasuda-Takahashi-Kogure paper text
(`/Volumes/SSD990-2/research/extracted_text/complexity-bounds-on-semaev-s-naive-index-calculus-method-1f7yzrgu06.txt`);
atlas card `KR-IC-401ebf` (`crypto-autoresearcher/knowledge/frontiers/ecdlp/index-calculus/KR-IC-401ebf.yaml`)
and the lead cards `LEAD-ICE-prime-field-semaev-solving-degree`, `LEAD-ICE-generic-oracle-arity-cost-floor`,
`LEAD-ICP-pkm-tower-algebraic-oracle`;
`research/notes/ecdlp-general/RESEARCH_REPRESENTATION_STRUCTURE.md` TL;DR (Shoup);
`docs/ic/RESEARCH_ISOGENY_CONDUCTOR_GAP_20261007.md` section 0 (what Jao-Miller-Venkatesan covers).

## 0. Summary

1. **What generalises KR-IC-401ebf.** That card is a *heuristic* known negative
   ("under stated statistical assumptions", all sources read in abstract only).
   The YYTK paper's Conjecture 10 and Assumption 11 are about the *dense*
   single-polynomial system `<S_{m+1}, F(x_1),...,F(x_m)>`; the paper does not
   treat auxiliary-variable towers (its text mentions PKM only as related work).
   So "Yokoyama-Yasuda on the PKM tower" is **not** supported by the paper.
2. **What is proved here, with no statistical assumption:** for a factor base
   cut out by a polynomial in one coordinate with no auxiliary variables
   ("direct presentation"), any solver charged the rank of the Macaulay matrix at a
   degree where its output is determined costs `>= c_m * r` in total, for every
   base size (Theorem 1; given only the two bookkeeping hypotheses H_C, H_E).
   That is *above* the `r^{1/2}` target and above exhaustive search.
3. **What is not proved:** the same for PKM-style towers. The whole theorem for
   `m >= 3` reduces to one statement about the tower presentation (GAP A4: the
   solving degree grows linearly in the number of levels and the rank is close to
   the number of columns). Theorem 2 makes the bar explicit: in-model
   sub-`r^{1/2}` cost is possible **iff** the per-attempt cost exponent
   `c < (m-1)/2`, and `c` is computed from the solving-degree slope. The slope bar
   is the quantitative form of the next action of `LEAD-ICP-pkm-tower-algebraic-oracle`.
4. The brief's claim "total cost `>= p^{1/2-o(1)}` for every arity m" is therefore
   **conditional** (Theorem 2, given H_tower) in the tower model; it is
   **unconditional modulo H_C, H_E** only for the direct presentation.

## 1. The model

Notation: `r` the prime order of the subgroup (the bound is in `r`; cofactor
ignored), `m` the arity (fixed), `V` a set of abscissae with `|V| = v`,
`F = {P : x(P) in V}` closed under negation, `B = |F| = 2v`. The relation
`R = a P + b Q` with `(a,b)` known, `R` a target, is *decomposed* when
`R = sum_{i<=m} P_i`, `P_i in F` (a multiset of at most `m` elements).

* **Presentation.** `V` enters the solver only through a polynomial system `Sigma`.
  *Direct:* generators `S(x_1..x_m)` (the Semaev polynomial `S_{m+1}` with `x_{m+1} = x(R)`
  substituted, total degree `s <= (m+1) 2^{m-1}`, degree `2^{m-1}` in each `x_i`) and
  `L(x_i)`, `i = 1..m`, `L = prod_{a in V}(X - a)` or any polynomial whose zero set in
  `F_p` is `V`. *Tower (PKM):* `L = L_t o ... o L_1`, `deg L_j = d`, one auxiliary
  variable per level and summand, so `n = m t` unknowns and generators of degree
  `<= max(d, s)`.
* **Macaulay charge.** `M_D(Sigma)` is the span of all monomial multiples of generators
  of total degree `<= D`. A solver *determines the answer at degree D* if its
  output (solution set or refutation) for the system is a function of the row space
  `M_D`, for all targets `R`. `D_sol` is the least such `D`. The charge of one
  decomposition attempt is `rank M_{D_sol}` (at least; real solvers pay more).
* **Linear algebra charge:** at least the number of columns it is handed (`>= |F*|`
  below).
* **Attempt.** One target `R`, uniformly random `(a,b)`, one solver call.
* **Outside the model by construction:** anything that reads `V` other than through
  `Sigma` (an interval test on a root, a pair table, MITM), large-prime variations,
  precomputation across curves, re-encodings that change the presentation.
  Section 6 tabulates which repository routes are inside.

## 2. Hypotheses table

| Id | Statement | Used by | Status |
|---|---|---|---|
| H_C | The relation matrix has no kernel supported on a proper sub-base `F* < F` found more cheaply than the full system (the algorithm gains nothing by shrinking `F` post hoc); precisely, the expected number of relations inside any `F*` the algorithm can name is `<= C(|F*|+m, m)/r (1+o(1))` | Lemma C2 | GAP (first-moment bound in a random hypergraph model is routine; for the real hypergraph it is an energy statement of C4 type) |
| H_E | Targets chosen by the algorithm are no more decomposable than uniform ones: `E[r_F(R_j)] <= (1+o(1)) C(B+m,m)/r` per attempt | Lemma E2 | GAP, tested by C3/C4 |
| H_T(alpha,rho) | For the tower presentation with `t` levels: `D_sol >= alpha t - O(1)` and `rank M_{D_sol} >= (columns)^{rho - o(1)}` | Theorem 2 | GAP, open; this is the card's next action |
| H_YY | Yokoyama et al. Conjecture 10 and Assumption 11 (statistical) | Remark A5 only (F4 / signature selection, dense system) | conjecture in the paper |
| H_JMV | GRH; ECDLP random self-reducible among curves of equal `End` (Jao-Miller-Venkatesan 2005) | interpretive remark D2 only | external |

## 3. Lemmas

### (a) Degree of the membership polynomial and the Macaulay rank

**A1. PROVED (root counting; this is not Bezout).** A nonzero polynomial with `v`
distinct roots has degree `>= v`. Hence `deg L >= |F|/2` for any `L` cutting out `V`
in one coordinate. For a tower, `prod_j deg L_j = deg L >= v`, so `t >= log_d v =
log_d(B/2)`, and the tower has `n = m t >= m log_d(B/2)` unknowns.

**A2. PROVED (direct presentation).** If `D < deg L` then `M_D` is the span of
multiples of `S` alone, which does not depend on `V`. Take `V` and `V'` differing in
one element, and `R` the target of a decomposition using that element: the answers
differ (a solution exists for `V`, none for `V'` or the solution sets differ) but
`M_D` is identical, so the answer is not a function of `M_D`. Hence
`D_sol >= deg L >= v`. (A univariate-eliminant formulation also works:
`(S) cap F_p[x_1] = 0` because `S` has positive degree in `x_2`.)

**A3. PROVED (direct presentation).** For every `D`, `rank M_D >= C(m + D - s, m)`:
distinct monomial multiples of the nonzero polynomial `S` are linearly independent
(`F_p[x]` is a domain). With A2, `rank M_{D_sol} >= C(m + v - s, m) = (B/2)^m/m! (1+o(1))`
for `B >> s`. That is the cost of enumerating the unordered `m`-subsets of `V`: the
membership polynomial of a dense base buys nothing below brute force.

**A4. GAP (tower presentation).** *Missing statement:* H_T(alpha, rho) above, for the
Semaev-plus-tower system on every curve in the class. What is proved: `n >= m log_d v`
(A1) and `rank M_D >= C(n + D - s, n)` (A3 argument unchanged), so the entire content
is a lower bound on `D_sol(t)`. Nothing proved bounds it below by anything but `s`. The
repository records (*measured elsewhere*, `LEAD-ICP-pkm-tower-algebraic-oracle`
`best_measured`, and the lead note section 1): F4 solving degree grows with `N`
on every tower family tried, `m = 2: 3 4 4 4 4 5 5 5 5 6` for `N = 2..20`,
`m = 3: 5 6 6 7` at `N = 6..15`, `m = 4: 6 7` at `N = 8, 12`; the shape-matched random
null gives the same `D`. The derived side (`prime_fall_degree_20261027/DERIVED.md`
D1-D8 as quoted in the card) gives the upper bound `d_F <= 2t + 2` and "no growing
lower bound proved". This note does not have that file in the worktree and does not
re-derive it.

**A4' (conditional exponent, PROVED given H_T).** Let `D_sol >= alpha t`,
`cols = C(mt + alpha t, alpha t)`. By Stirling,
`log2 cols = t (m+alpha) H2(alpha/(m+alpha)) (1+o(1))`, `H2` the binary entropy.
With `B = 2 d^t` this gives `rank >= B^{c}`,

    c(alpha, m, d, rho) = rho (m + alpha) H2(alpha/(m+alpha)) / log2 d.

**A5. Remark (what Yokoyama-Yasuda does and does not give).** Under H_YY the number of
necessary S-polynomials for the dense system is `~ d^m - (d-delta)^m ~ m delta d^{m-1}`
(`d = v`, `delta = 2^{m-1}`), i.e. `~ v^{m-1}` per attempt, and the paper's trivial-ideal
case gives `#supp >~ d^m`. Plugged into the same bookkeeping as Theorem 1 this gives
`T >= c'_m r` as well, but only under H_YY, and only for the dense presentation: the
paper's Conjecture 10 and Assumption 11 concern the reduced Groebner basis of
`I_m(a,b)` with `J = <F(x_i)>`; it has no auxiliary variables. Do not cite it for towers.
A3 is the unconditional replacement for the Macaulay-charged solver; F4 with
selection can use fewer rows than the full Macaulay matrix and is covered only under H_YY.

### (b) Cost shape, the MITM comparison, and the threshold `e < m/2 - 1`

**B1. PROVED given H_C, H_E (cost shape).** Let `r(B)` be the per-attempt charge and
`A` the number of attempts. Each attempt yields in expectation
`mu = E[#decompositions] = C(B+m, m)/r` relations (E1 below, exact over a uniform
target). The linear algebra needs `>= 2|F|/m` relations (C1, C2) and `>= |F|` columns.
By Wald's identity `E[A] mu = E[relations collected] >= 2B/m`, so `E[A] >= (2B/m)/mu`,
and `A >= 1`:

    T  >=  max( B ,  max(1, m! r / B^{m-1}) * r(B) )   up to constants depending on m only.   (1)

This is the form to optimise **over B**; the bound is not stated at `B = r^{1/m}`.

**B2. PROVED (derivation of the threshold).** Write `B = r^{beta}`, `r(B) = B^{c}`. The
cost exponent of (1) is `Phi(beta) = max(beta, max(0, 1 - (m-1)beta) + c beta)`.
* At `B = r^{1/m}` and `r(B) = r^{e/m}`: `T = m! r^{(1+e)/m}`; this beats `r^{1/2}` iff
  `e < m/2 - 1`. Enumeration has `e = m - 1`; the table-based comparison below has
  `e = m/2` (even `m`). So `m = 3` needs `e < 0.5` and `m = 4` needs `e < 1`.
  PKM's own condition (tower solve below `p^{1/2 - 1/m}`) is the same inequality
  in absolute terms, `r^{e/m} < r^{1/2 - 1/m}`.
* Over all `beta`: if `c >= (m-1)/2` then `Phi(beta) >= 1/2` for every `beta`
  (case `beta >= 1/2`: columns; `beta <= 1/(m-1)`: `1 - (m-1)beta + c beta >= 1 - (m-1)beta/2 >= 1/2`;
  `1/(m-1) < beta < 1/2`: `c beta >= (m-1) beta/2 > 1/2`). If `c < (m-1)/2` it fails at
  `beta = 1/(m-1)`. In general `min_beta Phi = 1/(m-c)` for `0 <= c <= 1` and `c/(m-1)` for
  `1 <= c <= m-1`. **The bar is therefore `c >= (m-1)/2`, stricter than the `m/2 - 1` of the
  single-point derivation** (worst case `B = r^{1/(m-1)}`, where one attempt already suffices
  and the cost is the solver charge alone).

**B3. Comparison algorithm (not a floor).** Table-based decomposition: store all
`a`-fold sums (`B^a` entries), and per attempt enumerate `(m-a)`-fold sums and look
each up. Time `B^a + m! r^{1/m} B^{m-a}` at `B = r^{1/m}`. Even `m`: `a = m/2`, total
`r^{1/2 + 1/m}`, memory `r^{1/2}` (the figure in the lead note section 2). Odd `m`:
`a = (m+1)/2` gives `r^{1/2 + 1/(2m)}` with memory `r^{(m+1)/(2m)}`, *smaller* than
`r^{1/2+1/m}`: the lead-note formula is exact for even `m` and conservative for odd `m`
(`m = 3`: `6 r^{2/3}` time, `r^{2/3}` memory). Either way it exceeds `r^{1/2}`, so the
conclusion of section 2 of the lead note is unchanged. This is an upper bound on a
non-algebraic route, not a lower bound. **GAP B4:** no generic-group lower bound
matching the table-based cost for `m`-fold decomposition in a prime-order group is
established here. The atlas card `LEAD-ICE-generic-oracle-arity-cost-floor` records
that structured generic-group bounds (CHW 2026/384) and non-adaptive time-space
bounds (DKM 2025/783) agree in-model; those were summarised, not re-read, in this
workflow.

### (c) Linear algebra and the relation count

**C1. PROVED.** If a combination of relation rows annihilates all base columns and has
`b != 0`, every column in its support appears in at least two rows of the
combination (coefficients are small nonzero integers, `r` is prime, so a column met once has
nonzero coefficient). With at most `m` columns per row, `#rows >= 2|F*|/m`, `F*` the support.
The linear algebra is charged `>= |F*|`.

**C2. PROVED given H_C.** With `F* = F`: `>= 2B/m` relations and `>= B` columns, as used in (1).
For `m <= 2`, (1) with `r(B) >= 1` gives `T >= max(B, 2r/B) >= sqrt(2r)`: **the relation count
alone excludes `m <= 2`** (`m = 1` gives `T >= r`). For `m = 3` the relation count alone gives
only `T >= r^{1/3}` (from `max(B, 6r/B^2)`). *Strengthening:* with the quadratic linear-algebra
charge `m B^2` of `RESEARCH_PKM_TOWER_ORACLE.md` section 3.7 the floor is `r^{2/(m+1)}`, so
`m = 3` ties `r^{1/2}` and `m <= 2` is excluded; that charge is that note's, not derived
here. **GAP C3 (= H_C):** relations supported on a proper sub-base. *Missing statement:*
the expected number of relations lying inside any set `F* subset F` the algorithm can
name is at most `(1+o(1)) C(|F*|+m, m)/r` times a union-bound slack that is `r^{o(1)}`.

### (d) Isogeny-class uniformity (JMV) and the CM panel

**D1. PROVED (no transfer needed).** Lemmas A1-A3, B1-B2, C1 and E1 depend on the curve
only through `r`, `m`, `s`, and the fact that `S` is a nonzero polynomial of positive degree
in at least two variables. They hold verbatim on every curve, in particular at every volcano
level and on every CM curve of the panel. A claimed CM or volcano effect inside the
direct-presentation model would contradict Theorem 1; inside the tower model it would
have to violate H_T *for that curve*, since the curve enters only through the coefficients
of `S`. (A4 is the single curve-dependent statement.)

**D2. Remark (what JMV does give, and its limit).** Jao-Miller-Venkatesan prove, under
GRH, that ECDLP is random self-reducible among curves with the same endomorphism ring;
vertical steps at small `ell` cost polynomial time. Large-prime conductor gaps are
the open case (`docs/ic/RESEARCH_ISOGENY_CONDUCTOR_GAP_20261007.md` section 0). So the
brief's "hardness is uniform over the isogeny class" is **not** a theorem for
all pairs of curves in a class. Moreover JMV transports *solvers*, not the in-model
quantity `D_sol`: a cheap in-model solve on `E'` would give a solver on `E` that contains
isogeny walks, which is outside the model. JMV is therefore not used in any proof here.
Experiment C2 (volcano) is the empirical test of D1 for the tower hypothesis.

**D3. PROVED given the cited section.** Endomorphisms and the Frobenius act on all points
at once, not pairwise (`RESEARCH_EXOTIC_COORDINATES.md` section 1.3); a coordinate invariant
under a torsion subgroup factors through an isogeny (section 1.2); degree-2 coordinates are
Mobius frames (section 1.1); section 4.1 records that over generic `F_p` only the
sign frame and, for `j = 0`, the `zeta_6` orbits remain. Folding the base by a group of order `g` of
automorphisms (`g <= 6`) divides columns by `g` and multiplies yield by `g`, so it divides
the bound (1) by a factor at most `g^2 <= 36`; the exponent is unchanged. Endomorphisms of
larger degree do not preserve a coordinate-defined `V` (their action on the `x`-line has
degree `> 1`), so no larger fold is available inside the model.

### (e) Yield

**E1. PROVED (the direction a lower bound needs).** For a uniformly random target in a
group of order `r` and *any* base `F` (structured or not),
`E[#multisets of at most m elements of F summing to R] = C(B+m, m)/r` exactly (double
counting). Hence the probability that `R` decomposes at all is at most this, and
the relations per attempt are at most `mu = (1+O(m/B)) B^m/(m! r)`. This holds for
interval, subgroup and random bases alike. **The task's formulation
`|F|^m/(m! N)(1+o(1))` as an equality is a conjecture only in the other direction**: the
probability of at least one decomposition equals `mu (1-o(1))` iff the number of
decompositions concentrates on {0,1}, i.e. iff the off-diagonal additive energy of the `m`-fold
sumset of `F` is `(1+o(1)) B^{2m}/r`. That is the pseudo-genericity conjecture C4 tests
(R8: 1.04x spread across coset, interval, random bases, *measured elsewhere*). Only
the tightness of the model (that it is not wasteful) needs it, not the lower bound.

**E2. PROVED given H_E (adaptive targets).** If the algorithm draws its targets from a set `W`
of density `f` (by any means costing at least one group operation per draw), then
`E[r_F(R) | R in W] <= mu / f` and `E[A] >= (2B/m) f/mu`, so the attempts shrink by at most
`1/f`, the self-reducibility ceiling of lead C3. **GAP E3 (= H_E):** *missing statement:*
for every algorithm that outputs `(a_j, b_j)` at expected cost `kappa >= 1`,
`E[r_F(a_j P + b_j Q)] <= (1+o(1)) mu`. Equivalently, `Dec_F(m)` is not a
set whose members can be sampled at cost `< 1/f` per member. This is the single place
where the group structure beyond Shoup is invoked, and it is the content of
`RESEARCH_REPRESENTATION_STRUCTURE.md` TL;DR (any gain must consume a fact about the
encoding that a random encoding would not have).

## 4. Theorems

**Theorem 1 (direct presentation; PROVED given H_C, H_E).** In the model of section 1 with
direct presentation, for every `B >= 1` and fixed `m`:
`E[T] >= c_m r`, with `c_m >= 2^{1-m}/m` for `B > B_0(m)` (from A3, (1)), and
`E[T] >= (2/m) m! r / B_0(m)^{m-1}` for `B <= B_0(m)` (attempt count alone). Hence in this
model algebraic index calculus costs `Omega_m(r)`, which exceeds both `r^{1/2}` and the
table-based comparison of B3.
*Proof.* A2, A3 give the per-attempt rank; B1 with `r(B) = (B/2)^m/m! (1+o(1))` gives
`T >= (2/m) 2^{-m} r B`; for small `B` use `A >= (2/m) m! r/B^{m-1}`. QED (given H_C, H_E).

**Theorem 2 (tower presentation; PROVED given H_T and H_C, H_E).** If the tower
presentation satisfies H_T(alpha, rho) with
`c = c(alpha, m, d, rho) >= (m-1)/2`, then `E[T] >= r^{1/2 - o(1)}` for every base size.
More generally `E[T] >= r^{c/(m-1) - o(1)}` for `1 <= c <= m-1` and `r^{1/(m-c) - o(1)}`
for `0 <= c <= 1`. *Proof:* B2 with the rank bound A4'. QED.

*Consequence.* "Total cost `>= p^{1/2 - o(1)}` for every arity `m`" holds in the tower model
**iff** the tower family satisfies `c >= (m-1)/2`; with `c = 0` (bounded `D_sol`)
the model gives only `r^{1/m}`, which is the PKM scenario. The claim is therefore
reduced, not proved, for `m >= 3`.

## 5. The quantitative bar on the solving-degree slope

With `rho = 1` (rank equals columns, the optimistic reading), the smallest linear slope
`alpha*` for which `c >= (m-1)/2` is the root of
`(m+alpha) H2(alpha/(m+alpha)) = ((m-1)/2) log2 d`. Computed here by bisection
(`threshold_alpha_star.dat`; asymptotic in `t`, offsets dropped):

| m | d=2: alpha* | alpha*/m | d=3: alpha* | alpha*/m | d=4: alpha* | alpha*/m |
|---|---|---|---|---|---|---|
| 3 | 0.1805 | 0.0602 | 0.3399 | 0.1133 | 0.4750 | 0.1583 |
| 4 | 0.2820 | 0.0705 | 0.5361 | 0.1340 | 0.7542 | 0.1885 |
| 5 | 0.3849 | 0.0770 | 0.7357 | 0.1471 | 1.0390 | 0.2078 |
| 6 | 0.4884 | 0.0814 | 0.9370 | 0.1562 | 1.3267 | 0.2211 |
| 7 | 0.5923 | 0.0846 | 1.1393 | 0.1628 | 1.6162 | 0.2309 |
| 8 | 0.6964 | 0.0871 | 1.3423 | 0.1678 | 1.9067 | 0.2383 |

*Reading (observations only).* For Kummer/Dickson/torus towers (`d = 2`) the bar is
`0.06-0.09` degrees per unknown. The slopes quoted from the repository (*measured elsewhere*; the lead note
section 1 states "about one degree per eight unknowns", `0.125`, for `m = 4`; the card's
cells give `3 -> 6` over `N = 2..20` at `m = 2`, `5 -> 7` over `N = 6..15` at `m = 3`,
`6 -> 7` over `N = 8..12` at `m = 4`) are of the same order as the bar and the cells are
few and noisy. Nothing here measured whether `rank` equals `cols`, whether the
slope persists beyond the reached `N <= 20`, or the `d = 3, 4` towers. The bar
`alpha*` is what the pre-registered fall-degree study
(`research/prime_fall_degree_20261027`, not present in this worktree) should be scored
against, together with a rank (not column) count.

## 6. Which measured routes are inside the model (falsifier check)

| Route | In / out | Covered by |
|---|---|---|
| Naive Groebner / eliminant on dense base (EV-SDEG-4a1c3d, `beta_work` 0.797 / 0.647 vs 0.30 gate) | in | A2-A3, Theorem 1 (measured values are consistent in direction; not compared numerically, units differ) |
| PKM / APS F4 (Kummer, Dickson, torus, isogeny chain) | in | A4 GAP, Theorem 2 |
| Root-finding per decomposition (enumerate `m-1`, solve last) | in as enumeration class | `e = m-1`, B2: total `Theta(r)` |
| MITM / pair table | out of the algebraic model | B3, generic-oracle card |
| One-hot / Boolean-digit re-encodings | out | `d_reg` falls at constant information (lead note section 1); the model charges rank and fixes the presentation, so a re-encoding is a different presentation that must satisfy its own H_T |
| Large-prime variations | out | they decouple the counting base from the columns, breaking C2 |
| Precomputation / multi-target | out | lead L3; a multi-target solve with `z` as a variable and `|T|` targets gives per-relation cost `>= r/(m+1)!` for the direct presentation (sketch only) |
| Symmetry-folded bases | in | D3, constant `<= 36` |

No lemma's hypothesis excludes a measured repository route inside the algebraic
model. The falsifier of the lead (an in-model algorithm below `r^{1/2}`) is not exhibited.

## 7. Gaps, stated exactly

1. **A4/H_T:** `D_sol(tower_t, m) >= alpha t - O(1)` with `alpha > alpha*_m`, and `rank M_D >= cols^{rho}` at that degree (the card's next action).
2. **H_C (C3):** relations inside a proper sub-base are not more numerous than the first-moment count.
3. **H_E (E3):** no algorithm samples decomposable targets faster than `1/f`.
4. **B4:** a generic-group lower bound matching the table-based decomposition cost.
5. **Charge gap:** F4 with selection charges fewer rows than the full Macaulay matrix; covered only under H_YY (A5).
6. **E1 tightness:** the equality `Pr[decomposes] = mu (1 - o(1))` (conjecture C4); needed for tightness only.

## 8. What this note does not claim

No speedup, no exponent change, no measurement. The `p^{1/2 - o(1)}` statement is *not* proved
for the tower family; it is reduced to H_T with an explicit bar. JMV is not used in a proof.
The atlas card `KR-IC-401ebf` stays `heuristic`: this note proves a Macaulay-charged
statement for the direct presentation and leaves the tower to H_T.

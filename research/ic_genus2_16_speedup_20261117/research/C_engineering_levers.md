# C. Engineering levers for index calculus in Jacobians of genus 2..16

Status: COMPLETE (first pass, 2026-11-17). Sections 0-11 written.
Date: 2026-11-17. Cost unit: FPM = M + 10 I (S counted as M), plus priced LA term.
All results here are literature/derivation claims: [pcc:unverified] unless noted.

## Outline
0. Cost model and where the money goes (relation search vs LA, by g)
1. Jacobian arithmetic (Cantor, explicit formulas, NUCOMP, balanced, projective, batch inversion, r-adding tables)
2. Smoothness / splitting test of Mumford u
3. Relation search replacing the walk (sieving, pencils, Diem lines)
4. Large-prime variation engineering
5. Linear algebra over Z/rZ (SGE/filtering, Wiedemann, Lanczos)
6. Automorphisms and Frobenius
7. Parallelism
8. Published end-to-end implementations and profiles
9. Ranked table of levers
10. Sources

## Notes on sources and verification status (read first)

- Network: web *search* worked in this session; *fetching* full texts failed for eprint.iacr.org,
  cspages.ucalgary.ca and hal.sorbonne-universite.fr (hosts not in the session allowlist). So
  primary-source claims below come from (a) search-result abstracts/snippets of the primary
  source, tagged **[snippet]**, (b) my recall of the paper, tagged **[recall, unverified]**, or
  (c) the lab's own smoke notes in this repo, tagged **[lab smoke]** with path. Nothing here is a
  verified result: every number is `[pcc:unverified]`.
- Derived formulas (operation counts I derive from schoolbook algorithms) are tagged **[derived]**
  and should be checked by counting in code before use.
- Lab context read (not edited): `research/hecdlp_fastgroup_20261116/NOTES.md`,
  `research/hecdlp_sieve_20261116/NOTES.md`, `research/hecdlp_sge_coord_20261116/NOTES.md`,
  `research/genus4_rho_refs_20261116/validation/validator-report.md` (baseline already has
  disc-character prefilter, `x^p mod u`, double large prime, peel, coordinate Wiedemann).

## 0. Cost model: where the FPM goes in a Gaudry-style IC

Per-trial cost of the classic random-walk relation search (r-adding walk, test the Mumford `u`):

    C_trial(g,q) = C_add(g) + C_test(g,q)

- `C_add(g)`: one Jacobian addition with a table element (Section 1).
- `C_test(g,q)`: decide whether `u` (monic, degree g) splits into linear factors over F_q
  (and, for a reduced/LP factor base, which roots lie in the base). Dominant part is
  `x^q mod u` by square-and-multiply **[derived]**: each step squares a degree-(g-1)
  polynomial (g(g+1)/2 M schoolbook) and reduces a degree-(2g-2) polynomial mod monic u
  ((g-1)g M), so about 1.5 g^2 M per bit:

      C_test ~= 1.5 g^2 log2(q) M  (+ one gcd(u, x^q - x) ~ g^2 M + ~g I, root finding only on survivors)

- Probability that a random reduced divisor of weight g is fully smooth over the full degree-1
  base is ~1/g! (Gaudry 2000; GTTD 2007) **[recall, standard]**; relations needed ~ |B|.

**Key consequence [derived].** At a fixed group size #J ~ q^g ~ 2^N (so log2 q = N/g):

    C_test / C_add ~= 1.5 g^2 (N/g) / (c_g g^2) = 1.5 N / (c_g g)

with c_g = C_add/g^2 in FPM: about 9 (g=2, Lange affine ~I+25M), 8-9 (g=3, I+67M), 12
(g=4, lab fast law 180M+2I), ~17-20 for generic Cantor (Stein's 17g^2 field ops). For the lab's
toy regime (N ~ 40-64):

| g | log2 q at N=64 | C_add (FPM) | C_test (M) | test/add |
|---|---|---|---|---|
| 2 | 32 | ~35 | ~190 | ~5.5 |
| 3 | 21 | ~77 | ~280 | ~3.7 |
| 4 | 16 | ~200 (lab fast) / 460 (Cantor) | ~380 | ~1.9 / 0.8 |
| 8 | 8 | ~1100 (Cantor est.) | ~770 | ~0.7 |
| 16 | 4 | ~4400 (Cantor est.) | ~1540 | ~0.35 |

So for g <= 4 the **splitting test, not the group law, dominates relation search**; for g >= 8
the group law dominates. Arithmetic levers (Section 1) and test levers (Section 2) should be
prioritised accordingly. Sieving (Section 3) removes *both* and is the largest lever on the
relation phase, but moves cost into LA.

## 1. Jacobian arithmetic

### 1.1 Cantor's algorithm as a function of g
- Replaces: nothing (it is the generic baseline).
- Cost: Stein (2001) counts about **17 g^2 + O(g)** field operations for a generic addition
  [snippet, via Scheidler slides]; a naive textbook implementation with polynomial XGCD has
  several inversions (g-dependent; the lab's Cantor shows 5 I at g=3, 7 I at g=4).
- Lab measured [lab smoke, `hecdlp_fastgroup_20261116`]: g=3 Cantor 219 M + 5 I; g=4 Cantor
  393 M + 7 I per generic addition (= 269 / 463 FPM).

### 1.2 Explicit formulas (g = 2, 3, 4)
| g | source | affine add | affine dbl | inversion-free / projective |
|---|---|---|---|---|
| 2 | Lange 2005 (AAECC), eprint 2002/147 | ~I + 22M + 3S [recall, unverified] | ~I + 22M + 5S [recall] | projective add ~47M+4S [recall]; weighted/"new" coordinates add 36M+5S (mixed?), dbl 32M+7S [snippet, Robert slides] |
| 3 | Wollinger/Pelzl/Guajardo/Paar; Fan-Wollinger-Gong | **I + 67M** (rational Weierstrass point) | **I + 68M** | 122M+9S add, 110M+11S dbl (Fan-Wollinger-Wang, prime fields) [snippet] |
| 3 | Sutherland 2016 (arXiv 1607.08602), no rational Weierstrass pt | I + 79M | I + 82M | [snippet] |
| 4 | Pelzl-Wollinger-Paar SAC 2003 (eprint 2003/097) | **2I + 160M + 4S** [snippet, garbled extraction] | 2I + 193M + 16S [snippet, garbled] | binary-field projective: 203M+18S add, 63M+19S dbl (Abarzua et al.) [snippet] |
| 4 | lab `FastJ` | 180 M + 2 I | 206 M + 2 I | batched: 186 M + 0.03 I (W=64) [lab smoke] |

- Before -> after (FPM, I = 10 M): g=3 269 -> 77 (3.5x); g=4 463 -> ~200 (2.3x, lab) or ->
  ~184 (Pelzl, 2.5x). g=2: generic Cantor at g=2 is ~3I+70M (Pelzl et al. table, [snippet]) =
  100 FPM -> Lange affine ~35 FPM (2.9x).
- Generated formulas beyond g=4: Asgari, He, Jacobson, Scheidler (CASC 2026) generate explicit
  formulas that "predominantly outperform" Cantor and NUCOMP in runtime for **g = 2..10**
  [snippet; counts not obtained]. This is the most relevant recent source for g = 5..10 and should
  be fetched once `cspages.ucalgary.ca` is reachable.
- Regime: matters most where C_add is a large share of C_trial, i.e. g >= 4 (Section 0 table);
  at g = 2, 3 the test dominates and a 3x cheaper add gives only ~1.2-1.5x on relation search.
- Effect on total IC cost: <= the relation-phase share times (1 - 1/speedup_add) times the add
  share. **Caveat**: the matched rho reference uses the same group law, so the IC/rho ratio
  improves only by the part of IC cost that is *not* group arithmetic. The lab already observed
  this: switching both arms to the fast law moved Qcal only 0.008 -> 0.017
  (`hecdlp_fastgroup_20261116/NOTES.md`). For the IC-baseline improvement criterion, a faster
  law is a real measured improvement; for the rho gap it is nearly neutral (rho gets it too).

### 1.3 NUCOMP / NUDUPL
- Replaces Cantor composition + reduction: NUCOMP (Shanks; Jacobson-van der Poorten 2002)
  stops the partial XGCD early so intermediate polynomials have degree ~g instead of ~2g.
- Cost: C g^2 with C < 17 (asymptotic constant smaller than Cantor's) [snippet]. Crossover with
  Cantor reported around **g ~ 7-8** (early), later Avanzi-Jacobson-Scheidler found NUCOMP faster
  "for most hyperelliptic curves except very small genus over small fields" [snippet].
- For IC: relevant to g = 8..16 where the group law dominates relation search. Expected gain on
  C_add ~1.2-2x at g = 10-16 [estimate, unverified]; but Asgari et al. 2026 show explicit
  formulas beat NUCOMP up to g = 10, so NUCOMP is the method of choice only for g ~ 11-16.

### 1.4 Balanced divisor arithmetic (even-degree / real models)
- For deg f = 2g+2 (two points at infinity), balanced representation (Galbraith-Harrison-Mireles
  Morales 2008; "Balanced NUCOMP", Lindner-Imbert-Jacobson) makes the group law cost comparable
  to the imaginary model. NUCOMP "is the method of choice for all but the smallest genera" in this
  setting [snippet].
- For IC: relevant only if the target curve has no rational Weierstrass point (cannot be moved to
  an odd-degree model). Lever value is "avoid a 1.5-2x penalty", not a speed-up over the
  imaginary model. Also the smoothness test must then handle the extra infinity contribution
  (u of degree g or g+1); minor.

### 1.5 Projective / inversion-free coordinates and Montgomery batch inversion
- With I = 10 M, affine g=2 add ~I+25M = 35 FPM vs projective ~50 FPM: **affine wins at
  I/M = 10**, as it does at g = 3 (I + 67M = 77 vs 131) [derived from counts above]. Projective only
  pays when I/M > ~25.
- Montgomery simultaneous inversion: W inversions -> 1 I + 3(W-1) M. With W parallel walks,
  amortised I cost -> 3 M. Lab measured g=4: 2 I -> 0.03 I + ~6 M extra per add, i.e. 200 ->
  186 FPM (**7 %**) [lab smoke]. At g=2 the inversion is a larger fraction: I+25M = 35 FPM ->
  ~28 FPM (**~1.25x**). This lever is cheap and certain *in the FPM metric* because the metric
  prices I at 10 M; it is not a wall-clock gain at p < 2^31 (lab note).
- Caveat: batching needs W independent walks/trials; trivially available in IC (relation search
  trials are independent; no need even for a walk structure, see 1.6).

### 1.6 Table-driven walks (r-adding)
- IC relation search does not need a pseudo-random *walk*: it needs many random-looking divisors
  with known representation `sum a_i P_i + b_i Q`. An r-adding walk (Teske; r ~ 20 gives
  near-random behaviour [recall]) costs one addition with a precomputed table element per
  trial; with the table stored affine and the walk point in affine, cost = C_add (mixed coordinates
  only matter for projective).
- Cheaper alternative [derived]: **enumerate sums of two table elements** `T_i + T_j` (or
  `D + T_i` for fixed D) — still one add per trial, but trials are independent, so Montgomery
  batching is free, and when the lab's coordinate Wiedemann needs only known coefficients this is
  equivalent. No asymptotic change; the per-trial cost is still one add. The real lever is
  removing the add entirely (sieving, Section 3).
- Note for rho: the matched rho also uses an r-adding walk with one add/step, so this is not a
  differential lever.

## 2. Smoothness / splitting test of the Mumford u (per-trial cost)

Baseline (lab): disc-character prefilter, then `x^p mod u`, gcd, root finding
(`validator-report.md` lists `disc_character`, `ModU`, `roots_of`). Cost per Section 0:
C_test ~ 1.5 g^2 log2 q M plus the prefilter.

### 2.1 Repeated squaring for gcd(u, x^q - x)
- Cost [derived]: log2 q squarings mod u at ~1.5 g^2 M each (schoolbook), plus ~popcount(q)
  multiplications by x (g M each, cheap). Karatsuba squaring cuts the g^2/2 part to ~g^1.58
  for g >= 8 (squaring share ~1/3 of the step, so <= 1.3x overall at g = 16). Barrett/Montgomery
  reduction with a precomputed reversed inverse of u does not change the g^2 order for g <= 16.
- Root finding (Cantor-Zassenhaus, equal-degree splitting) only on survivors (~1/g! of trials
  with full base, more with large primes); amortised it is negligible except at g = 2.

### 2.2 Closed-form splitting for g = 2, 3, 4 (strongest test lever at small g)
- Replaces x^q mod u with radical formulas. [derived; lab has a `ferrari` flag in `run_ic_ext`,
  default off in the measured arms]:
  - g = 2: u = x^2 + a x + b splits iff Delta = a^2 - 4b is a square: 2 M + one Legendre symbol;
    roots need one square root (~log2 q M, Tonelli-Shanks/Atkin, only on survivors).
    Cost before ~6 log2 q M (~190 M at q = 2^32) -> after ~2 M + chi.
  - g = 3: Cardano: square disc is necessary (Stickelberger); then a cube root in F_q (or F_q2)
    decides 0 vs 3 roots. ~1-2 log2 q M vs 13.5 log2 q M.
  - g = 4: Ferrari: resolvent cubic + up to 3 square roots: ~3-5 log2 q M vs 24 log2 q M.
- **Accounting caution**: the Legendre/Jacobi symbol can be computed by a binary Euclid-type
  algorithm with no field multiplications. In an FPM-only metric this would make the g=2 test
  look nearly free. That is a relabelling risk: either price the Jacobi symbol as its
  exponentiation equivalent (~1.5 log2 q M), or price its word operations. With exponentiation
  pricing, g = 2 test ~ 1.5 log2 q M (4x cheaper than 6 log2 q), g = 3 ~ 3-4x, g = 4 ~ 4-5x.
- Also a table: for q <= 2^24 a quadratic-residue bitmap (q/8 bytes) makes chi a lookup. The lab
  already treats table lookups as uncounted (`chi`); keep that disclosed.
- Expected factor on relation-phase FPM [derived, from Section 0 shares]: g=2: test was ~85 % of a
  trial -> trial cost /3-4; g=3: ~78 % -> /2.5-3; g=4: 65 % -> /2-2.5.

### 2.3 Cheap filters: Stickelberger / discriminant character
- Stickelberger: chi(disc u) = (-1)^(g - r), r = number of irreducible factors. A fully split u has
  square discriminant, so ~half the candidates are rejected [standard]. Cost: disc via
  resultant(u, u') ~ O(g^2) M (subresultant, a few I) + chi.
- Gain: at most 2x on C_test (the lab already has it: "parity prefilter").
- There is no analogous cheap character for "number of roots >= g - 2" beyond parity; further
  filters (e.g. evaluating u at a few small-x points) give nothing for random u.

### 2.4 Distinct-degree factorisation for degree-b factor bases (g large, q small)
- When g is large relative to log q (Enge-Gaudry regime; for us g = 12-16 with log2 q ~ 4-8 at
  N ~ 64-128), a factor base of places of degree <= 2 may be needed. DDF then needs x^q and
  x^(q^2) mod u: second power by modular composition with a precomputed Frobenius matrix (g x g,
  ~g^2 log q M to build, g^2 M per application) -> C_test ~ (1.5 log2 q + 1) g^2 M. Negligible
  extra over the degree-1 test.
- Early abort (as in NFS cofactorisation): compute gcd(u, x^q - x) first; if the number of rational
  roots < g - L (L = allowed large primes) abort before root finding. This is already implied by
  the order of operations; savings small.

### 2.5 Batch testing (product / remainder trees)
- Bernstein-style batch smoothness (integers: product tree of primes, remainder tree, repeated
  squaring) [snippet: standard procedure]. Polynomial analogue for a **reduced** factor base B:
  u | P_B (P_B = prod_{a in B}(x - a), degree |B|) iff u is squarefree with all roots in B. A
  remainder tree over n ~ |B|/g candidates computes all P_B mod u_i in ~M(|B|) log |B| ops.
- Per candidate [derived]: ~ g log^2 |B| M with FFT-class multiplication, vs 1.5 g^2 log2 q.
  Gain ~ 1.5 g log2 q / log2^2 |B|. With |B| = q^alpha: gain ~ 1.5 g / (alpha^2 log2 q).
  - g = 4, log2 q = 16, alpha = 0.75: 0.67 -> **no gain**.
  - g = 16, log2 q = 8, alpha = 0.9: ~3.7x on the test, but the test is only ~25 % of the trial at
    g = 16 (Section 0), so ~1.2x on relation search.
  - Requires fast polynomial multiplication over F_q (NTT-friendly p or Kronecker substitution);
    with Karatsuba only, the gain disappears.
- Verdict: batch trees are a weak lever in this regime; closed forms (g <= 4) and sieving
  (Section 3) dominate.

### 2.6 Trade-off summary for the test
| g | baseline C_test (M) | best lever | after (M) | factor on test |
|---|---|---|---|---|
| 2 | ~6 log2 q | discriminant + chi (priced 1.5 log2 q) | ~1.5 log2 q + 5 | ~4x |
| 3 | ~13.5 log2 q (x0.5 with parity filter) | Cardano | ~3 log2 q | ~2-4x |
| 4 | ~24 log2 q (x0.5 with parity) | Ferrari | ~5 log2 q | ~2.5-5x |
| 5-8 | 1.5 g^2 log2 q | parity filter (have), Karatsuba | x0.8 | ~1.2x |
| 9-16 | 1.5 g^2 log2 q | remainder tree (if NTT field) | x0.3-0.7 | ~1.5-3x on test, ~1.1-1.3x on trial |

## 3. Relation search that replaces the random walk

### 3.1 Hyperelliptic sieving (Sarkar-Singh; Joux-Vitse idea; Vitse-Wallet reformulation)
- Sources: Sarkar-Singh, eprint 2014/815 (DCC 2017): a decomposition method over Gaudry's factor
  base that "admits a sieve that removes the need to smoothness-test polynomials, and needs fewer
  Jacobian additions than Gaudry's approach" [snippet]. Vitse-Wallet, LATINCRYPT 2015 "Improved
  sieving on algebraic curves": reformulates the large-prime technique, applies to
  non-hyperelliptic curves, reports **~3x speed-up of relation search in Diem's and Kochinke's
  methods** [snippet]. Exact families and timings not obtained (full texts unreachable).
- Mechanism (as reconstructed in the lab, `hecdlp_sieve_20261116`): functions y - v_t(x), v_t =
  v + t u, where (u, v) interpolates g fixed factor-base points; (f - v_t^2)/u = R_t has degree
  g+1. A base abscissa a is a root of R_t exactly for t = (+-y_a - v(a))/u(a), so one pass over B
  with a batched inversion fills a counter array over t in F_q. No group operation, no x^q.
- Cost per relation [derived, full degree-1 base fraction beta = |B|/q]:
  - walk: g! beta^(-g) (C_add + C_test)
  - sieve (one parameter): (g+1)! beta^(-(g+1)) * c_s, with c_s ~ (2g + 5) M per base element per
    family (two evaluations, batched inversion) spread over q candidates, i.e. per candidate
    ~2 beta (2g+5) M.
  - ratio sieve/walk ~ 2(g+1)(2g+5) / (C_add + C_test): g=3: 88 / ~220 -> **~2.5x cheaper**;
    g=4: 130 / ~390 -> **3x**; g=8: 378 / ~1900 -> **5x**; g=16: 1258 / ~6000 -> **~5x**.
  - Lab smoke (p = 1031..2053, g = 3,4) measured relation-phase native work per relation down
    **3-6x** and group operations down 50-300x [lab smoke], consistent with this.
- The catch: **relation weight 2g+1** (g fixed + g+1 roots) versus g for the walk. LA cost scales
  with row weight and with fill-in under SGE. Lab smoke: LA up 2-5x, total native work 0.8-2x of
  baseline (no tuning) [lab smoke]. Mitigations:
  1. Family column: keep one virtual column per family (the g fixed points as one vertex) so each
     row has weight g+2; pays off only if families yield >= 2 relations. Yield per family ~
     q beta^(g+1)/(g+1)!: large in full-base regime (g=3, q=2^20, beta=1/2: ~2700), < 1 in the
     DLP-reduced-base regime unless large primes are counted. [derived]
  2. Allow large primes among the g+1 residual roots (the counter threshold g+1-L) -- lab does.
  3. Rebalance |B| after the switch (Section 5.0 elasticities): a cheaper relation phase moves the
     optimum to a smaller base, which cuts LA.
- Regime: largest at g >= 4 where both C_add and C_test are large; requires q small enough for
  a counter array of size q per family (or segment it). For q > 2^32 segment the t-range.

### 3.2 Pencils through anchor points (general form)
- Same idea with k < g fixed anchors: g+1-k free coefficients -> (g+1-k)-dimensional lattice
  sieve; relations still have 2g+1 places (weight 2g+1, or k-anchor vertex + (2g+1-k)). More
  parameters give more candidates per anchor set (fewer distinct anchor vertices) but each
  candidate must have 2g+1-k base places, probability ~beta^(2g+1-k)/(2g+1-k)! -- worse per
  candidate. k = g (one parameter) is the natural choice. [derived]
- Sarkar-Singh 2015/179 extends to curves over F_{q^2} (Nagao-style decomposition) with relations
  in ~(2g+3)! trials, implemented for g = 5, 6 [snippet]. This is an extension-field (Weil-
  descent-type) setting; relevant to the lab only for curves over F_{q^n}.

### 3.3 Diem's line method for plane curves (non-hyperelliptic)
- For a plane model of degree d, a line through two base points P1, P2 meets the curve in d-2
  further points; a relation when all lie in B. Per trial [derived]: substitute the line (O(d^2)
  M), divide out the two known roots, test the degree-(d-2) residual. For smooth plane quartics
  (g = 3, d = 4) the residual is a quadratic: disc + chi + (on success) a square root. That is
  **~20-30 M + chi per trial, no group operation, no x^q** -- versus ~77 + 140-280 M per
  trial for the hyperelliptic g=3 walk. Relation weight d = 4.
- Diem / Diem-Thome: O~(q) for non-hyperelliptic genus 3 (vs q^(4/3) hyperelliptic, GTTD)
  [snippet]; Diem-Thome implemented the DLP variant for a C_{3,4} curve of genus 3 over
  **F_{2^31}** [snippet via secondary source]. Laine-Lauter (J. Math. Cryptol. 2015, eprint
  2014/346) give a C++ implementation with time-memory trade-offs [snippet]; timings not obtained.
- Vitse-Wallet sieving on top of Diem/Kochinke: ~3x on relation search [snippet].
- Regime: applies only to curves with a low-degree plane model (non-hyperelliptic, g = 3 quartics;
  larger g via Diem's degree-d plane models, exponent 2 - 2/(d-2)). This is an algorithmic change
  (exponent), not just engineering; see note D. Engineering-wise its per-trial cost is the
  cheapest of all methods here.

## 4. Large-prime variation engineering

- Single LP: union-find keyed by large prime; a second relation with the same LP gives a combined
  full relation. Double LP (GTTD): graph whose vertices are large primes plus a special vertex "1";
  each partial relation is an edge; each new cycle yields a relation. Cycle detection by union-find
  with path compression, O(alpha(n)) per edge; cycle *extraction* (BFS/DFS on a spanning forest)
  only when an edge closes a cycle [standard; GTTD 2007].
- Cost in FPM: graph operations are integer/memory operations with **no field multiplications**;
  under the lab metric they are priced 0 unless the lab prices them. Combining relations along a
  cycle costs row additions (sparse; free under FPM). The real costs are:
  1. Memory: all partial relations must be stored. Count ~ (relations needed) x (partials per full)
     which grows as the base shrinks; GTTD-regime memory ~ q^(1-1/g) * polylog [recall]. Store as
     packed (lp1, lp2, row_id) and rows separately (g base indices + walk coefficients); ~40-60
     bytes per partial at g = 4 [estimate].
  2. Row weight of cycle-derived relations: a cycle of length c contributes a combined row of
     weight up to c*g; long cycles fatten LA. Prefer shortest cycles (BFS), cap cycle length, and
     let SGE/merge handle the rest. GTTD's analysis assumes bounded average cycle length.
  3. The large-prime bound/threshold (which places count as "large") is a tuning knob that shifts
     cost between relation search and LA; the lab sieve smoke shows lp<=2 cutting relation cost
     2x but nearly doubling LA at g=3.
- Expected factor: DLP itself is an algorithmic exponent change (already in the lab baseline).
  Engineering beyond it (storage format, cycle-length cap) is ~1.0-1.3x on total, mostly via LA
  row weight; memory, not FPM, is the binding constraint at q > 2^30.

## 5. Linear algebra over Z/rZ

### 5.0 How much a lever on one phase moves the total (elasticity model) [derived, heuristic]
Model the relation phase as R(B) = a * B^(1-gamma) (a = per-trial cost x q-dependent factors,
gamma = effective smoothness exponent) and LA as L(B) = b * w * B^2. Minimising R + L over B:

    L/R = (gamma-1)/2 at the optimum,  total T ~ a^(2/(gamma+1)) * (b w)^((gamma-1)/(gamma+1))

gamma = g reproduces Theriault's q^(2-2/(g+1)); gamma = g-1 reproduces GTTD's double-large-prime
q^(2-2/g) (the lab's regime). With the base re-tuned after the change, a lever that cuts the
relation phase by k cuts the total by k^e_R; one that cuts LA by k cuts it by k^e_L:

| g | e_R (DLP, gamma=g-1) | e_L (DLP) | LA share at optimum | e_R (no LP) | e_L (no LP) |
|---|---|---|---|---|---|
| 2 | 1.00 | 0.00 | ~0 | 0.67 | 0.33 |
| 3 | 0.67 | 0.33 | 33 % | 0.50 | 0.50 |
| 4 | 0.50 | 0.50 | 50 % | 0.40 | 0.60 |
| 6 | 0.33 | 0.67 | 67 % | 0.29 | 0.71 |
| 8 | 0.25 | 0.75 | 75 % | 0.22 | 0.78 |
| 12 | 0.17 | 0.83 | 83 % | 0.15 | 0.85 |
| 16 | 0.125 | 0.875 | 88 % | 0.12 | 0.88 |

**This is the single most important quantitative point for prioritisation**: relation-phase levers
(group law, test, sieving) lose leverage as g grows; at g >= 6 LA levers dominate. At fixed (not
re-tuned) base the factor is instead 1/((1-s) + s/k) with s the phase share; re-tuning always
does at least as well. Lab check: at g = 4, p = 2053 the measured LA share is ~35 % of native ops
(`hecdlp_sieve_20261116`, run_ic_ext row: 6.89M of 19.8M), below the 50 % optimum, so the lab's
base may be smaller than optimal for its own costs, or LA is priced differently. [lab smoke]

### 5.1 Matrix shape
- Rows: g base entries (walk) or 2g+1 (sieve/lines: d), plus cycle-combined rows from DLP; entries
  mostly +-1 (multiplicity 1), so a sparse mat-vec is mostly **additions mod r** -- zero FPM in a
  multiplication-only metric. Multiplications mod r come from non-unit entries, from projections
  and from the Berlekamp-Massey / Lanczos scalar work, i.e. the O(N^2) dense part, not the O(wN^2)
  sparse part. This is why the lab's "coordinate Wiedemann" (coordinate projections, no dense dot
  products) was a large LA win. **Pricing caution**: r ~ #J ~ q^g, so one mult mod r is
  ~(g log2 q / 64)^2 word multiplications; ensure the priced LA term uses a consistent unit
  across g, otherwise LA levers at large g are mis-weighted.

### 5.2 Structured Gaussian elimination / filtering
- Before Wiedemann/Lanczos: remove singletons (columns with one entry, and their rows), cliques
  (Cavallar 2000, "Strategies in filtering in the number field sieve": duplicate, singleton, clique
  removal, then 2- and 3-way merges; goal "shrink the matrix while keeping the weight under
  control" [snippet]). LaMacchia-Odlyzko 1990 for structured Gaussian elimination [recall].
- Cost after: LA ~ w' N'^2 with N' < N, w' > w. In NFS practice filtering shrinks the matrix several
  fold [recall: typical NFS factors 3-10 in N; not verified here]. For curve IC with weight-g rows
  and few heavy columns the gain is smaller: the lab measured cols 574 -> 312 (1.84x) and LA mults
  9.3e5 -> 4.0e5 (**2.3x**) at g = 4, p = 2053, and 980 -> 575, 2.75e6 -> 1.24e6 (2.2x) at
  p = 4099 [lab smoke, `hecdlp_sge_coord_20261116`]. Merge cost negligible (<= 42 mul).
- Note SGE merges turn +-1 entries into general entries (mults mod r per entry in SpMV); in the
  multiplication-only count this partly offsets the dimension cut. Stop merging when the
  predicted w'N'^2 (with mults counted) stops falling.
- Expected effect on total: LA /2-3 -> total /2^e_L .. 3^e_L: g=4: 1.4-1.7x; g=8: 1.7-2.3x;
  g=16: 1.8-2.6x (re-tuned). At fixed base and the lab's measured 5-10 % LA mult share at
  g = 4, small p, only ~5 % (as the lab note says); the gain grows with p and g.

### 5.3 Wiedemann vs block Wiedemann vs Lanczos
- Scalar Wiedemann: ~2N mat-vecs (sequence of length 2N) + Berlekamp-Massey O(N^2) mults mod r;
  plus one more pass of N mat-vecs to build the solution. Total ~3 w N^2 adds + O(N^2) mults.
- Block Wiedemann (Coppersmith 1994): N/m + N/n + O(1) products per sequence, n independent
  sequences [snippet, Stach CADO-NFS slides]; total mat-vecs ~ N(1 + n/m), same as scalar for
  m = n, but split into n jobs with no synchronisation. Generator via matrix Berlekamp-Massey
  (Thome 2002, subquadratic, FFT-based) [snippet]. **No FPM gain**; wall-clock and memory only.
- Lanczos: ~N iterations, each a product with A and A^T (or A^T A), plus a few dense inner
  products per iteration (~2-4 N^2 mults mod r in total, unavoidable). Fewer mat-vecs than
  Wiedemann (~2N vs ~3N) but the dense mults cannot be removed by coordinate projections.
  Under a multiplication-only metric with +-1 entries, **coordinate Wiedemann beats Lanczos**.
  [derived; worth a lab count, unverified]
- Expected factor on LA: Wiedemann -> coordinate Wiedemann already taken by the lab; Lanczos
  would be ~0.5-1.5x (uncertain sign under FPM). Block Wiedemann: 1.0x FPM.

### 5.4 Other LA levers
- Solve only for the needed logs: individual-log step needs only log(T) relative to G; with the
  "coordinate" approach the lab already computes a single coordinate.
- Rational reconstruction / CRT over factors of r: r is prime for the target subgroup; none.
- Reduce N by automorphism quotient (Section 6).

## 6. Automorphisms and Frobenius

- Hyperelliptic involution: P and -P give the same x; the factor base is stored as x-coordinates
  with a sign, halving |B| relative to points. Standard in Gaudry 2000 and already implicit in
  testing u only. **Not an extra lever** (counted in baseline); claiming it again is relabelling.
- Extra automorphisms (CM families, e.g. y^2 = x^(2g+1) + a with zeta_{2g+1} action, or
  y^2 = x^(2g+1) + a x): with automorphism group of order m (m > 2), orbits of size m/2 on B
  (after the involution) -> |B| / (m/2). Relations needed /(m/2); LA dimension /(m/2), LA cost
  /(m/2)^2 at fixed weight, but entries become eigenvalues lambda^j mod r (one mult mod r each in
  SpMV, losing the +-1 advantage) [derived]. Total IC cost /(m/2) .. /(m/2)^(1+e_L).
- **Rho also benefits**: equivalence-class rho gains sqrt(m) (Duursma-Gaudry-Morain 1999
  [recall]). So the IC-vs-rho gap shrinks by ~sqrt(m/2) .. (m/2)^(0.5+e_L), but only for those
  special curves. For generic curves (the lab's random challenge curves) this lever is **0**.
- Frobenius over extension fields: for C defined over F_q and the Jacobian over F_{q^n}, use the
  Frobenius orbit of base points (size n) to shrink B by n; also decomposition (Gaudry 2009, Nagao,
  Joux-Vitse, Sarkar-Singh 2015/179) -- algorithmic, exponent-changing; out of scope for C, see A/D.

## 7. Parallelism

- Relation search: embarrassingly parallel (independent walks/trials, seed-split). Wall-clock
  scales linearly; **FPM unchanged** (factor 1.0 on the metric). Side benefit: the W independent
  trials make Montgomery batch inversion free (Section 1.5).
- LA: block Wiedemann distributes n sequences with no communication until the BM step; Lanczos
  needs a global sync each iteration. FPM unchanged (block Wiedemann adds O(N/m) extra products,
  ~1.0-1.1x). Large-prime graph: needs a global merge of partials (shard by large prime, merge
  union-find); memory per node is the constraint (Laine-Lauter discuss per-computer memory
  [snippet]).
- Lever value in the lab metric: **1.0x**. Report parallel runs with total (not wall) FPM to avoid
  an accounting illusion.

## 8. Published end-to-end implementations and profiles

| work | curve / g | what was run | profile (relation vs LA) | status |
|---|---|---|---|---|
| Gaudry, Eurocrypt 2000 (inria-00512401) | hyperelliptic, Koblitz genus-6 curve broken | full IC, full base | predicts IC faster than rho for g > 4; split not obtained | [snippet] |
| Theriault, Asiacrypt 2003 (single LP; Harley's variant) | small g hyperelliptic | restricted base to balance search vs LA | notes LA is where much of the cost sits for g >= 3 | [snippet] |
| Gaudry-Thome-Theriault-Diem, eprint 2004/153; Math. Comp. 76 (2007) | hyperelliptic g = 3, C/C++ | DLP variant, q^(4/3) | **relation search ~9 days, LA ~5.8 days** on a 1.7 GHz Pentium-M (**LA ~39 % of total**); ~4e10 Jacobian operations + smoothness tests; block Wiedemann, LA parallelised over machines; "surpasses rho even for rather small field sizes" for g = 3 | [snippet; table cells not obtained] |
| Diem-Thome, J. Cryptology 2008 | non-hyperelliptic g = 3, C_{3,4} curve over F_{2^31} | Diem line method + DLP, O~(q) | not obtained | [snippet] |
| Laine-Lauter, J. Math. Cryptol. 2015 (eprint 2014/346) | non-hyperelliptic g = 3, C++ | time-memory trade-off variant of Diem | **~3x faster relation search and ~9x faster LA than Diem, asymptotically**; small-q experiments match theory | [snippet] |
| Vitse-Wallet, LATINCRYPT 2015 | non-hyperelliptic (and hyperelliptic) | sieving | ~3x on relation search of Diem / Kochinke | [snippet] |
| Sarkar-Singh 2014/815, 2015/179 | hyperelliptic; F_{q^2}, g = 5, 6 | sieve / decomposition | relations obtained for g = 5, 6; timings not obtained | [snippet] |
| Velichka-Jacobson-Stein, Math. Comp. 83 (2014) | high-genus hyperelliptic over F_{2^n} | improved (Enge-Gaudry type) IC | timings not obtained | [snippet] |
| Lab (`hecdlp_*_20261116`, genus4_*) | hyperelliptic g = 3, 4, p ~ 2^10-2^12 | DLP + parity prefilter + coordinate Wiedemann | g = 4, p = 2053: relation 12.9M vs LA 6.9M native ops (LA ~35 %) | [lab smoke] |

Consistency check of the Section 5.0 model: GTTD's g = 3 split (LA 39 %) is close to the predicted
33 % at the optimum for gamma = g-1; the lab's g = 4 split (35 %) is below the predicted 50 %.
Magma/PARI: no built-in Jacobian IC solver known to me [recall, unverified]; Magma has fast
Jacobian arithmetic (Cantor/NUCOMP-style) usable as a reference for group-law timing only.

## 9. Levers ranked by expected total-cost factor

Factor = expected reduction of total IC cost (FPM + priced LA) vs the **current lab baseline**
(DLP, parity prefilter, x^p test, Cantor-or-FastJ law, coordinate Wiedemann), with the base re-tuned
(Section 5.0 elasticities, DLP column). Ranges reflect unverified constants. "Rho-neutral" marks
levers that the matched rho reference also gets, so the gap to rho moves less than the factor.

| # | lever | applies to g | expected factor on total IC cost | evidence strength | effort |
|---|---|---|---|---|---|
| 1 | Sieving relation search (Sarkar-Singh / lab `y - v_t`), with family column + base re-tune | 3-16 (largest 4-8) | relation phase /3-6 -> total 1.3-2.5x (g=3: (3..6)^0.67 = 2.1-3.3x before the LA row-weight penalty; g=4: 1.7-2.4x; g=8: 1.3-1.6x); **can be < 1 if LA weight not mitigated** (lab smoke: 0.8-2x untuned) | medium: lab smoke + Vitse-Wallet ~3x [snippet] | medium (exists in lab; needs family column + tuning) |
| 2 | SGE / filtering + merge before coordinate Wiedemann | all; grows with g | LA /2-3 -> total 1.4-1.7x (g=4), 1.7-2.3x (g=8), 1.8-2.6x (g=16); ~1.05x at lab's small-p g=4 fixed base | medium: lab smoke 2.2-2.3x on LA mults; NFS practice | low (exists; wire into default) |
| 3 | Closed-form splitting (disc+chi g=2; Cardano g=3; Ferrari g=4) | 2-4 | test /2-5 -> relation phase /1.7-3 -> total 1.4-2.2x (g=2,3), 1.3-1.7x (g=4) | medium-low: derived; lab has `ferrari` flag unmeasured; **accounting risk** (price chi / roots honestly) | low |
| 4 | Explicit-formula group law (Lange g=2, I+67M g=3, ~2I+160M g=4; generated formulas g<=10) | 2-10 | add /2.3-3.5 -> relation /1.2-2 -> total 1.1-1.4x; **rho-neutral** | high for counts (literature + lab FastJ 2.3x at g=4) | low (g=3,4 exists), medium for g=5-10 |
| 5 | Diem line method (plane quartics) instead of hyperelliptic-style walk | 3 (non-hyp.), larger g via plane models | per-trial ~10x cheaper and exponent q vs q^(4/3); total >= 3x at q ~ 2^20 [derived] | medium (theory + Diem-Thome, Laine-Lauter implementations) | medium-high; only for non-hyperelliptic targets (note D) |
| 6 | NUCOMP / NUDUPL (or balanced NUCOMP for even-degree models) | 8-16 (11-16 if generated formulas exist) | add /1.2-2 -> total 1.05-1.15x (e_R small at large g); **rho-neutral** | medium (crossover g ~ 7-8 [snippet]) | medium |
| 7 | Montgomery batch inversion across independent trials | all, most at g=2,3 | 1.05-1.25x on relation phase -> total 1.03-1.2x; rho-neutral | high (lab measured 7 % at g=4) | low (exists for FastJ) |
| 8 | Large-prime engineering (shortest cycles, cycle-length cap, compact storage) | 3-16 | 1.0-1.3x via LA row weight; enables larger q (memory) | low-medium | low |
| 9 | Remainder-tree batch test (reduced base) | 9-16 with NTT-friendly q | test /1.5-3, total 1.02-1.1x | low (derived) | medium |
| 10 | Lanczos instead of coordinate Wiedemann | all | 0.7-1.3x (sign uncertain under mult-only metric) | low | medium |
| 11 | Extra automorphisms (CM curves) | special curves only | IC /(m/2)..(m/2)^1.5; gap to rho shrinks only ~sqrt(m/2)+ | high theory; **0 for generic curves** | medium |
| 12 | Parallelism (relation search, block Wiedemann) | all | **1.0x** in FPM (wall-clock only) | high | low-medium |
| 13 | Projective / inversion-free coordinates | all | **<1x** at I = 10M (affine wins) | high (counts) | -- (do not do) |

Combined, if levers 1-4 compose (they partly overlap: sieving removes the group law and the test,
making 3 and 4 moot on the relation phase except for the individual-log walk), a realistic total
gain over the baseline is **~2-4x at g = 3-4** and **~2-3x at g = 8-16** (driven by LA levers),
with the gap to rho reduced by somewhat less because levers 4, 6, 7 are rho-neutral. None of this
changes exponents; the exponent-changing items (DLP, Diem lines, decomposition over F_{q^n}) are
algorithmic and belong to notes A/D.

## 10. Open checks (to turn [snippet]/[recall] into verified)
1. Fetch Lange 2005 (AAECC) affine/projective counts; Pelzl-Wollinger-Paar 2003/097 g=4 table;
   Asgari-He-Jacobson-Scheidler CASC 2026 counts for g = 5..10 (needs cspages.ucalgary.ca).
2. Fetch GTTD 2004/153 experimental table (q vs relation/LA/rho) and Laine-Lauter 2014/346.
3. Fetch Vitse-Wallet (hal-01203086) and Sarkar-Singh 2014/815 for exact sieve families and
   whether they handle relation weight 2g+1 (family column) -- directly relevant to lever 1.
4. Lab: measure `ferrari: true` arm with chi priced as an exponentiation (lever 3), and the
   sieve with a family column + re-tuned base (lever 1), each as a bench.record.
5. Re-derive the elasticity table with the lab's actual R(B), L(B) fits per g.

## 11. Sources (search-result level unless stated)
- Gaudry, Eurocrypt 2000, "An algorithm for solving the DLP on hyperelliptic curves": https://hal.archives-ouvertes.fr/inria-00512401 ; https://rd.springer.com/content/pdf/10.1007/3-540-45539-6_2.pdf
- Gaudry, Thome, Theriault, Diem, "A double large prime variation for small genus hyperelliptic index calculus": https://eprint.iacr.org/2004/153 ; https://www.ams.org/journals/mcom/2007-76-257/S0025-5718-06-01900-4/S0025-5718-06-01900-4.pdf
- Theriault, Asiacrypt 2003: https://www.iacr.org/archive/asiacrypt2003/02_Session02/19_056/28940307.pdf
- Diem, Thome, "Index calculus in class groups of non-hyperelliptic curves of genus three": https://hal.archives-ouvertes.fr/inria-00107290 ; Diem SHARCS'06 talk http://www.hyperelliptic.org/tanja/SHARCS/talks06/Claus_Diem.pdf
- Diem, "Index calculus with double large prime variation for curves of small genus with cyclic class group": https://arxiv.org/pdf/math/0606607
- Laine, Lauter, "Time-memory trade-offs for index calculus in genus 3": https://eprint.iacr.org/2014/346.pdf ; https://doaj.org/article/22c38042f6a94fc7aa9e0c8fe9f95345
- Vitse, Wallet, "Improved sieving on algebraic curves", LATINCRYPT 2015: https://hal.sorbonne-universite.fr/hal-01203086
- Sarkar, Singh: https://eprint.iacr.org/2014/815 ; https://eprint.iacr.org/2015/179
- Velichka, Jacobson, Stein, Math. Comp. 83 (2014): https://www.ams.org/mcom/2014-83-286/S0025-5718-2013-02748-2/
- Lange, inversion-free genus 2: https://eprint.iacr.org/2002/147 ; AAECC 2005 record https://www.sciweavers.org/node/272696 ; Robert slides https://lfant.math.u-bordeaux.fr/seminar/slides/2015-05-12T10:00--Damien_Robert.pdf
- Pelzl, Wollinger, Paar, genus 4: https://eprint.iacr.org/2003/097 ; parallel explicit formulas https://rd.springer.com/content/pdf/10.1007/978-3-540-40061-5_6.pdf
- Sutherland, "Fast Jacobian arithmetic for hyperelliptic curves of genus 3": https://arxiv.org/abs/1607.08602
- Fan-Wollinger-Wang 2005/228 (genus 3 doubling): https://eprint.iacr.org/2005/228
- Jacobson/Scheidler NUCOMP line: https://cspages.ucalgary.ca/~rscheidl/Papers/pdfs.pdf ; Balanced NUCOMP https://hal-lirmm.ccsd.cnrs.fr/lirmm-02989881 ; Asgari-He-Jacobson-Scheidler CASC 2026 https://cspages.ucalgary.ca/~rscheidl/Papers/CASC2026.pdf ; Jacobson-van der Poorten 2002 https://link.springer.com/doi/10.1007/3-540-45455-1_10
- Cavallar, filtering in NFS: https://ir.cwi.nl/pub/4456/04456D.pdf ; https://ir.cwi.nl/pub/2078/2078D.pdf
- Block Wiedemann: Stach CADO slides https://members.loria.fr/EThome/cado2008/slides/stach.pdf ; https://arxiv.org/pdf/1402.3661.pdf
- Lab notes (local, read-only): research/hecdlp_fastgroup_20261116/NOTES.md, research/hecdlp_sieve_20261116/NOTES.md, research/hecdlp_sge_coord_20261116/NOTES.md, research/genus4_rho_refs_20261116/validation/validator-report.md
- research-kb was not queried in this session (no MCP tool available to this subagent); record as a gap.

# Idea 11 pilot protocol: solver-driven synthesis of the factor base

Written 2026-11-14 before any measured run. Pilot only (toy sizes, one core,
shared 4-core container). Every number produced is an unverified pilot assertion.

## Claim under test

C1 (structure exists): among l-dimensional F2-subspaces V of F_{2^n}, some have a
low-degree defect outside the random-subspace distribution.
C2 (structure pays): a V with the best defect found lowers the point-decomposition
solve cost, against random V on the same targets-per-base protocol, by more than
the base-to-base noise floor (A/A arm: random V set A vs random V set B).

## Defect definitions (fixed before running)

For a basis v_1..v_l of V, x_i = sum_j c_ij v_j, bit variables c_ij.
- `w(V) = dim_F2 span{v_i v_j : i<=j}` (product-space dimension; target independent).
  The S3 equation `p^2 + u p + u^2 s^2 + b = 0` is F2-affine in (s,p), s in V, p in V.V,
  so `l(l+1)/2 - w` is the count of target-independent F2-linear relations among the
  degree-2 monomial images. PRIMARY defect statistic: `w` (lower = larger defect).
- Literal Macaulay defect of the descended m=2 system S3(x1,x2,x(R)) (n quadrics in 2l
  bit variables, field equations x^2=x applied), fixed random target per cell:
  `def2 = #monomials(deg<=2) - rank(rows at D=2)`, `def3 = #monomials(deg<=3) - rank(rows
  multiplied by 1 and by each variable)`. Reported, secondary.

## Arms for C1 (cells (l,n) = (3,11),(4,13),(5,17),(6,19),(7,23); curve y^2+xy=x^3+1)

a. random V: 200 samples per cell (null distribution).
b. structured: polynomial span c*<1,t,..,t^(l-1)> (8 random (c,t)); Frobenius span
   <t,t^2,t^4,..,t^(2^(l-1))> (8 random t); random subspace of the trace-zero hyperplane (8).
   Positive control for the detector: n=15 (composite), V = subfield F_32 (l=5) and F_8 (l=3),
   where w must equal l.
c. search: (c1) blind hill-climb on w (replace one basis vector by a random element,
   accept if w does not increase, 2000 moves, 5 restarts); (c2) product-guided hill-climb
   (replacement drawn from {v_i v_j, v_i v_j / v_k}); (c3) exhaustive enumeration of all
   V at (l,n)=(3,7) and all V containing 1 at (3,11). z3/CEGIS over basis-change matrices
   only if time remains (expected: not covered).
Prior knowledge stated up front: the repository's linear 2+1 arm already uses the
polynomial span with p in span(1..z^(2l-2)), i.e. w = 2l-1; the linear Cauchy-Davenport /
Kneser bound (Bachoc-Serra-Zemor) says w >= 2l-1 for prime n when 2l-1 <= n. So the
expected outcome of the search is to rediscover, not exceed, the default base.

Kill 1: best-found w not below the 5th percentile of the random distribution of w
(in every cell) -> no structure to synthesise.

## Arms for C2 (same instances, interleaved base order S,A,B,S,A,B,...)

Bases per cell: S = 4 best-defect bases (polynomial spans, distinct seeded (c,t)),
A = 4 random V, B = 4 random V (A/A control).
Targets per base: 4 random curve points shared by all 12 bases of the cell, plus
4 planted targets per base (R = +-P1 +-P2 +-P3, P_i random base points, base specific).
Decomposable / non-decomposable reported separately (ground truth from the linear scan,
cross-checked by brute force on the smallest cell).

Solve arms (each run on every (base, target)):
- L21: linear 2+1 scan (baseline family of research/groebner_free_matched_20261113,
  "Full" variant): for every base point P3 (both signs) u = x(R-P3); solve the n F2-linear
  equations in the l+w bits of (s,p), p in a basis of V.V; enumerate the affine solution
  set (cap 2^14 per leaf, truncation recorded) and split z^2+sz+p. Units: `leaves`
  (= 2|F|), `cands` (sum over leaves of solution-set size = (s,p) candidates handed to the
  split; "degree-2 linearisation decision rate" = fraction of leaves with <=1 candidate).
  Sizes l=3..7.
- SAT3: CNF of the chained m=3 system S3(x1,x2,t)=0, S3(t,x3,x(R))=0, t free in F_{2^n},
  p=x1x2 written in a reduced basis of V.V (w coordinates, the same rule for every V),
  XORs cut to length <=5 by Tseitin chains, no symmetry breaking. CaDiCaL 1.5.3 (pysat)
  if it exposes counters, else Glucose4; conflicts primary, decisions/propagations secondary.
  Sizes l=3,4 (l=5 only if a probe instance finishes in < 60 s; cap 300 s/instance).
- SAT2 (secondary, m=2): CNF of S3(x1,x2,x(R))=0, cells (l,n)=(4,11),(5,11),(6,13),(7,17),(8,17),
  same base/target layout with planted R = +-P1 +-P2.
Plain enumeration reference counts reported next to each cell: 2^{(m-1)l} tuples-ish
(|F|^2 pair sums) and the 2+1 leaf count 2|F|.

Metric and analysis: per-arm geometric mean of the unit count per cell, split by
decomposable/non-decomposable; ratio S/A with the A/B ratio as noise floor; base-level
(not instance-level) spread = min/max of per-base geometric means in A∪B. Slope = least
squares of log2(geomean) on l.

Prediction: w(S)=2l-1, random w = min(n, l(l+1)/2). L21 `cands` per leaf: S about 1
(l+w-n <= 0 when n >= 3l-1), random about 2^{max(0, l+w-n)}, so the random-V L21 slope
drifts from 1 to about 2 bits per l while S stays about 1. For SAT3/SAT2 no confident prediction:
expected ratio S/A within noise or a constant (formula is smaller: n*w vs n*n ANDs).

Kill 2: S/A ratio of the primary unit inside the A/B base-to-base range in every
solve arm -> no solve-cost difference beyond noise. If only L21 separates, the result is
"known mechanism (already the repository default), nothing new to synthesise".

Seeds: master 2026111411; per cell seed = master*1000 + 37*l + n; all recorded in results.json.
Validation before measurement: irreducibility of moduli; group law spot checks; planted
targets must be SAT in every arm and every model/decomposition must lift to curve points
summing to +-R; brute-force triple enumeration vs L21 and SAT3 at (l,n)=(3,11).

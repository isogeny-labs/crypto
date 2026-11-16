# L3-digit protocol (frozen before any counted run), 2026-11-13

Lead: LEAD-ICP-digit-presentation-solving-degree ("lever 2"). All numbers `[pcc:unverified]`.

## Question
For a factor base given by a *digit presentation* (x = sum_i w_i d_i, each digit d_i in a small
set X_i cut out by prod_{c in X_i}(d - c) = 0), how does the **solving degree** D of the Semaev
decomposition system S_{m+1}(l_1..l_m, x_R) = 0 grow with the number of digits k per block, and what
per-relation cost does that imply against the §2.4 requirement (m = 4: T_dec << B^1, i.e. p^{1/4}
at B ~ p^{1/5}; general arity m: T_dec = B^theta needs theta < m - 3)?

## Object and instrument
- Ring A = tensor_j F_p[v_j]/(prod_{c in X_j}(v_j - c)) (function algebra on the grid; field/digit
  equations free). f = S_{m+1}(l_1,..,l_m, r) reduced in A, obtained exactly by evaluating S_{m+1}
  numerically on the grid (S_4, S_5 as resultants) and interpolating axis by axis.
- **Solving degree D** = least c such that the degree-c polynomial-calculus closure V_c (rows t*f
  with deg <= c; every element of V_c of degree < c multiplied by every variable, to a fixed point;
  exact Gaussian elimination mod p) contains 1 (unsatisfiable target), or contains every
  degree-<=1 element of I(Z(f)) (satisfiable/planted target). **Certificate**: V_{D-1} computed to its
  fixed point does not contain the target; V_D does. V_c is the largest set reachable by any
  degree-c-bounded PC/F4/XL-with-mutants derivation, so D lower-bounds the max degree of all of them.
- Cost observables at D: cols(D) = dim A_{<=D}, rank(V_D), wall seconds.
- Engine: own std-only Rust `pcdeg.rs` (closure algorithm ported from the semi-echelon closure of
  crypto:research/prime_digit_presentation_20261029/digit_fall.rs, generalised to arbitrary digit
  sets and u64 accumulators).

## Presentations (families)
- `bool`: X = {0,1}, w_i = 2^i (Boolean digits; interval base [0,2^k)).
- `b3`: X = {0,1,2}, w_i = 3^i.  `b4`: X = {0,1,2,3}, w_i = 4^i.
- `mix`: radices alternating 2,3,2,3,... (mixed radix; interval base).
- Controls: `uni-int`: one variable per block, X = {0..B-1} (same interval base, univariate
  presentation); `uni-rand`: one variable per block, X = random B-subset of curve abscissas
  (Theorem-B case); `supp`: random nonzero coefficients on the monomial support of the real f in the
  `bool` presentation (curve-free null).
- m = 2 (k as far as cols <= ~25k), m = 3 (small k), m = 4 (bool, k = 2,3: saturation check only).
- Curves: prime-order y^2 = x^3 + a x + b over p in 12..24 bits, a new prime/curve/target per draw.
  Targets: random unsatisfiable (primary) and planted satisfiable (secondary).

## Instrument validation (must pass before counted rows are read)
- V1: reproduce the cited `bool` m = 2 values e0 = d_last = 4 + floor(s/2) (s = 2..6) of
  digit_fall (repo study) on fresh draws.
- V2: grid zero count of f equals brute-force count of decompositions from curve arithmetic.
- V3: for planted targets, the planted digits are a zero of f.
- Positive control (detector sees presentation dependence): uni-int vs bool on the same base must
  differ (expected D ~ B vs ~ log B). A/A: two independent draws per cell must agree or the spread
  is reported.

## Predictions (written before data)
- P1 bool m=2: D = 4 + floor(k/2). P2 b3/b4: D grows linearly in k (slope unknown, guess ~ (b-1)/2
  per digit). P3 uni-rand / uni-int: D ~ B + 2 at m = 2. P4 supp null: D >= real D.
- P5 (theory): restriction of all other blocks to constants maps a PC refutation to one of
  (l_1 - rho) for a root rho; so D(f) >= D_lin(k) := PC degree of the single digit-linear equation.

## Translation to cost (derived, stated in report)
T_dec = cols(D)^omega with omega = 2 (optimistic sparse) and omega = 1 (free elimination floor);
theta = log T_dec / log B. Total IC exponent with LA: E_m = 2/(m+1-theta) when theta < m-1, else
>= 1. Distance to rho: E_m - 0.5. Projection to m = 4 uses the measured per-digit slope sigma,
assumed m-independent (supported by P5), and cols ~ exp(n * h_b(D/n)).

## Decision rule (preset)
- **ADVANCES** if any digit family shows D not increasing over the last 3 measured k-steps at m=2
  (certified), or if its projected m=4 exponent E_4 (omega = 2) is < 0.5.
- **KILL at toy scale** (per family) if D increases with slope >= 1/4 per digit at m = 2 over the
  measured range (certified rows), AND the projected E_4 (omega = 2, and also omega = 1) is >= 0.5.
- **INCONCLUSIVE** otherwise (including when fewer than 3 certified k values exist for a family).
- Censored runs (timeout/OOM/cmax reached) are reported as D > c bounds and never as values.

## Resource limits
One core, prlimit --as=3 GB, timeout 1200 s per command, total about 3 h wall. No cloud.

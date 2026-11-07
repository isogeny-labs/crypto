# DESIGN: exact triangular / FGLM extractor for split-graph secant systems

2026-11-07. Evidence classes: **proved** (argued below, finite algebra),
**derived** (follows from a proved statement plus bookkeeping), **measured**
(a counter printed by the code), **heuristic**, **cited**. Every number in this
file that was produced by a run is `[pcc:unverified]` and comes from a
*development check*, not from the frozen panel.

## 1. What is being replaced

The historical panel ([RESULTS.md](../prime_factor_base_degree_20261031/parallel_20261031/RESULTS.md))
solved each split-graph system with stock msolve-neogb (a reduced DRL basis) and then
extracted rational roots with a coordinate-cube search: pick a variable, try all `p`
values, specialise, recurse, stop at 2,000,000 assignments. That is an exhaustive
search whose size is a function of the basis shape, not of the number of roots; it
capped on 26 F127/d=5 systems, and the capped systems were never given an operational
root set or a cost.

The replacement does no assignment search. Rational points are obtained from
univariate polynomials over `F_p` only.

## 2. The extractor (all steps counted)

Input: generators `G` of a zero-dimensional ideal `I` in `F_p[x_0..x_{n-1}]`
(n = 3 for secant systems, 2 for the dense S3 system).

1. **DRL Groebner basis** (`sgsecant_alg.rs: groebner`). Buchberger, normal
   selection strategy (smallest lcm degree first), product and chain criteria,
   monic normalisation, then minimalisation and tail reduction to the unique
   reduced basis. `handled_degree` = the largest lcm degree of any processed pair
   or input generator, the same statistic the historical report calls handled
   degree. This is **not** matrix F4; see section 7.
2. **FGLM to lex** (`fglm`). Standard monomials of the DRL basis give the
   quotient `A = F_p[x]/I` (dimension `D`). Multiplication matrices
   `M_{x_i}` are built by normal forms of `s * x_i`. The lex basis (x_0 > x_1 > x_2)
   is then produced by the standard FGLM sweep: enumerate monomials in increasing lex
   order, compute their normal-form vectors by matrix-vector products, keep the
   linearly independent ones as the staircase, and every first dependency gives a
   lex basis element (the combination is tracked, so no extra solve is needed).
3. **Triangular rational-root extraction** (`triangular_roots`). Order of solving:
   the last variable first. At level `l` specialise every lex element that involves
   only `x_l..x_{n-1}` at the values chosen so far; the result is a univariate
   polynomial in `x_l`. Take the gcd of the non-zero ones, then
   `gcd(g, X^p - X)` (via `X^p mod g` by repeated squaring), then equal-degree
   splitting with `(X + a)^((p-1)/2) - 1` to list the distinct roots in `F_p`.
   Recurse on each root. A nonzero constant prunes the branch. If no univariate
   element survives at some level (the extension-theorem corner case) the level
   falls back to scanning all of `F_p`; that fallback is **counted and flagged**
   (`fallbacks`, `unresolved`), 64 fallbacks abort as unresolved, and an unresolved
   system is unmeasured.
4. **Soundness filter** (`extract`). Every candidate point is evaluated on every
   input generator; only exact common roots survive. Completeness does not rely
   on this filter: it follows from the lex basis being a Groebner basis (every true
   rational point is a root of every specialised elimination element, so the gcd
   at each level contains its coordinate).

Why this removes the assignment search: the number of operations is polynomial in
`D`, the degrees of the univariate eliminants and `log p`. The only enumeration
left is over roots of univariate polynomials, i.e. over actual partial solutions.
For a system with `r` rational roots the triangular phase visits at most
`n * r` nodes. (Derived; the node and fallback counts are measured and reported.)

Exact field-operation counters live in `sgsecant_ops.rs`. Every `F_p`
multiplication, addition/subtraction and inversion of the instrument goes through
`fmul/fadd/fsub/finv` and is added to the current phase's ledger:
`construct, target, build, groebner, fglm, roots, replay, dedup, la, rho, verify`.
A separate `native` counter records non-field work (monomial comparisons,
divisibility tests, set probes, trial divisions, hash calls). Multiplication by a
coefficient equal to 1 is not charged; a modular inversion is one `inv` (the
Euclid steps inside it are not expanded; PROTOCOL.md prices an `inv x10` sensitivity).
The row-reduction counters of the repository's `f4_fp` count multiplications only;
here `mul` alone is reported too (`mul_only`) so a comparison under that
convention is always available.

## 3. Sharing elimination across the four ordinate-sign branches

For a target `R = (r, s)` the four branch systems are
`I_{e,h} = (g1, g2, qx_e, qz_h)` with `qx_e = e*h(x) - m(x-r) + s`,
`qz_h = h*h(z) - m(z-r) + s`, `e, h in {+1,-1}` (identical to the historical
`expected_model`). Two extractor arms:

* **E1 (unshared)**: four independent solves. This is the complete, simplest
  accounting and is the reference.
* **E2 (epsilon-shared)**: two solves, `J_e = (g1, g2, qx_e)` for `e = +1, -1`; the
  `qz` equation is not part of the elimination. Each rational root `(x, z, m)` of `J_e`
  is classified by direct evaluation: compute `L = m(z-r) - s` and `h(z)`; the root
  belongs to branch `(e, +1)` if `L = h(z)`, to `(e, -1)` if `L = -h(z)`, and to
  neither otherwise. For nongraph (control) bases the membership polynomials
  `F_W(x), F_W(z)` are branch-independent and stay inside `J_e`.

**Claim (proved).** For every rational `(x, z, m)`: it is a root of `I_{e,h}` iff it is a
root of `J_e` and `L = h*h(z)` (this is the definition of the generators; `I_{e,h} = J_e + (qz_h)`
and the variety of a sum of ideals is the intersection). So the union over `h` of the
`I_{e,h}`-roots equals the `J_e`-roots with `L = +-h(z)`, and no branch's root set is dropped:
the four branch root sets are recovered exactly. In addition `V(J_e)` is finite
(derived): evaluating identity (3) of `SECANT_GRAPH.md` at `T = x` on a root of
`(g1, g2)` and using that `R` is on the curve (`c = 0`) gives `L(x)^2 = f(x)`; `qx_e` says
`L(x) = e*h(x)`, hence `h(x)^2 = f(x)`, which has only the `2d` roots `V`; `m` is then
determined by `x` when `x != r` and by tangency when `x = r` (`s != 0` because the group
has odd prime order), and `g1` gives `z`. The code checks `not_zero_dim = 0` on every system.

What this buys and costs: it halves the number of eliminations (4 to 2) but each
`J_e` has a larger quotient (it also contains the solutions whose `z` is not yet
constrained), and the `d+1` homogeneous theorem does not apply to `J_e`
(its top forms are `m^2, xz, x^d`, which is not zero-dimensional at infinity). Whether E2
is cheaper than E1 is therefore an empirical question that PROTOCOL.md pre-registers as a
secondary comparison. Development numbers (section 6) already show E2 costing more
than E1 on the tested cells, which is a reason it is *not* the primary.

Other sharing identities considered and **not** used as extractor arms:

* *Swap symmetry*: `I_{e,h}` with `x <-> z` equals `I_{h,e}`, so branch `(+,-)` and `(-,+)`
  are the same up to renaming; solving three branches suffices. Not adopted because the
  dense arm already enjoys the symmetry implicitly (its roots come in swapped pairs from
  one solve) and the protocol keeps the accounting of the candidate unmodified.
  Listed as an open lever.
* *Negation symmetry*: branch `(e,h)` at `R` is branch `(-e,-h)` at `-R` with `m -> -m`.
  A relation for `-R` is the negation of that for `R`; no extra saving arises in a random
  target stream.
* *Union ideal* `(g1, g2, F_V(x), F_V(z))` has the same roots as the union of the
  four branches (because `L(x)^2 = f(x)` makes the sign constraint redundant), but its
  generators have degree `2d`, i.e. it is the control-style presentation and loses the
  low-degree property.

## 4. Relation bookkeeping (what "complete cost per independent relation" means here)

A *run* fixes a curve `E/F_p` of prime order `n`, a base of `2d` abscissae (so `4d`
signed points and `K = 2d` columns), a generator `G`, and a challenge `H = lambda G`.
The relation stream is `R_j = a_j G + b_j H` with hash-derived `a_j, b_j`. For each
target the arm finds all signed pairs `(P, Q)` in the base with `P + Q = R_j`
(every sign branch), replays each by a group addition, deduplicates unordered pairs,
and feeds each distinct pair as a row `sigma_P u_P + sigma_Q u_Q - b_j lambda = a_j` into an
online elimination modulo `n`. The run stops when the rank reaches `K + 1 = 2d + 1`
(the unknowns are the `K` base logarithms and `lambda`). Back-substitution returns
`lambda`, which is verified against `H`. The reported quantity is the sum of all ledgers
(construction of the curve and base including rejected proposals, target generation,
system building, Groebner, FGLM, root finding, candidate verification, replay,
dedup, elimination modulo `n`, final solve and check) divided by the achieved rank
`2d + 1`. This is the "complete same-base field operations per independent relation".

The matched internal rho reference (r-adding walk, Floyd) is run on the same
`(E, G, H)` and charged with the same counters. It is an *internal* reference for
distance-remaining reporting only. It is not the ecbench matched rho with the
section 5a native-work band, which any leaderboard row would additionally need.

## 5. Arms

| id | base | presentation | extractor |
| --- | --- | --- | --- |
| cs2 | graph (candidate) | secant | E2 shared |
| cs1 | graph | secant | E1 four solves |
| cd  | same graph base | dense `F_V(x), F_V(z), S3` | one solve |
| ls2, ls1, ld | low-x nongraph control (`2d` smallest abscissae) | secant E2, E1, dense | |
| hs2, hs1, hd | hash-x nongraph control | secant E2, E1, dense | |
| hs2b | second independent hash-x control (A/A) | secant E2 | |
| pc2x | graph | cs2 with every system solved twice (positive control) | |

Controls are the historical ladder's `low-x` / `hash-x` definitions (same size,
hash-chosen ordinate signs, interpolated `h` of degree `<= 2d-1`, membership kept).

## 6. Development evidence (not claims; `[pcc:unverified]`)

All from `native/sgsecant_check.rs` and short development runs; none of it is
prospective data.

* Univariate root finding vs brute force over `p in {17, 83, 127, 491}`, 1,200 random
  polynomials: 0 mismatches (measured).
* F17 fixture (`SECANT_GRAPH.md` section 4): all 18 targets, graph and nongraph base, three
  presentations: exact root sets equal the exhaustive group reference, 56 ordered
  relations, handled degree 3 (graph secant), 4 (control secant), 6 (dense), matching
  the historical stock-F4 values (measured).
* Historical ladder reproduction, legacy hash domain, seed 315: F83 (a=59, b=30) graph
  attempts 1/4 for d=3/4, F101 attempts 3 for d=3, F127 (a=20, b=61) attempts 1/62 for
  d=3/5. All agree with `results-summary.json` (measured).
* Exact agreement with the exhaustive reference on **all finite targets** of nine legacy
  cells (F83, F101, F127 at d = 3, 4, 5) plus the F17 fixture: 87 base-by-presentation rows
  (graph, low-x, hash-x; E1, E2, dense), 0 root-set mismatches, 0 censored systems, 0
  triangular fallbacks, 0 unresolved (`dev_logs/leg_*.log`, `f17.log`). Handled degree is d+1
  for the graph secant and 2d+2 for dense, as in the historical report (measured). These
  logs include the 12 F127/d=5 candidate-secant systems that capped originally, so the new
  extractor produces the exact root set of each of them (a correctness check only).
* Historical-cap replay (`check-cap`): a faithful port of the 2,000,000-assignment
  coordinate-cube extractor applied to my reduced bases reproduces the historical counts
  exactly: 12 candidate-secant, 6 low-x secant, 8 hash-x secant systems cap (26 total) on
  F127/d=5, i.e. my reduced basis equals the original basis and the new extractor
  completes the same systems that capped. The historical panel remains censored; this
  check only validates the new extractor on those inputs.

## 7. Limits (stated, not hidden)

* The engine is a counted Buchberger, not matrix F4 and not msolve. It is used for
  **every** arm, so the comparison is internally consistent, but a conclusion is about
  this engine at this scale. `src/cryptanalysis/f4_fp.rs` was not reused: the module is
  entangled with the rest of the library (`groebner_f4.rs` imports `crate::ecc::field`
  and `symmetrized_semaev`, `fx_hash.rs` imports `pq_groebner_f2`, `f4_fp.rs` calls
  `jv_bitops` and `glv_invariant_base`), building the library needs the full dependency
  set (no local registry cache here) and a contended shared cargo lock. A cross-engine
  rerun of the primary comparison through `f4_fp::solve` (which exposes `field_ops`) is
  listed as an open thread, not a prerequisite.
* `native` ops are an engine-specific proxy for non-field work. Rankings are only
  accepted where they hold across 0, 0.1 and 1 unit per native operation (section 5a).
* Group order is prime and full (as in the historical ladder); `n` is close to `p`, so
  these are toy cells. Nothing here says anything about 256-bit curves.
* Only two summands (arity 2). The Boolean-offset successor needs its own system
  builder (7 variables at `k = 2`); the extractor is dimension-generic (up to 8 variables
  with exponents below 127) but the builder is not written (open thread).

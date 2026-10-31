# W3a protocol: the Smith walk via the elliptic 2-isogeny class

Frozen before any data is collected (the commit that adds this file is cited in
the report). Synthetic curves only; no deployed-curve or sub-rho claim.
Workstream W3a of `research/plane_quartic_program_20261018/PROGRAM.md`.
Tool: `src/cryptanalysis/smith_elliptic_walk.rs` (module, pinned tests) and
`examples/smith_elliptic_walk.rs` (driver). Native Rust; Python only plots.

## Questions

- **Q1 (exhaustive).** For `q = p` prime and `K = F_{p^3}`, over every elliptic
  curve `E/K` admitting a Joux-Vitse/Momose-Chao `(m, r) = (3, 5)` descent, over
  all of its weak models: what share of `E` have a descended `H` on which
  Smith's construction passes (`smith_prop24::analyze_curve`,
  `full_success(false)`); how far (in rational 2-isogenies, and in 2/3-isogenies)
  is a failing `E` from the nearest passing `E'`; what share of `E` have a
  passing member in their component; and is passing constant on components?
- **Q2 (sampled).** For prime `p` from about `2^10` to `2^20`: expected steps,
  and expected cost in `F_p` multiplications, of a 2-isogeny walk from a random
  failing weak `E` to the first `E'` whose descent passes Smith; is the cost
  `o(p)`?
- **Q3 (literature).** Do genuine hyperelliptic-to-hyperelliptic genus-3
  `(2,2,2)`/`(l,l,l)` isogenies exist on a positive-density set, and is any
  implementable here? Text with citations; no implementation.

## Definitions (fixed)

1. `K = F_{p^3} = F_p[z]/(m(z))` with `m` the first irreducible cubic found by
   the search order in the source. `sigma(x) = x^p`. `d` = the least quadratic
   non-residue of `F_p` (it stays a non-square in `K`: odd degree).
2. **Weak model** `(c, h, alpha)`: `c in {1, d}`, `h in F_p[x]` monic of degree 1
   or 2, or `h = 1` is excluded; `alpha in K \ F_p`; the curve
   `C: y^2 = c h(x) (x - alpha)(x - sigma alpha)` with the five branch points
   distinct (squarefree). `E` is the Jacobian of `C` over `K`, as a `K`-isomorphism
   class. `h` of degree 1 means the branch point at infinity is `F_p`-rational.
3. **Reduced enumeration.** `PGL_2(F_p)` acts freely on `P^1(K) \ P^1(F_p)`
   (`p^3 - p` points, three conjugates fix a Moebius map) and
   `|PGL_2(F_p)| = p^3 - p`, so it is transitive: every weak model is
   equivalent to one with `alpha = z`. The reduced set is
   `{(c, B)}`: `c in {1, d}`, `B` a reduced degree-2 divisor of `P^1_{F_p}`
   (a split pair, a conjugate pair, or `infinity` plus a point); `2 p^2` models.
   **Control (p = 7):** the unreduced set (all `c in F_p^*`, every `h` of degree
   1 or 2 not necessarily monic, all `alpha`) must give the same set of `E`
   classes, and per `E` the same set of Smith outcome signatures.
4. **Descent** `H(h, alpha)`: Joux-Vitse Appendix A.1 / Momose-Chao,
   `H: y^2 = 4 N^2 h(F/(4N))`, `N = min poly of alpha`,
   `F = N (x + Tr alpha) + sum_i sigma^i(D) N/(x - sigma^{i+2} alpha)` with
   `D = (alpha - sigma^2 alpha)(sigma alpha - sigma^2 alpha)`, scaled by `c`.
   Checks per instance: `N1 = p+1`, `N2 = p^2+1`, `N3 = p^3+1-3t` with
   `t = p^3 + 1 - #E(K)` (so `L_H = 1 - tT^3 + p^3 T^6`).
5. **Smith outcome of `H`**: from `analyze_curve`: cycle type, `#S(H)`, number
   of subgroups passing Prop. 2 (`n_trig`), passing Prop. 4 (`n_iso(false)`),
   `pass(H) = full_success(false)`. `pass(E) = OR` over `E`'s weak models.
   The quadratic-twist outcome `full_success(true)` is recorded but is not a
   pass.
6. **Graph.** Nodes: `K`-isomorphism classes of elliptic curves (key
   `(j, chi(A B A0 B0))` for `j != 0, 1728`; discrete-log class for `j = 0,
   1728`). Edges: rational 2-isogenies (Velu, kernel = a `K`-rational point of
   order 2) and, for the second graph, rational 3-isogenies (Velu, kernel
   polynomial root in `K`). Components are explored from every weak node;
   all explored nodes are kept, weak or not. Controls: `#E'(K) = #E(K)` on
   every edge; each edge's dual exists.
7. **JV-weak test** (sampled arm and a column of Q1): `E` has full rational
   2-torsion, roots `b_1, b_2, b_3` and `b_4 = infinity`, and for some
   partition into `{b_i, b_j} | {b_k, b_l}` the cross-ratio
   `r = ((b_l - b_i)(b_k - b_j)) / ((b_l - b_j)(b_k - b_i))` has
   `N_{K/F_p}(r) = 1` (Hilbert 90 gives `alpha`). Weak models of `E` built this
   way: the 6 choices of `{i, j}` times the 2 orders of `(k, l)`, each with
   `c` fixed by matching the `K`-class of `E`.

## Measurements

**Q1**, for `p in {7, 11, 13, 17, 19, 23}` and, if the wall budget allows,
`29, 31` (each cell <= 1 h):
- number of models, of `E` classes, models per `E`, split / conjugate / degree-1
  share; share of weak `E` that are JV-weak;
- per model: cycle type, `#S(H)`, `n_trig`, `n_iso`, `pass(H)`; shares of `H`
  with a tractable subgroup, with Prop. 2, with Prop. 4, with `full_success`
  (compared with Smith's 0.1857 and 0.5016 and the problem record's class ceiling
  0.43-0.50);
- per `E`: `pass(E)`, `t`; per component (2-graph, and 2/3-graph): size, weak
  count, whether it contains a passing `E`, whether pass is constant on its weak
  nodes; per `K`-isogeny class (same `t`): the same;
- shares: `D_H` (passing `H`), `D_E` (passing `E`), `C_2` (weak `E` whose 2-
  component holds a passing `E`), `C_23`, `C_t` (same-`t` class);
  `r_X = (C_X - D_E)/(U - D_E)` with `U` the same-`t` class share;
- distance distribution from each failing `E` to the nearest passing `E'` in
  the 2-graph and the 2/3-graph (`inf` if none in the component);
- invariance finding: of components with at least 2 weak `E`, the share that are
  monochromatic in `pass(E)`; the walk is "killed by invariance" iff all are.

**Q2**, primes `p_k` = the first prime above `2^k` for `k = 10..20` (all
`k <= 16`, then `k = 18, 20`, further `k` if budget remains):
- **Arm A** (the task): random failing weak start `E_0` (JV-split family,
  `h = c x`, `alpha` uniform in `K \ F_p`, `c` uniform in `{1, d}`, rejected if
  any of its weak models passes), non-backtracking 2-isogeny walk through nodes
  with full 2-torsion (a step to a node without full 2-torsion is not taken:
  such a node has only the dual edge and is never JV-weak), until a node's JV-weak
  models include a passing `H`. Per walk record: steps, nodes visited, JV-weak
  nodes visited, models tested, Smith calls, `F_p` multiplications
  (instrumented counter), inversions, wall time per component
  (step / test / descent / Smith).
- **Arm B** (reaching the weak family): random full-2-torsion curve (three
  uniform roots), walk until the first JV-weak node; steps and cost.
- Walks per `p`: 40 for `k <= 13`, 24 for `14..16`, 12 for `17..18`, 6 above, each
  capped at 2e7 nodes and 55 min wall; capped walks are right-censored and the
  mean is the censored-exponential estimate `(total steps)/(successes)`.
- **Cost unit.** `F_p` multiplications as counted by the instrumented field
  (K-multiplication = 13 F_p multiplications in the implementation; additions
  free), inversions priced at the measured multiplication equivalent
  (micro-benchmark on the pinned cores, recorded). `analyze_curve` is not
  instrumented: its wall time is converted by the measured seconds per `F_p`
  multiplication (labelled derived). Wall time is recorded on the pinned cores
  `29,30`. The normalised unit `S = cost / sqrt(l)` is not used: no discrete log
  is solved here.
- **Fit.** OLS of `ln(steps_to_first_pass)` and `ln(cost)` on `ln p` with a
  bootstrap 95% interval over walks (and over primes for the slope). Frozen
  verdict on the exponent `a` in cost `~ p^a`: `o(p)` iff the whole 95%
  interval is below 1; `Theta~(p)` iff it contains 1; `omega(p)` iff above.
  Also reported: hit rate `h(p) = JV-weak nodes / nodes`, pass probability per
  JV-weak node `pi(p)`, with the model `steps = 1 / (h pi)`.

## Pre-registered expectations (to be falsified)

- E1: `N(r) = 1` is one `F_p`-condition, so `h(p) ~ c/p` and the walk needs
  `Theta(p)` nodes; the verdict will be `Theta~(p)`, not `o(p)`. If measured
  `a` is below 1 we say so.
- E2: the share of weak `E` whose component holds a passing `E` is above the
  direct share, below 1; passing is not constant on components.
- E3: `pi(p)` is near Smith's 0.186 per `H` (more per `E`, with several models).
  A structural departure (for example Prop. 2 always failing, because the
  descended `H` has three bielliptic involutions) is a finding, not an error.

## Stop and amendment rules

- Any control failure stops the run; the fix is recorded as a dated amendment
  below before data are used. No result is dropped.
- Failures of `analyze_curve` (`None`, non-squarefree `H`) are counted and
  reported, never silently skipped.
- Walk or wall caps are reported as censoring.

## Literature note (Q3) scope

Howe-Leprevost-Poonen; Smith (thesis, J. Cryptology 2009); Couveignes-Ezome;
Milio; Bruin-Doerksen and related genus-3 isogeny-graph work; Ritzenthaler et
al.; Cosset-Robert; Dartois-Maino-Pope-Robert; Tian. arXiv/eprint abstracts
and statements read via curl; what was not readable is stated.

## Amendments

(none yet)

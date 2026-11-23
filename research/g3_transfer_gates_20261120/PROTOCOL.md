# Genus-3 transfer gates (leads #3 and #5): protocol

Frozen before any census data or theta computation (hashes in `frozen.sha256`).
Breakthrough-search report: `research/hec_breakthrough_search_20261120/REPORT.md`,
ranks #3 and #5. Standing of every number produced under this protocol:
`[pcc:unverified]` (one session, shared host, no independent replay).

Constraints: pure Python (standard library; `mpmath` from PyPI in a scratchpad venv for
the theta arm only), one thread, total compute under about 60 minutes. No isogeny is
computed over a finite field; nothing here is a benchmark.

## Lead #3: literature gate, plus one numerical check

Question. For a genus-3 hyperelliptic Jacobian `J`, the 135 Lagrangians of `J[2]` split
into 105 of pair type and 30 of affine type (the 30 affine `F_2^3` structures on the 8
Weierstrass points). (a) Is the codomain `J/L` for affine-type `L` known to be a
hyperelliptic Jacobian, a plane-quartic Jacobian, or curve-dependent? (b) Is any
affine-type `L` Galois-stable when `#J` is odd?

Literature. WebSearch with plain mathematical terms only (no project details). Each
source is recorded in `LITERATURE.md` with what it says, and whether the statement was
read in the source or only in a search summary.

Numerical arm T (decides (a) over C for random curves, if the literature does not):

- Curves: `y^2 = prod_{i=1..8} (x - e_i)`, `e_i` real, drawn with `random.Random(seed)`,
  seeds 1, 2, 3, `e_i = 3(i-1) + u_i`, `u_i` uniform in [0.2, 1.8] (well separated).
- Period matrix `tau` from a symplectic basis of rectangular contours (a_i around
  `[e_{2i-1}, e_{2i}]`, b_i around `[e_{2i}, e_7]`, nested), Gauss-Legendre per side,
  `sqrt` continued along the path, 30 significant digits (mpmath). The sign of the
  b-cycles is chosen as the unique choice giving symmetric `tau` with positive-definite
  imaginary part.
- Gate T0 (instrument): `|tau - tau^T| < 1e-20`, `Im tau > 0`, and exactly one of the 36
  even theta-nulls of `J` vanishes (`|theta| / max|theta| < 1e-12`, all others
  `> 1e-6`). That vanishing characteristic `delta_0` fixes the quadratic form
  `q_0(m, n) = m.n + a.n + b.m` (`delta_0 = [a; b]`) on `J[2] = F_2^6`; weight-2
  elements (differences of two Weierstrass points) are `q_0 = 1`, weight-4 are
  `q_0 = 0`. A Lagrangian is affine type iff `q_0` vanishes on it (30 of 135), pair
  type otherwise.
- Lagrangians computed: `L_0 = {(0, n)}` (codomain `2 tau`) and the 64 graphs
  `L_S = {(m, S m)}` (codomain `(tau + S)/2`, `S` symmetric 0/1). Each is classified
  affine/pair by `q_0`.
- Codomain test: the 36 even theta-nulls of the codomain period matrix.
  Hyperelliptic = exactly one with `|theta|/max < 1e-12` and the rest `> 1e-6`;
  plane quartic (indecomposable, non-hyperelliptic) = all 36 `> 1e-6`; anything else
  = undetermined.
- Decision (a), numerical arm: if every affine-type codomain on every curve that passes
  T0 is hyperelliptic: "affine-type codomain is hyperelliptic (numerical, 3 random
  curves over C)". If every one is a plane quartic: "generically plane quartic". Mixed:
  "curve-dependent". Pair-type codomains are reported alongside (expected: plane
  quartic generically, Smith/Donagi-Livne).
- Limits fixed now: three real-branch-point curves over C, double-checked by a second
  quadrature size; this is evidence about the generic member of the family, not a
  proof, and not a statement about a specific finite-field curve.

(b) is answered by the derivation already in the lab
(`research/g2plus_leads_20261116/L06_odd_ell_isogeny_census/DERIVATION.md` section 4,
checked by the L06 assessor) and re-checked here by the `l = 2` control of lead #5.

## Lead #5: census gate for odd-l rational Lagrangians

### Population and instrument

- Model `y^2 = f(x)`, `f` monic of degree 7 over `F_p` (every odd-degree model is
  isomorphic to a monic one). Primes `p in {29, 31, 37, 41, 43, 47}` (mod 3: 2,1,1,2,1,2).
- Draw `f` with `random.Random(1000003 * p + i)`, `i = 0, 1, 2, ...`; keep `f`
  irreducible (necessary for odd `#J`); count `N_1, N_2, N_3` by exhaustive summation of
  the quadratic character over `F_p`, `F_{p^2}`, `F_{p^3}` (log/Zech tables of a fixed
  primitive element); `S_r = p^r + 1 - N_r`; Newton gives
  `P(T) = T^6 - e1 T^5 + e2 T^4 - e3 T^3 + p e2 T^2 - p^2 e1 T + p^3`; `#J = P(1)`.
  Stop at 100 prime-order curves per prime (600 in all).
- Instrument gates (before the census, on separate seeds `random.Random(7 + i)`):
  G1 `e2`, `e3` integral and Weil bounds on every curve; G2 at `p = 13` and `p = 17`,
  `N_4` counted directly over `F_{p^4}` equals `N_4` predicted from `P` on 10 curves
  each; G3 `P mod 2` of every prime-order census curve equals
  `(T^3+T+1)(T^3+T^2+1)` (the `l = 2` control, below).
- Secondary arm B (replication at larger p, not used for the verdict): the 600 PRIME
  curves of the L06 census (`p = 257 ... 4099`, `raw/census_p*.jsonl`, recorded
  `s1, s2, s3`), whose `chi` is the same polynomial in the L06 convention. That census
  was void by its own gate (G1, ambiguity on gate draws, not on PRIME curves); its
  `chi` was spot-checked by its assessor on 6 curves. Used only as a cross-check.

### What the characteristic polynomial decides (method and limits)

`pi` acts on `V = J[l] = F_l^6` (`l != p`) with characteristic polynomial
`Pbar = P mod l`, and `e(pi x, pi y) = e(x, y)^p`. For a monic `h | Pbar` of degree d,
`h^*(T) = T^d h(p/T) / h(0)`. Facts used:

1. A `pi`-stable Lagrangian `L` has `charpoly(pi|L) = h` with `h h^* = Pbar`
   (`V/L ~ L^dual` twisted by the multiplier). Necessary.
2. `V` is the orthogonal sum of primary blocks: `V_f` for self-dual `f = f^*`
   (nondegenerate), and `V_f + V_{f^*}` for `f != f^*` (each half isotropic). A stable
   `L` is the sum of its intersections with the blocks, each Lagrangian in its block.
3. If `gcd(h, h^*) = 1`, `ker h(pi)` is a stable Lagrangian whatever the module
   structure (L06 DERIVATION section 1). If `h` is also squarefree, `pi` is semisimple
   on it and `k` = lcm of the multiplicative orders of the roots of the factors of `h`.
4. `k` = order of `pi` on `L` = (order of the semisimple part) times `l^j`, `j >= 1`
   iff `pi|L` is not semisimple. `L` is pointwise rational over `F_{p^k}`, so
   `L <= J(F_{p^k})[l]`, which needs `l^3 | #J(F_{p^k})` (computed from `P`, integrally).
5. Block guarantees independent of the module structure: a pair block of multiplicity
   1 (squarefree part) offers `V_f` or `V_{f^*}`; a self-dual block of multiplicity 1
   has no stable Lagrangian piece (irreducible, nondegenerate) so no stable `L`
   exists; a self-dual block of multiplicity 2 always has a stable Lagrangian piece on
   which `pi` is semisimple with the order of a root of `f` (semisimple: an isotropic
   `F_l[T]/f`-line exists in a 2-dimensional alternating or hermitian space; cyclic:
   `ker f(pi)` is Lagrangian in the block).

The charpoly does **not** determine the module structure when `Pbar` has repeated
factors (semisimple versus Jordan blocks), and in those cases neither existence nor
`k` is always decidable from `Pbar` and the integers `#J(F_{p^k})`. That would need the
actual action of `pi` on `J[l]` (torsion points over `F_{p^k}`, not computed here).

Per curve, per `l`, and threshold `K` on `k`, the status is:

- **YES** (decided): every primary block is of a guaranteed kind (fact 5, pair blocks
  of multiplicity 1, self-dual blocks of multiplicity 2) and some choice gives
  `lcm <= K`; this includes every squarefree `Pbar` with no self-dual factor.
- **NO** (decided): a self-dual factor of multiplicity 1 exists, or no factorisation
  `h h^* = Pbar` (fact 1) with a possible `k = k_ss(h) l^j <= K` that also satisfies
  `l^3 | #J(F_{p^k})` (fact 4).
- **AMBIG**: neither.

For the existence question (no `K`), the same with `K = infinity` and fact 4 skipped.
Recorded per curve and `l in {2, 3, 5, 7}`: `Pbar`, its factorisation, status for
existence, the number of stable Lagrangians and their `k` when decided (squarefree),
and status for `k <= 4` (all `l`) .

### Predictions written before data (l = 3, prime order)

- Roots of order dividing 4 in `F_9`: `+-1` and the roots of `T^2 + 1`. Prime `#J`
  gives `P(1) != 0 mod 3`, so `T - 1` does not divide `Pbar`.
- P1. `p = 2 mod 3` (29, 41, 47): `(T+1)^* = T - 1`, so any `h` with root `-1` forces
  `T - 1 | Pbar`; `h` built from `T^2 + 1` alone has even degree. So **no prime-order
  curve at `p = 2 mod 3` has a stable `l = 3` Lagrangian with `k <= 4`** (NO on all).
- P2. `p = 1 mod 3` (31, 37, 43): only `Pbar = (T+1)^2 (T^2+1)^2` (decided YES, k = 4,
  fact 5, both blocks self-dual of multiplicity 2) or `Pbar = (T+1)^6` (k <= 4 needs
  `pi = -1` on `L`, so `L <= J(F_{p^2})[3]`; NO when `27 ∤ #J(F_{p^2})`, else AMBIG).
- P3 (`l = 2` control): every prime-order curve has `Pbar = Phi_7 mod 2`, exactly 2
  stable Lagrangians (the affine-type ones), each with `k = 7`.
- Rough expectation from Sp_6(F_3) class sizes (unipotent share about 3^-3): `R_low`
  about 1-4 %, `R_high` a few points above it. The verdict can be INCONCLUSIVE.

### Kill rule (frozen; from the referee ranking)

The `l = 3` route is killed if fewer than 5 % of prime-order curves have a rational
`l = 3` Lagrangian with `k <= 4`. With `N` = 600 arm-A prime-order curves,
`R_low = #YES / N`, `R_high = (#YES + #AMBIG) / N`, Wilson 95 % intervals:

- **KILL** if the Wilson upper bound of `R_high` is below 0.05;
- **SURVIVE** if the Wilson lower bound of `R_low` is at least 0.05;
- **INCONCLUSIVE** otherwise (report point estimates and what would resolve it).

Also reported (no verdict attached): `l = 3, 5, 7` existence rates (YES, NO, AMBIG),
the distribution of `k_min` over decided-YES curves, and rates for `k <= 4, 6, 8, 13`.
Arm B is reported with the same statistics.

### Files

`census.py` (instrument and classifier), `census_out.jsonl` (one row per prime-order
curve), `census_summary.json`, `gates.json`, `theta_arm.py`, `theta_out.json`,
`LITERATURE.md`, `RESULTS.md`. Deviations, if any, are listed in `RESULTS.md` under
"Deviations".

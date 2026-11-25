# Route 1 memo: a T-independent solver for the PKM m=2 tower membership system

**Date** 2026-11-25 · **Scope** public/toy instances only (p = 12289, 40961, 65537;
no keys, wallets or third-party systems touched) · **Status** measurement, not a
T2/T3 claim.

---

## 1. Headline

The F4 wall is removable. A triangular/recursive-tree solver decides the same
m=2 tower membership system at **127 field multiplications per element of V**,
with that constant **flat across T**. Per-system cost becomes `c · 2^T` with
`c = 127` (121–127 measured, T = 5…8), instead of F4's measured
127.4 M / 2.003 G / 79.75 G multiply-adds.

| per level of T | F4 (measured) | this solver (measured) |
|:--|---:|---:|
| T=5 → 6 | 15.72× | **2.11×** |
| T=6 → 7 | 39.82× | **1.99×** |
| T=7 → 8 | — | **2.00×** |

So the per-system exponent in T drops from **≈ 4.0–5.3** to **exactly 1.0**.
Because the PKM m=2 parameters force `T ≈ 0.5 log2 p`, this is
`per system = Θ(p^0.5)`, against F4's measured `Θ(p^2.66)`.

**But "poly(T) per system" is still NO**, and the honest reason is structural,
not an implementation gap (section 6). What the solver does buy is that the
*solver* stops being the bottleneck: the binding term moves to the relation
**join**, which lands at `Θ(p^1.0)` — a factor `p^0.5` above rho. Section 7.

---

## 2. What the solver is

The system (`pkm_semaev.rs::build_system_fixed`, `mode=full`, `m=2`, `x_R`
specialised to the target abscissa `c`):

```
variables  y_{i,0..t-1}, i = 1,2                      -> 2t
equations  y_{i,j}^2 - y_{i,j+1} = 0   (j < t-1)      -> t-1 per summand
           y_{i,t-1}^2 - 1       = 0                  -> 1   per summand
           S_3(g y_{1,0}, g y_{2,0}, c) = 0           -> 1
                                                      -> 2t+1
```

Three facts make it T-independent:

1. **The tower is triangular and every level is a quadratic**, so it is a binary
   tree of depth `t` with `2^t` leaves and *no Macaulay matrix*. This is exact
   for the `prime-kummer` arm (`y -> y^2`, zero set `= g·μ_{2^t}`); for
   `prime-isogeny` the harness encodes the Vélu step as the same shape
   (`SquareRule` / `tower_rule_roots`: `y_{j+1}^2 - ξ_j y_{j+1} + τ_j = 0`), so
   the tree is identical and only the coefficients change. **The proxy claim,
   stated explicitly: every cost below depends only on the branching factor 2
   and on an explicit listing of V, so it is coefficient-independent and
   transfers verbatim to the isogeny arm.**
2. **S_3 is quadratic in x_2**, so the second summand is *solved*, never
   searched: with `x_1 = X`, `x_3 = Y`, `S_3(X, x_2, Y) = A x_2^2 + B x_2 + C`
   with, in closed form,
   `A = (X−Y)²`, `B = −2XY² − 2Y(X²+a) − 2Xa − 4b`,
   `C = X²Y² − 2XaY − 4bY + a² − 4bX`.
   This is **8 multiplications, independent of t** — that is the entire
   "symbolic setup"; there is nothing to re-prepare as T grows.
3. The `2^t` leaves are the cyclic group `μ_{2^t}`, so they are walked with
   `2^t` multiplications. The literal alternative — back-substituting the tower
   from `y_{t-1} ∈ {±1}` with square roots — costs `2^t` square roots, measured
   at **12–150× more** (`build_sqrt_mul` 4.9 k / 10.4 k / 21.5 k / 44.4 k vs
   `build_gen_mul` 32 / 64 / 128 / 256).

Cost per leaf, measured: 8 mul for the quadratic + 2 for the discriminant +
one `√Δ` (Legendre test + Tonelli–Shanks ≈ 110 mul at 14 bits) + one inverse
(batched: +3). Dominated by the single square root — which is intrinsic: the
root of the quadratic must be found, and `μ_{2^t}`-membership cannot be decided
without it.

---

## 3. Measured table

`p = 12289` (14 bits, `v2(p−1)=12`), `E: y² = x³ + x + 1`. Op unit = F_p
multiplications (squarings counted as 1); "full" = a complete scan (the
refutation cost, and the unit directly comparable to an F4 solver call).

| T | vars/eqs | \|V\| | mul, full scan | mul / leaf | wall, full scan | F4 mul-adds/call | F4 ÷ this |
|--:|:--|--:|--:|--:|--:|--:|--:|
| 5 | 10 / 11 | 32 | 3,874 | 121 | 311 µs | 1.274e8 | **3.3e4×** |
| 6 | 12 / 13 | 64 | 8,159 | 127 | 621 µs | 2.003e9 | **2.5e5×** |
| 7 | 14 / 15 | 128 | 16,218 | 127 | 1.26 ms | 7.975e10 | **4.9e6×** |
| 8 | 16 / 17 | 256 | 32,474 | 127 | 2.44 ms | *(outside panel)* | — |

Sanity: "mul/leaf" is 121, 127, 127, 127 — flat, which is the whole claim.
Early exit (the solution exists and is found before the last leaf) averages
4,276 / 7,456 / 12,259 mul at T = 5/6/7 on uniform random targets, and 97 % of
random targets are decomposable at T = 8 (`|V|² = p·5.3`).

**The only p-dependence is `log p`, not `p`** (second and third moduli, same code):

| p | bits | mul/leaf at T = 5,6,7(,8) |
|--:|--:|:--|
| 12289 | 14 | 121, 127, 127, 127 |
| 40961 | 16 | 85, 84, 84 |
| 65537 | 17 | 96, 92, 92, 93 |

Flat in T at every modulus. (12289 is the *most* expensive despite being the
smallest because `p−1 = 2^12·3` gives Tonelli–Shanks a long 2-adic loop; the
constant is set by `√Δ`, not by `p`.)

**Solving degree.** There is none: no Macaulay matrix is ever formed, each
level contributes degree 2, and the only high-degree object (`v^{2^t} = 1` on
the second summand) is avoided by using the explicit group parametrisation
instead of the algebra.

---

## 4. Why F4 costs what it costs — the `4^t` invariant

The tower ideal is a zero-dimensional **complete intersection of 2t quadratics**
(plus one quartic). Its quotient dimension is therefore the Bezout number

```
    deg = 2^{2t} · 4 = 4^{t+1},
```

although only `O(1)` of those points are `F_p`-rational (the tower has `4^t`
rational points on its own; S_3 cuts that to ~1). F4/Groebner/resultant engines
pay for the *quotient dimension*, not for the solution count: their cost is
`(4^t)^{2.0…2.65}`.

That is a base-`4` growth per level, and the measured F4 steps are exactly that:

```
    15.72× = 4^1.974       39.82× = 4^2.646
```

Two consequences, both of which the existing programme data already showed and
this explains:

- the wall **steepens** at T=7 (14 variables) because the elimination exponent
  itself rises as the Macaulay matrix grows — it is not a constant 15×;
- the **isogeny arm's 32–36× is necessarily a constant**: it has the *same*
  `2t` quadratics and the *same* `4^{t+1}`, differing only in coefficient
  density. No amount of encoding work can move a Bezout number. This is the
  structural reason the encoding-width diagnostic found a constant and had to
  retract its "encoding term" framing.

---

## 5. Is poly(T) per system achievable?

Split the question, because the answer is different for the two halves:

| block | cost | in T |
|:--|:--|:--|
| symbolic reduction of the `2t+1`-equation system to the bivariate quadratic | 8 mul, **once** | **O(1)** — actually constant |
| deciding one system | `≥ 2^T` leaves × O(1) | **Θ(2^T)** |

So: **the elimination is O(1) — cheaper than poly(T); the decision is Θ(2^T)
and is not poly(T).** No exact method can be poly(T) here: the constraint set is
`μ_{2^t} × μ_{2^t}` of size `2^{2t}`, the ideal is a complete intersection of
degree `4^{t+1}`, and the answer set has Θ(1) elements, so a decision procedure
must sift a `2^T`-sized structured set. Every elimination-flavoured route
(resultant, Groebner, F4) is *worse* than the tree, because it pays `4^t`
instead of `2^t`. The tree solver is the natural optimum of this family, and it
is `2^T`, not `poly(T)`.

This is still the decisive improvement the cost-model memo asked for at item
(1) — "a T-independent membership/elimination method ... cost per system
`2^{o(T)}` (ideally `poly(T)`)". What it delivers is `2^T`, not `2^{o(T)}`:
the elimination half of the requirement is met (O(1), better than poly(T)) and
the decision half is provably not (Θ(2^T)). At the fixed T of any panel the
per-call win is `3.3e4× – 4.9e6×`, and the slope in T falls from 4–5.3 to 1.

---

## 6. The remaining relation-join cost

Two independent routes, both measured:

**(a) Per-target scan, `O(2^T)` memory.** Each decision is `c·2^T`. The number
of decisions needed is `|V|` divided by the fraction of `F_p` that is
decomposable; that fraction was measured directly (it is the "coverage" column
below): 0.481 at `T=7` where `|V|²/p = 1.33`, so at `|V|² ≈ p` take `≈ 0.4`
(2.5·|V| decisions). So

```
    total ≈ 2.5 · 2^T · 127 · 2^T ≈ 320 · 2^{2T} = 320 · p^{1.0}
```

**(b) One-pass pair table, `O(p)` memory.** Enumerate every unordered abscissa
pair, hash by `x(P1+P2)`, then answer every target in O(1):

| T | pairs | distinct sums | coverage of F_p | ma/pair | total ma |
|--:|--:|--:|--:|--:|--:|
| 5 | 496 | 515 | 4 % | 150 | 7.4e4 |
| 6 | 2,016 | 1,840 | 15 % | 140 | 2.8e5 |
| 7 | 8,128 | 5,911 | 48 % | 140 | 1.1e6 |
| 8 | 32,640 | 11,522 | 94 % | 141 | 4.6e6 |

The pair count is `C(2^T,2) ≈ 2^{2T−1} = p/2`, so the total is
`≈ 141 · p/2 = 70 · p^{1.0}` — same exponent as (a), about 4.5× cheaper in
constant, at the price of storing `Θ(p)` sums (the `T=8` row already reaches
94 % coverage, so the table is essentially complete).

Both routes give **exponent 1.0 in p**, i.e. **`p^{0.5}` worse than rho**
(`p^{0.5}`), while F4's route gave `p^{2.66}` per call — so the solver fix is
real but insufficient: PKM m=2 with an optimal exact solver is still strictly
worse than rho, and it is the **join**, not the solver, that now binds.

Note that raising `m` does **not** escape this under exact enumeration: with
`|V| = p^{1/m}`, one decision costs `|V|^{m−1} = p^{(m−1)/m}` and `|V|`
relations are needed, so the total is `p^1` for every `m`. Only the memory
profile and the per-decision constant improve. The exponent moves only if the
`m`-sum itself is solved faster than `|V|^{m−1}`.

---

## 7. Next concrete step

The join is now the only object between this solver and a sub-rho exponent, so
the next experiment should attack it and nothing else. In exponent coordinates
`x_1 = gζ^a`, `x_2 = gζ^b` the leaf equation becomes an **8-term exponential
sum in `a`** (`A, B, C` have degree ≤ 2,3,4 in `ζ^a`, so `Δ` has degree ≤ 8),
and the join asks for `a ∈ Z/2^t` with `Δ(ζ^a) ∈ QR` and the root in `μ_{2^t}`:

1. **Cheapest, highest-information next probe (recommended): the m=3 chain with
   the same tree.** `mode=chain`, m=3 is two equations (`S_3(x₁,x₂,u)`,
   `S_3(u,x₃,c)`) over `3t+1` variables. Two equations give a *pruning* signal
   that m=2 cannot have (m=2 has one equation, so its tree can only prune at
   the last level). Measure whether fixing `x₁` and walking `x₂`'s tower one
   bit at a time rejects branches before the leaf: if it does, the per-decision
   cost drops below `|V|² = p^{2/3}` and the total below `p^1` for the first
   time. This is a ~100-line extension of the existing prototype and reuses
   every op-counting and verification path already in it.
2. **2-adic lifting on the exponent.** `a ∈ Z/2^t` is a 2-adic variable;
   `ζ^{a₀ + 2a₁ + …} = ζ^{a₀}(ζ²)^{a₁}…` gives a genuine bit-by-bit lift.
   With one equation (m=2) it cannot prune; with two (m=3) it can, which is the
   same experiment as (1) phrased as Hensel lifting. Test whether the lifted
   system has `o(2^t)` consistent partial assignments at level `t/2`.
3. **Sparse/pronynomial root finding for the exponential sum.** `Δ(ζ^a)` is an
   8-term exponential sum; deciding `Δ(ζ^a) ∈ QR` for all `a` is currently
   `O(2^t)` evaluations. A sub-`2^t` method would need a structured root finder
   (Prony/MUSIC/compressed sensing over `Z/2^t`). Lower probability, but it is
   the only route that would move the *per-decision* exponent.

Do **not** spend more effort on solver engineering: the constant 127 is within
~1.5× of the information-theoretic floor (one square root per leaf), and the
`2^T` leaf count is structural. Further constant-factor work cannot change any
exponent.

---

## 8. Claim boundary

- Toy public instances only: `p ∈ {12289, 40961, 65537}`, `T ≤ 8`. Nothing here
  is a statement about a deployed curve or a NIST-size parameter set.
- The F4 comparison reuses the programme's own measured per-call numbers
  (127.4 M / 2.003 G / 79.75 G) from `SUBEXP_COST_MODEL_20261125.md`; this work
  did not re-run F4.
- "mul" here counts F_p multiplications with squarings as 1; F4's unit is
  multiply-adds. They are the same order; the ratios quoted are
  `1.3e4 – 4.9e6`, far outside any unit-convention risk.
- The isogeny transfer is argued from the *shape* of `SquareRule` (one
  quadratic per level) plus the fact that the solver only uses the branching
  factor and an explicit list of V. It is not measured on an isogeny tower; it
  is a structural-proxy argument, as flagged in §2.1.
- The `4^{t+1}` reading in §4 is exact algebra; that it *explains* the measured
  15.72×/39.82× is a consistency argument at two data points, not a proof.
- No T2 or T3 claim. Section 6's `p^{1.0}` is a cost-model statement about
  exact enumeration, not a lower bound on all algorithms.

---

## 9. Files and exact commands

Everything is under `/Volumes/SSD990/llm/tmp/opencode/subexp_routes/route1/`.
The git repo was not modified.

```bash
cd /Volumes/SSD990/llm/tmp/opencode/subexp_routes/route1

# full measurement: T = 5,6,7,8 at p=12289, join tables, 2 extra moduli
python3 route1_tower_solver.py

# T = 5,6 only (~0.6 s)
python3 route1_tower_solver.py --quick
```

| file | what |
|:--|:--|
| `route1_tower_solver.py` | the prototype: closed-form quadratic, tree solver (plain + batched-inverse), chain-membership variant, naive grid oracle, join table, self-tests |
| `route1_results.json` | every number in §3 and §6, machine-readable |
| `ROUTE1_MEMO.md` | this memo |

Built-in verification (runs on every invocation, before any timing):

- the closed-form `A,B,C` equals a direct `S_3` evaluation on 300 random points;
- the generator walk of `μ_{2^t}` equals the square-root back-substitution walk;
- the tree solver equals the naive `V × V` grid on 10 random targets, for all
  four combinations of {plain, batched} × {hash membership, chain membership};
- the scaled naive-grid op count is checked against a real full grid at T ≤ 6
  to within 2 %.

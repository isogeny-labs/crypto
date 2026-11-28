# Three routes to a subexponential ECDLP algorithm — synthesis

## Final verdict (all three routes complete)

**None of the three routes reaches a subexponential exponent. The current PKM
family is closed above rho.**

| Route | Best measured / provable | Exponent in p | Versus rho (0.5) |
|:--|:--|---:|---:|
| 1 — tower tree solver | `Θ(2^T)` per system, 127 mults/leaf | system `p^0.5`, join `p^1.0` | still worse; solver wall removed, join binds |
| 2 — structured join, m=2 | identity `ops/rel = 2N/|F|`, total ≈ `2p`; best sumset table | **0.769** (asymptote 2/3) | 16.2·√p at 40961 |
| 2 — m=3 join | pair table / random-tuple join | **0.620** (3/5) | 5.5–23× rho at 2^13 |
| 3 — m≥3 F4 | 13,182× per call at equal size, censored; no pruning | best possible 0.667, realizable 1.00 | not viable |

The memory-free random-tuple ladder is `m/(2m−1)` = 2/3, 3/5, 4/7, 5/9, …; it
approaches 1/2 **from above** and only with `p^{1/2}` memory. The pair-table
route needs `p^{2/3}` entries (2^170 at 256 bits). m=3 fixes the m=2 graph rank
defect but costs 13,182× per call and inverts the 32× isogeny advantage (the
interval base becomes 53× cheaper). No finite m and no measured structure
supplies a per-relation cost `|V|^θ` with θ < 1/2.

**What would count:** a genuinely new join with `θ < 1/2`, or a solver/join
with `2^{o(T)}` at `T ~ (1/m) log p`. The measured structures do not supply it:
the join lives in E, the algebraic structure lives in F_p; Kummer membership is
14× slower than a hash hit, the Vélu tower test 214× slower, and the isogeny
base is additively generic. Constants (32×) and fixed-m tuning cannot reach T3.


## Shared measured wall

F4 multiply-adds per solver call, interval arm, 13-bit toy instance:

| T | Variables / equations | D | MA/call | Ratio |
|---:|---:|---:|---:|---:|
| 5 | 10 / 11 | 6 | 127.4 M | — |
| 6 | 12 / 13 | 7 | 2,002.9 M | 15.72× |
| 7 | 14 / 15 | 7 | 79,748 M | 39.82× |

The isogeny arm is 32.0–32.15× cheaper at equal T (13–15 bits). That factor is
constant. At fixed T=5, charged cost per cold solve grows 0.78 per bit from 13
to 15 bits, already above rho's 0.50.

## Route 1 result — solver wall removed, join remains

Route 1 built an explicit tower tree solver (no Macaulay matrix) and measured
it at p=12289:

| T | vars/eqs | \|V\| | full-scan mults | mults/leaf | F4 MA/call | F4 / tree |
|---:|---:|---:|---:|---:|---:|---:|
| 5 | 10/11 | 32 | 3,874 | 121 | 1.274e8 | 3.3e4× |
| 6 | 12/13 | 64 | 8,159 | 127 | 2.003e9 | 2.5e5× |
| 7 | 14/15 | 128 | 16,218 | 127 | 7.975e10 | 4.9e6× |
| 8 | 16/17 | 256 | 32,474 | 127 | — | — |

The per-level growth is **2.0×**, not F4's 15.7–39.8×: the solver exponent in T
is 1.0, so per system it is `Θ(p^0.5)` at `T≈0.5 log2 p` (vs F4 `p^2.66`).
The symbolic reduction is O(1) (8 mults); the decision is provably `Θ(2^T)`,
not poly(T), because the tower ideal is a complete intersection of degree
`4^{T+1}` and only O(1) points are rational.

**The new bottleneck is the relation join, not the solver.** Both measured
joins — per-target scan and one-pass pair table — land at `p^1.0`, a factor
`p^0.5` above rho. Under exact enumeration, m≥3 is also `p^1.0` for every m
(one decision `|V|^{m-1}`, `|V|` relations). The only remaining route is a
sub-`|V|^{m-1}` m-sum join, e.g. per-level pruning for m=3.

## m-general random model

Let the factor base have size `|V| = p^alpha` and use `m` summands. The
random-model relation probability is

```text
rho_m = |V|^m / p = p^(m*alpha - 1).
```

A constant relation probability needs `alpha = 1/m`, hence

```text
T ~ log2 |V| = (1/m) log2 p.
```

If the per-system solver cost is `C(T) ~ R^T`, the total per-target exponent in
`p` is `log2(R) / m`:

| m | exponent, R=15.72 | exponent, R=39.82 |
|---:|---:|---:|
| 2 | 1.99 | 2.66 |
| 3 | 1.32 | 1.77 |
| 4 | 0.99 | 1.33 |
| 5 | 0.79 | 1.06 |
| 6 | 0.66 | 0.89 |
| 7 | 0.57 | 0.76 |
| 8 | 0.50 | 0.66 |
| 10 | 0.40 | 0.53 |
| 12 | 0.33 | 0.44 |

Under this optimistic fixed-`R` model, m=8–11 is where the exponent crosses
rho. But `C_F4` also grows with `m` (the system has roughly `m*T` variables and
more equations), so the table is a lower bound on the m≥3 cost, not a plan. The
measured F4 wall is exponential in variables, so increasing m increases the
problem the solver must solve.

## What each route must actually deliver

### Route 1 — T-independent tower solver

Target: `C(T) = 2^{o(T)}` (ideally `poly(T)`) for the triangular tower system,
beating 127.4 M / 2.003 B / 79.75 B MA per call at T=5/6/7 by a factor that
grows with T. A constant-factor improvement is not enough. Even with a
`poly(T)` solver, the m=2 relation join remains `Omega(|V|) = p^{1/2}`, so
Route 1 alone removes the F4 wall but does not by itself go below rho. It must
be paired with Route 2 or with growing m.

### Route 2 — structured relation join

Target: a join for `P1+P2=Q` over `V` that is `2^{o(T)}` with `|V| ~ p^{1/2}`,
i.e. sub-`p^{1/2}`. The generic 2-sum lower bound is `Omega(|V|)`; success
requires a real additive/multiplicative structure in the tower factor base
(`x(R'+<Q>)` or `g*mu_{2^T}`) that a toy experiment can exhibit. Without a
structural reason, a hash/tree/MITM trick cannot beat `|V|`.

### Route 3 — m≥3 construction

Target: a measured cost exponent at m=3 below the m=2 exponent (1.99–2.66 in
the model), with the m=3 F4 system still tractable. The fixed-m random model
says the exponent falls as `log2(R)/m`, but `C_F4` rises with `m*T`; the
question is whether the net exponent falls. If m=3 is not better than m=2, the
fixed-m ladder is a dead end.

## Decision rule

No route is a subexponential claim unless it produces a cost whose exponent in
`log p` is `o(1)` — equivalently, a `2^{o(T)}` solver *and* a `2^{o(T)}` join,
or a growing `m` with a solver subexponential in `m*T`. Constants (the 32×) and
fixed-m improvements cannot reach T3.

The three route reports are under `route1/`, `route2/`, `route3/`.

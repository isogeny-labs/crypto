# F6: physics-style solvers on the m = 2 decomposition landscape (protocol, frozen 2026-11-10)

Moonshot G1 (`research/prime_moonshot_20261110/ideas-generator.md`). Question: does any local
search / annealing method find a planted decomposition `R = P₁ + P₂`, `x(P_i) ≤ B`, in fewer
cost evaluations than the exact baselines, with an exponent in `B` below theirs? Baselines:
naive enumeration of the box, `Θ(B²)`; root-solving enumeration (for each `x₁` solve `S₃` for
`x₂`), `Θ(B)`. Class: boundary diagnostic.

## 1. Instances
`p` = largest prime below `2^n`, `n ∈ {12, 16, 20, 24, 28}`; one prime-order curve per `n`
(seed `F6/curve/<n>`); `B = ⌈p^{1/2}⌉` (so about one decomposition exists per random `R`; we plant
one: `P₁, P₂` uniform in the box, `R = P₁ + P₂`, seed `F6/inst/<n>/<i>`, 20 instances per `n`).

## 2. Landscape and solvers (native Rust, std only, `tool/`)
State `(x₁, x₂) ∈ [1, B]²`; cost `c(x₁, x₂) = min(v, p − v)` with `v = S₃(x₁, x₂, x_R) mod p`
(exact; `c = 0` iff `±` decomposition at these abscissae, checked by lifting and adding). Moves:
single bit flips of `x₁` or `x₂` (Gray-coded box), `±1` steps, and random restart.
Solvers (all seeded, same cost oracle): simulated annealing (geometric schedule, 3 schedules),
parallel tempering (8 replicas), greedy with restarts, and uniform random sampling (control).
Metric: **cost evaluations to first `c = 0`** (time-to-solution), cap `50·B²` (censored).
Landscape statistic: autocorrelation of `c` along bit-flip walks vs the same walk on a random
landscape with the same marginal (seed `F6/rand/<n>`).

## 3. Predictions (frozen)
P1. Median evaluations to solution ∝ `B^e` with `e ≥ 1.8` for every solver (the landscape has no
usable gradient: `S₃ mod p` is pseudo-random off the solution set).
P2. Landscape autocorrelation within the random band at every lag ≥ 1.
P3. Uniform random sampling has `e ≈ 2` (control).
Prior that any solver reaches `e ≤ 1` (the root-solving baseline): 0.005; `e < 0.5`: 0.0005.

## 4. Decision rules
1. **Instrument failure:** P3 fails, or a reported solution does not verify.
2. **Lead alive:** some solver's `e` upper 95 % CI `< 1.0` on uncensored `n ≥ 20`, or P2 fails at
   lag 1 on ≥ 3 sizes. Then replicate with 100 instances and `n = 32`.
3. **Closed:** otherwise: no local-search method beats root-solving enumeration on the `m = 2`
   decomposition landscape at these sizes.

## 5. Route and outputs
Local host; counts. `results/run.json`, README, SVG (log evaluations vs log B per solver with the
two baselines), REPORT.pdf, session log.

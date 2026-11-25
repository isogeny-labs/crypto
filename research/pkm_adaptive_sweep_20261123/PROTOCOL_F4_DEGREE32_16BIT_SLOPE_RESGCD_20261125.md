# 16-bit rung, 13→16-bit slope, and a univariate resultant–GCD engine — protocol

Frozen 2026-11-25 before any 16-bit or resultant–GCD measurement. Stacked on
PR #5067 (`codex/pkm-adaptive-sweep-20261123`). Runtime: Claude Code.

## Questions

1. **Slope.** Does the degree-32 candidate's advantage over the equal-size
   `prime-abscissa` control (combined charged GAE ratio) and its median
   `s_over_rho` *bend downward* with field size, or stay flat/rise?
2. **Solver.** Does an m=2 univariate elimination of the chain system,
   `R(X) = Res_Y(F_V(Y), S₃(Y, x_R, X)) = ∏_{v∈V} S₃(v, x_R, X)` then
   `gcd(R, F_V)`, escape F4's measured growth in the tower height t
   (interval arm 127.4M → 2.00B → 79.7B multiply-adds per call at t = 5, 6, 7;
   candidate 3.96M at t = 5, 55.9M at t = 6)?

## Engines (all in `pkm-semaev`, m = 2, same targets, same pipeline)

| engine | what it does per target | ops counted |
|:--|:--|:--|
| `f4` | the #5067 system through `f4_tower` | F4 multiply-adds (solver block) |
| `exhaustive` | every unordered pair in V² against S₃ (existing grid) | F_p mults, native counter (unpriced in GAE) |
| `resgcd` (new) | Karatsuba product tree for R, classical Euclid gcd with F_V | F_p mults, solver block (priced like F4) |
| `scan` (new) | for each v ∈ V solve S₃(v, X, x_R)=0 (one sqrt), hash-test X ∈ V | F_p mults, solver block; lookups counted apart |

An inversion or square root is charged 32 multiplications (module convention,
`solve_s3_for_middle`). `F_V` is target-independent and is built once
(`pkm_resgcd_setup_fp_mults`).

Correctness gate: unit test `agrees_with_grid_skipping` now runs `resgcd` and
`scan` with `crosscheck=1` on every membership form it already covers
(Kummer, isogeny tower, bit interval, digit set): verdict per target equal to
the grid's and to exact decomposability, and **zero** solution-set
disagreements. In every bench run below `crosscheck` is off (it would add the
grid's cost); the unit test carries correctness, and each bench run carries
recovered-and-verified scalars.

## Experiment A — per-call cost against t

Instance: `--bits 16`, CLI seed of the first 16-bit panel instance below.
For t = 5 … 12 and the candidate base `prime-isogeny:t=T,seed=<first aux seed
that builds>,aux=ordinary`, run `ic bench --repeats 1 --max-trials 200
--rho-runs 0` with each engine; F4 only at t ≤ 7 (79.7B MA per call at t = 7
on the interval arm makes it a single-call probe; candidate F4 at t = 7 is one
call with the same cap). Report ops per solver call. Fit log₂(ops/call)
against t: F4's measured per-step factor is 15.7× (t 5→6) and 39.8× (6→7);
resgcd is predicted ≈ 4× per step (classical Euclid, `O(|V|²)`), scan ≈ 2×
(`O(|V|)`).

Kill rule: any call over 900 s is stopped and recorded as censored.

## Experiment B — 16-bit closed-pipeline rung

Selection, frozen: CLI seeds `20261400..20261460`; for each seed read the
`prime-abscissa:below=32` abscissa count (preflight `--oracle subtract
--max-trials 1`), take the first ascending t=5 aux seed in `0..20` with an
equal count; **the first three matching seeds in ascending order** are the
panel. No cost is observed before selection.

Per instance, 16 repeats (16 independent cold solves), flags exactly as the
15-bit panel (`--family generic --targets walk --max-trials 20000
--solver-budget-seconds 5 --linalg incremental-gauss-full-rank-checked
--rho-runs 1`), `RAYON_NUM_THREADS=1`, calls in this order:

1. interval `f4` (`prime-abscissa:below=32`, `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`)
2. candidate `f4` (`prime-isogeny:t=5,seed=<aux>,aux=ordinary`, `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`)
3. candidate `resgcd`
4. candidate `scan`
5. interval `resgcd`
6. interval `subtract` (group-law enumeration oracle, the harness's own baseline)

The three instances run in parallel, one core each (host has 4 vCPUs; the
fourth is left idle). Operation counts are deterministic and are the metric;
`ns/op` and wall are load-dependent and are a practicality note only.

Gate per instance: every call exits 0; equal abscissa counts in both bases;
all 16 rows `recovered` and `verified`; `full_rank_reached`; scalars agree
across calls at each repeat index.

## Decision rule (frozen)

Slope: fit `log₂(combined F4 GAE ratio)` and `log₂(median s_over_rho)` (each
arm) against bits over all completed rungs 13, 14, 15, 16 (instance-level
points). Declare a **downward bend** only if the 16-bit candidate/interval
ratio lies below the minimum of the 13–15-bit instance ratios (0.263) *and*
the fitted slope is negative with its sign stable under leave-one-rung-out.
Otherwise the lead is a **constant formulation factor** and closes as such.
A rising candidate `s_over_rho` slope is reported as the exponent gap to rho.

Solver: if resgcd or scan per-call ops grow by ≤ 4× per unit of t while F4
grows ≥ 15×, the 32× "Vélu vs abscissa" signal is an F4-formulation effect,
not a property of the base. That holds if, in addition, `resgcd` costs the
same on both bases at equal |V| (step 5 vs 3).

## Host disclosure

Local cloud container, 4 vCPU, 15 GiB, no hardware PMU claims. AGENTS.md §9a
(Iso Lab EKS default) could not be followed: this session has no AWS CLI or
credentials. Results are operation-count evidence, not a host-qualified
speed claim.

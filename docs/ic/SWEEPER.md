# The sweeper: `ic sweep` and `ic sweep-trial`

An Optuna-style search over index-calculus pipeline configurations, in
Rust, built on `ic bench`. It answers one question: *which configuration of
factor base, descent, solver and matrix is cheapest, against a frozen
baseline, and does its advantage grow with size?*

Design, assessment and open decisions:
[`research/ic_sweeper_20261024/README.md`](../../research/ic_sweeper_20261024/README.md).
This page is the user guide.

```bash
cargo build --release --bin ic

ic sweep spaces                                   # what can be searched
ic sweep spaces --space koblitz-algebraic         # one space in full

ic sweep run --space koblitz-algebraic --koblitz-a 0 \
    --rungs 13,17,19,23 --target-log2r 81 \
    --trials 60 --workers 4 --repeats 2 \
    --store runs/koblitz-a0-algebraic.jsonl       # created, or resumed

ic sweep show --store runs/koblitz-a0-algebraic.jsonl
ic sweep-trial --config-json '{"regime":"koblitz","degree":13,"koblitz_a":0,
  "factor_base":"binary-subspace:dimension=4","oracle":"descent-algebraic:m=2",
  "solver":"inherited-f4"}'
```

## The trial: `ic sweep-trial`

One configuration, run through `ic bench`, reduced to one record. The
instance, the planted logarithm, the verification and the matched rho are
`ic bench`'s, not a second copy.

**The score is a function of operation counts alone.** AGENTS.md §5a prices
every arm's native work at a weight `w` and ranks two arms only where the
order holds across the band. With `G` the charged GAE, `C` the GAE a pinned
calibration put on native work, `n_C` the operations behind `C` and `U` the
count behind unpriced counters, `priced_gae(w) = G − C + w·(U + n_C)`. `G − C`
is the group operations and `U + n_C` is every native operation, so

```
priced_gae(w) = (adds + doubles) + w · (native operations)        w ∈ {0, 0.1, 1}
S(w)          = priced_gae(w) / √r
```

No calibration ratio, host timing or solver wall pricing enters, so the same
configuration scores identically. *Checked:* a debug and a release build on one
host gave identical `S`. *Not checked:* other hosts, and a solver that reads
the clock internally could still differ. *Native operations* are the counters `price_phase` converts, every
`*_uncharged` counter, and the solver's own `ops` in its own unit (the same
set ecbench's `ic.pipeline` lists as priced or unpriced). Any other native
counter appears under `context.other_native_unscored` rather than being
silently free.

Every trial ends in exactly one class, and only `ok` is scored:

| class | meaning |
|:--|:--|
| `ok` | every repeat recovered the planted logarithm |
| `exhausted` | a repeat hit `max_trials`, a solver budget, or a safety net, without recovering it |
| `invalid` | a repeat recovered the **wrong** logarithm: a defect, never a result |
| `refused` | the instance or model cannot represent the configuration |
| `invalid_config` | the configuration is malformed |
| `crashed` | the run panicked or produced no rows |

**The solver's unit must match the baseline's, exactly.** `solver.ops` is
counted in whatever unit the engine uses, and the *scope* of what it counts
differs even where the unit's name is the same. The repository prices every
`word XORs` and `word XORs (…)` by one pinned ratio, which is right for pricing
and wrong for ranking. Measured at n = 31 on one instance with identical
relations (group operations identical), `f4-f2` ("word XORs (elimination
only)") scored S(0.1) = 9,078 and `inherited-f4` ("… elimination,
specialisation and linear elimination only") 630,638: a 70× gap that is only
the scope each one counts. Other units differ outright: monomial operations
(`buchberger-f2`), conflicts (`sat-cdcl`), partial word operations
(`crossbred-f2`), Gray-code steps (`fes-f2`). A ratio across scopes says which
engine counts least, and a search would learn to pick it (AGENTS.md §6,
changing the unit). So:

- a rung whose solver's unit string differs from the baseline's is recorded in
  full (`rankable: false`, band and `op_unit` kept) and **fails the trial**;
- each algebraic space offers one scope, so its sampler never learns around
  trials that cannot be ranked (failures caused by one parameter had been
  poisoning the sampler's model of another: with unrankable solvers on offer,
  60 trials never sampled the `m = 3` setting that the mock rewarded);
- engines with no scope-mate (Crossbred, SAT, Buchberger, FES) are not offered.
  Comparing across scopes needs one common accounting first (plan step P4b).

Budgets are counts (`max_trials`, solver node and conflict budgets). A
wall-clock solver budget is never passed (`--solver-budget-seconds 0`), since
it makes a run depend on the host. `--timeout-seconds` and `--max-rss-mb` are
safety nets enforced from inside the trial process (`ps` is not always
available and macOS does not enforce `RLIMIT_AS`); a trial that hits one is
`exhausted` and is classed by the host, so treat it as a failure, not a
measurement.

## The study: `ic sweep run`

```
baselines ─► per trial:  ask (TPE) ─► rung 1 ─► rung 2 ─► … ─► project to the target ─► store
                                         │          │
                                   median prune  median prune
```

- **Rungs** are sizes (field degrees, or subgroup bits for prime curves).
  Each trial is evaluated on every rung, smallest first, by a separate child
  process, so a crash or hang costs one trial and is recorded as one.
- **The baseline** (`Space::baseline`, frozen before the search) is scored
  on every rung first. A rung it cannot be scored on is dropped; a study
  needs at least two. It is the repository's own incumbent, **not published
  index calculus**, which has no reference arm in the repository yet.
- **The objective** is `log2` of the worst-case-over-the-band ratio
  `S_trial / S_baseline`, **projected to `--target-log2r`** by a least-squares
  line over the rungs when there are at least `--min-fit` (default 4), else
  the last rung's measured value (`projected: false`). Below 0 beats the
  baseline at every weight. Optimising one toy size picks configurations that
  only work there; the projection also rewards a ratio that improves with
  size. It is an extrapolation and is labelled as one.
- **The sampler** is TPE, in-tree, conditional: a parameter that exists only
  for some choices (a solver's own knobs) is modelled from the trials in which
  it existed. Same seed and history give the same suggestion. With
  `--workers > 1` the order of finished trials, hence the history each
  suggestion sees, depends on timing.
- **Pruning** is the median rule: a trial stops after a rung if it is worse
  than the median of the other trials that reached that rung, once
  `--prune-startup` have.
- **The store** is an append-only JSONL file: a header (what must match for a
  resume), one `baseline` line per rung, one `trial` line per finished trial,
  flushed as it finishes. Re-running the same command resumes it and runs only
  the missing trials; a store written for different settings is refused. A
  line torn by a crash is dropped on the next open.

## Tracks

Owner decision, 2026-10-24: an index-calculus pipeline counts only when its
relations come from solving summation-polynomial systems over an
algebraically defined factor base, without table collisions.

| track | spaces | counted? |
|:--|:--|:--|
| `algebraic` | `koblitz-algebraic`, `char2-algebraic` (inherited F4, baseline inherited F4), `koblitz-algebraic-elim`, `char2-algebraic-elim` (`f4-f2`, `matrix-f4`, `matrix-f5`, baseline `f4-f2`) | yes, within a scope |
| `table` | `koblitz-table`, `prime-table` | no: reference only, never ranked against the algebraic track |
| `unclassified` | `prime-rootfind` (`rootfind-subtract`) | no: searched and reported on its own, never ranked against either other track |

The `rootfind-*` oracles are `unclassified` because whether solving the
univariate Semaev polynomial per base point counts as "solving a
summation-polynomial system" or as a root table is not decided by the
criterion as recorded. They are included at the owner's request, and stay
uncounted until the owner rules.

## Scale-relative knobs

A configuration tuned at one size must say something at the next. Base sizes
are `2^(θ·scale)`, the descent dimension is `δ·n`, and large-prime domains are
ratios, so the search space is the same on every rung. Absolute caps
(`MAX_BASE = 4096`) bound memory by construction.

## What it cannot do yet

- **Sizes.** `ic bench` stops at n ≤ 63 (binary) and p < 2^62 (prime), and
  at `m ≤ 3`. The a = 0, n = 83 target is reached only by projection from
  rungs below 63. Widening the framework is the next phase of the plan.
- **No algebraic prime space.** The framework has no algebraic prime oracle;
  `prime-table` is reference only.
- **`m ≥ 4`.** `descent-algebraic` is `m ∈ {2, 3}`.
- **Baselines.** Provisional, and not published index calculus. The algebraic
  baseline is inherited F4 on the S3 descent (`m = 2`) at `d = ⌊(n+1)/3⌋`: the
  `m = 3` arm finishes only at n = 13 through this framework. Where a baseline
  exhausts, that rung is dropped, and a study needs at least two.
- **Rho is context, not scored.** The record carries
  `w0_group_ops_over_rho_group_ops`: `S(0)` over the matched rho's mean `S`,
  group operations on both sides and native work on neither. It is a ratio of
  two partial counts, not a bound on either method.
- **Rungs.** `--min-fit 4` needs four scorable rungs. K_0 degrees that exist
  are 9, 13, 15, 19, 23, 31, 37, 39, 41, 45, 51, 53, 57, 59, 61 (K_1: 9, 11, 15,
  17, 19, 23, 29, 43, 45, 47, 51, 57, 59, 61). Measured with the `m = 2`
  descent at `d = ⌊(n+1)/3⌋`: n = 31 scores (`f4-f2` 169 s, `inherited-f4`
  262 s, eight probes running at once) and n = 41 does not (all four solvers
  hit a 300 s cap). So the F4-reachable K_0 rungs are 13, 15, 19, 23, 31, and
  they **mix cofactors**: log2 r is 11.0, 9.6, 17.0, 21.0 and 20.5. The fit uses
  log2 r on the x axis, so the mix is allowed but the points are not evenly
  spread (n = 23 and n = 31 have almost the same log2 r), which widens the
  projection's uncertainty. Use `--timeout-seconds 900` or more when n = 31 is
  a rung.
- **The projection amplifies noise.** The objective reads a fitted line at the
  target. With rungs spanning about 10 bits and a target 60 bits away, the
  rung-to-rung rounding of integer dimensions (and ordinary measurement noise)
  is multiplied roughly sixfold: in a synthetic test a configuration that beat
  the baseline at every measured rung scored worse than it at the target.
  Treat a projected objective as a hypothesis and read the per-rung ratios
  beside it; a confidence interval on the slope is future work.

## Files

| path | what |
|:--|:--|
| `src/bin/ic/sweep/trial.rs` | `ic sweep-trial`: configuration, count-based score, classes, safety nets |
| `src/bin/ic/sweep/space.rs` | typed, conditional, scale-relative spaces; tracks; baselines |
| `src/bin/ic/sweep/tpe.rs` | the sampler |
| `src/bin/ic/sweep/objective.rs` | worst-case ratio, fit, projection |
| `src/bin/ic/sweep/store.rs` | the append-only study file |
| `src/bin/ic/sweep/driver.rs` | the loop, pruning, the child-process evaluator |
| `src/bin/ic/sweep/mod.rs` | `ic sweep spaces / run / show` |

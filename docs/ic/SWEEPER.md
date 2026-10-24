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

No calibration ratio, host timing or solver wall pricing enters: the same
configuration scores identically on any host, which is what an optimiser
needs. *Native operations* are the counters `price_phase` converts, every
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
| `algebraic` | `koblitz-algebraic`, `char2-algebraic` | yes |
| `table` | `koblitz-table`, `prime-table` | no: reference only, never ranked against the algebraic track |

The `rootfind-*` oracles are in neither track and no space offers them: whether
solving the univariate Semaev polynomial per base point counts as "solving a
summation-polynomial system" or as a root table is not decided by the
criterion as recorded.

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
- **Baselines.** Provisional, and chosen so the F4 baseline runs on the
  rungs a study uses; where it exhausts, the rung is dropped.
- **Rho is context, not scored.** The record carries `w0_over_rho_lower_bound`
  (`S(0)` over the matched rho's mean `S`), a lower bound: rho's own native
  work is not band-priced here.

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

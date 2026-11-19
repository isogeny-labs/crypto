# R36 production review: initial-walker setup-table reuse

Status: second-pass source and deterministic-control review. This review does
not edit the production implementation and makes no wall-time or frontier
claim. Producer observations are `[pcc:unverified]`.

Reviewed source digests:

- `strong_rho.rs`: `439489bd8933b78c1d2fbcfd821d92728ade3f883c309bdd2dd9f566368e504c`
- `main.rs`: `20e7e2bc9e80a62867583f17159b982c4c6d5dac63b67b332373b01596956c33`
- compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`

The exact command summaries and selected raw records are retained in
`receipts/r36_production_review.txt`.

## Review disposition

The selectable production R36 path preserves the accepted initial walkers,
the post-setup native counters, the subsequent walk, and the recovered scalar
in the deterministic controls executed here `[pcc:unverified]`. Its default is
off, so the unsuffixed `strong-bW` and `fast-bW` arms keep the previous
Karatsuba-six and jump-only setup path.

The implementation remains a candidate rather than a promoted performance
change. It creates several temporary vectors and executes one-element batch
inversions inside every joint-multiplication layer. Its counted setup record
changes, but isolated cold wall time has not been admitted or measured.

## Static audit

### RNG draw order and lane order

`new_walker` and `new_walker_with_table` use the same two expressions, in the
same order:

```text
a = rng.below(n)
b = rng.below(n - 1) + 1
```

Both add two to `NativeCounts::rng_draws` before constructing the candidate
point. `initial_state` constructs walkers through `(0..batch).map(...)`, which
is evaluated sequentially. The table path therefore does not draw all lanes
ahead of a retry or change which lane receives the next coefficient pair.

### Retry, discard, and infinity behavior

Both constructors loop until the computed point is not infinity. The table
path calls `batch_window2_mul` inside the loop, and its field counters are
cumulative; no counter or RNG state is rolled back when the point is rejected.
Canonicalization and `Walker` initialization occur only after acceptance in
both paths.

The q=7 sweep exercised this branch in production code: seeds 10, 21, 27, and
35 each consumed 82 setup RNG draws rather than the no-retry 80. Baseline,
table-reuse, and table-reuse-plus-mul5 arms retained equal accepted behavior
and downstream trajectories in those cells `[pcc:unverified]`.

### Group-law equivalence

The joint evaluator uses the same width-two table as the jump setup and a
base-4 Horner evaluation. The source self-test compares its output to
independent `ec.mul(G,a) + ec.mul(Q,b)` computations, including zero and
boundary coefficients. The q=223 setup test compares every jump and every
accepted walker between legacy, jump-batched, and walker-table paths.

### Default-arm compatibility

`StrongOpts::for_instance` sets both `reuse_setup_table_for_walkers` and
`use_mul5` to false. CLI parsing changes them only for the explicit
`-walker-table`, `-mul5`, or `-mul5-walker-table` suffixes. The self-test's
frozen default-arm oracle still passes at widths 1, 4, 8, 16, and 32
`[pcc:unverified]`.

### Composition with mul5

The R36 evaluator receives the `FastF3` selected for the entire run, so the
combined suffix exercises mul5 in table construction and every Horner layer.
Across the deterministic matrix, the combined arm retained the same
trajectory/native records as the Karatsuba table arm. For each phase, the
source self-test checks equal `f3_mul`, one fewer `fq_mul` per `f3_mul`, and
four `fq_small_div` operations per mul5 call `[pcc:unverified]`.

### Scope of reuse

The table is local to `initial_state`. Walk-time restarts still call the
ordinary `new_walker`, because the table is not retained after setup. Thus R36
optimizes initial walker construction only. The emitted setup-mode string,
`batched-window2-jumps-walkers`, is accurate, but reports should state
“initial walkers” explicitly so the arm is not read as covering restart
walkers.

## Deterministic controls

1. The complete standalone self-test passed, including the frozen field,
   joint-table, setup-state, five-width counted/counter-free, full-trajectory,
   mul5, and combined controls `[pcc:unverified]`.
2. A q=223 matrix covered 15 `(seed,width)` cells: seeds 1–3 and widths
   1/4/8/16/32. Baseline, table, mul5, combined, and counter-free-combined arms
   matched on recovery and trajectory/native fields; no setup retry occurred
   in those cells `[pcc:unverified]`.
3. A q=7 matrix covered seeds 1–40 at width 8. The verifier required equal
   recovery, trajectory, setup-native, walk-native, RNG, and mul5 identities.
   Four cells exercised one setup retry and the complete matrix passed
   `[pcc:unverified]`.
4. In q=7 seed 10, all three counted arms used 82 setup RNG draws, recovered
   scalar 31, and retained 18 steps, three DPs, zero walk restarts, two
   look-ahead retries, and zero detected cycles `[pcc:unverified]`.

Development `wall_s` fields were ignored. The q=7 sweep is a small-domain
correctness control, not a measurement substitute.

## Issues and remaining gates

### 1. Attempt/discard evidence is implicit

The production record has no `setup_attempts`, `setup_discards`, or post-setup
RNG-state digest. A reviewer can infer one retry in the current default
configuration from `(setup_native.rng - 2*r - 2*batch)/2`, but `r` is not
emitted and the inference depends on the present draw contract.

Required follow-up: add explicit attempt/discard fields and a post-setup RNG
digest, then make the analyzer reject inconsistent records. This is an
evidence-schema gap; the reviewed source does retain the rejected attempt's
work.

### 2. The in-source regression does not force an infinity retry

The production self-test compares two seeded initial states, but its current
seed consumes the no-retry draw count. The q=7 external deterministic sweep
exercised retries, yet that protection is not part of the source's permanent
self-test.

Required follow-up: inject a test-only scripted coefficient source or bind a
small deterministic seed known to force infinity, and assert coefficient
order, attempts, discards, counters, accepted point, and RNG state.

### 3. `StrongOpts` permits an incoherent programmatic combination

A non-CLI caller may set `reuse_setup_table_for_walkers=true` while setting
`batch_setup=false`. `initial_state` returns through the legacy branch before
consulting the reuse flag, silently ignoring the request. The current CLI
does not construct this combination.

Required follow-up: replace the two booleans with a setup-mode enum or assert
that reuse implies batched setup. This is a fail-closed API issue, not a wrong
result in the reviewed CLI arms.

### 4. Count reduction does not establish wall improvement

For each walker attempt, `batch_window2_mul` allocates output/lane/point/addend
vectors, and `batch_add_complete` allocates additional denominator and inverse
vectors. The single-coefficient path also raises the logical `batch_inv`
count substantially even while lowering base-field inversions in the tested
records.

Required follow-up: reusable scratch storage and a source-bound profile may be
screened, but promotion still requires the frozen checkpoint, isolation,
calibration, A/A, ABAB, holdout, replay, validation, and PCC gates.

## Conclusion

No production correctness defect was found in the reviewed CLI combinations
or deterministic cells `[pcc:unverified]`. R36 is implemented as an explicit,
default-off initial-setup candidate. Its open work is permanent forced-retry
coverage, explicit retry evidence, fail-closed option representation, scratch
reuse, and admitted performance measurement.

# Amendment 3: incremental residual sampling

Frozen after the first local pilot and before any sampling-variant observations.
That pilot is retained at `local-p67-all5-20261115-02/`, source `3188372cc`.
Its symmetric pair-only channel took 36.71 times local rho wall time; its
incomplete legacy model has group-operation share 95.53%. The source performs
two fresh scalar multiplications for each residual. [pcc:unverified]

## Hypothesis and implementation boundary

A 32-entry r-adding sampler can maintain `R=aG+bQ` with one group addition
per steady-state step, paying its jump-table/start construction and all
cycle/epoch restart work in full. The
algebraic finder, target projection, factor base, elimination and scalar
replay stay in the existing native IC path. Do not add a rho collision table
or use rho fallback recovery to claim an IC result. [pcc:unverified]

Preserve the historical fresh-scalar RNG stream and existing default API.
Expose a separate sampler selection. Verify the walk invariant by direct
point multiplication, match graph/pivot streams within each sampler, and
pin an actual fresh-stream fingerprint from the first pilot as a regression.
Record jump/start setup group operations and per-residual sampling group
operations separately, without resetting them out of total cost.

## 3a: pre-measurement cycle correction

The first end-to-end sampler regression failed on the unchanged p7/seed3
fixture: a deterministic trajectory yielded no decompositions in 20000
residuals. The unbounded functional walk can cycle without visiting a smooth
point. Retain that failed test output. No sampler measurements were taken.
[pcc:unverified]

The revised sampler stores point IDs only to avoid revisiting a point within
an epoch, and restarts at repeats or after `min(ceil(sqrt(n)),1024)` points
(minimum two). It stores no previous scalar coefficients and does not recover
the logarithm from a collision. Uniform restart scalar multiplications,
discarded step additions and table work stay in the measured interval, and
restart counts are explicit. This changes the prototype, not the test target,
seed, residual cap or scientific acceptance criteria. [pcc:unverified]

## Matched successor experiments

- Two create-new local runs on the same published native source/binary:
  fresh first, r-adding second. These are distinct predeclared treatments,
  not retries of an outcome or control failure.
- `p=67`, seeds `3,11`; seed 11 is fresh relative to the first pilot.
- Oracles: symmetric pair-only and symmetric hybrid triple. Other oracles
  are not resubmitted because this hypothesis targets sampling costs.
- Same graph/pivot/rho A/A arms, one retained warmup, five measured rounds,
  maximum 10000 residuals, 900-second bound per run, 8 GiB, one pinned CPU,
  zero automatic retries and unchanged local analysis criteria.
- Matched graph/pivot fingerprints are required within an oracle/sampler.
  Fresh and walk treatments use different residual streams; do not claim
  cross-sampler input-prefix equality. Compare end-to-end costs and retained
  successes/failures on the same target fixtures.
- Preserve every failure/cap. No small-instance result triggers the
  large-order reference-parity flag; after this component check, freeze
  larger fresh holdouts if the candidate remains viable.

Run/analysis code remains Rust, with thin shell capture. Local checkpoint and
published source/spec follow the user's local-backend authorization.

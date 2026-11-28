# Amendment 1: symmetric S3 relation finding and matched rho reference

Frozen before any performance observations. The original `protocol.json`
remains the record of the first pivot/forest plan; this amendment describes
the next native implementation and its expanded measurement workload.

The user requested continuation toward rho parity and explicitly required Rust.
Implementation, executable comparison, correctness checks and any new research
analysis use Rust. No Python research program or timing result is introduced.

## Algebraic candidate

Set `s=x1+x2`, `t=x1*x2`, and write `x=x(Y)`. The existing S3 identity becomes

```text
t² + (-4x²-2a-2xs)t + (a²-4bx) + (-2ax-4b)s + x²s² = 0.
```

The two nonconstant extension components are `uk(s)*t+vk(s)=0`, where
`deg(uk)<=1` and `deg(vk)<=2`. Elimination gives `u1*v2-u2*v1`, of degree
at most three. Before extracting roots, intersect it with
`v²-u*u0*v+u²*v0`, the numerator of the constant equation after substitution.
For each sum, recover products using either nonzero denominator. When both
vanish, intersect the remaining components and solve the constant quadratic.
Dependent component polynomials use an exact factor-base scan whose group work
stays charged. Split `X²-sX+t`, check base membership, then settle signs by
curve arithmetic. The filtered release run reports 30 passing tests, including
independent tiny-group/signed-pair comparisons and both new integrated solver
variants. These are producer-run correctness checks, not a performance or
novelty claim. [pcc:unverified]

Expose `Solver::SymmetricPairOnly` and `Solver::SymmetricMeetInTheMiddle` in
the existing cubic IC collector, usable with either protected pivot or graph
elimination. Keep existing solver defaults. Record component construction,
polynomial multiplications, fallback scans and zero-denominator events.
Scalar inversions and other omitted native work remain explicitly unpriced.

This changes the pair solver's finite cost, not its relation probability or
exponent. The existing section 11.9 analysis of the cubic pair-only channel
still applies. The small frozen cohort is a feasibility/controls cohort;
an admitted result there alone cannot establish a scaling crossover. Larger
held-out subgroup sizes and the fastest matched reference remain part of the
continuing rho-parity objective. [pcc:unverified]

## Measurement amendment

- Same exploratory primes/seeds and holdout as the original protocol.
- Five oracle blocks: the three original engines plus both symmetric variants.
- Each block interleaves `pivot_a`, `graph`, `pivot_b`, `rho_a`, `rho_b`, with
  reversed order on odd rounds. Every rho arm calls the existing `run_rho3`
  on the identical curve, generator and target; it pays its own setup/replay.
- Retain one warmup round per block, tagged `phase=warmup`, followed by five
  `phase=measurement` rounds. Warmup rows are retained and never enter measured
  aggregates. Failed/capped executions remain in the raw output and invalidate
  their whole block.
- Raw schema is `gaudry.cubic.graph-comparison/v2`, with execution and separately
  labelled block-diagnostic rows. Files are create-new and flush each record.
- Require graph/pivot projected-input fingerprints to match within an oracle
  block and rho A/A work/scalars to match. Different oracle engines can consume
  different root-finding randomness, so cross-oracle streams are not claimed
  to be identical; exact oracle comparisons use fixed target correctness tests.
- Ratios from the incomplete legacy cost model and unadmitted clocks remain
  diagnostics. The rho reference is the existing cubic r-adding implementation;
  fastest external rho comparison is still an additional acceptance condition.
  No parity verdict is emitted by the driver.
- Keep the original USD 5 total ceiling, four CPUs, 8 GiB, 900-second deadline
  and zero automatic retries. Do not enlarge a run or change its frozen inputs
  silently if it is capped; record the cap and freeze a successor separately.
- Checkpoint, published source/image, pool, native L2 and A/A admission,
  write-once benchmark record and durable evidence requirements still apply.

## Sources and evidence status

KB unavailable: repository wrapper has no executable and no KB MCP is loaded.
Sources read: `gaudry_cubic.rs` S3 expansion/polynomial engine/rho reference,
`RESEARCH_RESIDUAL_WALKS.md` sections 11.1, 11.4, 11.6, 11.7, and the earlier
graph-integration sources. Primary retrieval of Semaev 2004/031 and Gaudry's
small-dimension paper returned HTTP 403 in this continuation. The derivation
above is checked against the explicit existing S3 identity, with finite
independent signed-pair enumerations; no absence or novelty inference follows.

No performance run has launched. Existing missing checkpoint transport and
tested command-image/runner access remain recorded execution dependencies.

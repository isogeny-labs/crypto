# Weighted decompositions on larger cubic-field curves

Status: frozen before the native measurements in this directory.

The requested deliverables are a tracking PR and experiments on larger curves.
This study extends the conversation's small Python diagnostics using the
repository's existing native cubic-field arithmetic and algebraic S3 pair solver.
The historical Python figures are not native baselines and establish no native
runtime comparison.

## Hypothesis and matched reference

For a fixed factor base B, using R = B1 + B2 + [c]B3, c in {2,4}, increases
verified decomposition yield relative to c=1 without increasing logarithm
unknowns. The local reference is the identical hybrid pair solver with c=1,
not a claim that this is the repository's best complete IC pipeline.
An A/A arm repeats c=1 with identical inputs and RNG state. Both repetitions
must return identical operation counters and decisions.

Use `generate_instance3(p, seed)`, prime-order curves over F_p[t]/(t^3-c),
and `SubspaceBase::build`, with canonical point signs and x in F_p. This is
a new native, matched suite; it does not silently reuse the Python affine-line
models or their field moduli.

## Frozen workloads and limits

- Primary base primes: 31,61,127,241; further stage sizes: 499,1009.
- Construction seeds: 20261019 and 936451 (the latter is a separate holdout).
- Coefficients: 1,1 (A/A),2,4, in that fixed order.
- 128 independently sampled, identical public synthetic PDP targets per curve.
- One known-answer DLP per arm and curve up to p=241, with no setup shared
  between targets as a claimed amortized saving. The solver receives no secret.
- Recovery stop: 10,000 residuals or 5,000,000 pair tests, whichever comes first.
- Every returned decomposition is checked by group arithmetic. Every PDP
  decision is cross-checked against an independent signed-pair dictionary,
  unavailable to the algebraic solver. The reference setup and checks are
  recorded separately, never smuggled into an algorithmic cost claim.
- Exhaustive tests on small native instances check every group target, including
  identity and repeated factors. Failures and budget exhaustion remain evidence.
- The output directory must not exist. No frozen run is overwritten.

## Accounting and admissible conclusions

Record separate setup, residual generation, algebraic solving, relation
verification, modular elimination and recovery-check counters. Pair calls,
legacy F_p-multiplication counters, group operations and matrix operations
are different units and are never silently added.

These are stage diagnostics and verified synthetic recoveries. Complete S,
IC/rho ratio and end-to-end speedup remain null: the legacy solver API does
not emit all native/unpriced counters, there is no pinned conversion for every
phase, and a matched ecbench rho/independent-host comparison is not part of
this initial port. The applicable generic reference is a matched one-target
rho run; its measurement and the 0/0.1/1 pricing band remain open obligations.
No ranking against rho, exponent change, or external-tool speed claim follows.
Wall-clock figures are not admitted at L0 and are not a headline result.

Success for this bounded diagnostic means equal A/A decisions and counters,
zero incorrect relations or recovered scalars, zero reference disagreements,
and a reported yield/counter comparison whether positive, null or negative.
Coverage gains alone do not establish an end-to-end improvement. Binary fields,
ECC2K-130 transfer, optimized large-prime collection, and cryptographic-size
recoveries are outside the measured scope.

## Reproduction and evidence

The example's `--help` gives the exact native command. Commit the protocol
before measurement, retain the source revision and hashes, host information,
per-curve parameters and ICV1 identities, raw records, verification receipts,
and a source-linked report with graphs and a PDF. Preserve unsuccessful builds
and experiments. Missing external references (Magma/msolve/FGb) are named as
missing, never replaced by a claim of fastest performance.

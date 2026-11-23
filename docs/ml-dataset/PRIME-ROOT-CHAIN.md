# Prime-field affine S3 prefix/root adapter

2026-11-02. This source-only increment extends `crypto dataset`'s `root`
solver from m=2 to m=3..15 for prime curves and their registered twists.
The existing m=2 certificate and route remain separate. No cloud generation,
timing comparison, or training result is included.

## Sources and scope

- The repository's [prime F4 chain contract](PRIME-F4-CHAIN-CONTRACT.md) fixes
  the odd-characteristic S3 formula and documents the affine-infinity caveat.
- Semaev's [2015 primary paper, sections 2-4](https://arxiv.org/html/1504.01175)
  develops summation polynomials and chained decomposition. Its fall-degree
  assumptions are not adopted as labels here.
- Gaudry's [ECC 2004 slides](https://cacr.uwaterloo.ca/conferences/2004/ecc2004/gaudry.pdf)
  distinguish decomposition, lifting, and linear algebra. This adapter only
  measures the first two at a bounded prefix/root scope.

For every ordered signed factor-base prefix of length m-1, the adapter sums the
actual curve points and calls the native counted prime root finder on
`S3(prefix.x, target.x, final.x)`. The final x is checked against the original
polynomial, then any candidate is lifted against the signed factor base and
replayed in the curve group. It enumerates prefixes lexicographically, never
reads a private scalar, and stops at the global work/event/byte limits. This
is a prefix-search strategy, not F4 elimination of a full m-leaf system.

`prime-root-certificates.json` records every completed query's signed point
prefix, S3 coefficients, native root report/events, lift checks, work prefix,
and terminal state. The existing root-call and selected root-state artifacts
are retained. Replay reconstructs the public domain, prefix sum, polynomial,
native root certificate, bounded lift, and aggregate cost. A prefix whose sum
is infinity stops with explicit censoring because the affine S3 branch does
not describe it. Its outcome is `partial` with
`root_termination_cause=affine_infinity_prefix`, distinct from a work or trace
cap's `budget_exhausted` outcome. An exhausted affine search is only an encoding-relative
observation; it is not a proof of a global absence of decomposition, ECDLP
recovery, or a first/last fall-degree result.

The charged work units are prefix group additions, the fixed 15-operation S3
coefficient recipe, counted native field work, original-polynomial evaluation,
and bounded group-lift additions. Serialization, replay, and machine
instructions are outside this counter. A native root report certifies its
distinct roots at its own completion scope; retained events do not constitute
a restorable process checkpoint.

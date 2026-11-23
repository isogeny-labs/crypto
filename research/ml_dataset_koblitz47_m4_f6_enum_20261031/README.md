# Koblitz n47 m4 F6 and enumeration campaign

This frozen population is 16 native dataset cells: one Koblitz curve seed at
`n=47, m=4`, two generator indices, two target-key indices per generator,
both quadratic twists, and two methods (`f6`, `enumerate`). Within each
generator/key/twist triple, the methods share the same public input. Native
mixed-radix index order is `solver + 2*twist + 4*key + 8*generator`, with F6
digit 0 and enumeration digit 1. All 16 outcomes, including construction
failures, resource stops, unsupported results, and timeouts, remain in the
denominator without replacement.

`f6` is the native geometric four-sum pair index over a bounded signed finite
domain, not a general Gröbner F6 implementation. `enumerate` records visited
PDP transitions under a finite work limit. A bounded negative is not an
exhaustive no-decomposition proof; an ECDLP scalar label requires independent
native group replay. The odd degree 47 Koblitz model and its trace-one twist
remain defined over F2, but relation/orbit reductions require their own
physical transport, deduplication, rank and recovery evidence.

The protocol and exact plan will be committed before any scientific attempt.
Each cell will use a 30-second native limit, 90-second outer runner limit,
2 GiB memory, 64 MiB trace, and an 8 GiB XFS free-space floor. The controller
must retain one-attempt markers, native verify/report/strict-export results,
restricted archives, content-addressed S3 PUTs and fresh GET/hash receipts.
This campaign extends size/arity/method coverage; it does not replace the
requested full matrix.

The completed run is documented in [report/run.md](report/run.md), with an
[outcome chart](report/outcomes.svg) and [PDF](report/run.pdf). The
[independent audit](AUDIT.json) rehashes all 16 archives and their native
artifacts. [Cost observations](cost-observations.jsonl) preserve each phase's
raw PMU readings and eligibility flags without promoting partial instruction
counts to whole-worker labels.

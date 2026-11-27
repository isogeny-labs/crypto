# Larger public-fixture full-output protocol

2026-11-24. Local software and output qualification, with explicit incomplete
outcomes. This is not an admitted timing or scaling comparison.

## Fixed inputs and unchanged recovery implementation

Use the existing F8191 public fixture y²=x³+x+5, subgroup order 8053,
G=(0,789), Q=(3197,5073), seed 781815. Two suites are planned, at selector
depth 4 (degree 16) and depth 5 (degree 32), in that order. Each suite runs
Direct, Refined and ProductionCofactorMsolve, all with common `base-lookup`
and `full-output`, and a maximum of 30 draws. Stop at the first consumer scalar
or the existing fatal condition; an unrecovered 30-draw outcome is retained.
No extra seed, target, retry, extension or larger budget is authorized by this
protocol. A failed depth-four suite does not omit the planned depth-five suite.

The recovery executable is the already frozen source **77c465a6d1** binary:

```text
82de0db7d03119b3b2eeb1091a87775b126357c0be827274866c6f2438b75e88
```

No recovery algorithm, solver cap, output rule or accounting boundary changes.
The [full-output protocol](FULL_OUTPUT.md) and [cold-accounting boundaries](COLD_ACCOUNTING.md)
apply. Every oracle answer is consumed in order through recovery; all its
generated rows and the consumed prefix are retained. Identity/duplicate draws,
singleton hits, incomplete queries and dependent rows remain explicit.

Bounds: one worker and 2 GiB virtual memory; three seconds per query; 120 seconds
per child; 400 seconds plus five-second kill grace per suite. Records remain at
most 1 MiB and child stdout/stderr 32 MiB. No automatic retry. A fatal child
explicitly lists unrun arms. Parent polling can add 5 ms and OS caches are not
flushed. All raw timers remain development diagnostics.

## Separate post-run whole-output replay

Add a Rust `replay-account PATH` mode, with a separate executable identity,
qualified only on the two public fixtures at depths 4/5. It reads format 3
records with the same 1 MiB record bound and a 128 MiB file bound. It checks
the parent/child identities and accounting, regenerates public masks/residuals,
checks skips and singleton decisions, and reconstructs a fresh consumer.

For every completed query, enumerate the existing group-law reference set
and compare the **entire** returned set, including any unconsumed suffix.
For Refined, restrict that reference to four distinct coarse Dickson images;
Direct/cofactor use their original distinct-x, target-disjoint domain. Then
replay each selected relation and compare its canonical row, disposition,
rank, recovered scalar, arithmetic/work totals and final multiplication.
No reference answer is supplied to any algebraic extractor. Failed queries
remain incomplete: the audit does not fill in missing outputs.

This cross-check is separate from the algebraic solver but shares existing
fixture, reference and consumer source. It is not independent implementation
or signed validation. Its separate post-run work is outside the recorded
recovery intervals and supplies no timing advantage.

Before data collection, native tests replay both retained F127 full-output
suites and reject a structurally consistent lost suffix row, altered masks,
residuals, consumer receipts/counters and process success. A failed-oracle
fixture checks that identity/duplicate skips and incomplete recovery survive.
Preserve any failed tests and corrected versions.

After freezing/pushing auditor source, this protocol and exact commands:

1. Audit each of the two retained F127 full-output files once, with no new solve.
2. Run both new F8191 suites once, independently retaining stdout/stderr/exit.
3. Audit each new suite file once, even if a suite failed, retaining rejection
   or acceptance without reclassifying the raw recovery.

A failed historical F127 audit blocks new recovery collection and explicitly
records the four unrun recovery/audit processes. This is a correctness gate,
not an automatic retry or a substituted fixture.

Each of the four audit processes gets 60 seconds plus five-second grace and
the same worker/memory bound. Maximum outer allocation is 1,070 seconds:
two 405-second recovery allocations plus four 65-second audits. The command
refuses existing output destinations. The stream captures newly emitted JSON
only; it does not retransmit the historical F127 inputs as new observations.

## Evidence and decision rule

Report each arm's draws, complete/incomplete calls, generated/consumed rows,
rank, recovery status and audit coverage. Keep empty exact answers distinct
from solver failures and unrecovered endpoints. Do not treat exit zero or a
passed replay of an incomplete arm as recovered. Any complete-set mismatch,
bad group equality, changed mask or unreconciled receipt rejects that audit;
retain its original input and diagnostics before proposing a correction.

Sources checked: the [earlier F8191 refinement](REFINEMENT_RESULT.md),
[cofactor controls](COFACTOR_OUTPUT_RESULT.md), Base.reference/Base.separated,
and production relation_adapter.rs. The prior PKM/map/isogeny literature scope
is unchanged. KB unavailable: the whole-set replay query exits 127 and no KB
MCP tool is loaded. No novelty or broader absence conclusion follows.

The original requirement for a new supported mechanism and reproducible gain
including setup, collection and linear algebra remains open. Refined retains
its narrower chart; cofactor degree-32 and full-base oriented completion remain
separate obligations. Registered EKS payload/build and independent admission
are still unavailable. Local flow receipts use the development schema because
canonical experiment-export tooling was unreadable through the available
repository routes. No cloud dispatch or paid run is part of this protocol.

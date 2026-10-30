#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU progressive batching: mixed result
Shared-host diagnostic only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round10-20261030`, October 30, 2026 UTC.

Original optimized baseline u/x: `2bd4588fba60`. Progressive candidate v:
`ca37cf07f271`; batch1 control w. Added previous fixed-batch candidate y:
`abb2795c95cb`. All three fixtures and all earlier controls remain present.
AMD EPYC9R45, one software thread, 2 vCPUs / 4 GiB reserved, shared EKS VM (L0).

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Fixed y ms], [Progressive v ms], [y/v],
 [8-bit / 3 slots], [0.285852], [0.129360], [0.122401], [1.057],
 [16-bit / 3 slots], [3.973266], [0.664603], [0.681493], [0.975],
 [16-bit / 4 slots], [4.169826], [0.809593], [0.817353], [0.991])
Medians from nine measured repetitions after one warmup. Progressive batching
lowers the small-fixture median but raises both larger-fixture medians here.
Fixed batching is restored as the default. The further 10x target remains unmet.

#image("../../round10/results/latency.svg", width: 90%)

All 150 executions completed (135 measured, 15 warmups). Full certificate bytes
agree across arms; scalar checks, replay and corrupted-scalar rejection pass.
The complete interval charges setup, discarded trials, relation collection,
elimination, both certificate checks and serialization. Process launch, archive
I/O and negative controls are excluded. No checks were removed.

The initial export retained four-arm flow counts and an old analyzer reference;
the checker rejected both. It remains preserved as `results/export.json`.
`results/export-corrected.json` changes only flow counts and analyzer reference,
and passes file-verifying export checks. Raw data and frozen protocol are unchanged.
Earlier rounds6–9 carry separate additive provenance corrections for stale
candidate descriptions. These deviations are not erased by checker passes.

All 29 release tests pass after restoring fixed batching. This is a bounded
engineering diagnostic; strict isolation, independent review, matched external/
rho comparisons and larger-scale recovery remain unresolved.
Protocol: `179eefd5406a982ef9ae2e823260e8d303afe04d935501fc15a2b13ba89e4823`.
Raw rows, build identities and certificates: `round10/results/`.

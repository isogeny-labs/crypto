#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU density-aware query scheduling
Shared-host diagnostics only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round11-20261030`, October 30, 2026 UTC.

Original baseline u/x: `2bd4588fba60`. Candidate v: `d0ad5a617f59`; batch1 w.
Fixed-batch reference y: `abb2795c95cb`. One software thread, AMD EPYC9R45,
2 vCPUs / 4 GiB reserved, shared EKS VM (L0). All earlier controls remain present.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Fixed y ms], [Candidate v ms], [y/v],
 [8-bit / 3 slots], [0.287191], [0.131490], [0.120901], [1.088],
 [16-bit / 3 slots], [3.997944], [0.668873], [0.667152], [1.003],
 [16-bit / 4 slots], [4.192356], [0.814393], [0.813343], [1.001])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.375, 5.993, 5.154. The further 10x target remains unmet.

#image("../../round11/results/latency.svg", width: 90%)

The preselected rule uses progressive batches only when distinct suffix sums
cover at least 1/8 of the group. It is a scheduling heuristic, not an early-match
guarantee. Prefix order, exact first witnesses and exhaustive negative queries
are unchanged. Duplicate tuple sums cannot inflate the unique-key count.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. The complete interval
charges setup, discarded trials, elimination, both checks and serialization.
Process launch, archive I/O and negative controls are excluded.

Small-fixture ranges: v 0.120111–0.125070 ms, y 0.128741–0.133461 ms. The larger
fixtures have overlapping ranges and near-equal medians; neither improvement
nor equivalence is certified. Retain this candidate as a diagnostic engineering
choice. All 29 release tests and strict Clippy pass.

The export passes file-integrity/completeness checks with five-arm counts and
its frozen analyzer identity. Earlier metadata/provenance corrections remain
preserved in their own panels. Strict isolation, independent review, matched
external/rho comparisons and larger-scale recovery remain unresolved.
Protocol: `b8707ca8ac830868131ffa10390cb6273ae1a8d23d50b86e4cea59d2230e1abf`.
Raw rows, certificates, build identity and checked export: `round11/results/`.

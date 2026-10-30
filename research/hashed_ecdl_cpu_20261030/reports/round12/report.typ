#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU checked projective witness sums
Shared-host diagnostics only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round12-20261030`, October 30, 2026 UTC.

Original baseline u/x: `2bd4588fba60`. Candidate v: `308bf1a6a5bc`; batch1 w.
Previous candidate y: `d0ad5a617f59`. One software thread, AMD EPYC9R45,
2 vCPUs / 4 GiB reserved, shared EKS VM (L0). All earlier controls remain present.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Previous y ms], [Candidate v ms], [y/v],
 [8-bit / 3 slots], [0.286241], [0.121970], [0.119681], [1.019],
 [16-bit / 3 slots], [3.985123], [0.666682], [0.657712], [1.014],
 [16-bit / 4 slots], [4.174184], [0.809032], [0.796733], [1.015])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.392, 6.059, 5.239. The further 10x target remains unmet.

#image("../../round12/results/latency.svg", width: 90%)

Every oracle witness point is validated before projective accumulation, followed
by one affine conversion and exact comparison with the target. Solver relation
checks and both independent full certificate replays remain unchanged.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. The complete interval
charges setup, discarded trials, elimination, both checks and serialization.
Process launch, archive I/O and negative controls are excluded.

Small and middle fixture ranges overlap. Largest fixture ranges: v
0.794022–0.802362 ms; y 0.806052–0.814513 ms. No improvement or equivalence is
certified. Retain the candidate as a diagnostic engineering choice. All 31
release tests and strict Clippy pass. Ratios are derived from measured medians.

The export passes file-integrity/completeness checks with five-arm counts and
its frozen analyzer identity. Earlier metadata/provenance corrections remain
preserved in their own panels. Strict isolation, independent review, matched
external/rho comparisons and larger-scale recovery remain unresolved.
Protocol: `b941a51cfd8a3e48448c06a584c30564dd4d16b3d84c71215dc18d9511a686b5`.
Raw rows, certificates, build identity and checked export: `round12/results/`.

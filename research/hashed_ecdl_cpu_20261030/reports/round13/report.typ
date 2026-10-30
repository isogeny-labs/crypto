#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU direct projective witness comparison
Shared-host diagnostics only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round13-20261030`, October 30, 2026 UTC.

Original baseline u/x: `2bd4588fba60`. Candidate v: `92463171363c`; batch1 w.
Previous candidate y: `308bf1a6a5bc`. One software thread, AMD EPYC9R45,
2 vCPUs / 4 GiB reserved, shared EKS VM (L0). All earlier controls remain present.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Previous y ms], [Candidate v ms], [y/v],
 [8-bit / 3 slots], [0.287681], [0.122430], [0.119581], [1.024],
 [16-bit / 3 slots], [3.972973], [0.659942], [0.657142], [1.004],
 [16-bit / 4 slots], [4.180033], [0.800253], [0.792513], [1.010])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.406, 6.046, 5.274. The further 10x target remains unmet.

#image("../../round13/results/latency.svg", width: 90%)

Every oracle witness point and target is validated. The projective sum is
compared by cross multiplication, avoiding its final inversion. Solver relation
checks and both independent full certificate replays remain unchanged.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. The complete interval
charges setup, discarded trials, elimination, both checks and serialization.
Process launch, archive I/O and negative controls are excluded.

All candidate/previous ranges overlap. Candidate v ranges (ms): 0.118280–0.125210,
0.652072–0.668372, 0.790512–0.799413; y: 0.120040–0.132980, 0.658352–0.671782,
0.795922–0.806052. Retain the removed inversion and exact checks; these timings
do not certify improvement. All 31 release tests and strict Clippy pass.
Ratios are derived from measured medians.

The export passes file-integrity/completeness checks with five-arm counts and
its frozen analyzer identity. Earlier metadata/provenance corrections remain
preserved in their own panels. Strict isolation, independent review, matched
external/rho comparisons and larger-scale recovery remain unresolved.
Protocol: `cf4d7243010c43da6325e869fd0db71f57b7016d67ebea44ee8906b64284e066`.
Raw rows, certificates, build identity and checked export: `round13/results/`.

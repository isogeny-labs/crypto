#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= CPU optional-profiling snapshot
Shared-host observations only; no certified speedup or hypothesis promotion.
Job `hashed-ecdl-cpu-round14-20261030`, October 30, 2026 UTC.

*Unapplied snapshot:* repository `fb3cb55f518a` plus frozen optional-profile patch.
Original baseline u/x: `2bd4588fba60`; previous integrated y: `92463171363c`.
Snapshot v; batch1 control w. Intel Xeon 6975P-C, one software thread,
2 vCPUs / 4 GiB reserved. All comparisons below are within this run.

#table(columns: (1.3fr,1fr,1fr,1fr,0.7fr), inset: 4pt,
 [Fixture], [Baseline u ms], [Previous y ms], [Snapshot v ms], [y/v],
 [8-bit / 3 slots], [0.389109], [0.152397], [0.146386], [1.041],
 [16-bit / 3 slots], [5.784662], [0.855458], [0.830132], [1.031],
 [16-bit / 4 slots], [6.036159], [1.057224], [1.027651], [1.029])
Medians from nine measured repetitions after one warmup. Baseline u/v ratios:
2.658, 6.968, 5.874. The further 10x target remains unmet.

#image("../../round14/results/latency.svg", width: 90%)

The ordinary solver skips unused internal stage clocks. The profiled API remains
available. All algorithm phases, external timers, serialization and both full
certificate checks remain intact. This patch is not integrated; target reuse is absent.

All 150 executions completed (135 measured,15 warmups). Full certificate bytes,
scalar checks, replay and corrupted-scalar rejection pass. Setup and discarded
trials remain charged. Launch, archive I/O and negative controls are excluded.

Small ranges: v 0.141847–0.151094ms, y 0.151507–0.158814ms. Larger ranges overlap:
v 0.815642–0.848126 vs y 0.846418–0.873087ms; v 1.012017–1.054663 vs y
1.034722–1.079621ms. No certified improvement. Do not pool with prior AMD runs.
A/A x medians: 0.389082,5.756197,6.044363ms. Ratios are derived from medians.

The32-test snapshot passes strict Clippy and formatting. Export completeness and
file-integrity checks pass. Effective source and patch hashes are pinned in
`round14/candidate-materialization.json`; its returned copy matches exactly.
Integration, strict isolation, independent review, external/rho comparisons and
larger-scale CPU recovery remain unresolved. Build-path layout is a nuisance.
Protocol: `edcbf4bfef6eb1d14ea498655b0b3eb456e551fab64cb9c886f2f3623359ba9b`.
Raw rows, certificates and checked export: `round14/results/`.

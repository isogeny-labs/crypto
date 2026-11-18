#set page(paper: "a4", margin: 18mm)
#set text(font: "New Computer Modern", size: 10pt)
#set par(leading: 0.55em)
= N83 finite m=2/m=3 factor-base panel
_Run report · run-20261019T0536Z · 2026-10-19 UTC_

The exact primary N83 subgroup was queried with the retained U and direct-subgroup D bases, each 16,627 signed-Frobenius columns and 2,760,082 points. Eight frozen public points were used. Target 0 had two A/A repetitions per base and summand count. The native worker ran on EC2 Spot instance `i-01f85a6c905181498`, pinned to one CPU, hard 4 GiB memory, zero swap and no network. Other research jobs shared the host, so elapsed times are L0 diagnostics.

#image("setup.svg", width: 100%)

D setup point-set and labeled-row digests matched exactly between archive import and fresh generation. The plot shows one setup observation per mode; it omits one-time archive construction and all query work.

== Finite public-query observations

- m=2 U: 9 complete arms, 24,840,738 complement attempts, zero relation rows, rank zero; worker wall mean 9.475 s (range 9.415–9.579 s).
- m=2 D: 9 complete arms, 24,840,738 attempts, zero rows, rank zero; mean 9.615 s (9.537–9.712 s).
- m=3 U: 9 complete arms, 9,437,184 sampled-pair attempts, zero rows, rank zero; mean 8.402 s (8.336–8.479 s).
- m=3 D: 9 complete arms, 9,437,184 attempts, zero rows, rank zero; mean 8.526 s (8.452–8.664 s).

All 36 natural worker receipts were complete and all 36 separate replay receipts passed. Four constructed controls passed separate general-field replay with changed-target and changed-row rejection. These finite counts do not estimate behavior under other base sizes, query policies or budgets.

== Sources and limits

Protocol: `research/n83_prebuild_cost_m23_20261019/PROTOCOL.md`; raw records: `research/n83_prebuild_cost_m23_20261019/runs/spot-l0-v1/`; SHA-256 raw manifest: `runs.sha256`; graph data: `reports/run-20261019T0536Z/setup-data.json`. Worker source commit `a147d3abf4d8e47a7677a61794a1ab8003b2c307`. Full archive construction, rank accumulation, linear algebra, target verification and matched rho are outside this run. The separate matched D query modes will be reported independently.

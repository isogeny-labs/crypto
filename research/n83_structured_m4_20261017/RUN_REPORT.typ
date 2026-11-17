#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
= Retained structured N83 base: bounded m = 4 query and m = 5 control

*Date:* 2026-10-17 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The frozen protocol is `PROTOCOL.md`; full receipts and commands are in
`RUN_REPORT.md`. The signed-Frobenius archive passed the native import checks:
2,001 orbit columns, 332,166 points, 166,083 distinct x-coordinates.

#image("run-diagram.svg", width: 100%)

== Bounded observations

#table(
  columns: (auto, auto, auto, auto),
  inset: 6pt,
  [Arm], [Completed calls], [Completed conflicts], [Final receipt],
  [m = 4], [8 / 8], [40,000], [`UNKNOWN_solver_cap`],
  [m = 5], [4 / 8], [20,000], [`UNKNOWN_budget`],
)

All completed calls returned `Unknown` at 5,000 conflicts. The m = 4
worker exited 0 in 784.331 s L0, with zero accepted rows within the cap,
rank 0 of 2,002 columns, and 487,276,544 bytes cgroup peak. The m = 5
worker reached its 900-second own deadline during the next call and exited
124 without OOM. Its unfinished call has no completed conflict count; rank
and cgroup peak were not written.

== Provenance and interpretation

The same Linux arm64 executable served both arms, SHA-256
`a26a3aeda02c70cf732c9219f4eb09cb8be3a65afd8f03cad74f86bd0e9bc477`.
Source commit: `1b00cf8a3450540aacd7778b6b2115276f46a81c`.
Each container had one CPU, 4 GiB memory, zero swap and no network.
The worker enforced 900 seconds, 8 trials, 5,000 conflicts per query and
32 models per query. The shared host was L0; a separate exporter build
overlapped part of the m = 5 arm. The arms ask different decomposition
questions, so their wall times are not a speed ratio.

The m = 4 exact results are `run4/result.json`, `run4/events.jsonl` and
`run4.container-state.json`. The m = 5 receipts are `run5/cap.json`,
`run5/events.jsonl` and `run5.container-state.json`. No accepted relation
was available for independent relation replay. No useful-rank or full-cold
cost comparison is established. The current-head library build obstruction
is recorded in `problems/current-main-library-build-20261017.md`.

The next gate compares a retained K = 2,048 explicit-orbit base with this
base under m = 4 and a degree-83 planted relation control. A larger
K = 16,627 prebuilt base is the separate cardinality-driven construction
candidate. Canonical progress graphs and the IC scoreboard remain unchanged.

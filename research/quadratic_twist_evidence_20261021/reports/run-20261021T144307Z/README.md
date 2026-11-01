# Native replay record: run-20261021T144307Z

[Raw record](../../native/runs/run-20261021T144307Z/) · [Run-series PDF](report.pdf) · [Series graph](coverage.svg)

This run is described individually in the shared series report's run table.
The PDF covers all three successful native replays and the failed first port
comparison; its graphic labels all three successful run IDs by their UTC start.
It is a shared series report, not a newly executed run or a replacement receipt.

The successful runs each match all 434 deterministic fields and independently
verify 84 IC certificate matrices. The first port attempt has no accepted parity
receipt: its comparator rejected two differently represented JSON null values.
The exact failing source and log are preserved. Its recorded header hash matches
the reconstructed original header retained beside that source.

The [shared Markdown report](../native-20261021T150125Z/README.md),
[LaTeX source](../native-20261021T150125Z/report.tex),
[plot source](../native-20261021T150125Z/render.py), and
[graph data](../native-20261021T150125Z/graph-data.json) bind the observations.
Only finite correctness is reported; no timing or general hardness conclusion.

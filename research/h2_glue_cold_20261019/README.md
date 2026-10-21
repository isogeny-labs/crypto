# H2 settlement: genus-2 IC on glued h = 1 instances vs negation rho (2026-10-19)

Result: no IC-wins cell among 12 price cells (6 rho-wins, 6 unresolved at native price 0 only);
the frozen rule's branch is "scope the ≤22M+I addition", which the data show would not flip a
verdict outside native price 0. Read `REPORT.pdf` (source `REPORT.typ`).

- `PROTOCOL.md`: frozen protocol, decision rule, Amendment 1 (written after run 1, before run 2).
- `results/run-20261019T1700Z/`: run 1 (160 targets); failed my A/A point tolerance, retained, no decision from it.
- `results/run-20261019T1730Z/`: decision run (320 targets, fresh seeds); `cold.csv`, `instances.csv`, `analysis.{txt,json}`.
- `analyze.py`, `plot.py`, `build.sh`: price, aggregate and plot recorded counters only.
- Harness: `examples/h2_glue_cold.rs`, fork of #2170's `examples/n2_routes.rs` (blob df0f097b…); build `rustc --edition 2021 -O`, run `N2_SEEDTAG=1 N2_TARGETS=20 N2_SIZES=36,40 N2_INST=8 h2_glue_cold <dir> cold`.
- Not #2229's ecbench adapter (Scholten-only, different backend); no PMU, so no instruction metric; wall times descriptive.

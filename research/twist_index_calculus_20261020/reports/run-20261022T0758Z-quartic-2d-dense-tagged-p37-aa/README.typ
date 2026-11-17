#set page(paper: "a4", margin: 16mm)
#set text(font: "Noto Sans", size: 9pt)
= Combined tagged and dense p=37 exact-oracle A/A

*Run:* `run-20261022T0758Z-quartic-2d-dense-tagged-p37-aa` · 22 October 2026 · shared Intel Xeon 6975P-C · L0. Two producer executions on the same public `[271]G`, 624-point base and first seed-zero residual completed before the 300-second and 8-GiB caps. The exact commands, source SHA-256 digests and raw links are in `README.md`.

The canonical non-timing JSON diff is empty after deleting only three L0 timing fields. Both executions returned 53,551 signed decompositions, the same 52 selected rank-raising rows, anchored rank 52 and answer 271. Each counted 89,828,606 base-field multiplications and 848,000 group operations, with 47,721,464 retained requested table bytes and 78,834,528 requested peak bytes above the pre-table allocation. The shared-host path wall observations were 632.080 and 653.005 ms; no isolated wall speed is claimed.

#image("dense_tagged_aa.png", width: 100%)

This A/A controls repeatability of the exact group-oracle producer; the independent point-keyed oracle in the main run report separately checks the relation set, rank and answer. Polynomial relation finding and calibrated rho remain outside this run.

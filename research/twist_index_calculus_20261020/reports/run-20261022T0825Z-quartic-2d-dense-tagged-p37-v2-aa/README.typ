#set page(paper: "a4", margin: 16mm)
#set text(font: "Noto Sans", size: 9pt)
= Final-source tagged/dense p=37 A/A

*Run:* `run-20261022T0825Z-quartic-2d-dense-tagged-p37-v2-aa` · 22 October 2026 · shared Intel Xeon 6975P-C · L0 · one public target `[271]G`. The two executions completed the same first residual before 300 seconds and 8 GiB. Exact commands, frozen inputs, source hashes and raw links are in `README.md`.

The canonical non-timing diff is empty after deleting only three L0 timing fields. Both runs returned the same complete 53,551 signed decompositions, selected 52 anchored rows, reached rank 52 and recovered 271. Each counted 89,828,606 Fp multiplications and 848,000 group operations, with 47,721,464 retained requested table bytes and 78,834,528 requested peak bytes above pre-table allocation. The shared-host wall observations were 639.113 and 633.789 ms; no isolated wall speed is claimed.

#image("dense_tagged_aa.png", width: 100%)

This A/A controls repeatability. The independent point-keyed oracle in the main run separately verifies the relation set, rank and target. Polynomial relation finding and calibrated rho remain outside this run.

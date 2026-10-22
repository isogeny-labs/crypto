#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9pt)
= Quartic twist24 v2 A/A repeat
UTC start `2026-10-22 05:43:44.734`; source commit `4d6cb3168e`; host Intel Xeon 6975P-C, 32 logical CPUs; 16 Rayon threads; L0 shared host. The first run's frozen protocol and binary were reused.

The repeat completed 21/21 registered records, 63/63 IC solves and 1,008/1,008 rho solves with zero failures. Raw file: `quartic_twist24_full_v2_aa.ndjson`, SHA-256 `56d655c7e13285bf62f497720a2498cb360f673c37c199dcd79d880751302d2b`. Deleting only the wall-millisecond field from both raw files yields an empty diff. Independent relation, rank and scalar replay passed. The graph shows the same non-timing counts as the first run because every count matches.

#image("quartic_twist24_cost_v2.png", width: 100%)

The A/A control establishes deterministic arithmetic counts for this L0 protocol; it does not establish an isolated wall-time effect. Exact command, diff, raw links and limitations are in `README.md` and the main `QUARTIC_TWIST24_REPORT.md`.

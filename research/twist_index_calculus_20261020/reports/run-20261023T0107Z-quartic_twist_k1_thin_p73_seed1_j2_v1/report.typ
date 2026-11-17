#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0107Z-quartic_twist_k1_thin_p73_seed1_j2_v1
Raw SHA-256: a43b1734effe09e0df9aabc4fba37c4ef7997b446ba97ac7531c81041fc5342d
Audit SHA-256: 8c7d21edf94f16f1d3acb335aba0781459b1c33ba824bb74bc5e0e91eec8dec8
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `partial`; independent point-keyed/BSGS audit `partial`; 28 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 96 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 9 reference, 7 polynomial, 2 missing; 1,344 attempted S4 systems, 1 failures.
- Counted candidate: 829,490,482 Fp multiplications and 12,141 group operations; same-seed full-base control 1,183,617,228 Fp multiplications (full/candidate 1.427).
- Same-target twist24 rho: 366,965 Fp multiplications and 3,030 group operations. Independent BSGS verified 72 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

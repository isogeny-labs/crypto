#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0110Z-quartic_twist_k1_thin_p73_seed2_j3_v1
Raw SHA-256: 5f1872813478a1f4a1b7b10f8dae2d8753fde02f967448e52a5a3d2b3bd46be7
Audit SHA-256: 1718ce5f95708d7f7dc8bd3b6522acd146675dae1a6d3049c82f0bd08cf41b0f
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 43 residuals, anchored rank 7/7, recovered 271.
- Physical base 84 retained points from 96 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 11 reference, 11 polynomial, 0 missing; 3,096 attempted S4 systems, 0 failures.
- Counted candidate: 1,907,506,701 Fp multiplications and 16,014 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 1.333).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 84 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

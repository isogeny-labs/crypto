#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0108Z-quartic_twist_k1_thin_p73_seed2_j1_aa_v1
Raw SHA-256: 6c254529d4ddf65ff007c2360f0e54b49512bc5422f930b791e6347f024de3b2
Audit SHA-256: bd9ec176e37258298498e6e858e0752959d1d80669cb6b69e3e11257f9ed036c
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 47 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 96 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 5 reference, 5 polynomial, 0 missing; 1,128 attempted S4 systems, 0 failures.
- Counted candidate: 696,092,141 Fp multiplications and 12,281 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 3.653).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

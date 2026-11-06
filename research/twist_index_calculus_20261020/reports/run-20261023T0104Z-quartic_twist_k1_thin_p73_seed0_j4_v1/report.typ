#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0104Z-quartic_twist_k1_thin_p73_seed0_j4_v1
Raw SHA-256: 330e153cbd95c1cfea02af4c7b3b8fa66c05585798d8ba7ced0150a09c023452
Audit SHA-256: a4dbd337db05130a0314da027d150a47c5ffb0235ca9b8c74b5a796f40c48b34
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 0 4`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 15 residuals, anchored rank 8/8, recovered 271.
- Physical base 96 retained points from 96 constructed points; 4 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 8 reference, 8 polynomial, 0 missing; 1,440 attempted S4 systems, 0 failures.
- Counted candidate: 888,203,272 Fp multiplications and 9,988 group operations; same-seed full-base control 888,203,272 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 496,841 Fp multiplications and 3,735 group operations. Independent BSGS verified 96 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

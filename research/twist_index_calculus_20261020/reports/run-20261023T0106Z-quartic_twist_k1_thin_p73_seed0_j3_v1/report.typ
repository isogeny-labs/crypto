#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0106Z-quartic_twist_k1_thin_p73_seed0_j3_v1
Raw SHA-256: a070aa4ebda00cc1887793936adef34fc289dc5802521c67623b98d474e6627e
Audit SHA-256: 3b4619a119976b585c845e4ca3576e8a9028b7f7ac14d60b03d32f640d709f6d
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 0 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 15 residuals, anchored rank 7/7, recovered 271.
- Physical base 84 retained points from 96 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 7 reference, 7 polynomial, 0 missing; 1,080 attempted S4 systems, 0 failures.
- Counted candidate: 666,998,170 Fp multiplications and 11,760 group operations; same-seed full-base control 888,203,272 Fp multiplications (full/candidate 1.332).
- Same-target twist24 rho: 496,841 Fp multiplications and 3,735 group operations. Independent BSGS verified 84 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

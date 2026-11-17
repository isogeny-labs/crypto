#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0123Z-quartic_twist_k1_thin_p97_seed1_j3_aa_v1
Raw SHA-256: 4725bd4127d937eee48db33c476b16845a8ea4940140e916d5628c4fe6c23341
Audit SHA-256: 2d1275313d2ba0216f43eb2e3ea086dab5950a005898d90b261f53b6aa519b2c
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 75 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 72 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 6 reference, 6 polynomial, 0 missing; 5,400 attempted S4 systems, 0 failures.
- Counted candidate: 3,349,559,070 Fp multiplications and 17,475 group operations; same-seed full-base control 3,349,559,070 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 500,318 Fp multiplications and 3,822 group operations. Independent BSGS verified 72 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

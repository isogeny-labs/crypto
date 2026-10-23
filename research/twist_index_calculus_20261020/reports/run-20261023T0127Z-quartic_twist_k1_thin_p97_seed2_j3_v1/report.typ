#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0127Z-quartic_twist_k1_thin_p97_seed2_j3_v1
Raw SHA-256: 6d9f255fe55806951f4ac36dd8d498310a5701222cea148b91b274b3fe62fa0a
Audit SHA-256: 0e21b267b9612dcc6883225383d5738415010681adf0d6f025bf8dc3aaed64ee
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 150 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 72 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 6 reference, 6 polynomial, 0 missing; 10,800 attempted S4 systems, 0 failures.
- Counted candidate: 6,696,848,851 Fp multiplications and 28,588 group operations; same-seed full-base control 6,696,848,851 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 791,504 Fp multiplications and 5,395 group operations. Independent BSGS verified 72 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

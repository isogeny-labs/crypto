#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0119Z-quartic_twist_k1_thin_p97_seed1_j1_v1
Raw SHA-256: 966cfb925b7a41e06359c58925a8996b1c546469b47e709cfdfdf871895648b9
Audit SHA-256: ab7923b5dfc325443eff44f9a719b7507d1f4709c34b91e7f62c7ae81083e436
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 302 residuals, anchored rank 4/4, recovered 271.
- Physical base 48 retained points from 72 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 4 reference, 4 polynomial, 0 missing; 7,248 attempted S4 systems, 0 failures.
- Counted candidate: 4,496,535,068 Fp multiplications and 37,541 group operations; same-seed full-base control 3,349,559,070 Fp multiplications (full/candidate 0.745).
- Same-target twist24 rho: 500,318 Fp multiplications and 3,822 group operations. Independent BSGS verified 48 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

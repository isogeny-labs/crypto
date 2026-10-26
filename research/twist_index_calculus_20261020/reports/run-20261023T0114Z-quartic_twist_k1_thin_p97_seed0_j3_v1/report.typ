#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0114Z-quartic_twist_k1_thin_p97_seed0_j3_v1
Raw SHA-256: 18460bd93a7a3723ec22478b12059ba5540364b54041f71e30d206bcdf21cc23
Audit SHA-256: 36fb81da79ee86306b42202052b19d9c9e2ec81b189c93db5517b3eb77a294b7
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 310 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 72 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 16 reference, 16 polynomial, 0 missing; 22,320 attempted S4 systems, 0 failures.
- Counted candidate: 13,839,508,536 Fp multiplications and 53,111 group operations; same-seed full-base control 13,839,508,536 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 802,322 Fp multiplications and 5,463 group operations. Independent BSGS verified 72 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

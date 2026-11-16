#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0123Z-quartic_twist_k1_thin_p97_seed2_j1_v1
Raw SHA-256: fb0204667176e83870e770905b502a000fe550b5845cfcd85118ec86e552ea9f
Audit SHA-256: 9a20002ef596d59825d232cd97d41168922e99c535644e34a9459b7e8e56f6fa
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 182 residuals, anchored rank 4/4, recovered 271.
- Physical base 48 retained points from 72 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 4 reference, 4 polynomial, 0 missing; 4,368 attempted S4 systems, 0 failures.
- Counted candidate: 2,710,417,878 Fp multiplications and 25,189 group operations; same-seed full-base control 6,696,848,851 Fp multiplications (full/candidate 2.471).
- Same-target twist24 rho: 791,504 Fp multiplications and 5,395 group operations. Independent BSGS verified 48 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0105Z-quartic_twist_k1_thin_p73_seed0_j1_v1
Raw SHA-256: af8cfaf233c6b9920d2f64716c6670d0c8cdd8b0f9e3bacf3ea553c3d2a55a87
Audit SHA-256: ed0816775b6c9ae011e33c8b385ec2324fe59370e7c5b190ec58d42d1ee356fc
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 0 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 30 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 96 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 6 reference, 6 polynomial, 0 missing; 720 attempted S4 systems, 0 failures.
- Counted candidate: 445,445,605 Fp multiplications and 10,701 group operations; same-seed full-base control 888,203,272 Fp multiplications (full/candidate 1.994).
- Same-target twist24 rho: 496,841 Fp multiplications and 3,735 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

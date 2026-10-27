#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0109Z-quartic_twist_k1_thin_p73_seed2_j2_v1
Raw SHA-256: 042c2521c08d60fb83cd0d7327f0b99b63767ae218faeb8552c6c7db8c17fd46
Audit SHA-256: 941cbee09aa054d4b975001630de1625ad254b33a46af21229e8e99c63da6e15
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 45 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 96 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 6 reference, 6 polynomial, 0 missing; 2,160 attempted S4 systems, 0 failures.
- Counted candidate: 1,331,570,891 Fp multiplications and 14,074 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 1.910).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 72 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

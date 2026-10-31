#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0125Z-quartic_twist_k1_thin_p97_seed2_j2_v1
Raw SHA-256: 47fcde51f1b3e9cef6be0ce4126dd35d1dc5fad2225392e52cbfe53aebf79cb5
Audit SHA-256: ce3731210544554413fdb1294e56b92dee2f2a93f4c530ffc68e4766cbdc9993
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 150 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 72 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 5 reference, 5 polynomial, 0 missing; 7,200 attempted S4 systems, 0 failures.
- Counted candidate: 4,465,588,225 Fp multiplications and 26,573 group operations; same-seed full-base control 6,696,848,851 Fp multiplications (full/candidate 1.500).
- Same-target twist24 rho: 791,504 Fp multiplications and 5,395 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0121Z-quartic_twist_k1_thin_p97_seed1_j2_aa_v1
Raw SHA-256: f30c2d3e8ba4a3670b8eed89dcd47822ab2cdc849f02e5983bb730ea0f38226f
Audit SHA-256: ec659486e159c0fbe8e6c04d62ae621b24a9efe9054e4fbd4238b7addcc975db
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 75 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 72 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 5 reference, 5 polynomial, 0 missing; 3,600 attempted S4 systems, 0 failures.
- Counted candidate: 2,233,996,578 Fp multiplications and 17,273 group operations; same-seed full-base control 3,349,559,070 Fp multiplications (full/candidate 1.499).
- Same-target twist24 rho: 500,318 Fp multiplications and 3,822 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

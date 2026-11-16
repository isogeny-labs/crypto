#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0112Z-quartic_twist_k1_thin_p73_seed2_j4_aa_v1
Raw SHA-256: 28ea03f8b74337aaa1f72f3172cf0efe105ac9f439f96d2971836ff83dcbe1f1
Audit SHA-256: 54b8ca3088cd0b35132a6124a68ba4bd1cfbe137dae8f9a6948746299a889776
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 4`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 43 residuals, anchored rank 8/8, recovered 271.
- Physical base 96 retained points from 96 constructed points; 4 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 15 reference, 15 polynomial, 0 missing; 4,128 attempted S4 systems, 0 failures.
- Counted candidate: 2,542,727,892 Fp multiplications and 15,084 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 96 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

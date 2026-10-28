#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0108Z-quartic_twist_k1_thin_p73_seed1_j4_aa_v1
Raw SHA-256: 5fde1747a9106bc7e3e913c7faa9abae29897ed24af1c2af036f8021cfb02c0c
Audit SHA-256: 0c91510bf1449e475ce59e58d3e11ca5626409264ad3f81fa40672d7c5e1234e
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1 4`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `partial`; independent point-keyed/BSGS audit `partial`; 20 residuals, anchored rank 8/8, recovered 271.
- Physical base 96 retained points from 96 constructed points; 4 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 11 reference, 9 polynomial, 2 missing; 1,920 attempted S4 systems, 1 failures.
- Counted candidate: 1,183,617,228 Fp multiplications and 10,891 group operations; same-seed full-base control 1,183,617,228 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 366,965 Fp multiplications and 3,030 group operations. Independent BSGS verified 96 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

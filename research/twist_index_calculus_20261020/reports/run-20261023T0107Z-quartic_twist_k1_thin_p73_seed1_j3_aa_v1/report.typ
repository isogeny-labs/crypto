#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0107Z-quartic_twist_k1_thin_p73_seed1_j3_aa_v1
Raw SHA-256: 8254b88566d0cf9a15abc9b2a7fda89abee46257c037c1ce43dcfa1feeee95da
Audit SHA-256: 9bb81b224a9fe8f712ed93225db72abf2866424a570f661878c5c300aaaf745f
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1 3`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `partial`; independent point-keyed/BSGS audit `partial`; 20 residuals, anchored rank 7/7, recovered 271.
- Physical base 84 retained points from 96 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 10 reference, 8 polynomial, 2 missing; 1,440 attempted S4 systems, 1 failures.
- Counted candidate: 888,451,433 Fp multiplications and 12,564 group operations; same-seed full-base control 1,183,617,228 Fp multiplications (full/candidate 1.332).
- Same-target twist24 rho: 366,965 Fp multiplications and 3,030 group operations. Independent BSGS verified 84 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

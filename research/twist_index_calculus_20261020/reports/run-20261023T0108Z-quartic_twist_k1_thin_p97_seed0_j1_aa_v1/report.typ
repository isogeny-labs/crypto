#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0108Z-quartic_twist_k1_thin_p97_seed0_j1_aa_v1
Raw SHA-256: 29063dfceb88714a8f6d1c62535bb11e6e2d2020c2cbeba72f0df567fe563d1d
Audit SHA-256: 9711d15221f9f171067b96891c47904dd5a010f19fc8f15afd1d435b2e62e2f1
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 310 residuals, anchored rank 4/4, recovered 271.
- Physical base 48 retained points from 72 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 4 reference, 4 polynomial, 0 missing; 7,440 attempted S4 systems, 0 failures.
- Counted candidate: 4,616,280,829 Fp multiplications and 38,187 group operations; same-seed full-base control 13,839,508,536 Fp multiplications (full/candidate 2.998).
- Same-target twist24 rho: 802,322 Fp multiplications and 5,463 group operations. Independent BSGS verified 48 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

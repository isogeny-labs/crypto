#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0110Z-quartic_twist_k1_thin_p97_seed0_j2_aa_v1
Raw SHA-256: dabac33fd726de17762a76a6a978edf37952bd1f5eae0f1d6a28f22716072945
Audit SHA-256: ddbc153b9507bd2745d79ffded3013a1e45d53e7944b07a6f65298e88956bc3f
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0 2`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `pass`; independent point-keyed/BSGS audit `pass`; 130 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 72 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 8 reference, 8 polynomial, 0 missing; 6,240 attempted S4 systems, 0 failures.
- Counted candidate: 3,870,889,536 Fp multiplications and 24,149 group operations; same-seed full-base control 13,839,508,536 Fp multiplications (full/candidate 3.575).
- Same-target twist24 rho: 802,322 Fp multiplications and 5,463 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

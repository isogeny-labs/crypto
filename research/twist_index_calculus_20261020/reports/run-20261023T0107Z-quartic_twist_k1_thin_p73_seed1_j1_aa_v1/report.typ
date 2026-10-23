#set page(paper: "a4", margin: 18mm)
#set text(size: 9.4pt)
= run-20261023T0107Z-quartic_twist_k1_thin_p73_seed1_j1_aa_v1
Raw SHA-256: 4ee2db71f6aaaaca945500f0acb05bd196d16725be2fd93532a136d856d111bb
Audit SHA-256: 71c39bf3a5fe912cac337eacf2bbdda73fdf9634516464497a27e9a5cc38d47e
Protocol cadd7b9001; Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. Public target [271]G.
== Execution
`target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 1 1`
8 GiB address-space cap; 1200 s watchdog.
== Recorded observations
- Native `partial`; independent point-keyed/BSGS audit `partial`; 45 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 96 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 8 reference, 6 polynomial, 2 missing; 1,080 attempted S4 systems, 1 failures.
- Counted candidate: 667,133,505 Fp multiplications and 12,209 group operations; same-seed full-base control 1,183,617,228 Fp multiplications (full/candidate 1.774).
- Same-target twist24 rho: 366,965 Fp multiplications and 3,030 group operations. Independent BSGS verified 60 retained physical factor logs.
#image("graph.svg", width: 100%)
Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. No priced IC/rho or external-solver claim.

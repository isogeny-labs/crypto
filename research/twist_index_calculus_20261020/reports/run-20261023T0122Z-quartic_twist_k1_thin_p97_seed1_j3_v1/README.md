# run-20261023T0122Z-quartic_twist_k1_thin_p97_seed1_j3_v1

[Raw record](../../quartic_twist_k1_thin_p97_seed1_j3_v1.json) SHA-256 `26dcfa1e961c0c7c806bdbd93706474ed76bef79cc38a0e69822bf520d3c501a`; [independent audit](../../quartic_twist_k1_thin_p97_seed1_j3_audit_v1.json) SHA-256 `2d1275313d2ba0216f43eb2e3ea086dab5950a005898d90b261f53b6aa519b2c`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 1 3` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 75 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 72 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 6 reference, 6 polynomial, 0 missing; 5,400 attempted S4 systems, 0 failures.
- Counted candidate: 3,349,559,070 Fp multiplications and 17,475 group operations; same-seed full-base control 3,349,559,070 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 500,318 Fp multiplications and 3,822 group operations. Independent BSGS verified 72 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

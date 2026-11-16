# run-20261023T0110Z-quartic_twist_k1_thin_p73_seed2_j3_v1

[Raw record](../../quartic_twist_k1_thin_p73_seed2_j3_v1.json) SHA-256 `5f1872813478a1f4a1b7b10f8dae2d8753fde02f967448e52a5a3d2b3bd46be7`; [independent audit](../../quartic_twist_k1_thin_p73_seed2_j3_audit_v1.json) SHA-256 `1718ce5f95708d7f7dc8bd3b6522acd146675dae1a6d3049c82f0bd08cf41b0f`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 3` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 43 residuals, anchored rank 7/7, recovered 271.
- Physical base 84 retained points from 96 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 11 reference, 11 polynomial, 0 missing; 3,096 attempted S4 systems, 0 failures.
- Counted candidate: 1,907,506,701 Fp multiplications and 16,014 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 1.333).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 84 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

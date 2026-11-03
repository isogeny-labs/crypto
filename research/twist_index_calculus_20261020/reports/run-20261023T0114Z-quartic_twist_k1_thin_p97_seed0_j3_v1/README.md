# run-20261023T0114Z-quartic_twist_k1_thin_p97_seed0_j3_v1

[Raw record](../../quartic_twist_k1_thin_p97_seed0_j3_v1.json) SHA-256 `18460bd93a7a3723ec22478b12059ba5540364b54041f71e30d206bcdf21cc23`; [independent audit](../../quartic_twist_k1_thin_p97_seed0_j3_audit_v1.json) SHA-256 `36fb81da79ee86306b42202052b19d9c9e2ec81b189c93db5517b3eb77a294b7`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0 3` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 310 residuals, anchored rank 6/6, recovered 271.
- Physical base 72 retained points from 72 constructed points; 3 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 16 reference, 16 polynomial, 0 missing; 22,320 attempted S4 systems, 0 failures.
- Counted candidate: 13,839,508,536 Fp multiplications and 53,111 group operations; same-seed full-base control 13,839,508,536 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 802,322 Fp multiplications and 5,463 group operations. Independent BSGS verified 72 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

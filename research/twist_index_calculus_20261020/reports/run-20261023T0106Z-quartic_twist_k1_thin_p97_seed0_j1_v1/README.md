# run-20261023T0106Z-quartic_twist_k1_thin_p97_seed0_j1_v1

[Raw record](../../quartic_twist_k1_thin_p97_seed0_j1_v1.json) SHA-256 `9f3ad268a04cf5fc2baec41d858a1ecbbc6987f93b07acbe1c3dbb6976af4f79`; [independent audit](../../quartic_twist_k1_thin_p97_seed0_j1_audit_v1.json) SHA-256 `9711d15221f9f171067b96891c47904dd5a010f19fc8f15afd1d435b2e62e2f1`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 0 1` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 310 residuals, anchored rank 4/4, recovered 271.
- Physical base 48 retained points from 72 constructed points; 1 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 4 reference, 4 polynomial, 0 missing; 7,440 attempted S4 systems, 0 failures.
- Counted candidate: 4,616,280,829 Fp multiplications and 38,187 group operations; same-seed full-base control 13,839,508,536 Fp multiplications (full/candidate 2.998).
- Same-target twist24 rho: 802,322 Fp multiplications and 5,463 group operations. Independent BSGS verified 48 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

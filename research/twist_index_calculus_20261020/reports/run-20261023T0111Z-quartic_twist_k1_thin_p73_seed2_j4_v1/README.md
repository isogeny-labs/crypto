# run-20261023T0111Z-quartic_twist_k1_thin_p73_seed2_j4_v1

[Raw record](../../quartic_twist_k1_thin_p73_seed2_j4_v1.json) SHA-256 `04d94cce5f755241f906f59b4efb274c5073b8f3883a58f5231517c79a97be05`; [independent audit](../../quartic_twist_k1_thin_p73_seed2_j4_audit_v1.json) SHA-256 `54b8ca3088cd0b35132a6124a68ba4bd1cfbe137dae8f9a6948746299a889776`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 73 2 4` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 43 residuals, anchored rank 8/8, recovered 271.
- Physical base 96 retained points from 96 constructed points; 4 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 15 reference, 15 polynomial, 0 missing; 4,128 attempted S4 systems, 0 failures.
- Counted candidate: 2,542,727,892 Fp multiplications and 15,084 group operations; same-seed full-base control 2,542,727,892 Fp multiplications (full/candidate 1.000).
- Same-target twist24 rho: 481,339 Fp multiplications and 3,619 group operations. Independent BSGS verified 96 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

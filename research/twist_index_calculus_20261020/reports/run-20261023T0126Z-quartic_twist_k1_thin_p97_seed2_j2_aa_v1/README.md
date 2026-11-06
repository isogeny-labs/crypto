# run-20261023T0126Z-quartic_twist_k1_thin_p97_seed2_j2_aa_v1

[Raw record](../../quartic_twist_k1_thin_p97_seed2_j2_aa_v1.json) SHA-256 `cd80e740c39d61812cca68151aa59f4171b54d12bfb0a2202dec601d0db3ea99`; [independent audit](../../quartic_twist_k1_thin_p97_seed2_j2_audit_aa_v1.json) SHA-256 `ce3731210544554413fdb1294e56b92dee2f2a93f4c530ffc68e4766cbdc9993`.

Frozen native-thinning protocol commit `cadd7b9001`; [producer](../../../../examples/quartic_twist_k1_thin.rs) SHA-256 `b58e34d64a6f83ebc5dac7a1aa2670a60369515a1665aedbd37a2b46ceadd188`; [auditor](../../../../examples/quartic_twist_k1_thin_audit.rs) SHA-256 `46d6432070820bc431721a4d24bdff6e55bc8b49eda534272e274b9a6c9cc0a8`.

## Execution

Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run. One public target `[271]G`.

Command: `target/release/examples/quartic_twist_k1_thin research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 97 2 2` with 8 GiB address-space cap and 1,200 s watchdog.

## Recorded observations

- Native `pass`; independent point-keyed/BSGS audit `pass`; 150 residuals, anchored rank 5/5, recovered 271.
- Physical base 60 retained points from 72 constructed points; 2 complete L2 orbits. Full-base construction and map, plus a proper-subset map, are charged.
- Exact signed k=1 sets on processed prefix: 5 reference, 5 polynomial, 0 missing; 7,200 attempted S4 systems, 0 failures.
- Counted candidate: 4,465,588,225 Fp multiplications and 26,573 group operations; same-seed full-base control 6,696,848,851 Fp multiplications (full/candidate 1.500).
- Same-target twist24 rho: 791,504 Fp multiplications and 5,395 group operations. Independent BSGS verified 60 retained physical factor logs.

![Recorded rank and polynomial field-work trace](graph.svg)

The graph plots native recorded values only. Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. Field-operation ratios do not constitute a priced IC/rho or external-solver speed claim.

[PDF](report.pdf).

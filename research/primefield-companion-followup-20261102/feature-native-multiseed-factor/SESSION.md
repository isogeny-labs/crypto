## 2026-11-05 — Four-input residual factor and normalized character consumer (T-1712)

### What was done (commands, files, features)

Derived the residual monomial in the seed and three normalized division values. Implemented a geometry-only residual and homogeneous final factors; appended the seed transport column using existing public shifts and retained grids from the same geometry pass. Calibration compares homogeneous cross-products. Thirty-five native tests pass: 2545 symbolic exponent cases, 288 arbitrary scaled-input ratios, 499 full-subgroup characters/ratios, and176 shifted characters. Separate frozen-source checker accepts499 rows and rejects a corrupted numerator. Native/checker source locks contain14/15 entries. [pcc:unverified]

### Decisions made (and why)

Consume normalized inputs directly and share initial phases/value contexts. Keep the inherited phase-domain and seed-law assumptions explicit. Claims PCC-20261105-70dedf and PCC-20261105-37f140 have local verdict not_covered, no signed standing. Sources checked (KB): unavailable MCP and wrapper executable; used saved seed factor, exact scalar source and transport code, plus NIST DLMF23.10.3. The latter supplies the transport antecedent, not the program exponent proof. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Add a consuming adaptive cursor retaining the last two normalized columns and the projective geometry tail, then connect the quartic decoder while preserving skipped-sign behavior. Account for seed-column updates, per-input powers, retained grids, resultants and allocation. Original4563-case cohort unchanged. No comparative panel or cloud job. PR3498 remains open; extend it to include this consumer. Another task T-1713 currently holds the shared dated journal; append this entry once the scope is free.

### Errors (what went wrong, how it was handled)

The separate checker first failed Rust type inference for an Option point; preserved the failed build log and added an explicit type. No native test failure. Native build retained5 warnings. A claim command rejected unsupported proof kind note without writing a claim; repeated with the supported unsigned attestation type. Historical proofs and raw artifacts remain unchanged.

## 2026-11-07 — Adaptive combined-monomial recovery (T-1740)

### What was done (commands, files, features)

Integrated the combined pair into the adaptive geometry tail, consumer and frozen quartic decoder. Native suite passes207 tests including100 split windows,1375 advances,450 skipped residuals and20 frozen-agreeing forward-checked recoveries. Every extension carries one pair; phase/value direct counts are checked. Separate checker accepts479-input identity and20 records, rejects scalar and initialization corruptions. Thirty-six source/data hashes retained. [pcc:unverified]

### Decisions made (and why)

Preserve the conditional monomial law, all geometry lanes, parity anchors, length1 direct initialization, overflow/limit rejection and skipped-state advancement. No timing interpretation of unit-test elapsed time. Sources checked (KB): MCP/wrapper unavailable; predecessor derivations, cursor/source/tests/generator and previously reviewed DLMF23.10.3. Claim PCC-20261107-16444b remains unsigned (not_covered). Original4563-case cohort unchanged; no cloud job.

### Open threads (unfinished, pick up next time)

Matched accounting of four-column and monomial adaptive recovery, including setup, direct initializations, coordinate-difference powers on skipped steps, residual polynomial work, allocations and forward verification. PR3597 remains open; extend it with this component. Full goal remains active.

### Errors (what went wrong, how it was handled)

No native failure; build warnings retained. Initial filename lookups used underscores while the predecessor files use hyphens; corrected before implementation. A publication task was claimed without scopes; its card exposed the omission, so actual shared-file reservations were reacquired before packaging. Earlier PR body lint failure was fixed; current reported head has no failed checks at inspection. Shared daily-journal reservation became free; append this and the prior monomial component entry without altering prior history.

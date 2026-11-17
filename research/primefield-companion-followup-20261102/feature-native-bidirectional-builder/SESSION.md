## 2026-11-05 — Bidirectional nonsingular columns (T-1640)

### What was done (commands, files, features)

Implemented forward and reciprocal Miller prefixes, integrated with the reflection solver. Saved native run reports 102 passing tests, 1,037 column comparisons, 36 span controls, the existing q15 rejection, all479 identities and eight integrations. The packet matches the prior reflection packet byte for byte. Fresh separate checker replay accepts it; 22 source/data locks bind the run. [pcc:unverified]

### Decisions made (and why)

The two endpoint intervals cover cubic powers without a singular coefficient step. Preserve all prior records and retain the cubic label API in this implementation. Filed theorem PCC-20261105-f52ca7 and execution PCC-20261105-c46a77; the local PCC verifier returns not_covered, with no signed standing. No timing comparison inferred. [pcc:unverified]

### Open threads (unfinished, pick up next time)

The same endpoint argument covers quartic powers; next implement quartic synthesis and examine source-shift requirements. A low-length displacement generator for structured elimination remains an open construction. Original4563-case cohort is preserved. Current-head CI and review pending. [pcc:unverified]

### Errors (what went wrong, how it was handled)

Eight compiler warnings retained; no test failure recorded. Current FLINT function page fetch failed; module documentation and its revision notice were read and the recurrence derivation is explicit. Previous negative checker artifact retained, not rerun. No cloud job launched.

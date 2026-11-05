## 2026-11-05 — Normalized fixed-index transport (T-1703)

### What was done (commands, files, features)

Implemented public-point transport for normalized division values at the source's fixed indices. Seed normalization cancels the division-addition denominator power, yielding six-product homogeneous updates per column. Fifteen native tests pass: 1,490 complete subgroup values, 528 shifted-window values, homogeneous scale and rejection controls. A separate affine-point/frozen-jet checker accepts all 1,490 rows and rejects a corrupted numerator. Eight source/data hashes and four compiler warnings retained. [pcc:unverified]

### Decisions made (and why)

Preserve the inherited seed-law and phase-domain assumptions. Use existing multiplier lanes and anchors O,P,-P,2P; initialize at most the first two non-anchor offsets directly. This component exposes normalized columns, not an integrated character-recovery route. Claims PCC-20261105-421d0b and PCC-20261105-cef8e8 have local verdict not_covered. Sources checked (KB): unavailable MCP/wrapper; used saved seed/geometry code, DLMF23.10.3 and Stange2011 Sections3-4. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Factor the character residual's dependence on the seed and initial fixed-index values so it can consume transported homogeneous columns without undoing normalization. Reuse initial phases and geometry when integrating. Complete-cost accounting remains. Original 4,563-case cohort unchanged; no cloud or timing panel. PR3497 merged; publish this increment separately. Current-head CI/review pending.

### Errors (what went wrong, how it was handled)

No native failure. Intentional corrupted-column rejection retained. Published predecessor artifacts remain unchanged; upstream merged by fast-forward.

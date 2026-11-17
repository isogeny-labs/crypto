## 2026-11-05 — Normalized fixed-index transport (T-1703)

### What was done (commands, files, features)

Implemented public-point transport for normalized division values at the source's fixed indices. Seed normalization cancels the division-addition denominator power, yielding six-product homogeneous updates per column. Fifteen native tests pass: 1,490 complete subgroup values, 528 shifted-window values, homogeneous scale and rejection controls. A separate affine-point/frozen-jet checker accepts all 1,490 rows and rejects a corrupted numerator. Eight source/data hashes and four compiler warnings retained. [pcc:unverified]

### Decisions made (and why)

Preserve the inherited seed-law and phase-domain assumptions. Use existing multiplier lanes and anchors O,P,-P,2P; initialize at most the first two non-anchor offsets directly. This component exposes normalized columns, not an integrated character-recovery route. Claims PCC-20261105-421d0b and PCC-20261105-cef8e8 have local verdict not_covered. Sources checked (KB): unavailable MCP/wrapper; used saved seed/geometry code, DLMF23.10.3 and Stange2011 Sections3-4. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Factor the character residual's dependence on the seed and initial fixed-index values so it can consume transported homogeneous columns without undoing normalization. Reuse initial phases and geometry when integrating. Complete-cost accounting remains. Original 4,563-case cohort unchanged; no cloud or timing panel. PR3497 merged; publish this increment separately. Current-head CI/review pending.

### Errors (what went wrong, how it was handled)

No native failure. Intentional corrupted-column rejection retained. Published predecessor artifacts remain unchanged; upstream merged by fast-forward.

## 2026-11-05 — Artifact storage and PR packaging

### What was done (commands, files, features)

Moved the unchanged columns.tsv and negative-columns.tsv bytes to the repository content-addressed store. Each is 66,081 bytes. The adjacent .artifacts.json records SHA-256 and byte counts; store-readback.json records durable URIs and fresh downloads to a temporary directory with both hashes and lengths verified. Added the artifact pull command before replay.

### Decisions made (and why)

Keep data above the 64 KiB policy limit out of the final Git diff. Retain source, proof notes, receipts, raw logs and PDF in Git. Preserve the already pushed implementation commit and add a storage correction commit.

### Open threads (unfinished, pick up next time)

Open the regular PR, enable auto-merge under repository policy, and report current-head checks. Consumer integration remains the next research construction.

### Errors (what went wrong, how it was handled)

The first artifact check rejected the two oversized tables, but a shell separator allowed the implementation push to proceed. Correct packaging before PR creation; gate the next push on a successful cumulative artifact check. No native artifacts were regenerated.

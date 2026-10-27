# Index-calculus dashboard coverage and data architecture

The canonical page is `docs/ic-leaderboard.html`, generated alongside
`docs/ic/leaderboard.json` and `docs/ic/LEADERBOARD.md`. The lab browser's
`docs/browser/data.json` joins those rows to the ICV1 registry and committed
sessions. The scoreboard's progress chart reads `docs/ic/progress-timeline.json`.
These are published views of frozen evidence, not independent measurements.

## Meaning of comprehensive

1. **Discoverability:** every ICV1 slug in `docs/curves/registry.json` appears
   in both the leaderboard roster and the lab browser. Search must expose
   unpriced curves as unpriced. Registry coverage and whole-pipeline pricing
   coverage are different counts.
2. **Comparability:** each priced row identifies its target count, cold or
   online scope, operation or timing unit, measured factor-base size and
   recipe, exact FB1 linkage status, phase split, source, validation status,
   matched rho and floor. Counted operations use `2^x`; IC/rho uses `1.0×`
   as the parity line. Other regimes remain visibly separate.
3. **History:** a corrected or superseded figure remains findable, and each
   source-backed comparison gets a dated timeline point with its reference
   quality and class. A diagnostic can be indexed without becoming a priced
   whole-pipeline row.
4. **Freshness:** generators and `scripts/check_ic_dashboard_coverage.mjs`
   pass on complete inputs before merge. After merge, verify the published
   Pages route, not just the repository source.

The coverage checker prints the current registered, searchable, priced and
unreconciled counts. A passing check establishes index joins and required
metadata; it does not prove that every research result is a valid ECDLP
comparison. The frozen source, replay and validation gates decide that.

## Database decision

The committed frozen files and receipts remain authoritative. `ecbench` already
offers a rebuildable SQLite index for query work. The published dashboard is a
static snapshot built from those inputs. AWS RDS would add a mutable service,
credentials, operations and publication state without fixing missing ICV1
joins, unreconciled factor bases or stale generated files. Keep the snapshot
pipeline and enforce its joins first.

This is also a cost and operations choice: GitHub Pages serves static files,
whereas [RDS bills DB instances and provisioned storage](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_OnDemandDBInstances.html).
SQLite is a [portable single-file database](https://www.sqlite.org/onefile.html),
suited to the existing rebuildable local index. Neither database changes the
validation requirements on a published row.

The browser JSON was about 67 MiB uncompressed and 5.7 MiB gzip in the
2026-10-27 checkout, so load time deserves attention. If transfer size becomes
the bottleneck, split the static index by family or prefix and load details on
demand. Consider a hosted database only after a measured query or update need
cannot be met by generated shards or SQLite, with a documented schema,
rebuild procedure, immutable source hashes, public snapshot version and
failure behavior. RDS would then be a serving index, never the evidence store.

## Known coverage boundary at this change

The ranked tables contain 28 regime rows for 24 curves. The registry names
20,592 curves. Exact FB1 identities for the ranked rows are not reconciled;
the page shows measured base sizes and recipes with that limitation. The
browser index in the starting revision held 15,791 curves and an older
registry digest. The coverage gate treats that as a stale index to repair,
not as 4,801 absent curves to hide or price by extrapolation.

# Historical partial-panel accounting: verified implementation check

This is read-only analysis of the **already committed 29-exit snapshot**,
not a new solve or an updated live snapshot. It checks the nine configurations
marked verified in that exact snapshot, retains all 85 workload rows and
leaves the remaining rows' metrics null. Later native completions do not
enter these results. The full panel remains a separate running owner.

- Source commit before build: `3f2b78c8f9d9cdab267661e649d482966f9af770`.
- Three native tests passed, zero failed; build and explicit snapshot exit 0.
- Actual partial-as-complete control exits 2 before creating output.
- Wrapper terminal exit 0; compiled source and input-summary checks pass.
- Accounting executable SHA-256:
  `e82aaab4ed24f854ee91c7104cb9e0657883edeb46e17d364ce4503c833598e7`.
- [Accounting JSON](accounting.json): 199,384 bytes, SHA-256
  `02ccf1fbffc42204d76eb59414067547073aa3275293b05fb7a753804eac8afa`.
- Source summary SHA-256:
  `fc792d10ff81ca8759c9bad901cf021171bc54f6b2fde2b3557be4827ebafd0c`.

Every included configuration binds first/audit/parity hashes, three
registered exit-0 executions and direct deterministic A/A equality.
Recomputed counts agree with the frozen metadata reducer and the independent
system/rank/point audit. The eight exclusive phase clocks and reusable/online
cut telescope. Journal time is allocated across that cut. Raw clocks retain
their diagnostic status; all complete-price and admitted-speedup fields are
null. Group calls and their contained arithmetic are not separate additive
primitive prices.

See the [report](../../postprocess/REPORT.md),
[inspected two-page PDF](../../postprocess/REPORT.pdf),
[native implementation and analysis boundary](../../postprocess/README.md),
and [migration reconciliation](migration-reconciliation.json).
No Diem generator, legacy freeze script, EKS job or interrupted study was
restarted. The original panel service, source seals and attempt deadlines
are preserved.

Raw source snapshots, executable, commands and all stdout/stderr remain in
`../../runs/phase2-accounting-v1/`; the artifact-only render and both page
images remain in `../../runs/phase2-accounting-layout-v1/`. They are included
in the pending new Phase 2 archive after panel termination. These local
paths do not claim a published archive. The compact JSON and receipts here
are committed so the historical partial result and implementation gate are
reviewable while the original panel continues.

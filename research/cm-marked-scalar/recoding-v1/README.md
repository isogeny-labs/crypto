# Symmetric modular-representative scan

The single frozen finite scan completed30780 exact schedules on all five curves. The independent native checker recomputed every row and20 selected minimum sets. All200 selected-plan point/count checks passed; a fresh archive extraction produces byte-identical independent replay, and deterministic repacking reproduces the archive digest.

| Curve | Centered direct D+A | Translated direct | Centered fused | Translated fused |
| --- | ---: | ---: | ---: | ---: |
| P-192 | 229 | 228 | 238 | 235 |
| P-224 | 260 | 260 | 276 | 273 |
| P-256 | 300 | 300 | 311 | 309 |
| P-384 | 450 | 449 | 459 | 458 |
| P-521 | 606 | 606 | 613 | 613 |

This is the preregistered unweighted D+A display, not a latency result. All D/A/N/table counts and four declared weights remain in plans.json. The translated fused route remains above the equally optimized direct route on every curve in the primary comparison. Higher-weight sensitivity results also remain above their corresponding direct minima. P-192's translated fused plan uses196 doubles,39 additions,17 negations and9 table points at width5, shift-7. Its centered width5 counterpart uses196,42,20,9: three additions and three negations removed with the same table and doubling counts. The full first-use runtime gain is still unverified.

## Frozen design and custody

Source/checker commit41e44c26a057d9ca6cc72ba0424dca84cfcc1dc1 preceded protocol publicationbb798850daab55182fddc62dee2f2f69fdd695d8. Frozen protocol: ../protocol/recoding-v1/protocol.json, body SHA256328c33e50fe9cc4d6fd55acc3f103b317f93618b3161e022c45d443ff6d0cb3a. It binds both binaries, sources and seven fixture JSON hashes. PR3674 was open before implementation and scan.

Every integer shift-256..256 and width2..7 was scanned for both centered mu and centered mu/ell. Both methods receive the same optimization. Fused costs include the complete width2 multiplication by ell and both point tables. Tie order is score,N,D,A,abs(shift),shift,width. Primary weight1 is decisive; weights2..4 are sensitivities, not measured operation costs. No mu, search bound or criterion was changed after observations.

Software qualification covers1500 boundary cases. The first qualifier found the table counter overwrote its first-call value; the corrected cumulative counter passes. Five post-scan corruptions are rejected at the intended checks: altered selected plan, wrong raw count with updated checksum, missing row, duplicate row, and changed protocol. Build warnings, failed qualification, schema-format errors and the thin summary-serializer typo remain preserved. None triggered a repeated scientific scan.

## Replay and archive

From this directory, verify the archive with sha256sum -c runs.tar.xz.sha256, then extract tar -xJf runs.tar.xz. The local runs directory is ignored; no sealed record is rewritten. Verify sha256sum -c runs-manifest.sha256 here.

From the repository root, build and replay:

~~~sh
make -B -j1 -C research/cm-marked-scalar recoding-binary recoding-replay-binary
/tmp/cm-marked-scalar-build/recoding-replay research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/protocol/recoding-v1/protocol.json research/cm-marked-scalar/recoding-v1/runs/scan/scan.csv research/cm-marked-scalar/recoding-v1/runs/scan/plans.json
/tmp/cm-marked-scalar-build/recoding --qualify-plans research/cm-marked-scalar/fixtures-v2 research/cm-marked-scalar/recoding-v1/plans.json
~~~

Do not invoke --scan into the existing write-once output. Each invocation was limited to120 seconds, one thread/process and2GiB address space. No cloud build/job or unchanged retry occurred. No clock was collected.

The canonical experiment.export/v1 file passes native schema validation. Its80 rows are exact derived minimum cases (five curves x four method records x four weights) from one census, not80 independent process runs or timing trials. Centered controls reuse zero-shift rows. There is no stochastic confidence or population-power claim. A future runtime comparison must retain plan-generation work in the full first-use ledger and include cold runtime/table/map/correction/conversion/verification costs. These counts cannot certify latency.

The archive is approximately91KiB; exact bytes/hashes are in export.json and runs.tar.xz.sha256. plans.json is an unchanged copy of scanner output. The mathematical certificates remain in ../fixtures-v2.

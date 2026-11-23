# Six-arm CPU comparison: completed round18

Round18 completed once as job hashed-ecdl-cpu-round18-20261030,
UID 68845702-fd9d-4940-b64c-67be6a25260c. Its 180 executions include 162
measured rows and 18 warmups; the frozen native analyzer accepted all 18 cells
and byte-identical full certificates. Results remain in results/ and the
inspected report is ../reports/round18/report.pdf. The export's recorded
completeness/file-integrity check has zero findings. These are L0 diagnostics.

The frozen protocol SHA-256 remains
259ecd8eec7d7c17e99fa3f15c52196aa6b07cf33ac1bf0d60ab3c36f98e26a6.
It includes original baseline u/x, hash-enumeration v/w, Curve-context y and
inverse-cache z, across all three fixtures with complete-cost accounting.
Do not resubmit this job or overwrite its export or any historical record.

## Active validation path

Use ../native-panel-tools/README.md for Rust correctness tests and export
packaging. The six analyzer tests and three export tests previously written in
Python have native replacements, plus two additional fail-closed tests. A native
historical parity test regenerates the export in a temporary copy and compares
all measurements, per-run fields, flow counts and artifact hashes; it excludes
only export timestamp and producer notes.

The original Python tests, export and transfer helpers, plus this directory's
pre-native preparation README, are retained under
../native-panel-tools/legacy/round18/. They are historical source only. The
frozen analyzer, protocol, build and source manifests, results and report remain
unchanged. The original literature references remain in protocol.json and
../hash-enumeration/README.md; no new algorithm is introduced by this migration.

T-1016 addresses PR2980's native-language review. No solver or cloud benchmark
is run by these software checks. Round19 is a separate, unsubmitted comparison.

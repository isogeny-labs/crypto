StableRust1.99.1 Cargo release/offline, TMPDIR=tmp, CARGO_TARGET_DIR=tmp/target.
All eight native binaries built without errors; unused-helper warnings retained.
Copied joined-path code is adapted ONLY for mandatory runtime fresh setup,
actual publicB/sourceverification, dynamic constructedHC/QF and orchestration.
Coldprep reuses pinned cold construction routines only through geometry and
pointcount validation; oldmap/cyclicscan functions are not called.

Private msolve dependency is the prior rebuilt officialv0.11.0 commit
cc044551f8e202d00691990fd3fd0b1226a88ea2. Its71test successful build and runtime
provenance are in ../hec_fiberalign13_20261115/SOLVER-BUILD.txt and SOLVER-SOURCE.
All executables, wrapper, private libneogb and FLINT hashes are checked before
scientific execution. No library installs, remote worker or system edits.

Run from study root, after commit/checkpoint/gate:
HEC_MSOLVE=$PWD/../hec_fiberalign13_20261115/tmp/msolve/msolve
HEC_SOLVER_MANIFEST=$PWD/solver.sha256
./tmp/target/release/driver13 PUBLIC.txt run > DRIVER.txt 2> DRIVER.stderr
Both environment variables must be exported. Driver sets each fresh worker's
HEC_SETUP_DIR and HEC_PUBLIC_INPUT; loadernegativecontrol removes the former.

V2 executable hashes are in solver-v2.sha256; keep solver.sha256 unchanged for
V1 provenance. V2 driver uses that new manifest and a fresh run-v2/ directory.
Report rendering: typst compile REPORT.typ REPORT.pdf; per-run PDFs use
typst compile --root . reports/<run>/REPORT.typ reports/<run>/REPORT.pdf.
Both per-run PDFs and results PDF are one page and visually inspected.

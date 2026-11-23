# Root-library v4: eighteen failed rows, source triage

Read-only inspection identified absent runtime prerequisites for every one of the18 observed failed rows: five Certicom device tests require gpu/certicom/emulator.cpp; thirteen ecbench tests require retained record/audit/verdict/bound/rule-v8 fixtures. None of the listed paths is present in private SOURCE or the initial immutable manifest. The Certicom source searches all five source ancestors; no matching emulator.cpp exists at any of them.

All eight affected Rust test source files are byte-identical to published base29861c9a55f9ae5a33e1e09b3866aa397fbde605 and their c5178 manifest entries. This provides a concrete source/materialization diagnosis, not a completed repaired test result. The live stdout currently supplies FAILED rows but has not yet emitted final panic detail. Preserve all18 failures; correlate final native panics before concluding the actual runtime cause per row. C++ compiler availability is downstream of the absent emulator source and was not tested.

Next: first read final native panic details. Only if absent, consider the explicit60-second cached-binary diagnostic proposal; do not rerun a full suite or compile for this diagnosis. Repair requires a separately reviewed immutable fixture extension at the exact base, preserving original failed tree and receipts. The existing binary embeds CARGO_MANIFEST_DIR, so adding a different directory without a reviewed path strategy will not redirect these reads.

Sources checked (KB unavailable): exact eight Rust test modules, runner.rs read_record_lines, immutablec5178 manifest, base29861 git blobs and live failure-output prefix. This is source triage, no experiment or test invocation. No fixture copied, no compiler invoked, no mutable cache read.

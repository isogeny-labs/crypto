# Closeout checks

Producer checks only; no signed independent execution or numerical verdict.
The original closeout release suite passes 26 tests normally and 27 with
`count-field-ops`; the latter adds a counter-specific test and supplied no
timing samples. Subsequent lead checks live in `../quadratic-development/`.
`cargo fmt --all -- --check` and strict all-target clippy pass. PCC prose lint
passes, and the local append-only ledger chain verifies. Claim
`PCC-20261115-66235b` remains unverified, as intended.

`tests.log`, `count-tests.log` and `clippy.log` retain actual command output.
`closeout-source.sha256` hashes the final formatted sources/lockfile;
`derived-results.sha256` hashes the reported data and plot.
`executed-binaries.sha256` hashes immutable measured worker copies and the
actual CAS/replay binaries. Measured source is pinned in the report rather
than inferred from subsequent reporting/formatting changes.

Final source changes after timing add only Plotters/reporting and formatting;
the source-2471e4c worker binary was copied before those changes and never
rebuilt for the recorded confirmation. The report check rejects any unexpected
external failure/status; all 24 supported exact-basis checks match. Archive
hashes were checked locally after packing.

Limitations: API-staged standalone study, no full root-library suite, no
accepted `VALIDATION.json`, no independent signed PCC verifier, no matched-rho
or whole-ECDLP cost. The root session log and divergent PCC ledger prevent a
clean merge with main; neither history has been rewritten to hide that.

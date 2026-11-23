# Development evidence

`development-receipt.json` indexes the observed commands and outcomes. Its
artifact hashes bind the preserved records in this directory. These are local,
unsigned producer observations, not a scientific verifier's signed verdict.
The subsequent PCC statement binds the committed receipt and code revision;
registration alone does not promote any result. [pcc:unverified]

Build manifests and pre-call manifests are separate. Pre-call manifests were
created before invoking separately copied, read-only test or CLI executables.
They bind the binary and build-source manifest. Later environment and hash
checks explicitly say post-call. Builds occurred in the modified worktree at
the tracking commit; file and selected-range hashes bind those actual sources.
The source qualifiers do not establish a successful whole-library build.

The component suite has 16 distinct cases, run at two optimization levels.
The integration sequence has one failed build, one completed build whose test
suite found the visitor defect, and a corrected build/test execution. Keep
all three build attempts and both test outcomes. `integration-attempt2-source`
and `integration-tests-attempt2` name the second **test execution**, which
corresponds to the third integration build. [pcc:unverified]

`failed-fixture.reconstructed.rs` restores the original failing example by
reversing the four recorded callback-fix hunks. Its entire SHA-256 equals the
hash captured in the original build/pre-call manifests. It is a later,
hash-matching reconstruction, not a contemporaneous source snapshot. The
current example remains unchanged.

The complete CLI correctness JSON and larger Cargo JSONL streams use gzip to
stay within the repository's checked-in data-size policy. Decompression
recovers the original bytes. Some smaller `postcall.jsonl` files select Cargo
diagnostics/artifact messages; their matching `raw.jsonl` or `raw.jsonl.gz`
contains the full captured stream. Rust test-harness elapsed times are retained
as raw output, never used as admitted timing or a speed ratio.

`registration-admin-control.postcall.log` belongs to a disposable metadata
fixture. It tests the native statement writer's guards, not ECDLP arithmetic,
solver correctness, a PCC verifier, or signing. No production ledger mutation
was performed by that control.

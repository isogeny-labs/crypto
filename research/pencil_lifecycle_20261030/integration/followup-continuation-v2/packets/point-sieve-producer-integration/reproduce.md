# Reproduction

Fetch integration-evidence.tar.gz using the committed official pointer. Verify the archive and every archive-members.json entry. Its source/ tree contains the exact corrected private producer, engine, locked dependency source and Cargo configuration; test-binary/ holds the compiled real-crate unit-test binary. attempt1/ and attempt2/ preserve complete stdout/stderr and receipts.

The original invocation was timeout --signal=TERM --kill-after=5s 235s python3 -B <packet>/run.py. It failed before tests because one transitive source dependency was missing. The separately authorized corrected invocation used <packet>/successor/run.py under the same envelope. All command arguments and environment values are recorded in command.json and successor/command.json. The frozen source manifests bind compiler/sysroot and vendored sources; no network or dependency update occurred. One exact test and all expected pair IDs and summary fields are required by the wrapper.

The canonical source was copied only after the corrected private check passed and its bytes equal the tested source. canonical-application.json records both hashes. A new replay needs a fresh output directory and its own bounded authorization; do not overwrite or rerun these retained attempts. The test binary is archived as data and need not be executed for custody verification.

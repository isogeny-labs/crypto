# Result collection, 4 November 17:07 UTC

Native source: 507829726fbc731543f4527348b55c5e479242c9. Preparation head
at inspection: 3019171da05b6217539157b1e6c89ba7fc235c5a. Native source files,
manifest and build inputs have not changed between those commits.

The complete E2E harness log reports two passed, zero failed. Its raw bytes
and binary SHA-256 are retained. The former exec handle is unavailable, so
no separately collected Cargo exit status is invented.

Full-library attempt2 is incomplete. Cargo10207 and test10463 are defunct,
the former exec handle is unavailable, and there is no parent summary or
exit receipt. The interruption cause is unknown. The one-pass summary in
the partial log has 5936 filtered tests and belongs to a nested child; it
is not the result of the 5937-test parent suite. Two resource-publication
failures were already observed and reproduced in the preceding receipt.
No suite was restarted, test disabled or diagnostic-clock fallback added.

The raw parent output is preserved as one deterministic raw-run archive
under AGENTS.md section 13a. At 54,340 bytes the archive is below the 64 KiB
artifact-store threshold and is committed with its SHA-256; its 327,961-byte
extracted log remains ignored. This is development evidence, not scientific
measurement data. Earlier larger local archives still await approved upload.

From this directory, check and extract with:

    sha256sum -c runs.tar.xz.sha256
    tar -xJf runs.tar.xz

The extracted log's SHA-256 must equal outcome.json's raw_log_sha256.
The start, filtered and failure receipts in the parent directory remain
unchanged. This outcome supplements them without replacing historical passes
or claiming completion of the full library suite.

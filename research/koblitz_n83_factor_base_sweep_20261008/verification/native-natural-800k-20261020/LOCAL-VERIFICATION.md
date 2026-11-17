# Local archive custody verified; S3 custody blocked

Source was the newly sealed local runs.tar.xz, not a download. All294payload
SHA256 checks pass. Fresh local extraction reproduces both v14 full native
audits canonically, both original v12 native SAT prefixes byte-for-byte and
both expected OOM admission refusals. The original v13 immediate declaration
refusal reproduces with its corresponding exact guard. Worker, all three
immutable guards and public input-panel bytes compare equal. Deterministic
repack from the extraction compares byte-for-byte with the original archive.

The initial replay invocation omitted rg from sudo's PATH. It exited127 after
both full audits and the first OOM prefix/refusal had passed. Its partial
outputs and log remain in local-verification/ and local-verification-initial.log.
The corrected invocation explicitly inherits PATH and uses a fresh write-once
local-verification-v2/ namespace. It completes all checks. No timed search was
repeated by this verification correction. See local-verification-status.txt,
local-payload-checks.log, local-repacked.sha256 and the v2 replay receipts.

This establishes local extraction custody only. Full-bundle native store
publication stopped at HeadObject403; direct conditionalPutObject of the exact
archive was denied s3:PutObject. No successful upload or fresh S3 GET occurred.
Original absolute worker/input paths remain prerequisites for exact native replay.

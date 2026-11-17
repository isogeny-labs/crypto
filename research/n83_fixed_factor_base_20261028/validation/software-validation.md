# Software and delivery checks

The standalone native generator's four unit tests and container self-check passed.
Every produced shard was read back and verified for every candidate x; all384
certificates report verified_all_candidates=true. SHA-256 verification of all780
raw-manifest entries passed. The protocol/export checker reports zero findings.

All384 content-addressed point objects uploaded with server-verified SHA-256.
The artifact remote checker reports384 hydrated entries and zero problems.
The first and last shard of every family were downloaded into a separate scratch
directory from the content-addressed store and checked by SHA-256 and the native
candidate verifier (remote-readback.log). This is six full remote readbacks,
not a claim to have downloaded all384 objects again.

Repository cargo test --release --lib completed4938 passing,28 failing,122 ignored.
Failures were missing sparse-checkout runtime fixtures and two GPU-lock tests.
Exact tracked HEAD fixture blobs were restored, with no test/source edits. The
28 failing tests were run serially:24 passed and4 still lacked audit fixtures.
After restoring those two audit JSONs, all4 remaining tests passed. See
root-rechecks.log and root-rechecks-final.log. The full suite was not rerun.
The original complete root test log is archived separately in the shared bucket.

The two-page PDF was rendered with Typst and both pages visually inspected.
No independent scientific review or degree-of-regularity rerun is claimed.

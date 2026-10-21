# Exact source history

SOURCE-COMMITS.bundle retains full commits after published prerequisite
37679b3586992e52e44cd1e994f589aa2a9b204b through report/source snapshot
8e892fc37c29c91b3cb525bad71ed58407cea9db, including:

- 674d2075958c5cb388cbfcb271e8ebcd6dea646c: exact instrumented worker/guard-v21
  source for K2048; opt-in capacity/RSS/allocator telemetry and native checker.
- 7e8ae96c4af60ac63311b4c3c29efcb83321861e: exact queued guard-v22 for K4096;
  unchanged frozen worker bytes, bounded600-second lock queue before reservation.
- 8e892fc37c29c91b3cb525bad71ed58407cea9db: completed diagnostics, full release
  checks, inspected report/graphics and problem/benchmark updates.

Verify with git bundle verify. Restore the prerequisite checkout then fetch
this bundle into a fresh branch. Raw SOURCE.bundle contains this same ancestry.
Verification sidecar SOURCE-PUBLICATION.bundle additionally retains post-raw
publication metadata. Final upload/remote snapshot receipts remain external to
their own archives. No experimental source ID or old1M evidence is rewritten.

Final verification sidecar SHA256
fdf07b855f097ecb3afb5a331ae3440b43746ff2a4e53ff588d06cdec135f76e additionally
contains SOURCE-PUBLICATION.bundle through the raw-proof commit recorded in
sidecar-proof/source.txt. Both source bundles verify from fresh downloaded bytes.
Its own final upload receipts and six-name index promotion are external metadata.

Concurrent main merge05ec85b5287ebf137f1b1e16c8e91705d48cf77f was then preserved
in combined source d5aac8e9f4ef313dfea310a712fdeca8b96b3d8a. Supplemental
SOURCE-INTEGRATED.bundle is in S3 integration archive99aeb0c3, requires05ec85b5,
and retains all local diagnostic ancestry plus the combined code. Fresh source
verification/repack passes. Integrated4481/119 release checks are separate from
frozen4471/119; native SAT/guard source is unchanged. Metadata receipts/index
promotion remain external. See INTEGRATION.md and dated report clarification.

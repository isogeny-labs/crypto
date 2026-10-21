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

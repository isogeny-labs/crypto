# Standalone reproduction and artifact locations

Run `rustc +1.98 --edition=2021 --test harness.rs -o /tmp/dynamic-boolean-scheduler-tests`, then `timeout 60 /tmp/dynamic-boolean-scheduler-tests --test-threads=1`.

This research component is not linked into the library or dataset CLI. Production library source is unchanged; the standalone result is not an integrated library/CLI acceptance.

MANIFEST.json is the immutable original proposal manifest. Every listed source/log/document is copied unchanged here. The 7,745,712-byte scheduler-tests binary is intentionally stored in the S3 archive named by s3.receipt.json, under proposal/scheduler-tests. Download the archive, verify its outer SHA-256 and contained SHA256SUMS, then check the proposal manifest and test receipt. The independent-review.md and this reproduction guide are publication additions outside the original proposal manifest.

# N83 fixed factor base points: 2^27 domain

The point-set catalog is in [`factor-base-point-catalog-20261111.json`](factor-base-point-catalog-20261111.json). It contains direct content-addressed S3 URIs and SHA-256 digests for each factor-base manifest and each compressed point shard, plus selector ranges and point counts.

Each complete base covers all 134,217,728 selector indices for one coordinate-bit variant and one curve family. The same point set is reused for every `m` in 2–9. The catalog binds twelve complete variants (`r=27–33` and `r=35–39`), with three families per variant and 128 shards per family: 36 family bases and 4,608 point-shard objects. Each shard's source record reports complete candidate validation.

The listed point shards total 54,571,967,820 compressed bytes, excluding manifests, receipts, and other run files.

No complete three-family 128-shard point set for `r=34` was found in the reconciled attempt tree. Its partial and failed artifacts remain in place. The `r=27` and `r=28` points are cataloged from their source point indexes; this catalog does not replace those original records.

For each family, the JSON catalog provides a S3 URI for `manifest.json`; its 128 shard records provide the point-object URIs directly. The local source manifest and S3 index paths are included for provenance. Read each compressed shard and verify its listed SHA-256 before use. The stored point set uses the frozen `n83.factor-base/v2` representation and retains all affine lifts, including both signs when there are two.

This catalog is an index to objects already published by the preserved factor-base runs. It does not copy the point payloads into Git or perform another S3 write. The N83 research account is over its USD 1,000 monthly spending cap, so new S3 writes and cloud jobs remain paused. These bases provide factor-base inputs only; this catalog does not contain new degree-of-regularity measurements or trained-model results.

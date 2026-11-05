# r39 factor-base enumeration recovery

The existing EKS Job succeeded at 2026-11-01T20:12:14Z. Its immutable S3 file index is retained as `s3-file-index.json` (SHA-256 `0baee0b147a21fb2921d3fb897598c5596ce3defdd61e6321984ad677f052b30`), schema `n83.factor-base-s3-file-index/v2`. The index enumerates 786 producer files. All 402 non-compressed outputs (3,714,663 bytes) are restored under `restored/` and SHA-256 verified.

The 384 compressed point-data shards (4,575,883,253 bytes) remain in S3; `factor-base-shards.json` records each path, byte length, SHA-256 and immutable snapshot key. Each of the Koblitz, binary and prime families has 128 shards covering all 2^27 selectors; each complete base is reused unchanged for m=2..9.

`r39-recovery-audit.json` verifies contiguous ranges, producer receipts, per-family manifests and their joins to the immutable index. The original `FORMAT.json` says spike40/bit40, but the index, all three family manifests and sampled N83FB02 headers say spike39/bit39; that stale producer descriptor is preserved unchanged.

# r38 factor-base enumeration recovery

The original Job completed at 2026-11-01T19:52:39Z. Its immutable S3 file index is retained as `s3-file-index.json` (SHA-256 `f990221ecd91d7a7a8bc3eadf89529aec5f06c7ed8fa3cad40f31dd245337765`), schema `n83.factor-base-s3-file-index/v2`. The index enumerates 786 producer files. All 402 non-compressed outputs (3,715,017 bytes) are restored under `restored/` and SHA-256 verified against the index.

The 384 compressed point-data shards (4,576,000,923 bytes) remain in S3; `factor-base-shards.json` records each relative path, exact byte length, SHA-256, and immutable snapshot key. There are 128 Koblitz, 128 prime, and 128 binary shards. The dataset covers the fixed 2^27-selector domain for each curve family and reuses each same base for m=2..9, bound by protocol SHA-256 `7276bd557a5625c358a4d063366a415a0265c078ffb2eddaa74dee40174529b2`.

This is enumerated candidate factor-base point data plus its producer/format metadata. Degree-of-regularity, FFD/LFD consequences, and model leads require separate analyses and are not asserted by enumeration.

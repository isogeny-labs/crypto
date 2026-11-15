# r37 factor-base enumeration recovery

The EKS Job succeeded at 2026-11-01T18:26:29Z. Its producer emitted a complete S3 file index listing 786 output files. The index was downloaded from the immutable attempt-01 S3 index path and used to restore and SHA-256 verify every non-compressed file. The 384 compressed factor-base shards remain in S3 and their byte counts and SHA-256 hashes are recorded in the producer index and family manifests. The base covers 2^27 selectors per curve family and is explicitly reused for m=2..9.

The raw S3 file index is retained as `s3-file-index.json`; `restored/` contains all other non-compressed producer outputs in original relative paths.

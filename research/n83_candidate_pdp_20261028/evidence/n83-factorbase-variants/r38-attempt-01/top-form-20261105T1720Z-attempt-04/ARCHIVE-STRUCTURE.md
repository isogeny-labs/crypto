# Factor-base metadata archive

`factor-base-metadata.tar.gz` preserves the extracted selector metadata as a directory tree. The root contains `s3-file-index.json` (immutable object key/size/SHA-256 map), `r38-recovery-audit.json`, `STRUCTURE.json`, and `data/index.json`. `data/binary`, `data/koblitz`, and `data/prime` contain recovered `part-NNNN.json` selector records. The 384 compressed `.zst` point shards are separate immutable S3 objects named by `s3-file-index.json`; this archive does not duplicate their 4,576,000,923 bytes.

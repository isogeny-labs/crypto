# r35 completed 2^27 factor-base enumeration

Job `n83-fb-variants-r35-20261031-r1` completed under the frozen coordinate-variant protocol v9. Its immutable S3 output prefix is `s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r35-20261031-r1/`. The final content-addressed S3 file index has SHA-256 `d0b78cab215c8dd6467786ddf3bde24e84183cb67500e7cd6c0f3e448ed27bf7`, covers 786 files / 4,579,502,257 bytes, and is retained here as `s3-file-index.json`. The complete paginated S3 object inventory is `s3-object-listing.json` (793 total objects, 8 pages).

The output has three complete family bases, each enumerating 134,217,728 selector/x candidates, divided into 128 contiguous 2^20 shards. Metadata readback restored and rehashed all 402 non-compressed payloads (3,710,979 bytes), including all 384 shard manifests; the 384 compressed shard payloads remain in S3. The S3 list matched every indexed snapshot exactly: 786 expected, 786 present, no size mismatch, no extra snapshot, and each object key ends in the indexed SHA-256. `structural-audit.json` independently checks the complete coordinate map, contiguous shard ranges, manifest links, all candidate-verification flags and m=2..9 reuse.

| Family | Liftable x values | Affine points in fixed base | Compressed bytes |
|---|---:|---:|---:|
| Koblitz | 67,104,692 | 134,209,383 | 1,363,109,867 |
| General binary | 67,106,732 | 134,213,463 | 1,363,147,709 |
| Prime | 67,108,346 | 134,216,692 | 1,849,533,702 |

`restored/` mirrors every non-`.bin.zst` payload under its original relative path. `verify_s3_metadata.py` reproduces the content readback and hash check; `structural_audit.py` reproduces structural checks. `metadata-readback-summary.json` and `structural-audit.json` hold their outputs. `submission-preflight/` retains the frozen protocol binding and exact wrapper/runner inputs. `status-tool.json` and `logs-tool.json` retain the terminal EKS evidence. `listing-chunks/` are lossless transport slices that were recombined into the complete S3 inventory.

This is factor-base enumeration and custody evidence. It does not measure FFD, LFD, PDP degree of regularity, solving degree, or solver cost.

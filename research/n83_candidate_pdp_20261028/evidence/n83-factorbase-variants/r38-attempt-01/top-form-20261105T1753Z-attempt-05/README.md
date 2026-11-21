# N83 spike38 top-form attempt 04

Attempt 05 continues the frozen 48-system spike38 panel under the V2 scientific design. Attempt 03 is preserved and closed: it exited 1 before input systems or top certificates were emitted. Its exact failing shell command was not captured. The continuation amendment changes only operations and failure capture, not systems, factor bases, seeds, controls, metric, or analysis.

The base is staged from the recovered selector metadata (`factor-base-metadata/data`) and the 384 immutable compressed shards listed by `factor-base-metadata/s3-file-index.json`. Each shard is checked against its size and SHA-256. The recovered family directories contain selector JSON parts; compressed shards are separately fetched from S3. Binaries are reused from attempt03 build-validation-v1 after exact hash checks.

The controller pins all descendants to one allowed CPU, sets the 8 GiB address-space limit, allows four concurrent S3 shard fetches, and sets `AWS_MAX_ATTEMPTS=1`. The run has a 12,850-second wall cap plus 45 seconds termination grace, one attempt, zero retries, USD 2.90 compute cap, USD 0.25 artifact cap, USD 3.15 total. Outputs go to a new S3 `results/` prefix. Every partial output, error line, terminal receipt, file layout, hash manifest and S3 URI is retained.

The reported metric remains top-form quotient Dreg with replay. This panel does not label affine FFD/LFD or solver complexity.

A live S3 inventory check matched all 786 indexed snapshot objects and 4,579,715,940 total bytes; among them are 384 compressed shards totaling 4,576,000,923 bytes.

Attempt 05 uses a fresh output prefix. Its pinned protocol SHA-256 is checked and recorded in protocol-hash-check.json before shard download. Attempt 04 is retained separately with its failure reconciliation.

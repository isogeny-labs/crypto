# r27 attempt-07 producer index and shard-object verification

The completed producer index describes candidate id `spike27`, selector domain size 2^27, moved coordinate bit 27, and reuse of the exact same family base for m=2..9. It reports 134,217,728 candidate x values per curve and records measured affine lift and point counts for Koblitz, general binary, and prime families.

The S3 point payloads are the 384 content-addressed `part-NNNN.bin.zst` objects under `s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r27-20261031-r7/attempt-07/snapshots/`. `shard-object-verification.json` joins each of the 384 manifest path/hash/size entries to a listed S3 object key; every expected key is present with the expected compressed size. The native manifests mark every candidate as checked. The underlying run has producer exit code 0. The outer Kubernetes Job is still active while the runner's final whole-output index and terminal condition are pending.

This is point enumeration and custody evidence only. It contains no matched PDP systems and does not measure FFD, LFD, solving degree, or DoR.

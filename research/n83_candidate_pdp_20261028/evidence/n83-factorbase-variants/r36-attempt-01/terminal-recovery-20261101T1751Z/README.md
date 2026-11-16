# r36 factor-base enumeration recovery

The r36 EKS Job ended with `BackoffLimitExceeded` after its producer had completed. The producer's own S3-restored run manifest reports exit code 0, the factor-base index says `complete=true`, and the saved validation marker is `true`. Recovery matched all 784 expected producer files to 784 immutable S3 snapshots with no missing or extra paths. It restored and SHA-256 checked 400 non-compressed files totaling 2,982,883 bytes. The 384 compressed shards remain in S3 and each matches the size and SHA-256 in its recovered family manifest. Each of the three family manifests verifies 128 contiguous 2^20 ranges over the 2^27 domain and marks every candidate range verified. All families record same-base reuse for m=2..9.

`recovered-s3-file-index.json` is a custodian recovery map over original attempt-01 snapshots plus a byte-deterministically reconstructed `manifest.sha256`. The producer did not emit its `upload-history.jsonl` or final `s3-file-index.json`; neither is fabricated. `s3-object-listing.json` and `listing-chunks/` preserve the full failed-job inventory. `restored-noncompressed/` holds all 400 independently restored small files (2,982,883 bytes) in their original relative layout; `recovered/` holds the seven producer artifacts used for the first audit plus a duplicate reconstructed manifest. `FILE_LAYOUT.json` describes the recovery package structure; `RUN_OUTPUT_FORMAT.json` documents the producer layout and hash contract.

## Family totals

| Family | Liftable x | Points | Compressed bytes | Shards |
| --- | ---: | ---: | ---: | ---: |
| Koblitz | 67,110,424 | 134,220,847 | 1,363,224,705 | 128 |
| Binary | 67,111,066 | 134,222,131 | 1,363,232,632 | 128 |
| Prime | 67,106,953 | 134,213,906 | 1,849,506,935 | 128 |

Total points: 402,656,884. Total compressed payload: 4,575,964,272 bytes.

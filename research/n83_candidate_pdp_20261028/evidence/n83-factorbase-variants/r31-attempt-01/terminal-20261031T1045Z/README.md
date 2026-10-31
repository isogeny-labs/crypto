# r31 fixed-base S3 terminal and structural audit

Job `n83-fb-variants-r31-20261031-r1` reached producer success (exit code 0 in the saved run manifest). Its exact terminal S3 index was restored and SHA-256 checked as `dd0ef2be0d8af01e7842c047cdb1639fa426bdffd952428214352cefcd0004b0`. All 786 entries were reconciled against the authoritative S3 snapshot listing by exact object key, byte size, and content-address suffix; failures: 0.

The complete point-data index reports `complete=true`, coordinate bit 31, 134,217,728 candidate x values per curve, and reuse for m=2..9. Each of the three family manifests contains 128 shards. The 384 shard records all report `verified_all_candidates=true`. Family point counts are binary 134,222,463, koblitz 134,212,379, prime 134,210,276.

The S3 index lists 4,511,872,774 total bytes. The original full compressed payload remains in the producer's immutable S3 snapshots. This Git directory retains the complete S3 file index and the structural manifests needed to inspect every shard; it does not duplicate the multi-gigabyte payload locally. Structural files were individually restored and byte/hash checked against the index.

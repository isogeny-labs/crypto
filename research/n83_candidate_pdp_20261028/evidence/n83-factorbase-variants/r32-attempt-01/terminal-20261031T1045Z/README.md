# r32 fixed-base S3 terminal and structural audit

Job `n83-fb-variants-r32-20261031-r1` reached producer success (exit code 0 in the saved run manifest). Its exact terminal S3 index was restored and SHA-256 checked as `26f010c6a90ae371554c433c271b09415226530b0bbb0757ae7dc0c97d6699b2`. All 786 entries were reconciled against the authoritative S3 snapshot listing by exact object key, byte size, and content-address suffix; failures: 0.

The complete point-data index reports `complete=true`, coordinate bit 32, 134,217,728 candidate x values per curve, and reuse for m=2..9. Each of the three family manifests contains 128 shards. The 384 shard records all report `verified_all_candidates=true`. Family point counts are binary 134,208,179, koblitz 134,199,695, prime 134,219,630.

The S3 index lists 4,579,421,694 total bytes. The original full compressed payload remains in the producer's immutable S3 snapshots. This Git directory retains the complete S3 file index and the structural manifests needed to inspect every shard; it does not duplicate the multi-gigabyte payload locally. Structural files were individually restored and byte/hash checked against the index.

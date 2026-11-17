# r33 fixed-base S3 terminal and structural audit

Job `n83-fb-variants-r33-20261031-r1` reached producer success. Its terminal S3 index SHA-256 is `833fa3af50fe6303cd0667b2f04248509dfbbdc80a3f857e766f77fea08afcce`. The index has 786 entries. All 786 indexed snapshots matched the authoritative S3 object listing by exact key, byte size, and SHA-address suffix; failures: 0.

The data index reports `complete=true`, coordinate bit 33, 134,217,728 candidate x-values per curve, and reuse across m=2..9. Each family has 128 shards; all 384 shard records report `verified_all_candidates=true`. Point counts: binary 134,232,711, koblitz 134,211,085, prime 134,203,076. Total indexed bytes: 4,579,547,047.

All immutable compressed shards remain in the original S3 result prefix. This Git bundle keeps the full file index and structural files needed to reconstruct the shard map.

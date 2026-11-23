# External Mathlib cache availability after finite-history qualification

The exact focused compiler import closure is restored and hash-checked.
455 private-cache entries totaling122,739,848bytes were restored. This permits
the standalone history reproduction. Other Mathlib/editor dependencies remain
compacted; a broader build may require restoring the remaining exact bytes.
Preflight fixed memory and the existing physical guard before restoration or
starting a Lean worker. Do not treat unused cache availability as proof failure.

All bytes remain in these three local archives, previously fully compared and
rehashed before removing only duplicate uncompressed external-cache copies:

| Archive under /tmp/ecdlp-precision-audit-20261123 | SHA256 |
| --- | --- |
| cache-compaction/mathlib-editor-indexes.tar.zst | 397490d1832c0493b5d047c013adb8c6ccf76ee036450a80da1f640f5d91fbee |
| private-cache-compaction/mathlib-private-cache.tar.zst | ab140b503318036e9e30e50a2451370d883cbdf414eab5f4f8ed91b4ddb1abe3 |
| history-unused-cache-compaction/mathlib-private-cache.tar.zst | 1149b1366f5f363be980608ab07dfa13827b49970552056ff6ee0650409763fb |

The last archive contains1001 files/383,844,664uncompressed bytes outside the
inventoried history compiler import closure. Restoration directory for each:
/dev/shm/ecdlp-proof-check-20261123/mathlib-lean-libs.

After memory preflight, use standard `tar --extract --zstd --file=ARCHIVE
--directory=RESTORATION_DIRECTORY` for the needed archive/entries and verify
against its retained file manifest. Re-extracting already restored entries is
unnecessary; all archived bytes are the original pinned dependency cache.
No owned source, raw research, partial proof output/checkpoint or native binary
was selected for compaction. No Amazon S3 storage or additional capacity is used.

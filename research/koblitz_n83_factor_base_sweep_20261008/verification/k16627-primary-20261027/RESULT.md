# Primary N83 K16627 construction and replay

The frozen `a=0`, `public_x_hash`, seed `2026100801` construction produced
**16,627 signed-Frobenius orbit columns and 2,760,082 distinct point
records**. Generic multi-limb replay passed for every point and all 16,627
representatives, checking subgroup projection, Frobenius labels and orbit
closure. The 83,701,045-byte compressed object has BLAKE3
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`
and SHA-256
`2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369`.
The point-set BLAKE3 is
`564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`.

The [protocol](PROTOCOL.md) pinned the same exporter source commit as the
K4096 control, `f61b33c1b1bb98efa22177a615270c160859a605`. Its source
SHA-256 is
`cf31a5e9e8d5aad35aeb3c73be352992bc4e40ca6cbd425ad0ccf8e14de89340`.
The Linux arm64 exporter built with `cargo 1.93.1 --locked --release` has
SHA-256 `41c968ee8d82d085b8f9eca03ddcbfed69805e415408f36ddc1bcc58ccdeb898`;
its build log remains at
`/Volumes/SSD990-2/.codex/n83-k16627-build-20261027/build-f61.log`.
The container image ID was
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.

| Stage | Status | Observed wall | Cap and envelope |
| --- | --- | ---: | --- |
| One-object construction and Docker cleanup | `PASS_construction_only` | 83.929 s outer; 32.412 s worker | 3,600 s; one CPU, 6 GiB hard memory, zero swap, no network |
| Full generic replay | `PASS` | 28.21 s outer | 1,800 s; same Docker resource envelope |
| Fresh S3 GET and independent compressed-byte hashes | `PASS_existing_object_match` | Not timed as IC work | 83,701,045 bytes; SHA-256 and BLAKE3 match local object |

The exact object already existed at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz`.
S3 reports a 17 October 2026 last-modified time, AES-256 encryption and the
matching BLAKE3 metadata. Today's fresh GET matched the local construction
by SHA-256, BLAKE3 and length; HEAD metadata and ETag remained identical
before and after GET. The existing object was preserved rather than
overwritten. Today's manifest and replay receipts are retained in
[evidence](evidence/); the complete object and raw build/run logs remain in
`/Volumes/SSD990-2/.codex/n83-k16627-construction-20261027-01`.

This is an exact construction and replay result. It does not rank K16627 by
end-to-end IC time. The K4096 construction used a different binary and
resource envelope, so its 233.702-second outer wall is not a matched
performance comparator. The K16627 manifest records null relation yield,
matrix rank, scalar verification, `total_index_calculus_runtime_ms`, and
`selected_best_total_runtime`. The next gate is a source-pinned natural
relation/rank search followed by the required cold and same-target online
comparison.

The current parent branch head `af3af47900e064896837a4ae174ec0862ae2dccb`
failed a local Linux release build before this run: Rust reported an expected
expression at `koblitz_index_calculus.rs:9262` and an unexpected closing
delimiter at `sat.rs:2807`. The frozen exporter source was unchanged, so the
construction used the K4096 control's known-good commit. The current-head
build failure is a source integration gate for later pipeline runs, separate
from the verified object.

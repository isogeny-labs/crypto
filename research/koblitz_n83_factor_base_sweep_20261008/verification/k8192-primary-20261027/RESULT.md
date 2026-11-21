# Primary N83 K8192 construction, replay and S3 custody

The frozen `a=0`, `public_x_hash`, seed `2026100801` construction produced
**8,192 signed-Frobenius orbit columns and 1,359,872 distinct point
records**. Generic multi-limb replay passed on every point and all 8,192
representatives, checking subgroup projection, Frobenius identity, point
labels, orbit closure and the point-set digest. This completes object
construction at all five added primary size-frontier choices under the
declared policy and seed.

The compressed object is 41,238,219 bytes, with BLAKE3
`43419f2ec08e1a7f15bcfb9dfcceac6ca386869a0eea0bb04d3934af621b3f8d`
and independently computed SHA-256
`d57d523128b02f68fa97e43107c6158060d2d9fcc5c6ea5e43431e8d324602d1`.
The point-set BLAKE3 is
`47f1c0a8c3ca113806a6d150c53608cc9edef4a841aaaccc915f39c140179bcf`.
S3 HEAD returned 404 before upload. The native uploader's download check
passed, and a separate fresh GET matched byte count, BLAKE3 and SHA-256.
The versioned object URI is
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/43419f2ec08e1a7f15bcfb9dfcceac6ca386869a0eea0bb04d3934af621b3f8d.jsonl.gz`.
S3 reports AES-256 encryption and matching BLAKE3 metadata.

The [protocol](PROTOCOL.md) pinned source commit
`f61b33c1b1bb98efa22177a615270c160859a605` and source SHA-256
`cf31a5e9e8d5aad35aeb3c73be352992bc4e40ca6cbd425ad0ccf8e14de89340`.
The Linux arm64 constructor has SHA-256
`41c968ee8d82d085b8f9eca03ddcbfed69805e415408f36ddc1bcc58ccdeb898`.
The native macOS arm64 uploader has SHA-256
`5881dc22b3367b6076ef5c3d452eb270bfa3c4013ec63782a0276b4670cdc094`.
The Docker image ID was
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.

| Stage | Status | Observed outer wall | Cap and envelope |
| --- | --- | ---: | --- |
| Construction and Docker cleanup | `PASS_construction_only` | 59.029 s | 1,800 s; one CPU, 6 GiB hard memory, zero swap, no network |
| Full generic replay | `PASS` | 13.25 s | 1,800 s; same Docker resource envelope |
| Native upload and download check | `PASS` | 12.24 s | 600 s |
| Independent fresh S3 GET and hashes | `PASS_fresh_upload_and_get` | Separately retained | Byte count, SHA-256 and BLAKE3 match |

The [evidence directory](evidence/) contains the config, construction and
replay receipts, uploader receipt, fresh S3 HEAD and hash comparison. The
complete compressed object and raw logs remain in
`/Volumes/SSD990-2/.codex/n83-k8192-construction-20261027-01`.
These are construction and storage resource observations. Natural relation
yield, useful rank, final matrix work, target recovery and complete IC
runtime remain unmeasured for this K. The manifest retains null
`total_index_calculus_runtime_ms` and `selected_best_total_runtime`; a
ranked factor-base choice requires source-pinned natural searches and the
fully charged same-point IC/rho comparison.

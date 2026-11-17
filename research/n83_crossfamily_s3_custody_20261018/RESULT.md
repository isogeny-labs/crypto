# Four-family N83 factor-base custody

The frozen [plan](plan.json) (SHA-256
`9b221e6d61dab95689e88a21e72f950f8b9a551be9ea671bba0ddf4e9d2cd343`)
binds the four point-defined bases from GitHub Actions run
[`39075872898`](https://github.com/aburan28/crypto/actions/runs/39075872898),
attempt 1 at source commit `577c8cbe9ddfa20a2b6a62d8c39eb1dd339497ed`.
The [provider download receipt](receipts/provider-artifacts.json) binds all
four artifact IDs and ZIP hashes to that run and source; its artifact rows
match the plan exactly.
This is the degree-83, a=1 diagnostic curve with a 53-bit subgroup.

The [build receipt](receipts/build.json) checks all 24 archived raw copies per
base (six per independent stratum), exact candidate manifests, curve identity,
point-array cardinality and uniqueness, then deterministically compresses one
copy. The [independent audit](receipts/audit.json) reports
`PASS_FOUR_FRESH_S3_ROUNDTRIPS`: it checks the archived copies again and
matches fresh S3 GET bytes, decompressed bytes, SHA-256, BLAKE3, single-part
ETags, metadata, and point arrays. The [inventory](receipts/inventory.json)
contains exactly the two new high-K object keys under this study's prefix.
The [mutation control](receipts/mutation-control.json) flips one bit in a
copied K3450 fresh download; the verifier exits nonzero before emitting a
passing audit.

| Base | Orbit columns | Points | Storage | Content-addressed S3 object |
| --- | ---: | ---: | --- | --- |
| root11 K2816 | 2,816 | 467,456 | Reused, exact byte identity | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_root11_tier1_20261018/a1/objects/a4a048d9f26051ed4de64f0684bfdaf1471b2548467eb990bbbd2c1c1f690b93.jsonl.gz` |
| root11 K3072 | 3,072 | 509,952 | Reused, exact byte identity | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_root11_tier1_20261018/a1/objects/17b4ab1d1c5dbcd5a673deecd3d31bf987bd74d207d194c2518a6d8c5ad696f8.jsonl.gz` |
| wide3 K3450 | 3,450 | 572,700 | New, conditional PUT | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_crossfamily_cancel_20261018/a1/objects/96c4f700c58189e44ae9e187a68836bd250b53f26cddb9c815180c1abefd7d7f.jsonl.gz` |
| root4 K3500 | 3,500 | 581,000 | New, conditional PUT | `s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_crossfamily_cancel_20261018/a1/objects/c0cec5f5cc985c698d9e3648701f2660fa83bda09d6457da34758042d1949785.jsonl.gz` |

The two new uploads used `If-None-Match: *` and AES256 server-side encryption.
A repeated conditional PUT to K3450 returned `PreconditionFailed`; a fresh
HEAD retained the expected ETag, size and metadata
([receipt](receipts/no-overwrite.json)).
The compressed and raw objects, provider ZIPs, and fresh downloads remain in
local `artifacts/` for replay; their digests and S3 API receipts are committed
under `receipts/`. This custody result identifies stored bases. Runtime and
selection claims belong to the separate cross-family holdout and its frozen
selector. `selected_best_total_runtime` remains null in this study.

Validation on this feature branch: `py_compile` passed for both custody
programs; the N83 Python discovery suite passed 63/63 tests
([receipt and full log](validation/n83-python.json)); and
`cargo test --release --lib -- --test-threads=8` passed 4,085 tests, with
117 ignored and zero failed in 931.66 seconds
([receipt and full log](validation/cargo-release-lib.json)). Both raw and
deterministically compressed log hashes are recorded. No Rust source or
examples changed in this custody round.

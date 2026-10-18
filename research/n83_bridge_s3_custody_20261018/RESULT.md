# K1107 and K2304 bridge factor bases stored in S3

The two exact point-defined factor bases from the completed first stratum of
[native bridge run 39096257170](https://github.com/aburan28/crypto/actions/runs/39096257170)
are stored in the requested S3 prefix. The source artifact is
`n83-native-bridge-stratum-0-39096257170-1` (ID `12126920530`), with ZIP
SHA-256 `f3df5bc8a996fdf432d9f70a42c8d7f58b54a56f59eb2fa8fc7eaead136de1bb`.
Its completed job, source commit and artifact digest are frozen in the
[plan](plan.json) and checked against the saved GitHub API metadata. The
provider source commit is `77c8ebe307c047c78a7f66f6a7f41a0efef60584`;
the plan SHA-256 is
`514a698d61146333d539699ca8ed193890ad84bec18ac9480c62cb080b99482f`.

| Base | Signed-Frobenius columns | Points | Raw SHA-256 | Compressed BLAKE3 object key | Compressed bytes |
| --- | ---: | ---: | --- | --- | ---: |
| K1107/root11 | 1,107 | 183,762 | `08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc` | `1548c6cf720d79c931f5279061d344195b94b48cfe7c30ff788474cbc20381a2.jsonl.gz` | 4,387,310 |
| K2304/root11 | 2,304 | 382,464 | `f51e436c399688ebd710075f16a57cd631a6799ce79891d53250b46bfd6c4534` | `22725942579360448b03b626132d665cbdc80d3308a79cb81ba00caaab74f817.jsonl.gz` | 9,135,776 |

The complete S3 URIs and all eight provider-member hashes per base are in
[build.json](receipts/build.json). Both writes used conditional
`If-None-Match: *` PUTs. Fresh GETs passed the independent [audit](receipts/audit.json)
as `PASS_TWO_FRESH_S3_ROUNDTRIPS`, including compressed and decompressed
hashes, point arrays, lengths, ETags, metadata and candidate-manifest hashes.
The [live prefix inventory](receipts/inventory.json) contains these exact two
keys. A second conditional PUT to K1107 was rejected as `PreconditionFailed`,
and a one-byte change to a copy of its fresh GET was rejected by the verifier.

The separately frozen [all-copies audit](receipts/all-copies-audit.json)
checked every point-specific factor-base dump in all four completed bridge
archives: **32 exact copies of K1107 and 32 of K2304** match the fresh S3
base bytes. The four provider ZIPs are themselves stored at write-once,
content-addressed S3 keys and passed full fresh-download replay in the
[bridge provider audit](../n83_small_intermediate_bridge_20261018/results/39096257170/provider-audit.json).

These bases belong to the diagnostic N83 `a=1` curve with a 53-bit subgroup.
The primary `a=0` factor-base sweep is a different workload. This custody
round measures byte identity and storage only. The bridge timing panel is
complete, with separate online and cold choices among its two a=1
candidates. Neither promotes the primary `selected_best_total_runtime`,
which remains `null`.

# Retained N83 K256 compact-root capacity result

The frozen `a=0` K256 public-x-hash base at seed 2026100801 was fetched
again from its existing S3 object. The compressed bytes matched BLAKE3
`0b06b75e26cff57a62c28c2502c85f3fc3f1a5027e5620d90969ecae0947b045`,
the manifest row, replay receipt, S3 upload receipt, and fresh HEAD/GET
length and ETag. The static Linux worker binary had SHA-256
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`
and was built before this rung; its build provenance is retained with the
[K64 result](../compact-retained-k64-20261018/RESULT.md). The supervisor
bound the run to clean source commit
`af8b3703bf7905a2de4724bc12a4116ca23d3b48` and snapshotted the three
relevant Rust source files. The live container inspection recorded one CPU,
a 4-GiB hard memory cap, zero swap, no network, and read-only input mounts.

The independent [`audit.py`](audit.py) returned
`PASS_CAPACITY_ONE_QUERY_MISS`. The worker imported the exact factor base in
667.500 ms, validated the public target in 1.947 ms, and completed all
2,730,368 unordered candidate states. All 2,730,368 were regular states,
with 5,438,720 root-table entries. The single point probe took 675,192.805
ms and the full supervised process took 676,903.332 ms; cgroup peak memory
was 723,492,864 bytes. Exit code was zero, stderr empty, and the container
was gone after completion. The same auditor rejected a copy whose worker
status was changed from `MISS` to `HIT`; see
[`evidence/mutation-receipt.json`](evidence/mutation-receipt.json).

This was one public point query and it returned `MISS`. A relation row,
useful rank, linear algebra, column logs, and complete cold IC timing were
not measured. The 1,800-second cap was an upper bound, while 676.903 s is
the observed container-supervised wall charge for this successful query.
The prior cross-build and fresh S3 preparation are separate setup phases.
Docker executed an x86-64 binary on an ARM host, so this timing is a
capacity observation rather than a native hardware ranking. Under the
separate fresh-uniform-target model in `SUPPORT_MOMENTS.md`, the m=4 K256
full-smooth hit ceiling is `5.620988e-8`; this is a mathematical bound,
not a measurement of this fixed point's relation rate. The
selected minimum-total-runtime factor base remains unknown.

Compact review receipts are in [`evidence/`](evidence/); the raw local panel,
source snapshot, binary, and supervisor output remain under
`/Volumes/SSD990/crypto/artifacts/n83_compact_k256_20261018/` and the build
target under `/Volumes/SSD990-2/crypto/target/`. The base itself remains in
the requested S3 prefix. This capacity rung did not change that object.

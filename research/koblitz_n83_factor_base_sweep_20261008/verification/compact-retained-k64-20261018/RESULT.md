# Retained N83 K64 compact-root capacity result

The frozen `a=0` K64 public-x-hash base at seed 2026100801 was fetched again
from its existing S3 object. The compressed bytes matched BLAKE3
`1d0a6695564972f61db13c62c60a300da0cfa7bcd9b363c2e553a34345fae176`,
the manifest row, replay receipt, S3 upload receipt, and fresh HEAD/GET length
and ETag. The binary was static x86-64 Linux ELF with SHA-256
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`.
The supervisor bound it to clean source commit
`2ae7aea81dc13b29cc8ea17918ccd3c0883f6e61` and snapshotted the three
relevant Rust source files. The Docker image ID and live hard limits are in
[`evidence/container-inspect.json`](evidence/container-inspect.json).

The independent [`audit.py`](audit.py) returned
`PASS_CAPACITY_ONE_QUERY_MISS`. The worker imported the exact factor base in
201.617 ms, validated the public target in 4.361 ms, and completed all
172,640 unordered candidate states. All 172,640 were regular states, with
339,776 root-table entries. The single point probe took 38,342.610 ms and
the full supervised process took 39,627.168 ms; cgroup peak memory was
64,278,528 bytes. Exit code was zero, stderr empty, and the container was
gone after completion. The same auditor rejected a copy whose worker status
was changed from `MISS` to `HIT`; see
[`evidence/mutation-receipt.json`](evidence/mutation-receipt.json).

This was one public point query and it returned `MISS`. A relation row,
useful rank, linear algebra, column logs, and complete cold IC timing were
not measured. The 900-second wall cap was an upper bound, while 39.627 s is
the observed container-supervised wall charge for this successful query.
The separate cross-build took about 5 minutes 16 seconds, and the exact S3
preparation was separately measured; neither is silently folded into the
query time. Docker executed an x86-64 binary on an ARM host, so this timing
is a capacity observation rather than a native hardware ranking. The
selected minimum-total-runtime factor base remains unknown.

Compact review receipts are in [`evidence/`](evidence/); the raw local panel,
source snapshot, binary, and supervisor output remain under
`/Volumes/SSD990/crypto/artifacts/n83_compact_k64_20261018/` and the build
target under `/Volumes/SSD990-2/crypto/target/`. The base itself remains in
the requested S3 prefix. The factor-base archive was already stored there
before this capacity rung; this rung did not change it.

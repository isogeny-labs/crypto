# Linux V5 retained-K600 binding: exact pass and negative controls

The [protocol](../PROTOCOL.md) and [launcher](../run-binding.sh) were
committed at `8e718ddf1cbd` before this run. Its
[outer receipt](outer.receipt) spans 2026-11-16 15:25:28–15:25:31
UTC (three seconds at one-second resolution) and records exit 0.
This shared-host interval is an admission observation, not an IC
performance measurement.

| Gate | Observed result | Evidence |
|---|---|---|
| AWS identity and exact blob | Account `104186811093`; HEAD/GET each reported 3,025,798 bytes; independently measured GET SHA-256 `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` | [identity](aws-identity.json), [HEAD](s3-head.json), [GET](s3-get.json), [hash](object.sha256), [bytes](object.bytes), [object pointer](OBJECT_POINTER.md) |
| Exact V5 cell | `PASS_EXACT_ADAPTER_BINDING`, ordinal `26960859236`, K600 hash base, six summands, signed Frobenius plus ordered first pair, SAT/binary, one thread, no large primes | [native JSON](run/exact.json) |
| Adjacent ordinal | Exit 1, rejected as no retained adapter | [exit](run/adjacent.exit), [stderr](run/adjacent.stderr) |
| Altered manifest digest | Exit 1, rejected as a different K600 store manifest | [exit](run/bad-manifest.exit), [stderr](run/bad-manifest.stderr) |
| Container | Docker, inspection and cleanup exits 0; `OOMKilled=false`; 2 GiB memory, zero swap, one CPU, no network and read-only root | [Docker receipt](docker.receipt), [inspection](container.inspect.json), [post-run inventory](post-run-containers.txt) |

The exact source snapshot was
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`; the
[build packet](../../v5-linux-exporter-build-20261116/attempt-5/RESULT.md)
binds the Linux ARM64 binary to SHA-256
`87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`.
The fresh S3 read used the content-addressed key, while the retained
v3 manifest supplies the logical factor-base name. The
[artifact pointer note](OBJECT_POINTER.md) also gives the retrieval
path for the build binary moved by GitHub artifact automation.

The [flow diagram](admission.svg) and inspected [PDF](REPORT.pdf) show
the reached gates. There are no comparable algorithm-time values to
plot. [Raw file hashes](sha256.txt) cover the compact receipts. No
relation, rank, target descent, matched rho, or end-to-end IC runtime
was measured. The canonical IC scoreboard and performance graphs were
checked and left unchanged.

**Next gate:** a separately frozen constructed positive control on
the exact retained base, followed by an ordinary public-point query
only if that control and all host/input checks pass. Both require
native model replay and complete cost accounting before any factor-base
ranking.

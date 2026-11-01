# Bounded native roots with S3 storage

The native producer writes one fixed-memory stream of baby-root coordinates
for the unchanged 80-bit kernel. This first development/storage fixture
contains 4,096 roots (131,072 bytes), ordinal interval `[0,4096)`, with
kernel indices `2*ordinal+1`. PARI independently verified every root.
The [manifest](chunk-manifest.json) pins the field, kernel, source, interval,
encoding and bytes. It also records successful deterministic replay,
offset-resume consistency, count/window bounds and overwrite rejection.

No full quotient, map or performance measurement was run. The producer is
root preparation only. The old constructor cannot consume S3 objects as
working polynomials merely by moving their input files.

## Upload and retrieval

The authorized same-repository `artifact-autofix` workflow uploads selected
data to `s3://setec-astronomy-artifacts/store/sha256/`, downloads the complete
object again, checks the bytes and commits `.artifacts.json` plus an ignore
rule. Its pointer commit is the upload/readback receipt. Until that commit
exists the upload is pending; the creation-time manifest deliberately records
that state. No local AWS secret or newly created cloud identity is used.

After migration, from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/prime_conductor_gap80_20261031/s3-chunks
cd research/prime_conductor_gap80_20261031/s3-chunks
sha256sum -c roots-000000.bin.sha256
```

Artifact tooling verifies the downloaded digest before placing the file.
The object is immutable and content-addressed; the metadata preserves the
exact ordinal interval so regeneration does not need preceding chunks.

## Native producer and independent fixture check

Build with `NATIVE_PREFIX=/path/to/flint-gmp-pari-prefix sh native/build.sh`
from the study directory. `native/build/root_chunk` takes:

```text
P ELL KX KY FIRST_ORDINAL COUNT OUTPUT
```

The curve is fixed to `y^2=x^3-140*x+784`; use `parameters.json` for the exact
field, degree and kernel. The producer uses the existing GMP point arithmetic,
checks kernel membership/order divisibility and each generated point, and
rejects intervals outside the baby window. COUNT is capped at 65,536 for
bounded fixture work, and output creation is exclusive. This cap is not a
production-scale S3 object-size recommendation. Write incomplete or replay
files to distinct paths; publish only after the length, hash and independent
check pass. It emits no elapsed-time claim.

`native/build/check_root_chunk FILE` independently checks this specific
4,096-root, ordinal-zero fixture with PARI. Its exact source/kernel constants
match the manifest. It is deliberately not a generic verifier for arbitrary
offsets or parameters.

## Remaining external-memory work

S3 storage and root preparation are now separate from polynomial working
memory. A complete path still needs an external-memory product/remainder-tree
schedule, bounded native polynomial blocks, durable intermediate hashes and
the complete point evaluator. The existing FLINT routines accept contiguous
polynomials; they do not transparently operate on S3. Preserve every setup,
transfer, serialization, I/O and verification cost in a future measurement.

At this simple 32-byte encoding the entire baby window would occupy
13,668,958,625,088 bytes, before intermediates. It has not been generated,
allocated or uploaded. The fixture establishes storage identity and bounded
root generation, not feasibility or runtime for that full computation.
See the [report](../reports/s3-root-chunk.pdf) and unchanged
[endpoint measurement protocol](../MEASUREMENT_PROTOCOL.md).

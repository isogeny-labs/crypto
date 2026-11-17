# Preregistered construction peak recovery

The first frozen K=16,627 construction passed and produced the
content-addressed object in `manifest.json`; generic replay and its S3
download/hash round trip passed. The first container state records a 4 GiB,
zero-swap limit and no OOM, but the original command did not capture its
`memory.peak` cgroup file before the container exited. This is a missing
measurement, not evidence of a low memory peak.

Before the repeat, freeze the first manifest, replay and storage receipt in
Git. Then rerun the *unchanged* native Linux executable, SHA-256
`c40614fab6b73e59e5f29fc57d7ef53680be3de962a902a26eb25a6c12c9257b`,
from a clean committed checkout. The exporter source content must remain
unchanged. Use the same public curve, `public_x_hash` policy, seed
`2026100801`, K=16,627 and 3,600-second worker budget, in a fresh output
directory. Use the same pinned image, one CPU, 4 GiB memory, zero swap, no
network and 3,900-second outer cap. A shell wrapper reads
`/sys/fs/cgroup/memory.peak` to a separate output file after the exporter
returns and preserves the exporter's exit code. It does not alter object
bytes or source inputs.

`PASS` for this recovery requires exit 0 without OOM, a nonzero peak no
greater than 4,294,967,296 bytes, and equality of compressed BLAKE3,
uncompressed BLAKE3, point-set BLAKE3, compressed byte length and point/column
counts with the first manifest. Do not overwrite or replace the first
manifest, replay or S3 receipt. Record both process walls separately; the
shared-host L0 walls are descriptive. If hashes differ, retain both objects
and investigate before treating the peak as a matched reconstruction. A cap
or infrastructure failure is retained under its actual status.

This recovery does not run relation search or linear algebra and cannot
promote a base-quality or complete-runtime claim.

# RTL and native Envoy validation receipts

Run batch: `envoy-fpga-20261018-rtl-live`. This receipt extends the original
[Oct 17 import](../2026-10-17/README.md); historical logs and outcomes there
remain unchanged. The user requested continuation of the Envoy FPGA crypto
extension, including classical and post-quantum operations for AWS F2.

The [summary manifest](manifest.json) publishes the parent revision, current
project-file hashes, aggregate validation outcomes and remaining gaps. The
owner explicitly approved publication of the [raw archive](runs.tar.xz) and
[detailed run manifest](run-manifest.json). Both are preserved byte for byte
from the original validation batch. Their SHA-256 checksums are in
`runs.tar.xz.sha256` and `run-manifest.json.sha256`.

The detailed manifest records commands, tool versions, attempts and hashes
for all 66 raw records. Its project-file hashes identify the execution-era
snapshot, before the publication wording and report PDF were updated. Use
the summary manifest for the current documentation hashes; the production
code and test sources have not changed. Reproduction commands are in the
validation and kernel guides.

The archive preserves unsuccessful intermediate attempts and successful
validation logs. Intermediate source snapshots and exact initial standalone
lint command lines were not frozen, so those failures cannot be exactly
replayed from the final source snapshot.

Verify the downloaded files and extract the archive:

```sh
sha256sum -c runs.tar.xz.sha256
sha256sum -c run-manifest.json.sha256
tar -xJf runs.tar.xz
```

Reproduce the deterministic archive:

```sh
LC_ALL=C tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
    --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > runs.repacked.tar.xz
cmp runs.tar.xz runs.repacked.tar.xz
```

The native C++ hardware harness checks Keccak/SHAKE, canonical modular
multiplication, and the ML-KEM/ML-DSA forward and inverse NTTs. It passes all
15 test groups directly and the same 15 groups after sv2v/Yosys generic
synthesis. The source fixture seed is `0x4650433252544c`; simulation register
initialization is randomized with a fixed seed. Independent oracles include
OpenSSL SHAKE, direct polynomial evaluation/interpolation, and schoolbook
negacyclic convolution. Source RTL uses fatal `-Wall` lint and assertions.
The generic synthesis flow rejects runtime divider/modulo/latch cells and
requires zero combinational loops; its array-to-register warnings are retained.
It is not a formal equivalence proof or a vendor FPGA implementation.

The 36 existing host/TLS/PQ functional groups passed again. The original
262-mode exhaustive and sanitizer runs were not repeated in this batch.
The required root Rust test command again failed before tests on 26 errors
in unchanged source files. The duplicate `Fp4` base file still has Git blob
`0433fa620596bd24cdf1dbfd06c1ae67e841b2e1`. No existing Rust source or workflow
was modified to suppress its errors.

The minimal Envoy v1.39.3 executable linked successfully, and actual HTTP
listeners passed all five native C++ test groups using the explicit CPU
plugin. ML-DSA-65 signatures are independently verified by BoringSSL;
ML-KEM-768 interoperability is checked in both directions. This does not
exercise an actual Envoy TLS/F2 listener or prove hardware offload.
An initial live run passed 2/5 because the harness incorrectly expected
`Content-Type` on empty acknowledgments. The corrected test follows Envoy's
local-reply contract and passes 5/5 without changing production behavior.

The evidence includes failed lint/build/synthesis attempts, the generated
model killed for memory use, and intentionally interrupted Envoy builds that
preserved the compilation cache while the typed configuration fix was added
and the final build's concurrency was selected. A further restart corrected
the legacy HTTP/3 flag to Envoy 1.39.3's `--//bazel:http3=false`; the old
`--define=http3=disabled` option had been ignored. It also retains the initial
HTTP harness compilation failure on a missing BoringSSL include.

No F2 device, AWS identity or Vivado installation was available. Complete
hardware signature/KEM controllers, classical crypto cores, FPC1 dispatch,
entropy/key management, AFI integration, physical resource/timing checks and
measured acceleration remain unfinished. Generic kernel correctness does not
complete the user's requested system. See the
[source-linked report and diagram](../../reports/2026-10-18/README.md),
[validation scope](../../validation.md) and
[kernel contracts](../../rtl-kernels.md).

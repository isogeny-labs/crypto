# Native cubic process measurement

Development evidence is recorded in `PCC-20261105-740808`; the draft protocol
and access capture are in `PCC-20261105-f3b1ab`. Both have unverified standing.
No signed execution verification is claimed. [pcc:unverified]

Deadline handling was rechecked after ensuring watchdog scheduling cannot
extend the original limit. `PCC-20261105-8538a7` carries the completed regression
receipt and explicitly corrects the premature receipt in
`PCC-20261105-a3f112`. Both partial and final transcripts are retained.
[pcc:unverified]

The runner measures the actual `source_public` or `transfer_test` process.
Both entry points have an optional resident observation hook. Without the
runner's library and descriptor, the hook is disabled. No new scalar or
cryptographic input is supplied through it.

The standalone crate includes the production ecbench host, CPU reservation,
signal, checked PMU and grading modules. Grading versions 2 and 3 were moved to
`src/cryptanalysis/ecbench/measurement.rs`; their bodies and thresholds are
unchanged. The old `record` and `spec::Level` public paths remain reexports.

The parent reserves/pins using the existing native code, uses default
preflight thresholds, captures before/after conditions and waits with `wait4`.
A watchdog kills the process group at the deadline. The child reports actual
thread, affinity, NUMA, scheduler and interrupt observations through a descriptor
created exclusively by the parent. The payload and observation library execute
from pinned open-file descriptors; public inputs and binary bytes are checked
again after the process ends. Missing observations, timeout, signal, changed
input/binary or leftover descendants prevent a completed record. Output
directories are never reused.

Two intervals stay distinct: parent fork-to-reap cold time and the resident
interval inside main, before argument/input parsing through scope cleanup.
Both include native construction, retries, recovery and source checks when the
real pipeline runs. Neither is labelled online time. Harness preflight, hashing,
target generation and a separate mathematical audit stay outside those clocks.
The parent cold interval includes hook loading, probes and publication; these
costs are not subtracted. It measures the concrete instrumented implementation.

## Build and development checks

From a clean, source-bound checkout, build the ordinary native payloads with
the existing `build_source.sh`, `build_jacobian.sh` and `build_map.sh`, then:

```sh
bash research/cubic_extension_transfer_20261025/native_end_to_end/measure/build.sh
cargo test --release --locked --manifest-path research/cubic_extension_transfer_20261025/native_end_to_end/measure/Cargo.toml
```

The standalone lockfile is checked in. The build helper records commit and
dirty state through the production host module. An OCI build must use a
compatible userspace/toolchain and retain its exact dependency closure. The
older cached development container emitted a weak `GLIBC_2.39` warning while
running the host-built observer; do not treat that diagnostic as a qualified
image receipt.

The lifecycle test uses non-cryptographic C++/Rust fixtures. It exercises clean
completion, an ordinary error, Rust unwinding, timeout, leftover descendants,
missing observations, changed public input, strict-mode refusal and output
collision. These local controls use `--diagnostic` and `cpus=none`; their records
remain L0 and ineligible for timing comparison. The real transfer driver was
also built and checked on its invalid-argument path, without solving an ECDLP.

## One-process interface

```text
cubic-native-measure OUTDIR ABSOLUTE_WORKDIR ABSOLUTE_LIBRARY CPUS TIMEOUT_SECONDS
  [--diagnostic] [--input ABSOLUTE_PUBLIC_FILE]...
  -- ABSOLUTE_PAYLOAD ARGS...
```

Strict mode requires reservation and explicit public input bindings. For a
transfer, bind the public fixture, kernel-classification data and applicable
native Jacobian library in addition to the pinned payload/observer. Record the
complete native dependency closure and immutable image at the session level.
`inherit` uses the actual EKS cpuset; it does not itself establish isolation.
Preserve raw outputs, `host.json`, `request.json`, the child receipt and
`record.json` (or `blocked.json`) for every attempt.

## Admission and remaining work

`isolation_wall_eligible` requires a clean strict execution earning L2 under
the shared grader. Every record still has `performance_admitted: false` and
`mathematical_verification: null`: the enclosing study must perform source
verification, A/A, complete accounting and independent replay. PMU records keep
the checked collector's unverified coverage/platform fields; unsupported or
denied counters are retained. Complete native GAE remains unknown, never zero.

[protocol.json](protocol.json) is a canonical, validated **draft**, preserving
the proposed 400 measured executions and 80 warmups. No instrument hash, pilot
runtime, analysis implementation or freeze is fabricated. The panel scheduler,
canonical export/analysis integration, strongest applicable source-reference
review, final image, live cost/capacity admission and checkpoint launch gate
remain prerequisites. The whole-library release check was attempted but could
not finish after local disk exhaustion; standalone checks do not replace it.

The [prior launch plan](../benchmark_20261105/launch-plan.json) retains the
USD5 inclusive ceiling. The fresh access capture still reports denied budget,
launcher, registry and runner-discovery permissions. No scientific ECDLP
benchmark was launched by this continuation. [pcc:unverified]

Development receipts and their unsigned claim standing are in `evidence/`.

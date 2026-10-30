# Updated-parent native validation, 2026-10-24

## Requested gates and observed execution

This run completes the local release-library, affected-example, executable-build
and full-size construction gates for the D18 shared-bit encoding. The separately
bounded Linux construction, 22 constructed controls, 22 independent replays and
18 natural m=4 queries remain required by the [frozen protocol](../../PROTOCOL.md).

Tested source: `4e1d311d87571aa3309ed4bfac51d06539ef2297`, based on census parent
`2e58bc5be60f609f1760d7834e40c5604d2b48e5`. The affected source hashes were checked
again after documentation commit `19b70c0c8986629294ee012e22edab7111fd4eba` and
all matched. The original implementation remains at `99ba8febf`; the native
baseline files on the updated parent match encoding baseline `e294f8c92` exactly.

Host: Apple M4 Pro, arm64, Darwin 25.6.0, shared development host. The native busy
lock serialized this job. It is not a timing-isolation receipt; the construction
had no hard memory limit. Cargo's retained compiler-information JSON identifies
Rust 1.98.0, commit `88d9e12ae178fab0fb5cc050a94da85685d449ea`, LLVM 22.1.8.
The later interactive shell resolved Homebrew 1.93.1; that distinct observation
is preserved and is not substituted for the Cargo compiler receipt.

The successful sequence started at `2026-10-24T06:03:07Z` and ended at
`2026-10-24T06:23:50Z`. These timestamps identify validation execution; they are
not an algorithm timing comparison.

| Gate | Observed result |
|---|---|
| `cargo test --offline --release --lib` | 4,478 passed, zero failed, 119 ignored |
| Eight affected examples | 28 passed, zero failed |
| Five release example executables | Build exit 0 |
| Full D18 construction, both encodings at m=2,3,4 | Exit 0; all six actual models equal preflight |
| Construction receipt comparison | Byte-for-byte equal to committed `construction.json` |
| Tested source hashes, checked after documentation commit | All six match |
| Workflow budget metadata | 440 workflows, zero violations |
| Actionlint, touched receipt workflow and inherited ecbench workflow | Exit 0 |

The example counts are 3 construction/control tests, 5 shared-card audit tests,
5 historical full-card audit tests, 2 historical card-driver tests, 7 worker
tests, 1 reuse-accounting test, 3 general replay tests and 2 XOR-cache audit tests.
The full library includes the exhaustive selected-representative/phase/x checks,
shared-one/shared-zero cases and atomic cap/rejection tests.

## Structural observations

The curve is `icv1-f2m83-tm6151469093347-debefd74`. The fixed base has 16,627
representatives and 2,760,082 signed-Frobenius points. The independent inventory
finds 66 common zero coordinates. The reconstructed counts are:

| m | Original clauses | Shared-bit clauses | Variables, both | XOR rows, both |
|---|---:|---:|---:|---:|
| 2 | 2,926,660 | 732,028 | 90,258 | 1,743 |
| 3 | 4,420,949 | 1,129,001 | 145,928 | 2,822 |
| 4 | 5,915,238 | 1,525,974 | 201,598 | 3,901 |

![Reconstructed integer model counts](../../counts.svg)

One deterministic construction per cell; exact integer counts, with no sampling
interval. [Editable graph source](../../counts.typ) reads the
[committed receipt](../../construction.json), which the fresh output matches.
No SAT search or solver-runtime comparison was performed in this local gate.

## Preserved failures and remaining obligations

The initial resumed library run used retained source `99ba8febf`. It passed
4,395 tests, failed nine tests attempting to create local sockets, and ignored
118 tests. Each failure reports `Operation not permitted`. The full updated-parent
run above was then executed with local socket access; tests were not skipped or
disabled to obtain the passing gate. The earlier T7 run cannot be recovered
while that volume is unmounted and is not counted as passing.

PR #2271 was already merged at `18a6735fbefc53b0bffb2443a0075faa9c1ebfb0` when
this work resumed. Its later workflow-parser check failed on the 45-minute
harness budget. The updated parent contains the named budget exception and
separate historical audit job, and its local budget check passes. Exact-head
CI and merge of this follow-up remain separate obligations.

The isolated Linux source checkout was restored from a verified self-contained
pack after a thin bundle lacked five delta bases. Its source is clean at
`4e1d311d8`. The queued native validation waits behind the shared benchmark lock
with a four-hour total admission limit. It will perform its own full native
gates and the frozen 4 GiB, zero-swap, one-CPU construction. It does not launch
the paired query card. The actual host card must first be prepared, hash-bound
and committed. The checked Lean source and receipts were recovered from the
unchanged validation-v1 archive and compared with the current proof source.

Natural useful rank, stronger m=2/m=3 SAT policies, complete cold and reused IC
cost, linear algebra, matched same-point rho, fastest-external comparison and
best-base selection remain open. There is no new runtime observation or pipeline
ranking to add to canonical performance dashboards or `benchmarks/`.

## Evidence custody

[Validation v3](../../validation-v3.tar.xz) contains 34 files. The
[file manifest](../../validation-v3-files.sha256) and
[archive digests](../../validation-v3.sha256) bind the failed library attempt,
the successful sequence, source identities, Cargo compiler receipt, CI checks
and queued Linux command scripts. The archive was extracted, every file hash
checked, and repacked byte-identically. Earlier v1/v2 archives are unchanged.

Archive SHA-256: `e6036d207cb7b2720d4cfd1c20eca943c8ba162224ad03e96087f5db31d3ab4c`.

[Inspected run-report PDF](RUN_REPORT.pdf); [editable PDF source](RUN_REPORT.typ).

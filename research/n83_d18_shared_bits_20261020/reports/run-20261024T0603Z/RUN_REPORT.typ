#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 D18: native validation completed
_Run 2026-10-24 06:03:07–06:23:50 UTC · correctness and construction only_

The requested local native gate passes on census parent `2e58bc5be` plus
shared-bit implementation `4e1d311d8`. Encoding baseline: `e294f8c92`.
The working curve is `icv1-f2m83-tm6151469093347-debefd74`.

#table(columns: (2.5fr, 4fr), inset: 5pt,
  [*Gate*], [*Observed outcome*],
  [Full release library], [4,478 passed; zero failed; 119 ignored],
  [Eight affected examples], [28 passed; zero failed],
  [Release executables], [Five builds, exit 0],
  [Six full-size model constructions], [All actual counts equal preflight],
  [Receipt and source custody], [Construction byte-identical; six source hashes match],
  [Workflow metadata], [440 workflows; zero budget violations; actionlint passes])

#v(2mm)
#align(center, image("../../counts.svg", width: 80%))

Apple M4 Pro / arm64, shared development host. Cargo records Rust 1.98.0
and LLVM 22.1.8. The busy lock serializes work; it does not qualify timing.
There was no hard memory cap. Exact integer counts have no sampling interval.
No SAT search or solver-runtime comparison was performed by this gate.

The earlier sandbox attempt passed 4,395 tests and failed nine local-socket
tests with “Operation not permitted”; 118 were ignored. Its log is retained.
The successful full suite above ran with socket access; no tests were disabled.
The unmounted T7 run is not counted as a pass.

*Remaining gates:* bounded Linux construction; a committed actual-host card;
22 constructed controls, 22 independent replays, 18 natural queries; exact-head
follow-up CI and merge. The Linux validation job is queued behind the benchmark
lock. Natural useful rank, full cold/reused IC cost and matched rho remain open.

*Evidence:* `validation-v3.tar.xz` contains 34 files and was hash-checked after
extraction and byte-identical repack. SHA-256:
#text(size: 8pt)[e6036d207cb7b2720d4cfd1c20eca943c8ba162224ad03e96087f5db31d3ab4c].
Sources: `RUN_REPORT.md`, `validation-v3-files.sha256`, `construction.json`,
`PROTOCOL.md`, and `PROOF_MAP.md`. The graph is unchanged because the new
construction reproduces its source receipt exactly. Canonical runtime charts
and benchmark rows stay unchanged: this run adds no timing comparison.

# Local execution of the unchanged recovery pilot

Frozen before execution on 2026-11-28. Tracking PR: #5306; predecessor: #5126.

The owner explicitly requested proceeding without AWS. This execution therefore
uses the local CPU host, ordinary private files and local archives. No AWS call,
S3 gate, remote runtime, CRIU continuation or restored memory image is involved.
The original `../PROTOCOL.md` remains unchanged; this document overrides only
its cloud backend and checkpoint destination for this local attempt. Local
archives are not an external durable backup and cannot resume a constructed
in-memory table. A terminated process is retained as an incomplete attempt.

## Inputs, algorithm and stopping rule

Run precisely C0/G0/K0, the twelve original unknown-log factor-base points and
the original configuration. No rho/BSGS answer or factor-base log is an input.
The radius is 12, setup cap 48, and native deadline 3,600 seconds, including
table construction. Do not retry automatically or alter the domain after an
unfavorable outcome. Success requires rank 12, twelve checked base-log equations,
the unchanged target equation and successful replay by the separate verifier.
Insufficient rank, missing decomposition, deadline, OOM and verification failure
are retained separately. One pilot does not discharge the 4,600-instance study.

The collector uses the already implemented generic six-plus-six bounded
coefficient decomposition within an index-calculus relation/linear-algebra
pipeline. This run adds no algebraic polynomial solver or regularity result.
Prior polynomial-system measurements remain their own frozen evidence.

Pins checked before launch:

| Input or executable | SHA-256 |
| --- | --- |
| `../public_source_case.json` | `6bec6a602738acf8cdb9a3f73bcbaeb0d139615fe65e798166f942bcd0b975bd` |
| `../pilot_config.json` | `9b18b57a789416fdabd76599f45fcc67aa0680046ea90625668fdbb4513e3ac7` |
| `../SOURCES.sha256` | `ff0c1804287eb3f281bbe2908d6ba149f2b33013f4abd6ecfaa2ff8f96c8e162` |
| Native collector | `50343e2c6fddb2cdae522c9aa11178d276830be4c8a51497a25eb0a2086b6148` |
| Separate affine verifier | `f639a2222f33e6aa20616d514662db50fde135f1a6314cd33d4919ca49893dd3` |

## Resources, entropy and accounting

The observed local cgroup allows 17,179,869,184 memory bytes and four CPU cores.
The collector is single threaded. Apply a 15,032,385,536-byte address-space limit
and a 3,720-second external timeout with a 30-second termination grace. Keep the
original application table cap of 12,884,901,888 bytes. The calculated payload
is 9,566,497,092 bytes; peak RSS will be reported from execution, not inferred
from this calculation. Timing is descriptive, non-isolated and host-specific.

Use the frozen Linux `getrandom` implementation with unbiased rejection; retain
every returned word and rejected draw privately. There is no master-seed CLI in
this frozen collector. Per `docs/experiment-seeds.md`, record this limitation:
the attempt is not qualified for a deterministic A/B or performance comparison.
The complete random tape preserves the actual choices; the separate checker
checks equations rather than recreating an OS entropy stream.

Preserve stdout, stderr, outer exit status/wall times, native per-phase
wall/CPU/RSS and counters, all progress/failure rows, setup trials, coefficients,
matrix rank, private logs/scalar, entropy journal and separate replay output.
Multiplication/squaring are disjoint; inversion overlaps its executed arithmetic.
Keep setup and target phases separate. Preserve failed attempts and partial
journals; do not replace missing counters or phases with zeros.

## Execution and preservation

Use the existing hash-checked development binaries directly. Do not modify or
spoof cloud-wrapper admission variables. The private attempt root is
`/workspace/volcano-local-20261128/pilot-001`; the native create-new output is
its `native` child. Record exact argv, limits, host facts, hashes and UTC times
in its metadata before starting. Files remain outside Git with private modes.
Publish only a scalar-free summary; retain the raw archive locally and provide
its hash and download link. No uploaded or encrypted external backup is claimed.

Source review: frozen protocol, collector/checker, prior source provenance and
the seed policy. KB MCP/wrapper unavailable; use those committed sources as the
documented fallback. No new mechanism, novelty or leaderboard claim is planned.
Local Conductor check returned allow and task T-1 was claimed; it coordinates
this container only. Shared-code parity checked all 21 listed files.

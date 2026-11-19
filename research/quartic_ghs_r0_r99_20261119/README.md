# GHS strong-rho R0–R99 parallel optimization campaign

Status: parallel producer-side implementation and deterministic screening
wave complete. No wall-time or frontier result is claimed here
`[pcc:unverified]`.

Date: 2026-11-19

Tracking branch: `codex/ghs-rho-parallel-20261118`

Tracking PR: `#4757`

This directory expands the reviewed R1–R9 roadmap into one hundred explicit
optimization, integration, measurement, and publication lanes numbered R00
through R99. The lanes are searched in parallel; mutations of the composed
baseline and performance promotion remain serialized through the gates in
[`PROTOCOL.md`](PROTOCOL.md).

Read the source-linked [`REPORT.md`](REPORT.md) or the visually inspected
four-page [`REPORT.pdf`](REPORT.pdf) for the consolidated result.

## Requested scope and execution state

| Requirement | Evidence | Current state |
|:--|:--|:--|
| Register every ID R00–R99 exactly once | `registry/R*.json` and the native registry validator | Complete |
| Execute independent work in parallel | Six disjoint range reports, prototypes, and receipts | Complete |
| Implement every currently runnable lead | R17/R36/R64 production paths plus per-range deterministic screens | Complete at producer scope |
| Preserve blocked or dependent leads | Canonical state, detailed disposition, and explicit unblock condition | Complete |
| Measure admitted candidates end to end | Frozen matched grid and ECBench campaign | Blocked on execution admission |
| Update the leaderboard for validated gains | Validation/PCC/publication lanes R96–R99 | No eligible measurement yet |

“Complete at producer scope” means the registered deterministic implementation
or disposition gate ran. It does not mean that a lead produced a gain or
completed its eventual isolated measurement.

## Parallel batches

| Batch | Scope | Owned artifacts |
|:--|:--|:--|
| R00–R16 | provenance, controls, accounting, composition | `ranges/R00_R16.md`, `registry/R00_R16.json`, `prototypes/r00_r16.rs` |
| R17–R33 | base-field and cubic-field arithmetic | `ranges/R17_R33.md`, `registry/R17_R33.json`, `prototypes/r17_33.rs` |
| R34–R50 | scalar, group, setup, retry, and coordinate work | `ranges/R34_R50.md`, `registry/R34_R50.json`, `prototypes/r34_50.rs` |
| R51–R66 | walk state, partitions, symmetry, cycles, counters | `ranges/R51_R66.md`, `registry/R51_R66.json`, `prototypes/r51_66.rs` |
| R67–R83 | batching, memory, SIMD, threads, parallel DP rho | `ranges/R67_R83.md`, `registry/R67_R83.json`, `prototypes/r67_83.rs` |
| R84–R99 | ECBench, complete accounting, validation, publication | `ranges/R84_R99.md`, `registry/R84_R99.json`, `prototypes/r84_99.rs` |

## Evidence boundary

Local Rust controls may establish algebraic equality, deterministic trajectory
parity, schema coverage, or rejection of a candidate on a public synthetic
fixture. They do not establish throughput, isolated wall time, an end-to-end
gain, or a changed frontier. Those claims remain behind checkpoint, host
admission, preregistration, calibration, replay, independent validation, PCC,
benchmark-record, and leaderboard gates.

The research-KB and Conductor executables were unavailable at campaign start.
The repository evidence and cited primary-source inventory named in the
protocol are the recorded fallback; unavailability is not evidence of novelty
or absence `[pcc:unverified]`.

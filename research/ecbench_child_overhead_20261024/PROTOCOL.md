# Protocol: per-child overhead of `ecbench exec`

Frozen 2026-10-24, before any timing. Changes after measurement starts are
appended at the end with a timestamp and a reason.

## Question

Every measured run of every ecbench session is a fresh `ecbench exec`
child: it rebuilds the workload, solves, and prints one record. PR #2184
changed what a child does before the solve in two commits:

- `9ead78cf0`: parse the curve registry once per process, not once per
  `facts()` call;
- `0f5d9c272`: parse only the one registry entry a child needs, cut from
  the document text by its slug, instead of the whole 4.7 MB registry.

How much does a child cost before and after, in instructions and in
wall time, with every solve unchanged?

## Arms

| label | binary built from | role |
|:--|:--|:--|
| `before` | `1b74250e9` (parent of `9ead78cf0`) | baseline |
| `after` | `0f5d9c272` (both commits) | candidate |
| `after-aa` | the `after` binary again, same path | A/A control |
| `head` | this PR's head | the change as it stands on `main` today |

Each is `cargo build --release --bin ecbench` from a clean export of
that commit with the same `Cargo.lock`, so the dependency versions are
identical.

## Workload

The child inputs of the first 18 measured executions of
`docs/ecbench/specs/smoke-prime.json` (every arm of the smoke spec, both
workloads, round 1: rho, BSGS, kangaroo and IC on a 20-bit prime-order
curve), written by `examples/ecbench_child_overhead.rs inputs` exactly as
`ecbench verify` builds them for a replay.

## Measurements

1. **Instructions (primary).** Callgrind's total `Ir` for one child of
   each binary on three inputs (`rho-neg`, `bsgs-neg`, `ic-mitm`):
   `valgrind --tool=callgrind ecbench exec < input`. Deterministic up to
   the allocator; reported per input and binary.
2. **Wall time (descriptive).** `examples/ecbench_child_overhead.rs time`,
   30 rounds; in each round every input runs once under every arm, in an
   order alternating by round and shuffled within it (seed 20261024).
   Wall is spawn to exit. Reported: median per arm with a 95% interval from
   2,000 bootstrap resamples of whole rounds, and each arm's median over
   `before`'s.
3. **Identity (gate).** Before timing, every input runs once under every
   binary, and the answer, total, phases, counters and unpriced list must
   be identical across binaries. A disagreement stops the measurement.

## Reading

- The A/A ratio's interval must contain 1, or the wall result is not
  reported as a ratio.
- The host is a shared 4-vCPU cloud VM with no hardware counters and no
  isolation (L0); wall time is descriptive and host-specific, and IPC is
  not available.
- This is engineering: the counted unit (GAE) of every method is
  unchanged by construction, so there is no ECDLP cost change and no rho
  comparison. The matched rho reference is the same binary's own
  `rho-neg` solve, identical across arms.

## Amendment 1 (2026-10-24, after the first Callgrind pass, before any timing)

The first Callgrind pass found that the head of this branch spent 154M of
an IC child's 169M instructions building the curve alias map
(`ic_boundary::pinned_ratios` → `curve_id::same_curve`). This PR fixes that
in `9041407f6`. The arms become:

| label | binary built from | role |
|:--|:--|:--|
| `before` | `1b74250e9` | baseline |
| `after` | `0f5d9c272` | the two #2184 commits |
| `after-aa` | the `after` binary again | A/A control |
| `main` | `004b792b7` (this branch before the fix; its child path is `main`'s) | the child as `main` runs it today |
| `head` | `9041407f6` (the fix) | this PR |

Callgrind counts every dump part of the process. A child writes several
parts, split at its client requests (`generic_ic_setup`, …), and the
totals are their sum. Nothing else changes.

## Amendment 2 (2026-10-24, before any timing)

A second Callgrind pass on `9041407f6` found 22M of the IC child's 32M
instructions in the fix's own reverse scan. `645666691` replaces it with a
binary search. `head` is `645666691`. Nothing else changes.

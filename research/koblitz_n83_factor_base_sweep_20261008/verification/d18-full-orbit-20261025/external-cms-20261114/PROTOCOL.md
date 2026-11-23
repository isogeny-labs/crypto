# Source-bound D18 external XOR solver gate

**Frozen 2026-11-14 before export or solver timing.** This is a bounded
solver-backend diagnostic on the primary N83 `a=0` curve, subgroup order
`2417851639230796216685689`. It uses the exact 5,418,738-point,
32,643-column D18 input and the same constructed and first public natural
targets as [native attempt 7](../native-cadence-replay-20261026/attempt-7/REPORT.md).
The independent variable is the solver backend: CryptoMiniSat 5.14.7 parses
the worker's extended DIMACS XOR export. Native attempt 7's natural arm
returned `UNKNOWN_search_cap` at 100,000 conflicts; its process wall is a
capacity/context observation, not a matched speedup reference.

## Exact inputs and software

| Item | Identity |
| --- | --- |
| Worker source | clean commit `45ce91d503ee0d0acc57e3d8d5cb49108c90ded6` |
| Worker binary | SHA-256 `5f302b8cfd86470e4d08da34c972308f0c8b5dbc80422dc8b578625f8e075fd2`, Darwin arm64 Rust 1.93.1 |
| Producer manifest | SHA-256 `1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce` |
| S3 wrapper | SHA-256 `1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616` |
| Compressed D18 object | SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`, 160,747,529 bytes |
| Public corpus | SHA-256 `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd` |
| Solver | `/opt/homebrew/bin/cryptominisat5`, version 5.14.7, SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac` |

Fresh checksum-enabled GET receipts and full local hashes are required before
export. Use `primary-full-d18-m4-l0-xcnf positive|natural BASE_DIR
FIXTURE_DIR NEW_OUTPUT_JSON` from the pinned worker. The export is a
target-independent/target-specific construction charge as appropriate;
record process wall, CPU, peak RSS, XCNF bytes/hash, model shape, and exact
output paths for both modes. The worker's 400 MB XCNF output cap applies.
Apply a 310-second outer process-group guard to each export; a timed-out or
nonzero export is a retained producer failure and stops the panel.
Each export must have a terminal exit `0`, exact source and input identities,
and the expected positive/natural public point. Preserve the XCNF bytes.

After both exports, write a dated `EXPORT_LOCK.json` with their SHA-256,
lengths, shape, worker receipt hashes and the solver hash; commit this lock
before any CryptoMiniSat invocation. A changed export requires a new lock and
new run directory. No solver result from an unbound formula enters this panel.

## Solver arm and stop rules

Run the constructed positive first, then at most the one frozen natural
query. Both use `--threads 1`, `--random 20261114`, `--maxtime 600`,
`--maxconfl 1000000`, `--verb 1`, `--verbstat 1`, `--printsol 1`, and a
660-second external process-group watchdog. Use the same flags and solver
binary for both modes. Require one-minute host load at most 24, no new
swapouts across three 15-second samples, at least 2 GiB scratch free, and a
clean pinned source checkout before each arm. Retain exact command, UTC
boundaries, exit status, stdout/stderr, conflict count, process wall/CPU and
sampled peak RSS. Charge export, all attempts, and verification separately.
Do not combine this L0 shared-host timing with prior hosted or other-source
wall times as a performance ratio.

The constructed arm releases the natural arm only if CryptoMiniSat parses the
exact XCNF and returns a complete SAT assignment that passes an independent
original-XCNF clause and XOR replay. If the constructed solver returns
`INDETERMINATE`, reaches a watchdog, fails to parse, or its model fails the
XCNF replay, retain that outcome and stop. A natural SAT assignment is a
candidate relation only after the same XCNF replay and a separate original
field, orbit-domain, subgroup, curve-group and row verification. The current
native replay verifier expects a native worker receipt, so a natural external
SAT result is held at `PENDING_GROUP_REPLAY` until a source-bound external
model bridge is implemented and checked. `INDETERMINATE`, timeout, OOM,
unsatisfiable first pattern, malformed output, and rejected models retain
distinct rows; none is imputed as zero solver cost or natural yield.

This gate can reveal whether a different XOR solver reaches a verifiable
relation on the same D18 input. A complete factor-base selection still
requires multi-query natural relation yield, useful rank, full subgroup logs,
unseen-target descent, complete cold costs, and matched same-point rho on
both separately identified N83 curves. Keep `selected_best_total_runtime`
null until those checks support a selection.

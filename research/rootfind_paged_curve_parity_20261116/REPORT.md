# Paged binary pair index: actual-curve parity

Recorded 2026-11-16. This is a source-bound correctness result for a
standalone index on three actual Koblitz E0 torsion-harvest pair streams. It
does not contain a collector timing, process-RSS measurement, or whole-solve
comparison. The frozen [protocol](PROTOCOL.md), macOS execution
[amendment](AMENDMENT-20261116.md), [test source](../../tests/rootfind_paged_curve_parity.rs),
[machine-readable result](RESULTS.json), [raw stdout](test-harness.stdout),
and [raw stderr](test-harness.stderr) are retained. The test source was
committed at `1103716b6d92c210b39ec426c6cb094b90f65b22` and the harness
at `aa40df7ad378de5ab81fdd058dc076b204661432` before execution.
[pcc:unverified]

## Construction and exact check

The test reconstructs the ordered torsion pair-sum stream with the public
binary group law and inserts each nonidentity key into the proposed
[`PagedPairIndex`](../rootfind_paged_pair_index_20261116/paged_index.rs) and a
standard hash map. It checks first-witness parity at every insertion,
every retained key, boundary and absent keys, and all K13 keys. Every
duplicate has its full signed four-point group equation checked. The
complete duplicate tuple stream, retained-key count, and insertion count
match `PairTable::build_negation_folded_binary_batch_torsion_with_collisions`.
The curve ICV1 IDs and factor-base SHA-256 digests are pinned and recomputed.
The [editable data-flow diagram](diagram.dot) and [rendering](diagram.svg)
show the three-way comparison. [pcc:unverified]

The `--release --locked --offline` harness run with Cargo and rustc 1.99.1
on macOS arm64 exited zero: five tests passed, including the three-cell
actual-curve parity test. Its test executable SHA-256 is
`9ef1d8fb03a1710f0e0d31f891e4b51441fd899058aecb83731d0ba7e2e68a75`.
The [derivation script](derive_results.py) asserts the pinned counts and
output success before writing `RESULTS.json`. This is a same-producer check;
independent validation remains a separate step. [pcc:unverified]

A second same-host run at 2026-11-16T10:59:56Z exited zero with the same
three machine-readable parity records and five passing tests. Its
[metadata](replay.meta), [stdout](replay.stdout) and [stderr](replay.stderr)
are retained. `time -l` in the replay stderr measures the cached Cargo/test
launch path and does not price a complete collector or the full process tree.
The initial and replay stdout differ only in test-output interleaving; the
derivation asserts equality of their parsed parity records. [pcc:unverified]

| Curve | Signed base points | Pair inserts | First witnesses | Full group-checked collisions | Used / preallocated pages |
|:--|--:|--:|--:|--:|--:|
| K13 | 33 | 272 | 261 | 11 | 204 / 290 |
| K19 | 139 | 4,830 | 4,800 | 30 | 4,473 / 4,901 |
| K23 | 275 | 18,906 | 18,842 | 64 | 18,498 / 19,045 |

The [page-use figure](page_use.svg) shows touched 16-word pages against
preallocated capacity. The prototype allocates its full bounded page
capacity before insertion, so used pages are a key-distribution diagnostic,
not actual allocator bytes or collector peak RSS. The requested backing bytes in the
result record repeat the prior [storage derivation](../rootfind_paged_pair_index_20261116/REPORT.md);
no new measured memory result follows. The Cargo release build took 3m18s
and the test runner reported 0.01s for all tests; neither is a complete
collector timing. [pcc:unverified]

## Failed admissions retained

The default compiler and `rustup run` without an absolute `RUSTC` both used
rustc 1.93.1 and refused locked dependencies that require 1.94.1. Homebrew
Cargo rejected `+1.99.1`. With the correct compiler, root-package `cargo
test --test` compiled the unrelated Linux-only `icprog` binary and failed on
macOS `libc` symbols before the parity test. All four attempts have distinct
`test*.stdout` and `test*.stderr` files; the [worklog](WORKLOG.md) records
their commands and exit codes. The committed local harness builds this
checkout's library and runs the unchanged test target. None of the four
failed admissions is algorithm evidence. [pcc:unverified]

## Decision boundary and next gate

`dominated_by`: production hash and opt-in flat tables remain the comparison
arms for a later complete-call experiment. This run establishes parity of
the standalone paged first-witness index only. It does not establish a
relation-yield, rank, ECDLP cost, or F4/F5/F6 performance change.
`sota_delta`: zero demonstrated complete-call cost change. The internal
[rootfinding paper](../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex)
already describes binary pair tables and dense-index engineering; no
novelty claim is made. [pcc:unverified]

The next implementation gate is a guarded production method parameter in
`ic_boundary.rs` and the ecbench adapter, followed by exact answer and
collision parity on the same targets. Those source files were reserved by
Conductor T-6708 during this run. A separately frozen, isolated whole-solve
panel is needed to compare against the fastest comparable F4/F5/F6 arms.
The existing [complete-cost calibration problem](../../problems/rootfinding-complete-cost-calibration.md)
tracks native-work pricing, isolated timing and independent replay; this
parity result does not meet its exit criteria. The canonical performance
graphs were checked for scope and have no new timing edge to add.
[pcc:unverified]

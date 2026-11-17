# Torsion-row root-finding check — 2026-11-16

This report records local correctness and operation-counter observations only. It does not review a hypothesis, assert a wall-time advantage, or alter the frozen K13 EKS F4/F5/F6 comparison.

## Identity and method

- Source: `c6766ee4772ff03091553298ce1c397ee7a9ab96`, with no uncommitted `src/` changes during the source-pinned checks.
- Protocol: [`../../PROTOCOL.md`](../../PROTOCOL.md), committed before implementation. The curve cells are Koblitz E0 over GF(2^13), GF(2^19), GF(2^23), standard-subspace dimensions 5, 7, 8, and signed Frobenius-orbit columns. The K13 known scalar is 641, pipeline seed 811, target `[641]G`.
- Controls: scalar and batched first-witness tables, full group checks for retained duplicate keys, corrupted signed-pair index, unsupported `b=2` model, and a fixed same-target K13 pipeline.
- Host and exact compiler, commands, timestamps, outputs, exit codes, and timeouts: [`ecbench.json`](ecbench.json), [`lib-focused.json`](lib-focused.json), [`lib-full.json`](lib-full.json), and their adjacent stdout/stderr files.

## Observations

The source-pinned release `ecbench` binary suite passed 21 tests, with one pre-existing ignored test. The focused library test passed and checked every key-to-first-witness map entry and probe result on each fixed cell. The negative-index and unsupported-model controls refused as specified. These are local test assertions, not independent review.

| Fixed cell | Original collision candidates | Retained full-group checks | Torsion twins represented by one seed | Rank with seed |
| --- | ---: | ---: | ---: | ---: |
| E0/GF(2^13), d=5 | 27 | 11 | 16 | 5 |
| E0/GF(2^19), d=7 | 99 | 30 | 69 | 11 |
| E0/GF(2^23), d=8 | 201 | 64 | 137 | 23 |

The counts are assertions in the fixed-cell release tests, with the previous collision census inputs pinned by hashes in the protocol. The new table matches the scalar and batch tables at every retained key. Each omitted signed twin was checked to have the order-two first witness and a valid negative index. The retained rows still undergo complete group-equation verification. The rank numbers are modular ranks, not a prediction of relation yield outside these cells. Editable graph data and script: [`counts.json`](counts.json) and [`make_visuals.py`](make_visuals.py). Rendered [count graph](counts.svg) and [mechanism diagram](mechanism.svg) are included in [the PDF](report.pdf).

For the fixed K13 same-target pipeline, the original batch arm reported 36 target trials, 17 target-bearing rows, 305 setup additions, and 2,857 total group-addition equivalents. The previous harvest arm reported 23 trials, 13 target-bearing rows, five independent setup seed rows, 386 setup additions, and 2,316 total group-addition equivalents. The torsion arm reported 23 trials, 13 target-bearing rows, five independent setup seed rows, 305 setup additions, 11 verified non-torsion collisions, 16 skipped twins, and 2,262 total group-addition equivalents. All three arms verified scalar 641 against the same public target. These are counters from one deterministic fixture, not whole-process or cold-start timings; the new batch inversion has separate native counters and is not priced by the group-addition-equivalent total. See `ecbench.stderr` for the raw diagnostic lines. [pcc:unverified]

## Incomplete gate and next check

The full release library suite did not finish within the fixed 180-second watchdog. At stop, its output contained 2,502 passing tests, eight failing ecbench band/compare/spec/workload tests, and 22 ignored tests; the names and raw output are preserved in `lib-full.stdout`. These eight names match the earlier failing gate recorded in `docs/session-logs/2026-11-16.md`. This incomplete suite is not a clean library pass. The focused test and full binary suite passed. The local test binary emitted a nonfatal `rust-objcopy` warning because its `libLLVM.dylib` is missing; the tests still ran and returned zero.

Next, run a separately frozen matched whole-call timing panel, including the native field inversion and A/A control, and independently replay its receipts. The combined ten-arm K13 EKS F4/F5/F6 panel remains unrun while the live account budget gate is over its declared limits; see `coordination/publications/rootfind_f456_20261116/admission-readback-20261116T0838Z.json`. No current wall-time or end-to-end speedup follows from this check. [pcc:unverified]

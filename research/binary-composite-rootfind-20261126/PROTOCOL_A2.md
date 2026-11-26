# Amendment A2: linear algebra that finds every dependency (supersedes A1's run)

Frozen 2026-11-26. This was after A1's partial run
(`data/runs_a1_stopped.jsonl`, 223 records, stopped) and before any A2 data.
The machine-readable form is [`protocol_a2.json`](protocol_a2.json). It
contains A1 (the cycle-free target stream) as well.

## What A1's partial run showed

90 of 223 A1 IC records failed. Each failure was a run of LA calls that all
returned nothing, ending at the target cap. The cause is in the frozen LA:

- Wiedemann ran on the square padded matrix `B = Mᵀ`.
- It returned `B^t·g(B)v` with `t ≥ 1`.
- That vector lies in the image of `B`, which is supported on the first
  `ncols` relation rows.
- So it could only ever find a dependency among the first `ncols` rows.

In the frozen run this mostly went unnoticed. Its early rows usually miss
columns that later rows introduce, so the first `ncols` rows are often already
dependent. With the A1 stream that happened less often.

The frozen run's IC records and A1's partial records are kept and reported.
Neither is used as primary data.

## Change

The LA is replaced in every IC arm by incremental sparse echelon elimination
mod ℓ on the augmented rows `[M | a | b]`:

- each new relation is reduced at its smallest column against the pivots;
- if it survives, it becomes a pivot;
- if its `M` part vanishes, it gives `A + B·d = 0`;
- the solve stops at the first such row with `B ≠ 0`.

This is the earliest stop any LA can make on the same relation stream. It
replaces the `rows ≥ cols + 4` trigger.

Accounting:

- every product mod ℓ is one `z`;
- every addition mod ℓ is one `za`;
- an inversion mod ℓ is `2·bits(ℓ)` `z`.

Everything else is A1 unchanged:

- the AP relation-target stream;
- arms, factor base, curves, targets, seeds;
- the analysis and the reused frozen rho records.

The A1+A2 harness passed its self-test before this freeze. That self-test
covered 36 solves, all correct, on separate `selftest-*` instances, not on
evaluation curves.

**Primary IC data:** `data/runs_a2.jsonl`.

# Root-finding kernel baseline: protocol (frozen before data)

Date: 2026-11-17. Status: local diagnostic. Every figure this produces is
`[pcc:unverified]`: one shared 4-vCPU cloud VM, no CPU pinning, no hardware
PMU, no independent replay. It is a baseline for planning and not a claim.

## Question

What does one target of the root-finding decomposition cost today, per phase
and per primitive, on the CPU path? The survey of earlier studies
(`SURVEY.md`) found counted cost S and whole-process wall times, but no
per-kernel time across factor-base sizes.

## Instrument

`examples/rootfind_baseline.rs`, release build, one thread. For each cell it
times `prepare` (the pair-table build) and `decompose` over a fixed target
list, and reads the oracle's own ledger (additions, field products and
inversions, lookups).

## Cells

- Prime curves (u64 field): 24, 32 and 40 bits (seed 59297);
  `prime-abscissa:size=A` for A = 64, 256, 1024.
- Koblitz curves: n = 17 with `binary-subspace:dimension=6`, n = 23 with
  dimension 8.
- 256 targets per cell, drawn by a fixed xorshift from the generator. Random
  targets are almost all refutations, so a target costs the full 2A signed
  points and probes. The `found` count is reported.

## Arms

`rootfind-mitm` (serial inversions), `rootfind-mitm-prime-batch` (one
inversion per block of 32), `rootfind-subtract` at m = 3 (prime only),
`rootfind-pairs` (memoryless; A ≤ 256 only), and `rootfind-mitm-aa`, a
second copy of the first arm.

## Controls and confounders

- A/A: `rootfind-mitm-aa` against `rootfind-mitm` gives the noise floor. A
  difference between two arms smaller than the A/A spread is not reported as
  a difference.
- Order: arms run interleaved, rotated one place each round.
- Warm-up: round 0 is discarded. Five measured rounds; median, min and max.
- Same targets, same base, same binary for every arm of a cell.
- Counted work is deterministic and must be identical across rounds.
- Not controlled: other tenants on the host, frequency scaling, NUMA. This
  is why the result is a diagnostic.

## Derived quantities

ns per target, ns per charged addition, ns per table-build addition, and
field operations per addition. A separate micro-kernel times `add`,
`add_many` and `add_many_batched` over 256 fixed points with no table.

## Decision use

The numbers feed `PLAN.md`: each proposed lever gets a predicted factor
computed from these shares, and the cheapest test that would falsify it.

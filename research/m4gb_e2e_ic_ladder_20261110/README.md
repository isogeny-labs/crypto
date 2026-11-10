# M4GB end to end: the whole index-calculus pipeline on a curve ladder (2026-11-10)

Owner's ask after the descent ladder: a full end-to-end benchmark. Instrument:
`ic bench --sweep` ([`src/bin/ic/bench.rs`](../../src/bin/ic/bench.rs),
`docs/ic/FRAMEWORK.md`): on one curve with planted logarithms it builds the
factor base, runs the decomposition oracle until the relation matrix has
rank, eliminates, recovers the logarithm and checks it against the planted one,
and prices every phase in group-addition equivalents, `S = total / √r`,
beside the matched counted Pollard rho (`--rho-runs 16`) on the same subgroup
and targets (`s_over_rho`).

**Status words.** Local development check on the session container (4 vCPU
Intel Xeon 2.10 GHz, 16 GiB), `[pcc:unverified]`; not isolab, not a
leaderboard row (AGENTS.md validation gate). Benchmark record
`benchmarks/records/20261110T1937Z-m4gb-e2e-ic-ladder.json`.

## Cells

One sweep file per instance under `sweeps/` (each `ic bench --sweep` writes
its own JSON under `results/`): Koblitz curves of degree 13, 17, 19, 23 and
random binary curves of degree 15, 21 (`max_cofactor` 8). On each, the same
`binary-subspace:dimension=⌈n/2⌉` factor base (`2⌈n/2⌉` boolean unknowns, `n`
quadratics at `m = 2`) in front of:

- `mitm:m=2`, the pair-table oracle: the non-algebraic reference row;
- `descent-algebraic:m=2` with `inherited-f4` (the Koblitz oracle's
  degree-bounded hybrid), `f4-f2` (in-house full reduced basis) and
  `m4gb-f2` (M4GB upstream `01860a52` + patches 0001/0002, one thread,
  narrowest binary that fits, 60 s per call).

Repeats 2, seed 20261110, `--targets walk`, `incremental-gauss`.

## How the solver's work enters S

`ic_framework` prices word-XOR counts at the host's measured
`ns_per_word_xor / ns_per_add` ratio and every other unit, M4GB's included
(it reports no count), at measured wall time over the host's addition time
(`priced_by: measured`, see `ic_framework/mod.rs`). So the `m4gb-f2` rows
carry the solver's whole wall time per call, process start and file round
trip included, converted to additions on this host.

## Pre-stated expectation

From the per-call ladder (`research/m4gb_descent_ladder_20261110/`): the
pair table and `inherited-f4` give the lowest S everywhere; `m4gb-f2` lands
between `inherited-f4` and `f4-f2` from about 22 unknowns (degree 21 and up)
and above both below that; every row recovers the planted logarithm
(`verified: true`) or says why not `[pcc:unverified]`; every index-calculus row stays above the
matched rho (`s_over_rho > 1`), as on every class measured so far.
Falsifier: an `m4gb-f2` row with `S` below the `inherited-f4` row of the same
cell, or any row below its matched rho.

## Results

(filled by `summarize.py` from `results/*.json`; see `RESULTS.md`)

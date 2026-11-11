# M4GB as a persistent server: one process per binary instead of one per call (2026-11-11)

Owner's question (2026-11-11): "Any other perf optimizations here? Can we use
tricks like dynamic programming or things that I'm not aware of for Gröbner
bases?" The catalogue of levers, with what this repository already does and
what remains, is [`docs/ic/GROEBNER_PERF_LEVERS_20261111.md`](../../docs/ic/GROEBNER_PERF_LEVERS_20261111.md).
This study measures the first lever on that list, the only one that needs no
change to the algorithm: the end-to-end ladder
([`../m4gb_e2e_ic_ladder_20261110/`](../m4gb_e2e_ic_ladder_20261110/)) spends
one process start, one field-table build and one thread-pool start per
decomposition call, and at 12 to 16 unknowns the solve itself takes about as
long as that fixed cost.

## What changed

- M4GB patch `0003-serve-mode-persistent-solver-process.patch`
  ([`third_party/m4gb-patches/`](../../third_party/m4gb-patches/)): a `--serve`
  mode. One solver instance reads `SOLVE <in> <out>` lines from stdin, runs
  each through the unchanged `solve()` entry point (which re-initialises the
  solver's state), writes the output file and answers `OK`, `UNSAT` or
  `ERR <what>`; `QUIT` or end of input stops it. The one-shot `--solve` path is
  untouched; upstream's algorithm is untouched.
- Rust adapter `run_m4gb_persistent` in
  [`src/cryptanalysis/m4gb_oracle.rs`](../../src/cryptanalysis/m4gb_oracle.rs):
  one server per binary (`MAXVARS` ladder), kept in a process-wide pool, one
  request at a time per server, the same deadline as the one-shot path (the
  server is killed and replaced on a timeout). Registered as the engine
  `m4gb-f2-server` beside `m4gb-f2`, so `ic descent --solver` prices both on
  the same targets in one invocation.

## Protocol

`run_ab.sh` under `tools/benchrec.py run`: `ic descent --json --solver
inherited-f4 --solver m4gb-f2 --solver m4gb-f2-server`, one invocation per
cell and family, 6 seeded targets, 3 repeats, 60 s budget per call, one
thread (`M4GB_THREADS=1`), binaries `solver_m4gb_n{16,20,24,28,32}_gf2` from
`scripts/m4gb/build.sh` at the pinned upstream commit with patches 0001 to
0003. Cells: Koblitz (K) and random binary (R), m = 2 at n = 11 to 23 (12 to
24 unknowns), m = 3 at n = 11, 13, 15 (12 to 15 unknowns). The runs are
interleaved per target and the engine order rotates per repetition, so host
drift falls on every engine. Local container, no isolation; a stage
diagnostic, not a leaderboard row.

Raw mechanism check before the run (100 calls on one 16-variable descent
system, `research/m4gb_integration_20261110`'s `wd_n16.in`, shell timing, two
rounds) `[pcc:unverified]`: one-shot 30.7 and 31.3 ms per call, server 24.3
and 26.7 ms per call, process start plus option parsing alone 4.1 ms per
call; the two output files are byte-identical.

## Pre-stated expectation and falsifier

Expectation: the server removes a constant 5 to 15 ms per call (process start,
field tables, thread pool, and on the Rust side the pipe set-up), so the
server-to-one-shot ratio is lowest at 12 unknowns and rises towards 1 as the
solve grows past a few hundred milliseconds; the decisions agree on every
target. Falsifier: a ratio at or above 1 on a cell where the one-shot call
takes under 50 ms, or any disagreement with the reference, means the server
path costs more than it saves (pipe round trip, file writes, lock) or is
wrong.

## Results

Filled after the run (`RESULTS.md`, `results.json`, `results.svg/png`,
`REPORT.pdf`).

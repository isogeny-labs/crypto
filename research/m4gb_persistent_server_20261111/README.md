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

## Results `[pcc:unverified]`

Run 2026-11-11 20:24 to 21:28 Z, record
`benchmarks/records/20261111T1946Z-m4gb-persistent-server.json` (measured;
raw `results.tar.gz`, `results.json`; 4 metrics). Full table: `RESULTS.md`;
graph `results.svg/png`; `REPORT.pdf`. The Koblitz degree-21 cell does not
exist (composite degree) and was dropped by `ic descent`, as in the earlier
ladder.

| family | unknowns (m) | one-shot ms | server ms | server / one-shot | paired saving ms (targets server below one-shot) | `inherited-f4` ms |
|:--|--:|--:|--:|--:|--:|--:|
| K / R | 12 (2) | 8.90 / 9.00 | 3.22 / 4.12 | 0.36× / 0.46× | 5.7 (6/6) / 5.0 (6/6) | 0.53 / 0.65 |
| K / R | 14 (2) | 15.0 / 15.4 | 7.52 / 10.9 | 0.50× / 0.71× | 5.7 (6/6) / 5.9 (5/6) | 0.87 / 1.20 |
| K / R | 16 (2) | 20.1 / 22.7 | 13.9 / 17.0 | 0.69× / 0.75× | 6.1 (6/6) / 5.0 (6/6) | 1.98 / 1.71 |
| K / R | 18 (2) | 35.3 / 27.9 | 35.3 / 23.9 | 1.00× / 0.85× | 6.1 (4/6) / 3.8 (5/6) | 2.54 / 2.42 |
| K / R | 20 (2) | 33.3 / 45.3 | 26.5 / 37.2 | 0.80× / 0.82× | 7.9 (6/6) / 6.7 (5/6) | 4.20 / 5.42 |
| R | 22 (2) | 196.3 | 170.2 | 0.87× | 12.5 (5/6) | 7.92 |
| K / R | 24 (2) | 421.2 / 289.4 | 402.0 / 285.7 | 0.95× / 0.99× | 23.4 (5/6) / 20.4 (4/6) | 14.5 / 14.3 |
| K / R | 12 (3) | 6,207 / 6,124 | 6,364 / 6,458 | 1.03× / 1.05× | −188 (2/6) / −312 (1/6) | 22.2 / 24.0 |
| K / R | 15 (3), n = 13 | 46,187 / 46,591 (2/6 decided) | 45,441 / 46,297 (2/6) | 0.98× / 0.99× | 745 (1/2) / 294 (2/2) | 186 / 176 |
| K / R | 15 (3), n = 15 | over budget 6/6 | over budget 6/6 | n/a | n/a | 172 / 175 |

Medians over targets of the per-target median of 3 repeats, ms per call.

- **The fixed cost is 4 to 8 ms per call, and the server removes it.** `[pcc:unverified]` On the
  ten m = 2 cells whose one-shot call takes under 50 ms the paired per-target
  saving is 3.8 to 7.9 ms (median over cells 6.1 ms) and the server's median
  wall is below the one-shot's on 55 of 60 targets. The ratio falls from 0.36
  at 12 unknowns to 0.95 to 0.99 at 24, where the solve is 300 to 400 ms.
- **The falsifier fired on one cell by its summary statistic.** K at 18
  unknowns shows 1.00× by the median-of-medians (its six targets split into
  a 19 ms and a 35 to 40 ms group, and the two statistics landed on different
  targets); per target the server is below the one-shot on 4 of 6 with a
  median saving of 6.1 ms. The other nine short cells are at 0.36 to 0.85×.
  Read as: the effect is a constant of a few ms, and a cell summary at 35 ms
  per call sits inside the per-target spread.
- **Long computations are slightly slower in the server.** On the cubic 12-unknown
  cells (about 6 s per call) the server is 3 to 5 % slower on 9 of 12 targets.
  The one-shot process starts every solve from a fresh heap; the server
  carries the allocator state of its previous solves into a computation
  whose cost is dominated by the Gebauer–Möller update's allocations. Not
  isolated here (no counters on this host); a `MALLOC_ARENA`/`malloc_trim`
  experiment is the cheap next check.
- **Ranking unchanged.** `inherited-f4` is 6 to 28× below the server on every
  m = 2 cell and 240 to 290× on the cubic cells; the server narrows the gap
  to the in-house engine at 12 unknowns from 17× to 6×, nothing more. The
  end-to-end conclusion of the e2e ladder (M4GB never the best engine) stands.
- 0 disagreements across 20 cells.

Expectation held on nine of ten short cells and on the direction of the
ratio with size; the cubic-cell slowdown was not predicted and is the one
new fact. Not isolated (local container, one thread); no leaderboard row.

## Sources checked

KB (`research-kb`) unavailable in this session. `docs/ic/GROEBNER_PERF_LEVERS_20261111.md`,
`research/m4gb_descent_ladder_20261110/RESULTS.md` (cell list and the same
missing K 21 cell), `research/m4gb_e2e_ic_ladder_20261110/README.md`, M4GB
`src/common.hpp`, `lib/logger.hpp`, `lib/parser.hpp` (missing-file path).

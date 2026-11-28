# Where a complete index-calculus solve spends its time (2026-11-06)

**Status: measurement BLOCKED. No profile was taken. No speedup is claimed.**

Lane 2 of a four-lane parallel run. Branch `perf/solve-profile-20261106`.
This directory lands the analysis tool, its self-test, the frozen plan, and a
ranking built only from numbers other sessions already measured. Everything is
labelled measured (by someone else, with the source), inferred, or planned.

## Why nothing was run

1. **Resume checkpoint gate.** `scripts/checkpoint.sh push` fails: the upload
   token expired at 2026-11-05T02:54:53Z, and the `aws` CLI is not installed on
   this host, so the AWS fallback does not exist either. The rule is to stop
   before any run. I did not relabel another token.
2. **No hardware PMU, no perf.** `/sys/bus/event_source/devices/` lists
   breakpoint, msr, power, software, tracepoint, uprobe and no `cpu` PMU;
   `perf` is not installed; `tools/profile.sh` stops at "needs gawk". So cycles,
   IPC, top-down and perf-sampled attribution are refused here, not estimated.
   Callgrind is installed; it would give deterministic instruction counts per
   function (not cycles or wall time).
3. **Box limits.** 101 MB of root disk free after the sparse checkout (not
   1.6 GB), so `/home/user/wt/perf2` did not exist and I created the sparse
   worktree from `/home/user/crypto` instead. A release build was started on
   `/dev/shm` under `flock`; it was not needed once the gate failed.

## Measured by others (borrowed, not re-measured, not validated)

Source: `research/prime_ic_program_20261029/hashtable_prefetch/stage_split.csv`,
which copies PR #2806 `results.json` (branch head `db02dd4f`, not on `main`).
Per relation trial, prime curve, `mitm-prime-batch:m=2`, `walk-prime-batch`
with 32 lanes, one session per size, TSC cycles at 2.1 GHz. That PR calls these
sessions descriptive and not replayable. They cover relation collection only,
not table build, linear algebra or descent, and carry no run-to-run spread.

| stage (ns/trial) | 28 bit | share | Amdahl cap on the trial | 32 bit | share | Amdahl cap |
|:--|--:|--:|--:|--:|--:|--:|
| oracle probe | 80.9 | 25.4% | 1.34x | 90.6 | 26.0% | 1.35x |
| guard insert | 56.3 | 17.7% | 1.22x | 49.2 | 14.1% | 1.16x |
| touch (demand read of both tables) | 30.6 | 9.6% | 1.11x | 47.9 | 13.7% | 1.16x |
| target-walk addition | 39.6 | 12.4% | 1.14x | 40.5 | 11.6% | 1.13x |
| other (draw, prep, queue, rest) | 110.6 | 34.8% | | 120.4 | 34.5% | |
| **guard + probe + touch** | 167.9 | 52.8% | **2.12x** | 187.7 | 53.8% | **2.17x** |
| total | 318.1 | | | 348.7 | | |

The cap is 1 / (1 - share): the most the trial could gain if that stage cost
nothing. It is an upper bound, not a forecast.

## Inferred (no measurement behind it)

- Each stage interval contains one clock read (the PR times the loop body with
  `rdtsc`). If a read costs a few tens of cycles, the 137 ns "floor"
  (guard + probe) is inflated by roughly two reads per trial, and the true
  hash-table share is smaller than 53%. The size is not known; quantifying it
  needs the timer-off versus timer-on comparison planned below, which is
  crypto#2844's question and is still open.
- The hash probe is almost always a miss (about 1/1024 hit at 28 bits and
  1/4096 at 32 bits, derived in the hashtable_prefetch note), so the miss path
  is the one a layout change would have to speed up.
- Whole-solve ranking (collection vs table build vs linear algebra vs descent)
  cannot be ranked from borrowed data. Without it, "optimize hash tables" is a
  guess about the largest phase, which is the thing this lane was set up to
  avoid.

## Plan (frozen here, not executed)

Targets and phases:

- Prime: `ecbench` `ic.pipeline`, `factor_base=prime-abscissa:size=512` (28 bit,
  curve `prime_search` seed 59297) and `size=1024` (32 bit, seed 20261018202),
  `oracle=mitm-prime-batch:m=2`, `targets` in `walk`, `walk-prime-batch-4/8/16`.
  `walk-prime-batch-32` is not on `main` (stages.rs parses 4, 8 and 16 only); it
  exists only on the PR #2806 branch, so it is out of scope until that merges.
- Koblitz: `examples/koblitz_orbit_dlp_fast_online` at `construct:41:0` (K=125),
  which clocks its own phases on one chain of laps.
- Phases from `phases[].wall_ns` in the ecbench record (`generic_ic_setup`,
  `factor_base`, `precompute` = table build, `queries`, `pdp`, `relation_check`,
  `matrix_build`, `relation_la`, `target_*`, `recovery_check`).
- Bias test: the same child run (a) with the phase clock on and (b) as a whole
  with `ecbench exec` timed outside, plus Callgrind Ir per function. The
  difference of (a) and (b) bounds the cost of the diagnostic's own clock reads.
  A perf-sampled comparison needs a host with a PMU or at least `perf` with
  `cpu-clock`; neither is here.
- Repeats of at least 7 interleaved runs per arm under `/dev/shm/cpu.lock`,
  `timing.want` raised while waiting. Per-phase mean and sample sd from
  `analyse.py`.
- Report order: hotspot table with errors, phase bar chart, Amdahl ranking with
  measured and inferred columns kept apart.

## Files

- `analyse.py`: per-arm, per-phase share, sd across runs, Amdahl cap from
  ecbench `records.jsonl`. Unverified against real records (none exist yet).
- `test_analyse.py`: checks the arithmetic on an invented fixture (passes:
  `python3 -m unittest test_analyse`). The fixture is not a measurement.

## Not done

No PDF (typst is not installed), no flame graph or phase chart, no
`benchmarks/` entry (nothing was run, so repo rules give it no row), no
leaderboard change, no pcc claim. `conductor` CLI is not installed, so no
Conductor task was attached. Sources checked: the repo files named above; the
research-kb MCP server was not reachable from this subagent, so no
novelty claim is made.

# Resume: crypto @ claude/zealous-meitner-85ipb5

## Goal
Owner (2026-11-11): "Any other perf optimizations here? Can we use tricks like dynamic programming
or things that I'm not aware of for grobner base" — then "Go". Catalogue the levers
(docs/ic/GROEBNER_PERF_LEVERS_20261111.md) and measure the first one: a persistent M4GB server
process (patch 0003 `--serve`, Rust `m4gb-f2-server`).

## Lead / hypothesis
Per-call fixed cost (process start, field tables, thread pool, pipes) is 5 to 15 ms and dominates
decomposition calls at 12 to 16 unknowns. Expectation and falsifier: research/m4gb_persistent_server_20261111/README.md.

## Spec
research/m4gb_persistent_server_20261111/run_ab.sh under tools/benchrec.py run
(record benchmarks/records/20261111T1946Z-m4gb-persistent-server.json): ic descent --solver
inherited-f4 --solver m4gb-f2 --solver m4gb-f2-server, K and R, m=2 n=11..23, m=3 n=11,13,15,
6 targets x 3 repeats, 60 s budget, one thread. Binaries: scripts/m4gb/build.sh n{16,20,24,28,32}_gf2
with patches 0001-0003 (scratchpad prefix m4gb-prefix-v3). Local container, unisolated (stage
diagnostic, no leaderboard row).

## State
Patch 0003 written and clean-built; Rust client in progress (subagent); record open; run next.

## Runs in flight
None yet.

## Stop and success conditions
All cells finish or fail individually; summarize.py -> RESULTS.md/results.json/svg; make_report.sh;
benchrec add-raw/metric/close; session-log entry; PR on the branch (PR #3936 is the open
predecessor, auto-merge enabled).

## Next steps
1. cargo build --release --bin ic with m4gb-f2-server; cargo test --lib m4gb_oracle.
2. Run run_ab.sh under the record; summarize; report; close record; benchmarks entry; session log; push; PR.

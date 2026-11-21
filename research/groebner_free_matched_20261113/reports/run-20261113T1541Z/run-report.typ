#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: run-20261113T1541Z

*Command.* `python3 tools/benchrec.py run benchmarks/records/20261113T1539Z-ic-m3-linear21-vs-floor.json -- research/groebner_free_matched_20261113/run_grid.sh`. Commit at launch `c90b1d9d54`; binary `target/release/examples/ic_algebraic_floor`, SHA-256 `2685e5140daa6b801af15ac4d248dfbbcee1aabdd9aff9c00e68436e4dd6aba1` (`runs/binary.sha256`).

*Host.* Shared cloud container, 4 vCPU Intel Xeon 2.10 GHz, 15 GB, isolation L0. One process at a time, each solve on one core.

*Arms and cells.* `run` (floor truth over a 2,048-target pool; 12-target child solves, 120 s and 4 GiB each) with arms `f4-chain`, `lin21`, `lin21-ht` on 0:13:5, 1:17:6, 1:19:7, 1:23:8, 1:29:10, 0:31:11, and `lin21`, `lin21-ht` on 0:37:13, 0:41:14; `linpool lin21-ht` on all of those (2,048 targets) and on 1:47:16 (2,048), 0:53:18 (1,024), 1:59:20 (512). Seed 20261014.

*Completed.* 11 of 11 cells; 22 arm files, 264 child solves, 0 censored, 0 unsupported, 0 failures. 8 truth files, 11 pool files.

*Checks recorded.* Pool verdicts against the floor: 0 disagreements in 16,384. Child solves: 264 of 264 agree with the floor, every witness re-added in the group. Floor against exhaustive enumeration on the 12 arm targets: equal on the four cells where enumeration ran (n = 13, 17, 19, 23).

*Measured values* (wall ns, one core): see `SUMMARY.md` (whole pool) and `ANALYSIS.md` (12-target arms, frozen 2026-10-14 estimator). Floor, mean decision per target: 2.0 µs (n = 13) to 982.7 µs (n = 41). `lin21-ht`, mean per target: 5.7 µs (n = 13), 6.26 ms (n = 41), 682.7 ms (n = 59).

#figure(image("../../figures/cost_per_relation.svg", width: 90%), caption: [Recorded cost per relation by arm and cell, run 20261113T1541Z.])

*After the grid, same commit plus the `s4check` subcommand:* `s4check` on six cells, 1,024 targets each: 2,095 witnesses, identity zero on 2,095, control zeros 0 (`S4CHECK.json`). Not part of the frozen protocol.

*Raw records.* `research/groebner_free_matched_20261113/runs/`, `runs.log`.

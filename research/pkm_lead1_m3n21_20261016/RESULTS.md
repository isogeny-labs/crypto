# PKM lead 1 at `m = 3`, `N = 21`: results (observations)

Run on 2026-10-18 under the frozen [PROTOCOL.md](PROTOCOL.md) on a 32-vCPU,
124 GB EC2 spot host (deviation from §2's 192 GB floor, disclosed before
stage 2 in `runs/host-ip-172-31-35-131….txt`; the rules for `A` and `C` were
applied unchanged: `A` = 119,416,658 KiB, `C` = 2.03·10¹⁰). Run report with
the graph and PDF: [`reports/run-20261018T0611Z/`](reports/run-20261018T0611Z/report.pdf).
Every run on the host: `research/large_memory_runs_20261018/EXPERIMENT_LOG.md`.

| stage | recorded |
|:--|:--|
| identity (`N = 15`, target 0) | passes: every compared field equals round 6's row; native check agrees (52 s on 32 threads) |
| main, target 0 | 27 steps, 6,445 s; every degree-7 pair reduced without refuting (`last_productive_degree` 7); stopped in step 27, the first degree-8 step, by `--max-dense C`: `B'` would need 79,330,205,871 entries; `degree_reached` 8, `inconsistent` false, peak resident 67.1 GB |
| main, target 1 | identical on every field (width 667,444 at the stop, 271,278 at the last finished step, same `B'` demand), 6,416 s |
| confirmation | not applicable (no target finished with a `D`) |
| verify | identity row agrees with exhaustive search; the `N = 21` rows carry no verdict to check |

*Derived:* the degree-8 step's `B'` is ≈ 317 GB at 4 bytes per entry, ≈ 460 GB
at round 7's peak-to-`B'` ratio 1.45. A 192 GB host would have stopped at the
same cap. The card's 133–146 GB projection assumed `D` stays at 7 (§4.4).

**Reading under §5 (for the result PR, not made here).** The recorded trace
has the degree-7 closure exhausted without refuting on both targets and no
productive degree-8 reduction. §5's rows "stopped with `D ≥ 7` only
(inconclusive)" and "stopped after a productive step at degree ≥ 8 (H0 from
a lower bound)" bracket it; the second needs a productive degree-8 step,
which the cap prevented. No system was re-run.

#set page(margin: 1.6cm)
#set text(size: 9pt)
= Run report: eliminated S4 vs chained S3 oracle (run-20261018T2059Z)

*Command.* `research/s4_oracle_cost_20261018/run.sh`, engine `examples/s4_oracle_cost.rs`, preregistration commit dba1211d3. Host: shared 4-core x86-64 Linux container, 1 rayon thread, `prlimit --as=3000000000`, matrix cap 2^26 words, F4 budget 600 s. Isolation tier: none earned (load average 1.2 to 4.2), so wall seconds are a practicality note only. Queue 20:59:58 to 21:06:55 UTC. Raw records: `runs/cell-*.jsonl`.

*Observations (measured).* Cells (12,3), (13,4), (16,5), (18,6); 4 draws requested per cell (2 for (18,6), where the stop rule fired). Every finished record was verified against exhaustive enumeration. Chained solving degree 3 or 4, eliminated 7 on every measured draw. Medians of word_xors over verified draws, chained / eliminated: (12,3) 1.98e6 / 3.62e5; (13,4) 2.59e8 / 5.40e7; (16,5) 2.46e10 / 2.32e10 (chained has 2 of 4 draws measured). (18,6): both arms unmeasured (chained oversize, eliminated allocation failure under the 3 GB cap). A/A at (13,4) draw 0: word_xors spread 0 for both arms.

#figure(image("oracle_cost.svg", width: 100%), caption: [Recorded F4 word_xors and solving degree per draw, run-20261018T2059Z; crosses mark unmeasured draws; n = 4 draws per cell except (18,6).])

*Failures recorded.* Chained oversize: (13,4) d1, (16,5) d1 and d3, (18,6) d0 and d1. Eliminated rc 134 (allocation failed): (18,6) d0, d1. Not run: (21,7), (24,8).

*Scope.* Stage observation only; no end-to-end S moves; see `RESULTS.md` for verdicts, prediction outcomes and limits.

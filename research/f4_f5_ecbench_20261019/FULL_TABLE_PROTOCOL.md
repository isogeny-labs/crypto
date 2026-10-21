# Single-worker full Semaev table construction

Initial ecbench source0acfc55b7f verifies and replays every full-table
execution, but39/40 n17 and40/40 n23 measured records remain below L2.
Retained blockers include150.4 and177.4 preemptions/second, without steal
or foreign user work in those samples. PairSumTable::build_full_within
unconditionally submits its row construction and tuple sort to Rayon.
The frozen resource envelope has exactly one Rayon worker on one CPU.

Hypothesis: avoidable main/worker handoffs contribute to the short-call
preemptions and setup cost. Before editing, freeze a serial path only when
RAYON_NUM_THREADS is explicitly1. Other configurations retain the existing
parallel code. Use the identical batched point sums and full lexicographic
(sum,i,j) ordering; budgets, tuples, lookup witnesses and group accounting
must be unchanged. No new method search rule or relaxed isolation threshold.

Test serial construction against an explicit four-worker pool on identical
bases, every sorted entry, bucket, presence-filter word and budget boundary.
Existing m2/m3 oracle replay tests and full ecbench policy integration remain
mandatory. Include this source in the final source/binary manifests and rerun
the full native and unchanged full ecbench panels on fresh dedicated hosts.
The native rerun additionally pairs full-table cold construction/query calls
at four workers, n17/n23 planted seed230, five A/A and five A/B rounds with
the same first-eligible-three-attempt and exact-output/bootstrap rules.
Report the native harness's earned levels and all failures even if this does
not improve L2 admission; no admission improvement is presumed in advance.

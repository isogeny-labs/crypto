# Round22 report

All 180 executions completed, including 162 measured and 18 warmups. All 60 stage rows validated; full certificates matched and 500 worker-manifest entries verified before and after native export.

Raw evidence was published first at 0314a1b2bb18671ee53c21ad3e88e04456e06e77. report.pdf contains three inspected pages. latency.svg is the frozen analyzer output; stages.svg is rendered from stage-summary.csv using stages.awk. Recorded summaries retain min/median/max, n=9. These are L0 diagnostics, not isolated acceptance.

See ../../round22/EXECUTION.md and execution-outcome.json for operational receipts. One bounded attempt completed; no retry or pool-cap change. Oracle work is the largest measured individual stage in fixtures 1/2, directing the next source investigation.

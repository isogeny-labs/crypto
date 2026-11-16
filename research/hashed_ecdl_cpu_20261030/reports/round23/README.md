# Round23 completed suffix-filter comparison

Raw evidence was published first at 96a4b27eafd5a9a584dbc7684aeada0615eaf719. All 180 executions and 18 cells passed with identical full certificates. All 379 worker-manifest entries verified before and after native export. The two report pages were rendered and inspected; the initial unescaped multiplication symbol was corrected before successful rendering.

The graph is the frozen analyzer output, with source in ../../round23/analyze.rs and underlying rows in ../../round23/results/rows.csv. Summary medians/ranges are retained beside the graph. Descriptive complete-median reductions at batch64 are 4.19/2.65/2.99 percent and at batch1 1.77/2.64/2.53 percent, calculated as 100*(1-on/off). Fixture0 ranges overlap; fixtures1/2 matched ranges do not. These remain L0 observations.

One authorized attempt completed with 2 vCPUs/4 GiB, zero retries, a 300-second deadline and unchanged node cap. No data omitted; no new attempt authorized. See ../../round23/EXECUTION.md for exact operational custody.

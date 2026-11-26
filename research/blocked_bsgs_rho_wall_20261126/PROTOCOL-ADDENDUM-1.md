# Protocol addendum 1: admitted CPU envelope

Filed before any timing run.

This addendum supersedes only the CPU-allocation sentence in section 7 of
`PROTOCOL.md`. The admitted IsoLab native-L2 lane allocates four vCPUs, not two.
The benchmark solver remains pinned to one workload CPU with
`RAYON_NUM_THREADS=1`; the remaining CPUs are reserved for the parent process
and isolation machinery and may not run a competing benchmark workload.

The 4 GiB memory limit, 60-minute job limit, two-attempt limit, USD 2.00 cost
ceiling, algorithms, fixtures, seeds, schedules, estimands, censoring rules,
and decision thresholds are unchanged. No timing data had been collected when
this correction was filed.

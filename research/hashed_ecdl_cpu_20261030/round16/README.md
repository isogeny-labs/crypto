# Round16: bounded inverse-cache comparison

Five arms, three fixed fixtures, one warmup and nine measured repetitions:
150 complete executions, full certificate equality and corrupted-scalar controls.
Original baseline remains 2bd4588fba60. Candidate is integrated 5d7dcb67616c;
reference is integrated 4c03472f6bdd. Source manifests and binary hashes are
frozen before data. No snapshot patches are applied in this panel.

Table construction remains charged to setup. Candidate adds 8*p bytes plus
Vec overhead for p<=4096 (at most 32 KiB of entries). The existing retained
sum-table byte metric excludes this auxiliary memory and is not peak RSS.
Query-inversion counters count requests, including table lookups, not Euclid
executions. Larger fields retain extended Euclid.

Run once on EKS CPU Spot, 2 vCPU/4 GiB, 300 seconds, no automatic retries.
Ordinary jobs are non-resumable; interruption invalidates clocks and panel
completion, and remains a reported failure. L0 diagnostic timings cannot
complete the further10x goal or establish external/rho advantage.

T-934 owns this panel and its report. The prior candidate passes 34 release
tests and strict Clippy; the current source is byte-identical across the
subsequent merge of main. Full root/CUDA compilation remains unperformed.

# N83 high-K RunPod attempt: independently replayed first points, censored panel

The first public point on each of two RunPod allocations completed all three
K3300/K4300/K4405 IC arms and their same-point rho pairs. The completed IC
children passed independent base, relation and scalar replay. Their measured
one-target online times, paired rho times, A/A calibration and resource flags
are preserved per candidate in [`outcome.json`](outcome.json).

The sealed 16-point panel is `BOUNDED_UNKNOWN`, with
`selected_candidate_id: null` and Tier 1 `NOT_ADMITTED`. On allocation `s0`,
the K3300 A/A median spread was 12.585285345% against the predeclared 10%
limit. On allocation `s1`, the K3300 IC child on point 8 recorded five CPU
throttled periods. Both producers were stopped after their first full point;
the remaining point events were never run. The partial independent verifier
reported `VERIFICATION_FAILED` for the incomplete expected event schedule,
while retaining `PASS_INDEPENDENT_REPLAY` for each completed IC and rho child.
The supervised stop also preceded the runner's final allocation-receipt copy
into its archive, so the full aggregate gate did not pass.
Neither the completed points nor the later 20% retry threshold enter this
attempt's factor-base selector.

The two raw archives were copied and SHA-256 checked locally before each Pod
was stopped and terminated. `outcome.json` binds their hashes, member counts,
Pod and machine IDs, first-point receipts, stop times and a compute-only cost
estimate of $0.498230 for these two allocations. The archives are retained in
the experiment evidence store under
`/Volumes/SSD990-2/tmp/n83_highk_v1_results_20261019/{s0,s1}/raw_results.tar.gz`.
The stop and termination receipts are retained beside them. Volume charges
between stopping and termination are outside the compute estimate.

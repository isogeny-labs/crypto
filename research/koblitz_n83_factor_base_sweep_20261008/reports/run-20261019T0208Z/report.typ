#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
= Separate upstream K4096 native-worker receipt
Job 20261019-020823-n83-k4096-phases-final-e51bdf · source 8856f2eb36e1aa132af2c0c20c9d422789ab0774 · c7i.xlarge

This renders existing upstream records merged from 02792dec6. It is not a new run by the native Rust harness. Native Rust worker, Bash orchestration and separate Python audits/tests remain labeled as their original provenance; no cryptographic Python replay was run to produce this visual.

*Reported outcome:* One public primary a=0 N83 K4096 m5 orbit-phase chained-S3 trial returns UNKNOWN_solver_cap at 1000 conflicts, with 0 models, 0 relations and rank 0. Limits: 600 s, 32 models, 150000 variables, 2000000 domain clauses. Docker: 1 CPU, 4 GiB, no swap, no network. L0 diagnostic wall, n=1, no A/A or uncertainty interval.

#image("phases.svg", width: 100%)

Recorded checkpoints, not exclusive phase durations. Model installed: 131958 variables, 2021197 clauses, 4980 XORs. Model ready at 780.945 ms; solver return at 138237.881 ms after entry. Original receipt reports 137.457 s in the solver, 148.223 s worker pre-summary wall and 751669248 bytes cgroup peak. Runner elapsed includes checkout/build/setup and is a separate interval. No speed ratio is inferred.

#image("execution.svg", width: 100%)

*Failures retained:* local Bash 3.2 preflight, paid input-access denial and paid missing-sidecar failure. These provide no solver outcome. The final job completed and its instance terminated. Original audits and four mutation controls are reported by the upstream receipt; this visual does not independently recertify them.

*Sources:* REPORT.md links the original result, protocol amendments, launch failures, native worker stderr, summary, resource records and audit manifests under verification/k4096-native-phases-20261018. Figure-data.json copies all 12 native trace rows. Editable source, SVG/PNG and this inspected PDF are included.

*Scope:* This source/host differs from the matched native K2048/K4096 run-20261019T0140Z. Do not combine their timings or checks. No natural-yield estimate, fastest base, useful rank, cold winner or nonexistence conclusion follows. Canonical leaderboard/scoreboard/timeline remain unchanged.

#set page(width: 21cm, height: auto, margin: 1.5cm)
= n = 31, m = 3: curve effect with 32 subspaces (2026-10-24, in progress)

*Status:* scaffold. The 32-V experiment is still running; the 4 V and 8 V arms below are
recorded values from earlier runs.

== Question and design
Pre-registered metrics: yield_per_Fm, log_us_per_call, work_per_call,
log_projected_solve. H0 bound: ICC_curve ≤ 0.05; H1 threshold: ICC_curve ≥ 0.10.
Arm: 32 curves, geometric V, key `n=31 a2=0 l=10 m=3`.
TODO-RESULTS: spec/command, commit, host, isolation level, runs requested and completed.

== Recorded ICC by number of subspaces
4 V (seeds 1–4) and 8 V (seeds 1–8) are plotted when present; 32 V (seeds 1–32) is
added when `icc_n31_m3_geom_s1-32.json` exists.

#image("icc_vs_subspaces.svg", width: 100%)

*TODO-RESULTS:* 32-V ICC and 95 % CI per metric, with the verdict against 0.05 / 0.10.

== Simulated power (work_per_call)
Recorded rows from `icc_n31_m3_geom_s1-8_power.json`: P(H0) and P(H1) against the
true ICC for 4, 8 and 32 subspaces.

#image("power.svg", width: 100%)

== Limits
Observations only; no result is claimed until the 32-V numbers are in.

== Records
`experiments/koblitz_presentation/icc_n31_m3_geom_diag.json`,
`icc_n31_m3_geom_s1-8_power.json`, `icc_n31_m3_geom_s1-32.json` (TODO-RESULTS).

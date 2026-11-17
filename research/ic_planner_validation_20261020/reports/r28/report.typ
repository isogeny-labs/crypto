#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session r28 (`ECBS1h7d5230a73add`)

*Protocol* PROTOCOL_AMENDMENT_1.md (primary). *Spec* `specs/r28.json` (`ECS1h80ec82dbe9f4`), commit `d1be0c1c8` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:42:39Z, finished 2026-10-20T17:46:56Z.

*Curve* `icv1-fp28-t28201-65099091` (registered; `prime_explicit`). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/r28.audit.json`).
*Isolation* (measured runs) double-base: L1 2, L2 22; full-rank: L1 14, L2 10; half-base: L1 8, L2 16; plan: L1 3, L2 21; plan-aa: L1 5, L2 19; random-targets: L1 22, L2 2; rho-neg: L1 1, L2 23; rootfind: L1 24.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [32.039], [33.346], [45.104],
  [full-rank], [93.157], [110.499], [266.584],
  [half-base], [29.299], [34.811], [84.412],
  [plan], [20.437], [23.231], [48.374],
  [plan-aa], [20.437], [23.231], [48.374],
  [random-targets], [1053.374], [1056.043], [1080.063],
  [rho-neg], [0.943], [1.035], [1.862],
  [rootfind], [8970.428], [9868.86], [17954.743],
)
Paired ratios against `plan`: `sessions/r28/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/r28/records.jsonl`. Observations only; the interpretation is in the study README.

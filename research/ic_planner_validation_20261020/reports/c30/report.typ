#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session c30 (`ECBS1h0d23937f2c51`)

*Protocol* PROTOCOL.md (supplementary). *Spec* `specs/c30.json` (`ECS1h576265fa490e`), commit `05a0ebd92` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:05:12Z, finished 2026-10-20T17:16:04Z.

*Curve* `find_prime_order_curve(30, 1)` (ecbench `prime_search`; unregistered, not named by slug). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/c30.audit.json`).
*Isolation* (measured runs) double-base: L1 6, L2 18; full-rank: L1 22, L2 2; half-base: L1 11, L2 13; plan: L1 5, L2 19; plan-aa: L1 7, L2 17; random-targets: L1 24; rho-neg: L1 2, L2 22; rootfind: L1 24.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [38.583], [40.181], [54.56],
  [full-rank], [116.301], [138.026], [333.546],
  [half-base], [32.762], [38.914], [94.283],
  [plan], [24.161], [27.454], [57.089],
  [plan-aa], [24.161], [27.454], [57.089],
  [random-targets], [1417.226], [1420.61], [1451.06],
  [rho-neg], [0.937], [1.03], [1.861],
  [rootfind], [15379.978], [16919.62], [30776.389],
)
Paired ratios against `plan`: `sessions/c30/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/c30/records.jsonl`. Observations only; the interpretation is in the study README.

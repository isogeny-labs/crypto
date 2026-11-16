#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session r26 (`ECBS1h339ff9352e64`)

*Protocol* PROTOCOL_AMENDMENT_1.md (primary). *Spec* `specs/r26.json` (`ECS1h50dce2aabcc3`), commit `d1be0c1c8` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:41:34Z, finished 2026-10-20T17:42:37Z.

*Curve* `icv1-fp26-tm1775-7e8fb6df` (registered; `prime_explicit`). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/r26.audit.json`).
*Isolation* (measured runs) double-base: L1 6, L2 18; full-rank: L1 5, L2 19; half-base: L2 24; plan: L1 1, L2 23; plan-aa: L1 3, L2 21; random-targets: L1 19, L2 5; rho-neg: L1 3, L2 21; rootfind: L1 18, L2 6.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [24.296], [25.376], [35.102],
  [full-rank], [58.453], [69.158], [165.509],
  [half-base], [23.559], [27.967], [67.64],
  [plan], [15.228], [17.279], [35.741],
  [plan-aa], [15.228], [17.279], [35.741],
  [random-targets], [747.613], [749.736], [768.842],
  [rho-neg], [1.046], [1.145], [2.035],
  [rootfind], [3551.026], [3907.129], [7112.055],
)
Paired ratios against `plan`: `sessions/r26/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/r26/records.jsonl`. Observations only; the interpretation is in the study README.

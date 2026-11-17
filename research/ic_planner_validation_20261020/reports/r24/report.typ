#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session r24 (`ECBS1hcf6d4ac07e64`)

*Protocol* PROTOCOL_AMENDMENT_1.md (primary). *Spec* `specs/r24.json` (`ECS1h451f5f1382a1`), commit `d1be0c1c8` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:40:54Z, finished 2026-10-20T17:41:31Z.

*Curve* `icv1-fp24-tm5099-a6f137c6` (registered; `prime_explicit`). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/r24.audit.json`).
*Isolation* (measured runs) double-base: L1 1, L2 23; full-rank: L1 3, L2 21; half-base: L1 5, L2 19; plan: L1 4, L2 20; plan-aa: L1 2, L2 22; random-targets: L1 10, L2 14; rho-neg: L1 4, L2 20; rootfind: L1 7, L2 17.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [20.686], [21.577], [29.597],
  [full-rank], [49.985], [59.103], [141.162],
  [half-base], [21.428], [25.41], [61.245],
  [plan], [13.727], [15.583], [32.288],
  [plan-aa], [13.727], [15.583], [32.288],
  [random-targets], [615.639], [617.494], [634.186],
  [rho-neg], [0.954], [1.04], [1.818],
  [rootfind], [2341.216], [2576.224], [4691.297],
)
Paired ratios against `plan`: `sessions/r24/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/r24/records.jsonl`. Observations only; the interpretation is in the study README.

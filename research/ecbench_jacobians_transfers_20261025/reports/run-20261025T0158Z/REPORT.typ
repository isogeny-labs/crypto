#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)

= ecbench run report: Jacobian workloads and transfers

*Run:* 2026-10-25, 01:58–02:01 UTC. Four sessions, commit `6111849ed` (clean),
binary `e0dc91fae666…`, host class `ECBENV3h8be61a9c1732` (cloud container).
Runs earned isolation L1/L2. L3 is blocked: no isolcpus, no performance
governor, boost not known off. Study:
`research/ecbench_jacobians_transfers_20261025/` (README, specs, sessions).
This report records observations only; interpretation is in the study README.

== What ran

#table(
  columns: (auto, auto, auto, auto, auto),
  [*family*], [*session*], [*arms*], [*workloads*], [*records (verified; exact replays)*],
  [genus-2 Jacobians], [`ECBS1h1a7b060b52e9`], [rho.negation (ref.), rho.plain, bsgs.negation, ic.hyperelliptic, A/A], [2 curves × 4 targets], [160 (160; 120/120)],
  [plane-quartic Jacobians], [`ECBS1hcb9aa3261765`], [rho.plain (ref.), bsgs.textbook, A/A], [2 × 4], [96 (96; 72/72)],
  [GHS curves over $FF_(p^6)$], [`ECBS1h31140d2225ae`], [rho.negation (ref.), transfer.ghs_cover, A/A], [2 × 4], [96 (96; 72/72)],
  [prime curve, 16 bits], [`ECBS1hd12205a7f257`], [rho.negation (ref.), rho.plain, transfer.isogeny (ℓ=3), transfer.quartic_cover, A/A], [1 × 4], [80 (80; 60/60)],
)

Each session ran 3 measured rounds and 1 warm-up, measurement seed 20261025,
target seed 20261025. No run failed or timed out. The warm-up records are
verified but not replayed.

== Measured values

The unit is GAE: one Jacobian operation on a Jacobian workload, one addition
on $E$ on `ghs_fp6` and prime curves, transfers included. $S = "GAE" / sqrt(r)$.
Intervals are those `ecbench table` and `ecbench compare` recorded.

#figure(image("figures/s_by_arm.svg", width: 100%),
  caption: [Mean $S$ per arm and curve, 12 verified runs per bar, 95% interval as recorded by `ecbench table`. Sessions as titled. A/A arms omitted (identical operation counts).])

#figure(image("figures/ratio_to_reference.svg", width: 100%),
  caption: [Paired operation ratio of each candidate to its session's reference arm at native weight 0 (`ecbench compare`, linearised-t 95% interval over workloads). Blue: interval below 1; red: above 1; grey: covers 1. The prime-curve rows are INDICATIVE (4 workloads).])

#table(
  columns: (auto, auto, auto, auto),
  [*pair (session)*], [*ratio [95%]*], [*§5a band w = 0 / 0.1 / 1*], [*band verdict (ecbench)*],
  [genus 2: rho.plain / rho.negation], [1.106 [0.875, 1.399]], [1.106 / 1.023 / 0.610], [unresolved],
  [genus 2: bsgs.negation / rho.negation], [0.737 [0.609, 0.892]], [0.737 / 0.749 / 0.805], [below at every weight],
  [genus 2: ic.hyperelliptic / rho.negation], [1.473 [1.158, 1.874]], [1.47 / 37.5 / 216.4], [above at every weight],
  [quartic: bsgs.textbook / rho.plain], [0.497 [0.378, 0.654]], [0.497 / 0.544 / 0.959], [below at every weight],
  [GHS: transfer.ghs_cover / rho.negation], [6.623 [5.134, 8.543]], [6.62 / 6.17 / 3.98], [above at every weight],
  [prime: rho.plain / rho.negation], [0.996 [0.625, 1.587]], [0.996 / 0.930 / 0.585], [below at every weight],
  [prime: transfer.isogeny / rho.negation], [0.934 [0.744, 1.172]], [0.93 / 1.54 / 4.72], [unresolved],
  [prime: transfer.quartic_cover / rho.negation], [15,292 [11,065, 21,134]], [15,292 / 14,289 / 8,986], [above at every weight],
)

*Recorded conversion constants* (per run, `detail`). For `transfer.ghs_cover`,
$c_E$ = 331 $FF_p$ multiplications per addition on $E(FF_(p^6))$ and
6.26–6.37 $E$-additions per genus-3 Cantor operation. For
`transfer.quartic_cover`, $c_E$ = 30 and 13,775–13,776 $E$-additions per
plane-quartic Jacobian operation.

*Controls.* Each session's A/A arm repeats the reference with the same
seeds, and its operation ratio is 1.0000 [1.000, 1.000] in every session.
Wall-time A/A ratios: genus 2, 1.000 [0.925, 1.082]; quartic, 0.974
[0.914, 1.039]; GHS, 1.027 [0.864, 1.221]; prime, 0.846 [0.706, 1.013].
Wall-time comparisons are descriptive only (L1/L2).

*Missing data.* None. Every planned execution produced a verified record.

== Raw records

`sessions/<family>/records.jsonl`, `sessions/<family>.verify.txt`, `sessions/<family>/comparisons/*.json`, `out/table-<family>.txt`, `out/band-<family>.{txt,json}`, `out/compare.txt`. Graph source: `plot.py` (it reads the table and comparison files only).

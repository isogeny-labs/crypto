#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Tower F4, signature F5 and an F6 analogue on prime-field curves] \
  #text(size: 10pt)[Run report, `ecbench`, 2026-10-11 22:44 UTC onward (run `run-20261011T2244Z`)]
]

= What ran

- *Protocol.* `research/pkm_tower_ecbench_20261011/PROTOCOL.md`, committed
  before any measured run, and `AMENDMENT_1.md`.
- *First attempt.* It started at 22:44 UTC from commit `280c5ad52`. Its P6
  replay audit failed because the tower F4 count depended on the thread
  count. It is kept in `attempt-1-voided/`, and none of its numbers enters
  this report.
- *Measured attempt.* It started at 22:51 UTC from commit `faeeae26a`.
  Every session records that commit, a clean tree, and `ecbench` binary
  sha256 `64ed18283b4e8215…`.
- *Host.* An x86-64 Linux cloud VM. The isolation earned per run, L1 or L2,
  is recorded per arm in `ANALYSIS.json`.
- *Arms.* All IC arms use `ic.pipeline` with `prime-tower:t=T,seed=1` and
  `pkm-tower:m=2,t=T,seed=1,engine=E,degree=20`, `max_trials=200000`:
  - `f4`: tower F4 decides each node of the splitting tree.
  - `sig-f5`: signature F4 with the F5/GVW criteria decides each node.
  - `f6`: tower F4, with nodes of at most 4 candidates closed by charged
    group additions.
  - `enum`: closure at the root; no algebra.

  Reference: `rho.negation` on the same target.
- *Seeds and workloads.* Target seed 202610112301, measurement seed
  202610112302, one round. Public `hash_to_subgroup_v1` targets: 8 per
  curve, 4 at P8.

#table(
  columns: 8,
  align: (left, left, left, right, right, left, right, left),
  table.header([spec], [session], [curve], [p], [r], [t], [verified], [replay]),
  [P5], [`ECBS1h744c8f794e9a`], [`icv1-fp9-t25-33fa1ca2`], [257], [233], [5], [40/40], [ok],
  [P6], [`ECBS1h91a14b12224a`], [`icv1-fp11-t43-4d73e7f7`], [1,409], [1,367], [6], [40/40], [ok],
  [P7], [`ECBS1h4e2538d423f5`], [`icv1-fp13-tm41-756f569a`], [7,681], [7,723], [7], [40/40], [ok],
  [P8], [`ECBS1ha6de54733ae8`], [`icv1-fp15-t181-50ff72d2`], [18,433], [18,253], [8], [20/20], [ok],
)

Raw records: `sessions/<spec>/records.jsonl`. Replay receipts:
`audits/<spec>-replay-all.json`. Derived values: `ANALYSIS.json`, written by
`examples/pkm_tower_prime_analyze.rs` from the records alone.

= Measured values

*Units.*
- `W`: counted `F_p` multiply-adds per run (measured, deterministic).
- `S`: charged GAE over `√r` (measured). It is a lower bound for the tower
  arms, whose multiply-adds are unpriced.
- Medians are derived by the analyser.

#table(
  columns: 9,
  align: (left,) + (right,) * 8,
  table.header([cell], [rho S], [F4 S], [sig-F5 S], [F6 S], [`enum` S], [F4 W], [sig-F5 W], [F6 W]),
  [P5], [5.50], [25.84], [25.84], [25.55], [29.74], [8.05e7], [1.82e7], [8.02e7],
  [P6], [2.79], [15.55], [15.55], [15.67], [38.81], [2.35e9], [6.51e8], [2.35e9],
  [P7], [2.01], [9.37], [9.37], [9.38], [45.89], [4.07e10], [1.25e10], [4.07e10],
  [P8], [1.98], [7.48], [7.48], [7.50], [60.62], [4.86e11], [1.59e11], [4.86e11],
)

#figure(image("muladds_per_run.svg", width: 95%),
  caption: [Counted `F_p` multiply-adds per run for the three tower engines.
  Bars show the median and ticks the range over 8 targets (4 at P8), for
  sessions P5–P8.])

#figure(image("s_per_arm.svg", width: 100%),
  caption: [Lower-bound `S` per arm and cell, with the median and range. The
  red dotted line is the floor √(π/4). The tower arms' `S` omits their
  multiply-adds.])

= The protocol's questions (derived; rules applied as written)

- *Q1. Signature F5 / F4 multiply-adds.*
  - Medians: 0.227, 0.277, 0.307, 0.326 (P5 → P8).
  - The tree is the same on every target.
  - Rule: *F5 cuts F4's work* in every cell.
  - Relation wall, signature F5 / F4: 0.56, 0.94, 1.45, 2.06.
- *Q2. log₂(W F4 / W F6).*
  - Medians: 0.0059, 0.0005, 0.00007, 0.00001.
  - Rule: *no saving* in every cell.
  - Closure removes 27–47 % of the nodes, but almost none of the work.
- *Q3. S / same-target rho.* Every arm is above 1 on every target. Rule:
  *slower than rho on every target*.
  - F4: 4.64, 5.39, 4.77, 3.91.
  - `enum`: 5.42, 12.0, 21.7, 29.0.
- *Q4. Algebra / `enum`.*
  - Lower-bound `S(F4)/S(enum)`: 0.85 → 0.12.
  - Derived, with multiply-adds priced at 1/63 GAE:
    - F4: 2,949 → 899,485;
    - signature F5: 670 → 293,124.
  - Relation wall, F4 / `enum`: 8,041 → 360,005.
- *Q5. Slopes of log₂ S against log₂ r* (descriptive, four cells): rho
  −0.24, F4 −0.29, F6 −0.28, `enum` +0.15.

#figure(image("engine_ratios.svg", width: 95%),
  caption: [Per-target engine ratios in counted multiply-adds. Left: signature
  F5 / F4 on identical trees. Right: log₂(F4 / F6), with the preregistered
  0.1 threshold and the "no saving" band.])

= Limits

- *Wall time* is a practicality note from mixed L1/L2 runs.
- *Unpriced work.* The tower arms' multiply-adds are counted but not
  priced, and lookups and square roots are unpriced for every IC arm.
- *Scope.* `m = 2`, Kummer towers, one splitting rule; four sizes up to
  15 bits.
- *No claims.* No exponent, speedup or cryptographic-size claim is made.
  Interpretation is in the study's `RESULT.md`.

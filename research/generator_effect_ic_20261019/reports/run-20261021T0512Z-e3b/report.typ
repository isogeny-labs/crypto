#set page(margin: 1.8cm)
#set text(size: 8.5pt)
= Run report: E3b sessions, 2026-10-21T05:12Z
<run-report-e3b-sessions-2026-10-21t0512z>
Observations only. The interpretation is in
#link("../../E3b.md")[`../../E3b.md`].

#figure(
  align(center)[#table(
    columns: (50%, 50%),
    align: (left,left,),
    table.header([item], [value],),
    table.hline(),
    [protocol], [#link("../../E3b-PROTOCOL.md")[`E3b-PROTOCOL.md`],
    frozen in commit `f9cf6ab116` before these sessions; deviations D1,
    D2 at its end],
    [specs], [#link("../../e3b/specs/")[`e3b/specs/`] (`e3-c4`, `e3-c5`,
    `e3-c6`, `f4-c1-sem-{chain,full}-t{2,3}`)],
    [command], [`ecbench run --allow-busy --cpus none --lock <lock> --spec e3b/specs/<spec>.json --out e3b/sessions/<name> --quiet`,
    seven sessions started together at 2026-10-21T05:12:34Z; build of
    commit `ede431274` (the E3 binary, `ecbench` sha256
    `b72458fbfa802e88…`)],
    [host], [macOS (Apple silicon), shared and heavily loaded (load
    average above 100 when checked), no CPU topology, so `--cpus none`.
    Every record is at isolation level L0. Wall time is not used for any
    cost],
    [workloads], [8 `planted_shared` targets per ladder curve; F4
    sessions: E3's e3-c1 targets 0 and 1 (chain `t = 3`: target 0 only);
    target seed 20261019, algorithm seed 20261019, 1 round, no warm-up],
    [sessions, runs], [`e3-c4` ECBS1h33f9cde203a7: 104 of 104 runs, all
    `verified`, 1.4 process-hours summed over runs; `e3-c5`
    ECBS1h12ea835a948e: 104 of 104 runs, all `verified`, 1.4
    process-hours summed over runs; `e3-c6` ECBS1h478cd952402d: 104 of
    104 runs, all `verified`, 1.5 process-hours summed over runs;
    `f4-c1-sem-chain-t2` ECBS1h305ebf1e98d2: 4 of 4 runs, all
    `verified`, 5.2 process-hours summed over runs; `f4-c1-sem-full-t2`
    ECBS1hbe41d11e8904: 4 of 4 runs, all `verified`, 4.7 process-hours
    summed over runs],
    [not completed], [the two `t = 3` F4 sessions: started with the
    seven, interrupted with the host session (D1), rerun, interrupted
    again (D2); each wrote only `rho-neg` records and none is in
    `e3b/sessions/`. No number of theirs is used],
    [raw records], [`e3b/sessions/<name>/records.jsonl`,
    `session.json`],
  )]
  , kind: table
  )

== Measured values, ladder (S = GAE/√r, mean of 8 targets, 90% bootstrap interval in the aggregate)
<measured-values-ladder-s-gaer-mean-of-8-targets-90-bootstrap-interval-in-the-aggregate>
`S(w) = (G − C + w·(U + n_C))/√r`, `C = n_C = 0`. Per curve `rho-neg` is
measured once. Aggregated by `examples/generator_effect_e3_summary.rs`
into #link("../../e3b/summary.json")[`e3b/summary.json`] (derived).

#figure(
  align(center)[#table(
    columns: (10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%),
    align: (left,right,right,right,right,right,right,right,right,left,),
    table.header([curve], [t], [B], [grid S, w = 0], [grid S, w =
      0.1], [grid S, w = 1], [known-log S, w = 0], [known-log S, w =
      0.1], [known-log S, w = 1], [rho-neg S, w = 0 / 0.1 / 1],),
    table.hline(),
    [e3-c4], [4], [6], [80.4], [5.86e+04], [5.85e+05], [107], [117], [213], [0.831
    \/ 0.903 / 1.55],
    [e3-c4], [5], [13], [12.2], [5.88e+04], [5.88e+05], [14.9], [16.3], [29.2], [0.831
    \/ 0.903 / 1.55],
    [e3-c4], [6], [29], [3.41], [1.08e+05], [1.08e+06], [8.2], [8.93], [15.5], [0.831
    \/ 0.903 / 1.55],
    [e3-c4], [7], [68], [1.15], [1.51e+05], [1.51e+06], [3.67], [3.79], [4.87], [0.831
    \/ 0.903 / 1.55],
    [e3-c4], [8], [128], [1.02], [3.49e+05], [3.49e+06], [7.01], [7.04], [7.29], [0.831
    \/ 0.903 / 1.55],
    [e3-c4], [9], [263], [1.41], [1.48e+06], [1.48e+07], [24.5], [24.5], [24.5], [0.831
    \/ 0.903 / 1.55],
    [e3-c5], [4], [9], [40.4], [2.95e+04], [2.94e+05], [112], [123], [223], [1.01
    \/ 1.1 / 1.92],
    [e3-c5], [5], [19], [8.02], [3.78e+04], [3.78e+05], [28.1], [30.8], [55.6], [1.01
    \/ 1.1 / 1.92],
    [e3-c5], [6], [35], [2.55], [7.93e+04], [7.93e+05], [9.31], [10.2], [17.7], [1.01
    \/ 1.1 / 1.92],
    [e3-c5], [7], [69], [1.1], [1.7e+05], [1.7e+06], [3.26], [3.38], [4.4], [1.01
    \/ 1.1 / 1.92],
    [e3-c5], [8], [134], [0.895], [3.67e+05], [3.67e+06], [6.43], [6.47], [6.78], [1.01
    \/ 1.1 / 1.92],
    [e3-c5], [9], [276], [1.25], [1.35e+06], [1.35e+07], [22.4], [22.4], [22.5], [1.01
    \/ 1.1 / 1.92],
    [e3-c6], [4], [10], [32.5], [2.37e+04], [2.37e+05], [79.3], [87.2], [158], [1.14
    \/ 1.24 / 2.18],
    [e3-c6], [5], [18], [9.8], [4.7e+04], [4.7e+05], [18.3], [20.1], [36.1], [1.14
    \/ 1.24 / 2.18],
    [e3-c6], [6], [30], [3.56], [1.18e+05], [1.18e+06], [5.56], [6.04], [10.3], [1.14
    \/ 1.24 / 2.18],
    [e3-c6], [7], [61], [1.22], [2.1e+05], [2.1e+06], [3.65], [3.84], [5.51], [1.14
    \/ 1.24 / 2.18],
    [e3-c6], [8], [129], [0.893], [4e+05], [4e+06], [6.19], [6.24], [6.68], [1.14
    \/ 1.24 / 2.18],
    [e3-c6], [9], [253], [1.17], [1.4e+06], [1.4e+07], [19], [19], [19.1], [1.14
    \/ 1.24 / 2.18],
  )]
  , kind: table
  )

== Measured values, F4 full solves (`t = 2`, e3-c1)
<measured-values-f4-full-solves-t-2-e3-c1>
From #link("../../e3b/f4check.json")[`e3b/f4check.json`] (derived:
measured record against E3's grid record of the same target and E3's
capped means).

#figure(
  align(center)[#table(
    columns: (12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%),
    align: (left,left,right,right,right,right,right,right,),
    table.header([arm], [session], [target], [trials], [F4 calls], [F4
      mult-adds per call], [measured / substituted U], [wall per call
      (ms)],),
    table.hline(),
    [sem-chain-t2], [ECBS1h305ebf1e98d2], [0], [444341], [434258], [3.9079e+05], [1.000001], [30.6],
    [sem-chain-t2], [ECBS1h305ebf1e98d2], [1], [841257], [805655], [3.9079e+05], [1.000000], [6.5],
    [sem-full-t2], [ECBS1hbe41d11e8904], [0], [444341], [434258], [2.2429e+05], [0.999997], [28.2],
    [sem-full-t2], [ECBS1hbe41d11e8904], [1], [841257], [805655], [2.2429e+05], [0.999999], [5.9],
  )]
  , kind: table
  )

Missing data: no `t = 3` F4 record, no F4 record at `t ≥ 6` (not run),
no wall-time comparison across arms (L0 host, not admitted).

#page(flipped: true, margin: 1.2cm)[
#figure(image("e3b_ladder.svg", width: 100%, alt: "E3b ladder"),
  caption: [
    E3b ladder
  ]
)

#emph[Figure 1. S against B at w = 0, 0.1 and 1 (top) and grid /
known-log paired by target (bottom), from `e3b/summary.json`. Solid: E3b
curves e3-c4/c5/c6 (markers o/s/^); faint: E3's curves, with F4 chain
substituted, not run. Sessions ECBS1h33f9cde203a7, ECBS1h12ea835a948e,
ECBS1h478cd952402d. 8 targets per point, 90% bootstrap intervals.]
]

#set page(margin: 1.8cm)
#set text(size: 9.5pt)
= Run report: E3 sessions, 2026-10-19T23:34Z to 2026-10-20T05:35Z
<run-report-e3-sessions-2026-10-19t2334z-to-2026-10-20t0535z>
Observations only. The interpretation is in
#link("../../E3.md")[`../../E3.md`].

#figure(
  align(center)[#table(
    columns: (50%, 50%),
    align: (left,left,),
    table.header([item], [value],),
    table.hline(),
    [protocol], [#link("../../E3-PROTOCOL.md")[`E3-PROTOCOL.md`], frozen
    in commit `ede431274` before these sessions],
    [specs], [#link("../../e3/specs/")[`e3/specs/`] (`e3-ctl`, `e3-c1`,
    `e3-c2`, `e3-c3`)],
    [command], [`ecbench run --allow-busy --lock $TMPDIR/crypto-bench.lock --spec e3/specs/<c>.json --out e3/sessions/<c>`,
    one curve after another; build at `ede431274`],
    [host], [macOS (Apple silicon), shared, no CPU topology, so
    `--cpus none`. Every record is at isolation level L0. Other jobs ran
    concurrently. Wall time is not used],
    [sessions], [`e3-ctl` ECBS1h28077f2dbe1c (23:34:15--23:34:19Z),
    `e3-c1` ECBS1h642a1a44f9f9 (23:34:19Z--01:34:48Z), `e3-c2`
    ECBS1hd4736f2baf84 (01:34:48--03:32:32Z), `e3-c3` ECBS1h2a56f971e518
    (03:32:32--05:34:44Z)],
    [arms], [per curve: `rho-neg`\; per t: `sem-chain-t{t}`,
    `sem-full-t{t}` (t ≤ 4), `sem-grid-t{t}`, `kl-mitm3-t{t}`\; control:
    t = 2 only, `sem-chain` uncapped],
    [workloads], [8 `planted_shared` targets per curve, target seed
    20261019, algorithm seed 20261019, 1 round, no warm-up],
    [requested / completed], [416 / 416 runs. Every full-solve run
    verified: 248, comprising 72 per 24-bit curve and 32 on the control.
    Every capped F4 run `exhausted` at its `max_trials` as specified:
    168, 56 per 24-bit curve. No failures, timeouts or errors],
    [raw records], [`e3/sessions/<c>/records.jsonl` and `session.json`],
  )]
  , kind: table
  )

== Measured values (operation counts; S = GAE/√r at w = 0, mean of 8 runs)
<measured-values-operation-counts-s-gaer-at-w-0-mean-of-8-runs>
#figure(
  align(center)[#table(
    columns: (25%, 25%, 25%, 25%),
    align: (left,right,left,left,),
    table.header([curve], [rho-neg], [grid t2 / t3 / t4 /
      t5], [known-log MITM t2 / t3 / t4 / t5],),
    table.hline(),
    [e3-c1], [1.079], [389.3 / 128.9 / 31.68 / 6.37], [1111 / 246.2 /
    48.69 / 9.30],
    [e3-c2], [1.329], [316.9 / 134.0 / 26.93 / 7.30], [469.4 / 243.0 /
    56.79 / 16.74],
    [e3-c3], [1.174], [1845 / 277.0 / 33.81 / 8.24], [2553 / 572.3 /
    53.55 / 27.36],
    [control (12-bit)], [2.728], [18.38 (t2)], [14.34 (t2);
    `sem-chain-t2` uncapped 18.38],
  )]
  , kind: table
  )

- #strong[F4 multiply-adds per call] (capped records, all curves): chain
  3.908·10⁵, 1.587·10⁸, 1.060·10¹⁰ and 3.186·10¹² at t = 2…5; full
  2.243·10⁵, 1.260·10⁷ and 1.683·10¹⁰ at t = 2…4.
- #strong[Grid multiplications per call:] 244, 1,224, 7,440 and 51,232.
- #strong[Cross-check:] `pkm_crosscheck_disagreements` = 0 over 24,502
  capped F4 calls, all of them refutations.
- #strong[Pricing:] the weights 0.1 and 1, the intervals and the derived
  ratios are in #link("../../e3/summary.json")[`e3/summary.json`],
  produced by `examples/generator_effect_e3_summary.rs`.

#page(flipped: true, margin: 1.2cm)[
#figure(image("e3.svg", width: 100%, alt: "S against B at the three band weights"),
  caption: [
    S against B at the three band weights
  ]
)

#emph[S against B at w = 0, 0.1 and 1, sessions as above. F4 arms dashed
and substituted (see E3.md). 8 targets per point, 90% bootstrap
intervals.]
]

#set document(title: "Sparse relation matrices in ic.pipeline: first paired measurement")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8pt)

#align(center)[
  #text(15pt, weight: "bold")[Sparse relation matrices in `ic.pipeline`: first paired measurement]
  #v(2pt)
  research/sparse_la_timing_20261018 · 2026-10-18/19 · protocol frozen at `6414d1575` \
  Sessions `ECBS1h0bf2b60b4cc4` (S2, LA scaling, 330 runs) and FP4_SESSION (S1, E(F#sub[29#super[4]]) ladder, 20 runs)
]

#block(fill: luma(240), inset: 8pt, radius: 3pt)[
  *Verdict: null/negative; no improvement.* No deferred sparse matrix (`sparse-cado` = `filter+wiedemann`,
  `filter+lanczos`, `wiedemann`, `lanczos`) lowers end-to-end $S$ against the baseline row or the incumbent
  `structured-gauss` on any workload. They answer only at full column rank, while the incumbent stops when the
  target's column is pinned: 1.75× the relations on S1 workload 2 (end-to-end 1.398×), 5.5–10.5× on S2
  (end-to-end 5.72× at 128 columns, 1.030× at 4096). In counted `row_ops` the filtered solvers cost 1.47–1.97× the
  dense full-rank matrix on the same rows (a lower bound); their L0 wall time falls to 0.52× of it at 4096 columns
  (descriptive only). Every answer verified; S2 replay-all OK; AUDIT_S1_SHORT.
]

= Baseline and design

The baseline (AGENTS.md "Success criterion") is `ic-d4-chains` of `research/gaudry_quartic_d4_cost_20261014`:
`fghr-y-line:zero=1` + `gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1`, `linalg=incremental-gauss`,
end-to-end $S$ = 1,653,062.36 and 1,779,983.12 on E(F#sub[29#super[4]]) (`fp4_torsion p=29 seed 1`, r = 44,263).
Genus 3 after cubic GHS is not reachable from `ic.pipeline` and is not measured. Arms differ only in `linalg`; every
arm shares the algorithm seed per workload, so the relation stream is common. Controls: A/A (`sg-aa`; and the alias
`filter-wiedemann` = `sparse-cado`), positive control `incremental-gauss-full-rank`, matched `rho.negation`.
Gate G0: the `incremental-gauss` arm reproduced the baseline records bit for bit (S, GAE, counters, logs, seeds).

= S1: the E(F#sub[29#super[4]]) ladder (end to end)

#table(columns: 6, align: (left, right, right, center, center, right),
  [arm], [$S$ W1], [$S$ W2], [relations], [LA `row_ops`], [÷ matched rho],
  [`rho-neg`], [1.0647], [1.4117], [—], [—], [1],
  [`ig` (baseline), `sg`], [1,653,062.36], [1,779,983.12], [9 / 8], [189/158, 170/122], [1.386·10#super[6]],
  [`igfr` (pos. ctrl)], [1,653,062.37], [2,487,930.02], [9 / 14], [189 / 208], [1.672·10#super[6]],
  [filtered sparse (3 arms)], [1,653,062.37], [2,487,930.06], [9 / 14], [277 / 1,032], [1.672·10#super[6]],
  [`wiedemann`], [1,653,062.67], [2,487,932.26], [9 / 14], [6,441 / 45,940], [1.672·10#super[6]],
  [`lanczos`], [1,653,062.44], [2,487,930.17], [9 / 14], [1,739 / 3,103], [1.672·10#super[6]],
)
Full-rank arms ÷ baseline, totals over both pairs: 1.2062 (W1 1.0000, W2 1.3977). Linear algebra is ≤ 2·10#super[−5] of
$S$; the D#sub[4] oracle is the rest. Band (§5a): `ecbench band` refuses the session (pinned-unit pricing, no
calibration row for this curve); pricing every `row_op` at 1 GAE adds ≤ 218 S, rho's canonicalisations ≤ 0.67 S, so
the order holds at 0, 0.1 and 1 GAE.

#figure(image("figures/fp4.svg", width: 92%), caption: [S1, FP4_SESSION: end-to-end $S$ and LA `row_ops` per arm (2 targets).])

= S2: scaling with matrix size (30-bit prime curve, 6 targets)

#table(columns: 7, align: (right, right, right, center, center, center, center),
  [columns], [`sg` $S$], [deferred $S$], [deferred ÷ `sg` [95 %]], [relations sg → full rank], [LA row_ops filtered ÷ `igfr`], [LA wall `igfr` → filtered (ms, L0)],
  [128], [54.50], [311.94], [5.724 [4.336, 7.913]], [62 → 341.5], [1.97], [2.2 → 4.4],
  [256], [36.24], [179.36], [4.950 [4.027, 6.263]], [137.5 → 743.5], [1.58], [4.7 → 7.3],
  [512], [24.50], [98.12], [4.004 [3.384, 4.868]], [291 → 1,440], [1.47], [10.0 → 14.1],
  [1,024], [41.04], [89.77], [2.187 [2.024, 2.295]], [524.5 → 3,904], [1.54], [37.5 → 33.4],
  [2,048], [137.94], [166.63], [1.208 [1.192, 1.224]], [901 → 8,755], [1.52], [159.0 → 103.9],
  [4,096], [540.98], [557.10], [1.030 [1.026, 1.033]], [1,794.5 → 18,885.5], [1.53], [684.5 → 356.9],
)
Matched `rho-neg`: $S$ = 0.891 (0.978 / 1.769 at 0.1 / 1 GAE). `ecbench band --ranks` puts `sg` and `ig` below every
deferred arm at every size and weight. Descriptive log–log slopes of LA `row_ops` vs columns: `sg` 0.92, `igfr` 1.13,
filtered sparse 1.08–1.09, unfiltered 2.17–2.21.

#figure(image("figures/scaling.svg", width: 100%), caption: [S2, session `ECBS1h0bf2b60b4cc4`: LA `row_ops` (left) and end-to-end $S$ (right) against factor-base size; slopes are a descriptive fit.])
#figure(image("figures/band.svg", width: 100%), caption: [S2 under the §5a band: native work priced at 0, 0.1 and 1 GAE per operation; black line = matched rho.])

= Classification and next step

End to end: negative (regression), caused by the full-rank completion rule, not by the Krylov arithmetic; not an
improvement; leaderboard tables A–C unchanged. Stage: no improvement in counted units; the wall-time lead at ≥ 1,024
columns is L0 and not ranked. Distance to rho unchanged (baseline 1.386·10#super[6]× matched rho).

*Next improvement:* a deferred sparse solver that stops at the target pin (test whether the target's unit vector lies
in the row space with one Krylov solve on the filtered transpose), so it uses `structured-gauss`'s relation count;
count the block recurrence arithmetic first; then measure on genus 3 after cubic GHS and the stacked D#sub[4] + 2LP
F(p#super[4]) arm, where linear algebra carries weight.

#text(size: 7.5pt)[Host: Intel Xeon @ 2.10 GHz, 4 vCPU shared VM, no PMU; runs earned L1–L2 on the run CPU; wall time is
secondary. Card 1 was killed by the executor's 30-minute background limit after completing S2; its S1 attempt is kept
as `sessions/fp4-p29` (interrupted); S1 re-run by cards 2–3. Evidence: README.md, PROTOCOL.md, sessions/, analysis/.]

#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: a second invariant guess on the pair at ℓ = 8

- *Study:* `research/dreg_invariant_pair_guess_20261029/`, pre-registered
  before any D3 run. No amendments.
- *Run:* 2026-10-29 22:07 UTC to 2026-10-30 01:29 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Arm D3:* `x₁₅ := x₇ + c`, `x₁₄ := x₆ + f`, `x₂₄ = g`, with 8 branches,
  on the 12 committed unsatisfiable (22, 8) draws.
  - It runs the same `dreg_linear_guess` build as #2851 and #2883, at
    `--to 4 --max-cols 140000 --check`.
  - Early stop at the first branch that saturates degree 4 without refuting.
- *Comparator S3:* the committed records of the two earlier studies.
- *Identity control N:* S3 on draw 5 branch 0, re-run.
- *Limits:* `prlimit --as=15e9`, `timeout 7200`, one process at a time, on
  the 4-core, 16 GB container, not isolated.

== Completion

- 39 processes, all exited 0. None was killed or retried.
- Peak RSS was 7.0–8.5 GB.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches per arm and draw: refuting at degree 4, saturating degree
  4 without refutation, or not run. Source: `plot.py` (layout only) on
  `plot-data.json` from `dreg_linear_guess_pair_score --plot-data`.])

- *Controls:* N matches the committed record, every record passes the
  identity and exactness checks, and every draw is decided. Valid.
- *D3* suffices on 2 of 12 draws, and *S3* on 7 of 12.
- *Discordant draws:* D3-only 0, S3-only 5.
- *Registered verdict:* no detected difference (two-sided exact sign test,
  p = 0.0625).
- D3 is never better on any draw.

#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: eight more draws, invariant against raw guesses at ℓ = 8

- *Study:* `research/dreg_invariant_guess_more_20261029/`, pre-registered
  before any new draw was generated. No amendments.
- *Run:* 2026-10-29, 14:32 to 21:45 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Draws:* `dreg_ladder`, seed 20261022, cell (22, 8), continuing the
  committed sequence from index 7. Its replay of draws 0–6 equals the
  committed draws.
- *Registered set:* the first 8 unsatisfiable draws: 8, 11, 14, 15, 17, 24,
  25 and 27.
- *Arms:* the same `dreg_linear_guess` build as #2851, at `--to 4 --max-cols
  140000 --check`.
  - S3: `x₁₅ := x₇ + c`, `x₂₃`, `x₂₄`.
  - X3: `x₇`, `x₂₃`, `x₂₄`.
  - 8 branches each, stopping at the first branch that saturates degree 4
    without refuting.
- *Positive control:* `{7, 15, 23, 24} = 0` on every draw.
- *Limits:* `prlimit --as=15e9`, `timeout 7200`, one process at a time, on
  the 4-core, 16 GB container, not isolated.

== Completion

- 81 processes exited 0, and none exited non-zero.
- Two runs were killed in flight (the session's background limit, then a
  container restart) and repeated. Their empty first attempts are in
  `runs/attempts/`.
- Peak RSS was 7.0–8.6 GB.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches per arm and draw: refuting at degree 4, saturating degree
  4 without refutation, or not run. Top: the registered draws. Bottom: the
  four draws of #2851. Source: `plot.py` (layout only) on `plot-data.json`
  from `dreg_linear_guess_more_score --plot-data`.])

- *Controls:* the replay check passes, every run passes the identity and
  exactness checks, and the positive control refutes on 8 of 8. Valid.
- *S3* suffices on 5 of 8 draws, and *X3* on 0 of 8.
- *Registered verdict:* S3_better. All 5 discordant draws favour S3; the
  one-sided exact sign test gives p = 0.031.
- *Secondary:* with #2851's draws, 6 of 6 discordant draws favour S3,
  p = 0.016.

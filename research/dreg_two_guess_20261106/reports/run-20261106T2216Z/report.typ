#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: two guesses at (22, 8)

- *Study:* `research/dreg_two_guess_20261106/`, pre-registered before any run.
  No amendments.
- *Run:* 2026-11-06 22:16 UTC to 2026-11-07 01:17 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.
- *Standing:* unverified assertion until a pcc verdict exists.

== What ran

- *K2L:* `x₁₅ := x₇ + c`, `(e₁ + x₃)₂₀ = h`. *K2S:* `x₁₅ := x₇ + c`,
  `x₂₃ = h`. Four branches each, 12 unsatisfiable draws, degree 4,
  `--max-cols 150000 --check`, early stop at the first failing branch.
- *Identity control N:* the committed `S3.d5.b0`, rerun on the new build.
- 31 processes, all exited 0. 44 occurring variables (149,986 degree-4
  columns); 244–484 s per degree-4 scan.

== Measured values

#figure(image("branches.svg", width: 100%),
  caption: [Branches per arm and draw as a share of the arm's branches; the
  three-guess L3 and S3 are the committed records (hatched). Source: `plot.py`
  (layout only) on `plot-data.json` from `dreg_two_guess_score --plot-data`.])

- *Controls:* valid.
- *K2L* and *K2S* each suffice on 1 of 12 draws (draw 6), against 6 (L3) and
  7 (S3) with three guesses.
- *Primary verdict:* no detected difference (no discordant draw).

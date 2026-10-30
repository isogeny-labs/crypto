#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: the unspecialised (22, 7) matched null

- *Study:* `research/dreg_closure_null22_7_20261028/`, pre-registered in
  `PREREGISTRATION.md` (commit `e3091a668`, no amendments).
- *Run:* 2026-10-28, 03:35 to 04:15 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Script:* `run.sh` runs `dreg_closure --to 4 --max-cols 140000 --draw K …`, one process at a time, under `prlimit --as=15e9 timeout 7200`.
- *Binaries:* `dreg_closure` SHA-256 `d7246453…2ec4`, `dreg_score` `4e6150b0…ecc6a`, recorded in `runs/binary.sha256` before the first run and re-verified at scoring.
- *Draws:* m = 3, (n, ℓ) = (22, 7), N = 43, 44 equations, surplus +1, seed 20261028. Unsatisfiable draws 1, 2, 3, 4; satisfiable draws 0 and 8.
- *Arms:*
  - N0: the unspecialised system on the four unsatisfiable draws, 136,698 degree-4 columns.
  - B: variables 6, 13, 20 fixed to 0 (summand dimensions 6, 6, 6), the positive control.
  - N7: the unspecialised (19, 7) draw 0 of the closure-at-scale study, the instrument-identity control.
  - S: the two satisfiable draws, which the closure must never refute.
- *Host:* the 4-core, 16 GB container, not isolated.

== Completion

- 11 of 11 processes exited 0; none was killed, empty or rerun.
- Peak RSS was 7.05 GB.
- Draw generation ran as four parallel shards and was stopped once draw 8, the second satisfiable draw, was certified; draws 9 to 11 were not used.

== Measured values

#figure(image("null.svg", width: 100%),
  caption: [Draws on which the degree-4 closure refutes, by cell, read from the
  committed records. Source: `plot.py` (layout only).])

- *N0:* does not refute on 4 of 4 draws; degree 4 saturates without the constant `1` (closure ranks 129,221, 128,885, 129,053, 129,893 of 136,698 columns).
- *Controls:* N7 matches the committed record on every non-timing field; B refutes on 4 of 4; neither satisfiable draw is refuted. Valid.
- *Registered verdict:* N0 does not refute on at least 3 draws: the pure ℓ = 7 reading.
- *Per-run values:* in `score-output.txt`.

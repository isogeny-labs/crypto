#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: three guessed variables on the m = 3, ℓ = 8 draws

- *Study:* `research/dreg_closure_guess3_20261023/`, pre-registered in
  `PREREGISTRATION.md` (commit `821b33666`), no amendments.
- *Run:* 2026-10-23, 05:16 to 06:06 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Script:* `run.sh` runs `dreg_closure --to 4 --max-cols 140000 --draw K --fix …`, one process per (draw, arm), under `prlimit --as=15e9 timeout 7200`.
- *Binaries:* `dreg_closure` SHA-256 `d7246453…2ec4`, `dreg_score` `efa15cb5…18b9`, recorded in `runs/binary.sha256`.
- *Draws:* the frozen m = 3, (22, 8), N = 46 draws 0, 3, 5, 6 (unsatisfiable).
- *Arms:* G3 fixes variables 7, 15, 23 (8 branches, N = 43, 136,698 columns); G6 fixes 6, 7, 14, 15, 22, 23 to 0 (positive control); N7 is the unspecialised ℓ = 7 draw 0 (identity control).
- *Host:* the 4-core, 16 GB container, not isolated.

== Completion

- 16 of 16 processes exited 0; none killed, empty or rerun.
- Peak RSS 7.04 GB, under the 15 GB limit.
- Controls: N7 equals the committed scale-study record on every non-timing field; G6 refutes on 4 of 4. Both valid.

== Measured values

#figure(image("guess3.svg", width: 100%),
  caption: [Left: recorded outcomes per draw and arm (filled: the degree-4
  closure refutes; hollow: saturates without 1; cross: not run after an
  earlier non-refuting branch). Right: guessed variables against ℓ; ℓ = 6, 7
  and the k = 4 point at ℓ = 8 are read from earlier studies. Source:
  `plot.py` (layout only).])

- *Registered verdict:* "three guesses do not suffice at ℓ = 8" for this guess set: 1 draw suffices, 3 do not (H_grow).
- *Draws 0, 3, 5:* G3b0 saturates degree 4 without refuting; the other 7 branches were not run (registered early stop). *Draw 6:* 8 of 8 branches refute.
- *Bound only:* four guesses suffice (previous study); three do not for this set. One cell fixes no growth law.
- *Per-run values:* in `score-output.txt`.

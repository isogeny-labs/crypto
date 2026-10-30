#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: four guessed variables on the m = 3, ℓ = 8 draws

- *Study:* `research/dreg_closure_guess_20261022/`, pre-registered in
  `PREREGISTRATION.md` (commit `cd4b90dc7`), with Amendments 1 and 2 made
  before any degree-4 reading.
- *Run:* 2026-10-22, 05:55 to 09:51 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.

== What ran

- *Script:* `run.sh` runs `dreg_closure --to 4 --max-cols 130000 --draw K --fix …`, one process per (draw, arm), under `prlimit --as=15e9 timeout 7200`.
- *Binary:* SHA-256 `a11dddbb…1349` (`dreg_closure`), recorded in `runs/binary.sha256`.
- *Draws:* m = 3, (22, 8), N = 46, 44 equations; unsatisfiable draws 0, 3, 5 and 6 (exact count), seed 20261022.
- *Arms:*
  - G6 fixes variables 6, 7, 14, 15, 22, 23 to 0 (summand dimensions 6, 6, 6); this is the positive control.
  - G4 guesses variables 7, 15, 23, 24 (one per summand and one intermediate coordinate), all 16 branches, at N = 42.
- *Host:* the 4-core, 16 GB container, not isolated.

== Completion

- 68 of 68 processes exited 0.
- Amendment 2 fixed two instrument bugs before any degree-4 reading: the column cap now counts the variables that occur after `--fix`, and the early stop needs a saturated non-refuting degree-4 entry. The first launch's 8 degree-3-only outputs are kept unscored.
- The session's 2-hour background limit stopped the queue once; the run in flight left an empty output and was rerun. The rest ran in foreground batches of two.
- Peak RSS was 10.6 GB.

== Measured values

#figure(image("guess.svg", width: 100%),
  caption: [Left: recorded outcomes per draw and arm. Filled: the degree-4
  closure refutes. Right: guesses needed for degree 4 against ℓ, with the
  two registered rules; ℓ = 6 and 7 are read from the closure-driver and
  specialisation studies. Source: `plot.py` (layout only).])

- *Refuting at degree 4:* G6 on 4 of 4; G4 on 64 of 64 branches.
- *Registered verdict:* K_marginal favoured, "four guesses suffice at ℓ = 8". K_excess (6 guesses) is rejected for this guess set.
- *Bound only:* k = 4 is the smallest k measurable here, so fewer guesses may suffice.
- *Per-run values:* in `score-output.txt`.

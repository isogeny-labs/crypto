#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Run report: which guesses decide degree-3 refutation at m = 4

- *Study:* `research/dreg_m4_guess_bits_20261106/`, pre-registered before any
  run.
- *Run:* 2026-11-06 22:18 UTC to 22:55 UTC.
- *Class:* stage diagnostic. No `S`, no rho ratio, no speed claim.
- *Standing:* unverified assertion until a pcc verdict exists.

== What ran

- *Phase 1:* the #3571 design rerun (arms I and R, ten guesses, all 1024
  branches, 8 draws at ℓ = 5), 16,384 runs at degree 3.
- *Analysis:* for each arm and draw, the essential set E: the guesses whose
  value changes refutation on some branch.
- *Phase 2:* only E guessed, every assignment, 424 runs.

== Controls

- The phase-1 records pass the full #3571 scorer, and its per-draw counts
  reproduce #3571's line for line.
- Every phase-2 record matches its reduced guesses, with `identity_check`
  true.

== Measured values

#figure(image("essential.svg", width: 100%),
  caption: [Essential guesses (blue) per arm and draw; right, the refuting
  count with ten guesses and the E-only phase. Source: `plot.py` (layout only)
  on `plot-data.json` from `dreg_m4_guess_bits --plot-data score`.])

- No link-1 guess is essential on any arm-draw. Essential guesses by link:
  arm I 0, 9, 14; arm R 0, 9, 20.
- |E| is 0 to 6 everywhere.
- Guessing only E keeps every refutation on 7 of 13 tested arm-draws, all
  those with |E| ≤ 3, and gains none anywhere.

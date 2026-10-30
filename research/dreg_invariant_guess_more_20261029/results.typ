#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Guessing the swap invariant beats guessing a raw coordinate at ℓ = 8 (p = 0.031)

*Study* `research/dreg_invariant_guess_more_20261029/`, pre-registered on
fresh draws. *Class:* stage diagnostic. *Registered verdict:* S3_better.

== Question

The guess-then-degree-4 oracle fixes variables until the MutantXL closure of
the m = 3 chained S₃ system refutes at degree 4. At three guesses on the
(22, 8) cell, does fixing the swap-invariant coordinate `a₇ + b₇` of
`s = x₁ + x₂` (S3) succeed on more draws than fixing the raw `a₇` (X3)?

== Result

#figure(image("reports/run-20261029T1432Z/branches.svg", width: 100%))

- On the 8 new draws, S3 suffices (every branch refutes) on 5 and X3 on 0.
- All 5 discordant draws favour S3: one-sided exact sign test p = 0.031.
- With the 4 draws of #2851: 6 of 6, p = 0.016. S3 suffices on 7 of 12 draws
  and X3 on 1 of 12.
- Controls valid: the replay check, the identity and exactness checks on
  every run, and the positive control on 8 of 8.

== Reading

- The coordinate of the invariant `s`, not the dimension of the summands,
  governs refutation at this cell. S3 leaves both summands at dimension 8.
- In the guess-then-degree-4 oracle, invariant guesses are the measured
  better choice. This is a stage-level improvement over the index-calculus
  baseline.
- It moves no row and none beats rho: the guess count at the priced ℓ = 33
  cell is still extrapolated far above the break-even of 6.
- *Next:* two invariant guesses on the pair, ℓ = 9, and m = 4.

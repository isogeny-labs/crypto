#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= An invariant guess is never worse than a raw guess at ℓ = 8, but does not reliably suffice

*Study* `research/dreg_invariant_guess_20261029/`, pre-registered. *Class:*
stage diagnostic. *Registered verdict:* mixed.

== Question

The first link of the m = 3 chained S₃ system sees its two summands only
through the swap-invariants `s = x₁ + x₂` and `p = x₁x₂`. At an equal count
of three guesses, does fixing the invariant coordinate `a₇ + b₇` of `s` let
the degree-4 MutantXL closure refute where fixing the raw `a₇` does not?

== Result

#figure(image("reports/run-20261029T0308Z/branches.svg", width: 100%))

#table(columns: 3,
  [*draw*], [*S3 (invariant)*], [*X3 (raw)*],
  [0], [3 refute, then a failure], [fails at branch 0],
  [3], [fails at branch 0], [fails at branch 0],
  [5], [*all 8 refute*], [fails at branch 0],
  [6], [all 8 refute], [all 8 refute],
)

- *Registered verdict:* mixed. S3 suffices on 2 of 4 draws, a split; X3
  suffices on 1 of 4.
- *Paired by draw:* S3 is never worse, and is strictly better on draw 5.
- *Controls valid:*
  - the instrument identity and positive controls pass;
  - soundness, under a post-hoc rule (Amendment 1): every refutation of the
    satisfiable draw is in a branch with no solution, checked against the
    full solution list.

== Reading

- It is the first measured guess choice that beats raw coordinates at equal
  k on the same draws, but on only 1 to 2 of 4 draws. That is a direction,
  not an established effect.
- It weakens the summand-dimension reading: S3 leaves both summands at
  dimension 8 and still refutes on two draws.
- No row moves and none beats rho. The guess count at the priced cell stays
  far above the oracle's break-even of 6.
- *Successors:* more draws at this cell; two invariant guesses on the pair;
  m = 4.

#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Two guesses suffice on 1 of 12 draws at (22, 8)

*Study* `research/dreg_two_guess_20261106/`, pre-registered.
*Class:* stage diagnostic. *Standing:* unverified assertion until a pcc verdict
exists.

== Question

Three guesses (S3, L3) suffice at degree 4 on 7 and 6 of 12 draws. Does a
two-guess set, which would halve the branch count, suffice on any draw?

== Result

#figure(image("reports/run-20261106T2216Z/branches.svg", width: 100%))

- Both two-guess sets (S3 or L3 without `e₁[0]`) suffice on draw 6 only and
  fail at their first branch on the other 11.
- They never disagree: no detected difference.
- Controls valid.

== Reading

- At ℓ = 8 three guesses remain the floor for these guess kinds on 11 of 12
  draws; draw 6, a structural outlier, needs only two.
- No row moves, and rho is not reached.

#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= At m = 4 the link-1 guesses never decide degree-3 refutation

*Study* `research/dreg_m4_guess_bits_20261106/`, pre-registered.
*Class:* stage diagnostic. *Standing:* unverified assertion until a pcc verdict
exists.

== Question

In #3571 every per-draw refuting count at m = 4 (ten guesses, degree 3) was a
multiple of 16. Which of the ten guesses decide refutation, and does guessing
only those keep the refutations?

== Result

#figure(image("reports/run-20261106T2218Z/essential.svg", width: 100%))

- No link-1 guess (pair sums in arm I, raw x₂ in arm R) is essential on any of
  the 16 arm-draws. Essential guesses sit on link 3 (34) and link 2 (18).
- Guessing only the essential set keeps every refutation on 7 of 13 tested
  arm-draws, all with one to three guesses; on arm I draws 24 and 52, one
  `e₂ + x₄` guess (2 branches) keeps the refuting assignment that ten guesses
  (1024 branches) found.
- Controls valid, and the per-draw counts reproduce #3571.

== Reading

- At m = 4, degree 3, the work is done by guesses on the last links, the
  opposite of m = 3 at degree 4, where the pair invariant was the useful
  guess.
- A guess budget at m = 4 should go to links 2 and 3 first.
- No row moves, and rho is not reached.

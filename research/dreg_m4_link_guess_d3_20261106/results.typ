#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= At m = 4, degree 3, one invariant guess per link is not detectably better than raw guesses

*Study* `research/dreg_m4_link_guess_d3_20261106/`, pre-registered.
*Class:* stage diagnostic. *Registered verdict:* no detected difference.
*Standing:* unverified assertion until a pcc verdict exists.

== Question

At m = 3, invariant guesses on the chain's links did at least as well as raw
guesses (#2883, #3504). Every m = 4 cell this host can hold already refutes
at degree 4 unguessed, so the question is asked at degree 3 with k = 10
guesses: does placing them on the three links' swap invariants (arm I) refute
on more branches than raw guesses in the same places (arm R)?

== Result

#figure(image("reports/run-20261106T1633Z/branches.svg", width: 100%))

- Pooled over all 1024 branches of 8 draws: I refutes on 1,888 of 8,192, R
  on 1,312.
- By draw: I higher on 4, R on 3, both 0 on 1. The two-sided exact sign test
  gives p = 1.0, so the registered verdict is no detected difference.
- Controls valid: the instrument identity, the identity and exactness checks
  with arm I's link guesses re-derived from the basis, and every branch
  present.

== Reading

- At m = 4 in the degree-3 regime the per-link invariant placement does not
  reliably help: two draws favour it strongly, two the other way.
- The m = 3 result stays a statement about m = 3. The degree-4 three-guess
  regime at m = 4 needs a host above 16 GB.
- No row moves, and rho is not reached.

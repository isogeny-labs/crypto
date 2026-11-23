#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= An invariant guess on the second link matches the raw guess on x₃

*Study* `research/dreg_second_link_guess_20261105/`, pre-registered.
*Class:* stage diagnostic. *Registered verdict:* no detected difference.
*Standing:* unverified assertion until a pcc verdict exists.

== Question

S3 = {`a₇+b₇`, `c₇`, `e₁[0]`} is the best measured three-guess set at
(22, 8) (#2883). Does L3 = {`a₇+b₇`, `(e₁+x₃)₂₀`, `e₁[0]`}, which replaces
the raw `c₇` with a coordinate of the second link's swap invariant `e₁ + x₃`,
suffice on more draws?

== Result

#figure(image("reports/run-20261105T1642Z/branches.svg", width: 100%))

- L3 suffices on 6 of 12 draws and S3 on 7 of 12.
- One discordant draw, which favours S3. The two-sided exact sign test gives
  p = 1.0, so the registered verdict is no detected difference.
- The two arms agree on 11 of 12 draws.
- Controls valid: the instrument identity, the identity and exactness checks,
  and every draw decided.

== Reading

- In #2885, a second invariant guess on the first link was never better than
  `c₇`. An invariant guess on the second link is as good.
- One invariant guess per link does not lose at m = 3. It extends along the
  chain, so m = 4 is its test, and m = 4 needs a larger host.
- No row moves, and rho is not reached.

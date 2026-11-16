#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= A second invariant guess on the pair does not replace the raw guess on x₃

*Study* `research/dreg_invariant_pair_guess_20261029/`, pre-registered.
*Class:* stage diagnostic. *Registered verdict:* no detected difference.

== Question

S3 = {`a₇+b₇`, `c₇`, `e₁[0]`} beat the raw guess at (22, 8) (#2883). Does
D3 = {`a₇+b₇`, `a₆+b₆`, `e₁[0]`}, which replaces the raw `c₇` with a second
invariant coordinate of the pair, do better?

== Result

#figure(image("reports/run-20261029T2207Z/branches.svg", width: 100%))

- D3 suffices on 2 of 12 draws and S3 on 7 of 12.
- All 5 discordant draws favour S3. The two-sided exact sign test gives
  p = 0.0625, so the registered verdict is no detected difference.
- D3 is never better on any draw. Where both fail, D3 fails no later.
- Controls valid: the instrument identity, the identity and exactness checks,
  and every draw decided.

== Reading

- One invariant guess per summand pair is the useful part. The raw guess on
  the third summand carries weight that a second pair guess does not.
- The best measured guess rule stays S3's.
- No row moves and none beats rho.

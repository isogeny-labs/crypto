#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
= Round 5: the cofactor penalty is self-inflicted
_research/isogeny_conductor_gap_round5_20261012. E6 2026-10-12 (stage diagnostic), E7 2026-10-18 (whole-method against matched rho, and relation-phase engineering). Protocol frozen before E6; E7 section added after E6, before E7 ran._

== Findings
+ *Round 3 overstated its quotient gain; the real lever is yield (E6).* At matched columns the direct and quotient systems are the same size. A base pushed forward to E/H keeps columns and degree but lands in an index-|H| subgroup: decompositions per target grew as |H|#super[1.001 ± 0.036] for |H| = 2 to 200 (112 arms, 15 curves, 0 lift failures). F4 cost is unchanged on unsolvable targets; solvable ones cost 3–10× more.
+ *Any base inside the targets' subgroup gets that yield (E7a).* A cofactor-cleared base ([h]P for small-abscissa points) costs 0.705× (h = 4) and 0.275× (h = 72) the best fiber-closed table of rounds 1–2, as predicted (h#super[−1/3] = 0.63, 0.24). Best cell 7.3× rho (h = 72, 22 bits).
+ *Rho stays ahead.* Slope of log#sub[2] of IC/rho on log#sub[2] \#E: 0.122 [0.081, 0.163] (h = 4) and 0.068 [0.022, 0.114] (h = 72): positive, below the derived 1/6.
+ *Binary trace-zero bases (E7b, a known lever ported to ecbench).* Yield 1.94× [1.77, 2.14] over F#sub[2#super[23]] and 1.48× [1.37, 1.60] over F#sub[2#super[19]], with F4 work per relation 1.39× better and 0.87× (worse), since each solvable system costs more.

#figure(image("reports/run-20261012T0228Z/e6_yield_ratio.svg", width: 80%), caption: [E6: decompositions on E/H ÷ on E, same targets and columns, against |H|.])
#figure(image("figures/e7a_ic_over_rho.svg", width: 80%), caption: [E7a: best IC ÷ rho per curve, fiber-closed table on the raw base (hollow) against blind table on the cofactor-cleared base (filled).])
#figure(image("reports/run-20261018T1606Z/e7b_yield_ops.svg", width: 85%), caption: [E7b: yield and F4 work per relation, default against trace-kernel base.])

== Sessions
#table(columns: 4,
  [session], [records], [audit], [isolation earned],
  [e7a_h72 (ECBS1hd38ca558d7e4)], [432], [ok, 12 replays], [L2 380, L1 52],
  [e7a_h4 (ECBS1h54f0529e1598)], [480], [ok, 12 replays], [L2 395, L1 85],
  [e7b_k0 (ECBS1heff1e81e9e36)], [80], [ok, 12 replays], [L2 44, L1 36])
E6 records: e6/records.jsonl (not an ecbench session; exact counts). Run reports under reports/.

== §5a pricing band (weights 0, 0.1, 1 GAE per op)
- E7a: cleared base cheaper than the best fiber-closed arm on every curve at every weight (ratio 0.18 to 0.97). Cleared over rho spans 7.1 to 27; best cell 7.06 to 7.29 times rho. Rho is cheaper than every IC arm at every weight.
- E7b: trace-kernel MITM beats default MITM on both curves at every weight. Trace-kernel F4 beats default F4 over F#sub[2]#super[23]; over F#sub[2]#super[19] the pair is unresolved. Priced F4 arms trail rho by 10#super[4] to 10#super[6].

== Leads, ranked
+ Solvable systems cost 3–10× more to solve; every yield lever moves work there. Measure where (basis completion or solution extraction) and decide consistency first.
+ Affine trace-kernel subspaces for Tr(a) = 1 (a = 1 Koblitz, h = 2) need an offset in the descent.
+ Map where the trace-kernel lever pays (n = 17–29, dimension 8–10) and measure the default base's 2E fraction.
+ Close the prime-field pair-table line: a generic method at 7–17× rho whose gap widens.

#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Split evaluation on canonical fibers
T-1871 · 2026-11-07 · Prime-field public-point recovery

Selected roots now use scalar quotient arithmetic; unresolved fibers retain quadratic pairs. One coefficient reader supplies both components. The state update filters each support separately and joins newly selected roots with existing roots by coprime CRT.

#image("flow.svg",width:100%)

*Correctness.* Polynomial evaluation commutes with reduction modulo B and substitution V=v modulo S. Balanced composition returns the block value and base power, so each coefficient is read once even in mixed states. The canonical update retains the same candidate roots.

#table(columns:(3fr,1fr),inset:5pt,
 [Native control],[Observed],
 [Mode projections],[738],
 [Mixed / selected-only / unresolved-only modes],[693 / 45 / 0],
 [Canonical states equal to reference],[2,516],
 [Public targets per cap64 / cap16 panel],[281 / 281],
 [Source calls at cap64 / cap16],[2,881 / 3,152],
 [Route changes / unresolved targets],[0 / 0])

*Costs.* For L coefficients, each present component makes 2(L−1) ring products. Scalar products invoke one base polynomial product; quadratic products invoke three. These are structural calls, not field-operation or timing ratios. Mixed states pay both components. Setup, reductions, stream production, gcds, inverse/CRT work and temporary memory remain charged. Original source grids and initial C0 remain charged.

*Evidence.* validation01 contains compiler/source pins and native output; all inherited checks pass. No native failure or cloud/timing trial. Independent signed verdict pending. Original 4,563-case cohort unchanged. Full details and replay command: DERIVATION.md, RESULTS.md and README.md.

*Sources.* Exact canonical predecessor and official FLINT polynomial quotient conventions: flintlib.org/doc/nmod_poly/index.html. KB unavailable; standard projection/CRT, no novelty claim.

*Next.* Remove repeated evaluator powers or share branch compilation, keeping source and initial-support costs visible.

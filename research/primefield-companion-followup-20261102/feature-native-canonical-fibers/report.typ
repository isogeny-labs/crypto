#set page(paper:"a4",margin:16mm)
#set text(size:9.5pt)
= Canonical oriented state without equation history
T-1869 · 2026-11-07 · Prime-field compressed recovery

Replace a growing list of pair equations with disjoint monic supports B (both roots) and S (one selected root), plus the selected root function v modulo S. Ramified fibers W=±2 are selected from initialization, making the representation independent of update order.

#image("flow.svg",width:100%)

*Exact update.* Selected roots are filtered by gcd(S,a+bv). On B, split where b vanishes: retain both roots only if a also vanishes. Else select −a/b where its quadratic norm is zero. Invert only after restricting to a unit component; merge old and new selected roots by coprime CRT. This preserves exactly the same candidate set and removes all prior-equation storage.

#table(columns:(2.8fr,1fr),inset:4pt,
 [Native exact control],[Result],
 [Candidate-membership checks],[852],
 [Reverse-history / duplicate controls],[3 / 18],
 [Additional zero equations without history growth],[96],
 [States matching persistent reference],[2,516],
 [Live coefficient-slot sums: reference / canonical],[75,387 / 58,119])

*Public recovery.* Both281-target panels retain all scalars and routes: cap64uses2,881source calls; cap16uses3,152. Zero unresolved in both. At cap16,260later updates and265lift deferrals are retained. Existing adapters remain as controls.

*Costs and limits.* Four vectors are retained, including cachedG=B·S: at most3d+4live coefficient slots, independent of history length. Slot sums are not peak memory or allocated bytes; charge capacities/metadata and transient workspace. Updates add at mosttwo polynomial inversions and one CRT merge. The current inverse uses ordinary extended Euclid with a conservative O(d M(d))bound; no universal arithmetic/timing advantage is claimed.

*Evidence.* validation01 pins64transitive inputs and passes with overflow checks. Order, idempotence, ramification, nonunit and CRT controls pass. No native/report failure, cloud/timing trial or fullCargo rerun. Original4,563cohort unchanged. Producer evidence unsigned; full proof/replay in DERIVATION.md.

*Sources and next step.* Exact persistent predecessor and official FLINT CRT/modular conventions (flintlib.org/doc/nmod_poly/index.html). KB unavailable; no novelty claim. Next evaluate selected-root modes directly modulo S, reserving pair algebra for the both-root component.

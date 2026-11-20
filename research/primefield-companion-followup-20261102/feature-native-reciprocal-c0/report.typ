#set page(paper:"a4",margin:16mm)
#set text(size:9.5pt)
= Half-degree reciprocal constraints with exact phases
T-1862 · 2026-11-07 · Prime-field first sign-mode constraint

When 4m divides r−1, write h=2mn. Binomial symmetry makes F₀ reciprocal of degree2n. The substitution W=v+v⁻¹ gives F₀(v)=vⁿH(W), where H has degree n. Balanced affine Clenshaw composition builds H and Dₙ from the upper n+1 streamed coefficients, without expanding F₀.

#image("flow.svg",width:100%)

*Exact phase transfer.* On the nonzero image domain v^(4n)=1. For each u in {1,−1,i,−i}, form gcd(Dₙ(W)−u−u⁻¹, uH(W)−c). Lift surviving factors through v²−Wv+1 and intersect with vⁿ−u using modular powering. This preserves orientation and removes ramified multiplicity. Add zero exactly when f₀=A, then substitute v=tZ and normalize. The result equals the original first-constraint gcd.

#table(columns:(2.8fr,1fr),inset:4pt,
 [Native exact check],[Count],
 [Configurations / nonzero identity values],[27 / 4,496],
 [Branch / lifted point checks],[308,844 / 305,632],
 [Exact full image polynomials],[2,409],
 [Wrong orientations rejected / ramified incidences],[328 / 31],
 [Lift-cap rejection controls],[183])

*Integrated public recovery.* All281targets on F257, y²=x³+x+7, base(1,3), order281, mode5 match isolated scalar/routes/statistics:2,881source calls, zero unresolved. At the existing seven-element lift cap,10of275compressed attempts complete and265fall back. Fallback costs are additional. The inherited617-target panel remains5,929calls, zero unresolved.

*Costs and limits.* Balanced composition has O(M(n) log n) polynomial work plus coefficient generation; four degree-n gcds remain. H and Dₙ retain2(n+1)live values, with capacities/metadata charged. Temporary bound O(n log n+B); capped lifts, modular powers, inversions and up to64public quartic-root search exponentiations are charged. Half degree is not a total-work or timing claim. Unsupported parameters and exceeded lift/union caps use the original constraint.

*Evidence.* validation01 pins54transitive inputs; native overflow checks pass. Unsigned producer evidence. No cloud/timing trial or fullCargo rerun; original4,563-case cohort unchanged. DERIVATION.md provides proof, replay, controls and historical limitations.

*Source and next construction.* Bhargava–Zieve, Factoring Dickson polynomials over finite fields, websites.umich.edu/~zieve/papers/manjul.pdf; exact predecessor code. KB unavailable; no novelty claim. Next retain compressed phase-tagged states through later modes, avoiding the early lift that triggered265fallbacks.

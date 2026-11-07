#set page(paper:"a4",margin:16mm)
#set text(size:9.5pt)
= Keep reciprocal candidates across radii
T-1867 · 2026-11-07 · Persistent oriented quadratic fibers

The recovery adapter now retains compressed candidates when lifting exceeds its cap. Later radii act in the first radius's coordinate V=t₀Z: evaluate modes at (t/t₀)V, append constraints and shrink the same state.

#image("flow.svg",width:100%)

*Exact history invariant.* All retained linear forms a(W)+b(W)V share a quadratic root exactly when every norm and every pairwise determinant vanish. Each append checks its norm and determinants against all prior forms, then reduces their coefficients modulo the new G. This handles identically zero phase equations and opposite-orientation traps. Lifting still enforces every equation and the exact zero condition.

#table(columns:(2.8fr,1fr),inset:4pt,
 [Native exact control],[Result],
 [Dense state equalities across four radii],[2,516],
 [Small-cap deferrals / histories later fitting],[626 / 497],
 [Maximum retained equations],[8],
 [Fifth-radius / ninth-equation rejections],[629 / 21],
 [Degenerate-phase orientation traps],[21])

*Public recovery.* All281targets on F257/order281/mode5 match isolated recovered scalars and routes. At caller cap64 both use2,881source calls; no later updates are needed. At cap16 both use3,152calls; the persistent path records260later updates and265lift deferrals. Both panels have zero unresolved. Temporary lift cap=C and image cap=floor(C/m); these differ from the earlier first-radius helper's B=7 lift policy.

*Costs.* Retain at most8pairs per branch, at most4branches, each degree at most n: O(sn)live coefficients, s≤8, with Vec capacity/metadata charged. Mode evaluation, determinant/gcd updates, failed lifts, source setup and final replay remain charged. C is not a process byte ceiling. No timing or total-work advantage is claimed.

*Evidence and failure.* validation02 pins60transitive inputs and passes with overflow checks. validation01 passed the state controls but failed an unjustified expectation that cap64must defer; failed checker/receipt preserved. Corrected checks report actual counts and add cap16. Producer evidence unsigned; no cloud/timing run or fullCargo rerun; original4,563cohort unchanged.

*Sources and next step.* Exact predecessor; Bhargava–Zieve reciprocal fibers; official FLINT modular conventions (links in DERIVATION.md). KB unavailable. Next canonicalize the oriented history or share repeated mode evaluation while preserving the full common-root invariant.

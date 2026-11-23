#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Convolution for recurrence block progressions
T-1857 · 2026-11-07 · Conditional arithmetic improvement with exact fallback

Eligible packed batches now use cached Lagrange weights and a low convolution. Mixed point sequences, singleton batches and zero shift denominators retain the existing tree evaluator. Point ownership, coefficient recurrence and public recovery are unchanged.

#image("flow.svg",width:100%)

*Identity.* For H(X)=P(sX), degree at most d, cache W_i=H(i)(−1)^(d−i)/(i!(d−i)!). For first point x0, put a=x0/s. With c≤d requested values, form S_j=1/(a+j−d), j=0..d+c−1, only if every denominator is nonzero. Lagrange interpolation gives H(a+k)=Delta_k times coefficient d+k of W S, where Delta_k=∏(a+k−i), i=0..d. The same inverse array advances Delta. Truncation after d+c coefficients preserves all requested outputs.

*Cost.* Cached setup uses one d-point tree, one extra Horner sample, weights and two explicit inversions. An eligible query uses O(M(d)+d) polynomial/scalar work, one inversion and O(d) auxiliary field elements. Inverses are implemented by exponentiation and each adds O(log r) modular multiplications. Setup and the earlier block-polynomial construction remain charged. Fallback retains the old cost; small-degree timing dominance is not asserted.

#table(columns:(2.5fr,1fr,2fr),inset:4pt,
 [Native exact check],[Count],[Outcome],
 [Nonsingular / singular queries],[31,230 / 2,538],[114,093 equal values],
 [Integrated interval products],[36],[694convolutions / 10query trees],
 [Real coefficient stream],[21batches],[7convolutions / 14query trees],
 [Public m56 panel],[281],[14,421calls; zero unresolved],
 [Inherited public panel],[617],[5,929calls; zero unresolved])

*Counter scope.* Query-tree counts exclude the separately charged setup tree. Inherited flush counts are now labeled batches, since a flush need not build a tree. Point batches remain width-bounded; weights have w+1entries and inverse/convolution arrays at most2w. Kernel workspace and the setup tree remain explicit.

*Evidence.* validation01 pins49transitive inputs and stdout; overflow checks enabled. All inherited arithmetic/state controls pass. PCC-20261107-0995b5 is unsigned producer evidence. No native/report failure, cloud trial or fullCargo rerun. Original4,563-case cohort and prospective trace protocol unchanged.

*Sources and next step.* Bostan-Gaudry-Schost2004 Figure1/Proposition1 and progression scaling, plus exact predecessor/native low-product kernel; KB unavailable. This implements an established formula with checked preconditions, not a novelty claim. Next reuse cached samples for overlapping shifts or reduce repeated inversions. DERIVATION.md supplies proof, limits and replay; historical graphs remain unchanged.

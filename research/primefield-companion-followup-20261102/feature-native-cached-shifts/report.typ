#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Answer overlapping shifts from cached samples
T-1859 · 2026-11-07 · Total progression evaluation in the prime scalar field

Progression setup now retains its already-computed samples H(0)..H(d). Queries copy cached indices and evaluate only the outside-grid segment. Every valid progression shift is handled; irregular and singleton dispatcher batches retain the tree evaluator.

#image("flow.svg",width:100%)

*Segment proof.* A cyclic query contains at most d points. Two outside-grid segments would require crossing the entire cached arc of d+1points, which is impossible. There is therefore at most one outside segment. If it starts at b>d and ends at b+t−1≤r−1, its convolution denominators b−d..b+t−1 all lie in1..r−1. Cached copies and that nonsingular convolution give the exact requested sequence, including wrap.

*Costs.* Setup performs no additional sample computation: it keeps d+1live values already obtained. Vec spare capacity, metadata and allocator overhead remain charged; see README.md. A cache-only query uses one normalization multiplication and O(c)copies, with no query inversion or polynomial product. Other queries use at most one convolution/inversion plus copies. Setup, source, first-constraint and validation costs remain explicit; no timing crossover is claimed.

#table(columns:(2.5fr,1fr,2fr),inset:4pt,
 [Native exact check],[Count],[Outcome],
 [Total-shift queries],[50,652],[167,769equal values],
 [Previously singular queries],[3,780],[All answered exactly],
 [Cache-only / mixed queries],[2,214 / 1,566],[Zero / one convolution],
 [Copied values / wrapped queries],[15,579 / 981],[Exact order preserved],
 [Public recovery panels],[281 / 617],[14,421 / 5,929calls; zero unresolved])

*Controls.* The inherited36-product panel now has700progression batches and4irregular query trees. Progression count is not convolution count. The realr1009/m56 stream has no overlap in this panel and remains7convolutions/14query trees. Independent direct Horner, prior optional evaluator, caps and all inherited arithmetic/state controls pass.

*Evidence.* validation01 pins50transitive inputs and stdout; overflow checks enabled. PCC-20261107-020740 is unsigned producer evidence. No native failure. Initial report syntax error corrected; failed source and errata retained. No cloud trial or fullCargo rerun. Original4,563-case cohort and prospective trace protocol unchanged.

*Sources and next step.* Exact predecessor and Bostan-Gaudry-Schost2004 denominator conditions; KB unavailable. The convolution formula is established; the cyclic segment proof covers its exceptional shifts. Next share inversion work or reusable context across streams under an explicit memory cap. DERIVATION.md and README.md give proof, storage qualification and replay; historical graphs remain unchanged.

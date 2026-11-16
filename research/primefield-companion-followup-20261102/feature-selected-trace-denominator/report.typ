#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Selected normalization of character traces
T-1946 · 2026-11-08 · PCC-20261108-075c65 · Unsigned producer evidence

*Construction.* Retain V=U·c for the coefficient window, with U a unit. At step k, strip the characteristic factors from the recurrence numerator and working modulus. Scale surviving entries by the remaining denominator factor and update U. Invert U modulo r only when the coefficient contributes to a requested trace. Constant coefficient requires no inverse.

#image("flow.svg",width:100%)

*Exact checks.* 7,965 coefficients in45 polynomial controls,90 traces and all36 word systems match. Nine cap decisions and their costs are retained. Fourteen singleton phases checked. Public29 outcomes match:12 extracted,11 identity/endpoints and6 subset caps. Steps, calls and precision boundaries agree. Empty request, malformed index and both large preflight refusals pass. No native compile/assertion failure.

*Accounting.* Trace inversions648 versus15,840; products284,086 versus210,078. All word systems including refusals:184,722 inversions versus2,764,200;48,651,090 products versus35,982,184. Public selected products53,059,596 versus38,436,864. Selected inversions are in Fᵣ; reference inversions use the current lifted ring. These counts support no timing or total-work ranking. Each public arm retains2,654,208 steps,332 nominal+27 guard queries and19,844 correction multiplications; combined718 queries excluding setup.

*Bound and state.* Two trace index sets have at most2d distinct positive positions, or d for L1, so normalization uses at most2d inversions per degree-d subset. Request-set/window storage is O(d) entries; precision, containers and source/phase circuits are additional. All coefficient steps and explicit subsets remain. Caps500,000 steps/4,096 subsets/native modulus ceiling unchanged.

*Custody and next.* Thirteen copied files/17 dependencies hash-bound. Original4,563-case cohort and prior code unchanged. Next nonsingular matrix blocks with explicit singular boundaries and requested checkpoints; no block jumps are implemented here. The whole-word subset representation remains a separate dependency. Source, set lookups, reductions, Euclidean work, allocation and verification remain charged.

*Sources.* T1944 precision proof/source and Harvey–Sutherland II section2 equations2–4/remark2.2. Common-denominator formulation is established; no novelty or many-prime bound imported. KB unavailable. Independent signed review pending; no cloud work.

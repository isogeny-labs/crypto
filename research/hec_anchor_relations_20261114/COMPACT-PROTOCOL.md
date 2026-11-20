# Compact signed interpolation kernel, frozen before implementation

The10x mathematical objective remains unchanged. Current signed-orbit pilot gains1.27..2.94x
against its frozen table controls, on existing instances only [pcc:unverified]. Investigate the
dominant relation stage without changing relation distribution, anchor selection or rank.

## Exact reduction

For g in2..4, monic A of degree g+1, h of degree<=g, monic f of degree2g+1, assume A divides
N=f-h^2 (guaranteed by distinct rational interpolation points). Q=N/A is monic of degree g.
Let q_g=1. Descending k=g-1..0,

    q_k = N_(g+1+k) - sum_(j=k+1..g) a_(g+1+k-j) q_j.

This follows by comparing coefficient g+1+k in A Q=N. Only N coefficients g+1..2g are
needed. Do not compute the low norm coefficients, a polynomial inverse, or a remainder.
For g2: q1=f4-h2^2-a2, q0=f3-2h2h1-a1-a2q1.
The full-divisibility obligation is retained in tests and full principal group gates.

Use fixed bounded coefficient arrays for barycentric basis and cross-term polynomials.
Emit the same2^g Gray-ordered (h,Q) pairs as orbit.rs, with no per-tuple heap allocation
inside the compact kernel. Existing caller buffers, sorting, forest, oracle and cold setup
are unchanged. Inputs must be valid distinct rational places and correct barycentric weights.
The reference implementation remains selectable. No new sampling or matrix algorithm is used.

## Gates and falsifiers

1. Compare every output to reference orbit and fresh Newton interpolation/full norm division
   on random tuples in all11 original instances, all sign classes, including degree drops.
2. Exercise all distinct rational-place tuples on small fields (branch places included).
3. Paired complete solves must have equal k, catalogue/anchor size, draws, unsigned tuples,
   accepted rows, pure rows, unions, pins, descents and incremental solve insertion counts.
4. Run every admitted raw/projected row group check on the11instances x5seeds before timing.
5. Any mismatch rejects the kernel; retain original source and failure record.

Only after gates: interleave reference orbit, compact orbit and frozen table control, five
seeds, same known-answer target per instance. All cold costs and final verification included.
Use reference source868eec5948, preserve earlier raw records unchanged. Report all per-size
medians and per-seed data, L0 limitations, caps and losses. No >=10x or exponent prediction is
registered for this engineering kernel. A negligible gain redirects work to tuple/rank structure
and held-out scaling, rather than further equivalent arithmetic rewrites.

## Sources checked

KB revision9141e386254340a9bdba3036f2bb706a, last refresh2026-11-14T21:44:48Z, ready true,
lexical only; snapshot predates current studies. Queried interpolation/principal norm and monic
high-coefficient division. Relevant existing warning source:b3c90d39a347ef661ccf10680a742186,
crypto:research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md, cost-model correction:
do not convert coefficient savings into a total-time claim. Source and bounded context opened.
Other returned hits were not evidence for this kernel. No novelty inferred from search gaps.
Primary reference opened: https://flintlib.org/doc/nmod_poly/divrem.html, quotient-only and exact
division, high-coefficient dependence. This is standard long division specialized to known
interpolation divisibility. The document notes generated text; our recurrence is derived directly
above and checked against the existing implementation, rather than trusting timing assertions.

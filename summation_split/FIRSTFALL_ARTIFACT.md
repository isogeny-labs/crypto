# Explicit degree-two first-fall certificate: a negative complexity control

**2026-11-23 | follow-up PR #5063 | [pcc:unverified]**

Scope: only the unspecialized Semaev relation as compiled in the complete two-sector polynomial circuit (merged PR #5058). No factor-base equation and no ECDLP claim.

## Exact formal cancellation for every arity

Write (x_i,y_i) for the input leaf, s_i^2-s_i=0 for the Frobenius-sector selector, and u_i^+, u_i^- for the respective base/twist leaf abscissae. The actual polynomial generator list includes, for **every** i,

g_i^+ = u_i^+ - (1-s_i)x_i,  g_i^- = u_i^- - s_i x_i.

Both have total degree two, yet their constant-coefficient linear combination is

g_i^+ + g_i^- = u_i^+ + u_i^- - x_i,

a nonzero polynomial of degree one. The same identity holds for y_i and the two ordinate state variables. Thus, **under the standard presentation-dependent definition** in which a first fall occurs when a degree-d combination of multiples of generators produces a nonzero polynomial of degree strictly less than d, the system has a first fall at degree **at most two**, for *every* m, prime p>3, and curve parameter pair. No cancellation beyond two generators is required. The identity is exact over Z; it survives reduction modulo every prime and optional quadratic field-lift gadgets.

The negative result is the important point: **constant first-fall degree is automatic here and cannot justify a constant F4 solving degree.** It is a redundancy/coordinate-splitting artifact: elimination of the u_i^+,u_i^- variables simply returns the original leaf coordinate, and the exponential degree of S_(m+1) has not disappeared from the projection. The chart circuit has additional Boolean and inverse branches. It remains unknown whether d_F4 or true solver work is reduced. Conversely, maximum generator degree four does not measure first fall.

## Reproducible algebraic negative control

Run from repository root:

    PYTHONPATH=. python -m unittest -q summation_split.test_sector_firstfall

The test checks every leaf for the actual two generator identities, their individual degrees two, their nonzero linear combination of degree one, and the exact cancellation, both with and without quadratic finite-field membership lifts. Parameter sets: (p,a,b,m)=(5,0,1,3),(7,1,1,3),(11,1,1,4),(2^31-1,1,1,5).

In a **local producer-only** Python/SymPy fallback (not independently signed), all eight circuit settings passed and the first-fall artifact was exhibited for all m+1 leaves and both coordinates. Largest field-lifted Prime31 circuit: 862 variables and 1240 generators. Dedicated native GitHub CI must replay source after commit to establish a source-specific executable receipt.

This is **not** an F4, msolve, Sage or FGb benchmark; no d_F4, Macaulay degree, wall advantage or bounded regularity has been measured or proved. The explicit linear combination does not supply an asymptotic improvement.

## Next falsifier

Compare complete sector circuit, a redundancy-removed coordinate split, full projective S3 chain and original monolithic Semaev polynomial under the **same unspecialized positive-dimensional slicing and Fp semantics** using genuine F4/MSolve and a declared maximum Macaulay degree statistic. Admission requires >=50 exact projected-set gates per configuration and independent reproducibility, together with wall time and peak memory. Equality of projected finite-field point sets is not identity of elimination ideals over the algebraic closure.

Sources: Semaev 2004 https://eprint.iacr.org/2004/031 ; Kosters-Yeo 2015 https://arxiv.org/abs/1503.08001 ; earlier repository findings summation_split/FINDINGS.md, SECTOR_FINDINGS.md and SECTOR_COMPLETE.md. Research-KB service unavailable in this runtime; no novelty claim.

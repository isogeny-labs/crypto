#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Streaming lifted character-product traces
T-1944 · 2026-11-08 · Unsigned producer evidence

*Construction.* The established recurrence for coefficients of f^h uses the previous degree(f) coefficients. Initialize modulo r to the power1 plus the r-adic valuation of (degree(f)·h) factorial. At step k, remove exactly the characteristic factors of k from the numerator and working modulus, then invert the unit denominator. Two accumulators retain the selected phase traces; no dense coefficient vector is retained in production.

#image("flow.svg",width:100%)

*Kernel checks.* 45 polynomial powers,7,965 coefficients,90 character traces and60 precision-loss steps pass. Largest retained window8 and modulus6,234,839,521 in those controls. A concrete r19 Frobenius counterexample has the same differential equation, degree, constant and leading coefficients but a different weighted trace, showing why direct mod-r division cannot simply cross its zero denominator.

*Word checks.* 27 of36 systems match exact moments;14 singleton phases checked. Nine caps:6 subset preflights and3 step preflights after499,728 steps each. Successful work1,265,016 steps/15,580,180 modular products; refused work adds1,499,184 steps/20,402,004 products. Refusal rows retained. Retained window reaches12 entries; bit precision and container/temporary storage are additional.

*Public accounting.* 29 fixtures:12 extracted scalars,11 endpoints,6 subset caps. Source queries332 nominal plus27 guards; source setup additional. Recurrence2,654,208 steps,38,436,864 modular products and49,152 subsets; endpoint correction19,844 field multiplications. Moduli can be powers of r, so products are not the predecessor's field monomial-pair metric. Inversions, subset polynomial construction, additions, reductions, source/phase and allocation remain additional. No timing or total-work ranking.

*Boundary and next.* Caps500,000 steps,4,096 subsets, modulus≤i64MAX. Large step/precision controls refuse before recurrence work. The degree-sized state still uses linear coefficient stepping and exponential subset expansion. For fixed word-only identification of every nonendpoint image, 2^N≥L−B is necessary. Next study structured matrix products and complete-word traces without explicit subsets, accounting for both costs.

*Custody and sources.* Both overflow-checked builds/checks pass within90/60-second limits. Twelve copied sources/16 dependencies bound; original4,563-case cohort unchanged. Harvey–Sutherland II, section2 equation2 and remarks2.1–2.2, supplies recurrence/lifting precedent; its many-prime average bound is not this single-prime path. KB unavailable. No cloud work, novelty claim or signed independent review.

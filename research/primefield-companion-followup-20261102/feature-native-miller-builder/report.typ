#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Implemented Miller/Frobenius column builder
2026-11-05 · T-1623

Cubic character columns now use a short coefficient recurrence with a Frobenius anchor at the characteristic-r singular step. The generated r=479 packet is byte-identical to the dense reference.

#image("pipeline.svg", width: 100%)

== Construction and proof
For F=g^h and h=(r-1)/2, gF'=h g'F determines each coefficient from at most three predecessors, except at index r. The coefficient of X^r in gF²=g(X^r) determines the missing value using earlier coefficients and a nonzero denominator. Zero shifts factor out before powering. The completed ordinary polynomial is reduced only afterward. A shared inverse table supports every column.

== Checked equivalence
All 49 optimized native tests pass with overflow checks. Across seven prime fields, 1037 feature columns agree coefficient-by-coefficient with dense polynomial products, including 571 singular anchors. The r479 system includes all697 columns and455 singular anchors. The q15 inconsistency control, all479 output identities and eight source integrations pass. Elimination and residual code remain byte-identical; the final478-term packet also matches exactly.

The independent checker binary was hash-verified and run on the newly saved packet. It accepts the exact polynomial identity and all479 field values. Twenty source/data hashes verify before and after build. Four compiler warnings remain in the raw log.

== Cost and next step
For bounded cubic degree, column work is O(r+log r) field operations after shared O(r) inverse setup, replacing the reference's quadratic polynomial products. This is a derived algorithmic bound, not a timing result. Dense elimination still costs O(r²m) and stores O(rm) entries; its56196286 inner updates are unchanged on this fixture. Next test whether the column recurrences support structured elimination or a reusable factor.

== Evidence
DERIVATION.md, recurrence.rs, lib.rs, source-lock.json, receipts and raw logs. No cloud job or timing panel ran; original4563-case cohort unchanged. The standard recurrence is documented in #link("https://fredrikj.net/fdoc/html/fmpz_poly/_fmpz_poly_pow_multinomial.html")[FLINT's Miller formula]; the saved derivation supplies the finite-field anchor explicitly.

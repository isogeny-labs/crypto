# Prepared C128: a per-precision generating basis for discarded corrections

Status: derivation and static Python preflight only; no scientific worker has executed this source.

Let m=d-1 and monic degree2/3 factors A=Z^2+A1 Z+A0,B=Z^3+B2 Z^2+B1 Z+B0 have envelopes m,2m for A1,A0 and m,2m,3m for B2,B1,B0. The normalized input F4,F3 have u-degrees <=m,2m. At precision n, write the Sylvester corrections (a0,a1,b0,b1,b2). With leading factors Z^2 and(Z-1)^3, the top correction equations are e4=a1+b2 and e3=a0+beta*a1+b1,where beta=-3 in the coefficient field. No generic rank or root distinctness within clusters is assumed.

For n>m, coefficient Z^4 of the clipped product is A1+B2,already degree<=m,so e4=0. Thus b2=-a1. For n>2m,the clipped Z^3 coefficient A0+A1*B2+B1 has degree<=2m,so e3=0 as well; hence b1=-a0-beta*a1. Consequently all discarded corrections vanish iff a1=0 for m<n<=2m,and iff a0=a1=0 for2m<n<=3m. Keeping only expired A corrections generates the entire discarded-correction condition. For m17 this uses51 field-valued conditions instead of102. This is a per-precision module basis; it does not assert global algebraic independence on the63-parameter norm family or a search-dimension reduction.

The proof holds over any coefficient field,including characteristic3 where beta=0. The prepared implementation below is binary only and reuses the C127 clipped generator. Retain exact raw product equality and all recovery words. Prefix constraints are necessary; the full basis only enforces the bounded-factor lifting defects,not five-root support or original all-roots recovery by itself.

Protocol: independent small acceptance/root-multiset oracles; assert the paired row identities and full-basis equivalence,including repeated roots and zero coefficients; then fixed1/2/4/8A-row profiles. Test odd-field correction identities in a separate coefficient-field component using the general Sylvester map. A stronger8row prefix is a changed assumption from C127's two duplicate rows,but must be frozen and measured rather than repeated unchanged. All budgets90s/4GiB/CPU0 and full source/U/outer/matching-baseline obligations remain.

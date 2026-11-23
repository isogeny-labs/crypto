# Explicit proper sign-quotient equations, before data

The genus31 sign quotient is retained R25/R27 geometry, not a new cover.
Construct its actual monic proper chart equations from the accepted R50
integral-v charts. Finite delta=u^4+u^3+(u+1)v+1; infinity
deltaInf=1+z+(z+z^2)V+z^4. Reduce Q modulo E and t^2+delta*t+w.
Require no t remainder and monic w degree8. Check complete lift products
for Q minus the reduced quotient equation, the sign-shift source identity,
and the nonzero base norm of delta. Check generic degrees16 and32 for the
quotient and full coupled presentation respectively. Export each chart and
all coupled witnesses before proceeding. Then check the complete quotient,
base, delta and double-cover overlap differences against u*z+1, as well as
the additional w-coordinate roundtrip. All remain over F2.

The monic double-cover presentation is free with basis1,t over the quotient
ring. Translation t->t+delta fixes an element a+b*t iff b*delta=0. The
actual source is a domain and delta has nonzero base norm, hence b=0.
Thus this is the actual invariant ring, including the exceptional locus;
it is not just a rational invariant subfield. The quotient charts glue by
W=z^8*w,V=z^2*v and form a finite rank16 proper model over P1. Their
transition weights8i+2j give arithmetic genus449; under the retained genus31
bridge their normalization defect is418. This stage does not normalize that
model or certify a polarized target inclusion. Existing sign-invariance
proofs are retained; full polynomial coefficients remain native CAS data.

Same4GiB/1200s, seed50/one thread, serial exact CAS. Exit0, no error lines
and QUOTIENT50_ALL_CHECKS_PASS required. Keep every partial export/error/cap.
No coefficient search, performance comparison or statistical claim.

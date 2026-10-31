# Least scalar denominator for the completed closure, before affected data
Data-informed followup: closure52-02 passes with lengths376,220,200,180,
stability and determinant u^60(u+1)^120. Compute the least scalar annihilator
of the exact residual cokernel,not merely its determinant. Its only possible
irreducible factors are u,u+1. For the first exponent use exact whole32-column
membership of u^a(u+1)^120 I, a=0..60; analogously u^60(u+1)^b I,b=0..120.
Monotone binary search of these complete module tests determines both minima.
Check the combined candidate directly via a full32x32 recovery product.
Export both predecessor nonzero normal remainders against a freshly computed
standard basis,with both full standard-basis change products. Since the
annihilator is a principal ideal containing the determinant,these minima
identify its generator. Zero exponent is allowed and has no predecessor.
Reject changed recovery coefficient. Retain full matrices and remainders.

Reference: frozen R52 closure protocol and official Singular module std/lift
and reduction documentation already reviewed. Exact computation,no timing or
statistics. Same seed52,one thread,4GiBvirtual,1200s,serial,Git freeze before
data. Require exit0,no native ? errorlines,ANNIHILATOR52_ALL_CHECKS_PASS.
This supplies denominator recovery on u(u+1)!=0 with the retained normal-chart
premise. It is not a proper polarized embedding or target-factor action.

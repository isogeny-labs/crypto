# Binding the actual subtraction convention

Additional interface obligation frozen before its implementation/call,
2026-11-26, PR #5256. The original composition test is unchanged and runs in
its own immutable snapshot. This test does not widen any earlier criteria.

The geometric correspondence induces phi on degree-zero classes. The software
uses E(P)-E_ref for P-infinity, with reference indices 225,224,223. Bind that
exact convention by independently computing the complete specialization of
the actual U,V at the odd-degree source infinity, not just their differentials.

For each original function (a+bY)/c, require deg(a)<=deg(c) and either b=0
or 2*deg(b)+7<2*deg(c). On the smooth monic degree-seven source, ord(S)=-2
and ord(Y)=-7. The Y part then vanishes and the limit is the leading a/c
coefficient when their degrees match, otherwise zero. All graph coefficients
must be regular there; no alternate chart or extra precision after a failure.

Require U_infinity monic degree three and equal to the product of all three
retained x-reference factors, which must be distinct. Require V_infinity at
each x to equal that point's actual y, and replay quartic incidence for all
three points. These checks give a finite free rank-three source-DVR family
with three distinct finite special points, identifying the whole special
fiber with E_ref. Thus the actual subtraction convention sends source
infinity to zero and agrees with the induced homomorphism. No point search.

Controls: changed limiting V constant must fail reference binding; dropping
a reference factor must fail the whole-specialization identity; adding Y to
a valid coefficient must fail the regularity check. Retain exact limits,
three reference coordinates, source/input/binary hashes, phase clocks, exits
and full build/worker/wrapper diagnostics. One allowed CPU, 1 GiB, strict
build <=300 seconds and verification <=600 seconds, as before. Earlier
evidence and original composition source/protocol stay unchanged.

Sources: the exact complete-map and global-correspondence reference
conventions; Galbraith corrected chapter 7 uniformizers/extension of maps;
Milne III.6 Albanese normalization. This binds actual coordinates to the
standard theory, without a new PCC registration or novelty claim.

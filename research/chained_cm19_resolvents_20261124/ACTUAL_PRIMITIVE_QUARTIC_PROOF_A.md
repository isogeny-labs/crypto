# Actual primitive quartic over the original elliptic base

Native source ec5226f28ef38f235080ffa2434ea97437e6762b, protocol59eb12,
lock3d5c8e, data11a40a. Seven phases exit0, unchanged guards; complete
50618B arithmetic certificate and whole small originals in first-A-native.
This is a written actual-object bridge, not an independently signed verdict
or a formal theorem about Jacobians. [pcc:unverified]

The qualified parent gives genuine g=f4-(3,5)=(p4/q2,p6/q3). Addition on
the actual E gives b=p4-3q2, u=A/b2 with
A=3p4^2+4p4*q2+4q4+4p6*q. Native A reconstructs the unique degree12
Z4-normal-form B12 from the full70-by46 system and certifies
q3*B12=(p6-5q3)(p4*b2-A*q2)-p6*b3 modulo F_C. Thus v=B12/b3 is the
true translated y coordinate; the additional full E-equation identity is
a consistency check, not the only argument identifying the map.
[pcc:unverified]

For the bounded construction, the smooth plane quartic has canonical
bundle O_C(1). Its actual ramification section 2X+Z has complete simple
zero divisor R. The finite flat map f has rank4 and branch divisor
B=f_*R. The determinant norm of this section has divisor B, including
each simple tame ramification point with norm valuation1. Consequently
Norm_f O_C(1)=O_E(B). Norm(TX-Y) has coefficient sections of this bundle;
they have no common zero since X,Y have no common zero on C. The latter
follows by substituting X=Y=0 in F_C, giving Z4, so no projective point.
[pcc:unverified]

Set S=(5,6). The line v-u-1 meets E at the parent residual Q,Q' and S:
substitution gives -(u-5)(u2+4u+6). The vertical u-3 cuts out the two
rational branch values (3,5),(3,2), and has pole order2 at O. Hence
div((u-3)(v-u-1))=B+S-5O, and the norm bundle is O_E(5O-S).
Riemann--Roch on E and pole orders2,3,4,5 give the basis
1,u,v,u2,uv of H0(5O). Evaluation at S yields1,5,6,4,2; its kernel
has the basis s=(u-5,v-6,u2-4,uv-2). This establishes the exact small
coefficient space before solving, independently of rational fibre counts.
[pcc:unverified]

Native A certifies, as full homogeneous degree24 expressions modulo F_C,

    Sum(i=0..4) S_i(u,v)*(Y/X)^i = 0,
    S_i = dot(c_i,s),
    c_0=(1,3,0,0),
    c_1=(6,4,3,1),
    c_2=(0,6,3,4),
    c_3=(5,6,3,2),
    c_4=(0,4,3,1).

The entire94-by20 matrix has rank19, one-dimensional kernel, complete
zero row residuals and the printed full F_C quotient. The nonzero last
coefficient vector is nonzero as a section, because the basis is linearly
independent; f^* is an injection of function fields. A real c_0[0]+1
mutation gives the recorded nonzero remainder, and pristine restoration
gives the identical original polynomial and zero remainder.
[pcc:unverified]

The degree4 map y=Y/X cannot factor through f with degree1 E->P1,
since a degree1 map of smooth projective curves is an isomorphism and
their genera differ. A degree2 intermediate field is excluded by the
already qualified geometric primitivity. Therefore k(C)=k(E)(y) with
extension degree4. The displayed nonzero quartic is its minimal polynomial
up to multiplication by a nonzero target rational function. Its splitting
field is the actual arithmetic/geometric S4 Galois closure established in
the parent. This proves the polynomial represents the genuine cover and
supplies actual coefficients for the next resolvents. Singular affine
charts do not replace their proper smooth normalizations. [pcc:unverified]

Custody: original4673800B ELF and0555 qualifier remain whole in
/dev/shm/ct24-resolvents-firstA-native. Initial input content-wrapper
failure for one public keyring fetch was preserved; exact expected-blob
fallback returned full equal473B, completing all33 whole input/lock
readbacks before any strict call. No keyring, frozen input or predicate
changed. The native certificate does not prove the norm/Pic/Jacobian
foundations in Lean. [pcc:unverified]

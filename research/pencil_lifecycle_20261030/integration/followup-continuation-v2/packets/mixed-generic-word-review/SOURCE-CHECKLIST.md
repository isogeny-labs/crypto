# Source mapping checklist prepared before word proof inspection

Binding retained projective_law_gate.rs SHA5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0, generic232-292, outer828-878. All operations below are source Field calls; field source n1/tablesNone and canonical bounds are necessary for UInt64 interpretation.

z2=sq(z),z4=sq(z2),z5=mul(z4,z); az2=sq(az),az4=sq(az2),az3=mul(az2,az),az5=mul(az4,az). No pow-loop call occurs.
K=mul(add(add(mul(q,mul(z2,mul(ax,az2))),mul(sq(ax),z4)),mul(r,az4)),aw).
A=sub(mul(ay,mul(z5,w)),mul(add(mul(s,mul(az3,mul(ax,z2))),mul(t,az5)),aw)).
B=add(mul(s,mul(K,az)),mul(A,q)); C analogous t,r.
L=mul(K,mul(az,w)); M=mul(K,mul(z,w)); L2=sq(L),L4=sq(L2),M2=sq(M).
q3=sub(sub(mul(ax,M2),mul(q,L2)),sq(A)).
r3=add(sub(mul(add(sub(mul(curve3,z4),r),sq(q)),L4),mul(A,mul(add(B,mul(s,mul(K,az))),L2))),mul(q3,mul(ax,M2))).
s3=sub(mul(A,q3),mul(B,L2)); t3=sub(mul(A,r3),mul(C,L4)).
Zout=mul(K,mul(z,mul(az,w))), Wout=literal1.

Generic first K term uses q*(z2*(ax*az2)); outer first K term uses q*((ax*z2)*az2). Different word trees must first decode canonically; field associativity then canonical equality proves same word. Generic K==0 yields None BEFORE payload construction; pure eager Option model is value-only if payload constructed first.

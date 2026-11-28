# Explicit elliptic pullback and a candidate genuine chained transfer

This is a new written derivation for the selected fixture, **not yet executed**
as a map test, compiled in Lean, externally validated or PCC signed. The
underlying bielliptic-cover construction is standard and existing repository
cover_transfer/bielliptic code was already reviewed; no novelty claim.
[pcc:unverified]

## Prime-field cover in explicit coordinates

For C/F3:y^2=x^6+x^5+x^2+1, put
u=x/(x+1), v=y/(x+1)^3. The inverse is x=u/(1-u), y=v/(1-u)^3.
Direct homogeneous substitution gives
C2:v^2=2u^6+u^2+1.
The rational involution x->x/(x-1) becomes u->-u; its two lifts give the
usual two elliptic quotients. [pcc:unverified]

The explicit degree2 map to
E/F3:Y^2=X^3+2X+1 is X=2u^2,Y=2v. Substitution verifies the equation.
E has6 affine points, all with nonzero ordinate, and the identity, hence
order7 and trace-3; the other elliptic Weil factor has trace1. These small
point facts are written expectations for the next native qualification,
not a newly executed ECDLP benchmark. [pcc:unverified]

The pullback of an affine elliptic point has two preimages described by
u^2=2X,v=2Y, with multiplicity when u=0. The preimage of O_E is the pair of
infinities on C2, which becomes the original hyperelliptic fibre x=-1=2.
For any Weierstrass point W(r), every hyperelliptic fibre is linearly
equivalent to2W(r), so this denominator contributes the zero class relative
to2W(r). This is the base-divisor argument, not an unexplained omission.
[pcc:unverified]

In the already audited monic odd chart t=1/(x-r), Z=y*t^3/s, define
A(t)=r*t+1, B(t)=(r+1)*t+1. Then an explicit pullback on affine E points is

```text
u_D(t) = monic(A(t)^2 - 2X*B(t)^2)
v_D(t) = (2Y/s)*B(t)^3 mod u_D(t).
O_E maps to identity.
```

The polynomial leading coefficient cannot vanish for a nonzero order7
point: that would put W(r) in the fibre, whose image has Y=0 and order2.
The class can include t=0, corresponding to an original point at infinity;
that is a valid finite point of this odd chart, not a reason to drop a target.
The standard push-pull relation is psi_*psi^*=[2], so the pullback is
injective on odd-order subgroups. A native test must still independently
check every point image, all49 addition pairs, all7 distinct outputs,
Frobenius-fixedness and the real scale/basepoint negative controls.
[pcc:unverified]

## Candidate next chain without scalar-label input

Use Richelot factorizations of C2 itself, keeping its leading coefficient2
in the third quadratic and the first two quadratics monic. This avoids
introducing an original-source infinity for the affine preimages of E points:
u=+/-sqrt(2X), v=2Y; all these lie over F9. A fixed canonical factor ordering
and all its exceptional cases must be frozen before any call.
[pcc:unverified]

For each preimage (u,v), the actual Richelot point correspondence gives a
binary quadratic A(z)=G0(u)H0(z)+G1(u)H1(z) and an ordinate numerator
N(z)=G0(u)H0(z)*(u-z)/v of degree at most3. In a target odd chart based at a
branch root w, homogenize at the full weights:

```text
u_target(t) = monic(t^2*A(w+1/t))
v_target(t) = t^3*N(w+1/t)/s_target mod u_target(t).
```

Use the full cubic numerator rather than its affine remainder. This retains
the correct ordinate at an original codomain infinity when A has degree1;
the homogeneous quadratic also records that infinity at t=0. If A(w)=0,
the target Weierstrass point becomes the odd-chart basepoint and the degree
drops appropriately. Validate every resulting Mumford pair rather than
assuming these exceptional limits work. Add the two image classes using
the actual generic Cantor law. The two elliptic-identity preimages are
hyperelliptic conjugates, whose correspondence image classes cancel.
[pcc:unverified]

F3^6 is a candidate sufficient arithmetic field: F9 source lifts and all
possible3+3/6 branch roots fit there. The monic-chart square root must be
actually found and checked. For a degree6 irreducible branch polynomial,
the odd six-cycle has nonsquare discriminant over F3; the norm of its
derivative differs by -1 and is square, predicting a square derivative in
F3^6. For degree3 roots the additional degree2 extension makes every nonzero
F27 coefficient square. This is a written field-of-definition argument;
failure must remain a negative result, not a silently enlarged field.
[pcc:unverified]

For a nonrational target basepoint w, use the full moving-basepoint
Frobenius correction. Over F3^6 the inverse-cube root is w^(3^5)=w^243,
so the pole-preimage formula must generalize the prior r^9 special case.
All7 outputs should be fixed as Jacobian classes by original F3-Frobenius.
The transfer function must take only the public E point and frozen curve
data, never the test's scalar label or a seven-element discrete-log lookup.
[pcc:unverified]

## Research and cost limits

This route could qualify an actual E->Jac(C)->Jac(C_new) chain over the prime
field, rather than another abstract torsion matrix. It supplies no map result
yet. The tiny order7 complete cohort is a correctness qualification; it is
not a cryptographic-size ECDLP result or evidence of an advantage over direct
elliptic rho/BSGS. Complete setup/field/chart/maps/destination solver/replay
and memory must be charged in a separately frozen matched comparison.
Actual Weil8, generic geometric formalization, unsigned standing, original
EKS-only workloads, paid STOP and custody requirements remain unchanged.
[pcc:unverified]

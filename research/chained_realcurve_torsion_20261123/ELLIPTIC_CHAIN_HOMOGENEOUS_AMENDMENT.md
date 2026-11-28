# Prospective homogeneous elliptic-to-Richelot chain

Written derivation only; no elliptic/chain map execution or signed verdict yet.
This supplements frozen ELLIPTIC_PULLBACK_CANDIDATE.md without editing it.
The actual Richelot B resultc656d1 supplies4 real affine degree-drop witnesses,
which must become mandatory controls in the next native map unit.
[pcc:unverified]

## Original-source infinities without changing the published codomains

For an elliptic point (X,Y) on E/F3:Y^2=X^3+2X+1, the two preimages on
C2:v^2=2u^6+u^2+1 have u=+/-sqrt(2X),v=2Y, all over F9. As original-source
homogeneous coordinates take a=u,b=1-u and v=y*b^3. When u=1, b=0 is a
genuine original point at infinity, not an excluded target. [pcc:unverified]

For each original monic quadratic G_i, evaluate its binary form
G_i^h(a,b)=a^2+g_i1*a*b+g_i0*b^2. Then use

```text
A(z)=G0^h(a,b)*H0(z)+G1^h(a,b)*H1(z)
N(z)=G0^h(a,b)*H0(z)*(a-b*z)/v.
```

For b nonzero these follow by cancelling all b powers in the affine formula;
they remain polynomial at b=0. The bivariate identity homogenizes to
sum_i G_i^h(a,b)*H_i(z)=-delta*(a-b*z)^2, giving the same curve-equation
identity on the relation A=0. Thus the previously published original-C
codomains can be reused directly, rather than replacing them by a different
C2-coordinate model. [pcc:unverified]

## Target infinities and the four observed degree drops

In an odd target chart t=1/(z-w), Z=y_target*t^3/s_target, use the full
weights2 and3 even when A has smaller affine degree:

```text
u(t)=monic(t^2*A(w+1/t))
v(t)=t^3*N(w+1/t)/s_target mod u(t).
```

For degree1 A the first polynomial has a t=0 factor representing the original
codomain infinity. Reducing N modulo A before this substitution loses the
leading cubic ordinate and makes the t=0 value vanish. Since the target odd
chart has nonzero h(0), that copied-rule mutant should fail the actual Mumford
validity check on all4 retained degree-drop witnesses. This prediction must
be tested, not installed as an unconditional rejection. [pcc:unverified]

Add the two resulting target classes by the inherited Cantor law. The
elliptic identity pulls back to a hyperelliptic conjugate pair on the source;
their two correspondence image classes cancel. This supplies the candidate
E->Jac(C)->Jac(C_new) map without scalar labels. Any zero relation, bad chart,
failed exact polynomial condition or inverse must be preserved as a failed
case rather than silently dropped. [pcc:unverified]

## Candidate native qualification

Use F3^6 as F27[b]/(b^2-2). Two is nonsquare in F27. Find actual target roots
and monic-chart square roots and check every value. For Frobenius use the
inverse cube w^243; retain moving-basepoint and pole corrections. Reuse the
audited generic monic-odd Cantor bodies, not an unvalidated even-degree group
law. The source field/setup is new; do not rerun the old4096 J8 population.
[pcc:unverified]

The qualification should enumerate all7 E(F3) points, independently implement
their curve/group law, check all49 addition pairs for the source pullback and
both smooth Richelot outputs, require7 distinct outputs in each Jacobian and
Frobenius-fixed classes, and compare every scalar multiple. Transfer functions
must accept only the point and frozen curve data; scalar labels belong solely
to the control driver. Source infinities, double preimages at u=0, the4
observed degree drops, omitted chart scaling and omitted basepoint corrections
are required controls. [pcc:unverified]

A separate cost protocol must charge complete field/kernel/chart setup,
point maps, destination solver, replay/inversion, memory and the strongest
matched direct elliptic method. A successful order7 map test would establish
only this finite geometric transfer, not a cryptographic-size solve or a
computational advantage. [pcc:unverified]

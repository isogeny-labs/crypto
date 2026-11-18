# The existing etale double cover gives a nonzero Cartier control

Derived structural check, not a measured Cartier matrix. It sharpens the
R60 proposed filter: on THIS source a zero joint kernel should trigger an
audit of completeness/operator/geometry, rather than be treated as a new
expected exclusion outcome.

Assume the retained proper smooth connected source C/F2 and free sigma
action from R27, with geometrically connected source and full constantsF2.
The R59 complete global-regular kernel has dimension1 on the UNPROJECTED
complete100-space, supplying a source-bound constant-field check under the
RR bridge. A projected sigma-fixed constant kernel alone would not suffice.
Let D=C/<sigma>, the proper genus31 quotient. Then C->D is a geometrically
connected nonconstant etale Z/2 torsor overF2, not the constant F4extension.

1. Its class in H1_et(D,F2) does not lie in the constant-cover subspace.
   The Artin-Schreier sequence gives a nonzero Frobenius-fixed vector in
   H1(D,O_D). The constant-cover case must explicitly be excluded here.
2. OverF2 Frobenius is linear. Cartier on H0(D,Omega1) is its Serre-adjoint,
   so F-I and C-I have the same singularity. Hence a nonzero Cartier-fixed
   holomorphic differential on D exists overF2.
3. Pullback under the separable etale cover is injective on differentials,
   commutes with Cartier and lands in the sigma-fixed forms on C. Thus
   ker(Cartier-I) intersect ker(sigma-I) on source holomorphic forms is
   NONZERO already overF2, and remains nonzero overF32.

This is a built-in positive control for the complete61-form/Cartier
construction. The useful result will be its ACTUAL dimension and source-bound
differential directions, followed by logarithmic function/AS reconstruction.
A nonzero direction alone is not an elliptic map or intended11factor.
The structural argument does not identify the differential vector; compute
it and verify its source regularity, Cartier identity and sigma identity.

Keep matrix conventions explicit. OverF32 Cartier acts on coefficients
by inverse Frobenius (c->c^16), while H1 Frobenius uses squaring. Do not
identify their matrices or iterate them as ordinary linear operators.
The native whole-field square-decomposition check and305binarysystem avoid
this shortcut. No cohomological or scheme-level claim here is yet a Lean
proof; generic matrix/recovery and forthcoming scalable coefficient lemmas
have their separate stated premises.

Primary sources checked2026-11-01:
- Stacks Project59.63, Artin-Schreier exact sequence and induced cohomology:
  https://stacks.math.columbia.edu/tag/0A3J
- Achter and Howe, Hasse-Witt and Cartier-Manin matrices: A warning and a
  request, sections1-2, adjoint/semilinear conventions:
  https://www.math.colostate.edu/~achter/math/hwcm0618.pdf
- R27/CM.md proper etale double quotient; R59complete globalregular control.

## Stronger operator check from the proper double cover
On all61 source holomorphic forms, invariant differentials descend through
the finite etale torsor, hence the sigma-fixed space has dimension31.
Therefore N=sigma-I has rank30, and fixed modulo image(N) has dimension1.
This is a predicted geometric invariant to CHECK on the complete native
basis; it is not an observed matrix rank yet.

Tensoring the AS cover extension by the quotient canonical bundle gives
the trace sequence on differentials. Its connecting functional ell on the
31 quotient forms pairs with the nonconstant AS cover class alpha. Thus
image(trace) is ker(ell), of dimension30. Since Frobenius(alpha)=alpha,
Serre adjointness implies ell(Cartier(omega))=ell(omega) overF2. Consequently
Cartier induces IDENTITY on the one-dimensional fixed-mod-trace quotient.

Native acceptance: verify N²=0,rankN30/fixed31; full fixed/image/kernel
certificates; recover the nonzero functional ell on fixed forms annihilating
all trace columns; verify ell*C_fixed=ell. This is an exact separator and
Cartier-consistency target derived from the existing cover geometry.
Do NOT infer a Cartier-fixed representative outside trace image from this
quotient action: a nontrivial Jordan block at eigenvalue1 can obstruct such
a representative. Compute actual representatives and obstructions instead.
The nonzero left eigenfunctional still forces C_fixed-I singular and hence
at least one actual nonzero Cartier-fixed vector in finite dimension.
This field/cohomology bridge is mathematical reasoning, not yet a Lean
scheme proof. The abstract eigenfunctional obstruction is suitable for a
small separate Lean lemma with its operator hypotheses explicit.

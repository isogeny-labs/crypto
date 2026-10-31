# R53 exact trace geometry and normalized Hecke construction

## Verified trace and discriminant calculation

`discrim53-02` is accepted: exit0, exact terminal marker, no native error
lines. It uses all528 retained normal multiplication columns, not a selected
sample. All32normal-generator traces agree with the independently reconstructed
degree16 companion traces and quadratic base trace. The complete1024-entry
identity is

\[
 M_0^t\operatorname{Gram}_B M_0=\operatorname{Gram}_{\rm power}.
\]

The actual source is the retained quadratic equation
\(V^2+z^2V+z+z^3=0\) and the retained monic degree16 lambda equation.
Its derivative is checked against \(A^3\Delta\), with
\(R=(1+z)V+z^3,\ b=zR(R+1),\ c=z(R+1),\ A=1+c\),
\(\Delta=b^3(b+cA^3)\). The full source embedding products agree across
the compact PID, multiplication-table and closure certificates. Changed Gram
and trace entries are detected.

All eight predictions matched, with

\[
 f_A=z^5+z^2+1,\qquad f_C=z^5+z^3+z^2+z+1.
\]

| Exact computed quantity | Factorization |
|---|---|
| \(\operatorname{Norm}(A)\) | \(f_A\) |
| \(\operatorname{Norm}(\Delta)\) | \(z^{11}(z+1)^{22}f_C\) |
| \(\operatorname{disc}(B)\) | \(z^{76}(z+1)^{24}f_C^{12}\) |
| \(\det M_0\) | \(z^{82}(z+1)^{164}f_A^{24}f_C^2\) |
| \(\det M_1\) | \(z^{60}(z+1)^{120}f_A^8\) |
| \(\det M_2\) | \(z^{60}(z+1)^{120}f_A^4\) |
| \(\det M_3\) | \(z^{60}(z+1)^{120}\) |

Both full discriminant identities pass:
\[
 \operatorname{disc}_{\rm power}
 =z^{64}\operatorname{Norm}(A)^{48}\operatorname{Norm}(\Delta)^{16}
 =(\det M_0)^2\operatorname{disc}(B).
\]

Consequently the index lengths376,220,200,180 now have their exact supports.
The second and third adjunctions remove lengths20and20 solely over the
degree5prime \(f_A\); the final defect is entirely at z0and1. The first
adjunction removes the fA exponent24to8, the fC exponent2to0, and the z/z+1
exponents82/164to60/120. These are measured module statements. They do not
identify the completed local subrings. Matching the proposed (7,11)cusp
model still requires actual branch valuations and residue-field descent.

The table-derived Gram certificate, independent source Gram, all31universal
companion traces, source equation, normal traces and four determinants are
retained in raw output. `discrim53-01` is rejected: missing matrix-helper
library and unallocated ideal entries caused cascading errors; it was stopped
with SIGTERM. Its handler returned0, but the strict error/marker gate rejected
it. Attempt02 corrects those implementation errors and passes.

## Construction stages

`alphaunit53-01` is accepted:exit0,exact marker,no native error lines. It
rechecks the saved torsion membership and coprime-kernel Bezout products,
including exact agreement of their psi7 and of the actual R52norm alpha with
the line cubic. Write the torsion row as

\[
 \psi_7=Gx+Ny+\alpha z_0,\qquad U\psi_3+V\psi_7=\Delta^{16}.
\]

The native certificate then verifies the full product

\[
 (G,N,\psi_3,\alpha)
 (Vx,Vy,U,Vz_0)^t=\Delta^{16}.
\]

It rejects a changed witness entry and computes dimension64for the quotient
by G,N,psi3. Thus the actual alpha is a unit on the generic registered chart,
with inverse numerator \(Vz_0\) and denominator \(\Delta^{16}\). The full
7.6MiB witness is retained in raw custody. This is over generic F2(b,c);
specializations require the registered coefficient denominators as well as
Delta to remain invertible. This composition avoids a new inversion search.

`tate53-03` is accepted:exit0,exact marker,no native error lines. It maps the
original marked origin through the accepted degree3quotient to (r,q),then
constructs

\[
 h=Ar+b,\quad s=(r^2+a_4+Aq)/h,\quad D_2=b+r+s^2+As,\quad u=h/D_2.
\]

All four units t,h,D2,u have explicit psi3Bezout products. The point equation
has a full source product. With

\[
 X=u^2x+r,\qquad Y=u^3y+su^2x+q,
 \qquad b'=D_2/u^2=h/u^3,\quad A'=A/u,\quad c'=1+A',
\]

the full substituted old-curve equation plus
\(u^6(y^2+A'xy+b'y+x^3+b'x^2)\) is its saved explicit psi3multiple.
A changed x coefficient has nonzero remainder. Thus the marked generic
quotient is now explicitly in the required Tate form. Specializations need
the registered t,h,D2,u and coefficient denominators. Target QR/F2 descent
and the actual level11 map are separate next checks.

Tate attempt01failed a source-ring indexed expression import. Attempt02
passed reduced identities but failed serialization by direct polynomial slash
division;it finished before a proposed SIGTERM. Attempt03uses explicit ideal
lifts and verifies their full products. Every partial and rejected attempt is
retained. `PACKET53.sing` is frozen to evaluate all5target equations in the
complete rank64source algebra;its outcome is pending.

`lean/Transfer53.lean` supplies the determinant-change and composed unit
implications. All3declarations compile/audit with zero failing/trusted in transfer-lean53-02;
first audit launch failure is preserved. Native semantic binding is recorded
separately. No actual completed local-ring, target QR/F2 map or subgroup
action is asserted here before its respective check.

## Sources and premises

Retained R50 normal RR algebra, R51 compact PID/line/collision/isogeny
certificates, and R52 complete nu-adjunction certificates are the source
inputs, pinned in `donors.json`. Established model conventions were reviewed
in [Sage's isogeny documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html)
and the function-field/proper-curve correspondence in
[Stacks 0BXX](https://stacks.math.columbia.edu/tag/0BXX). These sources inform
the generic coordinate normalization and its proper-model obligations; they
do not provide the specific native arrays. No novelty claim is made for
trace pairings, tower discriminants or Tate normalization.

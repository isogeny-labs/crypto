# R55: complete affine A-zero quotient and degree-80 binary coordinate

## Derived and native-checked construction

Let B be the retained normal rank-32 RR algebra over F2[z], with its complete
528 multiplication columns. The actual base coordinate satisfies
V²+z²V+z+z³=0. Set R=(1+z)V+z³, c=z(R+1), A=1+c and
f_A=z⁵+z²+1. R55 rechecks the global source embedding and actual unit, then
builds multiplication by V from the full source table. Entrywise scalar
construction avoids native module-concatenation overloads.

The actual32x32 multiplication matrix M_A has conjugate
C=M_A+(z³+z⁴)I and both full products are

    M_A C=C M_A=f_A I32.

Thus f_A B lies in A B. Reducing the quotient by f_A introduces no extra
constraint: B/A B=B/(A B+f_A B). A changed C is detected. This is the full
module identity, stronger than the earlier norm/determinant prediction.

## Complete global quotient on this affine chart

The four R54 degree20point moduli N_i are pairwise comaximal. Their product
N has degree80 and gcd(N,N')=1. Full cofactor/Bezout products give four CRT
idempotents,all16orthogonality products,all16residue labels and partition1.
The actual source coordinate z=rho(w) and all32basis functions are assembled
using these idempotents. Every528source multiplication product and the unit
identity pass moduloN;f_A(rho)=0.

The complete source B/f_A B has160binary coordinates z^k e_j,k0..4,j1..32.
Expand M_A to Abar160x160 and the coordinate evaluation to E80x160. Both
ranks are80. Full arrays G160x80,H160x160 certify

    E Abar=0, E G=I80, Abar H=I160-G E.

These prove surjectivity and kerE=imageAbar,for all classes. Together with the
scalar identity they give the explicit F2-algebra identification

    B/A B ≅ F2[w]/(N) ≅ product(i=1..4) F2[w]/(N_i).

The multiplication table checks make this an algebra map;the recovery
products prove it is a complete quotient identification. In particular the
four points exhaust the A=0scheme in this normal affine chart. It is reduced,
with four degree20closed points,each local length1,and total binary length80.
No extra affine component or nilpotent thickening remains. The actual
annihilator is f_A:the nonzero quotient is a module over the degree5field
already certified in R54,so no proper nonzero divisor of f_A can annihilate it.

This completes an affine zero-scheme calculation. The complete projective
pole divisor,branches at infinity and modular-factor action retain their own
geometry checks. It is not a claim of full quotient saturation for a separate
incidence embedding or of a proper polarized embedding.

## Provenance and failures

quotient55-01 is rejected. Its scalar/matrix expression produced R32x64 and
failed the base identity;undefined downstream objects and a late marker are
not accepted. diag55-01 executes the dimension probes,not a quotient claim.
quotient55-02 builds scalar combinations entrywise and passes all original
source/quotient predictions,with exit0,exact marker and no native error lines.
Non-error ring/link redefinitions are retained. Same limits:4GiBvirtual,
1200seconds,one Singular/FLINT thread,seed55. Python/shell handle metadata only.

The serialized checker and Lean implications are separately frozen stages;
their actual terminal receipts,not this planned paragraph,decide acceptance.
Actual normal-source geometry and field arrays remain native premises for Lean.
No official hypothesis promotion or independent scientific review is claimed.

## Sources and next leads

Retained R50/R51 full normal RR algebra,R52 source PID embedding,R53 trace
geometry and R54 complete point fields are the mathematical inputs. Reviewed
[Stacks00DT](https://stacks.math.columbia.edu/tag/00DT) for comaximal CRT and
explicit idempotent recovery;native linalg.lib for rank/module lifts. The
method uses full matrix products in addition to dimensions.

The next geometric lead is now specific:find the complete pole divisor of A
and bind this certified affine zero scheme to its global divisor. The degree80
coordinate also gives a compact complete evaluator/control for target modular
identities and reverse divisors. Preserve compressed global target checking,
actual degree8/128level11map/intended11action,proper polarization,saturation,
node187,arbitrary reverseclasses and larger112/991 as separate active lanes.

# A guarded coefficient circuit

This is a prospective written construction, not a formal compiler theorem or
a coefficient-discovery algorithm. It continues section-coeff-v1 without
changing that study's source or evidence. [pcc:unverified]

## Domain and rational torsion

Use exactly the original fixed Fp/Fp2 contracts in section-coeff-v1/DERIVATION.md.
In addition, require A+2 a nonzero square and A-2 a nonsquare in Fp, with -1 a
square. These hypotheses are to be checked for p101/A95 and Curve25519/A486662.
They are not asserted for arbitrary Montgomery curves or their twists.

Put C=X^3+A X^2+X and

    J_A=(X-1) ((X^2-1)^2-4C).

For a rational curve point P with nonzero ordinate, [8]P=O exactly when
J_A(x(P))=0. Here is the written argument. The discriminant (A+2)(A-2) is
nonsquare, so (0,0) is the only nonzero rational 2-torsion point. A rational
point of order4 doubles to it. Montgomery doubling has numerator (X^2-1)^2
and denominator4C; hence its x is 1 or -1. At -1 the ordinate square is
A-2 (because -1 is square); this case is excluded. At 1 the point has order4.
A rational point of order8 doubles to a point with x=1, equivalently the
quartic factor vanishes. Conversely that factor, with nonzero ordinate,
implies the double has x=1 and order4. This covers all nonidentity points
killed by8 after excluding order2 via the nonzero ordinate hypothesis.

The old predicate proves rational, nonzero ordinates from squarefree original
membership and Fp-valued norm sections. Only on that support may its D3 unit
guard be replaced by the J_A unit guard. Over the algebraic closure the two
guards need not be equivalent. There is no new torsion or division-polynomial
algorithm in this observation. [pcc:unverified]

## Five coefficients, two inverse inputs, and acyclic wires

The public compiler receives Spec,R,Q=[8]R and limits. It receives no section
coefficients or labels. Normalize finite R at c4=1, or R=O at c5=1, leaving
exactly five section inputs. Construct the same monic sextic S by the original
norm formulas. Its coefficients have degree at most2 in those inputs.

Let r0=-A/2 and S=(X-r0)B+S(r0). Introduce beta with beta*S(r0)=1 and use
lambda=-(A/2)*beta*B modulo S. This retains the complete map chart guard;
it never divides by an unknown value without a constraint.

Let M_J be multiplication by J_A on Fp[X]/S in the monomial basis. Introduce
eta with eta*det(M_J)=1. The determinant equals Res(S,J_A), so is nonzero
exactly when J_A is a unit. Compute it by cached cofactor expansion without
pivoting or division. Its polynomial degree in the six coefficients of monic
S is at most5, and therefore at most10 in the section inputs. This is a
written resultant degree bound, not a solving-degree bound. [pcc:unverified]

Compile tau=4*T_q(lambda)^2-2 and the existing homothety/chirp membership
recurrence in the finite-free algebra with v^2-tau*v+1=0. All scalar inverses
inside that recurrence are public constants. Keep every Fp coordinate of the
final normalized H expression constrained to zero, including its redundant
outer and Fp2 coordinates. Structural constant folding may remove identically
zero gates; it cannot inspect supplied coefficient values.

Each arithmetic gate introduces a wire with z-a-b=0, z+a=0 or z-a*b=0.
Constants and inputs have explicit roles. Guards and output constraints are
included. A satisfying canonical Fp assignment projects to five coefficients;
induction on the topological gate order forces every arithmetic wire. The two
inverse equations force beta and eta uniquely when the denominators are
nonzero. Conversely any coefficient tuple passing the reduced predicate has
this extension. This argument is written and to be checked natively against
the unchanged numeric predicate; no independently checked compiler theorem
is claimed. Original indexed-root extraction/group replay remains mandatory.

Seven independent semantic/guard inputs does not mean seven total variables.
Quadratic gate equations do not imply quadratic solving degree. Extension
field roots, field filtering, setup, root discovery, useful yield, selection,
rank, memory and original recovery retain their existing obligations.

## Sources checked

Repository: section-coeff-v1/{DERIVATION.md,NEXT.md}, the frozen numeric
tools/torus-quotient-query/src/arithmetic.rs and query.rs, the radical predicate
and original indexed extractor. These supply the contracts and comparator.

Primary literature read: Costello--Smith, *Montgomery curves and their
arithmetic*, sections2.2/2.3 and3.3, https://arxiv.org/html/1703.01863v1 ;
EFD Montgomery XZ formulas, https://hyperelliptic.org/EFD/g1p/auto-montgom-xz.html ;
Macaulay2 determinant strategy documentation (cached cofactor expansion and
ring/division caveats),
https://www.macaulay2.com/doc/Macaulay2-1.26.06/share/doc/Macaulay2/Macaulay2Doc/html/_determinant_lp..._cm__Strategy_eq_gt..._rp.html .
Use these classical methods without a novelty claim. [pcc:unverified]

Sources checked (KB): no KB tools are loaded and scripts/research-kb status
reported no executable. ecdb's maintained read client refused missing token/
AWS access before a query; no credential was printed or exported. Repository
fixed parameters are the fallback, not a new certificate. Two distinct web
queries covered rational torsion/low-order coordinates; two covered quadratic
circuit encodings/division-free determinants. Survey PDF and circuit-language
documentation fetches failed. An attempted arXiv ID resolved to an unrelated
Kahler-geometry paper and was excluded. No novelty clearance is asserted.


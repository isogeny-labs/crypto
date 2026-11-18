# Next exact binary-field reduction: squarefree and smaller repeated factors

This is a proved algebraic reduction, not an executed fourth experiment.
Let H=X^5+tX^4+aX^3+bX^2+cX+d over any finite characteristic-two field.
Put A=b+t*a, B=d+t*c and

    Delta = B^2 + A*B*a + A^2*c.

As derived in C103, H' = X^4+aX^2+c and H+(X+t)H'=A*X^2+B.
If A=0,B!=0, gcd(H,H')=1 and Delta=B^2!=0. If A!=0, the unique root of
the residual is r=sqrt(B/A), and H'(r)=Delta/A^2. Therefore H is squarefree
iff Delta!=0. Delta is evaluated without inversion, using linear squarings
and three field multiplications. Its exact cost must be measured.

All repeated branches are retained:

1. If A=B=0, H=(X+t)Q^2 with Q=X^2+sqrt(a)X+sqrt(c). Use the established
   guarded quadratic splitting test, including a=0.
2. If Delta=0 and (A,B)!=(0,0), necessarily A!=0. Set u=B/A. Exact division
   gives H=(X^2+u)J where

       J=X^3+tX^2+(a+u)X+(b+t*u).

   Delta=0 implies c=u^2+a*u, and B=A*u implies d=u*(b+t*u), proving every
   coefficient identity. X^2+u always splits over the perfect finite field
   with one repeated root. Thus H splits iff the smaller cubic J splits,
   with arbitrary remaining multiplicities.
3. If Delta!=0, H is squarefree. For field size q, full splitting is exactly
   X^q-X mod H=0; no extra nilpotency squarings are required on this branch.

This removes inversion from the generic squarefree decision. Inversions are
confined to the precisely identified repeated cubic branch. It addresses
the earlier inversion-cost ambiguity, but a symbolic circuit containing all
branches can still cost more than the old predicate. Test native conditional
dispatch and any lazy guarded symbolic implementation separately. A generic
branch cannot replace exhaustive repeated-branch coverage in all-roots work.

Next bounded experiment: independent exhaustive monic quintics over small
binary fields, literal factor-division/root-multiplicity oracle and all
Delta branches; then frozen GF(2^83)/ordinary-binary/Koblitz-compatible field
fixtures with known repeated and squarefree factors. Charge inversions,
preprocessing, factor recovery and serialized output, not just Delta. This
derivation does not transfer unchanged to odd characteristic and establishes
no novelty or qualified gain over genuine F4/F5/F6.

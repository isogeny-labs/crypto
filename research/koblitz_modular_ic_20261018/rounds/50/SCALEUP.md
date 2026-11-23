# Next larger carrier: n11 via the marked23 base

R42 already supplies the n11 trace+1/order991 arithmetic control. The next
geometric base is the published X1(23) model, with explicit marked elliptic
curve and point23, from Sutherland/vanHoeij. The raw source is saved and
hashed in scaleup-reference.json. Its polynomial is monic degree7 in y;
its total x/y bidegrees are7/7. Primary index and exact source were read:
https://math.mit.edu/~drew/X1_optcurves.html and
https://math.mit.edu/~drew/X1/X1opt23.txt .

First native stage: reduce the exact polynomial modulo2, retain all factors,
and certify its generic degree7 prime over F2(x), rather than infer good
reduction from its name. Implement field arithmetic by explicit degree7
reduction and checked Euclidean inverses over F2(x), reusing R50's supported
coefficient-field arithmetic. Evaluate the supplied r,s,E,P formulas,
check the Tate discriminant and all coordinate denominators, and prove
[23]P=O in that actual function field. Exact curve residuals accompany
every addition, with known Tate2P/3P and corrupted-point controls. Save
field arrays and every accepted scalar step before proceeding.

There are simple inverse coordinates to check, in characteristic2:

    r=(x²+x+y+1)/(x²+xy),  s=(x+y+1)/x;
    b=r*s*(r+1),  c=s*(r+1);
    r=b/c,  s=c/(r+1);
    x=(r+s)/(r*s+1),  y=(s+1)*x+1.

These must be verified as rational function identities with their actual
nonzero denominators. They give a concrete route to a Tate-model binding.
The next extension then adjoins the universal QR7 incidence data to the
marked23 base, giving a level161 construction. The degree16 QR extension
and degree7 base suggest generic degree112 over F2(x); native complete
field/component and forward/reverse certificates must decide that value.
The known n5 construction remains the regression control for the same
universal QR formulas. No native X1(23) validation has run in this round.

For the independent level77 check, R29 MODULI_BRIDGE.md already retains the
Sutherland model and its raw hash, plus the irreducible cubic adjoining a
QR generator and the CRT56/22 identities. Reuse those bytes. A full-level
model must be compared after that degree3 extension, since the current
source is the diamond23 quotient, not the full X1(77) field. The remaining
check is explicit rational-function evaluation of the published elliptic
parameters, CRT and Tate normalization on that extension.

## Actual marked23 result
marked23-50-02 passes the degree7 prime,all coordinate inverses,nonzero
Tate discriminant,2P/3P and exact order23 via prefixes1,2,5,11,23.
marked23-50-01 fails a reserved local identifier and remains rejected.
The repaired source and all actual field/inverse/point arrays are retained.
Next apply only universal QR7 equations S3-S5 to this accepted base.

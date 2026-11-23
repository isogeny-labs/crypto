# Actual CM19 quartic automorphisms and elliptic quotients: A

ACTIVE-only567e3bd5 and draft5026 precede this implementation. Fixed
actual C7 source0b9b506b/result89d7b471 and mixed-H classificationb2ce1276
are immutable inputs. Parent5019's approved artifact migration remains open;
this new curve-map task supplies no policy/CI workaround. No new native
curve-map call has run. All claims unsigned. [pcc:unverified]

## Complete rational curve automorphisms, not a random matrix search

Fix C7:(Z^2+6X^2+3XY+Y^2)^2=Y(X^3+2XY^2+6Y^3).
Parse the original17 F7 curve points from the full3373B frozen F7 native
output. Normalize projectively, check exact equation/uniqueness and sort;
do not replay its old source/extension-field census or integral H search.
Select the first lexicographic projective4-frame with no3 collinear.
Enumerate all57120 ordered distinct image4-tuples from those17 points.
[pcc:unverified]

For a frame p0,p1,p2,p3, let A have the first3 columns and
u=A^-1 p3. Its canonical frame matrix is A diag(u), with all u_i nonzero.
For each general image frame construct the unique projective M, normalize
its first nonzero coefficient to1 and check the four actual images.
Reject singular/non-general images, then reject any M failing to permute
the complete17-point set. For every survivor compare all15 homogeneous
quartic coefficients of F(Mv) with lambda F(v), lambda!=0. Record every
accepted matrix, lambda, point permutation, complete coefficient vector,
matrix order and full group closure/inverse table. [pcc:unverified]

Completeness is written: every F7 automorphism of a smooth nonhyperelliptic
genus3 curve acts on its canonical plane model by PGL3(F7), hence maps
the fixed frame to one of these image frames. No projective automorphism
is lost by the point-set prefilter. The explicit coefficient identities
remain mandatory; agreement on a few affine points is not substituted.
There are17 distinct points, exceeding the16-intersection Bezout bound,
which supplies an additional consistency explanation for the prefilter.
This does not enumerate extension-only automorphisms. [pcc:unverified]

Emit every frame decision in a complete2-bit packed codec (four decisions
per byte, hex printed once):0 non-general,1 point-set rejection,
2 coefficient rejection,3 accepted. Loop order is increasing source point
indices, with unequal indices. Exactly57120 decisions/14280 bytes must
be retained, plus full accepted maps. This preserves the whole finite
decision population under the existing64KiB raw policy. No output
truncation, hash-only evidence or modified storage threshold. [pcc:unverified]

Mandatory actual positives are identity and deck involution diag(1,1,-1).
Reject an actual singular matrix and diag(1,1,2) by full coefficient/point
checks; restoration passes. Do not preselect an automorphism count.
Select the first lexicographic order3 automorphism if present; if absent,
preserve the negative rather than alter the curve or population.
[pcc:unverified]

## Actual degree2 elliptic covers

The original quotient q0 maps (X,Y,Z), Y!=0, to

```text
xE=X/Y,
yE=(Z^2+6X^2+3XY+Y^2)/Y^2 on E7:yE^2=xE^3+2xE+6.
Y=0 maps to the elliptic point at infinity.
```

For every discovered involution conjugate to the deck map, choose the
first lexicographic automorphism A with A^-1 deck A equal to it and use
the actual morphism q0 o A. Check full homogeneous pullback identities,
all17 point images, involution invariance and all fibres over the fixed
11-point E7 fixture, including zero-preimage and actual infinity fibres.
No scalar labels or expected image table enters a cover function.
The original infinity points[1:0:+/-1] provide positive pole handling.
Native E7 group law is checked on all121 addition pairs/1331 associative
triples and all11 order/scalar controls for this new map fixture; no old
whole-curve counter or qualifier driver is executed. [pcc:unverified]

## Order3 quotient as a genuine elliptic curve map

Lift the first order3 projective M to N^3=I over F7 and find its
actual eigenlines. Require the three simple eigenvalues, with exactly two
eigenpoints on C7. Choose the off-curve eigenpoint as axis X, and order
the other axes to get diag(1,2,4). This yields the exact normalized form

```text
A X^4+B X^2 YZ+C X Y^3+D X Z^3+E Y^2 Z^2 =0,
A=1; C,D,E nonzero.
```

Compare all15 coefficients and require every other monomial zero. On X!=0,
u=YZ/X^2, v=Y^3/X^3 give the actual quotient

```text
C v^2+(A+B u+E u^2)v+D u^3=0,
w=2C v+(A+B u+E u^2),
w^2=f(u)=(A+B u+E u^2)^2-4CD u^3.
```

Require f squarefree. The two fixed points on X=0 map respectively to
the two actual quartic infinity points with w/u^2=-E and+E. Verify
every source point image, order3 orbit invariance and all target fibres,
retaining the two ramified points and any empty fibres. Count only this
new quotient target's points, not the old C7 source census. [pcc:unverified]

Construct an explicit rational elliptic model conversion, rather than
infer an isomorphism from matching point counts. For f=a u4+b u3+c u2+d u+e
with a=s^2 choose s=E. Put t=b/(2s), r=(c-t^2)/(2s),
L=d-2tr, M=e-r^2, Q=s u^2+t u+r, K=2s. Then

```text
x=K*(w+Q), y=K^2*u*(w+Q)
y^2+a1*x*y+a3*y=x^3+a2*x^2+a4*x,
(a1,a2,a3,a4)=(2t,-2rK,LK,-MK^2).
```

Complete the square and remove the x2 term, then search all6 nonzero
F7 scalings for an explicit short-Weierstrass isomorphism to E7.
Require a whole polynomial function-field identity modulo w^2-f(u),
not only F7 point agreement. The actual inverse uses
u=y/(Kx), w=x/K-Q(u) when x!=0. Handle x=0 and both infinities
explicitly: infinity+s maps to O; infinity-s to(0,-a3); finite
w=-Q maps to(0,0) with u=-M/L. Preserve a missing/invalid model
or exceptional chart as a negative, no fabricated total map.
[pcc:unverified]

Check every actual quartic target point, every E7 point and both complete
roundtrips; print all model coefficients, scalings, group points and
source/quotient/E images. Mutate the target constant by1, require failure
of the actual function-field identity and at least one real image equation,
then restore pristine identities/images. The quotient map itself consumes
only points/coefficient data, never target scalar labels. [pcc:unverified]

## Scope, scientific standing and execution

For each separable degree2/3 cover, the standard Jacobian
pushforward-pullback=[degree] makes its pullback injective on11-torsion.
That written functorial argument is not a full constructed Jacobian group
or a native generic Mathlib theorem. Actual genus3 divisor arithmetic,
CM19 coordinate alpha/nontrivial S correspondence/target maps and full
strongest-reference cold ECDLP costs remain open. No exponent gain is
inferred from the larger ambient Jacobian. [pcc:unverified]

Publish/full-readback every source/protocol/lock before first strict format,
compiler or native phase. Rust1.99.1 edition2021, strict -Dwarnings,
overflow/debug assertions, unchanged2GiB/120s/one worker. Copied source,
immutable qualifier and source/tool/runtime/pre/post guards. Preserve all
first failures and new-version repairs with unchanged scope/limits. Python
only file/Git/hash/JSON custody. Complete small raw under actual current
artifact policy, checked source-linked report/diagram/PDF, original ELF
approved custody pending. No old census replay or paid/alternative backend.
[pcc:unverified]

Sources checked (KB): unavailable; exact qualified source and primary
Ritzenthaler2010 canonical/CM19 context reviewed. Canonical embedding,
finite-field projective frames, smooth-curve birational extension and
Jacobian functoriality are explicit written dependencies, not novelty or
absence claims. Existing owners/paid STOP/vault lock/formal/custody/CI/merge
requirements persist. [pcc:unverified]

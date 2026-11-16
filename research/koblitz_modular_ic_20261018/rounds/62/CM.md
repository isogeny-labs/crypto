# R62 complete field square operator and direct generator route

## Exact source construction
The retained R61 function field is presented over F2(u) by two polynomials
in lambda and v. The first is equivalent to v²+v=u³+u; the second is monic
of degree16 in lambda. All32 canonical monomials lambda^i v^j,0<=i<16,
0<=j<2, are standard monomials in the actual quotient.

Accepted matrix62-01 constructs the complete32x32 matrix S whose jth
column is the source coordinates of the jth monomial squared. Both matrix
inverse products are exactly I32, and each column is rebound to the full
source field. All entries are checked in F2(u). A changed entry fails the
inverse certificate. This is a field-basis operator, without an integral
order or global divisor assertion.

Accepted split62-02 decomposes each rational coefficient a=N/D by the
complete polynomial identity ND=E²+uO², hence a=(E/D)²+u(O/D)². Combining
these pairs with S inverse gives f=g²+u h². It checks all435 named functions:
32 monomials,3 controls,324 actual R61 cubic values and76 actual mixed
values. All13920 rational coefficient identities and435 whole-source
identities pass. The17 zero odd outputs comprise8 basis,2 controls,5 cubic
and2 mixed inputs. Lists retain every zero component and all435 slots.
For1,u,u² the exact pairs are(1,0),(0,1),(u,0); changing the root for u
fails the source identity.

Accepted check62-01 reloads saved roots and the source matrix. It checks
every source equation and input against R61, all435 identities, all32
squared columns, both inverse products and the changed matrix/root controls.
This is a fresh whole-source replay, not a producer rerun.

## Independent generator construction
Accepted direct62-01 reads no squared-basis matrix or inverse. It starts
from the actual pair(v,u+1) for v, decomposes the coefficients of the lambda
polynomial and groups lambda parity to obtain

    P(lambda)=a²+u b²+lambda c²+u lambda d².
    q=P'(lambda)=c²+u d².
    lambda=((ac+u bd)/q)²+u((ad+bc)/q)².

The full source grouping, derivative identity and actual inverse of q are
checked. Pair arithmetic builds all32 monomial pairs and evaluates all435
functions. Both resulting roots match the matrix method for every input:
870 root comparisons and435 whole-source identities pass, with10012
rational scalar splits and the changed-root control. The rational scalar
parity helper is shared between the methods; the generator construction,
monomial-pair arithmetic and evaluation route differ.

No timing comparison was frozen. Native wall receipts are custody data.
The matrix route and direct route have the same complete function cohort;
their coefficient-split counts arise from different representations.

## Formal and geometric boundaries
PROOF_MAP.md and the terminal audit in results.json specify formal acceptance.
The field implications require characteristic two and, for uniqueness or the
square criterion, nonsquare-u. Cyclic5 projector statements apply to general
endomorphism rings and do not establish a geometric T action or nonzero
primitive block. The source model, separating parameter, normal/maximal
order, all local branches and the complete holomorphic space need their
own bindings before this field operator is called a global Cartier matrix.
Likewise an actual supported-unit square root needs its complete divisor
before entering a saturated unit lattice.

## Failures and reproducibility
split62-01 is rejected: unsupported ideal equality produced a native error
before a late marker. All its numeric lines remain unaccepted. Repair02
checks generator counts and each source polynomial separately. The first
Lean compile is rejected on a lemma-name and numeral-cast issue. Every
attempt and its historical source bytes are retained in results.json and
raw custody. An early raw-index mistake was corrected without deleting
bytes or rewriting Git history. No accepted producer was repeated to publish.

Native execution uses the original manual local authorization: seed62,
1200seconds,4GiB virtual,one Singular/FLINT thread and one mathematical job
at a time. Lean uses the pinned4.34.1/mathlib d13f23b cache,-j1 -M4096,
CPU0,1200seconds separately per compile/audit and no virtual cap.

## Next construction
GENERATOR-LEAD62.md derives the direct formula; SCALE112-DESIGN62.md binds
its next use to the retained marked23 degree112 source, with selected
order991-factor action as a separate obligation. Complete normal-order and
infinity geometry for the61 holomorphic columns, supported-unit saturation,
actual T/F2/Hecke/order11 action and the other original research lanes remain
active. These accepted field identities supply reusable inputs to them.

References checked: R61/SQUARE-DECOMPOSITION61.md and
DIFFERENTIAL-DESIGN61.md; R51/BASE23-STRUCT-PROTOCOL.md; R52/CM.md;
https://www.singular.uni-kl.de/index.php/singular.pdf for native coefficient
and field operations. The algebraic formula is a direct derivation, with
no novelty claim.

## Complete square subspaces
Accepted kernel62-01 retains every one of the76mixed and324cubic inputs.
Checked rational denominator clearing yields1328binary rows for mixed
and1377for cubic;every cleared polynomial is reconstructed completely.
Mixed odd rank66 leaves square dimension10; cubic odd rank196 leaves
square dimension128. Kernel entries are explicitly0/1,not F2(u) solutions.
Every one of the138 resulting roots is checked against its actual full
source-function combination;corrupted kernels and all changed roots fail.
This tests full F2 combinations,not just individual zero-odd columns.
The ten mixed roots include seven with lambda in their canonical expressions.
Given the retained proper-source/RR identification and even pole caps,
squaring halves the mixed caps34,12,20,2 to17,6,10,1. Actual involution,
complete divisor/normality binding and target-factor transport are next.

# R26 prospective exact-computation protocol, 2026-10-28 UTC

Continue the existing user-authorized local fixed-model lane under4GiB virtual
memory/1200s per native invocation; no new EKS job, Diem, waiting-freeze,
fifth Conductor claim, merge or auto-merge. R22 recovery d=1 is complete.
Preserve R24/R25 archives and all source pins. Conductor clears the study;
the shared daily log is held by T682, so use this round's SESSION.md for now.

Sources before choosing method: R15 CM (primitive diamond sign), R16 CM
(same explicit involution and moduli stabilizer), R18 CM (diamond2 fifth
iterate and joint residual system), R22 CM (base charts and tower), R24
source charts/series, R25 full quotient/invariant identities. The character
sign is already a written R15 result; the new task explicitly connects it
to R25's quotient and tests the correct Prym direction. Primary references
read: Best et al., Computing classical modular forms, section4 (diamond
nebentypus eigenspaces), https://jvoight.github.io/articles/cmfs-mf-101222-scrubbed.pdf;
Molin-Page theorem37, https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf;
Stein modular-symbol diamond operators, https://wstein.org/books/modform/modform/modular_symbols.html.
No novelty, speed or independent-review claim. Lead-atlas/script and generic
experiment-protocol checker are absent in the available sparse sibling;
use the inherited custom exact-computation record, no generic lint pass.

CHAR26: native exact negation on Tate y^2+Axy+by=x^3+bx^2 and line
lambda*x+nu; double negation; CRT delta34 (1mod11,-1mod7), delta^2=1mod77.
Enumerate all60 units with chi exponent=eta11 exponent+5 for nonresidue mod7.
Require6 in every character fiber and delta exponent5. Test all four
primitive coefficient exponents1,3,7,9; generator2 fifth power gives -1mod11.
For subgroup orders2,5,10 require all corresponding character norm sums0;
trivial subgroup1 gives1. The desired factor's sign/annihilation follows
under the retained modular/newform and specialization bridges, not from an
uncomputed actual Jacobian matrix. Negative control changes actual delta34
to identity1 and requires the unchanged detector to reject even with exit0
and a late marker. Formalize finite character arithmetic and conditional
norm/Prym recovery and general quadratic function norm in Lean.

CD26: at C0=(u,v)=(1,0) and D0=(1,1), put t=u+1 and
P=t^2+t^3+t^4+t^6+t^8+t^12, v=branch+P+t^16*q.
Exact base relation is q+1+t^8+t^16*q^2=0, residueq1.
Set h*(1+t)=1. Retain original rn=(u+1)v+1, cn=rn+u^3,
b=rn*cn/u^7,c=cn/u^4. Derive exact rational tower coefficients
Delta=b^3*(b+c*(1+c)^3), C=b^4*(b^2+(1+c)^2*b+c*(1+c)^4).
Test whether their cleared numerators have exact factors t^11,t^14 and
unit residues1. If so test Z=t^3*y and t=w^7*x^2,Z=w^11*x^3, exact
rational inverses and both lambda residues0,1, unit full Jacobians.
A failed factor/unit test is retained and changes the next proposed chart,
not evidence of nonexistent branches. The proper normalization bridge and
degree accounting are written separately. No global normalization rerun.

Planned3 native calls: CHAR positive, CHAR corrupt, CD charts. One conditional
additional CD series call is authorized only after positive exact charts,
with192-term lifts, original five residuals modw100, all8 actual entries,
explicit valuation controls and effective nu precision after division.
Freeze its exact source before launch; any needed Laurent or nonunit branch
requires a prospective amendment. Preserve all failed sources/logs. Standard
acceptance exit0+marker+no error diagnostics, rather than exit0 alone.
No timing/statistical arms apply: deterministic fixed identities and exhaustive
finite sets only, no sampling, coefficient search, rank or cost comparison.
Checkpoint current source/raw before every invocation and do not mutate
repository state while checkpoint/gate runs. Use task-owned TMPDIR because
/tmp is inode-exhausted. Seal raw once finished, publish current existing PR.

Prospective implementation correction before CHAR02: CHAR01 rejects reserved
identifier ord. Rename subgroupSize; all equations, finite sets, detector and
caps stay unchanged. This is an implementation retry, not a new search arm.
Full CHAR01 diagnostics also reject intvec-entry ++. Replace it by explicit
indexed assignment before02; no finite population or detector is changed.

## Prospective transport amendment after rejected CD01
CD01 rejects exact t^14 divisibility and the unramified residue: the raw
Z coordinate has a different constant-term order at C/D. No branches are
accepted from it and its conditional series call is NOT launched. Instead,
use the existing R18 diamond2 re-normalization to transport the certified O
charts. Freeze one TRANSPORT26 call: universal exact Tate/line identities,
then exact reduction in F2(u)[v]/(v^2+v+u^3+u) to compare the base action to
u'=(u+v)/u^2,v'=(uv+u+v)/u^3, all5 iterates and identity at step5.
Check O maps toA0 with original local uniformizer leading coefficient1.
The proper source-chart transport and cusp-cycle interpretation use the
written R16/R18 moduli/normal-curve bridge; no new original-coordinate
valuations or complete divisor is inferred. Resource/detector caps unchanged.

## Prospective source-orbit strengthening
TRANSPORT01 accepts the exact base action/period and universal Tate/line
change. One final SOURCEORBIT26 call strengthens the written source inverse:
iterate the actual affine lambda/nu transformation coefficients five times
in the exact quadratic base function field, retaining each intermediate
coefficient. Require lambda coefficient1/offsetA,nu coefficient1/lambda
coefficient0/offsetb at step5. Together with sigma^2=1 this gives the actual
rational T^10 identity and inverse T^9. No finite fixture substitutes for
these function-field equalities; same4GiB/1200s cap. No further native calls
are planned beyond implementation corrections, and no CDseries is launched.

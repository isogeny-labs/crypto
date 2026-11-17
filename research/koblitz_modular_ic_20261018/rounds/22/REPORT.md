# Round22: integral recovery and the proper-geometry frontier

Round began2026-10-26;verification/publication continued2026-10-27 UTC.
Scientific input69f6740;publication parent3c87193;Conductor T-409.
This is an exact arithmetic and geometry investigation on public synthetic
fixtures. No clock,IC cost or efficiency conclusion is admitted. The source,
proof and raw-data manifests are in ledger.json and runs.sha256;see
[validation](VALIDATION.md),[proof map](PROOF_MAP.md),[sources](SOURCES.md),
[PDF](report.pdf) and [visual](round22.svg).

## Requested scope and observed outcomes

| Requirement | Status | Evidence and exact remaining gap |
|---|---|---|
| Integral equivariant recovery on four norm-zero factors | Verified complete in the stated CM augmentation category | G32 full integer system,Smith transformations/inverses,independent integer replay;G33 two-map recovery. Geometric modular/final-quotient realization remains separate |
| Proper level77 normalization,pole divisors,complete prescribed RR and factor map | Partial | Exact genus-one base,complete base divisors,generic QR model,degree8-by2 tower,irreducible plane projection,smooth registered open and original SS5 fiber (four degree20 points). Lifted branches/divisors and the complete RR basis remain open |
| Focused node187 coupled-divisor investigation | Partial | All four old scalar points excluded by simultaneous valuations under explicit proper/pole bridges;all80 ordinary coefficient conjugates bound to the source. No explicit polynomial separator or h-power membership certificate |
| Direct relations through an arbitrary-divisor reverse evaluator | Prerequisite open | n3 Klein and same-field n5 regression controls retained. No F2 modular-factor arbitrary-divisor evaluator;ordinary460/367 fibers/images remain open |

## Exact norm-zero recovery

The requested Smith calculation succeeds with least positive scalar **d=1**.
Set O=Z[w],w²+w+2=0 and use the actual augmentation rotation
R(v)=(-sum(v),v0,v1,v2). The integral embedding
J(v)=(v3,v2,v1,v0,-sum(v)) satisfies PJ=I and Pi J=JR;the original G25
rows satisfy Phi_j=J M_j. The complete commutant is O[R],with basis
1,R,R²,R³,because e0,Re0,R²e0,R³e0 form a unimodular cyclic basis.

Expanding all four recovery polynomials in1,w gives the complete8-by32
integer system. Its rank is8 and its Smith invariants are
(1,1,1,1,1,1,1,1). The retained U,V and their two-sided integral inverses
certify UHV=D. The24 remaining columns of V give the full integer kernel;
the right inverse W satisfies HW=I. Every solution is a particular solution
plus this kernel. This is an exhaustive lattice answer,with no unknown-
coefficient cutoff. The scaled2H and zero controls return scalar2 and no
positive scalar respectively;one corrupted recovery coefficient is rejected.

An independent coefficient-pairing construction supplies a second checked
four-map recovery. A further exact inversion modulo7 gives a concise third
representative using only M1 and M3=7I:

```
M1=(2+w)+7R+(6+2w)R³,
B1=(1+3w)+(2+5w)R+3R²+(1+2w)R³,
B3=(-2+4w)-3R²+(-2+3w)R³,
B0=B2=0,                 B1*M1+B3*M3=I.
```

Both matrices commute with R. Independent CM and8-by8 integer products
agree,and all22 order11 fixtures pass,eleven per ordinary sign,including
the identity. Frobenius represents w on a=0 and negative Frobenius on a=1.
The exact universal polynomial certificate and full matrix tables are in
[RECOVERY.md](RECOVERY.md) and the separate Lean modules.

The inherited augmentation polarization is G=JᵗJ=I+ones(4),with Smith
diagonal1,1,1,5. The original weights45,315,63,171 give the exact weighted
Gram33075 H_old=3*105² H_old. Replacing G by the identity is rejected.
An integral retraction on norm-zero subobjects need not extend to the
regular target cycles;R21's sharp denominator5 result remains valid there.
No final105/19 polarized quotient or modular Jacobian map follows from
this change of target objects alone.

## Proper geometry: new exact data and bounded failures

The existing Tate11 base is birational to the smooth genus-one cubic
v²+v=u³+u. Native rational substitution and independent Rust arithmetic
agree. Complete base divisors of b,c,delta0,1+c,b+c+c²,x(4P) and the generic
nu-recovery denominator are listed in [CM.md](CM.md). The residual
delta0 support has degree5 and polynomial u⁵+u⁴+u³+u²+1;the exceptional
supersingular support has degree5 and polynomial u⁵+u³+1. Both are checked
irreducible. Local rational-place series,unique infinity orders and exact
norm factors certify support completeness on this base.

The generic QR ideal has dimension16 over the rational base field. Both
lift matrices certify equality with its triangular lambda/nu presentation.
Taking the quadratic norm in v gives an irreducible degree32 plane
projection,with checked birational v recovery. The registered incidence
open has empty saturated singular locus under the written pure-curve bridge.
An additional check verifies all245 rational circuit gates by exact cross
multiplication,including nonzero denominators.

The exact trace variable z=lambda²+(1+c)lambda+b gives a degree8 subfield
and a quadratic lambda lift. Its polynomial H is in CM.md. The a1=1+c
exceptional support is essential:the generic nu formula has denominator
a1² b³ delta0,and the lambda projection loses information at a1=0 even
though the original incidence open is smooth there. Original nu branches
must be retained in any proper normalization.

That original SS5 fiber is now computed directly. It is exactly
lambda^4+b*lambda+b^2=0 and nu^4+b^3*nu+b^4*(lambda+1)=0 over
F32 with b=alpha+1,c=1,alpha^5+alpha^3+1=0. Both ideal lift products
vanish,all registered units survive,and the relative Jacobian is b^4.
The16-dimensional reduced algebra has four degree4 residue fields over
F32,certified by the complete q32 Frobenius matrix. Thus its four closed
points have degree20 over F2. This is an original affine-fiber certificate;
the full proper normalized branch accounting remains a written bridge.

At the frozen4GiB/1200s limits,affine closure normalization timed out;
degree96 and torically reduced degree72 place computations exhausted the
address-space cap. A local integral-basis attempt also timed out. The exact
unimodular monomial search tested1384 transformations in[-8,8]. The smaller
trace-subfield normalization is a separately registered attempt;its actual
outcome is recorded in VALIDATION.md. No timeout is a nonexistence result.

The next construction needs complete normalized charts and transition maps,
every lifted coordinate/denominator divisor including supersingular and
infinity branches,and a complete prescribed L(D) basis or dimension. A map
candidate must then satisfy exact function-field identities,coefficient
descent and nonzero order11 action. Sutherland's independent level77 model
is a possible check only after its relationship to this incidence component
is certified. The shared residual Hecke system is insufficient to certify
the subgroup action.

## Coupled node187 calculation

The actual first four ordinary functions have form P_j(x)+y S_j(x),with
denominator1,deg P_j=9,deg S_j=8 and monic S_j. Thus each has its only pole
at O,of exact order19 and leading coefficient1. The native expansion checks
all four programs and all five coefficient conjugates,80 functions total,
and binds every descent residual and cumulative product gate.

Node187=F0 F1,where F0=node126 and F1=node186;node247=F0 F1 F2 is also a
retained coordinate. For a hypothetical DVR limit with h of order N>0,
individual zero residues and product residue1 force v0,v1>-N and
v0+v1=-N. Both functions therefore have poles. Proper reduction and the
actual identities P1=sigma(P0)-sigma(P3),P2=sigma(P1)-sigma(P3) force a pole
of F2 as well. Then v(h node247)=v2<0 contradicts regularity. Coefficient
sigma permutes the split components and fixes the arc parameter;it is not
absolute Frobenius on the Laurent-series field.

The joint leading coefficient condition is
N=19(m0+m1),eta(c0 c1)^(-19)=1 in every conjugate. This is not replaced by
two independent nineteenth-root arguments. Lean proves the simultaneous
algebraic implication with the geometric hypotheses explicit. Native
controls check129024 integer assignments and retain an uncoupled positive
control that fails exactly the proper-descent bridge. See
[SATURATION.md](SATURATION.md) for the argument and its limits.

An explicit polynomial f separating the old points with a verified
h^k f in I has not been extracted. Full quotient saturation and a proper
polarized embedding remain separate obligations. The frozen Q40 scalar
models still satisfy their original necessary system.

## Reconciliation,controls and reporting limits

Migration reconciliation preserved the original source quiescence and
managed Diem deadline;no generator was relaunched and no EKS job submitted.
Interrupted integrated builds,failed generated Lean files,Singular syntax
errors despite exit0,resource failures and the incompatible F8-coordinate
Klein invocation are retained. Matching F512-coordinate Klein and F32
same-field n5 controls are replayed separately. Their observed counts and
the current complete test/axiom results are in VALIDATION.md.

The full task remains open on the precise geometric and evaluator gaps
listed above. No relation-rank promotion or efficiency assessment is supplied.
Canonical cost/rank graphs remain unchanged because this round measures
neither. The carrier-status text and source-linked visual are updated with
the exact recovery result and remaining obligations.

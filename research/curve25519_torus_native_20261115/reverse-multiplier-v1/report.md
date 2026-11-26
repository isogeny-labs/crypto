# Target-dependent torus multiplier: candidate and recovery gate

24 November 2026. This is a proposed mechanism and written derivation.
No solver implementation, native/formal qualification, measurement, root
discovery or exponent result is reported in this component. [pcc:unverified]

Instead of choosing a base multiplier and searching its member indices,
fix a few relative labels and solve for their common multiplier sigma from
the public target. A rational parametrization makes each raw Montgomery
x-coordinate a degree2 rational function. For two leaves the written cleared
S3 equation has degree at most8; for six leaves the analogous formal upper
bound is384 per branch. Those degrees are independent of q,K because the
base multiplier is free, not because the original fixed-base constraints
became cheaper. [pcc:unverified]

## Construction to qualify

Keep p=2^255-19,A486662,a60824497,q213156431,K20633, original h and
S={h^(j+a*t):1<=j<=K,0<=t<q}. For alpha_i=(a_i,b_i) in S and epsilon_i=+/-1,

    sigma(c)=((1+2c²)+2c*sqrt(2))/(1-2c²)
    V=1-2c²; U_i=a_i(1+2c²)+4b_i*c
    N_i=A*(epsilon_i*V-U_i); D_i=2U_i; u_i=N_i/D_i.

For finite R=(r,s), substitute into the classical Montgomery f3, clearing
D_1²*D_2². DERIVATION.md gives the complete degree8 expression and guards.
The infinity chart sigma=-1, zero polynomials, rational ordinates, signs,
original raw/cofactor labels and target torsion sectors remain explicit.
The Semaev equation alone is not rational group replay. [pcc:unverified]

The proposed public query receives curve/spec,template parameters,R/Q and
limits. It receives no known multiplier, candidate list, planted target
scalar, root or known section. A returned row lives in S_sigma. The current
unshifted Circuit cannot be called unchanged on a shifted map and counted
as original witness replay; the shifted projection is a new obligation.
[pcc:unverified]

## Why the recovery gate is decisive

S_sigma is invariant under multiplying sigma by H_q=<h^a>, which relabels
t. Under exact-order and interval hypotheses, this is the complete PARAMETER
set stabilizer; there are (p+1)/q classes. Actual point/cofactor aliases need
separate canonicalization. Relabelling an unchanged point gives no new row.
A fresh short decomposition per target is not reusable factor-base data.
[pcc:unverified]

On checked geometric prime-group columns, write M*z=a+b*x for public Q=xG.
After removing known-log columns and retaining all alias provenance, linear
recovery requires a left-kernel witness v with v^T*M=0,v^T*b!=0, giving
x=-(v^T*a)/(v^T*b). Equivalently rank([M|b])>rank(M). This is a written
finite-dimensional linear-algebra criterion, not a tested recovery instance.
FAMILY_GATE.md scopes the claim to those equations and known data.
[pcc:unverified]

If every row introduces its own private column, the rows have full row rank
and no useful left kernel. A common parameter-base class alone does not
establish shared columns or recovery. A candidate overlap/transport method
must supply that bridge and charge its full search/indexing cost. Neither
torus translation nor a birthday match of base labels establishes it.
[pcc:unverified]

## Proposed next operations

| Stage | Required evidence | Current status |
| --- | --- | --- |
| Algebra compiler | Clearing identity, degree/chart fidelity, mutation refusal | Written only |
| Two-leaf public roots | All toy roots vs exhaustive direct group; original-field public u9 controls | Unimplemented/unrun |
| Shifted-base projection | All raw/index/map guards and original target replay | Open |
| Shared-base recovery | Actual normalized M,a,b; useful left kernel and held-out scalar replay | Open; decisive before scaling |
| Six-leaf/scaling | Complete cold work,yield,family,index,memory/recovery bounds | Unimplemented/unrun |

PROTOCOL.md freezes prospective correctness domains and admission limits;
it schedules no experiment. A local development ceiling is not paid-job
authorization. No budget, signer, artifact policy or frozen evidence changes.
[pcc:unverified]

For the familiar COMMON-base six-sum model only, useful yield B^6/ell,
decomposition cost B^gamma and B independent rows give relation work
ell*B^(gamma-5) plus sparse algebra B², with the conditional balance
2/(7-gamma). This candidate has established neither that yield nor gamma.
In particular the free-multiplier equation cannot be plugged in as gamma0.
Original criteria still charge every phase and require a growing eligible
family. [pcc:unverified]

## Secondary degree audit and scoped stopping rules

Independent sign flips on the old multiquadratic cover may diagonalize its
pullback-degree form, replacing an L1-square estimate by a coefficient-square
sum. DERIVATION.md records the conditional argument and the unconstructed
geometric bridges. The old energy draft is unchanged and unpromoted.
This is classical quadratic-form reasoning, not a claimed new solver.
[pcc:unverified]

A clearing/lifting counterexample rejects the current construction/domain.
An incomplete factorization or resource refusal does not establish absence.
Private-column/rank failure rejects a claimed recovery from those rows;
it does not reject every adaptive-base scheme. Missing family/cost/yield
proofs stay open requirements. A sufficient phase budget is not a necessary
lower bound or universal disproof. Original ROUTE_CRITERIA.md/V3 remain
byte-identical. [pcc:unverified]

Sources: original section/index/received-ideal and theory drafts; EFD
[Montgomery formulas](https://hyperelliptic.org/EFD/g1p/auto-montgom.html);
Yokoyama etal [section2.1/Proposition1](https://nutmic2019.imj-prg.fr/confpapers/SemaevECDLP.pdf);
official [FLINT factor/root interfaces](https://flintlib.org/doc/fmpz_mod_poly_factor/index.html);
Milne [chapterI section5](https://www.jmilne.org/math/CourseNotes/AV.pdf);
Farashahi etal [Theorem3](https://www.normalesup.org/~tibouchi/papers/welldistributed.pdf).
SOURCES.md records the exact uses, KB127 and original Semaev PDF403 gap.
No exhaustive novelty clearance. [pcc:unverified]

[Editable diagram](diagram.dot) / [SVG](diagram.svg) / [PDF report](report.pdf).
No measured graph is available because this component executes no scientific
worker. Canonical scoreboards and all prior diagrams stay unchanged; no
performance datum or established map edge is added. [pcc:unverified]

# Full-quintic coefficient-span compression through two exact parameter blocks

UNEXECUTED next hypothesis; no frozen source, manifest, input or evidence is
changed. This constructs the SAME full73-bit relaxed norm family in fewer
coordinates. It does not reduce its surviving information dimension, solve its
support problem, or remove any outer parameter. Unlike doubled-source cubic
charts, it imposes no collision or affine-rank restriction: five-distinct/rank4
candidates remain in the family, although no rank4 supported root is claimed.

## Existing normal form and exact constants

The normal form is ALREADY proved in
research/coupled_line_cells_20261029_v1/PROOF.md. It is classical Frobenius,
Artin-Schreier and linear algebra, not a new primitive/novelty assertion.
For y²+xy=x³+A x²+B and fixed targetT=(z,yT), h,k, put w=s+zr,
b=h²+h+z and use the existing monic quintic H=(e5,e4,e3,e2,b,1). Then

    e2=C2+r²+hr,
    e3=C3+z r²+k r+h w,
    e4=C4+w²+kw,
    e5=C5+z w²,

where

    C2=k+h²A+zb,
    C3=k²+z C2,
    C4=k²(A+z)+z²k+h²*yT²+hk*yT+z4,
    C5=k²(yT+zA+z²)+z*h²*yT²+z5.

These follow by substitution into the pinned literal norm coefficients; all
parameters are shared. The proposed actual experiment fixes the SAME first
h=0x4208336d0d441df79c400,k=0 target/domain. No independent root or lift quotient
is introduced. Require h!=0, k=0, and admitted b inW1 before this reduction.
The actual source-domain product spans W1..W5 have dimensions18,35,52,69,83.
Thus e1 is already admitted and e5 is unrestricted inK. Only e2,e3,e4 constrain
the coefficient relaxation, exactly as in the existing166-variable solver.

## First independent block: e2 through a trace-admitted AS preimage

Let v=e2 range over W2 with a fixed independent35-element basis(v_i). Set
u=r/h. Then u²+u=(v+C2)/h². OverK=GF(2^83), this equation is soluble iff
Tr((v+C2)/h²)=0. On the35 basis coordinates this is ONE binary affine equation

    sum_i t_i Tr(v_i/h²)=Tr(C2/h²).

Let its actual rank be rho in{0,1}. rho0/RHS1 proves the ENTIRE coefficient
family empty, with a replayable constant trace contradiction. Otherwise solve
it exactly, giving v0 and35-rho independent allowed directions. The odd-degree
halftrace HT maps each trace-zero RHS to one AS solution. Therefore

    r0=h*HT((v0+C2)/h²),
    r_direction_i=h*HT(v_direction_i/h²),

plus the additional kernel direction h parameterize every allowed r exactly.
There are35-rho+1=36-rho independent r coordinates:35 ifrho1,36 ifrho0.
Independence is proved by applying r->r²+hr: the image directions force all
v coefficients zero, then the nonzero AS kernel direction forces its bit zero.
No generic trace rank is assumed. Cache/construct every trace and halftrace
operator with paid cost; a Frobenius arithmetic identity is not free setup.

## Second independent block: e4 through a Frobenius preimage

At k=0, e4=C4+w². Frobenius is a field bijection, so

    w0=sqrt(C4), w_direction_j=sqrt(W4_basis_j)

give the complete independent69-coordinate w family. Cache sqrt(W4basis)
ONCE per full compared workload and charge it. This is exact, not a generic
rank observation. k!=0 requires a different AS treatment for w²+kw and is
outside this proposed variant; it must not silently inherit the k0 formula.

Combining r and w, and restoring s=w+zr, gives an explicit affine bijection
between104 or105 compressed coordinates and the entire original e2/e4-admitted
(r,s) family. The map is injective because r,w are recovered from r,s. In these
coordinates e3 is F2-affine; its W3 complement has only31 bits. Solve these
remaining31 rows exactly, preserving inconsistent/particular/kernel branches.

This is a bordered block elimination:48 original r-only constraints and14
w-only constraints are solved independently, followed by31 coupling constraints.
The sparse structure is in the PROVED parameter dependence, not in formal
Jacobian zeros or a presumed low-rank neighbor update.

## What the existing actual93/73 certificate logically forces

If the normal-form bijection and counted constraints hold, the old actual
relaxation has dimension73, already independently Gaussian-certified from166
coordinates. A consistent rho0 branch would have105 compressed coordinates
and at most31 remaining constraints, leaving dimension at least74. This
contradicts the existing73 certificate. Therefore THIS actual branch must have
rho1,104 coordinates and coupling rank31, if the reduction is implemented
correctly. These conclusions follow from equivalence plus the old certificate;
they are not generic assumptions or new measurements. A future worker must
recompute all ranks and map equality: any discrepancy falsifies code, constants,
constraint accounting or the proposed bijection, not a reason to discard data.

The final dimension is104-31=73, unchanged. A cheaper construction/solve may
improve component time/storage but cannot become a search algorithm for2^73
supported quintics by itself. Original-root/auxiliary/ghost closure remains open.

## Concrete falsifiable next experiment

Write NEW code and obtain independent derivation/code review before hash freeze.
First test GF4/GF8/GF16 all r/s and deterministic fixedh,k0 cells against literal
norm products, exact original coefficient-span tests and both mapped affine
families. Exhaust rho0consistent/rho0inconsistent/rho1 cases with synthetic
trace subspaces where necessary; distinguish actual norm coverage from synthetic
branch controls. Include h0 refusal, k!=0 refusal, repeated roots and both empty/
nonempty families. Small domains may have W5 notfull; retain the extra e5
condition (and e1 admission), rather than importing the actual83 simplification.
For nonprefix domains construct actual basis-product spans, not prefix bounds.

Then freeze the SAME actual h,k0 fixture and compare the strongest current
166-coordinate exact affine coefficient-map solver with the104-coordinate
block solver. Both parse/validate the SAME input once, charge field/span/cache
construction, row formation, Gaussian elimination, mapped-family verification,
serialization and peakRSS. No candidate supported roots or2^73 states are
materialized. Retain all constants, trace row/RHS/rho, r/w directions, compressed
rows/particular/kernel and mapped166-bit family. Verify complete affine-family
identity by independent span/coset checks and full rank; do not compare coordinate
basis arrays as if they shared variable names.

Use3 alternating SAMEPROCESS repetitions initially,90s/4GiB/1core, compactraw
and strict output caps; no cold/qualifiedGB claim. The falsifiable performance
prediction is smaller PAID full-workload time or memory, after AS/halftrace and
sqrt cache costs, not merely fewer rows. If setup dominates or total is slower,
retain that negative result and change the outer-cell invariant or shared-cache
hypothesis before another run. Existing normal form already contains the main
algebraic identity, so neither this implementation nor a speedup establishes
novelty priority.

The strategic goal remains exact full-quintic support/whole-cell search and
original all-roots recovery, with both hard all-distinct inputs and genuine
F4/F5/F6 matching across fields. This span compiler is a relevant building block,
not another constant-functional ANF retry or an8x release certificate.

## Implemented v9 contract

One combined controls/profile worker uses the proved constants from literal
H(0,0), with normalform identities independently exhausted over GF4/GF8/GF16
r/s values on declared nonprefix domains. Extra e5 rows are retained ifW5 is
notfull, as is the constant e1 admission. h0/k!=0 returnsunsupported. Tiny even
extensions use a complete binary linearized AS solve, never odd halftrace;
actual n83 uses halftrace with trace/root replays. Synthetic trace and AS
preimage fixtures exercise rho0consistent/empty/rho1, independently allr.

The strong baseline is the current166-coordinate exact affine coefficient-map
compiler, with all5 residual coefficient blocks and Gaussian/provenance checks.
The new compiler constructs r/w directions and evaluates only remaining e3/e5
variation columns analytically. Both pay originalfield/fixture/spansetup and
mappedliteralparticular/kernel verification and serialization. Mutualkernel
span and affinecoset checks establish complete equivalence, no2^73enumeration.
ThreealternatingSAMEPROCESS reps are diagnostic; no cold/qualifiedGB claim.

Firstproof perrepresentation is retained, repeated timing records alias the
same deterministic implementation. Raw256KiB/gzip24KiB/stdout16KiB caps protect
disk. Exact artifact research/full_quintic_block_20261115_v9/raw.json.gz. Anylate
orrawomission demotes topcomplete/dimension/ratio; no originalrootcount exists.
90s/4GiB/1core with80swork84sdemote88sprint plusparentlifetime90s.
AuthorperformedASTparseonly, no scientific execution or prior frozen edits.

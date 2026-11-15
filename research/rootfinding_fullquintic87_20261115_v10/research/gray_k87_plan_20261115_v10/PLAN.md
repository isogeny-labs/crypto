# Unexecuted outer-k Gray updates for the exact87-coordinate compiler

This is a changed bounded compiler hypothesis, not a new root search or novelty
claim. No current source/input/raw evidence changes. Read the primary repository
prior work: research/gray_matrix_delta_20261029_v1/PROOF.md,
gray_matrix_delta_review_20261029_v1/RESULTS_REVIEW.md and
QUOTIENT_GRAY_UPDATE_NEXT.md, plus autolab_cycle_20261029_v6/NOVELTY_REVIEW.md.
That work varied SOURCE x inside one fixed73-parameter h/k family; raw83x73
neighbor differences had sampled rank68/69, refuting cheap low-rank updates.
Its vertical quotient45x35 already has exact XOR Gray increments. The present
plan varies OUTER k in the14x87 coefficient-span compiler at fixedh. Different
parameter/representation/cost does NOT establish novelty. No Sherman-Morrison
or low-rank neighbor assumption is made; a delta may have full row rank14.

## Fixed-h source coordinates

Use the reviewed any-fixed-k normal form and admitted nonzeroh. W2/W3/W4 and
trace coefficients Tr(v_i/h²) do not depend onk. Actual trace rankrho1 means
the one-row trace equation is solvable for everyk. With fixed Gaussian pivot/
free-column choices, its particular v0(k) is F2-affine ink, its kernel fixed.
The linear AS solution section (odd halftrace or a fixed even-degree linear
section) implies R(k)=r0(k) is F2-affine, and the r directions dr are FIXED.
This is F2-affinity, not K-affinity: R can involve many Frobenius powers.
If rho0, retain the exact trace-admitted affine k set and empty inadmissible
branches; never apply an everywhere-soluble coordinate chart. h0 unsupported.
The first actualh withrho1 is the initial experiment; generalrho0 is a guard
control, not an assumed absent exceptional case.

For an r direction d, put a=(z/h)d², b=d/h. Its w direction is a+kb.
Consequently its e4 direction is

    a²+k*a+k²*(b²+b).

For a v3 direction v, let c=v/h; its e4 direction is c²+kc.
Thus every field-valued matrix column has form M0+kM1+k²M2. Projection to the
fixed W4 complement is linear. The14x87 binary matrix is F2-affine in ALL83
bits ofk. A toggle delta gives a constant binary increment independent ofk.
Precompute those increments exactly and pay their construction; update by XOR.
They need not be low rank. The payload of83*14*87 bits is12637 rounded bytes;
row padding/Python objects/nativewords add overhead, and actual RSS must be
measured rather than called13KB totalmemory. On an8-bit cube only8 increments
are necessary; declare whether8or83 are constructed BEFORE timing.

## The right-hand side is QUADRATIC, not cubic

C2(k) is F2-affine; C3(k)=k²+zC2(k) is also F2-affine, as is C4(k).
Let R(k)=r0(k) and L(k)=h^-1*(C3(k)+zR(k)²). Then L is F2-affine and

    w0(k)=L(k)+(k/h)R(k),
    e4offset=C4(k)+L(k)²+(k²/h²)R(k)²+kL(k)+(k²/h)R(k).

Every displayed term is affine or a product of TWO F2-affine functions, because
k² and R² are Frobenius-linear in their bits. The naive cubic estimate for
k*w0 is loose: k*(kR)=k²R collapses the repeated k factor. Thus the projected
14-bit RHS has Boolean degree AT MOST2. No unsupported scalar-degree or
finite-sample identity inference is used.

On a frozen d-bit k cube, the constant, d single-bit and C(d,2) pair evaluations
exactly determine these quadratic coefficients BY THE PROVED DEGREE BOUND.
For d8 this is37 evaluations/37*14bits of coefficient payload. Precompute
first finite differences and mixed differences, then update the RHS exactly
along Gray order. Pay all evaluations, coefficient extraction and storage.
R0 and each required R(delta) can be cached from the same fixed AS section;
raw coordinate directions/particulars also need exact k-dependent updates or
recomputation, with their cost included in mapped-family verification/output.
Tiny W5extra rows have their own exact offset formulas and must not be dropped;
initial actualW5full removes them by proof, not convenience.

## Concrete next experiment and strong comparator

New source, independent review and immutable hashes before execution. First
exhaust tiny GF4/GF8/GF16 allk, includingrho0 admitted/inadmissible sets, against
direct current87 matrix/RHS construction, literalnorm and ALL coefficient-span
membership. Check degree2 RHS by full truth-table ANFs, not just differences
atbasis points; corrupt a delta/RHS coefficient and require rejection. Verify
mapped complete family equality at every point, including empty/rank-changing
cells and even AS sections. Rank changes cannot silently reuse an old kernel.

Then freeze the actual sameh and k cube spanned by canonical field bits0..7,
basek0,256 vertices. Use standard reflected Gray order g(i)=i XOR(i>>1).
Compare (a) current87 direct construction, (b) direct FIELD-column construction
with the same fixed-h AS/r-direction cache shared ONCE, and (c) exact matrix/
quadratic-RHS Gray updates with the same shared cache. Keep the stronger cached
comparator; do not win by reparsing input or repeating AS setup only onbaseline.
Decide8increment tables for this cube (not all83) and record extrapolated83
payload separately as arithmetic, never measured cost. All256 vertices remain,
no selecting empty/positive/fast cells. Botharms can freshly solve14-row systems;
no unsupported low-rank Gaussian update is required or claimed.

Pay parsing, h/domain/AS/cache preparation, increments, quadraticRHS37point
preparation, matrixupdates, Gaussian elimination, complete mapped166-bit family
verification, literalreplays and serialization. Record all stage costs ANDtotal,
peakRSS, ranks, contradictions and canonicalvertexlist. Use90s/4GiB/1core and
strict global proof/raw caps; any unvisited/incomplete vertex makes the fullbatch
comparison UNKNOWN and ratiosnull. Atleast3 cold repetitions/qualifiedisolation
are eventual release obligations, not supplied by initial SAMEPROCESS tests.
No2^73 family enumeration, all-root count or support solve is performed.

Falsifiable prediction: Gray updates beat the STRONG cached direct compiler in
paidfullbatch time without changing a single complete affine family. A gain
only in matrix formation is insufficient. If Gaussian/mapping/output dominates
or setup swallows the saving, preserve that loss and change the workload or
higher-level mechanism in a fresh plan rather than increasing the same timeout.
This accelerates256 outerk VISITS only; it does not prune2^83 possiblek,2^18 h
choices, the73-bit support family or original auxiliary/ghost branches. Full
rank4 support/root search and genuine matchingF4/F5/F6 fieldcoverage remain open.

## Strong-baseline fairness amendment and final frozen workload proposal

The original paragraph proposing only a direct166 comparator is SUPERSEDED by
this stronger requirement: the166 formulation ALSO has exact constant Gray
increments in(r,w) coordinates. Its direction changes are

    delta(e3_direction)=delta_k*dr,
    delta(e4_direction)=delta_k*dw,
    delta(e2_direction)=delta(e5_direction)=0.

Its constant changes are

    deltaC2=delta_k,
    deltaC3=delta_k²+z*delta_k,
    deltaC4=delta_k²*(A+z)+delta_k*(z²+h*yT),
    deltaC5=delta_k²*(yT+zA+z²).

Thus its RHS is AFFINE in allk bits, cheaper than87's quadratic RHS. Projected
nonzero166 constraints use93rows at the actualdomain. The mandatory strong
comparator is166Gray with the SAME fixedh caching/batching, not a deliberately
rebuilt166 matrix. Compare166Gray,87cached-direct and87Gray; optionally include
currentuncached compilers as clearly separate diagnostics. Both Grayarms pay
all field/span/AS/conversion/increment/cache/output costs and freshly recompute
rank/particular/kernel if needed. No low-rank Gaussian update is presumed.
The actual question is whether14x87 elimination saves more than the87-coordinate
transformation and quadratic-RHS preparation, versus93x166 withaffineRHS.

FINAL workload proposal replaces the earlier8-bit cube:256 fixed-seed EDGES
stratified across ALL83 directions, not one consecutive cube. Define xorshift64
state seed0x677261795f6b3837, shiftsleft13,right7,left17 with unsigned64wrap.
For edgei0..255 consume two successive words a,b and set
k_i=(a|(b<<64)) & (2^83-1), j_i=i mod83, k'_i=k_i XOR2^j_i.
Include every edge and both endpoints in everyarm; reportactualdistinctvertices
rather than assuming512. Directions0..6 have4edges and7..82 have3. This is a
source-defined development corpus, NOT unopenedholdout or a consecutive Gray
traversal. A matrix can reset frombase by XORing cached bitincrements for k_i,
then update the one selected bit for k'_i; resetcost is paid equally.

Both increment tables now cover83 bits. For87, the full quadratic RHS coefficient
list has1+83+C(83,2)=3487 Boolean monomials, payload48818bits (6103roundedbytes),
excluding object/word overhead. This table is determined by the proved degree2
identity; its3487 coefficient evaluations must be charged, not hidden as free
setup. Sharedh/AS maps can construct R(k),L(k) by exact cachedF2basis responses.
The166 affineRHS has84coefficientvectors, not a quadratic table. Everymemory
payload label distinguishes packedbits from actual allocatedmemory/RSS.

Declare90s/4GiB/1core andglobal512endpoint/fullproof limits, raw4MiB/gzip2MiB/
stdout32KiB beforeexecution. Partial/endpoints/capfailure make fullcorpus ratios
null; retain completed edge observations as diagnostics. Fullmappedcoset/kernel
identity and exact rank/constant reconstruction are checked againstdirectfield
formulas at every endpoint, with independent literal tinyoracles and all83
strata retained. A gain over104 alone or over uncached166 is not success. The
next prediction is a PAID fullcorpus gain over166GRAY; if thatfails, revisit the
higher-level support strategy instead of claiming Grayupdates are a new search.

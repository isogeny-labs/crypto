# Exact dual atlas for rank-deficient binary border fibers

UNEXECUTED theory/next design, 2026-11-15. This is a new bounded hypothesis
about varying the SAME global k, not another solve at an unchanged k. It uses
classical affine-matrix pencils, left kernels and Boolean quadratic equations;
no novelty, tractability, root or measured gain is claimed. [pcc:unverified]

## Fix the coordinate system before varying k

Fix the actual binary field, curve, target, first admitted h!=0 and source
product spaces. Use the already proved normal form and87-ordering:

    e2=C2(k)+r²+hr,
    e3=C3(k)+zr²+kr+h*w,
    e4=C4(k)+w²+k*w,    w=s+zr,
    C2=k+h²A+zb, C3=k²+z*C2,
    C4=k²(A+z)+z²k+h²*yT²+h*k*yT+z4.

All Cj are F2-affine in k. For the actual low18 domain e5 is unrestricted
because W5=K. Tiny cases with nonfullW5 must retain its extra rows; then the
number of dual masks changes and the14-row bound does not transfer.

Choose ONE ordered W2 basis and ONE AS right-inverse/section, independent of
k. The trace-functional rank rho of Tr(v/h²) on W2 is fixed by h, not k.
If rho1, choose a fixed vstar in W2 with trace1 and set
v0(k)=vstar*Tr(C2(k)/h²). The AS equation for(v0+C2)/h² has trace zero;
a fixed halftrace/right inverse gives an F2-affine particular r0(k).
Every allowed r is

    r=r0(k)+sum_i x_i*dr_i,

where the independent dr_i, including AS kernel directionh, are FIXED for
all k. There are35 such directions for actual rho1. If rho0, first require
Tr(C2(k)/h²)=0 on an explicit affine admissible-k domain; otherwise the entire
e2 family is empty. The consistent rho0 branch has36 directions and88 rather
than87 coordinates. It must not be forced into the rho1 model. Even binary
extensions require a complete fixed AS section, not odd halftrace.

Let v3=e3 range independently over fixed W3 coordinates and define

    w=h^-1*(v3+C3(k)+z*r²+k*r).

Thus the exact original(r,s) family satisfying e2/e3 is an affine coordinate
map for each k. No k-dependent pivot choice, direction basis or coordinate
permutation is allowed when claiming a global affine matrix pencil.

## The border matrix is affine; the border RHS is quadratic

Project e4 to K/W4 using a FIXED14-row complement. Write the remaining
system in x=(r-direction coordinates,v3 coordinates) as

    M(k)*x=b(k),  M(k) in F2^(14x87).

For an r direction dr, dw=h^-1*(z*dr²+k*dr)=alpha+beta*k. Its e4 direction is

    dw²+k*dw = alpha²+alpha*k+(beta²+beta)*k²,

which is F2-affine in k. For a v3 direction dw is constant, so its direction
is dw²+k*dw, also affine. Frobenius squaring is F2-linear. Consequently
M(k)=M0+sum_l k_l*M_l globally, despite dense binary incidence.

At zero x, r0(k) is affine. Write

    w0(k)=A(k)+h^-1*k*r0(k),
    A(k)=h^-1*(C3(k)+z*r0(k)²).

A and r0 are F2-affine. Then w0²+k*w0 consists of A², k*A,
h^-2*k²*r0² and h^-1*k²*r0. Each is a product of at most TWO affine maps,
since k² and r0² are F2-affine. Thus the projected b(k) is a Boolean
polynomial of degree<=2, including C4. Naively counting k*w0 as cubic misses
this Frobenius structure. Reduce diagonal k_l² terms to k_l canonically.

This is a formal proof over all83 k bits. Basis and two-basis evaluations
can construct/audit its coefficients; finite samples alone do not prove it.

## Enumerate dual masks, not all field elements

For every nonzero lambda in F2^14 define

    S_lambda={k:lambda^T*M(k)=0} intersect admissible_k.

The87 entries give AFFINE linear equations in the83 k bits. Exact Gaussian
elimination therefore gives empty or one complete affine coset S_lambda,
with particular/kernel/rank and contradiction certificates. There are exactly
2^14-1=16383 masks. This is a finite bound on masks, not a measured affordable
runtime or a bound on the number of surviving k/source roots.

Rank(M(k))<14 iff k belongs to the union of these S_lambda. Proof: a matrix
has deficient row rank iff its left kernel contains a nonzero vector. The
coordinate guard is applied first; no omitted inadmissible-k branch is lost.

For k in S_lambda, the border system is inconsistent whenever

    q_lambda(k)=lambda^T*b(k)=1.

Conversely every inconsistent binary linear system has such a left-kernel
witness, so the union over all masks of

    S_lambda intersect {q_lambda=1}

is EXACTLY the set of inconsistent border k values on the admitted domain.
This follows from column-space/left-annihilator duality; it is classical
linear algebra applied to a parameterized family, not new root information.

Restrict q_lambda to k=p+B*t using the full affine coset. Degree remains<=2.
If the restricted polynomial is identically1, the ENTIRE k-coset is rejected
for the coefficient relaxation: no r/s can satisfy it. If identically0 this
lambda is compatible there, but other masks may still prove inconsistency.
A nonconstant affine restriction yields an exact affine half-coset of
inconsistent k. A genuinely quadratic restriction must remain an explicit
quadratic equation or receive a separately proved exact decomposition; it
must not be labeled an affine coset or enumerated without its own cap.

Distinct lambda components may overlap. Store a union DAG or a canonical
least-witness rule; do NOT add their cardinalities as disjoint counts. A
canonical lambda for an already supplied k is the first nonzero mask passing
both equations, not a cheap implicit disjoint partition theorem. Incomplete
mask traversal retains the unvisited masks and global classification UNKNOWN.

Full row rank14 guarantees a consistent border with87-14=73 coordinate
bits. It does not prove support, prune that fiber, remove h, or classify all
original roots. Rank-deficient consistent fibers have even more free bits.
The method targets exceptional/inconsistent coefficient fibers only.

## Work/storage and falsifiable experiment

An affine pencil needs at most84 matrices of14x87bits (102312bits); a
quadratic14-bit RHS has3487 monomials (48818bits), before provenance and
runtime overhead. For each lambda, stream87 affine equations on83 bits,
solve once and restrict one quadratic. This is16383 bounded Gaussian tasks,
not a claim they fit90s. Do not materialize or enumerate 2^83 k values or
2^73 coefficient families. Cosets/quad constraints, not k point lists, are
the output. Stream or hash receipts under declared memory/storage caps.

First exhaust all k and all x for small affine pencils/quadratic RHS and
compare rank-deficiency, inconsistency and union coverage exactly. Include
zero/full-rank matrices, zero/nonzero constant RHS, affine guards, overlapping
mask cosets and genuinely quadratic contradictions. Then validate the fixed
actual field-level maps and all degree/basis claims before a capped actual
atlas profile. If preflight/caps prevent all masks, report only completed
mask receipts and UNKNOWN global coverage; a fresh implementation change,
not the identical timeout, is required next.

Prior repository overlap was checked by text search: coupled_line_cells_v1
already gives the normal form and line kernels; pair_cell_obstruction_v1
already uses exact dual inconsistency witnesses; prior local-support/jet
work already studies affine images and annihilators. No identical full-k
16383-mask atlas was located in this limited search, which is NOT evidence
of priority. Linearized-map theory and block elimination are established.

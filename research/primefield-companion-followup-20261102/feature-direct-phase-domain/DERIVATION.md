# Direct squarefree phase domains

Assume caller-certified odd prime r, 0<n<r and i²=-1 in F_r. The integrated
initializer retains m>=3, 4m|(r-1), n=(r-1)/(4m), the source contract and all
previous public-point recovery restrictions. Runtime never enumerates roots.

Let W=V+V^-1, D_j(W)=V^j+V^-j, S_j(W)=U_j(W/2), S_0=1,
S_1=W and S_-1=0. The second-kind recurrence is S_(j+1)=W*S_j-S_(j-1).
Writing a=S_k,b=S_(k-1), its doubling identities are

    S_(2k)=a²-b²,
    S_(2k-1)=b*(2a-W*b),
    S_(2k+1)=a*(W*a-2b).

second_pair recursively returns (S_j,S_(j-1)). At j=0 it returns (1,0).
At even j it uses the first two identities; at odd j it uses the first and
third. These are integer polynomial identities, hence specialize to F_r.
No division by V-V^-1 is performed, including at V=±1. The recursion depth
is logarithmic, but all polynomial products, copies and allocations count.

## Both positive phases from one pair

Set t=floor(n/2) and compute a=S_t,b=S_(t-1) once. The monic squarefree
phase-domain polynomials are

    n=2t:   g_(+1)=(W²-4)*b,       g_(-1)=2a-W*b=D_t;
    n=2t+1: g_(+1)=(W-2)*(a+b),   g_(-1)=(W+2)*(a-b).

The underlying factorizations are

    D_(2t)-2=(W²-4)*S_(t-1)²,       D_(2t)+2=D_t²,
    D_(2t+1)-2=(W-2)*(S_t+S_(t-1))²,
    D_(2t+1)+2=(W+2)*(S_t-S_(t-1))².

For n=1, t=0 and b=0, giving W-2 and W+2. Even formulas have t>=1.
The roots of V^n=±1 are distinct because r does not divide n. Inversion
identifies pairs, with fixed points only at V=±1. Counting those inverse
orbits gives degrees floor(n/2)+1 for +1, and ceil(n/2) for -1.
These equal the displayed factor degrees, so the monic factors contain exactly
the distinct W roots: no additional squarefree extraction is needed.
The explicit W±2 factors preserve the ramified endpoints.

All positive factory intermediates have degree at most floor(n/2)+1. It never
constructs D_n or its repeated-root specialization. The old constructor built
D_n (and an adjacent polynomial in its doubling helper) before taking a
squarefree part. This is a representation change, not a claimed time ratio.

## Negative phases

For u=±i, the phase equation is D_n(W)=0. Its V preimage is V^(2n)=-1,
which has 2n distinct roots as r does not divide 2n. None is V=±1, so inversion
pairs give n distinct W roots. The monic degree-n D_n is already squarefree.
The factory builds it once and returns it for both orientations, copying one
vector. No outer squarefree extraction is used on this path either. Negative
phases still have degree n, and the inherited Dickson helper can build an
adjacent degree-n+1 polynomial; the half-degree saving applies to positive
phases only.

## Integration and retained work

The optional new_direct_phase initializer replaces the old phase-polynomial
loop by this factory. Capacity preflight, initial character, V^n=u orientation,
mode0/signed section filtering, zero handling, later updates and final point
verification are unchanged. Limit fallback still calls the seeded streaming
initializer. The source and original cohort are unchanged.

CanonicalFiber::new still calls squarefree to validate its supplied domain.
Only the outer extraction is removed: no claim is made that all squarefree
gcds disappeared. The canonical constructor also handles ramified endpoints;
its operations, phase powers, quotient products/reductions, source setup/calls,
inversions, copies, allocations, lifting/replay and point verification remain
charged. Both domain vectors are held before their loop consumes them. The
section-slot cap is not a total-memory cap. No method-selection speed threshold
or total-work ranking is introduced.

## Evidence design and sources

The qualification checks second_pair against a separate linear recurrence,
compares every direct factor to squarefree(D_n-u-u^-1), verifies squarefreeness
and both W=±2 endpoints, rejects invalid factory inputs, and compares complete
candidate states and public recovery with the preceding initializer.
Enumeration is confined to known-answer fixtures.

Sources checked: ../feature-initial-phase-reduction/{DERIVATION.md,initial.rs},
its copied canonical.rs, and NIST DLMF18.5.1–2,
https://dlmf.nist.gov/18.5.E1 and https://dlmf.nist.gov/18.5.E2 (read2026-11-08).
Those standard reciprocal representations yield the identities above; no
novelty claim for the polynomial algebra. KB unavailable: MCP absent and
scripts/research-kb status has no executable; exact predecessor fallback used.
The derivation is written and finite-checked, not a new Lean runtime refinement
or independently signed verdict.

Next examine whether the shared phase domain can support shared evaluation
before orientation-specific filtering. Both positive factors and the negative
common support now have explicit structure, but quotient evaluation and the
source remain material costs; sharing cannot be assumed to preserve oriented
canonical state without an explicit construction and checks.

## Bounded negative-phase split opportunity

When n=2t, D_n=D_t²-2. If a public s with s²=2 exists in F_r, then
D_n=(D_t-s)(D_t+s), a coprime pair of degree-t factors. In the integrated
setting even n implies 8|(r-1), so 2 is a quadratic residue. An explicit
verified square root and its construction cost are required; it is not an
unpriced supplied scalar. One can alternatively obtain s from a verified
primitive eighth root eta via s=eta+eta^-1.

This suggests splitting each negative canonical phase into two W-domain
branches while retaining its original u=±i orientation. It is a bounded
algebraic split, not enumeration of scalar candidates. It halves each quotient
degree but doubles those branches, leaving total support degree unchanged.
All setup, root construction, copies and extra branch overhead must be charged.
The current patch does not implement this split or claim it is faster. Checks
would have to compare the combined materialized support and handle duplicated
u labels correctly; the present per-u canonical-signature comparison assumes
one branch per u and cannot be reused unchanged for that construction.

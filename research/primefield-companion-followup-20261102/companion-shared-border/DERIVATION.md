# Shared border functionals and a rank-one storage bound — 2026-11-02

Status: derived algebra and source construction, syntax/AST checks only.
Numerical, certificate mutation, heap and performance trials: zero.

## Scope and literature

Prime coefficient field F_p, p>3, cyclic down-shift Z_a with top-right wrap a.
The source satisfies Z_a B-B Z_b=GK^T, a!=b, with scalar leading pivot.
Reviewed preserved compact lift, border cache and fused operator sources.
[Bostan et al. (2017), Section 1, equation (2), and Section 6](https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf)
fixes the shift/displacement conventions and established structured elimination
context. Polynomial-in-shift identities below are derived directly. No new
circulant theory or general rank-based compression theorem is claimed.

## Conditional shared encoding

Let q=n-1 and t,c be row/column tails. With J the q-coordinate reversal,
check every coordinate of t=a Jc. On equality, retain c and a mode:

| Mode | Condition | Retained border field entries | Primal dot | Dual dot |
|---|---|---:|---|---|
| zero_row | a=0 and t=0 | q | 0 | c^T v |
| reverse | a=1 and t=Jc | q | c^T Jz | c^T v |
| scaled_reverse | a not 0,1 and t=aJc | q+1 | a(c^T Jz) | c^T v |
| plain | relation fails or no field-slot saving | 2q | t^T z | c^T v |

General scaled sharing is attempted only for q>1, since q=1 saves no field
slots. Tags and metadata are not field-value slots. Checking a=0 or a=1
uses q comparisons and no field arithmetic on success. General scaling uses
q field products and q comparisons on success; on first mismatch at one-based
index k, it charges exactly k products/comparisons. Zero/one failures charge
k comparisons only. Canonical-input scans and list/slice allocations are
additional host work, not included in field counts. No inversion or source
application is used. A fallback pays its detection overhead.

Every nonzero dot branch computes q products and q additions, even for zero
input vectors. The scaled primal branch adds one field product. The zero-row
primal branch returns zero without field arithmetic. Existing scalar lift
arithmetic, permutations and original-system checks are unchanged.

`BorderPair` implements detection and dot evaluation. A separate independent
`check_pair.verify` checks coefficients without producer imports; its own
multiplications are charged separately. This certifies the encoding relative
to supplied borders, not the borders relative to the original source.
`shared_frame.py` and `solve_shared.py` retain the prior one-use lifecycle.
The pair profile is retained by the trace, but its tuples are released after
success. Mode selection does not prove a structural property of the whole B.
In particular, local border equality need not survive the next Schur step.

For zero/reverse mode each frame stores q+2=n+1 field-value slots including
inverse and RHS head, versus 2n previously. For scaled mode it stores n+2.
For plain mode it stores 2n. All-linked zero/reverse full descent has
sum(n+1,n=2..N)=(N^2+3N-4)/2 slots; actual modes must be recorded per node.
These are representation-slot counts, not byte counts or measured peaks.
Full descent is still quadratic and all audit/certificate/current-source
memory remains outside this count.

## A source family that establishes the relation

For g=(g_0,...,g_{n-1}), put C_a(g)=sum_i g_i Z_a^i. Since Z_a^i e_0=e_i
for i<n, its first column is g. Its first row is
(g_0,a g_{n-1},...,a g_1), so t=aJc. It commutes with Z_a and therefore

    Z_a C_a(g) - C_a(g) Z_b
      = C_a(g)(Z_a-Z_b)
      = (a-b) g e_{n-1}^T.

This is displacement rank one whenever g!=0 and a!=b. The result permits
singular or nonsingular matrices; g_0!=0 is sufficient for the first scalar
pivot. Polynomials in Z_a supply positive validation fixtures, while a changed
row entry supplies a negative relation control. These fixtures are derived,
not generated or executed in this round.

## What rank one does not imply

Fix a=1,b=2,g_0=1. The tail c can be any of the p^q vectors in F_p^q, and
every resulting matrix has a nonzero first pivot and displacement rank one.
The family of linear functionals v -> c^T v has p^q distinct members:
if c!=c', a basis-vector query at a differing coordinate distinguishes them.
Thus a deterministic, exact, self-contained encoding capable of answering
all such dot queries needs at least q log2(p) bits in the worst case.
For L field slots plus at most M metadata states, p^L M>=p^q; counting
only slots while hiding data in unbounded tags would evade the premise.
Pointers to retained generators or external data are not self-contained;
recomputation can trade stored information for charged access/work.

A homogeneous scalar recurrence of order at most d, stored by d initial
values and d constant coefficients, represents at most p^(2d) length-q
sequences (lower orders embed by zero padding). Consequently 2d<q cannot
cover this rank-one family. Low displacement rank by itself cannot justify
a constant-order recurrence for the border. This is a bound for the stated
operator/query model, not a lower bound for elliptic discrete logarithms,
actual elliptic feature families, or algorithms asking fewer queries.

## Checks, failures and next construction

Separate static audits restore the declared frame substitutions and the
single solver import, then compare complete ASTs to preserved references.
All Python sources parse. No numerical execution is inferred from these
checks. Full validation must cover all four modes; first/last mismatch;
q=1; zero/one/general wrap; both dot directions; corrupted encodings;
consumed frames; exact counters; original-system equations and heap aliases.
Use the full existing upstream cohort and controls, without dropping failed
sharing attempts. Existing cost admission remains in force.

Next prioritize a recurrence supplied by the actual public feature map,
together with its preservation or explicit growth under pivoting and Schur
reduction. Alternatively price replay/recomputation. The rank-one bound
rules out inferring that recurrence solely from small displacement width;
it does not rule out feature-specific structure. No spectral recovery or
new elliptic transfer is claimed by this border representation.

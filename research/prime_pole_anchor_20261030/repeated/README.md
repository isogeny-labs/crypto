# Repeated anchors: gains and an all-level degree cost

2026-11-04. Follow-up to the [single-anchor certificate](../README.md).
**No novel sub-exponential prime-field ECDLP algorithm, improved exponent,
or measured speedup is established.** This note analyzes a specified family
of completions, not every possible H in W_base+C H.

## Sources and requested scope

The requested result remains a novel classical sub-exponential algorithm for
prime-field ECDLP. Reviewed before construction: the lead-atlas brief and
match for Cauchy/anchor/jet completion, the preceding pole and common-kernel
reports, and the pinned Oct25 manuscript D.4.3 and D.4.5--D.4.6. Primary source
links and digests are in [sources.md](../../prime_common_kernel_20261030/sources.md).
The source's existing linear-moment and Hermite arguments are the baseline.
An atlas match is not a novelty certificate. No new external priority search
was completed for this follow-up; elementary product/resolvent identities
are not claimed as new mathematics.

| Requirement | Status and evidence |
| --- | --- |
| Novel prime-field ECDLP algorithm | Unresolved; no complete solver or complexity improvement |
| Exhaustive review across six repositories | Partial historical review; not certified exhaustive |
| Extend the selected nonscalar jet completion | An explicit repeated-anchor family below |
| Exact empirical validation | Fixed native development certificates in PROTOCOL.md and replay.log |
| Full mathematical/formal certification | Written family analysis; Lean proves the abstract necessary budget only |
| Global recursive integration and total cost | Not implemented; no timing or end-to-end claim |

## Repeating the odd anchor

For integer k>=1 and delta even define

    W_delta^(k)(T) = B_delta(T) * ((1-T)/(1-delta))^k.

Leave odd-delta entries unchanged. The apparent pole at T=delta is already
cancelled in the original puncture polynomial B_delta. Its matrix degree
is exactly 2L-1+k. At T=delta the added factor is 1; at every other source
node B_delta is zero. Hence all node matrices, including W(0)=I, are preserved.

This is a genuine completion: ((1-T)^k-(1-delta)^k)/(T-delta) is an integral
polynomial. Therefore W^(k)-B is C/C'(0) times a polynomial with odd denominator.
The source chart divisibility of C/C'(0) proves chart integrality of the
correction. At an odd-node chart each coefficient of (1-T)^k is divisible
by 2^k, so the sharper original odd crossing content improves by at least k.

For y=y0+2j, d=T-y0+x and e=1-y0+x, the j^b coefficient is

    C(T)(1-T)^k/C'(0) * 2^b *
      sum_(r=0)^b binom(k+b-r-1,b-r) d^(-r-1) e^(-k-b+r).

It uses the SAME linear moments sum_j j^b X_y. At odd nodes its valuation
is at least a+1+k+b. At an even active level i, stride 2^(i+1), cancel
v=v2(d)<=i first. The binomial coefficient is integral, e is odd, and each
term has valuation at least a+1-v+r(i+1-v)+(b-r)(i+1)>=b+1.
Thus this repetition does not sacrifice the old even-level coarse bound.
Computing those binomial coefficients and sums is a cost, not a free oracle.

For an M-factor parent, degree becomes M(2L-1+k). The source's conservative
odd-node multiplicity lower bound 3ML alone suffices when k<=L+1.
Larger k requires explicit additional recovery headroom. This is a degree
sufficiency calculation, not a completed precision or runtime audit.
Only level zero receives this new guarantee, so even k growing with L does
not by itself improve the recursion's worst-level asymptotic bound.

## Trying to gain at every level

Here is a natural extension whose cost can be determined exactly. Choose
nonnegative multiplicities m_0,...,m_(a-1). For each delta, multiply B_delta by

    product over j with 2^(j+1) dividing delta:
       ((2^j-T)/(2^j-delta))^m_j.

Every included denominator is nonzero and has valuation exactly j. At
T=delta each factor is one, so source node values are again preserved.
This alone does NOT guarantee integral scaled charts. The diagonal delta=0
includes every anchor and has exact added polynomial degree sum_j m_j.

Consider the valid crossing entry x=y, at the retained node t=-2^i. It is
crossing when the packet's two symbols differ in coordinate i. For j<i,
the numerator and denominator valuations of the jth factor agree. For j=i
the ratio has valuation 1. For j>i it has valuation i-j. Consequently the
exact valuation change of this diagonal multiplier is

    G_i = m_i - sum_(j>i) (j-i)*m_j.

This also measures the change of the FIRST nonzero chart coefficient of
that entry: B_0(t)=0 with a simple zero, so the linear coefficient after
multiplication equals the original linear coefficient times this multiplier.
It does not assert that this is the minimum-content coefficient of every
chart or that the crossing matrix has reduced rank.

If one requires at least g extra bits for this coefficient at every level,
then necessarily

    m_i >= g + sum_(j>i) (j-i)*m_j.

All coefficients on the right are nonnegative. Backward induction therefore
shows the unique coordinatewise minimum schedule sets each inequality to
equality. To calculate its mass, build levels from highest to lowest.
Write S for the mass already built and P for its weighted penalty. Starting
at S=P=0, the next multiplicity is g+P and the update is

    S_new=S+P+g,    P_new=P+S_new.

For n>=1 this gives S_(n+1)=3S_n-S_(n-1), with S_0=0,S_1=g.
The Fibonacci recurrence implies F_(2n+2)=3F_(2n)-F_(2n-2), hence

    minimum total added degree = g*F_(2a).

In particular it is at least g*(2^a-1). More sharply,
F_(2a)=Theta(((3+sqrt(5))/2)^a)=Theta(L^log2((3+sqrt(5))/2)).
The exponent of L is about 1.388. For fixed g>0 this exceeds the O(L) degree
allowance of a constant-factor version of the original recovery budget.
Increasing multiplicities cannot repair this degree cost under the stated
requirement on ALL a levels. The source actually retains only u<a levels:
for anchors j=0,...,u-1 the same theorem gives g*F_(2u), not g*F_(2a).
In D.4, a=2*sum_(h<=d)u_h, so a>=2u and this necessary diagonal degree cost
can fit within O(L). Thus the all-a-level obstruction does NOT exclude a
useful retained-level variant of the source recursion.

Nor is diagonal gain sufficient. For example L=8, t=2 (i=1), x=0,y=1 is
a valid crossing pair with equal bit1. Its delta=1 enables no guarded
anchor, so that entry stays unchanged for ANY schedule. Refining the masks
or adding factors requires a new proof of crossing modes; masks for j>i
also vary within a source tile of stride2^(i+1). Charging the resulting
tile or moment refinements is unresolved.

Changing anchors,
using matrix cancellations, optimizing rank rather than every entry, or
using a different recovery budget is outside this obstruction.

The particularly tempting choice m_j=1 has G_i=1-r(r+1)/2 where
r=a-1-i. Thus higher anchors eventually REMOVE low-level bits. This is an
exact coefficient calculation; CPU contention cannot explain or fix it.

## Executed evidence and formal scope

The prospective [protocol](PROTOCOL.md) fixed all inputs before compilation.
Strict C++20/GMP native units pass 5152 full rational charts, 153368 integral
coefficients, 1232 odd-content comparisons, 108 direct rational level-gain
checks, and 24 insufficient-schedule negative controls. Sizes are L=4,8,16,
k=1,2,4,8 for polynomial checks, and a=1,...,8, g=1,2,4 for schedules.
These are local development correctness checks under the isolab-eks exception;
no scientific search, timing campaign, EKS job or background job was started.

For g=1, the exact minimum added degrees at L=2,4,8,16,32,64,128,256 are
1,3,8,21,55,144,377,987. These are certificate values, not fitted complexity.

The [Lean proof map](lean/PROOF_MAP.md) distinguishes the abstract all-length
budget theorem from the unformalized valuation-to-schedule bridge. Failed
builds and their diagnostics are preserved. No faster ECDLP conclusion follows
from passing those proofs or native units.

## Next discriminating obligation

A useful successor must avoid the charged diagonal scalar-product mechanism:
construct an actual nonscalar matrix completion preserving the SAME node
matrices, then prove a uniform weighted crossing factorization with its
degree, precision and entry costs. Fewer scalar summands or an odd-level
valuation gain alone does not establish that factorization. The original
two-axis common-kernel problem remains open. See the
[problem record](../../../problems/correlated-packet-common-kernel.md).

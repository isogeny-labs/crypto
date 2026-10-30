# Matched-level two-axis packets: recovery works; simultaneous geometry remains

2026-10-30 UTC. Written mathematical analysis, not a formalized proof or an
implemented algorithm. No attack, solver, timing experiment or benchmark was
run. No improved ECDLP algorithm is claimed.

## Sources and exact question

Read first:

- `/workspace/audit/multivariate-code-tradeoff-review.md`, which correctly
  separates code-distance statements from physical repeated-slot occupancy;
- the pinned Oct25 source `/workspace/audit/tensor-primary/openai-oct25.txt`,
  especially D.4.3-D.4.5 and LemmaD.5;
- the previously pinned Bochum source for the use of that contraction result.

The recovery below is a direct application and sharpening of the original
source's Hermite argument. Its credit belongs to that argument; this note
does not describe it as an original contraction theorem. An independent
derivation is recorded in `/workspace/audit/matched_recovery_child.md`.

Let L=2^a, S={-L,...,L-1}, alpha=2^(a+1), and

    S_i={t in S: v2(t)=i},     D_i=|S_i|=L/2^i.

Retain levels0<=i<u<=a. A proposed product Cauchy deformation gives a
polynomial F(T1,T2) of bidegree at most

    (d,d),       d=M*(2L-1),

whose origin value is integral. Assume all its scaled local expansions
F(t1+alpha*z1,t2+alpha*z2) are integral at the sampled matched-level pairs.
The proposed samples have t1,t2 in the same S_i. The question is whether
one can recover F(0,0) at precision O(M*D_i/u), while exploiting two genuinely
nonlocal axes to obtain a plane-partition rather than ordinary-partition
mode count.

The answer separates cleanly: origin recovery is available at that precision.
The fixed simultaneous packet geometry does not automatically have the
claimed mode density. A level-dependent geometry needs an additional common
deformation or matching-jets lemma.

## 1. Positive origin-recovery lemma on diagonal matched nodes

Put G(T)=F(T,T). Then

    deg G <= 2d=2M*(2L-1),       G(0)=F(0,0).

The diagonal nodes (t,t), t in S_i, are a subset of the proposed matched-level
set. At such a node write

    F(t+alpha*z1,t+alpha*z2)=sum_(b,c) f_(b,c)*z1^b*z2^c.

The coefficient of z^j in G(t+alpha*z) is

    sum_(b+c=j) f_(b,c).

Thus total-order jets b+c<q_i suffice. Rectangular jets 0<=b,c<q_i also
contain every needed coefficient. There is no division and no precision
loss in taking these sums. Integrality of the bivariate local expansion
implies integrality of this diagonal expansion.

A particularly direct safe corollary of source LemmaD.5 is to substitute
M'=2M into that lemma. It gives

    h'=ceil(6M/u),       q_i=h'*(u-i),
    p_i=p+8*h'*D_i+16M*(a+1).

Under the same precision-inheritance regime used in D.11, this is
O(M*D_i/u). The source's rational Hermite reconstruction then recovers
F(0,0) modulo2^p. It does not require mixed-valuation node pairs or a full
Cartesian grid. This alone proves the requested precision order.
The source proof uses local integrality only at its positive-multiplicity
nodes, so no data at omitted levels are required for this application.

For the asymptotically relevant u>=3 even the original source constants
already suffice. Set

    h=ceil(3M/u),        q_i=h*(u-i).

The total univariate multiplicity of the diagonal-node scheme is exactly

    W=sum_(i<u) D_i*q_i
     =h*L*(2u-2+2^(1-u)).

For u>=3 this is greater than4ML, and hence greater than deg G. The original
proof used the coarser odd-node lower bound because it only needed the
smaller original degree. Using the actual sum supplies the larger degree
without altering the reconstruction or its denominator estimate.

In detail, retain the original Hermite cardinal factor E_t. Its valuation
estimate from D.10 is still

    v2(E_t(0)) >= -6*h*D_i.

This depends on the taper q_i, not on deg G after unisolvency is proved.
Also q_i<=h*u<=4M because M>=u. Substitution of -t/alpha in a polynomial of
degree below q_i loses at most4M*(a+1) bits. Consequently the unchanged
source precision

    p_i=p+8*h*D_i+8M*(a+1)

suffices. Rational numerators and denominators have the same polynomial bit
description bounds as in D.5; there is no uncharged inverse of an even
number. After their charged powers of two are canceled, reduction modulo2^p
is valid. D.11's inherited-precision condition is still needed to abbreviate
this expression by O(M*D_i/u); the additive terms have not been discarded.

For small u the M'=2M corollary is sufficient and avoids a separate constant
analysis. The claimed asymptotic improvement concerns growing u.

This is origin-functional recovery. It does not claim stable recovery of
every bivariate coefficient at the same precision. Exact full polynomial
unisolvency, without this precision conclusion, already follows from the
odd Cartesian grid: L*q_0>=3ML>d in each variable, so repeated univariate
Hermite uniqueness applies. The low-cost origin formula is specifically the
diagonal one above.

## 2. One selected repetition bit really leaves half a grid

For a two-axis packet with L slots on each axis, let x1,x2 in{0,...,L-1} be
the physical slot coordinates. At one selected bit i impose

    bit_i(x1)=bit_i(x2).

Exactly L^2/2 positions survive. Equivalently, for a local D-by-D repeated
slot grid the allowed patterns00 and11 occupy D^2/2 positions. This is why
the unqualified inference from repetition-code dimension1 to only O(D)
physical slots is false. The review being read correctly identifies that
distinction.

The desired distance2 behavior can hold at that selected level: with
consistent orientation conventions, a change of the repetition label
requires both selected axis orientations to change. This observation alone
says nothing about simultaneously selecting every other retained level.

## 3. One fixed geometry imposing repetition at all retained levels

Now require the same physical packet to obey the repetition relation for
every retained level j=0,...,u-1:

    bit_j(x1)=bit_j(x2)  for every j<u.

These are u independent binary equalities. Thus the feasible set is

    x1 == x2 (mod 2^u),       K=L^2/2^u slots.

The analogous relation with fixed complements instead of equalities has
the same size. This is a statement about that fixed simultaneous orientation
design, not a general upper bound for all correlated packet constructions.

At a selected level i<u use the source's standard axis tiles

    y_k=y0_k+2^(i+1)*n_k,       0<=y0_k<2^(i+1), k=1,2.

Only2^(i+1) low-prefix pairs (y0_1,y0_2) are feasible, because their low
i+1 bits must agree. Each such tile has exactly

    K/2^(i+1)=D_i^2/2^(u-i+1)

feasible columns. The remaining restriction is

    n1 == n2 (mod 2^(u-i-1)).

Thus constraints below the selected bit reduce the number of nonempty
tiles, while constraints above it reduce the real multiplicity inside each
tile. One must count both effects.

Suppose packets are densely occupied on their feasible sets, with no
additional loss from graph packing, and there are Theta(M/K) packets. For
a fixed ordered pair of distinct repetition colors, half of the feasible
prefixes can be crossing source tiles; this is Theta(2^i). Consequently
the unmodified pair-moment construction has one mode per such tile per
(b1,b2), and its per-pair-index mode count has scale

    A_i=Theta((M/D_i^2)*2^(u-i)),                 (1)

up to fixed color constants. Sparse occupancy can change the count, but no
bound O(M/D_i^2) follows from the physical capacity argument. Formula(1)
describes the direct dense-feasible-slot implementation, not a lower bound
against algebraically merging modes or a different tiling/kernel.

## 4. What happens to the proposed plane-partition saving

Grant the proposed two-axis crossing expansion with mode index(b1,b2),
weight b1+b2+2, and precision p_i=O(M*D_i/u). There are s-1 index pairs of
weight s>=2 per base tile. Its standard generating-product upper bound is

    number_of_tuples <= exp(O(A_i^(1/3)*p_i^(2/3))).

For completeness, the generating product has logarithm at most C*A_i/theta^2:
sum_(s>=2)(s-1)*[-log(1-exp(-theta*s))]=O(theta^-2).
Optimize theta*p_i+C*A_i/theta^2 to obtain the displayed exponent.

If the unproved density A_i=O(M/D_i^2) held uniformly, the exponent would
indeed be O(M/u^(2/3)). The preceding fixed-geometry calculation instead
gives that method's cost envelope

    O(M*2^((u-i)/3)/u^(2/3)).

In particular the odd level i=0 carries factor2^(u/3). The stated method no
longer certifies the intended saving. This is not a lower bound on the
actual number of nonzero terms after all possible algebraic cancellations,
nor on another contraction procedure. It identifies the exact failure of
combining the direct pair-mode count with simultaneous repeated-bit slots.

There is a constructive way to improve that raw envelope, although it does
not give the proposed new exponent. Write k=u-i-1, N=D_i/2 and

    n1=n2+2^k*z,       0<=n1,n2<N.

The integer z takes only H=O(D_i/2^k) values. A pair moment expands as

    n1^b1*n2^b2
      =sum_(h=0)^b1 binom(b1,h)*2^(k*h)*z^h*n2^(b1+b2-h).

Writing j=b1+b2-h, the original divisibility weight b1+b2+2 is therefore
strengthened to j+(k+1)*h+2 for these terms. Reduce powers of z modulo its
monic integer vanishing polynomial over its finite possible range. Only
H difference degrees remain. The remainder coefficients are integers, and
replacing an old degree h by a smaller degree ell cannot weaken the claimed
bound j+(k+1)*ell+2.

Consequently there are at most H modes of each weight per active tile. With
T=O((M/D_i^2)*2^(k+1)) tiles, this gives

    T*H=O(M/D_i)

modes per weight. Ordinary partition counting now supplies the upper bound

    exp(O(sqrt((M/D_i)*p_i)))=exp(O(M/sqrt(u))).

This is an exact finite-support mode rewrite followed by a loose counting
bound. The needed polynomials and remainder operations have polynomial
description and arithmetic bounds in D_i and p_i, which are polynomial in
the source's global size parameters. It restores the one-axis counting
envelope in this scoped step. It does not prove the full recursive algorithm
or rule out a stronger use of the increasing difference-degree weights.
Additional weighted compression, and its recursively preserved entry costs,
remain open here; the raw occupancy penalty is not a universal lower bound.
The independent exact reduction is in
`/workspace/audit/congruence_moments_child.md`.

One can also see why the advertised weights of this particular reduced
index family do not themselves yield the proposed stronger count. At the
source root a=2u and odd level i=0, one has D=L, k=u-1,
H=Theta(2^u), and T*H=Theta(M/L) for the dense-feasible design. The maximum
difference-degree offset (k+1)*H is o(L/sqrt(u)). Taking j through a small
constant times L/sqrt(u) therefore gives Theta(M/sqrt(u)) formal modes of
weight at most a small constant times L/sqrt(u). Choosing the constants
small enough, every0/1 subset of these modes fits the source's budget
p_i=Theta(ML/u). The weight-only admissible tuple family consequently has
exp(Omega(M/sqrt(u))) members. This is a statement about retaining these
formal indices and their advertised weight budget. It does not assert
linear independence, absence of cancellation or nonzero contribution of
every tuple for a given contraction. Another basis or a code-aware
aggregation can still change the family being enumerated.

## 5. Repacking separately at each level needs a common-family bridge

Imposing repetition only for the level currently sampled avoids the factor
2^u loss locally: each such packet can retain half of its two-axis slots.
But rebuilding the packet embedding usually changes the polynomial matrix
W(T1,T2), hence changes F(T1,T2). All these deformations agree with the
original contraction at the origin; that fact alone does not allow their
off-origin values or jets to be combined by one Hermite formula.

For a basic diagnostic, separate node-dependent polynomials

    G_t(T)=(1-T/t)^q

all have origin value1 and q zero jets at their own nonzero node t. Treating
those zero jets as data from one common polynomial would incorrectly return
zero. A shared origin is weaker than shared interpolated jets.

There is a valid isomorphism operation that must be distinguished from this
change. If P is a port permutation and both the tensors and kernel are
renamed consistently, using

    W'=P*W*P^(-1),

the deformed contraction is unchanged by a change of summation variables.
This does not justify resetting the renamed kernel to a new standard
product-Cauchy matrix based on its new physical slot differences. An
arbitrary port permutation need not preserve those differences or the
packet block structure. The new standard kernel is generally a different
matrix polynomial. Keeping the conjugated old kernel preserves the common
F, but its desired new tile expansion and mode bound must then be proved;
they do not follow from the new names of the ports.

A constructive loophole remains possible. One could prove an invariant
common deformation, or a congruent-jets bridge showing that each repacked
local computation agrees with its jets at the precisions actually used.
Another possibility is a code-aware or interpolation-aware packet change
whose degree and denominators remain charged. Ordinary Hermite recovery
does not provide any of those bridges automatically, and arbitrary
repacking is not such a proof. No impossibility claim is made here.

## Hypothesis status

| Statement | Status | Exact scope |
| --- | --- | --- |
| Matched-level nodes can recover the origin at precision order MD_i/u | Proved from source D.5 | Restrict to diagonal G(T)=F(T,T); scaled jets integral; source precision inheritance retained |
| Original h=ceil(3M/u) and displayed precision constants suffice | Proved for u>=3 | Uses actual total multiplicity, not only the odd-node lower bound |
| Full matched-level rectangular data are exactly unisolvent | Proved algebraically | Odd grid alone suffices; no claim about equally cheap full coefficient recovery |
| One selected repetition bit permits D^2/2 physical slots | Proved | Direct repeated-slot interpretation |
| One fixed packet satisfying all u repetition bits retains L^2/2^u slots | Proved | Fixed axis orientation conventions at all retained levels |
| The direct dense-feasible packet design has mode density M/D_i^2 | False uniformly | Its raw per-pair-index density instead has factor2^(u-i) |
| Congruence moments can improve the raw factor | Constructive scoped rewrite | Monic finite-support reduction supplies the old exp(O(M/sqrt(u))) envelope |
| A better basis yields the proposed M/u^(2/3) exponent | Open here | Increasing weights, exact coefficients and recursive entry costs still need proof |
| Independent level-wise repacking supplies jets of one F | Not established | Needs an invariant/common-family or congruent-jets bridge |
| These ingredients establish an improved contraction or ECDLP algorithm | Not established | Packet geometry, mode compression and recursive invariants remain missing |

The useful positive result is genuine: matched-level origin recovery has no
intrinsic Cartesian-node or precision obstacle of the proposed kind. The
remaining gap is the compatibility of that common deformation with a dense
simultaneous packet geometry, or a proven way to change geometry without
changing the needed jets.

# Projector extraction, moments and the sparse-support barrier

## Exact finite phase algebra

Use the preceding implicit transcript over caller-certified prime r, with
h=(r-1)/2, m|h, L=h/m and epsilon in{1,-1}. Its candidate images lie on
H={y in F_r : y^L=epsilon}. There are exactly L distinct nonzero roots:
2L divides r-1 and r does not divide L. Thus evaluation identifies
R=F_r[Y]/(Y^L-epsilon) with the product of L copies of F_r.

Let E_i be all equations of the implicit three-mode circuit. In R construct

    I(Y) = product_i (1-E_i(Y)^(r-1)).

Fermat's identity makes each factor1 where E_i vanishes and0 elsewhere.
Therefore I is exactly the indicator of the common image-root set S in H,
and I^2=I. Binary powering and cyclic sparse multiplication compute its
canonical representative without enumerating H or unknown scalars. This
representation may nevertheless become dense; the implementation refuses
when its declared caps are reached and does not silently switch algorithms.

The problem wrapper constructs L and epsilon from the same r,m,sign used to
build the transcript, so a smaller unrelated phase cannot be substituted.
The retained transcript has the exact support established in its predecessor.

## Trace moments and annihilator

Write I=sum_(e=0)^(L-1)c_e Y^e. For the quotient trace,
Tr(Y^j)=0 unless L divides j, when it equals L*epsilon^(j/L).
Since I is an indicator,

    d=|S|=Tr(I)=L*c_0,
    p_j=sum_(y in S)y^j=L*epsilon*c_(L-j), 1<=j<L,
    p_L=L*epsilon*c_0.

The field representative of d is its ordinary integer value since0<=d<=L<r.
Reject a value above L as an arithmetic invariant failure. After checking the
output-degree cap, use p_1,...,p_d and Newton identities to recover the monic
annihilator q=product_(y in S)(Y-y). In descending coefficients a_0=1,

    a_j=-(1/j)*sum_(i=1)^j a_(j-i)*p_i.

All j<=d<r are invertible. This gives the exact candidate polynomial without
candidate scanning. If degree1, its root y=-q_0 feeds the earlier rational
phase k=N(y)/D(y), followed by independent public-point verification.
Higher degree or an unanchored singleton remains unresolved in this adapter;
there is no uncharged factoring/search fallback. Identity and observed zero
endpoints retain the prior verified public exits.

## Why a sparse indicator cannot hide a singleton

Let s be the number of nonzero coefficients of nonzero I and d=|S|.
Multiplication by I on R has rank d, because evaluation diagonalizes it with
exactly d nonzero entries. Its L columns are Y^j I mod(Y^L-epsilon).
Each column is a cyclic shift of I with nonzero wrap scalings, so has exactly
s nonzero entries. Choose d columns spanning the image. Every coordinate
occurs in some column (all cyclic shifts of a nonzero I cover all coordinates).
It must therefore occur in a basis column as well, since a linear combination
cannot introduce a coordinate absent from all its summands. The union of the
d basis-column supports has size L, proving

    s*d >= L.

This direct finite-field argument does not use complex positivity or a
characteristic-zero uncertainty bound. For a singleton y, the formula is

    I(Y)=(1/L)*sum_(e=0)^(L-1)(Y/y)^e,

and every coefficient is nonzero. Consequently a sparse coefficient cap below
L cannot represent a singleton projector, regardless of its construction.
The lower bound concerns this explicit indicator representation, not all
implicit root extractors or rational/moment representations.

Sources: Chen, Fan and Luo, arXiv2211.11204v3 (2025), introduction equations
1.2/1.4 and the split finite-field discussion, https://arxiv.org/pdf/2211.11204.
Their broader support/rank framework motivates this specialization; the direct
cyclic-column argument above states all assumptions needed here. No novelty
claim. The initial requested HTML version was unavailable; the PDF was read.
Repository sources are feature-implicit-oriented-circuit at75aaec57022c.
KB unavailable: MCP absent and wrapper reports no executable.

## Bounded implementation and cost scope

Sparse maps retain canonical exponents0..L-1. A product wraps exponents with
Y^L=epsilon. Before multiplication, the full number of monomial pairs must fit
the remaining pair budget; the counter records pairs actually entered. Each
coefficient update is counted, zero coefficients removed, and new entries are
refused before exceeding the per-map term cap. Conservative refusal can happen
on a temporary accumulation even if later cancellations would reduce support.
The retained sum of circuit-node coefficient slots has a separate cap. These
caps do not include map metadata, simultaneous temporary maps, allocator
capacity, circuit nodes or source memory and are not process-RAM limits.

Stats distinguish monomial pairs, coefficient updates, retained node slots,
maximum temporary map support and the limit that refused work. Wrapped
coefficient multiplication, constant generation, hashing/tree lookups, copies,
allocation, exponent loops, Newton arithmetic/inversions and point verification
are additional work; pair counts are not total operation counts. Source API
calls exclude source setup/calibration and internal arithmetic, all still charged.

The final candidate polynomial can be small while intermediate node values
and its indicator are large. No timing or total-work ranking is inferred.
The public adapter consumes source/curve/base/target, not a scalar or supplied
image. The same certified matching-source contract as the predecessor applies.

## Next extraction opportunity

A singleton needs only two moments to identify its image, despite its L-term
indicator. Therefore investigate direct trace/coefficient queries on the
implicit circuit, without materializing I. This is a new construction task,
not a capability supplied by the present extractor. Ordinary low-degree
truncation is invalid in the cyclic quotient: Y^(L-1)*Y=epsilon contributes to
the constant coefficient even when both inputs have zero constant terms.
The checks retain this wraparound counterexample explicitly.

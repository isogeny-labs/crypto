# Three-adic precision review for the proposed ternary packet variant

2026-10-30 UTC. Reviewer `/root/tensor_precision_review`. Pure mathematical
scrutiny of the supplied construction, compared with Appendix D.4 of the pinned
October25 manuscript. No implementation, CPU experiment, timing, ECDLP solver,
novelty certificate or improved-exponent claim is supplied.

## Verdict and indispensable correction

The proposed valuation, Cauchy-integrality and tapered-Hermite constants check
out. After the separately checked ternary packing/locality construction, the
weighted-mode argument also transfers, **provided modes are split by target
group**. A source tile can cross to two different target groups. The correct
bound is at most18M/D modes of each weight, not9M/D. This changes a fixed
constant and preserves the stated exponent class. I found no fatal local
precision obstruction after that correction.

The label field and coefficient ring are separate objects. This note uses the
latest proposed label field F67. The packet review's F64 version works as well:
both have characteristic different from3 and support the stated order-three
linear action. The coefficient localization in the present variant has residue
characteristic3. Boolean edge indices remain0/1; ternary slot digits do not
change the tensor network's port alphabet.

## 1. Grid, derivative valuations and integral Cauchy charts

Put L=3^a, S={-L,...,2L-1}, alpha=3^(a+1), and
C(T)=prod_(t in S)(T-t). S is a complete residue system modulo3^(a+1).
For any t in S, the nonzero differences t-q, q!=t, have valuation k on exactly
2*3^(a-k) positions, for k=0,...,a. No nonzero difference is divisible by
3^(a+1), because its absolute value is smaller than3L. Consequently

    v3(C'(t))=sum_(k=1..a) 2k*3^(a-k)

is independent of t. This includes the asymmetric endpoints of the chosen
integer interval; no translation-invariant integer interval assumption is
needed beyond its complete residue property.

Every slot difference delta=y-x lies in[-L+1,L-1] and hence in S. Thus
W_delta(T)=C(T)/(C'(0)(T-delta)) is a polynomial of degree3L-1. At T=0 the
diagonal values equal1 and every off-diagonal value retains an uncancelled
factor T, establishing W(0)=I on real ports.

For a local expansion at t, each factor t-q+alpha*z with q!=t has
coefficientwise content3^v3(t-q). These contents cancel the3-part of C'(0).
The remaining factor alpha*z makes C(t+alpha*z)/C'(0) divisible by
3^(a+1). Omitting the delta factor has two cases:

- If delta=t, it removes alpha*z and leaves an integral polynomial.
- Otherwise it removes content at most3^a, so at least one factor3 remains.

Therefore W(t+alpha*z) is integral over O3 and its nonlocal corrections have
the required positive valuation. Evaluation must cancel those factors first;
naive evaluation of the uncancelled rational expression can create artificial
0/0 at a root.

One may take O3=Z_(3)[U]/(f), with f monic and irreducible modulo3, using
coefficients0,1,2 below its leading coefficient. The minimum-coordinate
valuation is multiplicative on its fraction field because reduction after
scaling is a field. Scalar residues nonzero modulo3 are units, by the same
multiplication-matrix argument as in D.4.1. On O3[V]/(V^ell), divisibility is
coordinatewise; only lower-bound multiplicativity is needed for products.
An extension degree e=4ceil(log_3 s), for example, supplies at least s^4
residue-distinct interpolation points with unit pairwise differences. Its
construction and exact arithmetic remain polynomial in the stated parameters.

## 2. Multiplicities and exact high-valuation counts

Let1<=u<=a, M>=u, h=ceil(2M/u), and q_i=h*max(0,u-i). For a nonzero node
t with i=v3(t), put D=L/3^i. There are2L unit nodes. Hence

    sum_(t!=0) q_(v3(t)) >=2L*h*u>=4ML>M(3L-1).

This exceeds the degree of F. Also h<=3M/u and every q_i<=3M.

For j=1,...,a-i, there are exactly2D/3^j nodes q with v3(q)=i+j and the
same number with v3(t-q)=i+j. The latter nodes have valuation i themselves.
Nodes of valuation below i have v3(t-q)=v3(q) and cancel. Nodes with both
valuations equal i contribute zero. This last class can occur in residue
characteristic3 and must not be silently identified with the binary case;
its contribution is nevertheless zero.

For the normalized Hermite factor E_t, the resulting identity is exactly

    v3(E_t(0))
       =2D sum_(j=1..a-i) [j/3^j]*(q_(i+j)-q_i)
       >=-2Dh sum_(j>=1) j^2/3^j
       =-3Dh.

Here q_(i+j)-q_i>=-hj remains true after the taper reaches zero, and
sum j^2/3^j=3/2. No extra factor3 is missing.

The i=a boundary consists of the nonzero grid nodes t=-L and t=L. Since
u<=a, q_a=0, so neither has an active jet. Their zero multiplicities cause
no loss in the product/interpolation count. The displayed valuation identity
has an empty higher-level sum there and is consistent. In particular the
construction never needs to group by a nonexistent slot coordinate a.

## 3. Hermite recovery and the stated safe precision

The exact Hermite CRT formula from D.9 works over the characteristic-zero
fraction field of O3 with nilpotent V coefficients. Each local factor
E_t(t+alpha*z) is integral with constant term1. Its reciprocal truncated at
q_t is integral, so quotient formation costs no further3-adic digits.
The truncation is in z, independent of the coefficient-ring variable V.

The evaluation point has v3(-t/alpha)=i-a-1. Evaluating a polynomial of
degree below q_t<=3M loses at most3M(a+1) powers of3. Combining that
with the lower bound -3Dh above, an input-coefficient error at precision

    p_t=p+4hD+8M(a+1)

contributes an error of valuation at least

    p_t-3hD-3M(a+1)=p+hD+5M(a+1)>=p.

The proposed precision is therefore conservative. Summing errors does not
lose more digits. The reconstructed sum differs from the integral F(0) by
an element of3^p A, allowing the powers of3 in the exact rational common
denominator to be cancelled before modular reduction. The factors have
O(a) bits and polynomially bounded multiplicities/counts, so the same
polynomial-bit rational-arithmetic argument applies.

Canonical creation-precision lifting must be kept exactly as in the source:
compute an old tensor's final entry modulo its fixed pc, then lift those
coordinates. Recomputing the formula at a larger precision, or lifting its
intermediate factors before recombining, is not generally equivalent. The
construction needs only an entrywise congruent lift, not a lift homomorphism.

## 4. Locality, tiles and the corrected modes

The independently checked packing chooses the component
c=t/3^i modulo3, c in{1,2}. The coordinate action is S on the first
pair-label component and S^-1 on the second, so shifting a slot by leading
digit c makes its column label equal the corresponding row label in
component c. Thus W(t) is local.

For a nonlocal row and a tile y=y0+3^(i+1)j, locality implies
v3(t-y0+x)<=i. Otherwise that tile has the slot's prescribed local leading
digit and hence its selected group. Adding alpha*zeta, with zeta integral,
does not change this denominator valuation. The geometric expansion has
leading fraction valuation at least a+1-i>=1 and ratio valuation at least1.
Its bth mode coefficient is therefore divisible by3^(b+1). The omitted
geometric tail is zero modulo3^p_t.

Each tile has one source symbol but up to two distinct crossing target
symbols. Introduce a separate formal mode for each target. These modes may
share the same numerical source linear form; substituting that form after
the separate coefficient extractions is valid. Each mode then has exactly
one source and target, restoring the factorization in the analogue of D.16.

With at most3M/L packets and3^(i+1) tiles per packet, the count per weight is

    2*(3M/L)*3^(i+1)=18M/D.

Source substitution is integral, so monomials of total weight at least p_t
vanish. The colored-partition bound consequently remains

    exp(O(sqrt(M*p_t/D)))=exp(O(M/sqrt(u))).

Target splitting adds no Boolean ports. The child edge count and packet
degree bounds are unchanged; only a constant in the number of mode choices
and polynomial descriptor data increases.

## 5. Parameter absorption and domain limitations

For u_d=floor(epsilon*log2(s)/2^d), a_d=2sum_(j<=d)u_j and epsilon=2^-20,
one has a_d<=4epsilon*ceil(log2 s). With F67 labels,

    L=3^a, D_arr=67^(4a)

are small fixed powers of s: their exponents are bounded by
4epsilon*log2(3) and16epsilon*log2(67), both strictly below1/1000.
The child bound6M/67<M/8 gives the same active-depth range. At an active
node, u>=lambda^(4/5) eventually. For i<u,

    D=3^(a_d-i)>=3^(a_(d-1)+u_d).

The extra3^u absorbs inherited polynomial-logarithmic precision and the
M(a+1) evaluation term. Thus p_t=O(MD/u) and polynomial precision along
the path are consistent. These estimates require an effective sufficiently
large range, with the finite remainder handled separately as in the source;
they do not assert practical parameter admission.

The direct coefficient-domain conclusion is contraction over
(Z/3^P0 Z)[V]/(V^ell). It is not automatically a theorem about arbitrary
input entries modulo2^P0. For Boolean0/1 counting, choosing3^P0>2^M
recovers the nonnegative integer count exactly using P0=O(M), so that
application has a straightforward modulus choice. Converting arbitrary
mod2 tensor entries by integer lifting can require a larger precision
bound and must be accounted for separately.

This review supports the proposed local3-adic adaptation after its mode
correction. It does not prove that the adaptation is novel, improve the
source exponent class, certify a complete implementation, or supply a
distinct ECDLP algorithm meeting the parent task.

# The p+1 selector, the line FFT and its ECDLP boundary

All deductions in this note are producer mathematics, not Lean or signed PCC
verification. Established source facts are cited; no novelty is claimed.
[pcc:unverified]

## 1. A completely split squarefree selector

Let `p` be odd, `B=2^k`, and suppose `4B` divides `p+1`. Write

`T = {z in F_(p^2)^* : z^(p+1)=1}`.

This cyclic norm-one group has order `p+1`. For `z in T`, the trace
`X=z+z^(-1)` belongs to `F_p`. The Dickson identity

`D_n(z+z^(-1),1)=z^n+z^(-n)`

and composition `D_2(X,1)=X^2-2` imply that the roots of `D_B(X,1)` are
the traces of elements of exact order `4B`. There are `2B` such elements,
paired by inversion, and none equals its inverse. Hence the selector has
exactly `B` distinct roots, all in `F_p`. See Bluher, *Permutation properties
of Dickson and Chebyshev polynomials*, Section 2 and Theorem 6.1(ii):
https://arxiv.org/pdf/1707.06877. [pcc:unverified]

For `p=2^31-1`, this holds for `0<=k<=29`. Write `X_0=X` and
`X_(j+1)=X_j^2-2`; membership is exactly `X_k=0`. The polynomial is squarefree:
its derivative is `2^k product_(j=0)^(k-1) X_j`. If an earlier `X_j` were zero,
the subsequent orbit would be `-2,2,2,...`, so it could not end at zero.
[pcc:unverified]

The common substitute `D_B(X)-2` behaves differently. For even `B=2n`,

`D_(2n)(X)-2=(X^2-4) E_(n-1)(X)^2`,

where `E` is the second-kind Dickson polynomial. Its subgroup trace set has
`B/2+1` distinct elements, with double interior roots. Also `D_B(X)+2` is a
square for even `B`. A regular trace coset `D_B(X)-c`, with `c^2!=4` and a
fully split fibre, or the present zero fibre, avoids this multiplicity.
[pcc:unverified]

## 2. Exact circle-domain correspondence

For M31, write `i^2=-1` in `F_(p^2)`. The map

`z=a+i b <-> (a,b), a^2+b^2=1`

identifies `T` with the circle group. Choose `zeta` of order `4B`. The full set
of odd powers of `zeta` is a canonical coset of a subgroup of order `2B`.
Its `2B` circle points project two-to-one onto `B` values of `a`, and

`V_k = {2a : (a,b) lies in that coset}`.

The circle squaring map is `(a,b)->(2a^2-1,2ab)`. With `X=2a`, its projected
map is exactly `X->X^2-2`. Consequently, the univariate line transform used
here is the inversion-invariant half of the Circle FFT on that `2B`-point
canonical coset. Its maximum M31 domain size is `B=2^29`, while the full
circle coset has size `2^30`. [pcc:unverified]

The S-two Whitepaper, Lemma 2, Definition 3, Theorem 5 and Appendix B, gives
the group, canonical cosets, basis and butterflies:
https://eprint.iacr.asia/2026/532.pdf.
The correspondence above uses `X=2a`; it is not a transfer of the elliptic
curve group law to the circle. [pcc:unverified]

## 3. The precise basis and transform

Set `d_j(X)=D_(2^j)(X,1)`. For `i` with binary bits `i_j`, define

`b_i(X)=product_(j=0)^(k-1) d_j(X)^(i_j)`.

Each `b_i` is monic of degree `i`, so these are a basis for polynomials of
degree below `B`. In particular, `b_(2i)(X)=b_i(D_2(X))` and
`b_(2i+1)(X)=X b_i(D_2(X))`. If `f=sum c_i b_i`, splitting the coefficient
vector by its low bit therefore gives

`f(X)=E(D_2(X))+X O(D_2(X))`.

For each `t in V_(k-1)`, let `x,-x` be its two preimages in `V_k`. Then

`f(x)=E(t)+x O(t)`, `f(-x)=E(t)-x O(t)`.

One field multiplication and two additions produce each pair. Conversely,

`E(t)=(f(x)+f(-x))/2`, `O(t)=(f(x)-f(-x))/(2x)`.

Every `x` used as a twiddle is nonzero. Recursing until `V_0={0}` yields the
claimed transform. The forward recurrence is
`M(B)=2M(B/2)+B/2`, `A(B)=2A(B/2)+B`, giving
`M(B)=(B/2) log2 B` and `A(B)=B log2 B`. These counts exclude domain setup,
twiddle inverses and any conversion from ordinary monomial coefficients.
The normalized inverse here uses two multiplications per pair. The usual
scaled inverse replaces each pair by `(sum,difference/x)` and normalizes by
`1/B` once at the end. [pcc:unverified]

## 4. Quotient multiplication and genuine component scope

Because the selector is split and squarefree,

`A_k=F_p[X]/(D_B) ~= product_(x in V_k) F_p`.

Transforming two coefficient vectors, multiplying their values pointwise,
then interpolating computes their product modulo `D_B`. This has
`O(B log B)` field operation cost and `O(B)` transform storage in the
hierarchical coefficient representation. Exact unreduced multiplication of
two degree-below-B polynomials can instead use a supported domain of size
`2B`, keeping representation conversion explicit. [pcc:unverified]

This primitive can accelerate a dense quotient product, a batch of polynomial
evaluations, or interpolation on the full leaf fibre. A proof-of-concept
arithmetic equality does not establish that those operations dominate an
existing sparse F4 or elimination workload, nor that this implementation beats
an established optimized polynomial package. Those are separate measurements.
[pcc:unverified]

## 5. Why the elliptic relation remains the hard part

On `E:y^2=x^3+A x+B0`, a trace root need not lift to a rational point.
The value `x^3+A x+B0` must be a square, and subgroup and sign checks still
apply. One may store a liftability mask and a chosen y-value array on the
**full** `V_k` and transform them, but their coefficient representations can
be dense. An irregular selected subset need not itself support the uniform
halving tower. [pcc:unverified]

Likewise, elliptic pair sums need not have a Dickson-fibre support. The actual
repository note `iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`
under `research/prime31_summation_compression_20261122/` gives an explicit
regular F7 fibre whose pair-support coordinate algebra has nonuniform `D_2`
fibres. The sibling `torus_rotation_followup/NO_SINGLE_COVER_FFT.md` gives a
restricted no-lift theorem for torus shifts on one genus-three elliptic-lift
cover. Neither note proves a lower bound for all future succinct algorithms.
[pcc:unverified]

For `m` leaves the unfiltered tensor quotient has dimension `B^m`. A tensor
transform costs `O(m B^m log B)` and produces `B^m` values. It can reduce
arithmetic overhead while leaving exhaustive search size unchanged. The
univariate primitive here does not materialize that tensor. [pcc:unverified]

The torus multiplication trace equation

`x^2+y^2+z^2-xyz-4=0`

describes traces of `u,v,uv`; it is not the elliptic `S_3` equation. More
fundamentally, on an odd-prime-order elliptic subgroup of order `r`, every
group homomorphism into the M31 norm-one group is trivial: its image order
divides both `r` and `p+1=2^31`. This rules out that direct scalar-preserving
transfer, while permitting the coordinate-selector use above.
[pcc:unverified]

## 6. Conditional complete-cost threshold

Let `b` count negation-folded rational base points; full signed size is about
`2b`, and the unfiltered selector size `B` is typically comparable to `b`,
with the actual count measured. Under a random-relation heuristic and fixed
arity `m`, a trial has success probability on the scale of `b^m/r` when that
ratio is small. If a complete trial costs `C(b)` and enough useful independent
relations are obtained after order `b` successes, a basic sparse-linear-algebra
model gives

`T ~ r C(b)/b^(m-1) + b^2`,

up to fixed-arity and logarithmic factors, plus setup and recovery. This is a
conditional planning model; usefulness, rank, signs and unsuccessful trials
must be counted. See PKM16, Section 3.2, for its general relation/linear-algebra
accounting, without importing its unresolved solver cost as a bound:
https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf.
[pcc:unverified]

For four summands and the fixed small-base choice `b ~ r^(1/5)`, relation cost
is `r^(2/5) C(b)`, so beating `r^(1/2)` in this model requires
`C(b)=o(r^(1/10))`, as well as the other phase gates. If instead one assumes
`C(b)=b^alpha` and optimizes the base size, balancing terms gives
`b~r^(1/(5-alpha))` and exponent `2/(5-alpha)` in the unsaturated regime.
Thus `alpha<1` is the optimized four-summand threshold in this simplified
model. The fixed-base and optimized-base thresholds are different questions.
The line FFT proves neither condition for an elliptic relation trial.
[pcc:unverified]

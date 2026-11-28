# Why pool balance changes the exponent

This is a producer derivation for an abstract cyclic subgroup, not a signed
formal proof or a novelty claim. The finite executables retain explicit size,
field-width and retry caps. Standard unbalanced meet-in-the-middle already
matches classical baby-step giant-step time/storage; Pollard rho has the same
square-root time exponent with much less storage. The following explains the
scaling control and its correlation assumptions. [pcc:unverified]

The analysis assumes ideal independent uniform samples; deterministic MT19937
seeds in the finite executable are a reproducibility mechanism, not a certificate
of that ideal sampling assumption.

Let the subgroup have odd prime order r, P generate it, and Q=kP be the given
public point. Draw six independent uniform mixed labels in F_r². They define
three offsets U_h and three steps V_h. Before exceptional-point filtering, pool
h consists of U_h+i V_h for 0≤i<n_h. Constructing the actual points costs six
charged double-scalar combinations and O(n_1+n_2+n_3) group additions, using
projective steps and batch affine conversion. Labels advance modulo r; their
construction needs no k.

For each (i,j,l) and signs s,t in {−1,+1}, consider

    U_1+iV_1 + s(U_2+jV_2) + t(U_3+lV_3) = O.

The coefficient vector on the six mixed labels is (1,i,s,sj,t,tl). Its alpha
and beta linear forms A,B are independent uniform field elements. An
informative relation has A+kB=0 and B≠0, probability (r−1)/r². Two different
triples/sign choices have nonproportional coefficient vectors: the first
coordinate forces the proportionality scalar to1, then the signs and indices
must agree. Thus their alpha/beta pairs, and informative-event indicators,
are pairwise independent, provided every n_h<r.

For the number X of informative relations in this unfiltered model,

    lambda = 4 n_1 n_2 n_3 (r−1)/r²,
    P(X>0) >= lambda / (lambda + 1 − (r−1)/r²).

This is the elementary second-moment bound. It does not assume the pool
points are iid. Choosing n_1=n_2=m≈r^(1/4)/sqrt(c) and n_3=C≈c sqrt(r) gives
lambda≈4 and a constant success bound before filtering.

Exceptional points do not silently disappear. A zero step among the three
initial draws has probability at most3/r. Given nonzero steps, within one
progression x-collisions can only come from opposite points, since all indices
are below r. They require 2U_h+(i+j)V_h=O; at most2n_h distinct sums occur.
Infinity contributes at most n_h/r. Between two pools, a union bound for
same/opposite points gives at most2n_h n_j/r. Consequently the probability
that any nominal point needs rejection is bounded by

    [3 + 3(n_1+n_2+n_3) + 2(n_1 n_2+n_1 n_3+n_2 n_3)] / r.

For these unbalanced sizes this is O(r^(−1/4)). On the complement, the actual
filtered generator equals the nominal progressions. Subtracting the bound
from the second-moment success bound preserves constant success for growing
r. The implementation nevertheless charges every resample, rejected point
and preceding failed attempt, and retains four-attempt censors.

A complete pair scan uses O(m²) field operations/inversions in the usual
unit-cost group/field model, including both signs and witness recovery; third
pool construction, validation and indexing cost O(C). Independent scalar
verification and public checks add O(log r) group operations plus field/order
primality costs. Thus the abstract uncapped algorithm has expected
O(m²+C+log r)=O(sqrt(r)) main group/field work and O(m+C)=O(sqrt(r)) storage,
with standard randomized hashing for the expected lookup bound. This is a
known generic time–memory tradeoff, not sub-square-root ECDLP. Bit complexity,
primality, inversions, allocation, cache effects and finite hash behavior must
still be charged in the actual runtime comparison. A fixed deterministic
hash does not prove an adversarial expected-time bound.

In contrast, fixing n_1=n_2=B,n_3=8B and r≈8B³ keeps nominal density comparable
but leaves a full pair scan of order r^(2/3). Early termination affects measured
constants; it does not make a worst-case quadratic scan subquadratic. The
explicit-size experiment separates this shape choice from Barrett/container
engineering and records actual density, misses and memory for each shape.

Sources checked (KB unavailable): pinned crypto/cryptanalysis pair/quotient
assessment; current prime_fast.rs stride-start and meet-in-the-middle source;
retained affine group and pair controls. The input curve is given in the DLP
problem. No algorithmic auxiliary curve, isogeny transfer or structured-base
labels are supplied free by this argument. `test-incidence-moments.cpp` checks
finite coefficient ranks and exact unfiltered event moments; it is a sanity
check of this derivation, not a general proof certificate.

# A p-rank-zero bound for three-branch cyclic-11 covers

## Statement and scope

Let `p` be a prime with `p mod 11` in `{3,4,5,9}`. Every smooth
geometrically cyclic degree-11 cover `C -> P1` with exactly three
geometric branch points in characteristic `p` has genus five and
**geometric p-rank zero**. This holds for all branch positions and
forms of the cover, since after base change to an algebraic closure the
three points can be sent to `0,1,infinity` and the cover has a Kummer
equation `y^11=x^a(x-1)^b` with `a,b,c=-a-b` nonzero modulo 11.

Consequently, for any finite field `k` of this characteristic and
degree-five extension `K/k`, a genus-five carrier in this class cannot
receive a nonzero algebraic GHS transfer from an **ordinary** elliptic
curve `E/K`: `Res_(K/k) E` is ordinary of dimension five, whereas
`J(C)` has p-rank zero. The statement does not exclude supersingular
source factors, other curve genera, other branch counts, noncyclic
covers, or correspondences without this carrier.

## Proof and finite certificate

The genus is `(3-2)(11-1)/2=5`. The cited signature formula of
Ozman--Pries--Weir, Lemma 2.1, gives for `i=1..10`

`s_i=-1+sum_(e in {a,b,c}) < -i*e/11 >`.

Each `s_i` is zero or one, and the sum is five. Since `ord_11(p)=5`,
Cartier/Frobenius permutes the nontrivial characters in exactly two
orbits, `R={1,3,4,5,9}` and `N={2,6,7,8,10}`. Their published
p-rank bound (Theorem 2.3, attributed to Bouw) is

`f_p(C) <= 5*min_(i in R) s_i + 5*min_(i in N) s_i`.

The source-bound `orbit_bound.py` checks all 90 valid `(a,b)` rows:
each orbit contains an `s_i=0`, so both minima and the right side are
zero. There are 20 distinct signature patterns. `OrbitBound.lean`
checks the same 90-row modular condition and five-dimensional
signature sum without axioms. The external geometric normalization,
signature formula, and p-rank theorem are cited mathematical inputs;
Lean does not formalize them.

The final transfer exclusion follows from p-rank under isogeny.
An ordinary `E/K` gives an ordinary Weil restriction after base change
to an algebraic closure, while any nonzero homomorphism from it to
`J(C)` would have a positive-dimensional ordinary image. This is
incompatible with the target's p-rank zero.

## Prior work and limits

The cyclic-cover signature formula and p-rank upper bound are published
results, so this is a finite specialization and a GHS eligibility
corollary, not a new attack method. A targeted literature search did
not establish whether this exact order-five, degree-11 exclusion was
previously written down. It should be called **new to this project,
publication priority unresolved**. The finite F3 and F5 point-count
screens separately exclude the remaining supersingular elliptic trace
possibilities in their specified rational Kummer rows; they do not
make the global p-rank statement an all-prime vulnerability
classification.

Primary source: Ozman--Pries--Weir, *The boundary of the p-rank 0
stratum of the moduli space of cyclic covers of the projective line*,
Nagoya Math. J. 248 (2022), Lemma 2.1 and Theorem 2.3:
https://doi.org/10.1017/nmj.2022.12 .

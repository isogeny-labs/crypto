# Proposed seven-summand coverage for Dickson coordinate bases

P-61 / `icv1-fp61-tm813574577-a3b71255` · 27 November 2026

**Standing:** This is a proposed, unrefereed mathematical adaptation of
[Kaarelson v2 §5](https://github.com/kaarelkaarelson/fast-ecdlp/blob/main/v2/paper.pdf).
It is not a Lean proof or a verified PCC theorem. The finite arithmetic is
replayed by `p61_audit.rs` and `p61_bounds.cpp`; those programs do not prove
the general argument. No decomposition oracle or ECDLP speedup follows.
[pcc:unverified]

## Curve and selector

Let p=2^61−1 and E/F_p be y²=x³−3x+b, b=0x54aa66b458b77ec.
Its prime group order is r=2305843010027268529. Let T be the 1,024
distinct roots of D_1024(x)=0 in F_p. They are nonzero. For u∈F_p*, write
E_u:y²=x³−3u⁴x+u⁶b. Let F_u be all signed rational points on E_u
whose x coordinate belongs to the SAME fixed T, and let f(u)=|F_u|.
[pcc:unverified]

## Proposed claim: averaged seven-sum coverage

For any prime p>3, integer m≥2, nonsingular E:y²=x³+Ax+B with AB≠0,
any set T⊂F_p* of h distinct coordinates satisfying 12m≤h≤√p,
and any κ≥1,
define the following quantities. Here B is the curve's constant
coefficient; B=486 below is the separate factor-base abscissa count.

  Θ_m=(2m−1)!! 4^m,   D'=(2m−1)2^(2m−2),
  μ=Θ_m h^m + 2·4^m D' h^(2m)/(p−1),
  V=h+h(h−1)(11 ceil(√p)+5)/(p−1).

Subject to the proof sketch below, at least 1−1/κ−4V/h² of the isomorphic models E_u have f(u)≥h/2
and m-fold additive energy E_m(F_u)≤κμ. On each such model, the
fraction of all E_u(F_p) targets admitting a generic m-term signed
decomposition is at least

  δ = h^(2m) / (2^(2m+2) κ μ #E(F_p)).

This is a proposed bound, subject to the proof obligations below.
[pcc:unverified]

Here *generic* is Kaarelson v2 Definition 3.1: every previous partial sum
is affine and has x-coordinate distinct from the next summand. Thus every
addition is a chord; doubling, cancellation, infinity partial sums, and a
final target at infinity are excluded. [pcc:unverified]

## Proof sketch with scope of each input

1. Since AB≠0, for distinct t,t'∈T the ratio t'/t is never an
   exceptional square-equivalence ratio of ℓ(x)=x³+Ax+B. Thus the
   two character sums in the second-moment expansion of f(u)−h
   obey the Weil bounds used in Kaarelson v2, Lemma F. The interval
   shape of T is not used. This gives E_u[(f(u)−h)²]≤V, and
   Chebyshev gives Pr[f(u)<h/2]≤4V/h². [pcc:unverified]

2. For a nontrivial tuple t∈T^(2m), the dilation polynomial
   g_t(s)=S_(2m)(s t_1,...,s t_(2m);A,B) is nonzero. If it vanished
   identically, adjoin Y_i with Y_i²=ℓ(s t_i) to F_p(s) for each
   distinct t_i. The geometric Semaev statement gives signs of these
   Y_i with sum O. Distinct nontrivial Kummer characters follow from
   AB≠0 and the preceding square-equivalence fact. Character
   projection isolates a point P_i with x(P_i)=s t_i and forces it
   to be torsion. A torsion point of a constant elliptic curve over a
   function field has algebraic, hence nontranscendental, coordinates;
   this contradicts x(P_i)=s t_i. No interval assumption enters.
   Its degree is at most D'. [pcc:unverified]

3. Every actual 2m-point sum equality implies S_(2m)=0. Trivial
   coordinate tuples contribute at most Θ_m h^m to the averaged
   energy. Each nontrivial tuple has at most 2D' values of u, giving
   the second term of μ. Markov gives Pr[E_m(F_u)>κμ]≤1/κ.
   [pcc:unverified]

4. On a good model, at most (3m−2)f(u)^(m−1) tuples are nongeneric.
   Since f(u)≥h/2≥6m, at least f(u)^m/2 generic tuples remain.
   Cauchy–Schwarz with E_m(F_u) bounds their target support below
   by f(u)^(2m)/(4E_m(F_u)), giving δ after division by #E.
   [pcc:unverified]

The printed Theorem S in Kaarelson v2 incorrectly asserts that S_n=0
over an arbitrary field K implies K-rational point lifts. For example,
on y²=x³+x+1 over F_7, S_3(1,1,4)=0 but x=1 and x=4 do not lift.
Step 2 needs only the geometric converse after adjoining the square
roots; step 3 needs only the forward implication for rational points.
This is a proposed repair of the field-of-definition issue for the present
argument, not a formal correction to the cited paper. [pcc:unverified]

## Exact P-61 arithmetic conditional on the proposed lemma

With h=1024,m=7,κ=4, exact rational arithmetic gives
Θ_m=2,214,051,840, D'=53,248, μ≈1.05729919044·10^33,
V≈1024.00758846. Thus at least 0.746093721 of scalings are good.
Every good scaling covers at least 5,029 group targets, a fraction
δ≥2.18088033·10^−15. This is an extremely weak *conditional* proof bound,
not a measured yield. [pcc:unverified]
It says nothing about seven-sum coverage for the frozen u=1 model.

At u=1, exact enumeration finds B=486 lifted abscissae. The signed
base A={±P_i} has 972 points, and its pair sumset has exactly
|2A|=2·486²+1=472,393 elements. Cauchy–Davenport in the prime-order
group gives |7A|≥3(|2A|−1)+|A|=1,418,148, or a fixed-model target
hit fraction at least 6.15023657·10^−13. Counting signed seven-leaf
multisets gives the complementary upper bound 0.07207185 for a
uniform nonidentity target. The fixed-model lower bound counts
unrestricted sums; it does not guarantee the generic partial-sum
conditions of the candidate decomposition oracle. Neither bound
measures actual yield. [pcc:unverified]
The 236,196 nonzero pair abscissae are disjoint from T, so this base
has no signed three-leaf zero sum. Every fold D_(2^j), j=0,...,10,
is injective on the pair abscissae, so none of the selector's ten
direct folds compresses them on this fixed model. This does not
exclude implicit joint elimination.
For B=486, B^(4/3)≈3,821; the selector size q=1024 gives
q^(4/3)≈10,321. Merely scanning the D=236,196 pair abscissae on
each target takes about 62 times the B-based reference or 23 times
the q-based reference, before solving the remaining five-summand
condition. [pcc:unverified]

## Proposed explicit growing-field consequence

Let p_k be the least prime congruent to −1 modulo 2^(k+2).
Xylouris's published Linnik exponent 5.18<7 gives p_k≪2^(5.18k).
Set q_k=2^floor(log_2(p_k)/7). For all sufficiently large k,
q_k≤2^k and q_k=Θ(p_k^(1/7)), so 4q_k divides p_k+1.
The norm-one torus then gives q_k distinct rational roots of D_qk.
The curve y²=x³−3x+1 is nonsingular for p_k>3 and has AB≠0.
Conditional on the proposed lemma, the random-model coverage bound is a positive constant
independent of k for this explicit family. It does not require the
group orders to be prime. An ECDLP algorithm still needs a charged
exact seven-term decomposition oracle for these Dickson selectors.
[pcc:unverified]

## Conditional cube-root algorithm gate

Suppose such an oracle, including failed targets and signed witness
recovery, costs O(q_k^(4/3)) field operations per query, with at most
O(q_k² polylog p_k) setup and sparse linear algebra. A good model
has constant uniform-target success probability, so O(q_k) replayed
relations suffice for a left dependency. Draw u uniformly, cap each
round at a fixed multiple of q_k/δ queries, and redraw u if too few
relations appear; the good-model probability and a binomial tail bound
give an expected constant number of rounds. For a prime-order subgroup,
mixed queries aP+bQ recover d from that dependency without full
factor-base rank. For arbitrary E group order n=rc with a large prime
r>p^(2/3) dividing the public point's order, first project P,Q
to their order-r components. Mix aP+bQ with H=[r]U for uniform U∈E,
then project relations by [c] before taking the left dependency.
Here E decomposes
as its r-Sylow and complementary c-order subgroups, so H is uniform
in the c-order subgroup and the mixed target is uniform in the full
group. Uniform U can be sampled
by rejection from x/sign tokens plus one infinity token, accepting
each actual point with equal probability. Smaller prime
factors can be handled within cube-root group-operation cost by
Pohlig–Hellman and baby-step/giant-step. Indeed, at most one prime
factor can exceed √(2p); if it is at most p^(2/3), its deterministic
square-root cost is at most p^(1/3). All other prime factors have
square-root cost at most (2p)^(1/4) each, and
their total multiplicity is O(log p), which stays below the budget.
Point counting and integer factorization are charged separately. This
conditional reduction assumes that their combined cost fits the same
O(p^(1/3)) field-operation budget; no finite P-61 setup time is claimed.
Under that assumption, the stated seven-sum oracle assumption, and the
proposed coverage lemma, the total field-operation count is O(p_k^(1/3)). A complete
failed-query-inclusive oracle at this rate has not been established.
Bit operations incur polylog factors unless the oracle saves a power.
The field p_k is an input to the attack; searching for the least prime
in its definition is not charged as part of solving that input's ECDLP.
[pcc:unverified]

## End-to-end speed ledger at P-61

The native producer-side curve, generator, Dickson root set, and pair census fix
the input and setup sizes. They do not provide a target-decomposition
algorithm. The exact relation-query, matrix, and final-replay costs of a
complete attack have not been measured. This audit has not solved a
target Q. [pcc:unverified]

On the frozen u=1 base, B=486 canonical x columns. Any collector that
uses independently uniform nonidentity targets must obtain at least
B+1=487 relation outputs for the simple left-kernel route. The expected
number of signed seven-multiset representations per target is at most
binom(978,7)/(r−1)=0.0720718453, so even an all-witness collector
needs at least 6,757.15 target calls in expectation before matrix work.
A method that scans the full D=236,196 pair table on each
call would inspect at least 1.596·10^9 pair entries in expectation,
before checking the remaining five-term condition. This is a bound
for uniform targets and that scan architecture, not for every oracle.
Pair entries are not directly comparable to elliptic group operations.
[pcc:unverified]

The end-to-end accounting identity is

  C_E2E = C_point_count/factor + C_setup + N_query(C_7+ C_replay)
          + C_relation_handling + C_linear_algebra + C_final_recovery,

where N_query counts failed as well as successful exact seven-sum calls.
The conditional asymptotic result needs N_query=O(q), C_7=O(q^(4/3))
per call, and all other listed costs O(q² polylog p) or lower.
At P-61, p^(1/3)≈1,321,123 and ideal negation-quotient rho takes
about 1,345,735,808 group steps in expectation under a random-walk
model. The 6,757-query lower bound would leave at most about 195
notional operation units per call for a total budget of exactly
p^(1/3) such units, or about 199,157 rho-step equivalents per call
merely to beat that ideal
rho count. Both budgets must also pay setup, relation handling, matrix
work and final recovery. They are dimensional benchmarks, not measured
time comparisons; field operations, pair inspections and group steps
have different costs. In particular, the q^(4/3)≈10,321 reference is
only an asymptotic target for the missing oracle, not its observed cost.
[pcc:unverified]

For a random good scaling, the written coverage lower bound δ≈2.18·10^−15
allows a capped round of up to 2(q+1)/δ≈9.40·10^17 oracle calls when
rows use at most q canonical x columns. This is a very loose proof cap,
not an observed number. A complete exact O(q^(4/3)) seven-sum oracle
including misses has not been implemented or proved for this selector.
PR #5275 supplies a subquadratic target-specialized pair-query
primitive through depth six, but reports it 245–298 times slower than
the direct batched scan at depth six and stops before m=7.
[pcc:unverified]

Kaarelson v2 separately claims p^(2/7+o(1)) end-to-end for a SHORT
INTERVAL x predicate and a Boolean counting engine. It does not prove
that engine's rate for the Dickson selector. Its current formalization
leaves the complete ECDLP assembly and coverage as paper arguments;
no finite P-61 oracle time or matched complete attack is reported.
Therefore neither that claim nor the present Dickson reduction
establishes a measured M61-1 end-to-end speed or a practical gain.
[pcc:unverified]

The short-interval Theorem 1, if accepted, already gives an asymptotic
O(p^(1/3)) route for m=7 by taking its exponent slack 1/21; P-61 meets
its floor(p^(1/7))>=300 hypothesis with floor(p^(1/7))=420. The route
does not use Dickson roots and has no explicit finite running-time
constant at L=61. A literal attempt to add the Dickson predicate to
the paper's §3.1 Boolean network needs k=log2(q) modular squares per
leaf. Equations (1)-(2) and Lemma 3.2(S3) give at least 16mk(L+2)
extra unclustered shared bits and 8mk(L+2) extra held bits for its
same multiplication gadgets. Under the paper's product-setting
clustering, gP is at least the latter, or Ω(mL²) when q≈p^(1/7),
outside the proved O(mL) condition. For m=7,q=1024,L=61, the 70
extra gadget terms contribute 151,900 shared and 77,280 held bits.
This only rejects that literal transfer, not every possible Dickson
network or a cheap direct test of D_q at a known x. Any alternate
network must certify its count for empty targets and every prefix,
then bound U,gP,B and local evaluation cost before an oracle rate can
be inferred. [pcc:unverified]

## Sources and reproducibility

Kaarelson v2, Sections 3–6 and 10–11:
https://github.com/kaarelkaarelson/fast-ecdlp/blob/main/v2/paper.pdf
Xylouris, Acta Arithmetica 150 (2011), Theorem 1.1:
https://www.impan.pl/en/publishing-house/journals-and-series/acta-arithmetica/all/150/1/83078/on-the-least-prime-in-an-arithmetic-progression-and-estimates-for-the-zeros-of-dirichlet-l-functions
Canonical P-61 curve identity:
https://github.com/isogeny-labs/crypto/blob/main/docs/curves/testbed/testbed.json
Related native pair-query prior art:
https://github.com/isogeny-labs/crypto/pull/5275
Native finite replay in this PR: `p61_audit.rs`, `p61_bounds.cpp`,
`pair_spectrum.json`, and `coverage_bound.json`. The original Python
scratch exploration is not a repository research implementation.

Every new mathematical and finite conclusion in this note is
[pcc:unverified]. No `PCC-*` identifier is minted.

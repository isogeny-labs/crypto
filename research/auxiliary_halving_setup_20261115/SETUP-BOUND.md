# Conditional accounting for point-count-free auxiliary setup

Let p>3 be prime, d=2^t>=8 and p>=256d^4. This is a hand derivation for
an abstract sampler with complete exact finite-field root finding and
independent uniform choices. PCC-20261115-4b68b3 remains unverified. The
native implementation uses d=8, primes<=257 and fixed finite caps. Its
eight controls do not test this large-parameter count or prove the
program's expected cost. [pcc:unverified]

The premise from the existing [primary counting review](../prime_one_shot_20261029/continuation/reviews/modular-curve-counting.md)
is #Y_1(d)(Fp)>=p/2, with genuine Fp pointed elliptic curves. Re-opened
Conrad, Arithmetic moduli of generalized elliptic curves, Theorem 4.2.1(2)
and Remark 4.2.2: the level-d moduli problem is fine and pointed
automorphisms are trivial for d>=8 in this characteristic. The retrieved
author PDF has the exact earlier-review SHA256
f53e4ff819d534926e6ee59874cbce4e7bad4a315cbd6d05984b8405b11f15d8.
This uses a classical counting premise, not a new distribution assertion
about random elliptic-group orders. [pcc:unverified]

For each pointed Fp-isomorphism class (A,T), use its short models
y²=x³+ax+b. The Fp* scaling action is
(a,b,xT,yT) -> (u^4 a,u^6 b,u²xT,u³yT). Isomorphisms between short models
have this form in characteristic>3. Trivial pointed automorphisms make
every orbit have p-1 elements. Distinct pointed classes have disjoint
orbits. Thus the number M of actual tuples (a,b,T), with T of exact
order d, satisfies

    M >= (p-1)#Y_1(d)(Fp) >= p(p-1)/2.

This counts pointed short models. Counting only favorable unpointed
coefficient pairs discards the multiplicity used below. [pcc:unverified]

One subgroup trial draws a,x0 independently and uniformly in Fp, sets
b=-x0³-a*x0, rejects singular models and starts at K=(x0,0). It chooses
uniformly among all rational halves for t-1 steps, restarting on an empty
set. Find those sets with the standard cleared doubling quartic,
quadratic lifts, and actual [2]H=P checks. No point count is used.
The quartet identity is recorded as IDC1h6e44d7c75baf9f8d: 32+128 native
evaluations over F_(2^61-1). That record is not a formal integer or
program proof. Sources are MIT 18.783 Fall 2023 Lecture 2 section 2.2
and Lecture 3 sections 3.11–3.11.2. [pcc:unverified]

Fix one of the M tuples (a,b,T). Its initial pair is uniquely
(a,x([d/2]T)), selected with probability 1/p². Its successful path is
uniquely determined by successive doubles of T. Each nonempty rational
doubling fibre has at most four points: it is a coset of A[2](Fp), whose
geometric size in odd characteristic is four. Its path therefore has
conditional probability at least 4^(-(t-1))=4/d². Different terminal tuples
are disjoint outcomes. Summing gives

    Pr[success] >= M * (1/p²) * (4/d²)
                >= 2(p-1)/(p d²) >= 1/d².

A successful endpoint has exact order d: repeated doubling reaches the
nonzero order-two point K. The favorable-model sampling bias is included;
a uniform distribution over unpointed curves is not assumed.
[pcc:unverified]

The expected number of independent subgroup trials is at most d².
Each trial makes at most t-1 constant-degree root-finding steps and
polynomially many field/group operations in t. The cited Las Vegas
finite-field algorithms give expected poly(log p) bit cost per fixed-degree
step. Charging failed paths and final order checks gives expected

    O(d² t poly(log p))

for this abstract subgroup constructor. For d=exp(O(sqrt(log p))), this
setup contribution is subexponential. The relation solver's sufficient
degree, useful fixed-base yield, general arithmetic implementation and
later costs have separate obligations. [pcc:unverified]

Trace-good cosets can also be sampled without a point count. The uniform
proposal scheme gives each rational point exactly one accepted proposal
among 2p+1. Hasse bounds keep its expected proposal count constant in this
range. At most 4d points satisfy 2G in H; hence a uniform rational G is
trace-good with probability at least
1-4d/#A(Fp)>=1-8d/p>=1/2. The existing exact [2]phi(G) test applies.
Listing the d abscissas and constructing the tower are charged by
polynomial work in d,t,log p. [pcc:unverified]

The native cold filter also asks for exactly three main-base abscissas.
That condition is absent from this subgroup/trace argument. Neither its
acceptance probability nor the fixed 4096/64/128 native caps receives an
asymptotic guarantee from this lemma. Native controls use deterministic
fixed seeds; the mathematical probability model uses independent uniform
choices. No fitted exponent, measured speed, T3 or novelty claim is made.
[pcc:unverified]

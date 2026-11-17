# Compact scalar verification: a construction and scoped closures

2026-10-30 UTC. Pure mathematical analysis. No solver, contraction, benchmark,
timing experiment or discrete-log computation was executed. No original
sub-exponential algorithm or priority claim is established.

The constructive result is a uniform windowed verifier with
O(b^(20/19)) Boolean edges and degree O(b^(18/19)). It improves the literal
quadratic-size verification trace, but the supplied contraction theorem still
gives only 2^O(b^(20/19)/sqrt(log b)), which does not certify exp(o(b)).
Several tempting ways of declaring the verifier near-linear have precise
missing operations or counterexamples, detailed below. None is a universal
lower bound for prime-field ECDLP or for arbitrary Boolean encodings.

## Sources and input contract

Read the pinned Bochum v0.2 manuscript at
`/workspace/audit/tensor-primary/bochum-v02.txt`, especially Theorems2-4,
Lemma2, Lemma8/Corollary1, Corollary2, and the complete-cost statement.
Read AppendixD.1's exact hypotheses in
`/workspace/audit/tensor-primary/openai-oct25.txt`, including Boolean ports,
the global size parameter, implicit evaluation cost, coefficient precision and
truncation limits. Repository context includes the existing norm-circuit size
review and the session's reduced norm/divisor correspondence.

A delegated independent subtask additionally read Katherine Stange's primary
*Elliptic Nets and Elliptic Curves*, arXiv0710.1316v4, Theorem2.2,
Lemma4.2 and Corollary5.2. Its exact identities, source scope, PDF hash and
recurrence-window construction are recorded in
`/workspace/audit/division_poly_route.md`. The present note uses that checked
subtask rather than claiming another independent full reading of Stange.

Let E/Fp be nonsingular, p>3, with known odd prime whole order r and a
generator P. The target Q is valid. Write b=ceil(log2 p) and
B=ceil(log2 r)=b+O(1). Candidate scalars are canonical integers 0<=k<r.
This range restriction matters: otherwise integer encodings differing by r
can represent the same target. Q=O can be handled immediately.

D.1 supplies a bound of the form

    poly(S+L_description)*2^O(S/sqrt(log S))

for a network with at most C*S edges and degree at most ceil(S^(9/10)),
under its other fixed-constant hypotheses. The proof parameter S is not
automatically the number of unknown scalar bits. A single large vertex can
force S much larger than the actual edge count. Variable alphabet words also
are not single Boolean edges.

## 1. One implicit verification predicate does not meet the useful degree cap

For fixed public E,P,Q, the predicate

    f_Q(k)=1 iff 0<=k<r and [k]P=Q

is polynomial-time evaluable from B bits. Put it at the center of a star
with B bit edges and tautology leaves. Description and evaluation are small,
but the central degree is B. D.1 requires

    S>=B^(10/9).

Padding physical dummy edges is unnecessary; enlarging the theorem parameter
has the same cost consequence. The resulting stated bound is only
2^O(B^(10/9)/sqrt(log B)), not exp(o(B)). This does not prove the star is
intrinsically slow. It shows that D.1's stated guarantee cannot be applied
using S=B while ignoring its degree hypothesis.

The full star has exactly one satisfying canonical scalar, so its unrestricted
count is already known. Recovering the scalar requires prefix-conditioned
counts; those contain the unknown information.

## 2. A genuine smaller windowed encoding

Choose m windows, w=ceil(B/m), and public points

    P_j=[2^(j*w)]P, j=0,...,m-1.

They can be generated cold using O(B) doublings. No table containing all
window multiples is constructed or assumed. Write

    k=sum_j k_j*2^(j*w), 0<=k_j<2^w,

fix unused top bits, and enforce k<r with the source's linear-size comparator.
For each j introduce the canonical point word Z_j=[k_j]P_j, represented by
an identity flag and two b-bit coordinates, as in the source's point encoding.

Split this 2b+1-bit output word into h=ceil((2b+1)/w) chunks. For each chunk
make an implicit Boolean predicate with all w input bits of k_j and at most
w output bits. It computes [k_j]P_j by an ordinary deterministic scalar
multiplication and checks only that output chunk.

Every such predicate has arity at most2w and polynomial-time evaluation.
Its description contains a public point, the curve and a chunk index, not
a lookup table, a logarithm or solved-instance advice. The same scalar bits
must be copied to all h predicates using equality trees. That costs
O(w*h)=O(b+w) edges per window. Every output bit is forced, including the
identity representation when k_j=0.

Finally enforce sum_j Z_j=Q with Corollary2's complete point-addition network.
It costs O(mb) edges and degree O(b^(3/4)). Point words are joined with only
constant-factor copying. The full network has

    M=O(mb),
    Delta=O(b/m+b^(3/4)).

All setup and all vertex evaluation are polynomial in b. Repeated description
constants contribute at most polynomial length in M. The extension of each
canonical k is unique, so prefix-conditioned contractions count either0 or1.
One may consequently use root precision P0=1 and ell=1 for these queries;
this does not improve D.1's displayed exponent, whose recursive recovery
still requires its own precision buffers. Bipartite subdivision changes only
constant factors. This observation is specific to the full-point verifier:
an x-only division-polynomial terminal generally admits the two signs and
must not use parity as if its satisfying assignment were unique.

Take m=A*ceil(b^(1/19)) for a sufficiently large fixed A. Then

    M=O(b^(20/19)),
    Delta=O(b^(18/19)),
    (20/19)*(9/10)=18/19.

The degree cap is satisfied by a fixed constant adjustment of S. Polynomially
many prefix queries find k, but the certified bound remains

    2^O(b^(20/19)/sqrt(log b)).

The exponent divided by b grows as b^(1/19)/sqrt(log b), so the displayed
bound is not sub-exponential in b. There is no finite-size performance claim.

Within this explicitly exposed full-point window template, simply choosing
fewer windows and padding the theorem parameter does not improve that
asymptotic guarantee. Its declared window predicates have arity Theta(b/m),
so one must take, up to fixed constants,

    S>=max(mb,(b/m)^(10/9),b^(5/6)).

Balancing the first two terms gives m=b^(1/19) and S=b^(20/19).
This is a size/degree balance for this construction, not a lower bound for
all target-dependent encodings. The same calculation with a fixed cap
Delta<=S^alpha gives S=b^(2/(1+alpha)) for alpha in[3/5,1), where the
lower endpoint ensures the cited b^(3/4) arithmetic gadgets fit too.

## 3. Formal rank one is not a public factorization algorithm

For a valid target the truth tensor of f_Q has singleton support, so it
formally has rank one across every partition of the scalar bits. Its factors
can be written as delta functions at the unknown blocks of k. This describes
the answer rather than computing those factors from P,Q.

More precisely, suppose an efficient public procedure produced Boolean
factors on disjoint bit blocks,

    f_Q(k)=product_j g_j(k restricted to block_j),

with maximum block length d and efficient evaluation of each g_j. Since the
global support has size one, the support of each factor has size one. Each
block could be located by at most2^d evaluations. Total work would be
O((B/d)*2^d) evaluations, already sub-exponential for d=o(B) and polynomial
for d=O(log B). Uniformly producing these factors is therefore a missing
inversion operation, not a free consequence of low rank. This observation
does not exclude overlapping constraints or a compact auxiliary certificate.

## 4. Low-degree rational parametrization in the integer index is closed

Suppose one tries to avoid the binary trace by writing the scalar point as
public rational functions X(t)=A(t)/B(t), Y(t)=C(t)/D(t), with every numerator
and denominator of degree at most d, valid on N distinct field values
representing distinct nonzero scalar labels. For k=1,...,min(p,r)-1 one
has N=Theta(p), absent denominator exceptions.

Clearing the curve equation gives

    C^2 B^3-A^3 D^2-a*A*B^2*D^2-b_E*B^3*D^2=0.

Its degree is at most5d. If N>5d, it vanishes identically. The resulting
nonconstant rational map from P1 to E extends because E is proper, and is
impossible: a one-variable rational function field has only genus-zero
transcendence-degree-one subfields (Luroth). The argument also covers
inseparable maps. Thus this particular rational-parametrization proposal
requires d>=N/5=Omega(p).

This says nothing against a short arithmetic program representing a
high-degree function. It also does not rule out a target-dependent polynomial
whose zeros locate the logarithm: k-k_Q has degree one, but constructing its
coefficient would already reveal the answer. Do not transfer the map-degree
argument to such secret-dependent relations.

## 5. Radix batching does not shorten a literal rational multiplication map

For a variable curve point R, the doubling x-map is

    x([2]R)=(x^4-2a*x^2-8b_E*x+a^2)/(4*(x^3+a*x+b_E)).

Its numerator and denominator have no common root on a nonsingular curve:
at a root of the cubic the numerator becomes(3x^2+a)^2, so a common root
would be a multiple root of the cubic. The map on the x-line has degree4.
Composing it w times gives degree4^w. Equivalently x([2^w]R), as a function
on E, has pole degree2*4^w.

Start a rational straight-line program with x,y and constants. Their pole
degrees are at most3. Addition, multiplication and division each make the
output pole degree at most the sum of the two input pole degrees; inversion
preserves that degree. After q operations the maximum is at most3*2^q.
An implementation of this actual rational multiplication map therefore needs

    q>=log2(2*4^w/3)=2w-O(1).

Replacing w successive doublings by one explicitly evaluated rational radix
map does not remove the total Theta(B) arithmetic-operation scale. The literal
translation assigning a full b-bit interface to every operation still uses
Theta(b^2) interface ports; this is not a lower bound for arbitrary encodings.

This is deliberately restricted to that rational-map implementation. It does
not cover a different endomorphism agreeing with the multiplier only on the
finite subgroup, Frobenius shortcuts, branching predicates, implicit chunk
checks or a special-family algorithm. In particular a low-degree endomorphism
can act as a large scalar modulo r without being the same global map as that
integer multiplication map. Such finite-field action must be analyzed
separately; it is not refuted by the displayed pole count.

## 6. Short hashes and exposed random checks lose the required quantifiers

An exact faithful group-state token preserving arbitrary-target equality
must distinguish all r group elements and hence needs at least log2 r bits.
A noninjective homomorphic token cannot by itself certify an exact group sum.
This restriction concerns faithful state representations, not all encodings.

Allowing false positives and verifying candidates later does not automatically
help. For any hash h:G->a set of H buckets of sizes n_j, suppose trials sample
independently and uniformly with replacement from the bucket of a uniform
target Q and then verify the candidate. Conditional expected trials equal
n_(h(Q)). Therefore

    E_Q[trials]=(1/r)*sum_j n_j^2>=r/H.

Even with free candidate generation, outputting only t=log2 H=o(b) hash bits
gives exponential expected retry cost in this strategy. Multiple hashes are
the same argument for their joint range. This does not apply to the existing
many-point decomposition scheme, which checks its final group equality exactly.
It also is not a lower bound for a procedure that somehow selects the correct
member nonuniformly; that selection needs a new justification.

For proposed Freivalds/sumcheck compression, distinguish

    for each wrong witness, Pr_seed[accept]<=delta

from the needed static existential statement about whether some wrong witness
can be chosen after the seed. They are not equivalent. A single random linear
check r dot e=0 illustrates the issue: after seeing r with at least two
coordinates, a nonzero error e can be chosen in its kernel, e.g.
(r_2,-r_1,0,...) if those two coefficients are not both zero, or any nonzero
kernel vector otherwise. The check always accepts that adaptive error.
This does not show every such error is realizable by an elliptic trace; it
refutes the quantifier inference. A commitment or a sound protocol for the
correct order of choices must be supplied and charged. Exposing all future
interactive challenges to an existential tensor solver is not that protocol.

## 7. Residue-only divisor certificates do not test principality

There is a separate algebraic trap in using a logarithmic derivative to avoid
a large Miller/division-polynomial object. Assume r!=p. Then multiplication
by p is a bijection of Pic0(E)(Fp)=E(Fp). For every rational degree-zero
divisor D choose a rational class D0 with p[D0]=[D]. The divisor D-pD0 is
principal, so some rational function f satisfies

    div(f)=D-pD0.

In characteristic p the logarithmic derivative has the residues specified by
D; the extra pD0 contribution is invisible to those residues. Consequently
correct residue data alone can be supplied even when D is not principal.
For example take D=(P)-(Q) with P!=Q and let D0=(T)-(O), where
T=[p^(-1) mod r](P-Q). The original divisor is nonprincipal, but the displayed
principal modification exists. Exact pole/degree information cannot simply
be dropped from such a certificate. This is not an attack on all differential
methods; the r=p anomalous case is outside this argument.

## 8. Coefficient-ring words do not make large edge alphabets free

D.1 sums a bit on every edge. Its coefficient ring can store a polynomial-bit
scalar, but this does not provide an uncharged boundary table for2^b possible
point values. Direct generating-polynomial packing of an arbitrary b-bit
edge alphabet needs degree or coefficient-table dimension about2^b; ordinary
base-digit packing needs corresponding precision. Neither follows from the
stated limits P0,ell<=C*S. A different compressed representation and exact
composition rule would have to be given. Calling a point word a single edge
or summing independent hidden copies without enforcing their consistency
would change the problem.

## 9. Black-box entry access imposes a sharper degree-extension boundary

There is an exact query-model restriction on extending the same contraction
rate to very large predicates. Consider a star with d Boolean edges and one
center predicate. If entry routines are opaque evaluators, distinguishing the
all-zero predicate from an unknown point predicate

    h_a(x)=1 iff x=a, a in {0,1}^d,

requires2^d queries in the deterministic exact worst case. Every unqueried
point can still contain the sole1. Even if a unique accepting assignment is
promised, finding it under uniformly hidden a needs Omega(2^d) expected
queries for an exact search. Prefix-conditioned counting does not remove
this information requirement.

Consequently a black-box-entry contraction algorithm cannot have a general
poly(S)*2^O(S/sqrt(log S)) bound while admitting degree

    Delta=S/(log S)^c, fixed 0<=c<1/2,

with polynomial-size opaque handles. On the star choose
S=Theta(d*(log d)^c); the proposed rate becomes exp(o(d)), contradicting
the query bound. A cap around S/sqrt(log S) is compatible with this necessary
query condition, but a larger one is not for an evaluator-only method.

This is not a lower bound for algorithms permitted to inspect and exploit
the entry program. A point predicate's explicit code could reveal a directly.
The present D.4 procedure uses original routines through evaluation and
metadata, rather than a semantic program analysis; a description-aware
extension would require a new argument and could evade this scoped barrier.

For the explicit window family, M=Theta(mb) and declared large-predicate
degree Theta(b/m). Even a hypothetical improvement admitting degree as large
as the black-box-compatible S/sqrt(log S), with the same rate and allowing
padding of its global parameter S, requires

    S >= Omega(max(mb, (b/m)*sqrt(log S))).

Balancing these costs gives m=Theta((log b)^(1/4)) and the smallest certifiable
S=Theta(b*(log b)^(1/4)). Its resulting contraction guarantee cannot improve
the existing b/(log b)^(1/4) exponent through this parameter balance. Proving such a
degree extension could still give a structurally different direct algorithm;
it is not supplied here. A stricter asymptotic improvement in this route
needs either a better encoding, a different contraction rate or exploitation
of the particular entry programs beyond black-box evaluation.

## Result and surviving obligation

No distinct sub-exponential prime-field ECDLP algorithm has been constructed
here. The windowed verifier is an explicit public construction and improves
the literal trace representation, but remains outside the useful D.1 size
regime. A surviving direct-scalar route must provide a genuinely more compact
uniform encoding, or prove a stronger contraction result that preserves and
charges the functional structure. A conjecture about the number of free input
bits does not itself establish that invariant through the recursive deformation.
The exact source's current guarantees should be retained until such a lemma
or construction is actually proved.

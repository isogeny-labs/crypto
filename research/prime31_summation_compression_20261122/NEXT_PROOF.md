# Successor result: exact degree growth under scalar Dickson compression

Status: mathematical proof independently checked by the algebra agent; not Lean-kernel-verified, not PCC-verified, and no novelty claim. This is an obstruction for one proposed representation, not an ECDLP complexity lower bound. [pcc:unverified]

## Theorem

Let K be an algebraically closed field of characteristic p>3, let E/K be a smooth elliptic curve with origin O and Weierstrass x-coordinate, and let R be a nonzero point of finite odd order. Let k>=0, N=2^k, and let D_N be the monic Dickson polynomial characterized by D_1(X)=X and D_(2n)(X)=D_n(X)^2-2. Define rational functions on E by

f(P)=D_N(x(P)),

g(P)=D_N(x(R-P)).

Then K(f,g)=K(E). In particular the map P -> (f(P),g(P)) is birational onto its image. The irreducible affine polynomial relation F(U,V)=0 defining that image satisfies

deg_U F=deg_V F=2N.

The conclusion does not require E to be ordinary. Ordinary prime-field curves with prime odd group order are an important applicable case.

## Proof

### 1. Pole divisors and separability

The function x has its unique pole at O, of order two. Since D_N is monic of degree N, lower-degree terms cannot cancel its highest pole, and therefore

(f)_infinity=2N[O].

The map P -> R-P is an automorphism of E sending R to O. Consequently

(g)_infinity=2N[R].

The degrees of the finite maps f,g:E -> P1 are both 2N. Thus [K(E):K(f)]=[K(E):K(g)]=2N. Because p is odd, p does not divide 2N; the inseparable degree, which is a power of p dividing the extension degree, is one. These function-field extensions are separable.

Put L=K(E), L0=K(f,g), and e=[L:L0]. The tower law shows that e divides 2N, so e is a power of two. Let C be the smooth projective curve with function field L0. Its inclusion in L gives a finite separable morphism pi:E -> C of degree e.

**Correction to a tempting shortcut:** e being prime to p does not by itself imply that every ramification index of a non-Galois cover is prime to p. We do not claim global tameness or need it. The separable Riemann-Hurwitz formula with its effective different divisor suffices.

### 2. Two singleton fibres

Regard f and g as rational functions f0,g0 on C, pulled back by pi. Every pole of f0 pulls back to poles of f, whose support is the single point O. Since pi is surjective, f0 has exactly one pole a, and the set-theoretic fibre pi^-1(a) consists only of O. Over algebraically closed K, the degree of the fibre is e; hence

pi^*[a]=e[O].

Likewise g0 has one pole b and

pi^*[b]=e[R].

The points a and b are distinct. Otherwise the same fibre would have support both {O} and {R}, forcing R=O, contrary to hypothesis. In particular pi is totally ramified at O and R whenever e>1.

### 3. Eliminate both possible genera for a nontrivial quotient

Riemann-Hurwitz for the separable map pi gives

0=2g(E)-2=e(2g(C)-2)+deg(Diff_pi),

where the different is effective. Therefore g(C)<=1.

If g(C)=1, the formula gives deg(Diff_pi)=0; pi is unramified. The singleton fibre over a has ramification index e, so this forces e=1.

If g(C)=0, then C is isomorphic to P1 because K is algebraically closed. For the distinct points a,b there is a rational function h on C with divisor [a]-[b]. Pulling it back gives

div(h composed with pi)=e[O]-e[R].

The canonical group isomorphism E(K) -> Pic^0(E), P -> class([P]-[O]), implies that this divisor is principal only if [e]R=O. But e is a power of two, while R has nonzero odd order. This is impossible.

The genus-zero case is therefore excluded, and the genus-one case gives e=1. Hence L0=L, as claimed.

### 4. Exact bidegree

Let F(U,V) be the irreducible equation of the affine image. Since K(f,g)=K(E), the minimal polynomial of g over K(f) has degree [K(E):K(f)]=2N. Clearing denominators and taking the primitive irreducible equation does not change the degree in V, so deg_V F=2N. Interchanging f and g gives deg_U F=2N. This establishes the exact bidegree rather than only an upper bound from a resultant.

Equivalently, the two projections of the normalized image to P1 each have degree 2N. This also avoids ambiguity in conventions for the order of entries in a bidegree: both entries are the same here.

## Finite-field consequence

Apply the theorem after base extension from F_p to its algebraic closure. Any polynomial relation over F_p also remains a relation there, so it cannot have smaller degree in either variable. The image is geometrically integral, and its irreducible equation over F_p has the same projection degrees. On the repository's recorded prime-order Prime31 curve, every nonzero target R has odd order, so the result applies to each such R.

The theorem is a uniform statement for each k and each p>3. It neither assumes infinitely many Mersenne primes nor depends on the Dickson fibre splitting over F_p. Splitness matters for the proposed factor-base implementation, while this geometric degree obstruction is over the algebraic closure.

## Meaning for the next algorithmic iteration

Replacing both scalar coordinates by D_(2^k) of those coordinates cannot leave an exact scalar image equation of bounded bidegree: the degree in each variable grows exactly as 2^(k+1). Thus a proposal to push the two S3 messages through repeated D_2 maps while keeping one constant-degree bivariate equation has a precise obstruction.

This does **not** prove any of the following: [pcc:unverified]

- a lower bound on arithmetic-circuit size (repeated squaring itself gives large degree with a short circuit);
- a lower bound on the number of nonzero monomials;
- a lower bound on solving the relation restricted to one finite rational fibre;
- an ECDLP or index-calculus complexity lower bound;
- impossibility of a vector-valued auxiliary state, rational data structure, batched target method, or sublinear common-root algorithm.

In particular a finite-set-only predicate can have representations not governed by the function-field theorem. Any successor claiming to evade the degree growth must state exactly which of those representation changes it uses, then count the complete membership and witness-extraction cost.

## Sharpness checks and useful generalization

The exclusion R=O is essential: then g=f and the image relation is U-V=0. For nonzero rational 2-torsion and N=1, both x(P) and x(R-P) lie in K(x(P)); the standard S3 equation degenerates to a repeated rational-translation relation, illustrating why the odd-order condition matters.

The proof uses only two functions with unique pole divisors d1[O] and d2[R]. More generally, if such functions exist, their joint quotient degree e divides gcd(d1,d2); a nontrivial genus-zero quotient forces [e]R=O, while a genus-one quotient with either singleton totally ramified fibre forces e=1. Hence a sufficient general condition for birationality is that no integer e>1 dividing gcd(d1,d2) annihilates R. The Dickson case takes d1=d2=2N and an odd-order R.

## Standard inputs checked against primary mathematical references

- The Stacks Project, Algebraic Curves, Section 53.12, Tag 0C1B: separable Riemann-Hurwitz with the effective different divisor and its distinction from the tame ramification-index formula: https://stacks.math.columbia.edu/tag/0C1B
- William Stein, *Algebraic Number Theory*, “Abelian Groups Attached to Elliptic Curves”: the isomorphism E(K) -> Pic^0(E), P -> [P-O]: https://www.wstein.org/books/ant/ant/node65.html
- Ben Lynn, *Elliptic Curves — Zeroes and Poles*: div(x-a)=[P]+[-P]-2[O], including the 2-torsion multiplicity: https://crypto.stanford.edu/pbc/notes/elliptic/divisor.html

The argument above is our application of these standard facts, not a theorem claimed to appear in those references. No originality claim has been assessed.

## Formalization obligations

No Lean runtime was available in this session. A formal proof would require: pole orders of Weierstrass x; degree of a function from its pole divisor; the smooth projective model of the intermediate function field; pullback of point divisors and singleton-fibre multiplicity; separable Riemann-Hurwitz; genus-zero identification with P1 over an algebraically closed field; the elliptic Pic^0 group identification; and projection degree versus irreducible bivariate degree. The short elementary polynomial identities issued in the preceding iteration do not certify these function-field steps.

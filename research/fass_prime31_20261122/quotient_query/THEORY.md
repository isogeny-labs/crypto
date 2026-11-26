# Exact quotient queries as triangular Groebner decompositions

Tracking: [crypto PR #4966](https://github.com/isogeny-labs/crypto/pull/4966),
following the merged [target-quotient study](../target_quotient/THEORY.md).
Status: written derivation and source review; the mathematical statements in
this file have no checked Lean proof or signed PCC verdict. [pcc:unverified]

## 1. The computational object

Let k=Fp. Let F(x) and G(t) be monic, squarefree polynomials which split over k,
of positive degrees B and e. Let H(x,t) be represented with
deg_x(H)<B and deg_t(H)<e. Define

$$
I=\langle F(x),G(t),H(x,t)\rangle,\qquad
Z=\{(a,b)\in\mathbb F_p^2:F(a)=G(b)=H(a,b)=0\}.
\tag{1}
$$

The empty-selector cases F=1 or G=1 are handled before this definition:
Z is empty and I is the whole polynomial ring. A zero modulus is invalid.
Squarefreeness and splitting are checkable without enumerating the field:
F must divide x^p-x and G must divide t^p-t.

In the FASS application, assume p>3 and a nonsingular short-Weierstrass
curve. The full selector is L(z)=z^d-1, d divides p-1, and the input is the residue

$$
H=\operatorname{Res}_z(L(z),S_3(x,z,t))\bmod(F(x),G(t)).
\tag{2}
$$

F can be L itself or the polynomial of an explicitly supplied subset of its
roots. The target-dependent G is the squarefree rational part of the right
message. When it comes from the complete scalar gcd,

$$
G=\gcd\!\left(\prod_{z^d=1}S_3(t,z,X),\ t^p-t\right),
\tag{3}
$$

it automatically has the required split and squarefree properties. The
polynomial product in (3) is unnormalized: selector targets and degree drops
must not be processed by division by a vanished leading coefficient.
The construction of G and the binding of (2) are charged prerequisites.

The solver below is exact for (1). It does not need any generic-rank
assumption. It also applies to H which did not arise from Semaev polynomials.

## 2. A checkable decomposition certificate

The output consists of monic nonconstant q_i(t), together with polynomials
D_i,U_i,V_i,A_i,B_i in (k[t]/q_i)[x]. The verifier checks

$$
G=\prod_i q_i,\qquad \gcd(q_i,q_j)=1\quad(i\ne j),
\tag{4}
$$

and, on each block,

$$
\begin{aligned}
F&=D_iA_i\pmod {q_i},\\
H&=D_iB_i\pmod {q_i},\\
U_iF+V_iH&=D_i\pmod {q_i}.
\end{aligned}
\tag{5}
$$

D_i is monic in x, including the allowed constant D_i=1. All coefficient
representatives in t have degree less than deg(q_i). The leading coefficient
one is an equality in the quotient ring, not merely a nonzero polynomial.
Certificate input hashes bind F,G,H and the field modulus. An application
also binds the curve, target, selector and any subgroup restrictions.

**Proposition 1 (certificate soundness and coverage).** A certificate satisfying
(4)-(5) gives

$$
I=\bigcap_i\langle q_i(t),D_i(x,t)\rangle.
\tag{6}
$$

For every root b of q_i, D_i(x,b) is the monic gcd of F(x) and H(x,b).
Consequently a block with D_i=1 has no point of Z. A block with
r_i=deg_x(D_i)>0 has exactly r_i distinct x-coordinates over every one of its
deg(q_i) rational t-coordinates. In particular,

$$
\boxed{|Z|=\sum_i\deg(q_i)\deg_x(D_i).}
\tag{7}
$$

The entire affine grid is empty if and only if every D_i is one.
[pcc:unverified]

*Proof.* Reduce (5) at a root b of q_i. The first two identities say that the
monic D_i(x,b) divides both inputs. The third says every common divisor
divides D_i(x,b). This establishes the gcd assertion. Monicity preserves its
x-degree under specialization. Since F splits and is squarefree, this gcd
has precisely r_i distinct ground-field roots. The blocks partition the
roots of G by (4), giving (7).

The same identities give equality of ideals
<F,H,q_i>=<D_i,q_i>. The ideals containing different q_i are comaximal.
Chinese remaindering modulo G therefore gives (6), including the factors
equal to the unit ideal. Equivalently, work in
k[t]/G isomorphic to the product of k[t]/q_i and compare the generated ideals
component by component. This also establishes coverage without enumerating Z.

For an empty block, the last identity alone supplies a sufficient negative
certificate, U_iF+V_iH=1 mod q_i. For a positive block, that identity alone
is insufficient: the two divisibility identities must also be checked.
The verifier must reject a missing block, an overlapping partition, a
nonmonic purported gcd, or a failed identity.

## 3. The explicit Groebner-basis consequence

**Proposition 2 (reduced triangular bases).** For each positive block, the pair

$$
\{D_i(x,t),q_i(t)\}
\tag{8}
$$

is a reduced Groebner basis of its ideal for lexicographic order x>t.
An empty block has reduced basis {1}. The ideal I in (1) is radical.
[pcc:unverified]

*Proof.* The leading monomials of (8) are x^r_i and t^deg(q_i). They are
coprime, so Buchberger's product criterion makes their S-polynomial reduce
to zero. Both polynomials are monic. Every remaining term of D_i has smaller
x-degree and t-degree below deg(q_i); the remaining terms of q_i have
smaller t-degree. Thus each tail is reduced by both leading monomials.

For radicality, the algebra k[x,t]/<F,G> is a finite product of copies of k.
The ideal generated by the image of H selects some of those components;
the resulting quotient is again a product of fields, or the zero ring.
Hence I is radical. This conclusion concerns the finite-grid ideal after
the squarefree selector/right-message restrictions. It does not declare
the original composed-selector system radical, preserve its multiplicities,
or bound its Groebner solving degree.

The union of bases from different blocks is generally **not** a Groebner
basis for I: union generates a sum of ideals, while (6) is an intersection.
The solver returns a triangular decomposition with a reduced lex basis for
each component. Constructing one global lex basis would be a separate task.

## 4. Producing the certificate without factoring G first

Use Euclidean polynomial operations in x over K_q=k[t]/q. This is a product
of fields, so a nonzero coefficient need not be invertible. At a required
inverse of c(t), compute h=gcd(c,q):

| Result | Exact action |
| --- | --- |
| h=1 | Compute an inverse by scalar extended gcd and continue |
| h=q | c is zero in this block; trim the degree and reconsider |
| 0<deg(h)<deg(q) | Split into q=h and q=q/h, reduce the current state, and continue both |

The proper-split row includes linear factors. Squarefreeness makes h and q/h
coprime. Retain the Euclidean
transformation coefficients, normalize every terminal gcd, and compute the
two exact quotients in (5). The zero-polynomial case H=0 terminates with
D=F. No trial division by zero is licensed by this procedure.

Every proper split decreases the degree on both child branches. There are
at most e leaves and e-1 proper splits. Between splits, ordinary Euclidean
degrees decrease. Termination and correctness follow by specialization
to each root of q. A degree, memory or execution cap produces
INDETERMINATE with its partial state, never an empty certificate.

This is an application of classical **dynamic evaluation (the D5 principle)**.
It is not a newly discovered way to compute gcds over products of fields.
Dahan, Moreno Maza, Schost and Xie give the product-of-fields XGCD framework;
Li, Moreno Maza and Pan place related operations in regular-chain and
subresultant algorithms. The new component specializes their interface to
the retained S3 quotient and emits identities that another routine can check.

### A degree-changing control

Over F5, take F=x(x-1), G=t(t-1), H=tx+1-t. The apparent leading
coefficient t is a zero divisor. Splitting gives

$$
q_0=t,\ D_0=1;\qquad q_1=t-1,\ D_1=x.
\tag{9}
$$

The first block is empty; the second has one grid point (x,t)=(0,1).
For q_0, U_0=0,V_0=1 is a Bezout witness. For q_1 the same pair gives
D_1=x, and the division witnesses are F/x=x-1 and H/x=1.
This control detects both an invalid inverse and the error of assuming a
single gcd degree across all fibres.

Changing H to t(x+1) gives D=F on q=t and D=1 on q=t-1. This exercises an
entire zero polynomial in one block, including two grid points there.
These examples are algebraic controls, not elliptic-curve relation fixtures.

## 5. Keeping the norm circuit and charging its algebra

The existing division-free identity is

$$
N_d=a^d+c^d-T_d,\qquad T_0=2,\ T_1=-b,\
T_{j+2}=-bT_{j+1}-acT_j,
\tag{10}
$$

where az^2+bz+c=S_3(x,z,t). Its binary evaluation takes O(log(2d)) operations
in any commutative coefficient ring. It may therefore be evaluated directly
in k[x,t]/(F,G), without constructing the unreduced bivariate norm or
dividing by a. The resulting canonical H still has B*e coefficient slots.
The circuit length and the size of this explicit state are different costs.

Let M(n) denote a polynomial multiplication bound valid over the coefficient
rings being used. Nested multiplication and monic reduction give the
theoretical bound O(M(B)M(e)) field operations per product in this tensor
quotient. Thus the norm construction has bound

$$
O\!\left(M(B)M(e)\log(2d)\right).
\tag{11}
$$

For the gcd stage, Dahan et al., Proposition 2.4 combined with Proposition
4.1, supplies a classical dynamic **half-gcd** bound

$$
O\!\left(M(B)\log(2B)\,M(e)\log(2e)\right)
\tag{12}
$$

in base-field operations, with properly organized splitting and fast
polynomial arithmetic. Formula (12) is a literature target, not a complexity
claim for an implementation using schoolbook products or recursive restarts.
The source-dependent branch work, quotient reductions, coefficient inverses,
state copies and certificate construction must be recorded separately.

With quasi-linear multiplication, (11)-(12) are softly O(B*e), not polylogarithmic
in the selector size. The explicit H alone occupies B*e coefficient slots.
The terminal gcds and bounded-degree Bezout/divisibility data can likewise
occupy O(B*e) slots because sum_i deg(q_i)=e. This is no lower bound for every
possible circuit representation or query algorithm.

Computing the scalar resultant Res_x(F,H) and checking its gcd with G is
an equivalent unit test, but its evaluation and certificate have costs.
Calling that resultant one operation does not remove the tensor-algebra
work. Similarly, the cited elliptic-resultant evaluation algorithms require a suitable
elliptic index system; a multiplicative set of x-coordinates does not supply
one automatically. Bernstein et al., Example 4.13, explicitly notes the
dependence on coefficient-algebra size.

## 6. Witness routing and the original signed problem

The positive blocks route extraction as follows: isolate only roots b of
positive q_i, then isolate roots a of D_i(x,b). Both are known to be rational
under the theorem's hypotheses. A factor tree of supplied roots, or an exact
finite-field root routine, can perform this step; its construction and all
root work are charged. The count (7) is available before that enumeration.

For each (a,b), the eliminated selector coordinate is recovered from
gcd_z(L(z),S_3(a,z,b)); its degree can be zero, one or two unless the
quadratic is identically zero. All degree drops need explicit handling.
Recover the right selector witnesses similarly, or use retained right-message
backpointers. Point lifts, consistent signs, subgroup membership and direct
group replay then determine admissible signed relations. Preserve every
collision bucket needed by a complete-output contract.

When F=L contains coordinates with no rational lift, a positive grid point
need not produce a signed relation. A certified empty **superset** is still
a sound exclusion for any factor base contained in that grid. Using an
actual rational-lift selector F removes that particular mismatch, but does
not restore eliminated signs or handle infinity automatically.

In a three-summand decomposition P1+P2+P3=Q, the branch P1+P2=O has no
finite t-coordinate. With a nonempty sign-closed base and repetitions
allowed, it occurs exactly when Q is in the signed base; it contributes
cancelling first pairs. If the API permits Q=O, that target needs its own
coverage argument because x(Q) is undefined. Two-torsion, subgroup
restrictions and forbidden repetitions also remain explicit interface
conditions. A complete signed EMPTY result requires these external branches
to be discharged as well as all q_i to be empty. The query component emits
an affine-grid certificate and a routing interface, not a silent full-solver
verdict.

## 7. Proof status and sources checked

[PROOF_MAP.md](PROOF_MAP.md) distinguishes the written arguments, source
checks and uncompleted formalization. The local capability check found no
Lean or Lake executable. No placeholder theorem file was added and no
native identity evaluation is represented as a universal proof.

Sources checked (KB): no research-KB MCP capability was exposed; the parent
ran scripts/research-kb status and obtained no executable. Scoped fallback:

- [Prior target-quotient theory](../target_quotient/THEORY.md): generic
  coefficient rank, radical right message and exceptional targets.
- [Prior coupled report](../coupled/REPORT.md), sections 2 and 5: retained
  norm circuit and the explicit nonunit-query obligation.
- [Selector report](../selector/REPORT.md): universal division-free norm
  identity and degree-drop convention.
- [NEXT_IMPLICIT_INDEX.md](../../prime31_summation_compression_20261122/NEXT_IMPLICIT_INDEX.md)
  and [NEXT_ROOT_INTERSECTION.md](../../prime31_summation_compression_20261122/NEXT_ROOT_INTERSECTION.md):
  previously identified dense-state and exact emptiness costs.
- [Pair-spectrum continuation](../../prime31_summation_compression_20261122/iteration9/PAIR_SPECTRUM_RESULTANT.md):
  separate four-summand chart and scalar Bezout formulation; not duplicated here.
- Dahan, Moreno Maza, Schost and Xie,
  [On the complexity of the D5 principle](https://cs.uwaterloo.ca/~eschost/publications/DMSWX-05b-TC-2006.pdf),
  author preprint dated 2005-11-15, associated with TC 2006. All 18 pages
  read; Definitions 1.1, 1.7, 1.8 and Propositions 2.4, 4.1, 5.1 are the
  relevant input/certificate and complexity contracts.
- Li, Moreno Maza and Pan,
  [GCD Computations Modulo Regular Chains](https://www.csd.uwo.ca/~mmorenom/Publications/Li.Moreno-Maza.Pan-JSC-format-2009.pdf),
  author preprint dated 2010-02-07, introduction and regular-gcd sections:
  the regular-chain/subresultant comparison for this specialization.
- Van der Hoeven and Lecerf,
  [Directed evaluation](https://www.texmacs.org/joris/direval/direval.html),
  2020, section 1.1: naive branch recomputation does not inherit the best
  directed-evaluation complexity automatically.
- Petit, Kosters and Messeng,
  [Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 2.4 and 3.1: composed selector systems motivating the study.
- Bernstein, De Feo, Leroux and Smith,
  [isogeny evaluation paper](https://arxiv.org/pdf/2003.10118),
  Lemma 4.3, Theorem 4.11 and Example 4.13: biquadratic addition,
  compatible index systems and the cost of coefficient algebras.

No novelty priority or experimental performance result is asserted by these
derivations. [pcc:unverified]

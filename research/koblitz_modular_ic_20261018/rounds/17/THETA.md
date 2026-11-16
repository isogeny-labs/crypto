# Odd cover: actual kernel and factored theta descent

G17 starts with the actual odd-cover matrix B of Round16 graphs0 and2,
in Z[w] with w²+w+2=0. Exact Euclidean row/column operations give

    U B V = diag(1,1,3,105).

There are17 recorded Smith iterations. Both U and V are unimodular;
explicit two-sided inverses are exported and checked. Generated Lean
proves the actual8-by8 integral Smith and inverse identities. The general
kernel-parameterization implication is also proved in Lean, with its
additive equivalence hypotheses exposed.

Consequently the split-product kernel is E[3]×E[105], with four generators
of orders3,3,105,105, and total order99225. This conclusion uses the
actual Smith certificate and full small-prime torsion bases, not degree
alone. It is parametrized by

    V(0,0,a P3+b Q3,c P105+d Q105),
    a,b mod3; c,d mod105.

Native arithmetic constructs independent3/5/7 bases, verifies their exact
orders and primitive Weil pairings, applies V to the four generators and
checks B kills them. All diagonal Theta and3Theta commutators vanish;
finite exponent-form congruences are generated and proved in Lean.

The E0 Frobenius orders on3/5/7 torsion are8,24,42; for E1 they are8,24,21.
Both sets split over F_(2^168). The certified modulus is x^168 plus low
bits95, namely x^168+x^6+x^4+x^3+x^2+x+1. Full elliptic cardinality
prime valuations are(4,2,3), so torsion extraction retains the complete
prime-primary cofactor and reduces actual point orders. Independent bases
are found by point attempts5,5,9 for each trace model; independence and
pairing checks are explicit. The field compositum with the F32 split-cover
identification is F_(2^840). That identification has not been evaluated
there, and no F2 descent of this finite cover is asserted.

## G18 exact factored translation lifts

The ordinary diagonal bundle has factor degrees n=[9,63,315,855] and
Pic2 biases[1,1,1,1]. On each factor write

    D_i=(n_i-1)O+T, T=(0,1).

For a coordinate a_i of exact odd order d_i dividing n_i, let f_(-a_i,d_i)
be the Miller function with divisor d_i(-a_i)-d_i O. Put

    g_(T,a)(P)=(x(P)+x(T-a))/line(T,-a)(P),
    F_(a,i)=f_(-a_i,d_i)^(n_i/d_i)*g_(T,a_i).

If a_i=O its factor is1. The line through T and-a has third intersection
T+a. Dividing its vertical numerator cancels that point, so

    div(g)=(T-a)-(-a)-T+O,
    div(F_(a,i))=t_a^*D_i-D_i.

Therefore A_a s(P)=product_i F_(a,i)(P_i)*s(P+a) preserves the tensor
section space. The JSON exports all four generator tuples and complete
Miller addition-chain descriptors, including every line's input points and
sum, so these are exact factored rational functions. Billions of section
products or dense action matrices were not materialized.

For a=(s,t) and P=(x,y), the correction is explicitly

    g=(s*x+1)/(s*y+(t+s+1)*x+s).

Its normalization satisfies x(P+a)=s*x(P)*g(P)^2 wherever defined.
Native point addition checks this at48 coordinates per graph. A new
universal integer residual expresses the cleared difference as
A*curve(P)+B*curve(a)+2*C; exact cofactors are exported. Lean proves that
integer identity, its characteristic2 consequence and the rational identity
with all denominators nonzero. This supports the correction independently
of the sampled controls. The global divisor/Miller interpretation remains
written elliptic geometry.

For each of the four generators, G18 computes its period scalar at three
independent tuples and obtains exactly the same value. Successive exact
3/5/7 root extractions provide a scalar normalizing the operator to its
prescribed order3,3,105,105. Every extracted root and the final period
identity are checked. All six pairwise commutators agree at all three
tuples:18 controls per graph. No tuple is rejected in the accepted run;
the earlier stale-binary invocation and failed builds are retained.

The period and commutator ratios have divisor zero on a smooth proper
geometrically integral product. Thus they are constants; under the written
divisor bridge, the exact evaluations identify them. Trivial commutators
and normalized periods give a geometric kernel linearization on the
specified diagonal bundle.

## What this descends, and what remains

The factored invariant operator can be written

    product_(j=1)^4 (sum_(k=0)^(order_j-1) A_j^k).

There are99225 group terms before factoring. In characteristic2 the odd
group order is1 as a field scalar. Lean proves the general averaging map
is an idempotent projection onto invariants for any supplied representation
of this order. The line-bundle descent and Riemann–Roch arguments give
section dimensions8704314675 →87723 →243 for the cover and the19²
quotient stages. These dimension identities are checked; invariant bases
and the final243 target sections are not extracted.

A linearization differs from the canonical pullback linearization by a
kernel character. The current normalized lifts do not identify that
character against the original mixed-divisor trivialization. Compatibility
with the F32/F2 descent and the second19²-kernel action is also open.
The exact lift descriptors are a new descent input, not a completed proper
quotient embedding. Complete native field/Miller semantics and the global
abelian geometry are not claimed as Lean formalizations.

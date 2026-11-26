# The next discriminator: a sixfold point of an implicit base product

This is a classical product/multiplicity reformulation and a proposed
solver target, not a new solver or a novelty claim. It stays on joint-six
discovery and avoids reducing the query to rare original two-member rows.
No implementation, control, benchmark or campaign is scheduled here.
[pcc:unverified]

## Exact finite-set equivalence

Fix a public regular raw target R and a FINITE SET X of distinct regular
physical raw curve points. For now exclude O,+/-R and all original
projection/denominator exceptional strata. Work in the c_xy=1 chart with
five unknown coefficients c=(c0,c1,c2,c3,c5):

    f_c(P)=xy(P)+c0+c1*x(P)+c2*y(P)+c3*x(P)^2+c5*phi_R(P),
    F_R(c)=product_{P in X} f_c(P).

Every factor is a nonconstant affine linear polynomial in c: its c0
coefficient is1. Thus F_R is nonzero of total degree exactly |X|.
At any c*, each nonvanishing factor is a unit in the local polynomial
ring, while a vanishing factor has order1. Lowest homogeneous parts
multiply in a polynomial domain, so

    mult_{c*}(F_R) = #{P in X : f_c*(P)=0}.             (5)

Since f_c* is a nonzero section with pole bound H of degree6, there
can be at most six such distinct regular zeros. Hence the following are
equivalent, with the stated guards:

* c* cuts out six distinct members of X;
* mult_{c*}(F_R)>=6;
* all Hasse derivatives D^[alpha] F_R(c*) vanish for |alpha|<6.

In this distinct regular stratum, six zeros exhaust the degree6 bound.
Their divisor is equivalent to5O+R, so the elliptic divisor/group law
gives their raw sum R. This uses the written section/divisor instance
bridge; it is not a new native compiler or [8]-image proof.

There are binomial(5+5,5)=252 derivative equations, including order0.
This is a joint condition on unknown c, not an all-point GS interpolant
in a received-symbol variable. It is consequently outside DERIVATION.md's
stated certificate family. Classical Hasse/arrangement theory supplies
the algebra; the original-base product and solving cost are the work.
[pcc:unverified]

## A compact evaluation circuit would preserve its cost exponent

Let J=k[eps0,...,eps4]/(eps0,...,eps4)^6, dimension252. If an arithmetic
circuit of length L computes this EXACT F_R, evaluate it at c+eps in J.
Its output is exactly the252 Hasse derivatives of total order<6.
Addition takes252 scalar additions and naive truncated multiplication
at most252^2 scalar multiply-add slots. These are constant factors in
the base size. A division gate is legitimate only if its constant term
is nonzero; the inverse then follows from the finite geometric series
through degree5. Every such unit guard remains part of the original
solution contract. No inversion of a zero factor is allowed at the
very solutions being sought. [pcc:unverified]

This does NOT mean a suitable L-cost circuit is already present. The
supplied-section quotient-query recurrences evaluate related products
modulo a norm sextic. A resultant/norm of that remainder is a candidate
representation for F_R, but its exact equality up to a declared nonzero
factor, rational point domain, branch factors, multiplicities and guards
must be established before transferring its loop bound. A symbolic
monic norm may have specialization strata; retain original equations
and separate all required charts. The old source remains frozen.

## Two traps to exclude before coding a solver

1. **Aliases are multiplicity, not six distinct curve zeros.** A product
   over raw IDs with repeated physical points repeats the same affine
   factor. Six copies can give multiplicity6 from ONE simple zero.
   That does not establish the six-jet divisor or the desired group sum.
   Prove a physical squarefree product, or carry exact original jet and
   label constraints in separate strata. Never silently quotient away
   raw-ID witnesses needed by the original circuit. [pcc:unverified]
2. **252 equations is not bounded degree.** F_R has degree |X| and its
   alpha derivative has degree at most |X|-|alpha|. A small circuit or
   five inputs supplies no solving-degree bound. Differentiation keeps
   evaluation complexity within a constant factor; it does not extract
   c*, solve the system, list its points or bound failed work.

Both caveats are mathematical statements, not measured native behavior.
[pcc:unverified]

## Ordered work and acceptance

First derive the exact norm/resultant-to-F_R identity, including the
original q213156431,K20633 cosets and rational raw-R/[8]-target sectors.
Reject a proposed transfer if an undisclosed nonunit/alias factor changes
the sixfold locus. This is a rejection of that identity, not this entire
solver class. Then specify a separately versioned Rust circuit with
unknown c and a public target only; fixed controls compare coefficient
jets against a direct finite-set product and the unchanged original
six-member witness. No known roots, labels, scalar or section coefficients
may enter the DISCOVERY API as answers. A jet EVALUATION API necessarily
takes a candidate c and must not be described as discovery.

Before a large solve, supply a non-enumerating selection/root algorithm
for the252 equations and an explicit q,K-dependent degree/work bound.
A cap, unsupported extraction or incomplete basis is inconclusive, not
an absence certificate. A local cost loss rejects that implementation
under its budget; only a genuine lower bound excludes the modeled
algorithm family. Raising caps or enumerating triples is not the next
step. Complete cold setup, failures, memory, index recovery, original
row checks, useful target distribution, rank/private-tag/kernel and
scalar recovery remain charged. [pcc:unverified]

The earlier conditional model gives ell*B^(gamma-5)+B^2 and optimized
exponent2/(7-gamma); gamma<3 is its sufficient solver target. Holding
B~ell^(1/6) instead requires gamma<2 at that model point. Neither the
implicit product,252 derivatives nor these inequalities establishes
either gamma or useful yield. Do not change the frozen route criteria.
[pcc:unverified]

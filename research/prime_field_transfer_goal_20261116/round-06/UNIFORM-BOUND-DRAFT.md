# Candidate uniform recognition bound — derivation for review

This is an unproved proof obligation, not a completed theorem, native guarantee,
novelty claim or efficient arbitrary-curve reduction. The capped balanced Padé
program version07 does not implement the constant-denominator variant below.
Do not promote this draft without checking the arithmetic hypotheses, full
derivation and the generic-law/formal-library boundary. [pcc:unverified]

## Existing sources checked

LLL1982, section1 short-vector guarantee and section2 factor/lattice separation,
were read from the CWI primary preprint. Mathlib4.24 source at pinned revision
f897ebcf72cd16f89ab4577d0c826cd14afaafc7 was read for:

- `Algebra.discr_not_zero_of_basis`, RingTheory/Discriminant.lean;
- `Algebra.discr_mul_isIntegral_mem_adjoin`, same file: for an integral generator
  of a power basis over a fraction field of an integrally closed domain, the
  discriminant times any integral element belongs to the generated algebra;
- `Algebra.isIntegral_trace`, RingTheory/Trace/Basic.lean;
- the Cramer-rule proof and `mulVec_cramer` used by the existing discriminant law.

These are source/dependency references, not a type-checked new instance or proof
of this proposed bound. KB remains unavailable. Missing reusable reconstruction,
norm-divisibility or LLL laws must be built generically in crypto-lean first.

## Proposed hypotheses

Let K=Q(omega), O_K=Z[omega], omega²+omega+1=0. Let C be monic irreducible
degree d≥1 over K and suppose L=K(W), C(W)=0, contains all roots of C.
All roots in both complex coefficient embeddings have absolute value≤M,
where M≥1 is a known integer (a coefficient Cauchy bound is a candidate).
Choose an integer D≥1 clearing the Eisenstein rational denominators of C.
Then theta=D*W has monic integral polynomial h(Y)=D^d C(Y/D).

Assume a fixed embedding L→Q_p, with W and omega integral there, and a simple
source residue of C selecting an input Hensel root t_Q. Normality makes this
the image of a unique root beta of C in L. No scalar is an algorithm input.
The CM interpretation and scalar recoverability are separate obligations.

## Trace/Cramer coefficient-bound candidate

Put R=D*M, T=d*R^(2d-2), V=d*R^d. The trace matrix of
1,theta,...,theta^(d-1) has entries bounded by T under either complex embedding.
The right side Tr((D*beta)*theta^i) has entries bounded by V and is integral.
Let Delta be the trace-matrix determinant and delta=Norm(K/Q)(Delta).
Separable power-basis nondegeneracy gives Delta≠0. Integral trace and Cramer's
rule are intended to give

    D*delta*beta = sum_j z_j D^j W^j,   z_j in Z[omega].

The candidate bounds, to be checked independently, are

    |Delta| ≤ d! T^d,
    |z_j| ≤ (d!)² V T^(2d-1),
    |delta| ≤ (d!)² T^(2d).

For z=a+b*omega use the elementary coefficient bound |a|,|b|≤2|z|.
Thus a candidate integer height for all coordinates of a nonzero exact relation
A(W)−m*beta=0, deg A<d, is

    H=max(2 D^(d-1) (d!)² V T^(2d-1), D (d!)² T^(2d)).

This is the part missing from a separation argument alone: it proposes a
model-only bounded relation for every root, using trace/discriminant data rather
than assuming that a small Padé representative exists. The coordinate statement
and height estimates still need a faithful formal chain; adjoin membership alone
must not be substituted for the degree/coordinate bound.

## Constant-denominator lattice and norm-separation candidate

Use the existing2d+1 integer congruence lattice with coefficient pairs for
1,omega,W,omega*W,... and scalar denominator m:

    A(w_P) − m*t_Q = 0 mod p^N.

If standard uncapped delta3/4 LLL returns its first basis row, the intended
LLL bound gives every row coordinate at most B=2^d(2d+1)H. Put

    S=2d*M^(d-1)+M,
    U=(D^d * B * S)^(2d).

For a row let F=A(W)−m*beta and alpha=D^d F. The proposed argument is:

1. alpha is globally integral, by theta and D*beta integrality.
2. Every complex conjugate of alpha has absolute value≤D^d B S.
3. The source congruence gives v_p(alpha)≥N at the degree-one selected place.
4. Therefore, if alpha≠0, its nonzero integer absolute norm is divisible by
   p^N but is at most U.
5. Choosing p^N>U would force alpha=0. A nonzero basis row then has m≠0,
   by power-basis independence, so beta=A(W)/m is recovered without its scalar.

The source norm-divisibility implication needs its precise number-field and
valuation hypotheses checked. If the old native denominator-unit filter is
kept, an additional p∤D*delta condition is a proposed sufficient hypothesis;
otherwise a versioned evaluator must allow certified local cancellation.

## Scope and next checks

This would concern complete recognition in an explicitly supplied normal field,
with a degree-dependent bound and uncapped LLL. It does not establish efficient
cryptographic-size evaluation, a guarantee for current balanced Padé/capped code,
arbitrary-curve applicability, or a target speed advantage. Do not run the huge
candidate precision blindly. First check the derivation, compute bound sizes only
in a separately declared native study, and decide whether it is a useful theorem.
No such calculation or guaranteed-recovery run has occurred in round06.

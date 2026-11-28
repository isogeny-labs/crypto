# Generic Kummer route: exact splitting test and model gate

Status: classical derivation and implementation screen, not a speed measurement, new priority claim or proof of the tenfold objective. The previous incremental-pencil run strengthens the baseline but leaves all base-field work unchanged. This route targets that remaining work on genus 2 without substituting a different torsion model.

## Projective splitting test

For a generic genus-2 divisor with Mumford polynomial u(X)=X²+a1*X+a0, classical Kummer coordinates satisfy kappa1=1, kappa2=-a1, kappa3=a0. In projective coordinates (K1:K2:K3:K4), with K1 nonzero, define

 Delta = K2² - 4*K1*K3.

The discriminant of u is Delta/K1². Since the denominator is a nonzero square, u splits over Fp exactly when the Legendre symbol of Delta is 0 or 1. Scaling every coordinate by s scales Delta by s², preserving this test. Thus a split rejection need not normalize projective coordinates or recover the v polynomial. This is an immediate classical discriminant identity, not a new decomposition algorithm. [pcc:unverified]

The input must be known to come from the original rational Jacobian: an arbitrary rational Kummer point can lift only on its quadratic twist. For valid Jacobian inputs, split u and rational v imply rational support points. Delta=0 requires multiplicities, rather than discarding repeated roots. K1=0 includes theta-divisor and identity cases and needs explicit projection/recovery handling. The quotient forgets the divisor's sign; a signed relation cannot be emitted from the split predicate alone. Recover signs using a valid differential neighbor and the known source/target coefficients, and charge that work.

## The fastest split theta model does not apply to the retained controls

Gaudry 2007 explicitly requires full rational two-torsion for its theta-coordinate fast model. Our three retained genus-2 Jacobians have certified odd prime order, so their rational 2-torsion contains only zero. The classical Kummer quotient is injective on geometric 2-torsion: a rational image of a 2-torsion point has a unique preimage, hence that preimage is rational. Therefore the retained Kummer surface has only one rational node, while the split fast model has 16. An Fp projective isomorphism preserves rational singular points, so an Fp linear coordinate change cannot turn this instance into that split fast model. [pcc:unverified]

An Fp-isogenous abelian surface has the same Frobenius polynomial and rational group order. It therefore also has odd order, and cannot acquire full rational two-torsion. A quadratic twist does not change the action on 2-torsion because negation is trivial there. These observations exclude the simple base-field isogeny/twist shortcuts to the split model, not every possible Kummer model. [pcc:unverified]

There is also an explicit module check for the retained irreducible quintics. Let B_j be the five finite branch-point classes relative to infinity. They generate J[2] with the single relation sum B_j=0. Frobenius cyclically permutes them, so its characteristic polynomial on J[2] is Phi5(T)=T4+T3+T2+T+1 over F2. This polynomial is irreducible (the order of 2 modulo 5 is 4). There is no invariant two-dimensional subspace and hence no rational order-four subgroup to serve as a Richelot kernel. Full 2-torsion becomes rational over Fp5; extending to that field changes the arithmetic and must be charged. This is not a universal prohibition on faster base-field models. [pcc:unverified]

## Eligible route and cost falsifier

A general Kummer model over Fp remains available; Müller constructs its arithmetic without the split theta assumption. A fixed-difference chain K(T+jD) could use differential addition from the previous two values. Membership can use Delta before signed recovery. This changes the walk and its coefficient bookkeeping, so correctness and useful-rank arrival need fresh controls rather than borrowing the old random-walk yield.

Before prototyping a full solve, obtain a concrete generic differential-addition circuit on the original models and count its base-field multiplication, squaring, normalization and exception costs. Include initial projections, both initial chain values, signed recovery on accepted points, roots, source/target bookkeeping, final scalar verification, and any restart/exception path. Compare against both retained pencil pivot strategies. If the field change, recovery, or initialization consumes the proposed advantage, reject that version before a benchmark. A cheap membership predicate does not imply a cheap decomposition generator.

The specific published split fast-theta port is inapplicable to these controls. The generic projective route is still an open candidate, conditional on its circuit and recovery cost case. No generic Kummer implementation or scientific experiment was run in this screen. The broad genus-2+ objective remains active, including genus-3/4 obligations.

## Sources checked

- Müller, Explicit Kummer surface formulas for arbitrary characteristic, sections 2–5, https://arxiv.org/pdf/0910.2589 : classical projective coordinates and generic biquadratic arithmetic. This provides an eligible base-field framework, not a tenfold cost result.
- Gaudry, Fast genus 2 arithmetic based on Theta functions, introduction, sections 3.4, 5 and 6, https://www.degruyterbrill.com/document/doi/10.1515/JMC.2007.012/pdf : rational 2-torsion requirement, nodes and field-of-definition limitation. The publisher PDF was recovered after ePrint/author mirror access failed. This rules out silently importing split-model counts.
- Chung–Costello–Smith, Fast, uniform, and compact scalar multiplication, section 6.3, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf : Mumford-to-general-Kummer projection and signed recovery obligations. Recovery is classical and must be charged.
- Corte-Real Santos–Flynn, Isogenies on Kummer Surfaces, sections 2–4, https://arxiv.org/html/2409.14819v1 : distinguishes General, Squared and Fast models, their explicit ground-field parameters and isogenies. A ground-field construction for its specified parameters is not a same-instance conversion certificate for an arbitrary quintic.
- KB revision 9b2119e8fbac482b98b50521c0d604bb, ready lexical index, refresh 2026-11-17T06:15:25Z predates this work. `source:ee9cafa5e47c2fe0c5a5b4511910f638`, crypto-autoresearcher:experiments/EXP-ECDLP-96c4dc/specification.yaml, metadata/context read, already names generic/fast conversion and recovery obligations. It is an experiment specification, not execution evidence or authorization for this task. `source:8ad98c30941f974367c5d1a3e86f47db`, ledger/proposals/IDEA-20260807-8027a2.yaml, metadata/context read, separates sign and representation issues and does not establish genus-2 acceleration. Initial exact queries gave broad or irrelevant results; no absence or novelty claim follows.

# Final consistency review of the continuation reduction

Reviewer: `/root/theory_review`, 2026-10-30 UTC. Read `research/prime_one_shot_20261029/continuation/README.md` as present for this review and compared it with `/workspace/audit/continuation-coverage.md`, the completed rationality/coverage reviews, and `/workspace/audit/continuation-torus-review.md`. No solver development, scientific experiment, or new timing run was performed.

## Verdict on the present README

The current README's **scalar-Kummer conditional reduction is consistent** with the reviewed proofs. It clearly separates actual exponential arity-three solving from the missing growing-arity sub-exponential oracle. Its field-family restriction, finite-cutoff requirement, rank-free recovery protocol and cost exponents are stated appropriately. No blocking correction is required to that Kummer-only claim.

The torus extension is presently described as under examination, so the README does not yet overstate its application. The integration checklist and explicit conservative torus constants below should be preserved when that note is added. In particular, the Kummer finite constant 192 must not be silently transferred unchanged to the doubled torus moment bound.

## 1. Conditional complete cost

For n=log2 r, m=sqrt(n)/2+O(1), and d the least 2-power at least 2(m!r)^(1/m),

    log2 d = 2 sqrt(n)+O(log n).

The actual signed base has B<=2d. The Kummer coverage lower bound rho0=1/(192m16^m) therefore gives

    log2((B+1)/rho0)=4 sqrt(n)+O(log n).

Classical dense nullspace work O(B^3) over F_r has cost

    2^(6 sqrt(n)+O(log n))

times polynomial bit-arithmetic factors. Base construction O(d poly(n)), target generation, relation verification, matrix construction/storage and final scalar verification fit below these qualitative sub-exponential overheads. The README's oracle-call and dense-algebra exponents are correct at the declared scale.

Consequently a **computably bounded, complete** oracle with uniform cost C(n)=exp(o(n)) on the relevant good input instances would imply an exp(o(n)) complete algorithm within this admissible family. It would not imply the sharper exp(O(sqrt(n) log n)) total unless the oracle and every other cost had a corresponding sharper bound. The README does not currently make that stronger inference.

This is a conditional reduction, not an implemented sub-exponential algorithm. The supplied coefficient solver has an explicit p(p-1) loop and only arity three, so it does not satisfy the oracle hypothesis. A small final geometric solution count, reducedness, a quadratic circuit, or a morphism-degree bound does not independently supply C(n).

## 2. Finite cutoffs, restarts and unknown good-base status

The proofs give a constant asymptotic fraction of good bases, not an efficiently recognized good-base predicate. A valid conditional algorithm can sample a base, apply a known per-call step bound, make a known finite number of fresh mixed-target calls, and restart the entire base block on timeout or insufficient valid relations. Every output relation must be checked independently; a timeout must not be reclassified as an UNSAT proof.

On a good base, a fixed complete oracle accepts precisely the selected-chart-decomposable targets, so fresh independently uniform R queries have success probability at least rho0. At most a constant multiple of (B+1)/rho0 calls suffice to collect B+1 successes with constant probability. This follows either from a binomial-tail argument or the expected negative-binomial waiting time followed by Markov. The oracle may select any valid model without changing that acceptance event.

The accepted b labels remain independent uniform conditional on the label-free oracle transcript. A kernel selected from C alone therefore has beta!=0 with probability 1-1/r, regardless of full column rank. Combining constant good-base probability, constant collection probability and the beta bound yields a bounded expected number of finite base blocks.

The README correctly requires a uniform computable cost bound usable as an actual cutoff. A non-effective assertion that “some exp(o(n)) bound exists” is not enough to specify an executable schedule. Bad bases must also be interrupted at the same finite budget, because a good-instance average alone does not bound their contribution to total expected work.

If the final protocol allows a label-free adaptive oracle rather than a fixed complete one, its acceptance/cost hypotheses must be uniform **conditional** bounds at every attempt, as the README already says. A single marginal acceptance mean is not enough. No a,b labels may feed acceptance, model selection, stopping decisions or kernel selection.

## 3. Admissible field family versus arbitrary prime fields

The chosen d has to divide p-1 in the scalar-Kummer proof. Its existence is a real input-family condition, not an automatic property of prime fields. A fixed compatible standard prime is not an asymptotic family. The statement is therefore about an explicit restricted family of whole prime-order curves over prime fields, not an all-prime algorithm.

The p+1/norm-one extension broadens the compatible input family, including a concrete large tower at P-256 because v2(p+1)=96. It does not remove the field-family restriction: arbitrary odd primes can have only small powers of two in both p-1 and p+1. It also does not establish a P-256 attack without the missing solver bound. The scalar-Kummer theorem must not be presented as automatically applying to every Dickson or auxiliary-isogeny tower.

The point-count/order promise is also explicit. The branch and rationality arguments use odd **whole curve** order. The direct recovery lemma uses a prime-order group generated by P. Replacing r by an arbitrary small prime subgroup while averaging coverage over all E(Fp) would change the target distribution and requires another argument. The README's initial whole-group assumptions avoid that mismatch.

## 4. Simultaneous torus events without hidden density factors

For the torus extension, choose H=mu_d inside the norm-one group T(Fp), |T(Fp)|=p+1, and sample g uniformly from that group. There are four events to track on the **same sample space**:

1. Trace distinctness: g^2 not in H. Its failure probability is at most

       epsilon_trace=2d/(p+1).

2. Usable rational-base density. The character-sum expression d+S(g) is a multiplicity-weighted count on trace-bad g and equals the actual signed base size on trace-good g. A valid unconditional density error bound is

       epsilon_den,T=4/d+40 sqrt(p)/(p+1).

   The trace event is added separately. There is no undeclared factor two in B: on a trace-good coset there are d distinct abscissas, each giving zero or two rational points.

3. Absence of short signed geometric zero sums. A conservative numerator-degree bound is twice the Kummer one, giving

       Delta_T=(p+1)^(-1)
         *sum_(s=3..m-2) binomial(d,s)*2s*2^(s-2),

   capped at one. The signs are already included in Semaev; no extra sign factor is added.

4. An adequate second moment and small selected-chart loss. Projection degrees double, so the off-diagonal collision constant is 2m4^m and chart-loss constants double.

These events need not be independent. Their failure probabilities can be combined directly by a union bound. Defining tuple counts to be zero on trace-bad bases is one clean way to state the moment theorem; alternatively count index-labelled tuples globally and identify them with actual supports only on trace-good bases.

If the implementation instead samples uniformly after the cheap trace test has rejected bad g, the other unconditional error bounds must be divided by the acceptance probability, or the entire argument must remain phrased as a charged rejection-sampling block. Since epsilon_trace tends to zero in the intended regime, the asymptotic consequence is unchanged, but that conditioning cannot be omitted from a finite formula.

## 5. A conservative finite torus coverage constant

The present Kummer finite constant rho0=1/(192m16^m) derives from its particular moment bound. For a torus integration, one safe parallel derivation is as follows.

Let L=binomial(d,m), kappa=d^m/(m!r), and assume p>=17, d>=8m^2 and kappa>=2^m. The density-good estimate gives the same support supply lower bound as before. The doubled moment bound is

    K_T = 2^m L/r + 2m4^m L^2/((p+1)r).

The Hasse bound gives r/(p+1)<=2, so

    K_T/kappa^2 <=1+4m4^m <=5m4^m.

The chart-loss Markov error, with the same half-supply threshold as the Kummer proof, can be taken as

    epsilon_chart,T=4*4^m*(2m^2+3m)/(p+1).

Using the moment event nu<=4K_T and the same Cauchy-Schwarz step gives

    rho_T >=1/(320m16^m).

A looser doubled-Kummer constant 1/(384m16^m) would also be safe. The exact good-base fraction is at least

    3/4-epsilon_trace-epsilon_den,T-epsilon_chart,T-Delta_T,

capped by the obvious probability limits. These explicit bounds are sufficient for a constant fraction of good bases asymptotically. They preserve rho_T=exp(-O(m)), so the displayed 2^(4 sqrt(n)+O(log n)) call count is unchanged after the torus extension. The finite numerical constant should nevertheless be updated or kept visibly Kummer-only.

The sparse-regime choice m=c sqrt(n), c<sqrt(2/3), and d=next power two above 2(m!r)^(1/m) makes all four error terms tend to zero, when such a divisor of p+1 exists. The extra trace term and doubled Semaev degree do not change the leading 3m-2n/m short-relation exponent. There is therefore no conflict between simultaneous torus rationality and coverage at these parameters.

## 6. Encoding details to preserve in torus integration

- The Dickson recurrence is D2(X)=X^2-2, with terminal g^d+g^(-d). A trace-good coset makes D_d(X)-terminal fully split and separable. The terminal may be zero; only +/-2 is excluded by trace distinctness. Do not import the scalar-Kummer “nonzero c” condition into this different polynomial family.
- Shared retained-quotient/remainder membership, not merely terminal Graeffe equality, is the radical divisor interface used by the reducedness argument.
- At fixed input, selected-chart N=(X-x_R)U is squarefree and gcd(N,f)=1. The tangent equation A delta A=fC delta C and degree bounds force delta A=delta C=0. Together with finite etale divisor membership and uniquely determined auxiliaries this proves reducedness of the full fixed-input norm/remainder scheme. It remains compatible with genuine Fp2 points outside the rationality-good event.
- Reducedness and rationality are separate. Neither property bounds Groebner work, root-extraction work, or the uniform oracle cost.
- The auxiliary Fp2 torus indexes rational abscissas and rational curve points. It does not replace the prime-field ECDLP by an extension-field ECDLP, but it is also not a generic model-free group algorithm.

## Exact requested corrections / outstanding proof obligations

1. **No blocking correction to the current Kummer-only README.** Its conditional nature and open oracle requirement are clear.
2. **When integrating torus, update its finite coverage constant** and include the trace/density/chart/short-relation errors on one declared sampling distribution. Keep 192 explicitly Kummer-only if retaining it.
3. **Retain finite cutoffs and full-cost restarts.** A good-base existence/probability theorem is not a recognition oracle or a guarantee about unbounded bad-base execution.
4. **Retain the field-family restriction and whole-group assumptions.** The p+1 extension is not an arbitrary-prime construction or a proved attack on a fixed standard curve.
5. **The genuinely missing algorithmic step remains uniform complete decomposition cost.** No new oracle construction, formal proof certificate or exhaustive novelty audit is supplied by these supporting reductions.

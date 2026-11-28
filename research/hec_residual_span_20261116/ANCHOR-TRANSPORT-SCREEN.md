# Automorphism transport preserves the existing component span

Status: [pcc:unverified] algebraic screening deduction, not an independently reviewed theorem or performance result. No new experiment executed. This screens the proposed reuse before admitting a new collection prototype.

## Precise question

Can one compute a target-bearing pencil once, transport its residual rows under the rational rotation, and obtain several independent target constraints at little additional cost? Distinguish whole-anchor transport from rotating just a source summand. The existing PENCIL-NEXT.md same-anchor invariance obstruction concerns a related but different mistake.

## Module statement and proof

Let sigma have finite order n on a module of divisor classes annihilated by ell. Work in an action algebra R=F_ell[Z]/m(Z), with m(sigma)=0 and Z a unit. Finite order gives Z inverse=Z^(n-1). For the binomial odd models one may use m=S_n=1+...+Z^(n-1) under the premises in homogeneous-cover/THEORY.md. In genus4 n9 retain S9=Phi3*Phi9 unless a checked projection removes a factor.

Write a homogeneous relation as a row v over R on fixed orbit representatives and explicit source/target variables. Applying sigma^i to EVERY divisor in the geometric identity produces row Z^i*v. Hence

span_R {v_j, Z^i*v_j for all i,j} = span_R {v_j}.

Inclusion in one direction follows multiplication by R scalars; the original rows are included in the transported set at i=0. This statement does not require R to be a field and survives zero divisors. It is about the R-module span of complete relations, not an arbitrary row-selection heuristic.

If m is squarefree, decompose into irreducible components K_j=F_ell[Z]/m_j. In each component transport is multiplication by a nonzero unit z_j^i. The K_j row span, coefficient rank, consistency and homogeneous target-witness availability do not change when these copies are appended. Linear factors give the existing scalar-root case. Irreducible higher-degree factors use K_j-linear rank; expanded F_ell comparison must already include the complete K_j-linear closure. A rise in a deliberately incomplete scalar expansion is not new action-component information.

This excludes a rank multiplier from pure automorphism copies. It does not exclude savings in geometric checking, storage, evaluation or factorization, or a different family of genuinely independent rows.

## The target-fixed family is different

For A=aC+bT, full transport is sigma^i(A)=a*sigma^i(C)+b*sigma^i(T). The proposed target-fixed anchor B_i=a*sigma^i(C)+bT differs by

B_i - sigma^i(A) = b*(T-sigma^i(T)).

Transporting an old decomposition of -A therefore gives a decomposition of -sigma^i(A), not of -B_i. Recovering the latter requires a decomposition/correction of the displayed difference, or fresh anchor/pencil generation. It cannot silently retain the old support.

On a scalar component sigma acts by r. Full transport changes the explicit coefficient pair (a,b) to (r^i*a,r^i*b), preserving projective direction [a:b]. Target-fixed source rotation changes it to (r^i*a,b). For a,b nonzero, these directions differ whenever r^i!=1. Thus target-fixed anchors can supply different normalization information, but this is precisely the information the free full-transport copies do not supply.

No universal nonvanishing assertion is made: T may vanish on a component, r^i may equal1, and genus4 can have order-three components. The correction can vanish in those cases. State and check such premises per component rather than discarding it. Correct homogeneous all-component verification remains applicable under its annihilator/membership assumptions.

## Relation to current evidence

- anchor-rank/THEORY.md proves the single-anchor scaling ambiguity absent independent source normalization. Equal projective anchor directions preserve that particular ambiguity; different directions can break it but do not by themselves prove full target rank.
- PENCIL-NEXT.md records the classical Sarkar-Singh pencil and the obstruction to treating a nonzero target-bearing pencil as invariant under rotation.
- orbit-incidence/RESULTS.md records the non-invariant incidence parameter and the cost of explicit orbit norms.
- singleton-phases/RESULTS.md shows that removing all quadratic split cost still leaves the current query/anchor remainder above the registered10x budget on all12 retained targets.
- Current component-span reasoning is elementary module algebra. KB query on rotated anchors/component rank returned source:e331f4a51e3488bf4c9154e923353096, crypto-autoresearcher:experiments/EXP-ECDLP-ad0ffe/specification.yaml, revision9b2119e8fbac482b98b50521c0d604bb. It is an unreviewed rank-accounting specification, not proof of this deduction. No external novelty or corpus-absence claim is made.

## Admission decision

Reject the proposed rank gain from rotating a fully computed pencil, without executing another fixture campaign. Retain target-fixed rotated-source anchors only as a distinct, unmeasured candidate requiring a public construction/cost argument and fresh useful-rank evidence. A factor of n cannot be credited from output count alone.

A successor must exhibit new rows outside existing component spans, account for correction/support generation, charge failed searches and normalization, and compare complete costs on separately frozen targets. It must also state how genus3/4 membership and action components are qualified. The full mathematical10x objective remains open; this screen rules out one proposed shortcut, not index calculus generally.

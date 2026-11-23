# Transported decomposition hypergraphs and hyperelliptic pair energy

Derived [pcc:unverified], not novelty or speed claim; independent review pending.

## Exact transport equivalence

Let q:H->H' be an injective homomorphism, with H'=q(H), and let B be an indexed factor base in H. For every target t, every length r, and every coefficient/sign pattern (a1,...,ar),

    sum ai*bi=t iff sum ai*q(bi)=q(t).

The map on indexed tuples is bijective, including repeated points and signs. Thus every representation count, sumset cardinality, collision multiplicity, additive energy and coefficient relation hypergraph is identical after transporting THE SAME base. Uniform target sampling is also preserved. This concerns all possible future decompositions, not just rows already collected. It requires injectivity on the relevant group, not necessarily on the whole ambient Jacobian. In the exceptional19 lane, coprimality of67931 and361 gives the needed target-subgroup injectivity. Raw affine point classes must be projected if they are not themselves in that subgroup; do not presume this for the genus3/4 examples below.

A different coordinate circuit may still make these identical decompositions cheaper to find. Exact transport equivalence is not a circuit-complexity lower bound. A geometric destination base other than q(B) can have a different hypergraph, but its points, inverse transport, unknown logarithms and relations require explicit construction/costs. Coordinate x-values alone do not form a group-homomorphic image; their additive energy in Fp is a different statistic and is not covered.

## Full signed affine degree-one base

Let C:y²=f(x) be a squarefree odd-degree hyperelliptic model, genus g>=2, odd characteristic, and use its rational Weierstrass point at infinity for Abel-Jacobi embedding. Let B consist of ALL affine rational points as signed Jacobian classes, m=|B|, w the number of affine rational Weierstrass points. For r(s)=#{(P,Q) in B²:P+Q=s}, exactly:

- r(0)=m, from Q=iota(P), including branch points.
- There are m-w nonzero sums of multiplicity1, the doubles of nonbranch points.
- There are (m²-2m+w)/2 nonzero sums of multiplicity2, from distinct unordered nonconjugate pairs.
- No other nonzero multiplicity occurs.

Reason: nonconjugate effective pairs already give reduced degree-two divisors when g>=2. Their reduced Mumford representatives are unique; distinct effective pairs cannot collide. Conjugate pairs reduce to identity. A branch-point double belongs to the identity case. This is the full Jacobian group, BEFORE arbitrary subgroup projection: projection can create collisions and invalidate this histogram.

Consequently

    |B+B|=1+(m²-w)/2,
    E(B)=sum_s r(s)²=3m²-3m+w.

The second formula follows by adding m² for zero, m-w for doubles, and four times the multiplicity-two count. These formulas hold for all g>=2, not just g2. They are properties of this full signed affine base, not of truncated coordinate-selected subsets or unions of bases.

For isogenous Jacobians of curves of the same genus over Fq, the common Frobenius polynomial gives equal curve point counts and hence equal m when both use odd-degree models. Such full geometric bases therefore have energy differing only by w; 0<=w<=2g+1. This prevents attributing a large raw-group pair-energy gain to isogeny choice in that particular full-base model. It does not determine higher-arity collisions, useful target rank, subgroup-projected energy, polynomial-solving degree or runtime.

## Bounded implementation check

Enumerate all ordered signed-point pairs on retained curves (g,p,c)=(2,251,18),(3,421,2),(4,271,2), without projecting to subgroups or claiming whole Jacobian orders. Check validity and the exact histogram/energy above. This validates consistency with the shared native implementation, not independent algebraic proof or a fast relation collector. No actual isogeny maps are evaluated.

Sources: native jac.rs/Poly reduced Mumford representation and GENUS2-TRANSPORT-BOUND.md primary Milne Abel-Jacobi references. The displayed transport proof is elementary group algebra; the pair proof is explicitly derived from reduced-divisor uniqueness. Existing anchor-rank/THEORY.md and production-cost/RESULTS.md retain normalization and strong-control requirements.

Sources checked (KB): ready lexical revision9b2119e8fbac482b98b50521c0d604bb, refresh2026-11-17T06:15:25Z older than work; query injective homomorphism/additive energy/sumset/isogeny/transported base. source:3deec8c6711152374352e734475ec1a0 crypto-autoresearcher:ledger/proposals/IDEA-20260807-e0ddc6.yaml concerns transported tables and coordinate energy, but has unverified predictions; source:0f6500b027b3fefd1be2d0783aea7015 crypto-autoresearcher:ledger/proposals/IDEA-20260811-7734c5.yaml contains recalled additive-energy directions. Neither is proof; distinguish coordinate energy from Jacobian-group energy as above. No novelty or corpus-absence claim.

# Complete-residual collision span in genera 2, 3 and 4

Status: written mathematical derivation, not independently validated or machine checked. No experiment was executed. This supplements POLE6-BRIDGE.md without changing any frozen source, protocol or manifest. No priority or measured speed claim.

## Statement and assumptions

Let y²=f(x), with f monic squarefree of degree 2g+1 over an odd finite field, g≥2, and rational infinity O. Assume J(Fq) has odd order. Choose H nonbranch rational anchor points with distinct abscissae, allowing both signs. Relation coefficients are over an odd scalar field. Coordinates identify conjugate rational places with opposite signs.

For every signed (g+1)-subset T of distinct anchor abscissae, interpolate h_T of degree at most g and put U_T=product(x-x(P)). Then

    f-h_T² = U_T Q_T,  deg(Q_T)=g,
    E_T=(Q_T,h_T mod Q_T),
    div(y-h_T)=T+E_T-(2g+1)O.

Q_T is monic. E_T is a valid, reduced, degree-g effective divisor: its v satisfies v²=f modulo Q, it contains no conjugate pair, and branch multiplicities cannot exceed one since f is squarefree. Repeated nonbranch factors are allowed with their scheme multiplicities. Uniqueness of the reduced Mumford representative applies.

Let S be all these rows whose Q splits completely over Fq, including multiplicities and rational residual places outside the anchors. Let K be pair-difference rows between triples/general subsets with the same oriented E, for every E whose Q does not split completely. Canonicalization up to sign must retain the complete Q and v. In genus 3/4 this includes reducible Q with nonrational factors; an irreducible-only cache is insufficient.

Let P be all rows of a-cy with deg(a)=g+1, c≠0, all 2g+2 finite zeros supported in the signed anchors, and each zero of multiplicity at most two. Scale a to be monic. Then

    span(S union K) = span(S union P).

The equality holds in the full rational-place coordinate space, hence also after intersecting with the anchor-supported subspace. It does not identify the raw candidate lists or count independent rows.

## Collision implies a pole-(2g+2) row

Orient matching residuals identically. Because deg(h_i)≤g and deg(Q)=g,

    h1-h2=cQ.

If c=0, the signed input divisors coincide: each is a degree-(g+1) sub-divisor of div(y-h1), and removing the same E leaves the same T. Its pair difference is zero. Otherwise set a=(f-h1*h2)/Q. It is a monic polynomial of degree g+1 and

    a²-c²f = U1 U2,
    a-cy = (y-h1)(y+h2)/Q.

These are polynomial/function-field identities, including repeated factors. The divisor of Q on the curve is E+involution(E)-2gO. Therefore

    div(a-cy)=T1+involution(T2)-(2g+2)O.

The function is primitive since c is a nonzero constant. It cannot vanish on both sheets above the same nonbranch abscissa. Each T has distinct x, so the surviving zeros have multiplicity at most two. Thus every nonzero K row lies in P. This argument uses full Mumford equality, not equality of Q alone, nor a heuristic equality of divisor classes.

## Every restricted pole row is obtained

Partition the 2g+2 zeros of a-cy into two sets T1 and Trest, each of size g+1 with distinct abscissae: assign the two copies of each double zero to opposite parts, then fill the equal remaining capacities with single zeros. Primitive constant-y functions have no vertical pairs, so this partition is possible. Set T2=involution(Trest).

Principality gives [T1-(g+1)O]=[T2-(g+1)O]. Interpolation produces E1 and E2 as above, with [E_i-gO]=-[T_i-(g+1)O]. Reduced uniqueness gives E1=E2, including all Q coefficients, v coefficients and multiplicities. Its class is nonzero because its reduced degree is g>0. Odd Jacobian order excludes a nonzero self-negative residual, so sign canonicalization has no torsion exception here.

If Q splits, subtract the corresponding two S rows. Otherwise the pair difference belongs to K, and can be generated from star differences against the first stored occurrence. In either case the residual cancels and the resulting row is exactly T1-T2, which is the original P row in signed coordinates. This proves the reverse containment.

An all-double pattern with only g+1 distinct zeros would give 2[T1-(g+1)O]=0. Odd order would make that class zero, contradicting its nonzero degree-g reduced residual. Thus a nonzero row in this domain has at least g+2 distinct abscissae and is reconstructible by ordinary degree-(g+1) interpolation at g+2 zeros. This explains the direct interpolation coverage; higher contacts, even-order Jacobians and branch anchors require separate treatments.

## Enumeration and cost boundary

The complete signed subset schedules, modulo whole-row negation, have sizes

    E_old = 2^g binom(H,g+1),
    E_direct = 2^(g+1) binom(H,g+2),
    (E_old+E_direct)/E_old = 1+2(H-g-1)/(g+2).

This is only the direct *distinct-input* schedule. To compare exactly the restricted P family, retain rows whose remaining zeros are anchored and whose multiplicities meet the assumptions. Complete coverage follows from the preceding distinct-abscissa argument. It does not imply all directly enumerated rows satisfy those filters.

Caching complete nonsplit E values during E_old generation spans this restricted augmentation with no E_direct norm schedule, at the cost of residual construction, canonical keys, dictionary storage/lookup, row emission and elimination. For fixed g, storing E costs O(g) field elements per label. The cached key is an entire reduced divisor, not an individual prime. It may have rational and nonrational components.

For the direct schedule alone to exceed the cached norm schedule by tenfold requires H≥21 for g2, H≥27 for g3, H≥32 for g4. These are exact candidate-count thresholds from the formula, NOT speed predictions. The baseline Compact does not enumerate E_direct, so none of these ratios is its improvement factor. Useful added rank may be zero.

There is also a worsening conditional birthday warning for whole residual keys. Under an explicitly hypothetical iid uniform model on M=Theta(q^g) signed full-residual labels, N outputs supply approximately N²/(2M) pair collisions in the sparse regime. If this route needs Theta(H) extra constraints and generates at most order H^(g+1) candidates, the optimistic count condition is H^(2g+1) at least order q^g. Thus the corresponding necessary candidate scale in that model is q^[g(g+1)/(2g+1)]: exponents 6/5, 12/7 and 20/9 for g2/3/4. This is not an unconditional bound or an observed distribution. It warns against substituting an order-q degree-one LP pool for a full degree-g residual pool. Factoring residuals and eliminating shared components could improve on whole-key matching, but then the theorem's star-cache cost/rank model is no longer that algorithm's analysis.

## Sources and next falsifier

Reviewed POLE6-BRIDGE.md and current ../hec_anchor_relations_20261114/rational.rs: d=1 takes g+2 distinct signed anchors and constant b, matching the direct family. Re-read sections 2–3 of Gaudry–Thériault–Thomé, 16 February 2005, https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf . Its LP pool consists of degree-one curve points (order q); it does not justify a uniform full-residual distribution or the present span claim. The proof above uses classical function divisors and reduced-representative uniqueness, with no novelty assertion.

Sources checked (KB): revision 9141e386254340a9bdba3036f2bb706a, refresh 2026-11-14T21:44:48Z, ready=true, lexical backend, hybrid_current=false. The snapshot predates these derivations; current files are authoritative. Queries: `hyperelliptic reduced divisor interpolation residual collision principal relation`; `hyperelliptic large prime composite residual relation rank limitation`. Returned source:10e38068649ede552491a9b73df2f477 (crypto-autoresearcher:experiments/EXP-ECDLP-a9b6a5/specification.yaml) and source:4548c0e85d47f9299ddc900c047c266d (crypto:docs/ic/ECDLP_RESEARCH_TECHNIQUES.md). Returned excerpts concern fixed factor-base identity and existing LP adapters, not a proof or priority receipt for this statement. No absence/novelty inference is made.

After the original unchanged genus-2 rank gate can pass its checkpoint prerequisite, separately freeze a direct/cached span comparison in g2/3/4. Include repeated factors, mixed rational/nonrational Q, complete (Q,v) sign normalization, split residual cancellation, and independent full-coordinate row-space checks. Reject this statement if any admitted direct row lies outside the augmented cached span, or any nonzero cached row fails the polynomial/divisor construction. Do not execute that new study under the original gate's protocol. Full setup, target descent and scalar solving remain outside this written enumeration argument.

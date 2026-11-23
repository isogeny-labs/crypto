# Genus-two isogeny transport: exact constraints on the proposed extra channels

Status: derived, [pcc:unverified], independent review pending. No new experiment or novelty claim. This addresses whether the four exceptional19 quotient candidates can supply cheap additional relation channels on the original target. Their exact geometric realization remains conditional/unconstructed; the theorem below applies if such polarized isogenies exist.

## Statement and proof

Work over an algebraic closure of a field of characteristic p. Let C,C' be smooth genus-two curves, embedded in their Jacobians by Abel-Jacobi maps. Write Theta,Theta' for their canonical principal-polarization classes. For genus two the embedded curves themselves represent these divisor classes; Theta²=2. Let q:J(C)->J(C') be an isogeny with q*Theta'=n Theta numerically, n>1, and deg(q)=n² prime to p. For ANY fixed translate D'=t+image(C'), at most2n geometric points of C, counted without multiplicity, have q(image(P)) in D'. Multiplicities can only decrease the number of distinct points.

First C cannot map entirely into D'. If it did, the embeddings give a finite nonconstant map f:C->C'. Since q is etale, its restriction has nonzero tangent map, so f is separable. Riemann-Hurwitz gives2=2deg(f)+ramification_degree, hence deg(f)=1 and f is an isomorphism. Subtracting the value at the base point removes the fixed translate. The universal property of the Jacobian then identifies q with the induced Jacobian isomorphism f_*, contradicting deg(q)=n²>1.

Therefore q^-1(D') restricts to a nonzero effective divisor on C. Its degree is the intersection number

    image(C) . q*D' = Theta . (n Theta) = 2n.

This proves the bound. No assumption about generic position, rational-point distribution, mixing, CM, endomorphism rings or numerical theta evaluation enters the argument. Numerical equivalence suffices for the intersection computation. Separability is essential; this statement does not cover a characteristic-divisible isogeny.

## Application to the exceptional19 lane

Here n=19, deg(q)=361 and p=251, so the bound is38 for each fixed target Abel-Jacobi translate. The retained source has270 affine rational points. Consequently at least232 of these270 points cannot land in that fixed target degree-one locus. This is a conservative count ignoring whether the source point at infinity consumes an intersection multiplicity. The statement concerns signed points, not canonical +/- orbit representatives.

If C' has an odd-degree model with a rational Weierstrass base point, its standard degree-one Mumford classes, together with identity, are precisely this symmetric embedded curve. Thus the whole transported degree-one source factor base cannot become the destination's usual degree-one factor base. If there is no such target model, the theorem still bounds any fixed Abel-Jacobi degree-one locus; one must account for the actual destination representation rather than silently assume an odd-degree model exists. The theorem does not say that the other image classes are irreducible quadratics: they may split as degree-two divisors, and determining that is paid work. It also does not forbid constructing another abstract factor base from the transported points.

A union of r fixed translates has at most2nr source preimages by the union bound; do not silently promote the single-translate statement to a growing union or arbitrary divisor family.

## Transporting existing rows gives no new coefficient information

Let H be the source order67931 subgroup and H'=q(H). As gcd(67931,361)=1, q|H is an isomorphism onto H'. For any fixed group elements G_j in H and coefficients a_j modulo67931,

    sum a_j G_j=0  iff  sum a_j q(G_j)=0.

Thus copying a verified row through q, while using q(G_j) as the destination base, leaves its coefficient vector unchanged. Adding these copies cannot increase the rank of the coefficient matrix. The same holds for target-bearing rows: q(T)=k q(D) has the same k, but applying q to an already known relation does not supply another independent equation. This is elementary injective-homomorphism algebra, not a new attack or a lower bound on all isogeny algorithms.

New geometric destination-base decompositions can supply additional equations; they require explicit point maps, destination base construction, decomposition and cross-base logarithm accounting. Those costs and new unknowns cannot be omitted. The separate exact saturation census bounds success probability alone by1.849922387735 on the uniform full-source-base instance; it does not bound changed arithmetic, rank or nonuniform samplers.

## Decision and next construction requirements

Do not attribute a10x mechanism to four complex-distinct quotient candidates alone. Before spending more work on their exact CM reconstruction for this goal, provide either a concrete cheaper relation circuit with new useful rank, or a paid cross-base construction whose arithmetic, new unknowns and final verification fit the complete comparator budget. A transported-base representation is mathematically valid but preserves existing row information. A geometric destination base can differ, but cheap one-place transport is excluded as above.

Existing source-phases/RESULTS.md independently records32.8-71.0percent base-field verification shares on its exact12cases. Holding that final check unchanged already caps the possible base-field saving at1.409-3.053x even if all other phases become free. These are recorded implementation-specific operation bounds, not wall-time lower bounds or a theorem about larger fields. The source-phases proposed relation-certificate successor and singleton-prefix failed cost gate must both be respected; do not repeat those rejected constructions under new labels.

## Sources checked

Primary: Milne, Jacobian Varieties, Proposition2.3 (Abel-Jacobi closed immersion), sections5-6 (curve/theta class and canonical polarization), Exercise6.12 (Theta^g=g!), https://www.jmilne.org/math/xnotes/JVs.pdf ; Milne, Abelian Varieties, isogeny and polarization sections, https://www.jmilne.org/math/CourseNotes/AV.pdf . Read relevant passages; Riemann-Hurwitz and projection/intersection formulas supply the explicit deduction above. It is a use of classical geometry, not a novelty claim.

Current repository inputs: kernel19-lattice-v2/RESULTS.md, kernel19-invariant/RESULTS.md, genus2-saturation/RESULTS.md, source-phases/RESULTS.md, singleton-prefix/RESULTS.md, production-cost/RESULTS.md. These are authoritative records at this checkout, not independent review.

Sources checked (KB): lexical revision9b2119e8fbac482b98b50521c0d604bb, last known refresh2026-11-17T06:15:25Z predates this deduction. Queries: genus2 index-calculus sieving/relation collection; Jacobian minimal class/theta/isogeny intersection/factor-base transport. source:1c8daaebce1785877fafc9ad73855272, crypto:research/notes/index-calculus/RESEARCH_AUTOLAB_LOG.md, and source:24b06267948ecf652481851a67520c8b, crypto-autoresearcher:ledger/proposals/IDEA-20261005-1e54b5.yaml, returned unreviewed proposals, not proofs. The primary Milne statements and explicit deduction are used instead. Improved-sieving PDF browser fetch timed out; existing PENCIL-NEXT.md records that its primary sections2.2-2.3 already describe the implemented classical pencil, so that construction is not proposed again as new.

# Genus-two relation-span bridge: quadratic collisions and pole-six functions

Status: post-freeze mathematical derivation. No new finite experiment has run. This does not amend the pending native gate, certify its outputs, or establish a complete IC speedup. It is a consequence of classical divisor/interpolation identities; priority is not established.

## Setting and row families

Let C: y²=f(x) be smooth over an odd finite field, with f monic of degree five and a single rational infinity O. For the span statement below assume the rational Jacobian has odd order. Let A consist of H rational points with distinct abscissae; both ordinate signs are available. There are then no finite rational branch points, and no nonzero rational two-torsion divisor classes. Work over the odd scalar field used for relation elimination.

For every signed triple T of distinct anchor abscissae, let h_T be its unique polynomial interpolant of degree at most two. Write

    f-h_T² = U_T Q_T,

where U_T is the monic product of the three input x-factors. Q_T is monic of degree two and E_T=(Q_T,h_T mod Q_T) is a valid reduced divisor. Its principal identity is

    T + E_T - 5O = 0.

R5 is the row family from all triples for which Q_T splits rationally, retaining all rational residual places even when outside A. C2 is the anchor-only row family obtained by eliminating equal nonzero irreducible quadratic E_T labels (up to the hyperelliptic sign), as in the prepared cache gate. Star differences against the first occurrence span all pair differences in each label class.

R6 is the family of rows of primitive functions a(x)-c y, with deg(a)=3 and c nonzero, whose six finite zeros are signed points from A, each with multiplicity at most two. No zero at infinity is allowed. Multiplicity at most two is a real restriction; higher-contact functions are outside this bridge. The rows of R6 are supported only on A.

Under these assumptions,

    span(R5 union C2) = span(R5 union R6).

This is a span statement in the full rational-place coordinate space. Taking intersection with anchor-supported coordinates preserves equality. It is not an equality of raw candidate lists or a statement about the number of independent outputs.

## Forward identity: every nontrivial quadratic collision gives a pole-six row

Normalize the two triple relations so their actual residual is the same oriented E=(Q,v), not merely the same x-polynomial. This multiplies an entire triple/interpolant by its canonical sign where needed. The interpolants h1,h2 satisfy h1≡h2 modulo Q, hence

    h1-h2 = c Q.

If c=0, their norm quotients and signed triples agree and the collision row is zero. Otherwise c is a nonzero constant. Define

    a = (f-h1*h2)/Q.

This is a polynomial because h1*h2≡v²≡f modulo Q, and deg(a)=3 with monic leading term. Direct expansion gives

    a²-c² f
      = ((f-h1²)*(f-h2²))/Q²
      = U1*U2.

Furthermore, in the function field,

    a-c y = ((y-h1)*(y+h2))/Q.

Its divisor is T1+involution(T2)-6O, the same signed row as T1-T2. Since Q is irreducible, it has no anchor abscissa root, so the divisor cancellation introduces no ambiguity at rational anchors. Each input triple has distinct abscissae; the output has multiplicity at most two and no vertical pair because c is nonzero. Thus each nonzero C2 row belongs to R6. Global signs of the row do not change its linear span.

## Converse: split a pole-six row into two triple relations

Take a function from R6. Divide its six zeros into effective triples T1 and Trest so each triple has distinct abscissae: put the two copies of any repeated zero into different triples, then distribute the remaining single zeros to make three in each. Set T2=involution(Trest). The principal divisor gives [T1-3O]=[T2-3O].

Interpolate h1,h2 on those triples. Their reduced degree-two residuals E1,E2 have the same Jacobian class, and therefore the same Mumford representative by uniqueness. The residual is nonzero: a valid reduced degree-two Mumford divisor is not the identity. Odd Jacobian order rules out a nonzero two-torsion residual, so its v is not zero.

If that common quadratic splits rationally, both triple identities occur in R5 and their difference is the R6 row, with rational off-anchor places canceled. If it is irreducible, both triples have the same canonical C2 label and the row is a pair difference, spanned by the cache's star differences. All target-free sign classes are included up to whole-row negation. Hence R6 is contained in span(R5 union C2), proving the equality with the forward inclusion.

The all-double, three-abscissa exceptional zero pattern would make twice a nonzero triple class vanish; odd order excludes it. Thus a nonzero R6 function in this domain has at least four distinct abscissae and can be reconstructed by ordinary cubic interpolation at four of its zeros. If branch/two-torsion classes or higher multiplicities are allowed, extra cases and possibly confluent interpolation are needed; they are not silently included here.

## Algorithmic consequence and precise comparison boundary

The repository's genus-two d=1 rational family has deg(a)≤3, deg(b)≤0 and four input anchors. For b≠0 it is ordinary cubic interpolation, so exhaustive four-anchor generation covers R6 (as well as other cases). The same enlarged row span can instead be generated by all quadratic triple norms plus endpoint lookup/elimination, subject to the domain above.

Scheduled sign-class counts are

    E5 = 4*binom(H,3),
    E6 = 8*binom(H,4),
    (E5+E6)/E5 = (H-1)/2.

These are algebraic enumeration counts, not measured time. The cache uses E5 norm candidates already needed for R5, plus O(E5) endpoint storage/lookup and emitted-row operations (or a logarithmic dictionary factor for deterministic lookup). Direct exhaustive augmentation adds E6 cubic-norm candidates. This reindexes this matched restricted span augmentation from order H^4 candidate enumeration to order H^3, without asserting that the baseline IC solver previously paid H^4. Existing Compact already uses only R5-style candidates; the factor above is NOT its speedup.

The gain can be zero if quadratic labels do not collide or if the resulting rows are already in the rational row span. The exact N-L+Z budget and conditional large-pool warning remain relevant. Matrix work, hash/storage cost, primitive filtering and complete source/target solving must still be charged. No tenfold end-to-end premise is established.

## General identity, without an unproved general span claim

For genus g≥2, two degree≤g interpolants with the same monic degree-g residual Q and the same oriented v again satisfy h1-h2=cQ. The same formulas give deg(a)=g+1 and

    a²-c²f = U1*U2,

a pole-(2g+2) function with a constant y coefficient. This algebraic identity extends directly. A complete higher-genus span statement needs its own hypotheses on partitions, torsion and multiplicities; none is inferred from the genus-two proof. A composite reduced residual key is also not an irreducible-prime label in higher genus.

## Validation still required

The pending compiled rank gate checks the quadratic cache and its rank budget, but does not enumerate R6 or construct the bridge polynomial a. Once approved checkpoint access is restored, its original finite gate can run unchanged. A separately frozen extension should then compare direct pole-six rows filtered to this domain against the cache-augmented span and check the displayed polynomial/divisor identities, including sign normalization and repeated-zero cases. No finite match or performance result is claimed now.

Sources checked: current ../hec_anchor_relations_20261114/rational.rs and RATIONAL-PROTOCOL.md confirm g=2,d=1 is four-anchor cubic/constant-y interpolation; current compact.rs, native Jacobian and prepared gate fix the triple norm conventions. New KB query hyperelliptic/interpolation/collision/relations returned source:b3c90d39a347ef661ccf10680a742186, an accounting/trajectory note, not evidence of this bridge's novelty. Search results for group-law interpolation identify classical adjacent work; the primary 2011/306 PDF fetch returned403. The accessible 2005 LP preprint was read for graph/background only, not as a source for this span claim. This explicit derivation is unvalidated by the blocked finite gate and is not a machine-checked theorem. The mathematical tenfold objective remains unchanged and active.

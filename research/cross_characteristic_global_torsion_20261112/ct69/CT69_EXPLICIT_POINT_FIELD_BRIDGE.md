# CT69 — Explicit degree-eight 11a1[3] point-field bridge across 29 and 2

**Standing: [pcc:unverified].** A bounded positive public toy control. Unlike CT51's rational 5-torsion and CT67's abstract 3-torsion Frobenius matrix labels, this provides a nonrational algebraic torsion point with a single explicit degree-eight number-field presentation, a selected source prime, and a specified binary residue representation. It does not scale to arbitrary high-order encoded inputs without a lookup. [pcc:unverified]

## Global 3-torsion point

Let E/Q be the Cremona 11a1 curve y²+y=x³−x²−10x−20, whose 3-division polynomial is ψ₃(X)=3X⁴−4X³−60X²−237X−21. Eliminating X from ψ₃ and the Weierstrass equation gives

\[
H(Y)=27Y^8+108Y^7-4813Y^6-14817Y^5+162543Y^4+349907Y^3+267096Y^2+89655Y-101306819.
\]

Put K=Q[Y]/(H), let β denote the class of Y, and define

\[
X=-\frac{108\beta^6+324\beta^5-19603\beta^4-39746\beta^3+332454\beta^2+352381\beta+15208005}{4992581}.
\]

The Python executable checks, using **exact rational polynomial remainder arithmetic**, that ψ₃(X)=0 and β²+β=X³−X²−10X−20 in K. It also verifies that Y→−Y−1 preserves H. Thus T=(X,β) is a nonzero geometric 3-torsion point, and −T=(X,−β−1). H modulo 2 is irreducible of degree eight, proving [K:Q]=8. The full 3-division field has degree 48 by the separately established CT52 full-image result. [pcc:unverified]

## Source and destination specializations

Modulo 29, H has precisely two linear roots: β→5 and β→23; the remaining roots occur in quadratic factors. Selecting the β→5 residue prime gives X→18 and P=(18,5) in E(F29)[3]. The conjugate β→23 corresponds to −P=(18,23). The source curve has 30 points. [pcc:unverified]

Modulo 2 the polynomial is

\[
M(Y)=Y^8+Y^6+Y^5+Y^4+Y^3+Y+1\quad(0x17b).
\]

Choose F256=F2[β]/M, with β encoded as the byte 2. The same rational expression for X reduces to β⁴+β+1, encoded as 19. Consequently the binary point U=(19,2) has exact order three, with 2U=−U=(19,3); these are finite-field element encodings, not ordinary affine integer values. The reduced curve E/F256 has 225 points. The eight Frobenius powers β↦β^{2^i} enumerate the eight possible residue-coordinate orientations. [pcc:unverified]

The selected global reductions send T to P and U, and −T to −P and −U. This is a concrete **characteristic-zero-mediated abstract torsion correspondence**, not an algebraic morphism between distinct-characteristic finite fields. [pcc:unverified]

## Public-input toy solve and cost limit

The program accepts the complete encoded source order-three subgroup {O,(18,5),(18,23)}. It uses a **three-entry coordinate/sign lookup** to select {0,T,−T}, reduces to {O,U,−U}, and exhaustively recovers all three target discrete logarithms {0,1,2}. It also independently counts the 30-element source and 225-element target groups. No private scalar is passed into the transfer routine, but the lookup directly distinguishes the three possible scalars. Consequently it is **not** a scalable logarithm-free lift or an improvement on optimized rho. General source subgroup sizes would incur an unbounded root-selection problem; fixed-11a1 full-image oriented point fields have degree ell²−1 (CT52), while input-dependent curves and higher genus remain open. [pcc:unverified]

The exact Python Q-algebra checker and independently authored JavaScript finite-field/point-count checker were run in normal and alternative optimization modes and produced **byte-identical** JSON on the producer-local host, SHA-256 e75985028d47468b26465c0107e80ac2b4029595783884d07c470b2850a03283. The JavaScript checker independently enumerates all 256² binary affine pairs and validates the finite-field modulus and all three encoded logarithms; Python adds exact characteristic-zero ideal membership. This is a reproducible local audit, not an independent PCC science signature. [pcc:unverified]

Reproduction from repo root: with D=research/cross_characteristic_global_torsion_20261112/ct69, run python3 -B "$D/ct69_pointfield.py"; python3 -O -B "$D/ct69_pointfield.py"; node "$D/ct69_pointfield_independent.js"; node --no-opt "$D/ct69_pointfield_independent.js". Compare their stdout byte-for-byte, as enforced by .github/workflows/ct69-pointfield.yml. [pcc:unverified]

## Provenance and next research obligation

Prior primary repo records: CT51 rational torsion, CT52 global 3-torsion Galois image, CT67/68 Frobenius orientation gates, root [crypto PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), cross-characteristic [PR #4015](https://github.com/isogeny-labs/crypto/pull/4015), and related cryptanalysis/autoresearcher cross-field PR searches. External references: [Snowden, elliptic curves over DVRs](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html) (finite-etale torsion reduction) and [Shoup, generic-group DLP bounds](https://www.shoup.net/papers/dlbounds1.pdf) (does not apply as a concrete speedup theorem here). This construction is elementary symbolic elimination; no novelty in general torsion lifting is claimed. Shared research-KB MCP and checked-out wrapper were unavailable; targeted GitHub research and the cited mathematical sources are a non-exhaustive fallback. [pcc:unverified]

**Remaining decisive gate:** replace the order-three scalar-revealing coordinate lookup with an input-uniform algorithm that selects compatible algebraic global preimages for arbitrary large-order encoded P,Q, including both decomposition groups and residue embeddings, without x. Price field degree, coefficient height, factoring, root selection, source map, destination solve and optimized rho/index-calculus baselines in a complete cold protocol. No paid compute, root-native edits or protected merge. [pcc:unverified]

# CT225 — absolute non-transfer to the complementary elliptic and surface factors

**2026-11-19 UTC.** Continuation of [CT224](CT224_SPLIT_FACTOR_OBSTRUCTION.md) on the existing [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). This is a **mathematical exclusion for the fixed F101 cubic cover**, not a complete solver or a timing result. **[pcc:unverified]**.

## Claim (stronger than CT224)

For the separable cubic cover
\[
C/\mathbf F_{101}:y^2=u^9+u^3+46\longrightarrow
E/\mathbf F_{101}:y^2=x^3+x+46,
\]
CT221–224 established, conditional on the independently recounted finite zeta data,
\[
J_C\sim_{\mathbf F_{101}}E\times E_0\times B
\]
with Frobenius polynomials
\[
P_E(X)=X^2+7X+101,\quad
P_{E_0}(X)=X^2+101,\quad
P_B(X)=X^4+195X^2+101^2.
\]

**The complementary factors remain geometrically orthogonal to E:**
\[
\operatorname{Hom}_{\overline{\mathbf F}_{101}}(E,E_0)
=\operatorname{Hom}_{\overline{\mathbf F}_{101}}(E,B)=0.
\]
Consequently, **no finite extension of F101** can make a *geometric homomorphism* from the source E to either complementary factor carry a nonzero source DLP. This improves CT224's F101 and F101² finite-point-group exclusions. It says nothing against the known nonzero pullback into the **full** Jacobian, an entirely different cover, or a nonlinear algorithm that is not an abelian-variety homomorphism.

## Proof

1. Over F101², E has Frobenius trace \((-7)^2-2(101)=-153\), with discriminant
   \[
   (-153)^2-4(101^2)=-17395=49(-355).
   \]
   Since \(101\nmid153\), E is ordinary and its geometric rational endomorphism algebra is the imaginary quadratic field \(\mathbf Q(\sqrt{-355})\).
2. E0 has trace 0 in characteristic 101 and is supersingular (for p>3). Supersingularity is invariant under algebraic closure and isogeny, so there can be no nonzero homomorphism \(E\to E_0\).
3. The roots of \(P_B\) occur in opposite pairs. Squaring Frobenius changes its characteristic polynomial to
   \[
   P_{B,\mathbf F_{101^2}}(Y)
   =(Y^2+195Y+10201)^2.
   \]
   By Honda–Tate, over F101² the surface B is isogenous to \(D^2\), where \(D/\mathbf F_{101^2}\) is an elliptic curve with trace \(-195\) (the Hasse bound is 202). It is ordinary because \(101\nmid195\); its Frobenius discriminant is
   \[
   (-195)^2-4(101^2)=-2779=-7\cdot397.
   \]
   Thus \(\operatorname{End}^0_{\overline{\mathbf F}_{101}}(D)=\mathbf Q(\sqrt{-2779})\), different from E's CM field. A nonzero homomorphism between elliptic curves over an algebraic closure is an isogeny, which would identify their rational endomorphism algebras. Therefore \(\operatorname{Hom}(E,D)=0\), hence \(\operatorname{Hom}(E,B)=0\).

The argument is about **geometric morphisms**, stronger than merely comparing group cardinalities over a few fields. We do **not** infer from it that arbitrary abstract homomorphisms between finite rational-point groups must vanish in all extension degrees.

## Independent arithmetic negative control: rational-group overlap eventually occurs

The standalone [CT225 exact script](ct225_geometric_hom_obstruction.py) computes \(\#A(\mathbf F_{101^n})\bmod109=\det(I-F_A^n)\bmod109\) using companion-matrix powers and modular Gaussian elimination, without a point-counting black box. Over degrees 1 through 1320, the **first** shared rational 109-divisibility is:

| Factor | First degree n with 109 dividing #factor(F101^n) |
| --- | ---: |
| Source E | 1 (and every n thereafter) |
| Complementary E0 | 24 |
| Complementary B | 1320 |

So, at extension degrees 24 or 1320, common 109-torsion can enable an **abstract finite-group** injection. This cannot be interpreted as a map of abelian varieties: the above geometric-Hom proof still forces every algebraic group homomorphism \(E\to E_0,B\) to zero. This is a concrete counterexample to using common subgroup order alone as an authorization for a chained algebraic transfer.

## Reproduction

\`\`\`bash
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct225_geometric_hom_obstruction.py --self-test
python3 -O research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct225_geometric_hom_obstruction.py --self-test
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct225_geometric_hom_obstruction.py > /tmp/ct225_hom.json
cmp /tmp/ct225_hom.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct225_f101_geometric_hom.json
sha256sum /tmp/ct225_hom.json
\`\`\`

Producer-local normal and optimized Python tests passed, including three altered-model negative controls and signed squarefree field checks; the deterministic output has SHA-256 **9ce82b4861d8d2437a177eb673772ce6a89267820025aa4307436f92c3aa0702**. The existing bounded CI workflow has been extended to replay and compare exact output on its checked-out commit. The new exact-head hosted CI verdict and independent PCC signature have **not yet been observed**. No paid compute was used.

**Theory references:** [Zarhin (2023), §5, Honda–Tate over finite fields](https://doi.org/10.4213/im9332e) and [De Feo et al., SIDH background on ordinary and supersingular endomorphism algebras](https://github.com/defeo/sidh-paper/blob/master/eprint.tex); the CT221–224 repo sources contain the specific curve and zeta calculations. Research-KB MCP is not available in this connector run, so this is not a complete prior-art search and no novelty claim is made.

## Performance decision

**Stop extension-degree searching for an algebraic homomorphism from the source E into only E0 or B in this specific cover.** Even extensions sharing 109-primary torsion are geometrically impossible destinations. Retain the source component inside full composite J_C and prioritize a native, fully cold composite-order genus-4 **index-calculus** solve with measured setup, offset relation collection, rank/LA, norm/pullback, recovery, peak RSS, and matched optimized rho/BSGS/IC. CT219's fully charged Python transferred toy solve was strongly negative and should not be repeated as if it were a new solver.

Original authenticated 695/701-record Jacobian norm-reuse pricing remains blocked by S3 authorization and approved budget gates. No substitute fixture is represented as genuine. No complete single-target ECDLP performance advantage is claimed. Keep PR #4715 draft and **[pcc:unverified]**.

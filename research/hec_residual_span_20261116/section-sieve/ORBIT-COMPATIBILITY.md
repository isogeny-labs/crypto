# Automorphism sharing: rank and pencil compatibility

This is an algebraic screen of a proposed orbit-size gain, not a measured result, novelty claim or universal sieve lower bound. It distinguishes the already qualified factor-base column contraction from a proposed reduction of the target-bearing rational-point scan. [pcc:unverified]

## Fixed-anchor obstruction

Let sigma fix infinity and act on the tracked prime-order component as scalar lambda!=1 modulo ell. A pencil for R has residual divisors E of a fixed degree with [E]=-R. Applying sigma sends the entire linear system to the linear system of -sigma(R). If sigma preserves that entire system then its divisor class must be unchanged, so (lambda-1)R=0 in the component, hence R=0 there. Thus a nonzero target-bearing anchor in this component has no automatic sigma-stable complete linear system. One cannot simply scan one point per orbit and assert that its images populate the same pencil by a universal parameter transformation. This does not exclude accidental intersections or a separately proved filter on a strict subfamily.

For the binomial model y²=x^(2g+1)+c and sigma(x,y)=(z*x,y), a degree-g reduced representative transforms as

    u_sigmaR(x)=z^g*u_R(z^-1*x),
    v_sigmaR(x)=v_R(z^-1*x).

Consequently, for t_R(P)=(y-v_R(x))/u_R(x),

    t_sigmaR(sigma(P))=z^-g*t_R(P).

This provides an exact equivariance between DIFFERENT anchors. It does not imply a formula for t_R(sigma(P)) from t_R(P) alone. On degree drops use the actual degree rather than g. The formula keeps support and signs; it is not an independent relation generator.

## Orbit-anchor rows collapse after contraction

Assume the qualified folded map L from signed factor-base divisors to coefficient vectors satisfies L(sigma E)=lambda*L(E), componentwise modulo ell. Let R=aD+bT, and E be a split residual, giving

    L(E)*X + a + b*k = 0.

The transformed anchor/residual gives

    L(sigma E)*X + lambda*a + lambda*b*k = 0,

which is exactly lambda times the original equation, including RHS and target coefficient. Every power of sigma produces the same one-dimensional row span. Normalizing a nonzero target coefficient makes these rows identical. On a scalar eigenspace, simply replacing anchors by their automorphism orbit therefore contributes no new rank after the column contraction already used by the control. Even without column contraction, the expanded representation must include the same automorphism constraints; dropping them is an information imbalance.

This is a statement about images of THE SAME relation. Independently searching other residuals of sigma(R) can reveal additional relations; the proof does not identify them with images of rows already found. But such searches must charge their point scans or provide a correct mechanism sharing them. Distinct scalar coefficient ratios or a public normalization row remain separate rank inputs, as in anchor-rank/THEORY.md.

## What remains eligible

Batched evaluation of u,v across an orbit can share powers, constants or transforms while still producing all required values; the obstruction above does not prohibit arithmetic savings. Likewise a multi-anchor algorithm may share computations across anchors that are not scalar copies of the same row. Neither implies an orbit-size saving without an explicit arithmetic DAG, useful-rank gate and full setup/verification ledger. Existing g2/g3 binomial and g4 C4 controls already contract columns with certified signed scalar weights, so that reduction cannot be counted a second time.

Decision: do not launch an orbit-copy relation campaign or drop all nonrepresentative point evaluations on this premise. A new candidate must demonstrate a same-anchor equivariance or an explicit shared evaluation circuit and must show new independent folded rows. Broad mathematical10xgenus2+goal remains open. No scientific process, S3 access or cloud resource was used.

## Literature and current-source receipt

Primary Improved sieving on algebraic curves, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf sections2.2–2.3: the residual class is fixed by the anchor and the sieve evaluates each rational point's parameter; this supplies the classical construction, not a claimed new orbit improvement. Current incremental-pencil/binomial.rs partition records the scalar lambda and signed orbit weights; normalized/RESULTS.md establishes equal public-source information in controls.

KB query `hyperelliptic automorphism orbit pencil independent relations rank`, lexical revision9b2119e8fbac482b98b50521c0d604bb: source:a99a3a2fcb07f95da98f94e175a602f6, IDEA-20260807-b42544, novelty_screen, already describes classical automorphism column quotients and the need to distinguish rank from hit rate. Its search-result assertions about external papers are not treated as independently verified sources here. No absence/priority claim follows.

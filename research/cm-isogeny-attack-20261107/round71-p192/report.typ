#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Round71: Kummer-surface response product
Written reduction plus local exact qualification; scientific standing unverified.
#image("flow.svg", width: 100%)
The product of n giant numerators is represented by four bivariate polynomials with individual degree at most 2n. Evaluating them at b signed baby pairs preserves both response coefficients, including singular jets. With published finite-field multipoint evaluation, the proof charges b+n squared plus lower-order public point, denominator and clipped-tail work. Balancing b approximately r to the two-thirds gives the stated exponent with arbitrary positive slack.

The Rust prototype uses classical sparse multiplication and direct evaluation. It preserves all 381 decoder outputs, 2694 response queries and 24 prescribed P-192 coefficients. Nine algebra controls pass. Fresh builds reproduce three science files and three diagnostics byte for byte; both coefficient mutations reject.

Across the 24 P-192 queries the prototype records 36087 quadratic multiplications, 213 inversions and 168 source additions. These are instrumented prototype counts, not evidence for the asymptotic backend. Legacy qualifier cache setup remains separately charged.

The first source preceded data. A metadata-only correction retained the initial output and verified unchanged scientific rows. No cloud, generator or unchanged retry was used. Full proof, exact scope, primary-source links and limitations appear in surface-product-proof.md and report.md.

A further written extraction sets U=G. Its unique zero index j satisfies j(G-Q)=G, hence k=1-1/j modulo r. A zero baby value plus an O(n) giant scan identifies the signed index. This removes an action-oracle loop from the written algorithm; zero-index extraction has not yet been qualified natively.

#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Signed-product recovery across changing radii
T-1899 · Prime-field optional recovery adapter · 2026-11-08

The signed reciprocal equation now runs through canonical phase/orientation state and a public-point recovery adapter. One coefficient traversal evaluates two reciprocal bases per active scaled branch. Existing source queries, endpoint handling, history replay and final point verification are retained.

#image("flow.svg",width:100%)

*Why transport matters.* At a new radius the variable is sV₀. Conjugation in the old chart maps it to s/V₀, whereas the required inverse is 1/(sV₀). The implementation evaluates F₁ at sV₀ and s⁻¹(W₀−V₀), then multiplies. Its equation can contain both old-chart components and is passed to the orientation-preserving filter. At s=1, the ordinary norm is used. Phase updates use ε=sᴸu².

*Exact qualification.* The first build and all checks passed: 96 transported quotient products, 3,640 exact image states, and 843 public target/cap cases recovered with 735 subsequent image updates. A negative control records 1,749 wrong-chart products. Repeated radii, zero scalar, ramified and repeated quotient factors are covered. Inherited stdout is byte-identical. These are finite correctness checks, not timing trials. Producer evidence awaits independent signed review.

*Formal scope.* Lean4.34.1 checks the scaled inverse-pair and transported-trace identities over an arbitrary commutative ring. Axioms are propext, Classical.choice and Quot.sound. The finite-field weighted projection and runtime refinement remain written/finite-check obligations.

*Cost.* For L coefficients at scale s≠1, the balanced pair evaluator uses 4(L−1)+1 quadratic products including the final product, plus one checked scalar inverse per active branch evaluation. Coefficient transitions, mode0, allocations/copies, canonical filtering, lift, source and replay work remain. A single coefficient traversal is not a single quotient evaluation. Candidate retention and full cost versus mode2 need a frozen matched comparison.

*Evidence.* transport.rs, generated signed-image.rs and signed-adapter.rs, image-checks.rs, DERIVATION.md, Transport.lean and validation01/receipt.json. No new build/runtime failure, cloud job, production panel or automatic method switch. Historical full-Cargo missing-fixture limitation unchanged.

*Sources.* Prior signed-reciprocal derivation and existing canonical-image, quadratic and persistent modules; NIST DLMF18.5.1, https://dlmf.nist.gov/18.5.E1. KB unavailable; exact-source fallback recorded.

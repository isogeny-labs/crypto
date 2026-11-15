# Independent source review: native signed collision bridge

The candidate normalizes both labels, retains the first pair, derives the scalar with the expected minus sign, and gates every successful return in its opt-in path on original-curve multiplication equality. These are source observations under the stated arithmetic/transfer premises; native behavior remains unexecuted. [pcc:unverified]

Reviewer did not author the candidate. Review inherits root's prospective plan at aadc4e94b8; its design was disclosed, so this is not a blind review. Candidate SHA256 is 708bb29ce8348ca73ee6ef039ec52e0504a17d2d901a88b4ad2eb44795369b33, published in 2e280f6b70 (producer df926cd122). The complete findings and exact file hashes are in result.json and findings.csv. [pcc:unverified]

## Derivation and source checks

For Q=kG and nonzero prime-order image P, equal canonical divisors give (A1-A0)+(B1-B0)k=0. Both A and B incorporate the canonical sign. Nonzero delta_b yields -delta_a/delta_b; zero delta_b with zero delta_a is redundant, and nonzero delta_a is an invariant conflict. For example, over F7 with k=3, label pairs (1,1) and (5,2) have equal coefficient 4; deltas (4,1) recover 3. Negating both labels and the divisor preserves its canonical pair. These are hand-derived examples, not executed native controls. [pcc:unverified]

The prior source is Handbook of Applied Cryptography, section 3.6.3, Algorithm 3.60, pages 106-107 (https://cacr.uwaterloo.ca/hac/about/chap3.pdf); its prime-order collision equation supplies the established algebra. Local DERIVATION.md and the preregistered review plan were read first; the primary source was checked directly. [pcc:unverified]

The first-label map uses vacant-only insertion; both prospective jumps are processed before choice, and restarts retain the fixed target context. First-occurrence graph insertion still uses raw edge labels, while duplicate recovery is a separate scalar-return branch. Duplicate recovery and prior identity/graph recovery all verify source_curve.mul(generator,candidate)==target. Observer errors propagate through Result before successful Outcome, including the final terminal event. The source curve clones the same Rc field counter, so source verification enters the final relation phase; scalar operations use the shared Ring counter. The false opt-in path has no new label-map allocation or verification arithmetic, although function/branch overhead and runtime equivalence have not been measured. [pcc:unverified]

## Boundaries requiring native qualification

L1: candidate lines 274-318 perform assertions and transfer before the new domain/order validation. Malformed input may panic rather than return Err. The assertions are inherited; the opt-in checks do not make this interface total, and the producer does not explicitly claim a total Err contract. This does not show an accepted incorrect scalar; bind valid fixtures and preserve panic receipts, or move validation earlier in a successor before treating Result as a total error interface. [pcc:unverified]

L2: candidate lines 468, 482, 502, 604 and 655 may return Err without Outcome/phases. Retained observer prefixes and successful source-verification events are not completed runs. An outer receipt must charge every failed attempt, observer failure and panic; missing costs stay unknown. [pcc:unverified]

L3: map counters are unpriced, and 24 value bytes per label omit keys, tree/allocator overhead and observers. The existing instance-field preflight check also lies outside the destination count closure. No total memory, timing, collision frequency or matched-cost comparison follows. [pcc:unverified]

L4: Ring::new asserts the delegated prime constructor; transfer homomorphism, exact image order and group operations remain premises. The source multiplication check uses the same arithmetic implementation rather than an independent checker. [pcc:unverified]

## Disposition and next checks

No sign or successful-return bypass was found by this scoped source inspection. Proceed to a separately authorized prospective native qualification covering both signs, both zero-denominator cases, source mismatch, immutable first labels, two-candidate order, restarts, all observer-error points and false-path equivalence. Preserve complete setup/failure accounting and the original full-cohort controls. Compiler, tests, candidate imports, experiments and new target runs were not executed by this review. [pcc:unverified]

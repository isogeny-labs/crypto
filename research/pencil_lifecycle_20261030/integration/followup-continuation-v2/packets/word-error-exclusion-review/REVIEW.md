# Independent bounded word error-exclusion review

All nine copied Lean modules (CanonicalResidue, UInt64Residue, NatClassifier, WordClassifier, CrossScaling, WeightedSameSupport, WordErrorExclusion, Readback, Controls) recompiled successfully in a private directory using the retained toolchain and pinned dependency search path. No copied producer olean was used for those nine modules. Separate readback prints only propext, Classical.choice and Quot.sound for the target theorem and canonical-word nonzero lemma. Producer manifest hashes were rechecked, and the complete producer packet including unsuccessful compile receipts was preserved in producer-copy.

## Statement fidelity

The theorem excludes exactly `crossPreludeWord p a b = some sameUMixedZeroSlope` under prime 5<=p<2^31, canonicality of all twelve words, nonzero four weight words, and normalized Mumford divisibility for a common polynomial. It does not assume support equality or coefficient identities. Its outer guard supplies cross-scaled support equality; the weighted validity theorem derives the required ordinate identities. Canonical word nonzero plus bounded reduction supplies nonzero field weights, and p>=5 supplies characteristic not two. The distinct-support None path excludes this particular error without being classified as failure.

Read Rust gate add22_weighted at /home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs lines 895–940 and the corresponding Field methods in its src/cryptanalysis/three_leads.rs. CrossScaling uses the opposite input's z^2,z^3,z^4,z^5,w factors exactly as the gate. The inverse test precedes equality; equality delegates to double22_weighted; only the later zero subtraction emits same_u_mixed_zero_slope_delta. WordClassifier preserves that ordering. sameUDouble denotes prelude delegation, not a returned doubling branch label. Under n=1, tables=None and odd p, table-free mul/sq and neg/sub value semantics agree with the bounded word model. Counter headroom, normal execution, compiler correctness and preservation of these input invariants remain external assumptions, as reported.

The normalized validity hypothesis states monic degree-two support divisibility. It is a real mathematical input condition; this theorem does not prove a collector constructs or preserves it. No curve-degree or squarefreeness hypothesis is needed for this local exclusion argument. It is not a proof of complete addition-law totality.

## Nonvacuity and negative control

Producer Controls compiles with p=5, coordinates (0,0,0,1,1,1), and f=X^5+X^2+1: support X^2 divides f-1, and the prelude selects sameUDouble. ReviewerControls2 additionally instantiates the full target theorem at that witness, discharging both validity predicates and all canonicality, bound and weight premises.

The reviewer negative control uses canonical coordinates badA=(0,0,1,0,1,1), badB=(0,0,1,1,1,1), p=5. Lean decides that the actual word prelude returns some sameUMixedZeroSlope. Thus the modeled error is reachable on canonical words outside the simultaneous validity hypotheses; the classifier is not defined to avoid it. This validates a meaningful premise boundary rather than claiming validity for those bad inputs.

The reviewer's first additional control compilation failed because decide did not unfold the opaque canonical predicate. Its source/stdout/stderr are retained unchanged. ReviewerControls2 unfolds that predicate and compiles successfully. No producer defect was involved.

## Outcome

No blocking discrepancy found in the stated bounded word-prelude exclusion. The independent compilation, source mapping and controls support this component claim. They do not establish reachable-state validity, counters/compiler refinement, whole-law output correctness or scientific execution. These are the same concrete boundaries stated by the producer.

Read PLAN before source/results; reviewed exact theorem/dependency text, producer failures, and retained source mapping. Review was not source-blind. No root edits, scientific runs or cloud work occurred.

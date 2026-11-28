# Conditional target consensus accessor passes synthetic gate

Pushed freeze ccc05211dab, single invocation exits0; all frozen hashes pass.
Eight scenarios,19prefixes,76independent field-component comparisons agree
for consistency/rank/k and resulting consensus.

Constant k=7accepted with4action roots and no unique eigenvalue. Varying k=Z
correctly rejected as nonconsensus. Empty survivors rejected. Consistent
zero-pivot component with free k blocks acceptance; adding k7then accepts
with4roots. A single inconsistent component is removed without discarding
the3consistent ones. Contradictory equal-left-side rows reject allcomponents.

PROOF.md derives true-action survival for certified whole-prime-order rational
Jacobians and exact conditional labels/CRT elimination. Consequently unique
surviving action is already the true action, and common kover all live
components is correct even without action uniqueness. This is a written,
unformalized argument using standard facts; synthetic tests qualify the
accessor, not all geometric hypotheses or a formally verified producer.

Current producer acceptance contract has NOT been changed. Old action scalar
checks remain charged in retained cost data. Proposed integration must guard
certified model/order inputs, retain [k]D=T, separately compare strict/consensus
outputs and complete counters, and report changed validation contract openly.
No action-check saving or10xresult measured here. No novelty claim, arbitrary
source/curve coverage, or genus3/4implementation qualification.

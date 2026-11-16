# Sealed producer outcome facts

A fixed typed OutcomeFacts snapshot is sealed only after successful terminal encoding. Terminal serialization and the parent-facing snapshot share one Claims value. Access fails after failed terminal retention or after moving the final artifact owners. The snapshot remains a producer report; independent replay is still required. Native invocation/statistics fields do not certify execution, basis derivation, or a complete operation trajectory.

The extra fixed state is included in the existing size_of<Step4State> resource admission. No new heap owner is introduced by the snapshot. Parent JSON construction and raw custody remain separate outer costs; there is no exact RSS or instruction-count claim.

The actual Bound producer test compares these claims with serialized terminal fields and independent replay for ordinary and exceptional witnesses. Existing final-confirmation-failure and terminal-retention-failure controls require the expected false completion and preserved witness acknowledgement. A post-transfer control refuses access after owners move.

Sources reviewed: prime_f4_wide_dynamic.rs Step4State, Claims, retain_step4_terminal, confirm_step4_witness and take_step4_artifacts; solvers.rs verify_dynamic_claims; OUTER_CLAIMS.md, CONFIRMATION_WORK.md and PARENT_FLUSH.md. This integrates existing custody contracts without a new mathematical or performance claim.

Predecessor d631d7ce159756f594098b628868535ff6b21654 compiled and passed585 focused tests,0 failed,7 ignored. This source has not yet compiled; focused validation and strict lint are pending. Production profile admission/runtime orchestration and native432 coverage remain unfinished.

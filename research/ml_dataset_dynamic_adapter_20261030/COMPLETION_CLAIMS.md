# Dynamic terminal completion claim review

Baseline4ed99e30c3c48deceb89105ff5958911eae00667. Source inspection of independent terminal(), producer retain_step4_terminal(), and Verified exposed a gap: a positive locally verified/durable witness could satisfy the solver_completed branch with disposition complete even when independent traversal was incomplete, because completeness was only required when group_domain_complete or negative_verified was separately claimed. Producer semantics require full group traversal for solver completion.

Tighten solver_completed acceptance to independently exhausted traversal, explicit group_domain_complete, complete disposition, finished search phase, IndependentTraversalFinished stop and cause none. Preserve mathematical group exhaustion independently of a failed final staging: those facts need not imply producer completion. Expose separately validated solver_completed_verified and negative_verified fields to the future parent outcome reconciler.

Authored regression mutates the real ordinary positive partial producer terminal to claim complete/solver_completed, rehashes the canonical file and descriptor, and requires file replay rejection. It leaves existing genuine ordinary/exceptional replay controls intact. Patch is outside-only and untested while original-input validation runs. This is a source-level correctness finding, not a completed malicious replay or a mathematical/fall-degree result.

Applied after original-input constructor source4ed99e30c3c48deceb89105ff5958911eae00667 compiled and passed584focused tests (0failed,7ignored). The completion fix and its new tamper assertion require fresh validation. No production profile activation or scientific result follows this commit.

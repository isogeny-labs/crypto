# Reviewable launch boundary

Both PR2381 and proposal PR3438 are merged. This new, focused execution-design branch is `codex/twist-chord-preflight`; no merged worktree, old claim or frozen output is advanced or overwritten. It is a linked follow-on, not a duplicate proposal or background writer.

The old upload approval is branch-scoped and does not cover this branch. The prior unanswered request for `codex/twist-collector-and-slope` also does not authorize a different prefix. Requested scope for this branch is ONLY:

    s3://setec-astronomy-artifacts/checkpoints/crypto/codex__twist-chord-preflight/

Allow only start, finish/failure and plan-change checkpoint writes through CURRENT UNMODIFIED `scripts/checkpoint.sh`. No arbitrary result assets, other branches, paid compute, alternate uploader/profile/destination or broad AWS credential reuse. No checkpoint command or upload has run here. Credential values must not be pasted in chat, task metadata, PRs, Git, logs or archives.

Current [AGENTS.md](../../AGENTS.md#resume-checkpoints-in-s3-before-every-run) says Codex must “run `scripts/checkpoint.sh gate` before launching and do not launch if it fails.” The gate uploads; without explicit new-prefix approval, the scientific/proof launch remains blocked. This does not prevent ordinary static design/metadata/document work. [pcc:unverified]

After approval persists for the exact scope, check configured access without reading or publishing token values; compare the official script against current main; bind truthful ignored RESUME to actual branch/head; require actual successful gate upload. Expired/denied credentials fail closed; no fallback or fake marker. Update RESUME from actual outcomes before finish/failure/plan checkpoints. Approval of the experiment is not permission to bypass its gates.

Conductor is not installed (ordinary CLI check exits127), and no applicable connected coordinator was exposed. No shared lease, reservation or replacement is claimed. Native/proof instruments, canonical freeze, source-bound identity and signed PCC gates also remain pending. The same single automation was rescheduled for this active follow-on; it cannot grant approval or launch dependent work. No duplicate automation or researcher is created.

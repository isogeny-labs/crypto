# Resume: fourteen-target theorem audit

## Goal
Audit all fourteen user targets and supply faithful Lean proofs; preserve all
requested exhaustive ranges, source checks and independence requirements.

## Lead / hypothesis
Exact corpus hypotheses may already exclude the proposed counterexamples.
BK explicitly excludes r dividing q-1. Existing S3_root_iff covers distinct x.
Test repeated x structurally, and preserve all other targets in the study map.

## Spec
research/theorem_audit_20261108/PROTOCOL.md. No paid cloud jobs; local proof
builds only so far. Original prior-turn finite evidence is outside this repo
at /workspace/theorem-audit and remains historical, not a corpus refutation.

## State
Sources read: corpus nodes, F4 model, round6 proof, S3Roots proof. Lean4.35 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 installed. New S3Completion and
StructuralAudit are being compiled, not yet claimed as checked.

## Next steps
In research/theorem_audit_20261108/lean run lake build; repair compilation,
then lake env lean Audit.lean and root tools/lean/proof-gate.sh for the target.
Read primary Diem Condition2.7, msolve selection/simplify source and remaining
heuristic hypotheses. Finish source/proof mapping, PCC records and a PR.

## Runs in flight
No research experiments. Local Lake dependency cache/update and proof build
are development commands in this checkout; tool sessions retain output.

## Stop and success conditions
Success requires the original fourteen-item acceptance criteria, not just a
proof component. No external compute spend. Never run a new research
experiment if checkpoint gate fails; record missing token/access concretely.
No native_decide, holes or axioms encoding the claimed result.

## Open questions
Full Jacobian and msolve refinement proofs remain open; another-machine
validator requires an actual independently controlled host. Check credentials
without printing them; don't infer independent verification from local runs.

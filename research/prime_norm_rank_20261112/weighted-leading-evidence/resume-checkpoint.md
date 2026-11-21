# Resume: crypto weighted leading norm proof construction

## Goal
Novel classical non-generic sub-exponential general prime-field ECDLP with
faithful Lean mathematics and complete expected bit-cost accounting.

## Lead / hypothesis
Pole weights4,2,3,1 for the existing four-coefficient membership projection.
Prove explicit leading support has only the zero coefficient tuple when
dividing a pure power, then check actual source binding. The low last-fall
bound is unproved; weighted input degree is not that bound.

## Spec
research/prime_norm_rank_20261112/WEIGHTED_LEADING_PROTOCOL.md.
Exactly existing public m5,t4,seed2 fixture over F14680063, reduced.ms SHA
9a5061c73c01cb74f397fcae980c351341cf60e237c896527771d1da0cf5ef3f.
Path research/prime_norm_rank_20261112/membership-elimination-evidence/native-output-001/reduced.ms.
Fixed symbolic construction only; no roots/branches/logs/paid/performance run.

## State
Branch codex/norm-membership-projection-20261113; PR4085 pending at23:02UTC.
Own maintenance scheduled23:51, no foreground wait. Existing projection
gate286/fresh0 and native evidence remain unchanged. Weighted contract
commit7d82d1872; new NormWeightedLeading.lean under development.
No weighted native artifact has run. Initial gate rejected missing resume.

## Next steps
In /workspace/audit/norm-recovery-completion-worktree, build Lean module with
PATH=/workspace/audit/toolchains/lean-4.35.0-linux/bin:$PATH lake -d research/prime_norm_rank_20261112/lean build NormWeightedLeading
Preserve build errors and receipts. Before native artifact run, freeze source
and run bash scripts/checkpoint.sh gate; never launch when it fails.
Produce unsigned PCC, proof map, editable diagram/SVG and inspected PDF.

## Runs in flight
None. Initial Lean invocation used an incorrect relative path and exited1;
no compiler or experiment currently running.

## Stop and success conditions
Explicit leading-support implication must be kernel checked. Source binding
is separate. Native caps256MiB/120CPUseconds/100million pairproducts/500000
terms; mismatch rejects proposed identification; cap is cap. No gate bypass,
force push, ledger rewrite, vault access, signatures or unrelated edits.

## Open questions
Original W-jet map, efficient public finder, weighted closure degree and
complete expected bit cost remain open. PR completion does not complete goal.

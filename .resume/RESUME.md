# Resume: crypto @ claude/prime-field-index-calculus-atxvot

## Goal
Continue the prime-field IC leads (research/prime_ic_leads_20261110/README.md §7) that run without EKS:
lead 3 (null-support slope), lead 4 (ready-to-run spec for the first algebraic-track row), lead 5
(compile the hash-table prefetch Design A patch). PR #3828 is open (auto-merge on, CI green so far).

## Lead / hypothesis
Null-support: prediction d_F(null) = d_F(tower) on every cell (shape-driven). So far (m = 2 t = 2..9;
m = 3 t = 2, 3): identical. Width ladder (--null-width 1..4) pending the agent's implementation.

## Spec
devcheck/run_null.log + null_*.jsonl (closure engine, Kummer P1, seed 0x504B4D54, 2 random targets).
Agents (Sonnet): null-width control in examples/pkm_tower_pilot.rs; algebraic_row_spec/ under the note;
prefetch patch analysis in the scratchpad (read-only), build pass after the pilot agent's build.

## State
Worktree /home/user/crypto, branch pushed through f1504a4029. No AWS identity (no EKS, no S3 from aws);
checkpoint push works via the upload token. KB, ecdb, cairn, Conductor unavailable.

## Next steps
1. Collect the null ladder (m = 4) and the agents' reports; run the width ladder; fold into the note §6/§7.
2. Prefetch build pass (sequenced after the pilot rebuild); commit; session log; push.

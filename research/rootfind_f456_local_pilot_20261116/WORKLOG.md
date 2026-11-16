# Work log

## 2026-11-16, protocol freeze

### What was done

- Checked current source and the frozen combined K13 L2 panel; the production collector files remain reserved by Conductor T-6708. Checked live correct-account AWS budgets: monthly USD 2,135.003 / 1,300 and daily USD 222.412 / 200, last updated 2026-11-16 06:15/06:16 PST. The L2 submission remains inadmissible. [pcc:unverified]
- Read the earlier local record-only K13 diagnostic in a separate worktree. Its `records.jsonl` has no corresponding committed invocation and session capsule in that directory, so it is treated as a lead rather than authoritative comparative evidence. Read the current source and KB revision/sources named in the protocol. [pcc:unverified]

### Decisions made

- Freeze a two-target, eight-arm L0 complete-call diagnostic using the exact registered curve and seeds from the L2 panel. It does not alter the L2 panel or its claim gate. Conductor T-6826 owns only this study directory. [pcc:unverified]

### Open threads

- Commit the protocol/spec, plan and build a clean source-bound binary, then pass a current branch-specific S3 checkpoint before any run. Conductor T-6814 still reserves `.resume/RESUME.md`; a scope-conflict readback blocked adding it to T-6826. [pcc:unverified]

### Errors

- None in the read-only setup. The checkpoint scope conflict is a run-admission obstruction, not experimental data.

## 2026-11-16, pre-data order correction

### What was done

- The cached release planner accepted the first spec but warned that one measured round leaves alternating arm order unbalanced. Saved its plan and spec as version 1. No timed run occurred. [pcc:unverified]

### Decisions made

- Amendment 1 increases measured rounds from one to two; this balances arm order within each fixed target while preserving every method, seed and target. The new plan has 48 scheduled executions. [pcc:unverified]

### Open threads

- Plan version 2, bind source/binary, and satisfy the checkpoint gate before execution.

### Errors

- The planner warning was a pre-data protocol defect, corrected additively.

## 2026-11-16, version 2 plan check

### What was done

- Cached-binary `ecbench plan --doc` accepted the amended spec with 48 scheduled executions, 16 warmups and 32 measured calls. `check_plan.py` compared both K13 workloads byte-for-byte with the first two workloads of the frozen L2 panel and checked all eight selected arm definitions. `PLAN_CHECK.json` records IDs, target points, spec/plan/binary hashes and the pending clean-build binding. [pcc:unverified]

### Decisions made

- Use this plan only as preflight until a clean source-bound release binary is built. The current cached binary hash is diagnostic and is not asserted to be built from this branch. [pcc:unverified]

### Open threads

- Build source-bound release ecbench when disk headroom permits, then obtain branch-specific `.resume` scope and S3 checkpoint gate before any timed call.

### Errors

- None in the version 2 planner; the version 1 arm-order warning remains preserved in Amendment 1.

## 2026-11-16, source-bound build and run admission

### What was done

- Preserved all three build attempts. The first failed under Homebrew Rust 1.93.1; the second was stopped at 300 seconds during active compilation; the third completed with Rust 1.99.1 at 14:52:53 UTC. `BUILD_STATUS.json` binds the exact source commit/tree, binary hash, command and all build log hashes. [pcc:unverified]
- Replanned with that binary and compared the plan byte-for-byte with the cached preflight plan. The first two registered workloads still match the frozen L2 plan; `PLAN_CHECK-BOUND.json` binds the new binary. [pcc:unverified]
- Rechecked Conductor's `.resume/RESUME.md` scope. T-6814 still holds it in `verifying`; no timed execution started.

### Decisions made

- Preserve the build and plan as runnable preparation. Do not use the stale resume to bypass the required current branch-specific S3 checkpoint. [pcc:unverified]

### Open threads

- Publish this preparation. When the resume path is released, write the new handoff, pass the checkpoint gate and execute/replay the one bounded local pilot. The L2 panel keeps its separate budget and isolation gates.

### Errors

- The first two build failures are infrastructure/toolchain outcomes, not scientific trials. The third build's missing `libLLVM.dylib` strip warning was nonfatal.

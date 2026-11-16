# Paired source-quotient capture and canonical export

2026-11-07, T-1846. Implemented adapter and synthetic software validation. **No native collector was executed and no workload trace was collected.** The valid draft is not frozen/admitted. Original4,563-case cohort unchanged.

## Completed path

The predecessor trace distinguishes source Fp quotient stages from scalar-field Modes work. This adapter executes the prospective collector twice, in separate processes, inside one admitted attempt directory. Both processes share a monotonic120-second child-execution deadline clipped to the admission expiry. The second process receives only the remaining time. Source/binary/protocol and adapter bindings are checked before execution; default is check-only. The CLI requires the canonical frozen pre-data protocol and expected hash. The capture entry also rejects unfrozen or changed protocols.

Each child receives PF_QUOTIENT_TRACE_ADMITTED=1 only on the explicit execution path after admission checks. This environment flag is an accidental-launch guard, not an authorization mechanism. The adapter neither submits jobs nor changes IAM, quota, Karpenter, cloud budget or resource limits outside its own child processes. It requires existing job-submitter/isolab admission with publication/capacity/budget evidence, exact attempt directory, one CPU/1024MiB/concurrency1, one attempt, proposedUSD1 reservation and sufficient finite remaining funds. Evidence JSON is not a signature or live verification; an authorized external operator supplies and verifies it. No real admission was created or funds reserved here.

Each child's stdout/stderr has an RLIMIT_FSIZE hard limit of16MiB (or a tighter inherited hard limit). Four potential output files therefore have a64MiB maximum payload, excluding small metadata. CPU/RAM and the outer attempt deadline must be enforced by the admitted job; this wrapper bounds child output and the combined child runtime, not all postprocessing/host activity. Collector arithmetic is single-process native code with no subprocess tree. After each child, record status, exit, attempted/executed flags, file hashes/bytes and command. An OSError launch attempt is not labelled executed. Timeout, nonzero exit or launch failure stops the pair; output and a shared receipt remain. Forced host termination may prevent receipt creation; a missing receipt is not success and never authorizes a duplicate attempt.

After two successful exits, the strict predecessor parser validates each entire617-target transcript. Raw bytes must also be identical. Malformed output or unequal valid transcripts yields invalid_data and retains both outputs. The record cap/overflow detector in the native collector prevents silent valid truncation. These are source-only quotient counts, not full source, scalar-field or timing totals.

## Canonical export

export.py accepts only a complete successful single-attempt receipt, the exact two named processes, correct source/protocol/binary pins, executed/zero exit states and matching bounded stdout/stderr file hashes. It reruns strict pair analysis. Output is write-once: analysis.json plus experiment.export/v1. There are1,234 rows:617 target instances for trace and617 for aa, seed0, one replicate, zero warmups. Counted fields are quotients, products and inversions per target. Both rows retain their own raw-file/receipt custody. Clock validity is false and verification.status is no_claim: producer checks are not independent review.

Failed or partial attempts remain raw files and structured failure receipts, never fabricated unexecuted target rows or successful exports. The canonical export's complete-cell requirements do not justify creating missing measurements. A separate future failure-export convention may be added if required; the failure receipt is already durable and explicit.

The draft binds the complete collector input closure from the predecessor build receipt, the strict analyzer, paired analyzer, adapter code and reusable admission/canonical authority. Production source/binary binding, admitted preflight, canonical freeze and publication before affected data, live budget/quota reconciliation and authentic admission remain before any workload execution. The development build is not silently promoted. These are the existing execution requirements, not new permission requests.

## Software validation

validation-01: six synthetic/mocked test methods passed. After inspection, refined attempted vs executed distinction for launch OSError and added explicit file-limit and prelaunch-deadline tests. validation-02: seven methods pass. No collector or other native arithmetic program is invoked; subprocess collector calls are mocked. Tests fabricate complete transcripts only in temporary directories.

Coverage includes:

- Valid canonical draft and refusal to execute it; sample-size/profile mutations.
- Check-only with no child/no output directory; complete two-process capture and1,234-row export.
- Upstream canonical check-export with --verify-files on the synthetic export, plus false wall-clock and no_claim labels.
- Shared deadline expired between processes: exactly one child attempted; second marked unexecuted timeout. Prelaunch expiry starts no child.
- Timeout, nonzero exit and OSError retain receipts and never launch a replacement or extra process; exports refuse these attempts.
- Malformed second transcript and structurally valid unequal pair both invalidate the attempt.
- Existing attempt-directory and renamed-directory refusal; raw-file tampering; incorrect binary or source binding; admission mutations for role, funds, resource types and expiry.
- File limit respects a tighter inherited hard limit.

There were no failed test runs. validation01's earlier capture implementation is preserved as capture-validation01.txt; validation02 covers the final implementation. Test duration text is software runner output, not a benchmark measurement. No full Cargo rerun: prior missing-fixture/deadline records remain.

## Reproduction and sources

Use the existing Python environment with jsonschema Draft202012Validator. From this directory run `python test-adapter.py`; all collector launches are mocked. `python make-draft.py` regenerates the unfrozen draft from pinned predecessor settings and current source hashes. Validate using the pinned `../feature-native-resultant-protocol/authority/tools/experiment_protocol.py validate protocol.draft.json`. CLI usage is available with capture.py/export.py --help. Never set --execute merely to test this adapter.

Sources checked (KB): unavailable (no MCP/wrapper executable). Read exact predecessor trace/parser/collector, prior resultant common/capture/export/tests and pinned canonical experiment-protocol documentation/schema. Reused its external admission requirements and canonical API. No algorithmic novelty claim. Current report/diagram are new; historical records and canonical research graphs remain unchanged.

Next reconcile production build/preflight admission and remaining budget, bind and freeze this exact prospective panel, then collect it when those existing conditions are satisfied. Until data exists, continue treating source quotient frequencies and aggregate savings as unknown.

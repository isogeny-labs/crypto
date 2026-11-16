# BSGS semantic-boundary maintenance - 2026-11-15

## 2026-11-15T22:53Z - BSGS planner and accumulated-cost boundaries

### What was done
Found two floating-point seeded square-root loops whose upward saturating-product condition remains true at u64::MAX. The exact old helper exceeds a bounded one-second local control; the owned control child was terminated. Replaced both loops with the integer square root already used elsewhere in this repository, sharing the helper between planning and reporting.

Cost reporting also summed three uint64 counters before conversion to floating point. The exact old code undercounts an overflowing accumulated sum in release and can panic in debug. Use checked integer addition for ordinary sums, preserving their prior rounding, and sum in floating point only when the integer total cannot fit.

The exact-source extraction passes both new boundary controls and 20,000 deterministic random square-root inequality cases checked with uint128 products. The first actual library replay passes 19 BSGS tests; final-source library replay, including the cost-sum control, is compiling. The maximum-width plan test does not allocate its table. No performance measurement or frozen experiment rewrite.

### Decisions made
Carry the canonical benchmark utility pair after owner PR3312 merges. Publish the combined consumer change with the existing prior-PR closure metadata, then follow required CI and merge policy.

### Open threads
Final library replay, owner landing, shared copy and this consumer PR. Original multi-target oracle semantics remain tracked in the owner problem record and require the missing original reference.

### Errors
First library compilation failed because sparse checkout omitted an existing include_str input. Materialized that exact tracked directory and retained the failed log. The selected Rust toolchain lacks rustfmt; used the existing Homebrew formatter for this file. No toolchain or dependency downgrade.

The initial attempt to append the shared daily log exposed five pre-existing prose-lint findings in earlier entries. Those entries remain byte-identical; this session uses its own dated log so its changed text can pass the existing lint without rewriting historical entries.

Final source replay: all 20 focused BSGS library tests pass, zero failures or ignored tests, 6,529 unrelated tests filtered. The build completes in the release profile; its duration is not a performance comparison. Existing compiler warnings are retained. Autonomy metadata verification passes 167 entries with zero errors.

## 2026-11-16T02:12Z - owner synchronization and base integration

Copied the exact benchmark utility pair from owner PR3312 merge76923ef45a6e734f7cda337dde387981940d2c5d and changed only their two lock digests. All58 utility tests pass; four report formats remain byte-identical. Merged current main and restored its committed PCC metadata so the owner keyring aligns; ledger1179 chain and parity21 pass. Initial record-gate failures were sparse omissions. Materialized committed manifests, report/include inputs, and54 raw/graph files totaling1,119,976bytes; all215 records now pass, retaining the existing hash-bound archived registry replay. No run status, payload or report was rewritten.

The current-base library build exposed two leftover Vec returns in the cached branch of lift_x_packed, whose API now returns a packed array/count. Changed the empty branch to (none,0) and the successful branch to two packed roots, preserving order. The original E0308 failure log remains. The new thread-local cache control covers0/1/2 results, and the existing ordered-root comparison passes across its declared fields. The complete release library builds; all20 BSGS tests and both packed-lift controls pass. Formatter checks pass.

## 2026-11-16T03:25Z - late PCC base race

### What was done
PR1609 merged as28d1c3b043481baad61aa7a84423586f56aaa1b7 while its PCC job still ran. Its ledger166 preserves every entry of first parent f885c90394f9addd6e71cd807da8cc28d03cb132. Main advanced to167 entries in5d4bcf089a164a1ac8eb248ab89aedd8eb804463 before the job fetched it, causing a false prefix failure. Pin the ledger, trust keyring and advisory diff to HEAD^1 of the default PR merge checkout, following the existing owner workflow. Push BEFORE verification is unchanged.

### Decisions made
Use the immutable base of the tested merge tree; retain prefix, keyring and sabotage checks. No ledger or frozen report changes. Sources checked (KB): source:6ca2a527db9b9503b93d4e57be4091e6, crypto:docs/SABOTAGE_RESISTANCE.md; index revision9141e386254340a9bdba3036f2bb706a last refreshed2026-11-14, older than this race. Current workflow, verifier and exact failed-run history govern the repair.

### Open threads
Replay the moving-base failure and existing negative controls; publish and follow CI.

### Errors
The merged PR's late PCC failure remains recorded; merging did not establish a green result.

## 2026-11-16T03:32Z - artifact custody and benchmark admission

### What was done
Restored the evidence-discovery functions from9023f2b5e7 removed byb513d2ca4e, retaining newer explicit declarations and generated-step stripping. Current43 artifact tests pass; original artifacts.py fails the new tracked-successor control by overwriting the current file. Hydration now preserves staged/tracked paths, local edits and omitted tracked files while hydrating untracked siblings. Regenerated workflows remove obsolete pulls for authoritative Git inputs; non-generated workflow lines are checked equal in docs/audits/2026-11-16-artifact-workflow-replay.json. No scientific workflow command changes.

### Decisions made
Keep all frozen artifacts and the46-entry tolerance snapshot unchanged. Archive the two exact benchmark rows that fail current pin/tolerance checks via the existing provenance-bearing archive. The pre-cap inventory now reads only tracked report files; a new negative control proves hydration cannot expand it and wide untracked reports still fail admission. Ninety validation controls pass locally. The full committed-tree test remains pending in CI because this sparse checkout lacks raw store inputs.

### Open threads
Follow current-head full validation CI, sync the owner artifact source to consumers after merge, and check the immutable PCC base in hosted CI. Re-admission requires prospective validation evidence; the archive gives none. PR4381 autonomy closure is partial, records the human nudge with a rule classifier and does not claim CI green.

### Errors
The first local validation run lacked audit_runs/curve_id and raw inputs; materialized the tracked helpers and retained the original log. Local store retrieval returned403 and the report cannot be revalidated from absent bytes. No credential or policy change. A first sparse staging command omitted two native docs; staged them explicitly in a second ordinary commit before PR1618 creation. [pcc:unverified]

## 2026-11-16T03:36Z - draft-tier CI recovery

### What was done
Initial PR4393 parse job43798786418 rejected57 workflows outside the existing draft gate. The local complete workflow inventory found58. Used the repository draft_tier_gate.py transformation and checked each parsed result against its expected structure before atomically writing. Each change adds only ready_for_review and the draft condition; commands and inputs remain equal. The pinned historical copies are untouched. The complete check now reports0 workflows outside the gate. Receipt: docs/audits/2026-11-16-draft-tier-repair.json.

### Decisions made
Repair the actual CI failure without adding fast-tier exemptions or changing execution on ready PRs or push events. Retain the earlier generated-step receipt as the separate prior transformation.

### Open threads
Current-head hosted checks and owner artifact-source consumer synchronization.

### Errors
The initial parse failure was an existing workflow policy drift, not a malformed YAML change. The failed run is retained; no rerun is used as a substitute for this patch. [pcc:unverified]

## 2026-11-16T03:40Z - artifact manifest path validation

### What was done
The ongoing semantic audit reproduced accepted parent traversal in manifest entries using an owned disposable local remote. The new path control fails before the change. The same source now rejects non-basename names, malformed SHA256/byte metadata, symbolic manifest files and manifest directories resolving outside the checkout before hydration. Existing8204 entries have0 invalid names and0 invalid metadata, so the stricter parser changes no frozen manifest.

### Decisions made
Validate the manifest at its read boundary, before parallel writes; keep the hash/length verification and tracked-file preservation. No remote artifact was altered. Receipt: docs/audits/2026-11-16-artifact-path-validation.json.

### Open threads
Final46 artifact controls, parity/generator replay, current-head CI and owner-source consumer sync.

### Errors
Old code accepted relative parent paths and a symlinked parent could redirect writes. Reproductions were confined to temporary directories. [pcc:unverified]

Final replay:46 artifact tests pass; parity21, generated workflow check and draft-tier check pass. The pinned original inputs remain byte-equal. [pcc:unverified]

## 2026-11-16T04:43Z - current-main integration

### What was done
GitHub started only pull_request_target jobs for the latest owner head because it conflicted with moving main. Integrated88112c5a93. Upstream PR4398 now carries the evidence-discovery restoration, so this branch has no remaining wire_workflows.py diff. Upstream PR4394 restored the canonical registry and reverted the cap assessment. Keep its validation scripts and benchmark index byte-identical: the two-row archival proposal and its pre-cap inventory change are superseded; both benchmark rows remain admitted under current main policy. Preserve the removed policy snapshot as historical bytes in docs/audits/2026-11-16-policy-history/REL_TOL_PRE_CAP.json.

### Decisions made
Resolve the prime-transfer trigger with upstream's push-only policy. Keep the independent artifact guards, immutable PCC base, and draft-tier repair. Parsed workflow documents match upstream or its standard draft transformation after removing generated blocks; receipt docs/audits/2026-11-16-integrated-workflow-replay.json.

### Open threads
Current-head hosted CI and exact owner-source consumer sync. Full committed-tree validation needs hosted store inputs.

### Errors
Sparse merge omitted manually materialized receipts/helpers outside its patterns. Confirmed their committed objects and restored them; an absent worktree file is not an upstream deletion. Earlier stage receipts remain historical and are superseded by docs/audits/2026-11-16-current-main-integration.json. [pcc:unverified]

## 2026-11-16T04:46Z - PR budget checker recovery

### What was done
The integrated CI parse job advances beyond draft gating and reports one existing rebench job with four routine PR budget violations. Limit its pull_request allowance to25 minutes under the existing checker; push and explicit dispatch retain300 minutes. Panel inputs, commands and promotion rules are unchanged. Record this one timeout exception to the otherwise generated/draft-only workflow transformation in the integration receipt.

### Decisions made
Use the existing routine PR ceiling without adding a budget exemption or changing historical measurements. A timeout remains a failed or censored CI run, not a completed panel.

### Open threads
Current-head hosted checks and owner artifact-tool consumer copy after landing.

### Errors
A later checker step was masked by the initial draft-tier failures. Repair the actual reported budget rather than widening the checker. [pcc:unverified]

## 2026-11-16T06:28Z - current registry, exact custody and retained plan

### What was done
The records job identified an empty open plan and a stale progress report. Preserve the exact open-plan bytes under benchmarks/records/plans-retained/ instead of fabricating a launcher or closing it as an execution. All219 execution records pass; the regenerated rolling progress outputs equal the committed bytes.

The three ecbench failures and sealed factor-base replay failure follow the registry rollback. Restore the exact22,300-entry registry from730e036d50^1, SHA256c0e1027ac8023409e7ce072428b4a978cd4d81cbcf7101dc7e728b7abb8f0134. All606 local ecbench tests pass, including the three previously failing cases (6 ignored); all5 sealed panels replay, and the independent observations/coverage/guard coverage are byte-identical. No F4 or new measurement was run.

A prospective aimed-gather custody binding preserves the complete original target and all211 other raw pins (210 actual local byte hashes,1 manifest-only isolation file). Its Rust checker reuses the existing logic with date/predecessor updates and exact serde_json numeric features. An independent Decimal check rejected the default-parser successor's1-ULP changes; exact serialization and all7 positive/adversarial fidelity controls pass. The original report, prior bindings and previous manifest bytes remain retained. The171195-byte new report is pending trusted artifact-autofix upload/replay; its locator is not an upload receipt. Validation CI runs the numeric controls after hydration.

### Decisions made
Follow the existing push/manual-only research policy for gb-baseline-rebench: remove its routine PR trigger and restore the full300-minute allowance for its existing non-PR modes. Do not shorten a full scientific panel just to fit a routine check. Preserve the canceled25-minute arm as incomplete; no comparison or measurement is claimed from it. The actual routine budget checker passes without an exemption.

### Open threads
Trusted artifact publication, current-head hosted CI, then canonical source copies to both consumers.

### Errors
The first local Cargo command selected Homebrew rustc1.93 despite the newer Cargo. Invoke the installed1.99.1 Cargo and rustc by exact path; no dependency downgrade or global toolchain change. The first successor used default JSON floating parsing, which altered scientific values; rejected it before updating the manifest, then enabled arbitrary precision/round-trip parsing and checked exact Decimal equality. An early negative-control invocation lacked its fixture and was discarded; the completed mutated-target control fails specifically at the complete-record assertion. [pcc:unverified]

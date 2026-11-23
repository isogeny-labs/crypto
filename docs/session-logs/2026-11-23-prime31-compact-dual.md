# Prime31 compact dual provenance — 2026-11-23

## 2026-11-23 UTC — source-level compact dual-certificate follow-up, draft PR #5033

### What was done (commands, files, features)
- Re-read current repo AGENTS.md, ACTIVE.md, the previous #5020 handoff and source, and current open/merged ownership. At 17:24:42 UTC, #5020 had **merged** (head c3a1f1b; merge add5ea9), so its branch was not edited. Opened a separate source-only draft #5033 on `automation/prime31-compact-dual-provenance-20261123` from main `4344dba5d9d65f88dd63681ad940af7421ee41a6`. First commit `c49cf3069e9a0fe588434b47d44cd1e45c73fdca` changes only ACTIVE.
- Revised original Rust `research/cheap_ecdlp_cold_gate_20261123/src/bin/rr5_receipt_check.rs`: opt-in `dual_from_rows` now stores each normalized base pivot's originating row, normalization inverse and a triangular list of previously eliminated lower-column pivots. At target visibility, descending-column substitution reconstructs coefficients of *original authenticated rows*; it still replays all coefficients and independently requires `[d]G=Q` through the unchanged caller. Other options, caps and default behavior remain unchanged. Commit `fe10b77718167c4cb8042df9029da9392d8adf9c`, blob `98b760116e03c91461bf33741cb7b76c7d86a5ac`.
- Added two authored original-Rust regression tests for out-of-order pivot columns and transitive triangular dependencies (now 19 tests in the binary). The test fixtures are public algebraic elliptic-curve identities and include full five-row recombination; **tests have not been compiled or run** here.
- Independent standalone mathematical model over F101: 9,000 deterministic seeded streams (6,696 target-visible) agreed with independently recomputed rank-gap earliest prefixes and scalars; no disagreements. A second **exact BigInt** model over F2147446831: 1,700 seeded streams, 1,199 target-visible and 501 underdetermined, zero disagreements; recombined every returned source-row witness to [0,...,0,1,d]. The fixed out-of-order and transitive examples recombined with expected weights using BigInt. These development checks are *not* source-exact Rust executions, paid benchmark measurements, independent signed worker validation, or PCC.
- Source literature: in-repo `research/cheap_ecdlp_cold_gate_20261123/README.md`, merged PRs #4969/#5020, upstream RR5 #4970 and B677 #4974, Semaev 2004 (https://eprint.iacr.org/2004/031), PKM 2016 (https://doi.org/10.1007/978-3-662-49387-8_1), prior row-span certificate control. Research-KB MCP and Conductor/native child launcher unavailable; repository review was the fallback. No novelty claim.
- A full read of canonical `docs/session-logs/2026-11-23.md` blob `cbc8a78dda6c667fa34eb7628e937e68f829b09f` showed **1,314,589 characters** and existing end-of-file merge-conflict markers. `fetch_file` returned empty content for this oversized blob; this new dated supplemental log preserves the canonical byte-for-byte pending its owner's reconciliation rather than overwriting or dropping other sessions' entries.

### Decisions made (and why)
- The new algorithm improves **optional verifier provenance bookkeeping only**, not relation extraction, Gröbner solving, the complete ECDLP pipeline, or proven actual runtime. Old dense source-provenance elimination was O(B*M^2) modular operations and O(B*M) pivot provenance words for B base variables and M accepted rows. Triangular dependencies bring the additional dual pass to O(M*B^2+B^2) operations and O(B^2+M) auxiliary words *besides stored accepted rows*. Actual end-to-end verifier memory still includes O(M*B) for authenticated source rows; benchmark the cross-over and charge all work.
- No changes to graph/sparse-LA, summation-polynomial, RR5 extraction, transfer, optimized negation-rho, research baselines, frozen evidence, PCC registry or benchmark reservations. No new automation, EKS job, merge, paid compute, or scientific timing run.
- Actual workers: one coordinator (repository inspection, Rust integration, model-based verification, shared writes); **zero native spawned workers** available. Repository reads were concurrent, writes sequential. The two mathematical models are not independently signed verifiers, and no elapsed-time speedup is claimed.

### Open threads (unfinished, pick up next time)
- Obtain admitted source-frozen original Rust toolchain/lock and run `cargo test --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml --bin rr5_receipt_check`; retain full stdout, exact source/binary/toolchain identities, timing and RSS on the *dual verifier itself*. Compare old dense vs compact routines on the same accepted-row stream, including all source EC replay costs; run the two new cases and mutation/refusal tests.
- After RR5 owning lane delivers *blind, unknown-index* original Prime31 B677 accepted rows, replay target-separation weights and check rank-gap and `[d]G=Q` independently before any fully charged cold T=1 IC vs optimized negation-map rho vs fresh BSGS comparison. The B8 original-positive fixture still provides no target log. Keep #5033 draft and [pcc:unverified] until compiled and independent qualification.

### Errors (what went wrong, how it was handled)
- A concurrent check showed #5020 merged after the previous handoff. An attempted source edit was safely aborted before writing, and work moved to distinct #5033 with a tracking first commit.
- This runtime has no cargo/rustc or native subagent launcher. BigInt was required for exact Prime31 modular arithmetic: ordinary JS Number multiplication can exceed 2^53, so a non-exact scratch check was rejected and rerun in exact BigInt; no invalid arithmetic result was used as evidence.
- Canonical shared daily log is oversized and contains pre-existing merge-conflict markers. A separate dated session supplement and PR handoff were used; the corrupted shared blob was not modified.

## 2026-11-23 UTC — Post-integration live owner/CI check

### What was done (commands, files, features)
- Read back draft #5033, all four changed paths and exact original-Rust blob `98b760116e03c91461bf33741cb7b76c7d86a5ac`; confirmed 19 authored `#[test]` functions and a four-file PR diff: ACTIVE, this supplemental log, the research report and the isolated verifier.
- Rechecked live #4970, #4974, #4993 and #4988: all are now **merged**; #4974's body still says the actual blind B677 extraction has not run, and #4993 explicitly did not seal negation_v5 as the best rho baseline. Corrected the task report and PR body to distinguish completed source controls from outstanding extraction and baseline qualification.
- Inspected current #5033 head CI once: `migrate` in progress at https://github.com/isogeny-labs/crypto/actions/runs/45228950390/job/137882958752 and `review` skipped at https://github.com/isogeny-labs/crypto/actions/runs/45228950690/job/137882958178. Neither constitutes a passing native original-Rust test.

### Decisions made (and why)
- Keep #5033 draft and isolated, no Actions rerun, competing build, benchmark, publication or merge. The experimental reduction is a verifier-complexity control, not validated cold solver cost.

### Open threads (unfinished, pick up next time)
- Source-frozen cargo compilation, replay of both new adversarial cases, independent exact row recombination and same-stream dense/compact timing/RSS when a runner and reservation are available; genuine unknown-index B677 rows remain an external dependency.

### Errors (what went wrong, how it was handled)
- Repository ACTIVE and PR prose were behind merge events in other lanes; reconciled our own report/PR without modifying those merged branches or reclassifying their scientific outcomes. Canonical daily log's oversized/conflicted blob remains untouched.

# 2026-11-19 Prime31 W8 signed-weight collision audit (PR #4805)

## 2026-11-19 11:20 UTC — what was done (commands, files, features)

- Read repository `AGENTS.md`, `docs/ACTIVE.md` and recent Prime31 PRs [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4728](https://github.com/isogeny-labs/crypto/pull/4728), [#4803](https://github.com/isogeny-labs/crypto/pull/4803), [#4793](https://github.com/isogeny-labs/crypto/pull/4793). Shared research KB MCP/wrapper not available through this connector runtime, so review is bounded by these primary GitHub records, autoresearcher search, and PKM 2016 / APS 2018 published summaries.
- Created branch `automation/prime31-w8-weight-signature-audit-20261119`, recorded task in `docs/ACTIVE.md` before research write, created draft [#4805](https://github.com/isogeny-labs/crypto/pull/4805).
- Preregistered `research/prime31_ic_20261115/rr9/W8_WEIGHT_SIGNATURE_AUDIT_PROTOCOL_20261119.md`.
- Independently enumerated the four-slot signed **integer** coefficient vectors in Python stdlib (not an ECDLP solver): original [1,3,5,7] and [11,13,17,19] each had 64,737 distinct vectors, 904 unordered word-collision pairs, max fiber 16; comparison [1,2,4,8] and [16,32,64,128] each had 65,536 distinct vectors, no pairs, max fiber 1. The historical 64,737 actual half-sum support in #4728 exactly matches the formal count, suggesting a structural explanation for that source fixture.
- Implemented original-crate Rust example `prime31_w8_weight_signature_audit.rs` using `prime31_algebraic_ic::{check_curve,Arithmetic,Point,PRIME31}`. It performs exact formal-vector census, actual EC half-support measurements and a matched panel of four windows × 64 natural public targets per arm; the generic MITM is *only* an out-of-band grader. No natural target scalars or witness words are passed to the membership test.
- Added the pinned original-source workflow `prime31-w8-weight-signatures.yml` (Rust 1.100.0). Workflow-run completion and measured data must be checked before any empirical claim.

## Decisions made (and why)

- This is a distinct weight-geometry and yield-control experiment, **not** a duplicate of the genuine unknown-index F4/remainder-tower extractor owned by #4692.
- No speculative whole-solver or non-generic performance claims. Deliberately compare identical public residual points to isolate weight choice. Report accidental EC collisions rather than assuming their absence, because the support upper bound is mathematical, not guaranteed equality.
- Preserve original curve, 31 base points and four windows to avoid changing curve security or rank requirements. Alternative powers-of-two are a *candidate control*, not assumed better.
- Literature receipt: Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009). Both give prior index-calculus context, not a novelty claim.

## Open threads (unfinished, pick up next time)

- Inspect GitHub Actions [candidate source workflow](https://github.com/isogeny-labs/crypto/actions/runs/44515210069) to confirm compilation, formal counts, actual EC half-support and 256-target paired hits.
- If Rust/CI fails, correct original-crate E0499/type mistakes and rerun. Report actual paired old/new success and memory/operations; do not conflate group-sum collision removal with algorithmic extraction or whole-solver improvement.
- Publish source-linked frozen JSON summary and mathematical result, mark task completed only after passing source-specific execution. Coordinate with owner #4692 if meaningful.
- New mechanism remains efficient public-only B8 weighted RR9 unknown-index extraction, then rank31/1024 complete solver and same-runner strongest-BSGS/IC/rho evaluation.

## Errors (what went wrong, how it was handled)

- Container has no GitHub DNS or Rust toolchain; used GitHub connector for source reads/writes and source-specific GitHub Actions, with pure-integer local independent precomputation only.
- Avoided nested mutable borrow in first-x lift; loosened brittle assertions equating formal and actual EC cardinalities so unforeseen group collisions appear as measurements rather than a false test failure.
- Shared research-KB tooling unavailable in this runtime; bounded fallback explicitly documented.


## 2026-11-19 11:32 UTC — source-bound CI result and measured decision

### What was done (commands, files, features)
- GitHub Actions [run #44515210069](https://github.com/isogeny-labs/crypto/actions/runs/44515210069) **PASSED** on pinned original source and Rust 1.100.0: **40/40** library regressions, **2/2** focused tests, full four-window × 64 public-target grade, raw JSON uploaded as [artifact #14525571822](https://github.com/isogeny-labs/crypto/actions/runs/44515210069/artifacts/14525571822). Tested code/workflow commit `2a98e0cf52faa7fca1f5a6f7d69f46be70a1ea0d`. Raw JSON SHA-256: `8430535e90e731a5edfab09959b4f052f41146e473d4abf8529d1fb036f284b3`.
- Exact formal count 64,737 old vs 65,536 alternative per half; 904 formal old collision pairs vs 0 alternative, independently reproduced. Old EC distinct 64,737/half all four windows; proposed alternative 65,536/half for W0/W2/W3, **65,496/half W1** (40 unforeseen actual EC coincidences). All weighted unsigned slot x sets each have 64 distinct images.
- **256 matched natural public residuals**: old **205 hits** vs candidate **226 hits** (181 both / 24 old only / 45 candidate only / 6 neither); proposed arm +8.203125 percentage points on *this frozen cohort*. Descriptive paired McNemar exact p=0.0154323, not a preregistered multi-seed inference. Exact grader cost and per-window detailed records published in `W8_WEIGHT_SIGNATURE_AUDIT_RESULT_20261119.md` and `W8_WEIGHT_SIGNATURE_AUDIT_SUMMARY_20261119.json`.

### Decisions made (and why)
- Report component-level natural-public **generic 4+4 grading** yield improvement only; this is not public-only F4, new complete ECDLP logs, nor a cost-competitive solver. Historic optimized shared BSGS is the actual complete 1,024 target competitive frontier and must be rerun same-runner before claims.
- Keep the draft task's source-bound passing code intact, publish the full report and frozen summary, close the task in `docs/ACTIVE.md`, mark PR ready after complete documentation.
- Prioritize a **held-out seed** and W1 accidental EC collision analysis before any weight-promotion; this pilot alone does not establish statistical or asymptotic improvement.
- Source-specific PASS is not a signed third-party PCC ([pcc:unverified]).

### Open threads (unfinished, pick up next time)
- Post weight-control results to nonenumerative extractor owner [#4692](https://github.com/isogeny-labs/crypto/pull/4692); test algebraic extraction of both weight choices without generic-sum tables on the original B8 46-positive/18-negative corpus, then replace grader in original rank31/1024 full Rust solver and measure strong same-runner BSGS/IC/rho.
- Inspect W1 40 extra group coincidences between formally distinct vectors; independently rerun on fresh 1,024-target/window held-out cohort before claiming robust yield.

### Errors (what went wrong, how it was handled)
- Whole-repo Workflow files parse [run #44515209875](https://github.com/isogeny-labs/crypto/actions/runs/44515209875) failed before parser due inherited artifact-store pull failure; separate source-specific Rust study passed. No infrastructure modifications or paid compute commissioned.

## 2026-11-19 11:40 UTC — GitHub review handoff

### What was done (commands, files, features)
- Updated PR #4805 with all frozen evidence and the exact 256-point table; marked it **ready for review** after source-specific CI passed. Posted handoff to RR9/F4 owner [#4692](https://github.com/isogeny-labs/crypto/pull/4692) (GitHub comment ID 7007994044), distinguishing the grading-only power-of-two yield from true public-only coefficient extraction.

### Decisions made (and why)
- Preserved the complete measured result and protocol without treating source-specific Rust CI as an independently signed PCC. PR stays open and reviewable.

### Open threads (unfinished, pick up next time)
- Inspect an independent signed replay, held-out seeds, and 40 W1 accidental collisions; do not promote an actual solver-speed claim until #4692 provides an efficient extractor and a fully charged same-runner whole-solver comparison.

### Errors (what went wrong, how it was handled)
- GitHub auto-merge failed because the configured GraphQL squash merge method is disallowed by this repository: `Merge method squash merging is not allowed on this repository`. No merge was performed; PR #4805 remains open, marked ready for review.

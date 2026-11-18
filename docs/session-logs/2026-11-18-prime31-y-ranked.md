# Prime31 y-coordinate-ranked B31 full-cost Rust control — 2026-11-18 UTC

## 03:24 UTC — Repository and literature audit, tracked task, prospective protocol

### What was done
- Read current repository `AGENTS.md` and `docs/ACTIVE.md`, latest available dedicated session log; reviewed PRs #4575/#4581/#4596/#4604/#4623/#4626/#4629/#4635/#4647 and `crypto-autoresearcher` #3305, `cryptanalysis` #1536. Read existing original Rust arithmetic, `prime31_s9_b31_half_sum_oracle.rs`, balanced S4/S3 bridge and published PKM/APS prior art. Shared KB was unavailable in this connector execution; GitHub, repository files and named papers are a bounded fallback.
- Created stacked branch `automation/prime31-y-ranked-base-complete-20261118` based on open #4647, added active item alone, opened draft [PR #4652](https://github.com/isogeny-labs/crypto/pull/4652), attached PR number to active item.
- Froze `research/prime31_ic_20261115/native/B31_Y_RANKED_COMPLETE_PROTOCOL_20261118.md` **before source modification**: same 256 public targets, same rank and full pipeline, AB/BA, all setup and negatives charged; no non-generic speedup claims.

### Decisions made
- Test a deterministic y-coordinate-ranked base (first 31 by canonical y from first 256 rational-x lifts), not an arbitrary claimed asymptotic change. Keep original pinned curve and Rust 4A+4A/negation quotient.
- Avoid existing concurrent owners #4604 algebraic RR9 and #4596 matched BSGS; do not duplicate their work. No EKS/paid compute requested or launched.

### Open threads
- Obtain and verify exact-source Rust AB/BA full solver results, add independent arithmetic checker and committed evidence.
- Distinguish control engineering gains/negative from genuine non-generic relation extraction.

### Errors
- No local Rust/Cargo runtime in this connector-backed session; source-specific GitHub-hosted Rust 1.100 Actions required. KB MCP/CLI not loaded.

## 03:26 UTC — Original Rust implementation, first matched run and independent exact arithmetic

### What was done
- Added `low_y_among_first_256_x` to existing 95-KB original Rust example; added `PRIME31_B31_Y_RANKED_COMPARE=1` AB/BA, reused 31-column rank, original source known-target controls, batched signed complement oracle and unmodified public targets. Added sparse exact-source workflow.
- [CI #44230419721](https://github.com/isogeny-labs/crypto/actions/runs/44230419721) PASS: pinned native library tests plus two 256-Q full-solves, 31/31 rank and 256/256 blinded logarithms per arm. AB first-x 2194.191246 ms, y-ranked 2824.345782 ms; BA y-ranked 2771.233421 ms, first-x 2160.964930 ms. First-shift hits x=244 versus y=232 (target exhaustive negatives x=12 vs y=24). No generic fallback.
- Independently implemented exact BigInt EC arithmetic in separate JS execution; replayed all 1,024 candidate-target scalar equalities and signed eight-point witnesses, 124 basis logs and 124 modular relations across both AB/BA panels, no failure. Independently selected first 256 rational-x lifts and sorted by canonical y, matching 31 Rust points exactly; only 3 of 31 points overlap x-first31.
- Added standalone Python independent arithmetic replay `research/prime31_ic_20261115/native/verify_b31_y_ranked.py`, and CI step that checks every output log and writes replay evidence. Re-running source-specific CI on exact updated commit.

### Decisions made
- The new low-y base is a **measured negative** under the fixed 256-Q complete generic-equivalent IC pipeline, not a better solver. Hold final numbers to final exact-source CI evidence.
- Retain first-x baseline for future algebraic extractor comparison; prioritize sign-independent S4/S3 or RR9 extraction without 4A table.

### Open threads
- Confirm final source-specific Rust+Python Actions re-run and archive its independent transcript.
- Commit final scientific report and raw summary with exact reference/metrics; check this ACTIVE item off when finished.

### Errors
- No failed Rust mathematical tests in first CI. An earlier GitHub API fetch with a disallowed filtered workflow URL was replaced with allowed commit-workflow-run and job retrieval; no data loss. Independent signed PCC verification remains separate/unavailable.

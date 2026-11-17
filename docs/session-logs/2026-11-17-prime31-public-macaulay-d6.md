# Prime31 public RR9 Macaulay degree-six follow-up — 2026-11-17 UTC

## 2026-11-17 13:20–13:30 UTC — Register a distinct experiment, implement prospective cap

### What was done
- Read AGENTS.md, docs/ACTIVE.md and the prior experiment log/report/protocol/source at PR #4548, plus the full stacked #4539→#4548 RR9 chain and previous #4513/#4532 generic frontier, #4373 F4, #4228/#4349 quotient, crypto-autoresearcher #3301/#3305.
- KB tools absent; consulted the public APS (2018) prime-field index-calculus paper and Kudo et al. (2018) hybrid/limitation publication records and preserved a bounded source receipt; neither supports claiming novelty here.
- Created branch research/prime31-rr9-macaulay-degree6-20261117 at #4548 source SHA f0487ccb; registered it in docs/ACTIVE.md alone, opened **draft PR #4549**, linked PR to active item, froze RR9_PUBLIC_MACAULAY_D6_PROTOCOL_20261117.md BEFORE writing candidate code.
- Reused the immutable prime31 native library and #4548 coefficient quadrics, merely adding public degree_six_controls() and a small separate Rust example. Three exact public coordinates are fixed, with sign enumeration restricted to independent test-only classification. No unknown target scalar or signed supports enter public_macaulay.
- Added source-specific pinned Rust 1.100.0 GitHub Actions workflow with release library tests, new example test, raw degree-six JSON validation and capped candidate run (300 seconds / 1GiB address space).

### Decisions made and why
- Degree six is a distinct preregistered follow-up, not an unrecorded revision to failed degree five. Degree seven is not authorized in this experiment. A unit witness must replay exactly and a positive coefficient vector must pass independent original-curve signed summation.
- Retain producer [pcc:unverified], no complete-ECDLP or speed claims. Historical generic BSGS #4532 stays a different-run comparator.

### Open threads
- Inspect first exact-source GitHub Actions result and raw artifact. Fix compile-only issues without altering degree, public fixtures or gates.
- Record per-case rank, unit and coefficient results, time/field ops/memory, then assess if degree six actually improves extraction.
- If no public positive recovery, pivot to structural syzygies and do not expand dense Macaulay without a separately frozen protocol.

### Errors
- None observed before the initial pinned CI run; local container lacks a Rust compiler, so source-specific GitHub Actions is mandatory.

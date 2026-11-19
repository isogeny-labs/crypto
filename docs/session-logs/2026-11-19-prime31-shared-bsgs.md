# Session log — 2026-11-19 Prime31 original-Rust signed shared BSGS comparison

## 2026-11-19 UTC — preregistered source experiment in PR #4774

### What was done (commands, files, features)
- Read AGENTS.md, docs/ACTIVE.md, #4770 and #4728 Rust source and frozen results, the new #4773 and #4772 algebraic components, #4692 extractor draft, #4548 negative public Macaulay, latest dated Prime31 logs. Confirmed research KB MCP was unavailable and used repo/literature fallback; checked PKM (2016) and APS (2018) bibliographic primary records.
- Created branch automation/prime31-shared-bsgs-paired-20261119 stacked on #4770 at f74968435f1a278e3c91469b990b978b6e299deb; first commit only tracked the task in docs/ACTIVE.md; created draft PR #4774.
- Registered W8_MATCHED_SIGNED_BSGS_PROTOCOL_20261119.md before source/benchmark. Implemented signed cached BSGS in native examples/prime31_shared_signed_bsgs.rs with original Arithmetic, 32768 x-keyed babies, signed ± residue, 1024 Q, counted setup and online, complete verification.
- Added independently authored Python BigInt verifier with exact same-source target-stream check against both #4770 W8 arms; instrumented original Rust tests, equality of repeated BSGS answer/count transcripts, six-arm old-ternary-bsgs-bsgs-ternary-old same-runner execution, raw artifact.
- Used sparse pinned checkouts in source-specific GitHub Actions to avoid 10GB repository overhead and include the original Rust library F4 dependencies.
- Source-specific workflow launches at commit 12f7906e9c0703dd838a1b1b614b31d4b99e46fc; do not record a passing benchmark until the final Rust and independently executed Python results are fetched.

### Decisions made (and why)
- Reuse #4770's original EC arithmetic and identical target stream, rather than substitute an unrelated fast ECC library. The signed BSGS is **generic control** and intentionally not promoted as non-generic IC; a qualified benchmark against fastest BSGS/rho still requires separate implementations.
- Distinguish exact signed BSGS coverage theorem from empirical speed measurement.
- Do not run paid EKS work or merge; source-specific GitHub CI and producer-side independent replay can inform but not replace signed PCC.

### Open threads (unfinished, pick up next time)
- Inspect exact CI, failures and raw artifact, and commit frozen paired measurements only after full replay.
- Continue public-residual-only RR9/F4 unknown-index extraction on natural B8 controls; integrate into the existing verified four-window rank31/1024 solver if successful.

### Errors (what went wrong, how it was handled)
- Before CI fixed a likely Rust nested immutable/mutable borrow in construction of the stride; use separate scalar then negation.
- Earlier #4728 transcript has a different deterministic target seed from #4770; verify against *the actual #4770 arm* in the same six-arm execution, not against #4728's distinct fixture stream.

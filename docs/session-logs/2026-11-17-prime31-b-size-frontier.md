# 2026-11-17 — actual-prime31 B27/B29/B31 fully charged sensitivity

## What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, parent #4626 session log and primary native Rust 4A oracle and complete rank→1024-Q test; checked previous #4623/#4626, contemporaneous #4596/#4604 and cross-repo reviewed leads. Checked primary PKM 2016 and APS 2018 bibliographic sources. `research-kb` not offered by connected tools and repository CLI unavailable in this runtime.
- Created branch `automation/prime31-b27-b29-complete-sensitivity-20261117` stacked on open #4626, committed one-line task record in ACTIVE.md first, opened draft PR #4629, and attached its number in a separate commit.
- Committed prospective B27/B29/B31 experiment protocol before editing Rust. No new performance or solve claim at this stage. [pcc:unverified]

## Decisions made (and why)
- Reuse the actual Rust original-curve arithmetic, 4A index, batched complement, modular rank learning and frozen public targets. Vary **prefix base size**, not Dickson construction depth, so the only intended independent variable is B.
- Demand 256/256 complete blinded scalar recovery and rank B/B with all failed shifts and charged fallback, not a coverage-only optimism signal. Treat all 4A code as generic-equivalent, not novel index calculus.

## Open threads (unfinished, pick up next time)
- Insert a focused test in the original Rust source; add source-specific pinned CI; run, capture and independently read back raw evidence, evaluate full cost; update PR and handoff. Then attack sign-independent algebraic residual decomposition on the real B31 base.

## Errors (what went wrong, how it was handled)
- Native Rust/cargo and KB CLI are not installed in the connected container. GitHub Actions source-specific Rust runner will be necessary; no unexecuted test is presented as passed.

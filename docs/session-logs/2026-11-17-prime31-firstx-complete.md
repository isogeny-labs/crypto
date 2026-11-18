# 2026-11-17 — actual-prime31 matched Dickson31 versus first-x B31 complete Rust cost

## What was done (commands, files, features)
- Read repo AGENTS.md, prior #4629 full-cost and #4626 matched-coverage reports, original Rust example, last parent session log, and open #4596, #4604, #4594 owner lanes. Reviewed PKM (2016), APS (2018), and Kudo et al (2018) publication summaries to bound novelty.
- Created branch `automation/prime31-b31-firstx-complete-frontier-20261117` on #4629 and draft tracking PR #4635; recorded ACTIVE.md line first. The branch is stacked on the unmerged parent; no merges or paid compute.
- Committed prospective `B31_DICKSON_FIRSTX_FULL_COST_PROTOCOL_20261117.md` **before** editing candidate Rust, then appended a native release test in the existing `prime31_s9_b31_half_sum_oracle.rs` source and a sparse-checkout source-specific Rust 1.100 GitHub Actions workflow.
- Pinned curve p=2147483647, n=2147446831 and original native arithmetic commit 1df714645aa683cf975bc76fcacb9842cb4780fd. Candidate source commit fbbe0fe22c5d07c8ea1ac8b3e8a1f0d644a45d2c, workflow commit 7ecdf941daad3368ea1898070788a5e42c20e5ea.
- Initial exact-source CI: https://github.com/isogeny-labs/crypto/actions/runs/44215701063 (pending when this entry was written). No success or timing claim before the Rust runner completes.

## Decisions made (and why)
- The 244/256 first-x versus 237/256 Dickson prior result is a *coverage-only* control. Measure full original Rust index-calculus-style rank-31 learning, 16 control relations, 256 unknown logs and all misses/shift/fallback charges on the same public Q to see if the coverage signal carries into complete elapsed cost.
- The 4A+4A pair-sum oracle is **generic-equivalent**; do not call a full-cost improvement non-generic, novel or faster than matched shared BSGS. Treat first-x point selection as a conventional base change, not a trapdoor or new attack.
- Do not duplicate the dedicated same-runner 1024-Q BSGS owner #4596 or D9 Wiedemann owner #4604.

## Open threads (unfinished, pick up next time)
- Read CI, fix build/source-specific issues if needed, capture raw JSON, compare 256-Q complete costs, replay scalar/witness correctness and document limitations. Mark PR ready only if source-specific correctness gate passes.
- Primary unresolved requirement: sign-independent, public-R-only B31 RR9/S9/K961 extraction, with original Rust F4/quotient/root machinery and no FourSet generic enumeration. Then rank and solve unknown logs, compare complete 1024-Q against optimized BSGS, rho, published prime-field IC, with PCC validation.

## Errors (what went wrong, how it was handled)
- Rust/cargo and shared research-KB are unavailable in the connector runtime; no local Rust benchmark executed. GitHub Actions is the measurement source, not a replacement for independent PCC.

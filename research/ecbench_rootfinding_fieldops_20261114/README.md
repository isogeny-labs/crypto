# Prime rootfinding IC: expose the counted field-operation portion

Status: source instrumentation; no new ecbench measurement or speed claim.

## Scope and prior work

This change resumes the open [complete-cost calibration problem](../../problems/rootfinding-complete-cost-calibration.md) for the prime `rootfind-subtract` and batched pair-table collectors. The frozen [lane study](../ecbench_rootfinding_lanes_20261013/REPORT.md) and [buffer study](../ecbench_rootfinding_alloc_20261014/REPORT.md) remain unchanged. The rootfinding oracle belongs to the generic decomposition track under [the prime IC decision](../prime_ic_program_20261029/DECISIONS.md); this change does not alter its relation search.

Sources checked before implementation (KB revision `9141e386254340a9bdba3036f2bb706a`, 2026-11-14): `source:17a1b67993e4b781bbf1198fd2598c2f` (`crypto:papers/rootfinding-decomposition/rootfinding_decomposition.tex`) describes the existing S3 construction; `source:b9dda86f2baa2f4b18955b8e1ee126f1` (`crypto:papers/ecdlp-index-calculus-program/LITERATURE.md`) points to prior group-sum membership work. Primary sources read: [Semaev 2004](https://eprint.iacr.org/2004/031.pdf), for the summation-polynomial condition, and [McGuire--Mueller 2017, Algorithm 4.1](https://eprint.iacr.org/2017/1262.pdf), for factor-base sum-and-membership search. The current task concerns counting and replay, not a mathematical novelty claim.

## Record contract

For an IC run whose `GroupOps` tallies field arithmetic, `detail.counted_field_ops` now carries schema `ecbench.counted-field-ops/v1`, five named phases, and the exact sum of each phase's multiplication, squaring and inversion tally. It is emitted only when the sum is nonzero. Its `complete` value is `false`: `GroupOps` does not count every prime lift, test, hash operation or matrix operation. `SolveReport::field_ops` therefore remains absent for the IC pipeline. The auditor compares this block exactly on replay when a recorded run carries it; historical records without it keep their original replay rule.

The cost chain is an accounting identity: the existing counted GAE phase costs and relation stream are unchanged by this record addition. There is no replacement component and no candidate cost reduction in this change. The next technical step is to price the remaining native counters in a common unit, then run the frozen isolated lane panel with A/A controls and independent-host replay. Any performance comparison needs a separate frozen measurement and its claim record.

## Validation

The focused rootfinding test checks the block's phase names, sum, nonzero relation inversion tally, and incomplete-cost boundary. The focused replay test checks exact matching and compatibility with old records. Both passed with the release test binary before the `origin/main` merge. `rustfmt --check` on the two edited Rust files and `git diff --check` passed.

The required `cargo test --release --lib` was attempted with Rust 1.99.1 on macOS. After about 25 minutes it had recorded 6,287 passes, 86 ignored tests, and 19 failures; the remaining long tests were interrupted, so the full suite has no passing verdict. Five ecbench band/compare failures open an absent, untracked `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`. A separately rerun `ml_dataset::report` failure requires Linux `/proc`; a separately rerun wide-prime fixture test failed on a missing file. These failures are outside the edited code. Both focused release tests passed again after merging `origin/main` at `2f847a3711` (6,498 other tests filtered out in each run).

After the 2026-11-15 merge of `origin/main` at `b05c8a03af`, the release rootfinding report test and replay-binding test each passed again (6,543 other tests filtered out per run). The full library suite was not repeated on that final tree; its earlier failed/interrupted run is the only full-suite attempt. No new benchmark or performance claim follows from these tests.

## Handoff

### What was done

Added the partial phase tally and replay binding in `methods.rs` and `audit.rs`; merged current `origin/main` into the branch. The commands and outcomes above are the local validation receipt.

### Decisions made

Keep `SolveReport::field_ops` absent because the group-law tally does not cover the whole IC pipeline. Treat the full-suite attempt as incomplete and failing, while retaining the focused test outcomes at their exact source revision.

### Open threads

Price the remaining native counters in a common unit and run the isolated lane panel with controls and independent replay. The complete-cost calibration problem remains open.

### Errors

The macOS full suite encountered absent raw session inputs and Linux-specific `ml_dataset` requirements. Its long tail was interrupted after the failures were observed; no full-suite pass is claimed.

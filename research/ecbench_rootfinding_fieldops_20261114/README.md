# Prime rootfinding IC: expose the counted field-operation portion

Status: source instrumentation; no new ecbench measurement or speed claim.

## Scope and prior work

This change resumes the open [complete-cost calibration problem](../../problems/rootfinding-complete-cost-calibration.md) for the prime `rootfind-subtract` and batched pair-table collectors. The frozen [lane study](../ecbench_rootfinding_lanes_20261013/REPORT.md) and [buffer study](../ecbench_rootfinding_alloc_20261014/REPORT.md) remain unchanged. The rootfinding oracle belongs to the generic decomposition track under [the prime IC decision](../prime_ic_program_20261029/DECISIONS.md); this change does not alter its relation search.

Sources checked before implementation (KB revision `9141e386254340a9bdba3036f2bb706a`, 2026-11-14): `source:17a1b67993e4b781bbf1198fd2598c2f` (`crypto:papers/rootfinding-decomposition/rootfinding_decomposition.tex`) describes the existing S3 construction; `source:b9dda86f2baa2f4b18955b8e1ee126f1` (`crypto:papers/ecdlp-index-calculus-program/LITERATURE.md`) points to prior group-sum membership work. Primary sources read: [Semaev 2004](https://eprint.iacr.org/2004/031.pdf), for the summation-polynomial condition, and [McGuire--Mueller 2017, Algorithm 4.1](https://eprint.iacr.org/2017/1262.pdf), for factor-base sum-and-membership search. The current task concerns counting and replay, not a mathematical novelty claim.

## Record contract

For an IC run whose `GroupOps` tallies field arithmetic, `detail.counted_field_ops` now carries schema `ecbench.counted-field-ops/v1`, five named phases, and the exact sum of each phase's multiplication, squaring and inversion tally. It is emitted only when the sum is nonzero. Its `complete` value is `false`: `GroupOps` does not count every prime lift, test, hash operation or matrix operation. `SolveReport::field_ops` therefore remains absent for the IC pipeline. The auditor compares this block exactly on replay when a recorded run carries it; historical records without it keep their original replay rule.

The cost chain is an accounting identity: the existing counted GAE phase costs and relation stream are unchanged by this record addition. There is no replacement component and no candidate cost reduction in this change. The next technical step is to price the remaining native counters in a common unit, then run the frozen isolated lane panel with A/A controls and independent-host replay. Any performance comparison needs a separate frozen measurement and its claim record.

## Validation

The focused rootfinding test checks the block's phase names, sum, nonzero relation inversion tally, and incomplete-cost boundary. Rust formatting and the required release library suite are run from the source branch; the exact results belong in the branch's session entry and PR description.

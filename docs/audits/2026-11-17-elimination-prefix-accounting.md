# Retaining interrupted elimination work

The maintained Boolean F4 kernels accumulated logical XORs, performed XORs,
lookup-table preparation and auxiliary operations in local variables. Deadline
returns could leave those locals out of the returned statistics. The framework
then exposed the remaining tally as complete performed work. [pcc:unverified]

The deterministic FL control stops at the second reducer-loop deadline check.
Its 63 completed substitutions perform 32 three-word XORs and 31 one-word XORs:
127 word XORs. Before the repair, the unchanged algorithm reported zero. The
failed baseline is retained. [pcc:unverified]

The repair retains the original work and cutoff points and flushes local costs
on both successful and interrupted returns. Separate controls cover FL prefix
work, completed full-M4RI table preparation and lookup tables prepared before
expiry. All 19 module controls passed in the isolated source, including existing
rank, pivot, row-space, logical-cost and performed-cost comparisons. The integrated
release library also passes: 7431 passed, zero failed and 141 ignored. [pcc:unverified]

Historical records and source inputs remain unchanged. No performance gain or
new scientific standing is asserted. The counters describe the performed prefix
when the solver reports a budget outcome; that outcome remains incomplete.

Sources checked (KB): `source:08f3e36526264f46e4eca5d5381c7e31`,
`crypto:docs/ic/FRAMEWORK.md`, section "Every stage: charge what you spend",
revision `9b2119e8fbac482b98b50521c0d604bb`. The contract requires each stage to
retain spent native work. Current source review covered `pq_f4_f2.rs` and the
`f4-f2` adapter in `ic_framework/solvers.rs`.

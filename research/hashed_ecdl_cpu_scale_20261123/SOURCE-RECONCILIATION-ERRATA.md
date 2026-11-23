# Historical CPU source reconciliation correction

The preparation README at line 12 states: "Later round24/25 proposals have no
timing rows." That combined statement is wrong for round24. The 23 November
11:52 journal entry repeats the same mistaken reconciliation. Both frozen
records remain unchanged; this sibling note corrects their interpretation.
[pcc:unverified]

At repository snapshot `310d7fb0d908e9ca32c7d9282e31810b700d52cd`, round24 has
[EXECUTION.md](../hashed_ecdl_cpu_20261030/round24/EXECUTION.md), actual
[rows](../hashed_ecdl_cpu_20261030/round24/results/rows.csv) and
[summary](../hashed_ecdl_cpu_20261030/round24/results/summary.csv). The execution
record reports 180 calls, 162 measured/18 warmups, with matching certificates.
Its within-panel L0 observations led to reverting early residual rejection.
[The revert note](../hashed_ecdl_cpu_20261030/residual-filter-revert/README.md)
explicitly restores the suffix-only parent. These are historical records,
not newly executed measurements or an authenticated revalidation. [pcc:unverified]

The round24 proposal README preserves its earlier pending status. The source
audit should have followed its later execution/result artifacts. Round25 has
a published frozen proposal/readiness package and no result tree at this
snapshot; this is a scoped repository observation, not proof about every
external execution or unrecorded result. [pcc:unverified]

Current CPU `lib.rs` is already byte-identical to the numerator-scratch candidate
at `c47886258a0e2b3d00a0734f059aaaa3373ca322`, Git blob
`49221e50b514e2c672c1c657496418c280b1ddf5`. The retained parent at
`4939261c9fcc48bf86f17ac36fa458f52a155b2e` has Git blob
`5d36193ca097b942c4b0c6bf6fb4524e8beef314`. Both retain the suffix lookup filter;
the candidate removes stored numerators and derives them in the reverse pass.
Thus the completed 24-call public-target cohort validates the current candidate's
recovery. It does not compare its cost with that parent. [pcc:unverified]

Next comparative work should isolate those two library files on shared source,
inputs and build settings, under a new prospective local protocol. Existing
round24/round25 protocols, raw records, frozen 40-bit cohort, reports, manifests
and review scopes remain unchanged. No new algorithm, runtime observation or
novelty claim is supplied by this correction.

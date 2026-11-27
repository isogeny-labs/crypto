# M31 native build repair: source-pin transition gate

2026-11-24. Scope notice #5122 comment 7139562645. Source-only compatibility audit; no new scientific experiment. [pcc:unverified]

## Material change since the previous invocation

Main build-repair PR #5078 merged at 2026-11-24T15:48:21Z, merge ee44af42892026802dd55bfe96d383faea697926, repair head f4a5570be83710b7b4580076497abd72caeeb46a. Its owner reports successful workspace all-target and wide-vars cargo checks, plus touched-module tests; those are not rerun here. The previously failing #5120 source head 573d704563ed1375bd4cfe2fd8b771f46861c6a1 still has the old prime_root_trace.rs blob 27be9d2730fc0f75c68c615376ca596ccd365122 with powers.get_mut/remove and gcds.remove references. The repair head and observed main both expose cb43be2b6342019574ca009bc99799c5b0b4c74d without these old map references. The old build blocker is repaired on main, not on the unsynchronized lane.

#5120 remains open, mergeable=false at the inspected head; no permission to merge or alter its ledger. #5076 retains eleven-query native admission; #5097 owns collection; #5118 owns source-ID ordering. The #5078 and #5120 PR patch lists contain 67 and 205 paths respectively, overlapping only docs/ACTIVE.md and the Nov24 session log. This is not a full merge-conflict proof against current main; other commits changed the ledger and source dependencies.

## Source identity gate

Read #5120 graph/build.rs, graph/SOURCE_SHA256SUMS, structured/lib.rs and native/f4-source/build.rs. The graph builder asserts each pinned whole-file SHA-256. The RR5/F4 selector asserts the whole-file SHA-256 before extracting uniquely anchored production ranges. The F4 layout copies the whole parent and children and inventories the public example. Thus source repair alone does not qualify the synchronized standalone crates. Five targeted dependency identities were compared at the exact #5120 head and the repair merge (Git blob SHA, not fabricated SHA-256 values):

| Dependency | old Git blob | repair merge Git blob | guard/receipt |
|---|---|---|---|
| src/cryptanalysis/large_prime_graph/eval.rs | 751f7f9a98be3a8f88042328c29c5e5da1f7ab4a | 9659cd8241368c60a806879ec08671f157f59bdd | strict graph pin |
| src/cryptanalysis/ic_boundary.rs | 2ef3958c2d83e1c6a40d2dbc345ab8a8c1032b89 | a694d42cebee77e2841a0a64b3f6a55c4b7803da | strict extraction pin |
| src/cryptanalysis/prime_fast.rs | 2f337a0df99b74fe6d20a9ff090c675d1ed6b310 | b6e38aeff6e2f8e103e42295b0b3ae8edc4dc042 | strict extraction pin |
| src/cryptanalysis/f4_fp.rs | 7be68ee9160b5f8837c6e262b4b7d63bb258a8a6 | d99337946c6f2819cd0f0c1fa6d6e021615722f4 | whole-file copy/inventory |
| examples/prime31_rr5_quartic.rs | d8d4151ca063b810d9255409bf5aa625b9ad69bc | 18e3765e74a997dd67095c92c29cf39fde693c35 | whole-file inventory |

All five differ. At least three explicit whole-source guard checks would refuse the new bytes if the old pinned hashes are retained (the first failure can mask later failures). This is a prediction from source, not an observed compiler failure. Do not simply transplant old receipts to the synchronized source.

The #5078 patches for these files show lint/format changes: evaluator borrow/format and duplicate-module lint, test iterator and formatting in ic_boundary, a lint annotation in prime_fast, equivalent ceiling-word expression in F4, and Option predicate cleanup in the RR5 example. This is not an independent behavioral-equivalence proof across all intervening commits.

Replayed the *anchor selection only* for the two extraction-pinned files using exact connector-returned text at the two frozen refs. Each start/end anchor occurs once, with start before end, in all nine ranges. ic_boundary ranges 2 and 5 differ; ranges 1,3,4,6 match exactly. prime_fast range 1 differs; ranges 2,3 match. Thus 6/9 selected text ranges match and 3/9 change despite unchanged anchors. Range extraction was a deterministic string comparison, not a build or SHA-256 receipt. The changed text is consistent with the displayed repair patch; scope is bounded, and no exhaustive dependency closure audit is claimed.

## Next executable gate (owning native lanes only)

After the existing owner resolves ledger/main synchronization under repository policy, inspect the full changed dependency closure, deliberately refresh pin hashes only for reviewed source, preserve old manifests and all original receipts, and produce a newly pinned source manifest. Use the existing run wrapper and full root gate on the synchronized head; verify graph/structured/RR5 tests and exact original M31 witness replay. No need to re-collect the existing B677 stream. Keep old fixtures as historical evidence, not current-head native passes. A source-only audit cannot establish runtime equivalence. No auto-merge, paid compute, manual workflow dispatch or new collector requested.

## Provenance and constraints

Read current AGENTS, ACTIVE, main and lane Nov24 journal, catalog and SOURCES, current five graph modules, #4917/#4905/#5076/#5097/#5118/#5120/#5122/#5078, bounded cross-repo open PR search and source build guards. KB MCP and Conductor unavailable; primary repository source is the bounded fallback for this compatibility audit, with no new algorithm or novelty assertion. Container inspection found no checkout or cargo/rustc on PATH. No independent worker tool; one coordinator, zero workers. No global job/lease census. A broad connector batch exceeded its tool-call limit; narrowed to the five named dependencies and nine ranges. No claims about unchecked files. Phase elapsed time, CPU and RSS unmeasured. No new random source, measurement, field experiment, seed, or A/B trial; historical M31 p=2147483647 and r=2147446831 retained, prior producer gaps unchanged. Keep experiment-seeds/v1, resource/parallel, publication/no-merge/no-paid-compute, stop and impact requirements. No leaderboard change.

Impact score: 3/10 — Confirmed the main root repair and identified three strict source-pin incompatibilities and two changed inventory inputs that must be qualified before native restart; no synchronized build or speedup has been executed.

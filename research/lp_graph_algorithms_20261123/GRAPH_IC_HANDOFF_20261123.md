# Graph-IC lane handoff — raw projection-rank identity, November 23, 2026

Owner: current draft PR [#4988](https://github.com/isogeny-labs/crypto/pull/4988). **Do not merge** or launch paid compute. This handoff records the continuation rather than replacing historical notes.

## Exact source / study
- Production graph/filter source remains unchanged from the 34-test predecessor. Test source change: `tests/large_prime_graph.rs`, authored at `a15e0cd0ad1f0c653ebe24fbe31f2c397ffa59ba`; single unused-import cleanup at `db7257021de5a2d00dcdfb9f023d90dde5036166`. No edits to another session's worktree or to PRs #4972/#4960.
- Frozen protocol and proof: [RAW_PROJECTION_RANK_IDENTITY_20261123.md](RAW_PROJECTION_RANK_IDENTITY_20261123.md). Input: 128 deterministic seeded two-LP linear systems, 2–34 rows, F3/F7/F101/F1000003/F2147446831, factor-base columns including sparse protected u32::MAX target placeholder. Exact real Prime31 curve relation stream remains **missing**.
- Independently checks `rank(W S)=rank([L|S])-rank(L)` and `rank([W S|Wb])=rank([L|S|b])-rank(L)` without using forest witnesses for raw rank. Source replay and pivot/MST refusal/fallback checked; RHS must be fixed and all unknowns explicit columns.
- Executed original-source native Rust tests: [Actions #45208649527](https://github.com/isogeny-labs/crypto/actions/runs/45208649527), 35/35 PASS on authored test commit (Rust 1.100.0, job 137811171128). The final unused-import-only cleanup revision has separate CI pending at handoff writing. No compiler/runtime errors in the passing set; the first run had one unused import warning.
- No elapsed-time microbenchmark, measured memory, authentic ECDLP relation yield, completed rho or index-calculus comparison, full root-crate build, or signed PCC occurred. Status: original-module bounded correctness PASS; [pcc:unverified].

## Sources/coordination
Read current AGENTS.md, docs/ACTIVE.md, docs/session-logs/2026-11-23.md, graph catalog README/SOURCES, related current PR #4988, #4972, #4960, merged #4905/#4917. Neighbor source receipts: cryptanalysis gain-forest k=3 harvester RESULTS.md at f9c7b49, crypto-autoresearcher tranche2 planning at d46b06b; neither validates ordinary Prime31 two-LP performance. Diem 2006 and Cavallar 2000 remain primary literature context. Research-KB and conductor/worker runtime not exposed; bounded fallback GitHub source/literature reads, no KB source_id. One coordinator authored and checked source; no independent worker outputs claimed.

## Next executable step / blockers
1. Confirm final exact-source CI result at revision db7257021de5; update this handoff and PR with terminal receipt, preserving the earlier run and warning.
2. Search qualifying frozen group-verified natural Prime31 two-LP stream; record actual curve parameters, G, Q, ell=2147446831, hashes and source row IDs. If unavailable, prepare a Rust collector adapter in the existing authorized lane, not a synthetic substitute.
3. Compare forest/lazy, pivot and MST on exactly the same natural stream, including rank and augmented-rank identity, original group replay, materialized support/source fill, inversions, peak RSS, retry/refusal and complete target recovery. Full cold target cost vs best existing IC and matched negation rho is a distinct future gate.
4. Run the unshimmed root library and independent PCC under established authorization; no future PR merge, spend, or leaderboard promotion without owner approval.

No outstanding paid jobs were started. Hosted PR validation workflows may be running automatically; do not duplicate them.

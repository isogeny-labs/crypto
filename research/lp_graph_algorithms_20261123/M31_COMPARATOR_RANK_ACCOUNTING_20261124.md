# M31 comparator: rank accounting and duplicate reduction

2026-11-24 — source-only audit [pcc:unverified]. Scope notice #5122 comment 7141002811.

## Exact source inspected

#5120 head 573d704563ed1375bd4cfe2fd8b771f46861c6a1: graph/compare.rs blob 139b30d0fa2dd162b9596e04c825199c7c6d3124, production projected_rank.rs 3da704552423e65c976481fe5ed42869a9e53813, graph/main.rs 6c11b92e4e73fc6ded9fea20889fa2d76bd54cd2, graph/tests/graph_suite.rs 6637c7b124d666da96f1f79f4077f8ae7bd19ab3, README and source manifest. B677 curve p=2147483647, a=847514846, b=2087444724, G=(625037715,2100627251), prime order ell=2147446831; relation arithmetic F_ell. No new field execution.

## Source-derived findings

1. compare::run calls audit_projected_rank, then source replay, then recover_target. Both reductions use the same original row order, least-column ordered-map pivot, normalized coefficients and RHS. By induction their pivot maps agree on admitted canonical rows over the prime field if both finish. If rank k and E pivot coefficient entries visited, the two successful reductions perform 2k pivot inversions and two traversals of E entries. One opt-in audit exposing the target pivot RHS could avoid the second reduction; this is not half of total graph time or cold cost.
2. Write projected coefficients [F|t], with t the protected target. The current useful_small_rank counts t, while augmented_rank appends a KNOWN RHS. For consistent rows these ranks are equal. The factor-base-only rank is k-I, where I indicates a TARGET pivot. Proof: TARGET is the largest column ID; non-target echelon pivots retain distinct leading coordinates when restricted to F and span the F projection; the TARGET-only pivot projects to zero. Thus I=rank([F|t])-rank(F), exactly target identifiability, and its normalized RHS is the determined target. Preserve historic fields, add explicit factor-base-only rank, target rank increment and identifiability diagnostics. Retain source replay and literal group check. Partial-output absence is not full-stream absence.
3. The first audit_projected_rank has no fill-cap parameter. The second recover_target checks stored+work.len() only at the beginning of each while iteration, not before every transient insertion or initial allocation. Thus max_active_coefficients is not an end-to-end hard working-set cap. OS limits remain separate. A shared bounded audit should preflight allocation, preserve exact refusal semantics and report diagnostic peak separately; don't relabel old receipts. Four-arm rank agreement alone does not prove the full projected span: retain independent raw-rank controls.
4. #5118's witness.rs fix will also change #5120's strict pin if integrated: old manifest SHA-256 80dfe96e62779b661ca5a28a593ac91c65b23ec022d90f9a996e48d92b304fdc, candidate-pre-call receipt 37ee227b10b9aab391784acdf5c74b6c834ac03d4837b58323d7da1e688933bf. Preserve both frozen receipts and qualify the integrated bytes; do not blindly refresh pins.

## Handoff

#5120 owns implementation, #5076 admission, #5097 source, #5118 source-ID fix. Native next gate after protected-ledger/main reconciliation: unchanged two-pass vs opt-in bounded shared audit on identical admitted bytes, limits and original witness checks; existing planted positive/negative prefixes, original B677 ten/eleven-query captures after admission, cap-boundary and mutation cases, independent raw rank oracle. No new collection. Existing seed and paired replay requirements (experiment-seeds/v1), M31, resource/parallel/stop, no merge/no paid compute and impact rules remain binding. No new measurement or general speedup claim; no new seeds or leaderboard update.

Read main AGENTS, ACTIVE and Nov24 journal, lane journal/tracker, catalog and SOURCES, #5078/#5118 patches, current #5120/#5076/#5097/#5118/#5122, bounded open graph PR search across three repos. KB and Conductor unavailable; repository source and catalog fallback; no novelty claim. No local checkout, Rust tools or independent workers; one coordinator, zero workers, no private lease/job census; elapsed/CPU/RSS unavailable. Source-only derivation, no tests or paid work.

Impact score: 3/10 — Identified redundant rank passes and a cap-scope gap and derived an exact factor-base-only rank diagnostic; native integration and performance remain untested.

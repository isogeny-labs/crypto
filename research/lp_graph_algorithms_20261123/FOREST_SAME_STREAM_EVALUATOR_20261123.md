# Opt-in exact same-stream LP forest evaluation — preregistered

Date: 2026-11-23 UTC. Lane: existing draft PR #4988. Status: proposed source change / native gate pending. [pcc:unverified]

## Bottleneck and hypothesis

The source-exact forest witness extractor and sparse useful-rank auditor now exist, but the established `large_prime_graph::eval::evaluate_stream` only records pivot and MST outputs. The next cheapest useful gate is to **wire the existing code into the same frozen-input evaluator**, not invent a new graph algorithm or silently extrapolate synthetic tests into a Prime31 ECDLP result.

Hypothesis: for a canonical arity-0/1/2 stream over prime relation field F_ell, the forest's LP-cancelling witnesses span the complete LP left-nullspace and so its emitted small rows attain the raw projected rank: rank(WS)=rank([L|S])-rank(L), provided materialization caps do not refuse. Pivot and MST are controls on **identical input, modulus and caps**; their realized projected useful ranks must not exceed that upper bound. Witness count and graph-cycle count alone do not certify useful rank.

## Code change and control

- Existing source-only Rust integration: `src/cryptanalysis/large_prime_graph/eval.rs`, `tests/large_prime_graph.rs`; reuse `witness::collect_witnesses_lazy`, `materialize_complete_relations`, `projected_rank::audit_projected_rank`, and the existing pivot/MST source replayer. No new collector, ECDLP arithmetic or graph implementation.
- Add explicit opt-in with a row-count cap. Disabled/default evaluator must preserve the old JSON structure and behavior; enabled arity>2 or over-limit streams report **skipped**, not a false success or an empty output.
- For eligible streams charge witness construction, materialization, useful-rank auditing and exact original-source replay separately, including refusal. Preserve all source identifiers; never turn a partial `None` projection into a successful empty result or hide it behind pivot fallback. Pivot and MST projected ranks are computed and timed only in opt-in mode; refuse an MST rank audit when MST has declined to emit.
- Record LP rank, LP left-nullity, forest witness source terms, emitted small-column N, sparse Z, emitted provenance terms, row weight, exact useful small rank, augmented RHS rank, failure status and all phase times. Timing uses one source-bound CI runner and is **diagnostic**, not benchmark admission. Unknown target log is always an explicit small column, not an RHS constant.

## Native correctness protocol (no performance inference)

Run the unchanged existing hosted source-exact Rust 1.100.0 release test workflow on the frozen PR revision. Add fixed weighted 2-LP cases over F7 and F2147446831, including anchored-bridge and balanced-cycle cases, an explicit target-coordinate small column, repeated/reduced terms, strict materialization refusal and an arity-3 skip, plus at least one deterministic seeded cohort. Assert original-source replay of all emitted rows; exact forest projected rank against independent raw dense rank([L|S])-rank(L), forest rank/nullity accounting, pivot/MST ranks bounded above, and no opt-in fields in default JSON. An implementation or comparison that violates any assertion is falsified. Preserve any failed CI attempt.

## Resource, permissions and evidence limits

- One coordinator, no native Rust compiler in the connected session. The existing GitHub Actions source-exact Rust job is the only execution backend authorized here; check prior jobs before pushing. No parallel builds, paid compute, hosted EKS workloads, root-crate bypass claims, PR merge or leaderboard promotion.
- Curve for eventual natural-stream follow-up: p=2147483647, E: y²=x³+847514846x+2087444724, G=(625037715,2100627251), ell=2147446831 (cofactor one). Do not claim these small algebra-only fixtures are authentic group-generated Prime31 relations. Group verification and full cold comparison remain separate.
- Sources checked (KB): research-KB MCP tools not exposed; no local checkout or executable wrapper. Bounded fallback: current AGENTS.md, ACTIVE.md, dated log and handoff, graph catalog/SOURCES, exact original Rust modules, merged #4905/#4917/#4960/#4972, cryptanalysis harvester-k3 Results (not prime-field equivalent), crypto-autoresearcher code search, Diem 2006 https://arxiv.org/abs/math/0606607 and Cavallar 2000 https://ir.cwi.nl/pub/4456. No novelty or global absence claim.
- Coordination: continue existing #4988, avoid neighboring PQ/Wiedemann and `ic_graph` lanes. Conductor/worker API unavailable: sequential authorship, not delegated work.

## Evidence ledger

- Protocol frozen before implementation. No new native test or timing receipt.

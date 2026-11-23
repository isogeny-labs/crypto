# Source-frozen complete-union candidate for inherited binary HEC blocker

Date: 2026-11-23. Lane: [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977). Current reviewed parent head: `f566c6faa6a7c69d58a20f59ffd4a1dbe2a1778c`. **Status: source-only review artifact, [pcc:unverified]. Not built, tested, approved, merged or installed at the production path.**

## Concrete deliverable

The standalone [candidate](./BINARY_HEC_COMPLETE_UNION_CANDIDATE.rs) has **1,543 lines**, blob `616422524711c1dd0e54b6bb6c557348b999e262`. It was reconstructed at exact frozen sources, not copied from the malformed merge:

- Parent A `a9d68fbd9c2dc5b6db267746c483febd22cb5501`, blob `12758d4edc3f1d5f6c6c66ebb32397d04c991af7`, 1,475 lines: original relation classification, historical solver and tests.
- Parent B `0d4721abfeb0c402cb7de6227e0f2108101162dc`, blob `89a190d53bcd20a8141c219db2f0f8fd963b9ce2`, 1,540 lines: projected-stage API and scale-one wrapper, with the same tests.
- Broken merge `77d388738a48eb5a6e2abd80636c2e31cceab254` blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`, still identical on live `main` and this PR head during this review.

Candidate construction is **Parent B plus exactly two text-level replacements**: restore Parent A's full `match classified { Some(entries) => ..., None => {} }` block in place of B's equivalent `if let Some(entries)` block; and use the explicit `curve.genus as u32` expression in `hasse_interval`. No module in `src/` was changed.

## Independently challenged source properties (not Rust compilation)

1. The parent-to-parent line-LCS difference consists of **16 alignment blocks** with the chosen tie-breaking (the earlier audit's **15** used a different alignment); they concentrate in the no-op `let pos=pos`, the classification syntax, projected API/stage/timing/verification additions, and the redundant genus cast. Hunk count depends on alignment and is not a semantic proof or an exact line-loss count.
2. The reconstructed source retains all public function/type symbols from both parents. It contains `BhecIndexCalculusStage`, `binary_hec_projected_index_calculus_dlp`, and the historical `binary_hec_index_calculus_dlp` wrapper calling projected with scale one. The `#[cfg(test)]` suffix is **byte-identical in both parents and in the candidate**. These are structural inspections, not passing tests.
3. The replacement classification block is unique in each parent, and differs only by match-vs-if-let control flow. Parent B already preserved the identity relation path and smoothness reporting. Parent A's `let pos = pos;` is a name-for-name binding and adds no transformation; it was not reintroduced.
4. On documented **prime order** `n`, the scale-one wrapper projects labels by multiplying by 1 modulo n and adds a no-op stage callback. Thus the modular rows passed to `solve_for_logarithm` are the same as the original collector's rows, conditional on identical deterministic group operations and collection. New stage/timing instrumentation, verification counters and final duplicate-verification removal can change *reports/work*, so this is **not** a proof of bit-identical reports or compiled semantic equivalence.
5. Nonunit label scaling in composite-order groups is **outside** the documented prime-order contract. A nonzero `label_scale` is invertible modulo prime `n`; under a composite `n` that implication fails. A caller that requires completeness must prove its map and source-order assumptions rather than treat a nonzero scale as sufficient.
6. Raw brace counts balance (**296 opens, 296 closes**) and the expected legacy test ending is present. Parent B's public exports and unchanged test module survived. Raw delimiter counts are only a coarse guard: this run lacked `rustc`, `cargo`, `rustfmt`, tree-sitter, authorized original-root builds and independent signature support.

## Exact next gate

An independently assigned reviewer must inspect the **actual 16 difference blocks** and caller surfaces, including projected map semantics, the four stage callbacks, nonunit/zero-scale behavior and both parent test suites. Only then install the candidate through a separately authorized, narrowly scoped root-source repair; freeze checkout SHA, retain original `Cargo.lock`, the restored guards from #4962 and all inherited tests, and run the five original-root commands plus scale-one/nonunit-scale tests under the same resource/admission rules. Never replace the module with just one unreviewed parent, strip tests or use this standalone file as evidence that root CI passed. A root repair is an execution dependency, **not** an S3 query speedup.

## Provenance, workers and literature

Coordinator checked current AGENTS, ACTIVE, dated handoff, state, live #4977/#4979, review threads and CI; source revision `f566c6faa6a7c69d58a20f59ffd4a1dbe2a1778c`; both original parent files and current main's blob were fetched through the authorized GitHub connector. Existing root-source audit: [BINARY_HEC_BOTH_PARENT_AUDIT.md](./BINARY_HEC_BOTH_PARENT_AUDIT.md). No native child-worker slots were exposed. Coordinator performed algebraic contract critique, parallel read-only comparisons and validation checklist; Rust-worker implementation was limited to this **isolated, noncompiled source artifact**; the validation work package is source-only. Neither teammate agreement nor producer inspection is signed validation.

Relevant mathematical context remains Semaev 2015, https://arxiv.org/abs/1504.01175; this source-only repair does not claim a new elimination method. New literature checked: Fischer, *Sumsets, 3SUM, Subset Sum: Now for Real!* (SODA 2025), https://doi.org/10.1137/1.9781611978322.155, and Dinur–Golovnev, *Improved Time-Space Tradeoffs for 3SUM-Indexing* (ICALP 2026), https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html. Both rely on representation/preprocessing models not freely supplied by elliptic group addition; neither creates a DLP-free, sublinear empty-query oracle here. Shared research-KB and Conductor were not exposed in this runtime; no novelty claim is made.

The original Prime31 exact query remains open: a source-specific, near-linear-setup, empty-inclusive sublinear target index including signed witnesses, exceptional paths, useful rank and total cost has not been demonstrated. [pcc:unverified]
